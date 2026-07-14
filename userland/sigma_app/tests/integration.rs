use sigma_app::{SigmaEditor, SigmaBrowser};

#[test]
fn test_editor_buffer_operations() {
    let mut editor = SigmaEditor::new();
    assert_eq!(editor.active_buffer.content, "");
    
    editor.insert_text("Hello SigmaOS");
    assert_eq!(editor.active_buffer.content, "Hello SigmaOS");
    assert_eq!(editor.active_buffer.cursor_offset, 13);

    assert!(editor.move_cursor(5).is_ok());
    assert_eq!(editor.active_buffer.cursor_offset, 5);

    editor.insert_text(", World!");
    assert_eq!(editor.active_buffer.content, "Hello, World! SigmaOS");

    assert!(editor.move_cursor(99).is_err());
}

#[test]
fn test_browser_navigation() {
    let mut browser = SigmaBrowser::new();
    assert_eq!(browser.views.len(), 0);

    let view = browser.navigate("https://sigma.sovereign").expect("Navigation failed");
    assert_eq!(view.current_url, "https://sigma.sovereign");
    assert_eq!(browser.views.len(), 1);
}
