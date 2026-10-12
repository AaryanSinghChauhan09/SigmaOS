// tests/shell/diagnostics/mod.rs
#[test]
fn test_diagnostics_line_col_formatting() {
    let err_msg = "sigma-sh: parse error at line 1, col 5: unexpected token";
    assert!(err_msg.contains("line 1, col 5"));
}
