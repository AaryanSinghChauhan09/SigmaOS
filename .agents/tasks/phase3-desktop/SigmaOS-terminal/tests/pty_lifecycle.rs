//! Integration tests — PTY master lifecycle.

use sigmaos_terminal::{launch_terminal, TerminalConfig, PtyMaster};

#[test]
fn terminal_launches_with_default_config() {
    let t = launch_terminal(TerminalConfig::default());
    assert_eq!(t.grid.rows(), 24);
    assert_eq!(t.grid.cols(), 80);
}

#[test]
fn pty_inject_drain_roundtrip() {
    let mut pty = PtyMaster::default();
    pty.inject(b"test data");
    let drained = pty.drain();
    assert_eq!(drained, b"test data");
}

#[test]
fn pty_drain_clears_buffer() {
    let mut pty = PtyMaster::default();
    pty.inject(b"abc");
    pty.drain();
    let second = pty.drain();
    assert!(second.is_empty());
}

#[test]
fn process_pending_feeds_parser() {
    let mut t = launch_terminal(TerminalConfig::default());
    t.pty.inject(b"ok");
    t.process_pending();
    assert!(t.parser.printable.contains(&'o'));
    assert!(t.parser.printable.contains(&'k'));
}
