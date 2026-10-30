// SPDX-License-Identifier: MIT
// Integration Test for Omarchy Linux Missing Components V84 PR Gateway Engine
// (`tests/test_omarchy_linux_v84_pr.rs`)

#[path = "../src/distro/omarchy_linux_missing_components_v84_pr.rs"]
mod omarchy_linux_missing_components_v84_pr;

use omarchy_linux_missing_components_v84_pr::*;

#[test]
fn test_omarchy_linux_v84_integration_suite() {
    let suite = OmarchyLinuxMissingComponentsV84MasterSuite::new();
    let prs = suite.collect_all_prs();
    assert_eq!(prs.len(), 10);
    assert_eq!(suite.calculate_parity_score(), 100.0);

    for pr in &prs {
        assert!(pr.pr_id >= 8401 && pr.pr_id <= 8410);
        assert!(!pr.title.is_empty());
        assert_eq!(pr.status, OmarchyPrStatusV84::Validated);
    }

    // Verify individual component operations
    let mut hyprland = OmarchyHyprlandCompositorLayoutEngineV84::new();
    assert!(hyprland.set_layout("dwindle").contains("dwindle"));

    let waybar = OmarchyWaybarStatusHudEngineV84::new();
    assert!(waybar.reload_theme_css("/etc/waybar/style.css").contains("24 modules"));

    let mut ghostty = OmarchyGhosttyPtyTerminalEngineV84::new();
    assert!(ghostty.tune_font("JetBrains Mono", 13.0).contains("13.0pt"));

    let walker = OmarchyWalkerLauncherEngineV84::new();
    assert!(walker.search_query("terminal").contains("8192 cliphist entries"));

    let mut wallust = OmarchyWallustThemeSyncEngineV84::new();
    assert!(wallust.sync_wallpaper_and_palette("/path/wall.png", "catppuccin").contains("catppuccin"));

    let mut hyprlock = OmarchyHyprlockPamSecurityEngineV84::new();
    assert!(hyprlock.set_idle_timeout(300).contains("300s"));

    let mut ananicy = OmarchyAnanicyGameModeEngineV84::new();
    assert!(ananicy.toggle_gamemode(true).contains("true"));

    let herdr = OmarchyHerdrAiSchedulerEngineV84::new();
    assert!(herdr.trigger_ai_agent_workflow("analyze logs").contains("analyze logs"));

    let shield = OmarchyFailClosedSudoPrivilegeShieldEngineV84::new();
    assert!(shield.verify_sudo_request("admin", "systemctl restart sshd"));

    let chord = OmarchyWallustChordThemeNavigatorEngineV84::new();
    assert!(chord.trigger_chord_action("SUPER+ALT+L").contains("SUPER+ALT+L"));
}
