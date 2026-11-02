// Integration Test Suite for Sovereign Desktop Environment & Window Manager Advancements V92 PR
// Verifying all 17 DE/WM architectures, master suite, and PR gateway

#[path = "../src/desktop/sovereign_de_wm_advancements_v92_pr.rs"]
mod sovereign_de_wm_advancements_v92_pr;

use sovereign_de_wm_advancements_v92_pr::*;

#[test]
fn test_gnome_advancements_v92_integration() {
    let mut gnome = GnomeAdvancementsV92Engine::new();
    assert!(gnome.mutter_direct_scanout);
    assert!(gnome.mutter_kms_lease_active);
    assert!(gnome.triple_buffering_enabled);
    assert_eq!(gnome.quick_settings_toggle_count, 18);
    assert!(gnome.evaluate_extension_sandbox("pop-shell@system76.com"));
    assert!(!gnome.evaluate_extension_sandbox("malicious_no_at_sign"));

    let scale_res = gnome.set_fractional_scale(1.75);
    assert_eq!(scale_res, Ok(1.75));
    assert!(gnome.set_fractional_scale(0.1).is_err());

    gnome.sync_libadwaita_theme("#99c1f1", true);
    assert_eq!(gnome.libadwaita_accent_color, "#99c1f1");
    assert!(gnome.libadwaita_high_contrast);

    let container = Rect::new(0, 0, 1920, 1080);
    let tiles = gnome.compute_tiled_window_bounds(container, 3);
    assert_eq!(tiles.len(), 3);
    assert_eq!(tiles[0].x, 8);
    assert_eq!(tiles[0].y, 8);
}

#[test]
fn test_kde_plasma_advancements_v92_integration() {
    let mut kde = KdePlasmaAdvancementsV92Engine::new();
    assert!(kde.kwin_vrr_adaptive_sync);
    assert!(kde.plasma6_icc_enabled);
    assert!(kde.wayland_explicit_sync);

    let mount_ok = kde.mount_plasma_vault("FinanceVault.gocryptfs");
    assert!(mount_ok.is_ok());
    assert!(mount_ok.unwrap().contains("FinanceVault.gocryptfs"));
    assert!(kde.mount_plasma_vault("NonExistentVault").is_err());

    kde.register_krunner_runner("passwords", "org.kde.krunner.passwords");
    let matches = kde.search_krunner("passwords");
    assert_eq!(matches.len(), 1);

    assert_eq!(kde.krunner_calculate("2 + 2"), Some(4.0));
    assert_eq!(kde.krunner_calculate("100 / 4"), Some(25.0));

    let tiles = kde.calculate_kwin_tile_grid(1920, 1080, 4);
    assert_eq!(tiles.len(), 4);
}

#[test]
fn test_xfce_and_mate_and_cinnamon_v92_integration() {
    let mut xfce = XfceAdvancementsV92Engine::new();
    let action_output = xfce.execute_thunar_action("Batch Rename Files", "/docs");
    assert!(action_output.contains("Batch Rename Files"));
    xfce.set_panel_preset("DockbarPreset");
    assert_eq!(xfce.panel_layout_preset, "DockbarPreset");
    assert!(xfce.export_xfconf_channel_xml("xfce4-panel").contains("xfce4-panel"));

    let mut mate = MateAdvancementsV92Engine::new();
    assert!(mate.marco_csd_toggle_enabled);
    assert!(!mate.toggle_csd_decorations());
    assert_eq!(mate.search_brisk_menu("menu").len(), 2);

    let cinn = CinnamonAdvancementsV92Engine::new();
    assert!(cinn.spices_sandbox_verified);
    assert_eq!(cinn.trigger_hot_corner(3), Ok("LockScreen"));
    assert!(cinn.trigger_hot_corner(10).is_err());
    assert!(cinn.run_nemo_action_script("meld").is_some());
}

#[test]
fn test_lxqt_and_lxde_and_budgie_and_moksha_v92_integration() {
    let mut lxqt = LxqtAdvancementsV92Engine::new();
    assert!(lxqt.toggle_qterminal_dropdown());
    assert!(lxqt.generate_qterminal_color_scheme().contains("DarkPalette=true"));

    let mut lxde = LxdeAdvancementsV92Engine::new();
    lxde.apply_gtk_theme("Breeze-Dark");
    assert_eq!(lxde.lxappearance_gtk_theme, "Breeze-Dark");
    assert!(lxde.generate_openbox_pipe_menu_xml().contains("Breeze-Dark"));

    let mut budgie = BudgieAdvancementsV92Engine::new();
    assert!(budgie.toggle_raven_sidebar());
    assert_eq!(budgie.build_raven_widget("calendar"), "RavenWidget:calendar");

    let mut moksha = EnlightenmentMokshaAdvancementsV92Engine::new();
    moksha.set_efl_scale(2.0);
    assert_eq!(moksha.efl_scale_factor, 2.0);
    assert_eq!(moksha.compile_evas_theme("solarized"), "EvasCompiledTheme:solarized.edj");
}

#[test]
fn test_tiling_wm_advancements_v92_integration() {
    // i3
    let mut i3 = I3AdvancementsV92Engine::new();
    let parent = Rect::new(0, 0, 1920, 1080);
    let (l, r) = i3.compute_split_geometry(parent);
    assert_eq!(l.width, 960);
    assert_eq!(r.width, 960);
    assert!(i3.process_i3_command("split v").contains("vertical"));
    let (t, b) = i3.compute_split_geometry(parent);
    assert_eq!(t.height, 540);
    assert_eq!(b.height, 540);

    // bspwm
    let mut bspwm = BspwmAdvancementsV92Engine::new();
    bspwm.preselect_node("south", 0.6);
    let (top_bsp, bot_bsp) = bspwm.calculate_bsp_split(parent);
    assert_eq!(top_bsp.height, (1080.0 * 0.6) as u32);
    assert_eq!(bot_bsp.height, 1080 - (1080.0 * 0.6) as u32);

    // dwm
    let mut dwm = DwmAdvancementsV92Engine::new();
    let (m, s) = dwm.calculate_layout_areas(parent, 3);
    assert_eq!(m.width, (1920.0 * 0.55) as u32);
    assert_eq!(s.len(), 3);
    assert!(dwm.set_master_factor(0.70).is_ok());

    // awesome
    let mut awesome = AwesomeAdvancementsV92Engine::new();
    assert_eq!(awesome.send_naughty_notification("Alert", "Build done"), 1);
    let geom = awesome.calculate_client_geometry(1920, 1080);
    assert_eq!(geom, (0, 24, 1920, 1056));

    // Hyprland
    let mut hypr = HyprlandAdvancementsV92Engine::new();
    assert!(hypr.glassmorphism_blur_enabled);
    hypr.enter_submap("resize");
    assert_eq!(hypr.active_submap, "resize");

    // Sway
    let mut sway = SwayAdvancementsV92Engine::new();
    assert!(!sway.swayidle_inhibitor_active);
    assert!(sway.toggle_idle_inhibitor());
    let workspace = sway.apply_gaps_to_workspace(parent);
    assert!(workspace.width < parent.width);

    // Openbox & Fluxbox
    let openbox = OpenboxAdvancementsV92Engine::new();
    assert!(openbox.generate_root_menu().contains("Openbox3"));
    let fluxbox = FluxboxAdvancementsV92Engine::new();
    assert!(fluxbox.attach_window_tab("term1", "term2").contains("attached as tab"));
}

#[test]
fn test_master_suite_and_pr_gateway_v92_integration() {
    let suite = SovereignDeWmAdvancementsSuiteV92::new();
    let audit = suite.audit_all_environments();
    assert_eq!(audit.len(), 17);

    assert!(audit.contains_key("GNOME"));
    assert!(audit.contains_key("KDE Plasma"));
    assert!(audit.contains_key("Xfce"));
    assert!(audit.contains_key("LXQt"));
    assert!(audit.contains_key("LXDE"));
    assert!(audit.contains_key("MATE"));
    assert!(audit.contains_key("Cinnamon"));
    assert!(audit.contains_key("Budgie"));
    assert!(audit.contains_key("Enlightenment"));
    assert!(audit.contains_key("i3"));
    assert!(audit.contains_key("bspwm"));
    assert!(audit.contains_key("dwm"));
    assert!(audit.contains_key("Openbox"));
    assert!(audit.contains_key("Fluxbox"));
    assert!(audit.contains_key("awesome"));
    assert!(audit.contains_key("Hyprland"));
    assert!(audit.contains_key("Sway"));

    let proposal = suite.generate_pr_proposal();
    assert!(proposal.contains("PR PROPOSAL: Sovereign DE & WM Advancements V92"));
    assert!(proposal.contains("PQC Dilithium5 Signature Valid: true"));
    assert!(proposal.contains("Covered Environments & Window Managers: 17"));

    let gateway: SovereignDeWmAdvancementsV92PrGatewayEngine = suite;
    assert!(gateway.pqc_dilithium5_signature_valid);
}
