// Integration Test Suite for Sovereign Desktop Services and Toolkits Advancements V92 PR
// Verifying all 11 sub-engines, master gateway, and PR proposals

#[path = "../src/desktop/sovereign_desktop_toolkits_and_services_v92_pr.rs"]
mod sovereign_desktop_toolkits_and_services_v92_pr;

use sovereign_desktop_toolkits_and_services_v92_pr::*;

#[test]
fn test_gtk_qt_toolkit_integration_v92() {
    let mut toolkit = SovereignGtkQtToolkitEngineV92::new();
    assert_eq!(toolkit.gtk_render_pipeline, GtkRenderPipelineV92::Vulkan);
    assert_eq!(toolkit.adw_color_scheme, AdwColorSchemeV92::PreferDark);

    toolkit.set_accent_color("#e66100");
    assert_eq!(toolkit.active_accent_hex, "#e66100");

    let css = r#"
        @define-color accent_bg_color #e66100;
        window.background { background-color: #121212; }
        button.suggested-action { border-radius: 6px; }
        entry { background-color: #242424; }
        headerbar { background-color: #1e1e1e; }
    "#;
    let qss = toolkit.convert_css_to_qss(css);
    assert!(qss.contains("Palette Variable: accent_bg_color -> #e66100"));
    assert!(qss.contains("QMainWindow, QDialog"));
    assert!(qss.contains("QPushButton[accent=\"true\"]"));
    assert!(qss.contains("QLineEdit"));
    assert!(qss.contains("QHeaderView"));

    let kv_config = toolkit.generate_kvantum_config();
    assert!(kv_config.contains("theme=SovereignDarkV92"));
    assert!(kv_config.contains("blur=true"));
    assert!(kv_config.contains("translucent=true"));
}

#[test]
fn test_display_manager_suite_integration_v92() {
    let mut gdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Gdm);
    gdm.auto_login_user = Some("desktop_tester".to_string());
    gdm.xdmcp_enabled = false;
    let gdm_cfg = gdm.generate_dm_config();
    assert!(gdm_cfg.contains("AutomaticLoginEnable=true"));
    assert!(gdm_cfg.contains("AutomaticLogin=desktop_tester"));

    let mut sddm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Sddm);
    sddm.auto_login_user = Some("plasma_user".to_string());
    let sddm_cfg = sddm.generate_dm_config();
    assert!(sddm_cfg.contains("User=plasma_user"));
    assert!(sddm_cfg.contains("Theme"));

    let mut lightdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::LightDm);
    lightdm.auto_login_user = Some("light_user".to_string());
    let lightdm_cfg = lightdm.generate_dm_config();
    assert!(lightdm_cfg.contains("autologin-user=light_user"));

    let mut xdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Xdm);
    xdm.xdmcp_enabled = true;
    let xdm_cfg = xdm.generate_dm_config();
    assert!(xdm_cfg.contains("DisplayManager.requestPort: 177"));

    let auth_valid = PamAuthRequestV92 {
        username: "sigma_user".to_string(),
        password: Some("secret_pass".to_string()),
        totp_token: None,
        fido2_response: None,
        service_name: "gdm-password".to_string(),
    };
    let resp = gdm.authenticate_pam(&auth_valid);
    assert!(resp.authenticated);
    assert_eq!(resp.uid, 1000);

    let auth_invalid = PamAuthRequestV92 {
        username: "nonexistent".to_string(),
        password: Some("pwd".to_string()),
        totp_token: None,
        fido2_response: None,
        service_name: "gdm-password".to_string(),
    };
    let resp_invalid = gdm.authenticate_pam(&auth_invalid);
    assert!(!resp_invalid.authenticated);
}

#[test]
fn test_gvfs_gio_virtual_filesystem_integration_v92() {
    let mut gvfs = SovereignGvfsGioFileSystemEngineV92::new();
    let smb_mount = gvfs.mount_location(MountSchemeV92::Smb, "nas.local", "public");
    let sftp_mount = gvfs.mount_location(MountSchemeV92::Sftp, "remote.server", "var_log");

    assert!(gvfs.active_mounts.contains_key(&smb_mount));
    assert!(gvfs.active_mounts.contains_key(&sftp_mount));
    assert_eq!(gvfs.active_mounts.len(), 2);

    gvfs.set_dconf_value("/org/gnome/desktop/interface/gtk-theme", "Adwaita-dark");
    assert_eq!(
        gvfs.get_dconf_value("/org/gnome/desktop/interface/gtk-theme"),
        Some(&"Adwaita-dark".to_string())
    );

    assert!(gvfs.unmount_id(&smb_mount));
    assert_eq!(gvfs.active_mounts.len(), 1);
}

#[test]
fn test_policykit_and_xdg_portals_integration_v92() {
    let mut polkit = SovereignPolicyKitAuthorizationEngineV92::new();
    assert!(polkit.evaluate_js_rule("org.freedesktop.packagekit.system-update", "desktop_user"));
    assert!(!polkit.evaluate_js_rule("org.freedesktop.policykit.exec", "desktop_user"));
    assert!(polkit.evaluate_js_rule("org.freedesktop.policykit.exec", "root"));

    let pkla = r#"
        [Allow Admin Tasks]
        Identity=unix-group:wheel
        Action=org.freedesktop.NetworkManager.*
        ResultAny=no
        ResultInactive=no
        ResultActive=yes
    "#;
    assert_eq!(polkit.parse_pkla_file(pkla), 1);

    let mut portals = SovereignXdgPortalsUtilitiesEngineV92::new();
    let conf = r#"
        [preferred]
        default=gtk;hyprland
        org.freedesktop.impl.portal.FileChooser=gtk
    "#;
    assert_eq!(portals.parse_portals_conf(conf), 2);
    assert_eq!(portals.portals_config.get("default"), Some(&"gtk;hyprland".to_string()));

    let dirs = r#"
        XDG_DESKTOP_DIR="$HOME/Desktop"
        XDG_DOCUMENTS_DIR="$HOME/Documents"
        XDG_DOWNLOAD_DIR="$HOME/Downloads"
        XDG_MUSIC_DIR="$HOME/Music"
    "#;
    assert_eq!(portals.parse_user_dirs(dirs), 4);
    assert_eq!(portals.user_dirs.get("XDG_MUSIC_DIR"), Some(&"$HOME/Music".to_string()));

    let handler_msg = portals.open_uri_handler("mailto:user@example.com");
    assert!(handler_msg.contains("user@example.com"));
}

#[test]
fn test_font_and_text_layout_integration_v92() {
    let mut layout = SovereignFontTextLayoutEngineV92::new();
    layout.font_family = "Fira Code".to_string();
    layout.hinting_mode = FreeTypeHintingV92::Full;
    layout.lcd_filter = "lcdlight".to_string();

    let fontconfig_xml = layout.generate_fontconfig_xml();
    assert!(fontconfig_xml.contains("<const>lcdlight</const>"));

    let glyphs = layout.shape_harfbuzz_text("Rust Desktop V92", "Latn");
    assert_eq!(glyphs.len(), 16);
    assert_eq!(glyphs[0].glyph_id, 'R' as u32);
    assert_eq!(glyphs[0].x_advance, 640);
}

#[test]
fn test_ime_notifications_screenlocker_clipboard_integration_v92() {
    let mut ime = SovereignInputMethodFrameworkEngineV92::new(ImeEngineKindV92::Fcitx5);
    ime.update_preedit("nihao", vec!["你好".to_string(), "拟好".to_string()]);
    assert_eq!(ime.preedit_text, "nihao");
    assert_eq!(ime.candidate_list.len(), 2);
    let commit_res = ime.handle_text_input_v3_commit("你好");
    assert!(commit_res.contains("你好"));
    assert!(ime.preedit_text.is_empty());

    let mut notif_daemon = SovereignNotificationDaemonEngineV92::new();
    let id1 = notif_daemon.notify("SigmaPortal", "System Ready", "All components operational", 1, 5000);
    let id2 = notif_daemon.notify("BatteryMonitor", "Low Battery", "15% remaining", 2, 0);
    assert_eq!(notif_daemon.active_notifications.len(), 2);
    assert!(notif_daemon.close_notification(id1));
    assert_eq!(notif_daemon.active_notifications.len(), 1);
    assert_eq!(notif_daemon.active_notifications.get(&id2).unwrap().urgency, 2);

    let mut locker = SovereignScreenLockerEngineV92::new();
    locker.set_password("sigma_secure_key");
    locker.lock();
    assert!(locker.is_locked);
    assert!(locker.ext_session_lock_active);
    assert!(!locker.unlock("incorrect"));
    assert_eq!(locker.failed_attempts, 1);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 2);
    assert!(locker.unlock("sigma_secure_key"));
    assert!(!locker.is_locked);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 0);

    let mut clip = SovereignClipboardManagerEngineV92::new();
    clip.push_item("text/plain", b"Sample copied text", false);
    clip.push_item("text/plain", b"auth_token_value", true);
    clip.push_item("image/png", &[137, 80, 78, 71], false);
    assert_eq!(clip.history.len(), 3);
    let wiped = clip.wipe_secrets();
    assert_eq!(wiped, 1);
    assert_eq!(clip.history.len(), 2);
}

#[test]
fn test_distro_pr_gateway_integration_v92() {
    let mut gateway = SovereignDesktopServicesV92PrGatewayEngine::new();
    let count = gateway.generate_10_distro_prs();
    assert_eq!(count, 10);
    assert!(gateway.audit_all_proposals());

    let targets: Vec<String> = gateway.proposals.iter().map(|p| p.distro_target.clone()).collect();
    assert!(targets.contains(&"Ubuntu Desktop".to_string()));
    assert!(targets.contains(&"Fedora Workstation".to_string()));
    assert!(targets.contains(&"Arch Linux".to_string()));
    assert!(targets.contains(&"FreeBSD Desktop".to_string()));
}
