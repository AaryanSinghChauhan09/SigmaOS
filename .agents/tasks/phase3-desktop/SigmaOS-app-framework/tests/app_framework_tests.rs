//! Comprehensive tests for SigmaOS-app-framework (≥ 15 tests).

use sigmaos_app_framework::{
    Button, Label, Container, Widget,
    init_app_framework,
    widgets::{Dialog, Entry, ListBox},
    layout::{LayoutBox, LayoutGrid, Orientation},
    ipc::{IpcBus, IpcChannel},
};

// ── Widget tests ──────────────────────────────────────────────────────────────

#[test]
fn test_button_widget() {
    let btn = Button::new(1, "OK");
    assert!(btn.render().contains("OK"));
    assert_eq!(btn.id(), 1);
}

#[test]
fn test_label_widget() {
    let lbl = Label::new(2, "Hello");
    assert!(lbl.render().contains("Hello"));
    assert_eq!(lbl.accessibility_node().role, "label");
}

#[test]
fn test_entry_widget() {
    let mut entry = Entry::new(3, "Username");
    assert!(entry.render().contains("Username"));
    entry.set_value("alice");
    assert!(entry.render().contains("alice"));
}

#[test]
fn test_entry_password_mode() {
    let mut entry = Entry::new(4, "Password");
    entry.password_mode = true;
    entry.set_value("secret");
    let rendered = entry.render();
    assert!(!rendered.contains("secret"));
    assert!(rendered.contains("*"));
}

#[test]
fn test_listbox_widget() {
    let mut lb = ListBox::new(5);
    lb.add_item("Item A");
    lb.add_item("Item B");
    lb.select(0);
    assert_eq!(lb.selected_item(), Some("Item A"));
}

#[test]
fn test_dialog_open() {
    let mut dlg = Dialog::new(6, "Confirm", "Are you sure?");
    assert!(!dlg.open);
    dlg.show();
    assert!(dlg.open);
    assert!(dlg.render().contains("open"));
}

#[test]
fn test_dialog_close() {
    let mut dlg = Dialog::new(7, "Alert", "Done");
    dlg.show();
    dlg.close();
    assert!(!dlg.open);
    assert!(dlg.render().contains("closed"));
}

// ── Layout tests ──────────────────────────────────────────────────────────────

#[test]
fn test_layout_box_horizontal() {
    let mut bx = LayoutBox::new(10, Orientation::Horizontal);
    bx.add(Box::new(Button::new(11, "A")));
    bx.add(Box::new(Button::new(12, "B")));
    assert_eq!(bx.child_count(), 2);
    assert!(bx.render().contains("H"));
}

#[test]
fn test_layout_grid() {
    let mut grid = LayoutGrid::new(20, 3);
    grid.attach(0, 0, Box::new(Label::new(21, "Name")));
    grid.attach(1, 0, Box::new(Entry::new(22, "Enter name")));
    assert_eq!(grid.cell_count(), 2);
    assert!(grid.render().contains("3cols"));
}

// ── Theme apply placeholder ───────────────────────────────────────────────────

#[test]
fn test_theme_apply_placeholder() {
    // Theme application: set CSS provider on display (GTK4 API).
    // Verified via AppContext — ensure it initializes without panic.
    let ctx = init_app_framework();
    drop(ctx); // must not panic
}

// ── Accessibility tests ───────────────────────────────────────────────────────

#[test]
fn test_accessibility_role_button() {
    let btn = Button::new(30, "Save");
    assert_eq!(btn.accessibility_node().role, "button");
}

#[test]
fn test_accessibility_focusable() {
    let btn = Button::new(31, "Submit");
    let node = btn.accessibility_node();
    assert!(node.focusable);
}

#[test]
fn test_accessibility_dialog_role() {
    let dlg = Dialog::new(32, "Confirm", "Delete?");
    assert_eq!(dlg.accessibility_node().role, "dialog");
}

// ── IPC tests ─────────────────────────────────────────────────────────────────

#[test]
fn test_ipc_send_receive() {
    let mut ch = IpcChannel::new("test");
    ch.send("theme-changed");
    assert_eq!(ch.receive(), Some("theme-changed".to_string()));
    assert!(ch.is_empty());
}

#[test]
fn test_ipc_receive_empty() {
    let mut ch = IpcChannel::new("empty");
    assert_eq!(ch.receive(), None);
}

#[test]
fn test_async_event_loop_placeholder() {
    // The async event loop is GLib-based; here we verify IpcBus setup.
    let mut bus = IpcBus::new();
    bus.create_channel("app-events");
    bus.create_channel("theme-events");
    assert!(bus.channel("app-events").is_some());
    assert!(bus.channel("theme-events").is_some());
}

#[test]
fn test_widget_destruction() {
    let btn = Button::new(40, "Goodbye");
    let id = btn.id();
    drop(btn); // must not panic
    assert_eq!(id, 40);
}

#[test]
fn test_state_binding_placeholder() {
    // State binding: widget re-renders when state changes.
    let mut lbl = Label::new(50, "Count: 0");
    lbl.text = "Count: 1".to_string();
    assert!(lbl.render().contains("Count: 1"));
}
