// SPDX-License-Identifier: MIT
// Integration Test for Omarchy Linux Missing Components V78 PR Gateway Engine
// (`tests/test_omarchy_linux_v78_pr.rs`)

#[path = "../src/distro/omarchy_linux_missing_components_v78_pr.rs"]
mod omarchy_linux_missing_components_v78_pr;

use omarchy_linux_missing_components_v78_pr::*;

#[test]
fn test_v78_omarchy_linux_integration_full() {
    let suite = OmarchyLinuxMissingComponentsV78MasterSuite::new();
    let prs = suite.collect_all_prs();
    assert_eq!(prs.len(), 10);
    assert_eq!(suite.calculate_parity_score(), 100.0);

    let mut hyprland = OmarchyHyprlandCompositorLayoutEngineV78::new();
    assert!(hyprland.set_layout("dwindle").contains("dwindle"));

    let waybar = OmarchyWaybarStatusHudEngineV78::new();
    assert!(waybar.reload_theme_css("/etc/waybar/style.css").contains("style.css"));

    let mut ghostty = OmarchyGhosttyPtyTerminalEngineV78::new();
    assert!(ghostty.tune_font("JetBrains Mono", 11.5).contains("JetBrains Mono"));

    let walker = OmarchyWalkerLauncherEngineV78::new();
    assert!(walker.search_query("terminal").contains("terminal"));

    let mut wallust = OmarchyWallustThemeSyncEngineV78::new();
    assert!(wallust.sync_wallpaper_and_palette("/usr/share/bg.png", "catppuccin").contains("catppuccin"));

    let mut hyprlock = OmarchyHyprlockPamSecurityEngineV78::new();
    assert!(hyprlock.set_idle_timeout(300).contains("300s"));

    let mut ananicy = OmarchyAnanicyGameModeEngineV78::new();
    assert!(ananicy.toggle_gamemode(true).contains("true"));

    let herdr = OmarchyHerdrAiSchedulerEngineV78::new();
    assert!(herdr.trigger_ai_agent_workflow("launch session").contains("launch session"));

    let shield = OmarchyFailClosedSudoPrivilegeShieldEngineV78::new();
    assert!(shield.verify_sudo_request("user", "systemctl status"));

    let chord = OmarchyWallustChordThemeNavigatorEngineV78::new();
    assert!(chord.trigger_chord_action("SUPER+ALT+ENTER").contains("SUPER+ALT+ENTER"));
}
