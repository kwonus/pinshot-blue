use crate::{get_parse_raw};

#[test]
fn invoke() {
    let statement_str = "#foo-bar";
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
    let statement_str = "\"/BoV/&in|out&/prep/ /det/ begin* God [create]\"";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}
#[test]
fn unordered_statement() {
    let statement_str = "/BoV/&in|out&/prep/ /det/ begin* God [create]";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}
#[test]
fn assert_view_history() {
    let statement_str = "@view #1";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}
#[test]
fn assert_view_macros() {
    let statement_str = "@view from 1/1/2024 to 4/4/2924";
    let raw = get_parse_raw(&statement_str);
    assert_eq!(raw.result.is_empty() && !raw.error.is_empty(), false);
}