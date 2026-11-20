//! Integration tests — workspace management.

use sigmaos_window_manager::WorkspaceManager;

#[test]
fn workspace_switch_valid() {
    let mut wm = WorkspaceManager::new_tiling(4);
    wm.switch_to(3).expect("workspace 3 should exist");
    assert_eq!(wm.active_index(), 3);
}

#[test]
fn workspace_switch_out_of_bounds_returns_error() {
    let mut wm = WorkspaceManager::new_tiling(2);
    let result = wm.switch_to(5);
    assert!(result.is_err());
}

#[test]
fn workspace_active_window_count() {
    let mut wm = WorkspaceManager::new_tiling(3);
    wm.active_workspace_mut().add_window(42);
    wm.active_workspace_mut().add_window(43);
    assert_eq!(wm.active_workspace().window_count(), 2);
}

#[test]
fn workspace_windows_isolated_between_workspaces() {
    let mut wm = WorkspaceManager::new_tiling(2);
    wm.active_workspace_mut().add_window(1);
    wm.switch_to(1).unwrap();
    // Workspace 1 should be empty even though workspace 0 has a window.
    assert_eq!(wm.active_workspace().window_count(), 0);
}
