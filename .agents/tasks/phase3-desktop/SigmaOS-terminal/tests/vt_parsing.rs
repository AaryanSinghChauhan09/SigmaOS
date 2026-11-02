//! Integration tests — VT/ANSI escape sequence parsing.

use sigmaos_terminal::{VtParser, VtParserState};

#[test]
fn printable_ascii_passes_through() {
    let mut p = VtParser::default();
    p.feed(b"hello");
    assert_eq!(
        p.printable,
        vec!['h', 'e', 'l', 'l', 'o']
    );
}

#[test]
fn escape_sequence_consumed_silently() {
    let mut p = VtParser::default();
    // ESC [ 2 J  — clear screen
    p.feed(b"\x1b[2J");
    assert!(p.printable.is_empty(), "escape bytes must not appear as printable");
    assert_eq!(p.state(), VtParserState::Ground);
}

#[test]
fn mixed_text_and_escape() {
    let mut p = VtParser::default();
    p.feed(b"hi\x1b[1mworld");
    // "hi" appears before the escape, "world" after — both should be printable
    assert!(p.printable.contains(&'h'));
    assert!(p.printable.contains(&'w'));
}

#[test]
fn control_bytes_not_printed() {
    let mut p = VtParser::default();
    p.feed(b"\x00\x01\x02\x03");
    assert!(p.printable.is_empty());
}
