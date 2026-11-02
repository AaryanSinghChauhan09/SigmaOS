//! Integration tests — syntax highlighting.

use sigmaos_text_editor::{SyntaxHighlighter, TokenKind};

#[test]
fn comment_line_highlighted_as_comment() {
    let hl = SyntaxHighlighter::new("rust");
    let spans = hl.highlight_line("// this is a comment");
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].kind, TokenKind::Comment);
}

#[test]
fn empty_line_produces_no_spans() {
    let hl = SyntaxHighlighter::new("rust");
    let spans = hl.highlight_line("");
    assert!(spans.is_empty());
}

#[test]
fn non_comment_line_highlighted() {
    let hl = SyntaxHighlighter::new("rust");
    let spans = hl.highlight_line("let x = 42;");
    assert!(!spans.is_empty());
    assert_eq!(spans[0].start, 0);
    assert_eq!(spans[0].end, "let x = 42;".len());
}

#[test]
fn span_covers_full_line() {
    let hl = SyntaxHighlighter::new("plain");
    let line = "hello world";
    let spans = hl.highlight_line(line);
    assert_eq!(spans[0].end, line.len());
}
