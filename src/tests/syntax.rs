use crate::{assert_grammar_revision_internal, get_parse_raw};

#[test]
fn invoke() {
    let statement_str = "$foo-bar";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}
#[test]
fn absorb() {
    let statement_str = "@absorb foo-bar";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}
#[test]
fn exit() {
    let statement_str = "@exit";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}

#[test]
fn ordered_statement() {
    let statement_str = "\"/BoV/&in|out&/prep/ /det/ begin* God \\create\\\"";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}
#[test]
fn unordered_compound_statement() {
    let statement_str = "/BoV/&in|out&/prep/ + /det/ begin* ; God \\create\\";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}
#[test]
fn assert_correct_version() {
    let v :u16 = 0x4101;
    let result = assert_grammar_revision_internal(2, 0, v);
    assert_eq!(result, v);
}
#[test]
fn assert_incorrect_version() {
    let v :u16 = 0x3C21;
    let result = assert_grammar_revision_internal(2, 0, v);
    assert_eq!(result, 0);
}
/*
#[test]
fn bad() {
    let statement_str = "/foo/";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}
 */