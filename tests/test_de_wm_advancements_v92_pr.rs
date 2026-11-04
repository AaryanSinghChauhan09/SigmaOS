// Integration Test Suite for Sovereign Desktop Environment & Window Manager Advancements V92 PR
// Verifying all 17 DE/WM subsystems, master suite, and PR gateway

#[path = "../src/desktop/sovereign_de_wm_advancements_v92_pr.rs"]
mod sovereign_de_wm_advancements_v92_pr;

use sovereign_de_wm_advancements_v92_pr::*;

#[test]
fn test_gnome_advancements_integration_v92() {
    let mut gnome = GnomeAdvancementsV92Engine::new();
    assert!(gnome.mutter_direct_scanout);
    assert!(gnome.mutter_kms_lease_active);
    assert!(gnome.triple_buffering_enabled);
    assert_eq!(gnome.fractional_scale, 1.25);

    assert_eq!(gnome.set_fractional_scale(1.75).unwrap(), 1.75);
    assert!(gnome.set_fractional_scale(5.0).is_err());

    gnome.set_pop_shell_tiling("auto-tile", 10);
    assert_eq!(gnome.pop_shell_tiling_mode, "auto-tile");
    assert_eq!(gnome.pop_shell_active_gap, 10);

    gnome.sync_libadwaita_theme("#e01b24", true);
    assert_eq!(gnome.libadwaita_accent_color, "#e01b24");
    assert!(gnome.libadwaita_high_contrast);

    assert!(gnome.evaluate_extension_sandbox("dash-to-panel@jderose9.github.com"));

    let container = Rect::new(0, 0, 1920, 1080);
    let single = gnome.compute_tiled_window_bounds(container, 1);
    assert_eq!(single.len(), 1);
    assert_eq!(single[0].width, 1900);
    assert_eq!(single[0].height, 1060);

    let tiles = gnome.compute_tiled_window_bounds(container, 4);
    assert_eq!(tiles.len(), 4);
}

#[test]
fn test_kde_plasma_advancements_integration_v92() {
    let mut kde = KdePlasmaAdvancementsV92Engine::new();
    assert!(kde.kwin_vrr_adaptive_sync);
    assert!(kde.wayland_explicit_sync);
    assert_eq!(kde.plasma6_hdr_color_profile, "sRGB_D65_PQ");

    let mount_ok = kde.mount_plasma_vault("PersonalSecrets.cryfs");
    assert!(mount_ok.is_ok());
    assert!(kde.mount_plasma_vault("NonExistentVault").is_err());

    kde.register_krunner_runner("network", "org.kde.krunner.network");
    let matches = kde.search_krunner("network");
    assert!(!matches.is_empty());
    assert!(matches[0].contains("org.kde.krunner.network"));

    assert_eq!(kde.krunner_calculate("2 + 2"), Some(4.0));
    assert_eq!(kde.krunner_calculate("10 * 5"), Some(50.0));
    assert_eq!(kde.krunner_calculate("100 / 4"), Some(25.0));
    assert_eq!(kde.krunner_calculate("25 * 4"), Some(100.0));
    assert_eq!(kde.krunner_calculate("42.5"), Some(42.5));
    assert_eq!(kde.krunner_calculate("invalid * expr"), None);

    let tiles = kde.calculate_kwin_tile_grid(1920, 1080, 3);
    assert_eq!(tiles.len(), 3);
    assert_eq!(tiles[0].width, 960);

    let (bg, fg, accent) = kde.generate_breeze_palette();
    assert_eq!(bg, "#232629");
    assert_eq!(fg, "#fcfcfc");
    assert_eq!(accent, "#1d99f3");
}

#[test]
fn test_xfce_advancements_integration_v92() {
    let mut xfce = XfceAdvancementsV92Engine::new();
    assert_eq!(xfce.xfwm4_wayland_transition_state, "WaylandCompositorActive");
    assert!(xfce.xfce4_power_manager_dpms);

    let action_res = xfce.execute_thunar_action("Compress to Tarball", "/tmp/archive.tar.gz");
    assert!(action_res.contains("Compress to Tarball"));

    xfce.set_panel_preset("DockOnly");
    assert_eq!(xfce.panel_layout_preset, "DockOnly");

    let whisker = xfce.filter_whisker_menu("terminal");
    assert_eq!(whisker.len(), 1);

    let xml = xfce.export_xfconf_channel_xml("xfce4-panel");
    assert!(xml.contains("xfce4-panel"));
}

#[test]
fn test_lxqt_advancements_integration_v92() {
    let mut lxqt = LxqtAdvancementsV92Engine::new();
    assert!(lxqt.qt6_wayland_session_active);
    assert!(lxqt.pcmanfm_qt_spatial_grid);
    assert!(lxqt.idle_inhibitor_active);

    assert!(lxqt.toggle_qterminal_dropdown());
    assert!(!lxqt.toggle_qterminal_dropdown());

    let scheme = lxqt.generate_qterminal_color_scheme();
    assert!(scheme.contains("QTerminalColorScheme"));
    assert!(scheme.contains("DarkPalette=true"));
}

#[test]
fn test_lxde_advancements_integration_v92() {
    let mut lxde = LxdeAdvancementsV92Engine::new();
    assert!(lxde.lxpanel_config_parsed);
    assert!(lxde.pcmanfm_desktop_icons_visible);

    lxde.apply_gtk_theme("Numix-Frost");
    assert_eq!(lxde.lxappearance_gtk_theme, "Numix-Frost");

    let menu_xml = lxde.generate_openbox_pipe_menu_xml();
    assert!(menu_xml.contains("Numix-Frost"));
}

#[test]
fn test_mate_advancements_integration_v92() {
    let mut mate = MateAdvancementsV92Engine::new();
    assert!(mate.caja_python_extension_loaded);
    assert!(mate.atril_document_preview_active);

    assert!(!mate.toggle_csd_decorations());
    assert!(mate.toggle_csd_decorations());

    let brisk = mate.search_brisk_menu("system monitor");
    assert_eq!(brisk.len(), 1);
    assert!(brisk[0].contains("System Monitor"));
}

#[test]
fn test_cinnamon_advancements_integration_v92() {
    let cinn = CinnamonAdvancementsV92Engine::new();
    assert!(cinn.spices_sandbox_verified);
    assert!(cinn.xapp_status_icon_support);

    assert_eq!(cinn.trigger_hot_corner(0), Ok("ExpoOverview"));
    assert_eq!(cinn.trigger_hot_corner(1), Ok("ShowDesktop"));
    assert_eq!(cinn.trigger_hot_corner(2), Ok("ScaleWorkspace"));
    assert_eq!(cinn.trigger_hot_corner(3), Ok("LockScreen"));
    assert!(cinn.trigger_hot_corner(4).is_err());

    let script = cinn.run_nemo_action_script("Bulky");
    assert!(script.is_some());
    assert!(script.unwrap().contains("Batch Rename with Bulky"));
}

#[test]
fn test_budgie_advancements_integration_v92() {
    let mut budgie = BudgieAdvancementsV92Engine::new();
    assert!(budgie.wayland_magpie_compositor);
    assert_eq!(budgie.solus_accent_sync, "SolusTeal");

    assert!(budgie.toggle_raven_sidebar());
    assert!(!budgie.toggle_raven_sidebar());

    let widget = budgie.build_raven_widget("calendar");
    assert_eq!(widget, "RavenWidget:calendar");
}

#[test]
fn test_enlightenment_moksha_advancements_integration_v92() {
    let mut moksha = EnlightenmentMokshaAdvancementsV92Engine::new();
    assert!(moksha.efl_evas_physics_enabled);
    assert!(moksha.terminology_vt_emulator);

    moksha.set_efl_scale(2.0);
    assert_eq!(moksha.efl_scale_factor, 2.0);

    let theme = moksha.compile_evas_theme("solarized");
    assert_eq!(theme, "EvasCompiledTheme:solarized.edj");
}

#[test]
fn test_i3_advancements_integration_v92() {
    let mut i3 = I3AdvancementsV92Engine::new();
    assert!(i3.i3bar_status_active);

    let parent = Rect::new(0, 0, 1200, 800);
    let (l, r) = i3.compute_split_geometry(parent);
    assert_eq!(l.width, 600);
    assert_eq!(r.width, 600);

    let res_v = i3.process_i3_command("split v");
    assert!(res_v.contains("vertical"));
    let (t, b) = i3.compute_split_geometry(parent);
    assert_eq!(t.height, 400);
    assert_eq!(b.height, 400);

    let res_scratch = i3.process_i3_command("scratchpad show");
    assert!(res_scratch.contains("Scratchpad displayed"));
}

#[test]
fn test_bspwm_advancements_integration_v92() {
    let mut bspwm = BspwmAdvancementsV92Engine::new();
    assert_eq!(bspwm.focused_monitor, "DP-1");

    bspwm.preselect_node("south", 0.6);
    assert_eq!(bspwm.preselect_dir, "south");
    assert_eq!(bspwm.split_ratio, 0.6);

    let parent = Rect::new(0, 0, 1000, 1000);
    let (top, bottom) = bspwm.calculate_bsp_split(parent);
    assert_eq!(top.height, 600);
    assert_eq!(bottom.height, 400);

    bspwm.preselect_node("east", 0.3);
    let (left, right) = bspwm.calculate_bsp_split(parent);
    assert_eq!(left.width, 300);
    assert_eq!(right.width, 700);
}

#[test]
fn test_dwm_advancements_integration_v92() {
    let mut dwm = DwmAdvancementsV92Engine::new();
    assert!(dwm.window_swallow_patch_active);
    assert!(dwm.pertag_enabled);

    assert_eq!(dwm.set_master_factor(0.65).unwrap(), 0.65);
    assert!(dwm.set_master_factor(1.5).is_err());

    let screen = Rect::new(0, 0, 1000, 800);
    let (master, stack) = dwm.calculate_layout_areas(screen, 3);
    assert_eq!(master.width, 650);
    assert_eq!(stack.len(), 3);
}

#[test]
fn test_openbox_advancements_integration_v92() {
    let ob = OpenboxAdvancementsV92Engine::new();
    assert!(ob.menu_xml_parsed);
    assert!(ob.rc_xml_parsed);
    assert!(ob.ewmh_hints_active);

    let pipe = ob.execute_pipe_menu("/usr/bin/ob-places");
    assert!(pipe.contains("ob-places"));
}

#[test]
fn test_fluxbox_advancements_integration_v92() {
    let fb = FluxboxAdvancementsV92Engine::new();
    assert!(fb.tab_grouping_enabled);
    assert!(fb.fluxbox_remote_ipc);

    let tab = fb.attach_window_tab("Editor", "Terminal");
    assert!(tab.contains("attached as tab"));
}

#[test]
fn test_awesome_advancements_integration_v92() {
    let mut aw = AwesomeAdvancementsV92Engine::new();
    assert!(aw.multi_tag_rules_active);
    assert!(aw.lgi_lua_bindings_active);

    let count = aw.send_naughty_notification("Alert", "Disk Full");
    assert_eq!(count, 1);

    let (x, y, w, h) = aw.calculate_client_geometry(2560, 1440);
    assert_eq!((x, y, w, h), (0, 24, 2560, 1416));
}

#[test]
fn test_hyprland_advancements_integration_v92() {
    let mut hypr = HyprlandAdvancementsV92Engine::new();
    assert!(hypr.glassmorphism_blur_enabled);
    assert_eq!(hypr.wayland_layout_mode, "dwindle");
    assert!(hypr.hyprpm_plugin_manager);

    hypr.enter_submap("resize");
    assert_eq!(hypr.active_submap, "resize");
}

#[test]
fn test_sway_advancements_integration_v92() {
    let mut sway = SwayAdvancementsV92Engine::new();
    assert_eq!(sway.wayland_gaps_inner, 8);
    assert_eq!(sway.wayland_gaps_outer, 4);

    let screen = Rect::new(0, 0, 1920, 1080);
    let ws = sway.apply_gaps_to_workspace(screen);
    assert_eq!(ws.x, 12);
    assert_eq!(ws.y, 12);
    assert_eq!(ws.width, 1896);
    assert_eq!(ws.height, 1056);

    assert!(sway.toggle_idle_inhibitor());
    assert!(!sway.toggle_idle_inhibitor());
}

#[test]
fn test_master_suite_and_pr_gateway_v92() {
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
    assert!(proposal.contains("Covered Environments & Window Managers: 17"));
    assert!(suite.pqc_dilithium5_signature_valid);

    let default_suite = SovereignDeWmAdvancementsSuiteV92::default();
    assert_eq!(default_suite.audit_all_environments().len(), 17);
}

#[test]
fn test_rect_constructors_and_equality_v92() {
    let r1 = Rect::new(10, 20, 300, 400);
    let r2 = Rect { x: 10, y: 20, width: 300, height: 400 };
    assert_eq!(r1, r2);
}
