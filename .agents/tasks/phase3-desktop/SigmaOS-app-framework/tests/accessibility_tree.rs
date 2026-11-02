//! Integration tests — accessibility tree.

use sigmaos_app_framework::{Button, Label, Container, Widget};

#[test]
fn button_accessibility_role_is_button() {
    let btn = Button::new(1, "Save");
    let node = btn.accessibility_node();
    assert_eq!(node.role, "button");
}

#[test]
fn button_accessibility_label_matches() {
    let btn = Button::new(2, "Delete");
    let node = btn.accessibility_node();
    assert_eq!(node.label, "Delete");
}

#[test]
fn disabled_button_not_focusable() {
    let mut btn = Button::new(3, "Locked");
    btn.enabled = false;
    let node = btn.accessibility_node();
    assert!(!node.focusable);
}

#[test]
fn label_accessibility_role_is_label() {
    let lbl = Label::new(4, "Username");
    let node = lbl.accessibility_node();
    assert_eq!(node.role, "label");
    assert!(!node.focusable);
}

#[test]
fn container_child_nodes_collected() {
    let mut c = Container::new(10, "form");
    c.add_child(Box::new(Button::new(11, "Submit")));
    c.add_child(Box::new(Label::new(12, "Hint")));
    let nodes = c.child_accessibility_nodes();
    assert_eq!(nodes.len(), 2);
    assert!(nodes.iter().any(|n| n.role == "button"));
    assert!(nodes.iter().any(|n| n.role == "label"));
}
