// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Omarchy Linux Gap Closure V75 PR Suite

#[path = "../src/distro/omarchy_linux_missing_components_v75_pr.rs"]
mod omarchy_linux_missing_components_v75_pr;

use omarchy_linux_missing_components_v75_pr::*;

#[test]
fn test_omarchy_linux_v75_pr_suite_full_cycle() {
    let suite = OmarchyLinuxMissingComponentsV75MasterSuite::new();
    assert_eq!(suite.calculate_parity_score(), 100);

    let records = suite.generate_all_pr_records();
    assert_eq!(records.len(), 10, "Expected 10 Omarchy V75 PR records");

    for rec in &records {
        assert_eq!(rec.status, OmarchyPrStatusV75::Merged);
        assert!(!rec.title.is_empty());
        assert!(!rec.target_component.is_empty());
        assert!(!rec.diff_patch.is_empty());
    }

    let markdown_proposal = suite.generate_markdown_pr_proposal();
    assert!(markdown_proposal.contains("# PR Proposal: Sovereign Omarchy Linux Gap Closure V75"));
    assert!(markdown_proposal.contains("Parity Score: 100%"));
    assert!(markdown_proposal.contains("Hyprland Compositor & Tiling Layout Engine"));
    assert!(markdown_proposal.contains("Waybar Status HUD & Live CSS Module Engine"));
    assert!(markdown_proposal.contains("Ghostty / Alacritty GPU Terminal PTY Engine"));
    assert!(markdown_proposal.contains("Walker Fuzzy Search Launcher & Clipboard History"));
    assert!(markdown_proposal.contains("Wallust Dynamic Palette Extractor & GTK/Qt Theme Sync"));
    assert!(markdown_proposal.contains("Hyprlock PAM Biometric Locker & Hypridle Governor"));
    assert!(markdown_proposal.contains("Ananicy-cpp Auto-Nice Real-Time Priority Scheduler"));
    assert!(markdown_proposal.contains("Herdr AI Agent Workflow & Quickshell HUD Overlay"));
    assert!(markdown_proposal.contains("Fail-Closed Sudo Privilege Shield & Polkit Policy Gate"));
    assert!(markdown_proposal.contains("Wallust Chord Theme Navigator & Shortcut Engine"));
}

#[test]
fn test_omarchy_linux_v75_subsystem_engines() {
    let mut hypr = OmarchyHyprlandCompositorLayoutEngineV75::new();
    assert_eq!(hypr.active_layout, "dwindle");
    hypr.set_layout("master");
    assert_eq!(hypr.active_layout, "master");
    assert!(hypr.generate_hyprland_conf().contains("layout = master"));

    let waybar = OmarchyWaybarStatusHudEngineV75::new();
    assert!(waybar.render_config_json().contains("\"position\": \"top\""));

    let ghostty = OmarchyGhosttyPtyTerminalEngineV75::new();
    assert!(ghostty.render_ghostty_config().contains("JetBrainsMono"));

    let walker = OmarchyWalkerLauncherEngineV75::new();
    let res = walker.search_query("test");
    assert_eq!(res.len(), 1);

    let wallust = OmarchyWallustThemeSyncEngineV75::new();
    assert!(wallust.generate_wallust_colors_json().contains("#1e1e2e"));

    let hyprlock = OmarchyHyprlockPamSecurityEngineV75::new();
    assert!(hyprlock.generate_hyprlock_conf().contains("pam_module = hyprlock"));

    let ananicy = OmarchyAnanicyGameModeEngineV75::new();
    assert_eq!(ananicy.get_nice_value("hyprland"), -15);

    let herdr = OmarchyHerdrAiSchedulerEngineV75::new();
    assert!(herdr.dispatch_ai_task("analyze").contains("Herdr AI Agent"));

    let sudo_shield = OmarchyFailClosedSudoPrivilegeShieldEngineV75::new();
    assert!(sudo_shield.validate_privilege_request("root", "ls -la"));
    assert!(!sudo_shield.validate_privilege_request("guest", "ls -la"));

    let chord = OmarchyWallustChordThemeNavigatorEngineV75::new();
    assert_eq!(
        chord.trigger_chord("SUPER+SHIFT+T"),
        Some("wallust next-theme".to_string())
    );
}
