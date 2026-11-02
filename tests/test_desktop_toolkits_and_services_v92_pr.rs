// Integration Test Suite for Sovereign Desktop Services and Toolkits Advancements V92 PR
// Verifying all 11 sub-engines across GTK, Qt, DMs, GVfs/GIO, PolicyKit, XDG utilities/portals,
// fonts, IBus/Fcitx, notification daemons, screen lockers, clipboard managers, and 10-distro PR gateway.

#[path = "../src/desktop/sovereign_desktop_toolkits_and_services_v92_pr.rs"]
mod sovereign_desktop_toolkits_and_services_v92_pr;

use sovereign_desktop_toolkits_and_services_v92_pr::*;

#[test]
fn test_gtk_qt_toolkit_v92_integration() {
    let mut engine = SovereignGtkQtToolkitEngineV92::new();
    assert_eq!(engine.adw_color_scheme, AdwColorSchemeV92::PreferDark);
    assert_eq!(engine.qt_style, Qt6PlatformStyleV92::Kvantum);
    assert_eq!(engine.gtk_render_pipeline, GtkRenderPipelineV92::Vulkan);

    engine.set_accent_color("#ff5500");
    assert_eq!(engine.active_accent_hex, "#ff5500");

    let css = r#"
        @define-color accent_color #ff5500;
        window.background {
            background-color: #121212;
            border-radius: 8px;
        }
        button.suggested-action {
            background-color: #ff5500;
        }
        entry {
            border-radius: 4px;
        }
        headerbar {
            background-color: #202020;
        }
    "#;
    let qss = engine.convert_css_to_qss(css);
    assert!(qss.contains("/* Palette Variable: accent_color -> #ff5500 */"));
    assert!(qss.contains("QMainWindow, QDialog"));
    assert!(qss.contains("QPushButton[accent=\"true\"]"));
    assert!(qss.contains("QLineEdit"));
    assert!(qss.contains("QHeaderView"));

    let kv_cfg = engine.generate_kvantum_config();
    assert!(kv_cfg.contains("theme=SovereignDarkV92"));
    assert!(kv_cfg.contains("blur=true"));
    assert!(kv_cfg.contains("translucent=true"));
}

#[test]
fn test_display_manager_v92_integration() {
    let mut gdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Gdm);
    gdm.auto_login_user = Some("desktop_tester".to_string());
    gdm.xdmcp_enabled = true;
    let gdm_cfg = gdm.generate_dm_config();
    assert!(gdm_cfg.contains("AutomaticLoginEnable=true"));
    assert!(gdm_cfg.contains("AutomaticLogin=desktop_tester"));
    assert!(gdm_cfg.contains("Enable=true"));

    let mut sddm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Sddm);
    sddm.auto_login_user = Some("desktop_tester".to_string());
    let sddm_cfg = sddm.generate_dm_config();
    assert!(sddm_cfg.contains("User=desktop_tester"));
    assert!(sddm_cfg.contains("Current=SovereignSddmV92"));

    let lightdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::LightDm);
    let lightdm_cfg = lightdm.generate_dm_config();
    assert!(lightdm_cfg.contains("greeter-session=slick-greeter"));

    let xdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Xdm);
    let xdm_cfg = xdm.generate_dm_config();
    assert!(xdm_cfg.contains("DisplayManager.requestPort: 0"));

    // PAM authentication check
    let pam_req_valid = PamAuthRequestV92 {
        username: "sigma_user".to_string(),
        password: Some("secret123".to_string()),
        totp_token: None,
        fido2_response: None,
        service_name: "login".to_string(),
    };
    let resp = gdm.authenticate_pam(&pam_req_valid);
    assert!(resp.authenticated);
    assert_eq!(resp.uid, 1000);
    assert_eq!(resp.home_dir, "/home/sigma_user");

    let pam_req_empty = PamAuthRequestV92 {
        username: String::new(),
        password: None,
        totp_token: None,
        fido2_response: None,
        service_name: "login".to_string(),
    };
    let resp_empty = gdm.authenticate_pam(&pam_req_empty);
    assert!(!resp_empty.authenticated);

    let pam_req_unknown = PamAuthRequestV92 {
        username: "unknown_person".to_string(),
        password: Some("pass".to_string()),
        totp_token: None,
        fido2_response: None,
        service_name: "login".to_string(),
    };
    let resp_unknown = gdm.authenticate_pam(&pam_req_unknown);
    assert!(!resp_unknown.authenticated);
}

#[test]
fn test_gvfs_gio_v92_integration() {
    let mut gvfs = SovereignGvfsGioFileSystemEngineV92::new();
    assert!(gvfs.network_connected);

    let smb_id = gvfs.mount_location(MountSchemeV92::Smb, "nas.local", "backup");
    let sftp_id = gvfs.mount_location(MountSchemeV92::Sftp, "remote.server", "var/www");
    assert_eq!(gvfs.active_mounts.len(), 2);
    assert!(gvfs.active_mounts.contains_key(&smb_id));
    assert!(gvfs.active_mounts.contains_key(&sftp_id));

    gvfs.set_dconf_value("/org/gnome/desktop/interface/font-name", "Cantarell 11");
    assert_eq!(
        gvfs.get_dconf_value("/org/gnome/desktop/interface/font-name"),
        Some(&"Cantarell 11".to_string())
    );

    assert!(gvfs.unmount_id(&smb_id));
    assert_eq!(gvfs.active_mounts.len(), 1);
    assert!(!gvfs.unmount_id("non_existent"));
}

#[test]
fn test_polkit_v92_integration() {
    let mut polkit = SovereignPolicyKitAuthorizationEngineV92::new();
    assert!(polkit.evaluate_js_rule("org.freedesktop.policykit.exec", "root"));
    assert!(!polkit.evaluate_js_rule("org.freedesktop.policykit.exec", "regular_user"));
    assert!(polkit.evaluate_js_rule("org.freedesktop.packagekit.system-update", "regular_user"));

    let pkla = r#"
        [Admin Rights]
        Identity=unix-group:wheel
        Action=org.freedesktop.udisks2.filesystem-mount-system
        ResultActive=yes
    "#;
    let count = polkit.parse_pkla_file(pkla);
    assert_eq!(count, 1);
}

#[test]
fn test_xdg_portals_v92_integration() {
    let mut portals = SovereignXdgPortalsUtilitiesEngineV92::new();
    let conf = r#"
        [preferred]
        default=gtk;hyprland
        org.freedesktop.impl.portal.FileChooser=gtk
    "#;
    let loaded = portals.parse_portals_conf(conf);
    assert_eq!(loaded, 2);
    assert_eq!(portals.portals_config.get("default"), Some(&"gtk;hyprland".to_string()));

    let user_dirs_text = r#"
        XDG_DESKTOP_DIR="$HOME/Desktop"
        XDG_DOWNLOAD_DIR="$HOME/Downloads"
        XDG_TEMPLATES_DIR="$HOME/Templates"
        XDG_PUBLICSHARE_DIR="$HOME/Public"
        XDG_DOCUMENTS_DIR="$HOME/Documents"
        XDG_MUSIC_DIR="$HOME/Music"
        XDG_PICTURES_DIR="$HOME/Pictures"
        XDG_VIDEOS_DIR="$HOME/Videos"
    "#;
    let parsed_dirs = portals.parse_user_dirs(user_dirs_text);
    assert_eq!(parsed_dirs, 8);
    assert_eq!(portals.user_dirs.get("XDG_PICTURES_DIR"), Some(&"$HOME/Pictures".to_string()));

    let open_res = portals.open_uri_handler("mailto:developer@sigmaos.org");
    assert!(open_res.contains("mailto:developer@sigmaos.org"));
}

#[test]
fn test_font_text_layout_v92_integration() {
    let font_engine = SovereignFontTextLayoutEngineV92::new();
    let xml = font_engine.generate_fontconfig_xml();
    assert!(xml.contains("<?xml version=\"1.0\"?>"));
    assert!(xml.contains("<edit name=\"hintstyle\" mode=\"assign\"><const>hintslight</const></edit>"));
    assert!(xml.contains("<edit name=\"lcdfilter\" mode=\"assign\"><const>lcddefault</const></edit>"));

    let shaped = font_engine.shape_harfbuzz_text("SigmaOS Desktop", "Latn");
    assert_eq!(shaped.len(), 15);
    for glyph in shaped {
        assert_eq!(glyph.x_advance, 640);
        assert_eq!(glyph.y_advance, 0);
    }
}

#[test]
fn test_ime_framework_v92_integration() {
    let mut ibus_engine = SovereignInputMethodFrameworkEngineV92::new(ImeEngineKindV92::IBus);
    ibus_engine.update_preedit("nihao", vec!["你好".to_string(), "擬好".to_string()]);
    assert_eq!(ibus_engine.preedit_text, "nihao");
    assert_eq!(ibus_engine.candidate_list.len(), 2);

    let commit_msg = ibus_engine.handle_text_input_v3_commit("你好");
    assert!(commit_msg.contains("committed '你好'"));
    assert!(ibus_engine.preedit_text.is_empty());
    assert!(ibus_engine.candidate_list.is_empty());

    let fcitx_engine = SovereignInputMethodFrameworkEngineV92::new(ImeEngineKindV92::Fcitx5);
    assert_eq!(fcitx_engine.active_kind, ImeEngineKindV92::Fcitx5);

    let wayland_ime = SovereignInputMethodFrameworkEngineV92::new(ImeEngineKindV92::WaylandTextInputV3);
    assert_eq!(wayland_ime.active_kind, ImeEngineKindV92::WaylandTextInputV3);
}

#[test]
fn test_notification_daemon_v92_integration() {
    let mut notif_daemon = SovereignNotificationDaemonEngineV92::new();
    assert!(notif_daemon.sound_feedback_enabled);

    let id1 = notif_daemon.notify("System", "Disk Warning", "Low disk space", 2, 5000);
    let _id2 = notif_daemon.notify("Chat", "Message Received", "Hey there!", 1, 3000);
    assert_eq!(notif_daemon.active_notifications.len(), 2);

    let n1 = notif_daemon.active_notifications.get(&id1).unwrap();
    assert_eq!(n1.urgency, 2);
    assert_eq!(n1.summary, "Disk Warning");

    assert!(notif_daemon.close_notification(id1));
    assert_eq!(notif_daemon.active_notifications.len(), 1);
    assert!(!notif_daemon.close_notification(9999));
}

#[test]
fn test_screen_locker_v92_integration() {
    let mut locker = SovereignScreenLockerEngineV92::new();
    assert!(!locker.is_locked);
    assert!(!locker.ext_session_lock_active);

    locker.lock();
    assert!(locker.is_locked);
    assert!(locker.ext_session_lock_active);

    assert_eq!(locker.calculate_pam_backoff_seconds(), 0);
    assert!(!locker.unlock("incorrect_password"));
    assert_eq!(locker.failed_attempts, 1);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 2);

    assert!(!locker.unlock("incorrect_password_2"));
    assert_eq!(locker.failed_attempts, 2);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 4);

    assert!(locker.unlock("user_secure_pass"));
    assert!(!locker.is_locked);
    assert!(!locker.ext_session_lock_active);
    assert_eq!(locker.failed_attempts, 0);
}

#[test]
fn test_clipboard_manager_v92_integration() {
    let mut clip = SovereignClipboardManagerEngineV92::new();
    assert_eq!(clip.max_history_size, 100);

    clip.push_item("text/plain", b"public text", false);
    clip.push_item("text/plain", b"password_token_value", true);
    clip.push_item("image/png", &[0x89, 0x50, 0x4e, 0x47], false);
    assert_eq!(clip.history.len(), 3);

    let wiped = clip.wipe_secrets();
    assert_eq!(wiped, 1);
    assert_eq!(clip.history.len(), 2);
    assert!(!clip.history.iter().any(|i| i.is_secret));
}

#[test]
fn test_10_distro_pr_gateway_v92_integration() {
    let mut pr_gateway = SovereignDesktopServicesV92PrGatewayEngine::new();
    let pr_count = pr_gateway.generate_10_distro_prs();
    assert_eq!(pr_count, 10);
    assert_eq!(pr_gateway.proposals.len(), 10);

    assert!(pr_gateway.audit_all_proposals());

    // PQC Dilithium5 signature verification
    let valid_signature = [0x42u8; 32];
    assert!(pr_gateway.verify_pqc_signature(92100, &valid_signature));
    assert!(pr_gateway.verify_pqc_signature(92109, &valid_signature));
    assert!(!pr_gateway.verify_pqc_signature(99999, &valid_signature));

    let invalid_short_signature = [0x42u8; 16];
    assert!(!pr_gateway.verify_pqc_signature(92100, &invalid_short_signature));
}
