// SPDX-License-Identifier: MIT
// Integration Test Suite for Omarchy Linux Missing Components V74 PR Gateway
// (`tests/test_omarchy_linux_v74_pr.rs`)

#[path = "../src/distro/omarchy_linux_missing_components_v74_pr.rs"]
mod omarchy_linux_missing_components_v74_pr;

use omarchy_linux_missing_components_v74_pr::{
    OmarchyAnanicyGameModeEngineV74, OmarchyFailClosedSudoPrivilegeShieldEngineV74,
    OmarchyGhosttyPtyTerminalEngineV74, OmarchyHerdrAiSchedulerEngineV74,
    OmarchyHyprlandCompositorLayoutEngineV74, OmarchyHyprlockPamSecurityEngineV74,
    OmarchyLinuxMissingComponentsV74MasterSuite, OmarchyPrStatusV74, OmarchyWalkerLauncherEngineV74,
    OmarchyWallustChordThemeNavigatorEngineV74, OmarchyWallustThemeSyncEngineV74,
    OmarchyWaybarStatusHudEngineV74,
};

#[test]
fn test_omarchy_v74_master_suite() {
    let suite = OmarchyLinuxMissingComponentsV74MasterSuite::new();
    assert_eq!(suite.calculate_parity_score(), 100);

    let records = suite.generate_all_pr_records();
    assert_eq!(records.len(), 10);

    for record in &records {
        assert!(record.pr_id >= 7401 && record.pr_id <= 7410);
        assert_eq!(record.status, OmarchyPrStatusV74::Merged);
        assert!(!record.title.is_empty());
        assert!(!record.target_component.is_empty());
        assert!(!record.diff_patch.is_empty());
    }

    let proposal = suite.generate_markdown_pr_proposal();
    assert!(proposal.contains("Sovereign Omarchy Linux Gap Closure V74"));
    assert!(proposal.contains("Parity Score: 100%"));
}

#[test]
fn test_hyprland_layout_v74_engine() {
    let mut hypr = OmarchyHyprlandCompositorLayoutEngineV74::new();
    assert_eq!(hypr.active_layout, "dwindle");
    hypr.set_layout("master");
    assert_eq!(hypr.active_layout, "master");
    let conf = hypr.generate_hyprland_conf();
    assert!(conf.contains("layout = master"));
    assert!(conf.contains("workspace_swipe = true"));
}

#[test]
fn test_waybar_status_hud_v74_engine() {
    let waybar = OmarchyWaybarStatusHudEngineV74::new();
    let json = waybar.render_config_json();
    assert!(json.contains("\"position\": \"top\""));
    assert!(json.contains("hyprland/workspaces"));
}

#[test]
fn test_ghostty_terminal_v74_engine() {
    let ghostty = OmarchyGhosttyPtyTerminalEngineV74::new();
    let conf = ghostty.render_ghostty_config();
    assert!(conf.contains("JetBrainsMono Nerd Font"));
    assert!(conf.contains("gpu-acceleration = true"));
}

#[test]
fn test_walker_launcher_v74_engine() {
    let walker = OmarchyWalkerLauncherEngineV74::new();
    let default_results = walker.search_query("");
    assert_eq!(default_results.len(), 3);
    let matched_results = walker.search_query("firefox");
    assert_eq!(matched_results.len(), 1);
    assert!(matched_results[0].contains("firefox"));
}

#[test]
fn test_wallust_theme_sync_v74_engine() {
    let wallust = OmarchyWallustThemeSyncEngineV74::new();
    let json = wallust.generate_wallust_colors_json();
    assert!(json.contains("#1e1e2e"));
}

#[test]
fn test_hyprlock_pam_v74_engine() {
    let hyprlock = OmarchyHyprlockPamSecurityEngineV74::new();
    let conf = hyprlock.generate_hyprlock_conf();
    assert!(conf.contains("pam_module = hyprlock"));
    assert!(conf.contains("fingerprint = enabled"));
}

#[test]
fn test_ananicy_gamemode_v74_engine() {
    let ananicy = OmarchyAnanicyGameModeEngineV74::new();
    assert_eq!(ananicy.get_nice_value("steam"), -10);
    assert_eq!(ananicy.get_nice_value("hyprland"), -15);
    assert_eq!(ananicy.get_nice_value("unknown"), 0);
}

#[test]
fn test_herdr_ai_scheduler_v74_engine() {
    let herdr = OmarchyHerdrAiSchedulerEngineV74::new();
    let task_res = herdr.dispatch_ai_task("optimize window layouts");
    assert!(task_res.contains("Herdr AI Agent scheduled task"));
}

#[test]
fn test_sudo_privilege_shield_v74_engine() {
    let shield = OmarchyFailClosedSudoPrivilegeShieldEngineV74::new();
    assert!(shield.validate_privilege_request("root", "systemctl restart NetworkManager"));
    assert!(!shield.validate_privilege_request("user", "systemctl restart NetworkManager"));
}

#[test]
fn test_wallust_chord_navigator_v74_engine() {
    let chord = OmarchyWallustChordThemeNavigatorEngineV74::new();
    assert_eq!(
        chord.trigger_chord("SUPER+SHIFT+T"),
        Some("wallust next-theme".to_string())
    );
    assert_eq!(chord.trigger_chord("INVALID"), None);
}
