// SPDX-License-Identifier: MIT
// Integration Test for Omarchy Linux Missing Components V75 PR Gateway Engine
// (`tests/test_omarchy_linux_v75_pr.rs`)

#[path = "../src/distro/omarchy_linux_missing_components_v75_pr.rs"]
mod omarchy_linux_missing_components_v75_pr;

use omarchy_linux_missing_components_v75_pr::*;

#[test]
fn test_omarchy_v75_integration_suite() {
    let suite = OmarchyLinuxMissingComponentsV75MasterSuite::new();
    assert_eq!(suite.calculate_parity_score(), 100);

    let records = suite.generate_all_pr_records();
    assert_eq!(records.len(), 10);

    let proposal = suite.generate_markdown_pr_proposal();
    assert!(proposal.contains("Sovereign Omarchy Linux Gap Closure V75"));
    assert!(proposal.contains("100%"));

    let mut hypr = OmarchyHyprlandCompositorLayoutEngineV75::new();
    hypr.set_layout("master");
    assert_eq!(hypr.active_layout, "master");

    let waybar = OmarchyWaybarStatusHudEngineV75::new();
    assert!(waybar.render_config_json().contains("\"position\": \"top\""));

    let ghostty = OmarchyGhosttyPtyTerminalEngineV75::new();
    assert!(ghostty.render_ghostty_config().contains("JetBrainsMono"));

    let walker = OmarchyWalkerLauncherEngineV75::new();
    assert_eq!(walker.search_query("test").len(), 1);

    let wallust = OmarchyWallustThemeSyncEngineV75::new();
    assert!(wallust.generate_wallust_colors_json().contains("#1e1e2e"));

    let sudo_shield = OmarchyFailClosedSudoPrivilegeShieldEngineV75::new();
    assert!(sudo_shield.validate_privilege_request("root", "ls -la"));

    let ananicy = OmarchyAnanicyGameModeEngineV75::new();
    assert_eq!(ananicy.get_nice_value("hyprland"), -15);

    let chord = OmarchyWallustChordThemeNavigatorEngineV75::new();
    assert_eq!(
        chord.trigger_chord("SUPER+SHIFT+T"),
        Some("wallust next-theme".to_string())
    );
}
