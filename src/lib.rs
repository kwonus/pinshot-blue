extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::any::Any;
use std::borrow::Borrow;
use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "avx-quelle.pest"]
struct QuelleParser;

struct ParseResult {
    result: String,
    error: String,
}

fn get_parse(stmt: &str) -> ParseResult {
    let input_string = stmt.to_string();

    let task = QuelleParser::parse(Rule::statement, &input_string);

    if task.is_ok() {
        let pairs = task.unwrap();
        //recurse(pairs, &mut top);
        let root = ParseResult {
            result: pairs.to_string(),
            error: "".to_string(),
        };
        root
    }
    else if task.is_err() {
        let mut root = ParseResult {
            result: "".to_string(),
            error: task.unwrap_err().to_string(),
        };
        if root.error.is_empty() {
            root.error = "Unknown Error".to_string();
        }
        root
    }
    else {
        let root = ParseResult {
            result: "".to_string(),
            error: "Internal Error".to_string(),
        };
        root
    }
}

// adapted from: https://rust-unofficial.github.io/patterns/idioms/ffi/accepting-strings.html
// and from:     http://jakegoulding.com/rust-ffi-omnibus/string_return/

use std::ffi::{c_char, CString};
use std::task::Poll;

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

    let mut root = get_parse(&statement_str);

    if statement_str.is_empty() {
        root.error = "Invalid Input Error".to_string();
    }
    if root.result.is_empty() && !root.error.is_empty() {
        root.error = "Unexpected empty result".to_string();
    }
    let output = if root.error.is_empty() { root.result } else { root.error };  // non-error will always begin / end with { / }
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
