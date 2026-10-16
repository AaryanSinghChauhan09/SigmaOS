// tests/shell/interactive/interactive_test.rs
// Interactive Shell Test Suite for SigmaOS Shell

#[path = "../../../src/shell/interactive/mod.rs"]
mod interactive;

use interactive::{InteractiveShellEngine, SignalEvent};

#[test]
fn test_interactive_prompt_and_history() {
    let mut engine = InteractiveShellEngine::new();
    let prompt = engine.render_prompt();
    assert!(prompt.contains("sigma@sigmaos:~"));

    engine.add_history("ls -la");
    engine.add_history("pwd");
    assert_eq!(engine.history.len(), 2);
    assert_eq!(engine.history[0], "ls -la");
    assert_eq!(engine.history[1], "pwd");
}

#[test]
fn test_interactive_key_handling() {
    let mut engine = InteractiveShellEngine::new();
    let evt = engine.handle_key_input("\x03");
    assert_eq!(evt, SignalEvent::Interrupt);
}
