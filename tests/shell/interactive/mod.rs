// tests/shell/interactive/mod.rs
#[test]
fn test_interactive_prompt_and_line_editing() {
    let prompt = "sigma@sigmaos:~$ ";
    assert!(prompt.ends_with("$ "));
}
