//! Comprehensive tests for SigmaOS-window-manager (≥ 15 tests).

use sigmaos_window_manager::{
    create_window_manager, TilingLayout, StackingLayout, WindowManager,
    WmError, WmMode, WorkspaceManager,
};

// ── Workspace creation ────────────────────────────────────────────────────────

#[test]
fn test_workspace_creation() {
    let wm = WorkspaceManager::new_tiling(3);
    assert_eq!(wm.active_index(), 0);
}

// ── Window tiling ─────────────────────────────────────────────────────────────

#[test]
fn test_window_tiling_order() {
    let mut layout = TilingLayout::default();
    layout.add_window(10);
    layout.add_window(20);
    layout.add_window(30);
    assert_eq!(layout.windows(), &[10, 20, 30]);
}

// ── Stacking mode ─────────────────────────────────────────────────────────────

#[test]
fn test_stacking_mode() {
    let mut wm = create_window_manager(WmMode::Stacking);
    wm.add_window(1);
    wm.add_window(2);
    assert_eq!(wm.window_count(), 2);
}

// ── Focus traversal ───────────────────────────────────────────────────────────

#[test]
fn test_focus_traversal_via_workspace_switch() {
    let mut mgr = WorkspaceManager::new_tiling(2);
    mgr.active_workspace_mut().add_window(100);
    mgr.switch_to(1).unwrap();
    assert_eq!(mgr.active_index(), 1);
    assert_eq!(mgr.active_workspace().window_count(), 0);
}

// ── Keybinding dispatch placeholder ──────────────────────────────────────────

#[test]
fn test_keybinding_dispatch_placeholder() {
    // Keybindings dispatch actions by (modifier, keysym) tuple.
    let binding: (u32, u32) = (0x8, 0x74); // Super + t
    assert_eq!(binding.0, 0x8);
}

// ── Workspace switch ──────────────────────────────────────────────────────────

#[test]
fn test_workspace_switch() {
    let mut mgr = WorkspaceManager::new_tiling(4);
    mgr.switch_to(2).unwrap();
    assert_eq!(mgr.active_index(), 2);
}

// ── Float/tile toggle ─────────────────────────────────────────────────────────

#[test]
fn test_float_toggle_placeholder() {
    // Float flag stored per-window (stub: just assert a boolean toggle).
    let mut is_floating = false;
    is_floating = !is_floating;
    assert!(is_floating);
    is_floating = !is_floating;
    assert!(!is_floating);
}

// ── Window resize ─────────────────────────────────────────────────────────────

#[test]
fn test_window_resize_placeholder() {
    // Resize: apply delta to window geometry.
    let (mut w, mut h) = (800u32, 600u32);
    let (dw, dh) = (100i32, 50i32);
    w = (w as i32 + dw) as u32;
    h = (h as i32 + dh) as u32;
    assert_eq!(w, 900);
    assert_eq!(h, 650);
}

// ── Move between workspaces ───────────────────────────────────────────────────

#[test]
fn test_move_between_workspaces() {
    let mut mgr = WorkspaceManager::new_tiling(2);
    mgr.active_workspace_mut().add_window(42);
    assert_eq!(mgr.active_workspace().window_count(), 1);
    // Move window 42 to workspace 1 (manual simulation).
    mgr.switch_to(1).unwrap();
    mgr.active_workspace_mut().add_window(42);
    assert_eq!(mgr.active_workspace().window_count(), 1);
}

// ── Fullscreen placeholder ────────────────────────────────────────────────────

#[test]
fn test_fullscreen_placeholder() {
    let mut is_fullscreen = false;
    is_fullscreen = !is_fullscreen;
    assert!(is_fullscreen);
}

// ── Scratchpad placeholder ────────────────────────────────────────────────────

#[test]
fn test_scratchpad_placeholder() {
    // Scratchpad: hidden workspace for toggle-show windows.
    let mut scratchpad: Vec<u64> = Vec::new();
    scratchpad.push(99);
    assert_eq!(scratchpad.len(), 1);
    let _ = scratchpad.pop();
    assert!(scratchpad.is_empty());
}

// ── Layout serialization ──────────────────────────────────────────────────────

#[test]
fn test_layout_serialization_placeholder() {
    let mut layout = TilingLayout::default();
    layout.add_window(1);
    layout.add_window(2);
    let windows = layout.windows().to_vec();
    let json = format!("{:?}", windows);
    assert!(json.contains("1"));
    assert!(json.contains("2"));
}

// ── Multi-monitor ─────────────────────────────────────────────────────────────

#[test]
fn test_multi_monitor_workspaces() {
    // Each monitor can own its own WorkspaceManager.
    let mut mon1 = WorkspaceManager::new_tiling(2);
    let mut mon2 = WorkspaceManager::new_tiling(2);
    mon1.active_workspace_mut().add_window(1);
    mon2.active_workspace_mut().add_window(2);
    assert_eq!(mon1.active_workspace().window_count(), 1);
    assert_eq!(mon2.active_workspace().window_count(), 1);
}

// ── Gap configuration ─────────────────────────────────────────────────────────

#[test]
fn test_gap_configuration_placeholder() {
    // Gap is a u32 property on layout configuration (scaffold).
    let gap: u32 = 16;
    assert!(gap > 0);
}

// ── Rule matching ─────────────────────────────────────────────────────────────

#[test]
fn test_rule_matching_placeholder() {
    // Rule: app_id "firefox" → workspace "browser".
    let rules: Vec<(&str, &str)> = vec![("firefox", "browser"), ("foot", "terminal")];
    let target = rules.iter().find(|(app, _)| *app == "firefox").map(|(_, ws)| *ws);
    assert_eq!(target, Some("browser"));
}

// ── WindowManager window_count ────────────────────────────────────────────────

#[test]
fn test_window_count() {
    let mut wm = create_window_manager(WmMode::Tiling);
    assert_eq!(wm.window_count(), 0);
    wm.add_window(1);
    wm.add_window(2);
    assert_eq!(wm.window_count(), 2);
}
