//! Wayland compositor initialisation tests.
//!
//! Follows the Arrange-Act-Assert pattern with deterministic, isolated tests
//! (FIRST: Fast, Isolated, Repeatable, Self-validating, Timely).

use sigmaos_desktop::{
    CompositorError, Rect, SurfaceBuffer, WaylandCompositor,
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_compositor() -> WaylandCompositor {
    WaylandCompositor::new("/run/user/1000/wayland-0")
}

fn full_hd() -> Rect {
    Rect::new(0, 0, 1920, 1080)
}

// ── init ──────────────────────────────────────────────────────────────────────

#[test]
fn compositor_starts_not_running() {
    // Arrange
    let c = make_compositor();
    // Assert
    assert!(!c.is_running());
}

#[test]
fn compositor_init_sets_running() {
    // Arrange
    let mut c = make_compositor();
    // Act
    c.init().expect("first init must succeed");
    // Assert
    assert!(c.is_running());
}

#[test]
fn compositor_double_init_returns_error() {
    // Arrange
    let mut c = make_compositor();
    c.init().unwrap();
    // Act
    let result = c.init();
    // Assert
    assert_eq!(result, Err(CompositorError::AlreadyInitialised));
}

#[test]
fn compositor_shutdown_clears_running() {
    // Arrange
    let mut c = make_compositor();
    c.init().unwrap();
    // Act
    c.shutdown();
    // Assert
    assert!(!c.is_running());
}

// ── socket path ───────────────────────────────────────────────────────────────

#[test]
fn compositor_exposes_socket_path() {
    let c = WaylandCompositor::new("/tmp/wayland-test");
    assert_eq!(c.socket_path(), "/tmp/wayland-test");
}

// ── surface lifecycle ─────────────────────────────────────────────────────────

#[test]
fn create_surface_returns_distinct_ids() {
    // Arrange
    let mut c = make_compositor();
    // Act
    let id1 = c.create_surface(full_hd());
    let id2 = c.create_surface(Rect::new(0, 0, 800, 600));
    // Assert — ids are unique
    assert_ne!(id1, id2);
}

#[test]
fn attach_buffer_to_valid_surface_succeeds() {
    // Arrange
    let mut c = make_compositor();
    let id = c.create_surface(Rect::new(0, 0, 4, 4));
    let buf = SurfaceBuffer {
        width: 4,
        height: 4,
        stride: 16,
        data: vec![0u8; 64],
    };
    // Act
    let result = c.attach_buffer(id, buf);
    // Assert
    assert!(result.is_ok());
}

#[test]
fn attach_buffer_to_unknown_surface_returns_error() {
    use sigmaos_desktop::SurfaceId;
    // Arrange
    let mut c = make_compositor();
    let buf = SurfaceBuffer { width: 1, height: 1, stride: 4, data: vec![0; 4] };
    // Act
    let result = c.attach_buffer(SurfaceId(999), buf);
    // Assert
    assert_eq!(result, Err(CompositorError::SurfaceNotFound(999)));
}

#[test]
fn destroy_surface_succeeds_for_known_surface() {
    // Arrange
    let mut c = make_compositor();
    let id = c.create_surface(full_hd());
    // Act
    let result = c.destroy_surface(id);
    // Assert
    assert!(result.is_ok());
}

#[test]
fn destroy_surface_fails_for_unknown_surface() {
    use sigmaos_desktop::SurfaceId;
    // Arrange
    let mut c = make_compositor();
    // Act
    let result = c.destroy_surface(SurfaceId(42));
    // Assert
    assert_eq!(result, Err(CompositorError::SurfaceNotFound(42)));
}

#[test]
fn render_frame_returns_ok_when_running() {
    // Arrange
    let mut c = make_compositor();
    c.init().unwrap();
    c.create_surface(full_hd());
    // Act
    let result = c.render_frame();
    // Assert
    assert!(result.is_ok());
}

// ── geometry helpers ──────────────────────────────────────────────────────────

#[test]
fn rect_contains_point_inside() {
    let r = Rect::new(10, 10, 100, 100);
    assert!(r.contains(50, 50));
}

#[test]
fn rect_does_not_contain_point_outside() {
    let r = Rect::new(10, 10, 100, 100);
    assert!(!r.contains(200, 200));
}

#[test]
fn rect_intersects_overlapping() {
    let a = Rect::new(0, 0, 100, 100);
    let b = Rect::new(50, 50, 100, 100);
    assert!(a.intersects(&b));
    assert!(b.intersects(&a));
}

#[test]
fn rect_does_not_intersect_adjacent() {
    let a = Rect::new(0, 0, 100, 100);
    let b = Rect::new(100, 0, 100, 100);
    assert!(!a.intersects(&b));
}
