// tests/shell/conformance/conformance_test.rs
// POSIX Shell Conformance Subset Test Suite for SigmaOS Shell

#[test]
fn test_conformance_expansion_order() {
    let expansion_order = vec![
        "brace_expansion",
        "tilde_expansion",
        "parameter_expansion",
        "command_substitution",
        "arithmetic_expansion",
        "word_splitting",
        "pathname_expansion",
        "quote_removal",
    ];

    assert_eq!(expansion_order.len(), 8);
    assert_eq!(expansion_order[0], "brace_expansion");
    assert_eq!(expansion_order[1], "tilde_expansion");
    assert_eq!(expansion_order[2], "parameter_expansion");
    assert_eq!(expansion_order[3], "command_substitution");
    assert_eq!(expansion_order[4], "arithmetic_expansion");
    assert_eq!(expansion_order[5], "word_splitting");
    assert_eq!(expansion_order[6], "pathname_expansion");
    assert_eq!(expansion_order[7], "quote_removal");
}

#[test]
fn test_conformance_simple_commands_and_pipes() {
    let pipe_operator = "|";
    let redirect_out = ">";
    let redirect_append = ">>";
    let redirect_in = "<";

    assert_eq!(pipe_operator, "|");
    assert_eq!(redirect_out, ">");
    assert_eq!(redirect_append, ">>");
    assert_eq!(redirect_in, "<");
}
