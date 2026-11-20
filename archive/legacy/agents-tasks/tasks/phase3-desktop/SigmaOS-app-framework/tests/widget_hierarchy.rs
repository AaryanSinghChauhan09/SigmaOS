//! Integration tests — widget hierarchy.

use sigmaos_app_framework::{
    init_app_framework, Button, Label, Container, Widget,
};

#[test]
fn button_render_contains_label_text() {
    let btn = Button::new(1, "Submit");
    assert!(btn.render().contains("Submit"));
}

#[test]
fn disabled_button_render_shows_disabled() {
    let mut btn = Button::new(2, "Click");
    btn.enabled = false;
    assert!(btn.render().contains("disabled"));
}

#[test]
fn label_render_contains_text() {
    let lbl = Label::new(3, "Hello World");
    assert!(lbl.render().contains("Hello World"));
}

#[test]
fn container_render_includes_children() {
    let mut c = Container::new(10, "main");
    c.add_child(Box::new(Button::new(11, "A")));
    c.add_child(Box::new(Label::new(12, "B")));
    let rendered = c.render();
    assert!(rendered.contains("A"));
    assert!(rendered.contains("B"));
}

#[test]
fn nested_containers() {
    let mut inner = Container::new(20, "inner");
    inner.add_child(Box::new(Label::new(21, "nested")));
    let mut outer = Container::new(30, "outer");
    outer.add_child(Box::new(inner));
    assert_eq!(outer.child_count(), 1);
}

#[test]
fn alloc_id_starts_at_one() {
    let mut ctx = init_app_framework();
    assert_eq!(ctx.alloc_id(), 1);
}
