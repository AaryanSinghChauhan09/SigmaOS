// SPDX-License-Identifier: MIT
// Integration Test for Omarchy Linux Missing Components V82 PR Gateway Engine
// (`tests/test_omarchy_linux_v82_pr.rs`)

#[path = "../src/distro/omarchy_linux_missing_components_v82_pr.rs"]
mod omarchy_linux_missing_components_v82_pr;

use omarchy_linux_missing_components_v82_pr::*;

#[test]
fn test_omarchy_linux_v82_integration_suite() {
    let suite = OmarchyLinuxMissingComponentsV82MasterSuite::new();
    let prs = suite.collect_all_prs();
    assert_eq!(prs.len(), 10);
    assert_eq!(suite.calculate_parity_score(), 100.0);

    for pr in &prs {
        assert!(pr.pr_id >= 8201 && pr.pr_id <= 8210);
        assert!(!pr.title.is_empty());
        assert_eq!(pr.status, OmarchyPrStatusV82::Validated);
    }

    // Verify individual component operations
    let mut hyprland = OmarchyHyprlandCompositorLayoutEngineV82::new();
    assert!(hyprland.set_layout("dwindle").contains("dwindle"));

    let waybar = OmarchyWaybarStatusHudEngineV82::new();
    assert!(waybar.reload_theme_css("/etc/waybar/style.css").contains("20 modules"));

    let mut ghostty = OmarchyGhosttyPtyTerminalEngineV82::new();
    assert!(ghostty.tune_font("JetBrains Mono", 12.0).contains("12.0pt"));

    let walker = OmarchyWalkerLauncherEngineV82::new();
    assert!(walker.search_query("terminal").contains("4096 cliphist entries"));

    let mut wallust = OmarchyWallustThemeSyncEngineV82::new();
    assert!(wallust.sync_wallpaper_and_palette("/path/wall.png", "catppuccin").contains("catppuccin"));

    let mut hyprlock = OmarchyHyprlockPamSecurityEngineV82::new();
    assert!(hyprlock.set_idle_timeout(300).contains("300s"));

    let mut ananicy = OmarchyAnanicyGameModeEngineV82::new();
    assert!(ananicy.toggle_gamemode(true).contains("true"));

    let herdr = OmarchyHerdrAiSchedulerEngineV82::new();
    assert!(herdr.trigger_ai_agent_workflow("analyze logs").contains("analyze logs"));

    let shield = OmarchyFailClosedSudoPrivilegeShieldEngineV82::new();
    assert!(shield.verify_sudo_request("admin", "systemctl restart sshd"));

    let chord = OmarchyWallustChordThemeNavigatorEngineV82::new();
    assert!(chord.trigger_chord_action("SUPER+ALT+L").contains("SUPER+ALT+L"));
}
