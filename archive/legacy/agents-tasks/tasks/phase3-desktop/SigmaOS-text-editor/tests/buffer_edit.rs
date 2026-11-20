//! Integration tests — buffer editing operations.

use sigmaos_text_editor::{Buffer, open_editor};

#[test]
fn editor_opens_with_empty_buffer() {
    let ed = open_editor(None);
    assert!(!ed.buffer.dirty);
}

#[test]
fn buffer_append_marks_dirty() {
    let mut buf = Buffer::from_str("hello\nworld");
    buf.append_to_line(0, " there");
    assert!(buf.dirty);
    assert_eq!(buf.line(0), Some("hello there"));
}

#[test]
fn buffer_insert_line_after() {
    let mut buf = Buffer::from_str("a\nb");
    buf.insert_line_after(0, "middle");
    assert_eq!(buf.line_count(), 3);
    assert_eq!(buf.line(1), Some("middle"));
}

#[test]
fn buffer_delete_line() {
    let mut buf = Buffer::from_str("x\ny\nz");
    buf.delete_line(1);
    assert_eq!(buf.line_count(), 2);
    assert_eq!(buf.line(1), Some("z"));
}

#[test]
fn buffer_to_string_roundtrip() {
    let content = "fn main() {\n    println!(\"hi\");\n}";
    let buf = Buffer::from_str(content);
    assert_eq!(buf.to_string(), content);
}

#[test]
fn editor_stores_file_path() {
    let ed = open_editor(Some("/home/user/notes.txt"));
    assert_eq!(ed.file_path.as_deref(), Some("/home/user/notes.txt"));
}
