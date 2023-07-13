#[cfg(test)]
mod tests;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pairs;
use serde::Serialize;
use itoa;

#[derive(Parser)]
#[grammar = "avx-quelle.pest"]
struct QuelleParser;

struct ParseResult {
    parse: Vec<Parsed>,
    error: String,
}

#[derive(Serialize)]
pub struct Parsed {
    rule: String,
    text: String,
    children: Vec<Parsed>,
}

#[derive(Serialize)]
pub struct RootParse {
    input: String,
    result: Vec<Parsed>,
    error: String,
}

#[derive(Serialize)]
pub struct RawParseResult {
    input: String,
    result: String,
    error: String,
}

fn get_parse(stmt: &str) -> ParseResult {
    let input_string = stmt.to_string();
    let mut top: Vec<Parsed> = vec![];
    let task = QuelleParser::parse(Rule::statement, &input_string);

    if task.is_ok() {
        let pairs = task.unwrap();
        recurse(pairs, &mut top);
        let root = ParseResult {
            parse: top,
            error: "".to_string(),
        };
        root
    }
    else if task.is_err() {
        let mut root = ParseResult {
            parse: top,
            error: task.unwrap_err().to_string(),
        };
        if root.error.is_empty() {
            root.error = "Unknown Error".to_string();
        }
        root
    }
    else {
        let root = ParseResult {
            parse: top,
            error: "Internal Error".to_string(),
        };
        root
    }
}

pub fn get_parse_raw(stmt: &str) -> RawParseResult {
    let input_string = stmt.to_string();
    let return_input_string = input_string.clone();

    let task = QuelleParser::parse(Rule::statement, &input_string);

    if task.is_ok() {
        let pairs = task.unwrap();
        let root = RawParseResult {
            input: return_input_string,
            error: "".to_string(),
            result: pairs.to_string(),
        };
        root
    }
    else if task.is_err() {
        let mut root = RawParseResult {
            input: return_input_string,
            error: task.unwrap_err().to_string(),
            result: "".to_string(),
        };
        if root.error.is_empty() {
            root.error = "Unknown Error".to_string();
        }
        root
    }
    else {
        let root = RawParseResult {
            input: return_input_string,
            error: "Internal Error".to_string(),
            result: "".to_string(),
        };
        root
    }
}

// adapted from: https://rust-unofficial.github.io/patterns/idioms/ffi/accepting-strings.html
// and from:     http://jakegoulding.com/rust-ffi-omnibus/string_return/

use std::ffi::{c_char, CString};

/// Log a message at the specified level.
///
/// # Safety
///
/// It is the caller's guarantee to ensure `stmt`:
///
/// - is not a null pointer
/// - points to valid, initialized data
/// - points to memory ending in a null byte
/// - won't be mutated for the duration of this function call
#[no_mangle]
pub unsafe extern "C" fn create_quelle_parse(c_stmt: *const c_char) -> *mut c_char {

    // SAFETY: The caller has already guaranteed this is okay (see the
    // `# Safety` section of the doc-comment).
    let statement_str: &str = if c_stmt.is_null() {""} else { match std::ffi::CStr::from_ptr(c_stmt).to_str() {
        Ok(stmt) => stmt,
        Err(_e) => "",
    } };

    let mut result = get_parse(&statement_str);

    if statement_str.is_empty() {
        result.error = "Invalid Input Error".to_string();
    }
    else if result.parse.is_empty() && !result.error.is_empty() {
        result.error = "Unexpected empty parse result".to_string();
    }

    let root = RootParse {
        input: statement_str.to_string(),
        result: result.parse,
        error: result.error,
    };
    let output: String = match serde_json::to_string(&root) {
        Ok(r) => r,
        Err(_err) => "{ \"error\": \"bad program design\"}".to_string(),
    };

    let output_cstr = CString::new(output).unwrap();
    output_cstr.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn create_quelle_parse_raw(c_stmt: *const c_char) -> *mut c_char {

    // SAFETY: The caller has already guaranteed this is okay (see the
    // `# Safety` section of the doc-comment).
    let statement_str: &str = if c_stmt.is_null() {""} else { match std::ffi::CStr::from_ptr(c_stmt).to_str() {
        Ok(stmt) => stmt,
        Err(_e) => "",
    } };

    let mut raw = get_parse_raw(&statement_str);

    if statement_str.is_empty() {
        raw.error = "Invalid Input Error".to_string();
    }
    if raw.result.is_empty() && !raw.error.is_empty() {
        raw.error = "Unexpected empty parse result".to_string();
    }

    let output: String = match serde_json::to_string(&raw) {
        Ok(r) => r,
        Err(_err) => "{ \"error\": \"bad program design\"}".to_string(),
    };

    let output_cstr = CString::new(output).unwrap();
    output_cstr.into_raw()
}

#[no_mangle]
pub extern "C" fn delete_quelle_parse(c_lent: *mut c_char) -> bool {
    unsafe {
        if c_lent.is_null() {
            return false;
        }
        let _reclaim = CString::from_raw(c_lent);
        return true;
    };
}

pub fn assert_grammar_revision_internal(iversion: u32) -> u32 {  // "2.0.3.711" == 203_0711

    let major :u32 =  iversion / 100_0000 ;
    let minor :u32 = (iversion /  10_0000) % 10;
    let year  :u32 = (iversion /   1_0000) % 10;
    let mmdd  :u32 =  iversion %   1_0000;

    let mm :u32 =  mmdd / 100;
    let dd :u32 =  mmdd % 100;

    let mut decimal = itoa::Buffer::new();
    let mut version = "_AVX_REV_ := ".to_owned() + decimal.format(major) + "." + decimal.format(minor) + "." + decimal.format(year) + ".";

    if mm > 12 {
        version.push_str("X");
    }
    else if mm == 12 {
        version.push_str("C");
    }
    else if mm == 11 {
        version.push_str("B");
    }
    else if mm == 10 {
        version.push_str("A");
    }
    else {
        version.push_str(decimal.format(mm));
    }
    if dd < 10 {
        version.push_str("0");
    }
    version.push_str(decimal.format(dd));

    if major > 0 && major < 10 && minor < 10 && year >= 3 && year < 10 && mm > 0 && mm <= 12 && dd > 0 && dd <= 31 {
        let result = get_parse(&version);

        if !result.error.is_empty() {
            return 0;
        }
        if result.parse.len() == 1 && result.parse[0].children.len() == 1 && result.parse[0].rule == "statement" &&  result.parse[0].children[0].rule == "avx_rev" {
            return iversion;
        }
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn assert_grammar_revision(iversion: u32) -> u32 {  // "2.0.3.711" == 203_0711
    return assert_grammar_revision_internal(iversion);
}

fn recurse(children: Pairs<Rule>, items: &mut Vec<Parsed>)
{
    for pair in children {
        let mut result: Vec<Parsed> = vec![];
        let text = pair.as_str().to_string();
        let mut rule = pair.to_string();
        // A non-terminal pair can be converted to an iterator of the tokens which make it up:
        recurse(pair.into_inner(), &mut result);

        let paren = rule.find('(').unwrap();
        if paren > 0 {
            rule = rule[0..paren].to_string();
        }
        let item = Parsed {
            rule: rule,
            text: text,
            children: result,
        };
        items.push(item);
    }
}