//! Integration tests — tiling layout.

use sigmaos_window_manager::{TilingLayout, WmMode, create_window_manager};

#[test]
fn tiling_layout_preserves_order() {
    let mut layout = TilingLayout::default();
    for id in [3u64, 1, 2] {
        layout.add_window(id);
    }
    assert_eq!(layout.windows(), &[3, 1, 2]);
}

#[test]
fn tiling_remove_middle_window() {
    let mut layout = TilingLayout::default();
    layout.add_window(1);
    layout.add_window(2);
    layout.add_window(3);
    layout.remove_window(2).expect("window 2 must be present");
    assert_eq!(layout.windows(), &[1, 3]);
}

#[test]
fn tiling_remove_nonexistent_returns_error() {
    let mut layout = TilingLayout::default();
    layout.add_window(1);
    let result = layout.remove_window(99);
    assert!(result.is_err());
}

#[test]
fn create_tiling_wm_starts_empty() {
    let wm = create_window_manager(WmMode::Tiling);
    assert_eq!(wm.window_count(), 0);
}
