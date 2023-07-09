use crate::get_parse_raw;

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