//! Input event dispatch and routing tests.
//!
//! Verifies that `InputHandler` correctly queues, routes, and drains events,
//! tracks pointer position, and enforces queue limits.

use sigmaos_desktop::{InputError, InputEvent, InputHandler, Modifiers, SurfaceId};

// ── helpers ───────────────────────────────────────────────────────────────────

fn no_mods() -> Modifiers {
    Modifiers::default()
}

fn key_press(keycode: u32) -> InputEvent {
    InputEvent::KeyPress { keycode, modifiers: no_mods() }
}

fn key_release(keycode: u32) -> InputEvent {
    InputEvent::KeyRelease { keycode, modifiers: no_mods() }
}

fn pointer_move(x: i32, y: i32) -> InputEvent {
    InputEvent::PointerMove { x, y }
}

// ── initial state ─────────────────────────────────────────────────────────────

#[test]
fn handler_starts_with_empty_queue() {
    let mut h = InputHandler::new();
    assert!(h.drain_events().is_empty());
}

#[test]
fn handler_starts_with_no_focus() {
    let h = InputHandler::new();
    assert_eq!(h.focused_surface(), None);
}

#[test]
fn handler_starts_at_origin() {
    let h = InputHandler::new();
    assert_eq!(h.pointer_pos(), (0, 0));
}

// ── event queuing ─────────────────────────────────────────────────────────────

#[test]
fn push_single_event_drains_correctly() {
    // Arrange
    let mut h = InputHandler::new();
    let ev = key_press(30); // 'a' keycode
    // Act
    h.push_event(ev.clone()).unwrap();
    let drained = h.drain_events();
    // Assert
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0], ev);
}

#[test]
fn drain_is_fifo_ordered() {
    // Arrange
    let mut h = InputHandler::new();
    let events = [key_press(30), key_release(30), pointer_move(10, 20)];
    for e in &events {
        h.push_event(e.clone()).unwrap();
    }
    // Act
    let drained = h.drain_events();
    // Assert — FIFO order preserved
    for (i, e) in events.iter().enumerate() {
        assert_eq!(&drained[i], e);
    }
}

#[test]
fn drain_empties_the_queue() {
    let mut h = InputHandler::new();
    h.push_event(key_press(1)).unwrap();
    h.drain_events();
    assert!(h.drain_events().is_empty());
}

// ── pointer tracking ──────────────────────────────────────────────────────────

#[test]
fn pointer_move_updates_position() {
    // Arrange
    let mut h = InputHandler::new();
    // Act
    h.push_event(pointer_move(300, 400)).unwrap();
    // Assert — position updated eagerly
    assert_eq!(h.pointer_pos(), (300, 400));
}

#[test]
fn multiple_pointer_moves_keep_latest() {
    let mut h = InputHandler::new();
    h.push_event(pointer_move(10, 20)).unwrap();
    h.push_event(pointer_move(50, 60)).unwrap();
    assert_eq!(h.pointer_pos(), (50, 60));
}

#[test]
fn non_move_events_do_not_change_pointer_pos() {
    let mut h = InputHandler::new();
    h.push_event(pointer_move(100, 200)).unwrap();
    h.push_event(key_press(30)).unwrap();
    assert_eq!(h.pointer_pos(), (100, 200));
}

// ── focus management ──────────────────────────────────────────────────────────

#[test]
fn set_focus_updates_focused_surface() {
    // Arrange
    let mut h = InputHandler::new();
    let sid = SurfaceId(7);
    // Act
    h.set_focus(Some(sid));
    // Assert
    assert_eq!(h.focused_surface(), Some(sid));
}

#[test]
fn clear_focus_sets_none() {
    let mut h = InputHandler::new();
    h.set_focus(Some(SurfaceId(1)));
    h.set_focus(None);
    assert_eq!(h.focused_surface(), None);
}

#[test]
fn focus_can_be_changed_multiple_times() {
    let mut h = InputHandler::new();
    h.set_focus(Some(SurfaceId(1)));
    h.set_focus(Some(SurfaceId(2)));
    assert_eq!(h.focused_surface(), Some(SurfaceId(2)));
}

// ── queue overflow ────────────────────────────────────────────────────────────

#[test]
fn overflow_at_256_events_returns_error() {
    // Arrange
    let mut h = InputHandler::new();
    for _ in 0..256 {
        h.push_event(key_press(1)).unwrap();
    }
    // Act — 257th event should fail
    let result = h.push_event(key_press(2));
    // Assert
    assert_eq!(result, Err(InputError::QueueOverflow));
}

// ── event variants ────────────────────────────────────────────────────────────

#[test]
fn push_all_event_variants_without_error() {
    let mut h = InputHandler::new();
    let events = [
        InputEvent::KeyPress { keycode: 1, modifiers: no_mods() },
        InputEvent::KeyRelease { keycode: 1, modifiers: no_mods() },
        InputEvent::PointerMove { x: 0, y: 0 },
        InputEvent::ButtonPress { button: 1, x: 0, y: 0 },
        InputEvent::ButtonRelease { button: 1, x: 0, y: 0 },
        InputEvent::TouchBegin { id: 0, x: 0, y: 0 },
        InputEvent::TouchUpdate { id: 0, x: 10, y: 10 },
        InputEvent::TouchEnd { id: 0 },
        InputEvent::Scroll { delta_x: 0, delta_y: -3 },
    ];
    for e in events {
        assert!(h.push_event(e).is_ok());
    }
}

// ── modifiers ─────────────────────────────────────────────────────────────────

#[test]
fn modifiers_default_all_false() {
    let m = Modifiers::default();
    assert!(!m.shift);
    assert!(!m.ctrl);
    assert!(!m.alt);
    assert!(!m.super_key);
}

#[test]
fn modifiers_can_be_set() {
    let m = Modifiers { shift: true, ctrl: true, alt: false, super_key: false };
    assert!(m.shift);
    assert!(m.ctrl);
    assert!(!m.alt);
}
