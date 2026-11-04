// Integration Test Suite for Sovereign Desktop Services and Toolkits Advancements V92 PR
// Verifying all 10 desktop sub-engines, master PR gateway with PQC Dilithium5 verification, and integration edge cases.

#[path = "../src/desktop/sovereign_desktop_toolkits_and_services_v92_pr.rs"]
mod sovereign_desktop_toolkits_and_services_v92_pr;

use sovereign_desktop_toolkits_and_services_v92_pr::*;

#[test]
fn test_gtk_qt_toolkit_sync_engine_v92_integration() {
    let mut engine = SovereignGtkQtToolkitEngineV92::new();
    assert_eq!(engine.adw_color_scheme, AdwColorSchemeV92::PreferDark);
    assert_eq!(engine.qt_style, Qt6PlatformStyleV92::Kvantum);
    assert_eq!(engine.gtk_render_pipeline, GtkRenderPipelineV92::Vulkan);

    engine.set_accent_color("#7856ff");
    assert_eq!(engine.active_accent_hex, "#7856ff");

    let css = r#"
        @define-color accent_color #7856ff;
        @define-color window_bg #1e1e2e;
        window.background {
            background-color: @window_bg;
        }
        button.suggested-action {
            background-color: @accent_color;
            border-radius: 6px;
        }
        entry {
            border-radius: 4px;
        }
        headerbar {
            background-color: #11111b;
        }
    "#;

    let qss = engine.convert_css_to_qss(css);
    assert!(qss.contains("/* QSS Converted from GTK4/GTK5 CSS Engine V92 */"));
    assert!(qss.contains("/* Palette Variable: accent_color -> #7856ff */"));
    assert!(qss.contains("QMainWindow, QDialog"));
    assert!(qss.contains("QPushButton[accent=\"true\"]"));
    assert!(qss.contains("QLineEdit"));
    assert!(qss.contains("QHeaderView"));

    let kv_config = engine.generate_kvantum_config();
    assert!(kv_config.contains("theme=SovereignDarkV92"));
    assert!(kv_config.contains("blur=true"));
    assert!(kv_config.contains("translucent=true"));
    assert!(kv_config.contains("svg_path=/usr/share/Kvantum/SovereignDarkV92/SovereignDarkV92.svg"));
    assert!(kv_config.contains("shadow_size=12"));
    assert!(kv_config.contains("corner_radius=8"));

    engine.kvantum_spec = None;
    assert_eq!(
        engine.generate_kvantum_config(),
        "[General]\ntheme=Default\n"
    );
}

#[test]
fn test_display_manager_suite_engine_v92_integration() {
    let mut gdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Gdm);
    gdm.auto_login_user = Some("sigma_user".to_string());
    gdm.xdmcp_enabled = true;
    let gdm_cfg = gdm.generate_dm_config();
    assert!(gdm_cfg.contains("AutomaticLoginEnable=true"));
    assert!(gdm_cfg.contains("AutomaticLogin=sigma_user"));
    assert!(gdm_cfg.contains("Enable=true"));

    let mut sddm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Sddm);
    sddm.auto_login_user = Some("sigma_user".to_string());
    let sddm_cfg = sddm.generate_dm_config();
    assert!(sddm_cfg.contains("User=sigma_user"));
    assert!(sddm_cfg.contains("Session=plasma-wayland"));
    assert!(sddm_cfg.contains("Current=SovereignSddmV92"));

    let mut lightdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::LightDm);
    lightdm.auto_login_user = Some("guest".to_string());
    let lightdm_cfg = lightdm.generate_dm_config();
    assert!(lightdm_cfg.contains("autologin-user=guest"));
    assert!(lightdm_cfg.contains("greeter-session=slick-greeter"));

    let greetd = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Greetd);
    let greetd_cfg = greetd.generate_dm_config();
    assert!(greetd_cfg.contains("command = \"agreeter --cmd /bin/sh\""));
    assert!(greetd_cfg.contains("user = \"greeter\""));

    let mut xdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Xdm);
    xdm.xdmcp_enabled = true;
    let xdm_cfg = xdm.generate_dm_config();
    assert!(xdm_cfg.contains("DisplayManager.requestPort: 177"));
    xdm.xdmcp_enabled = false;
    assert!(xdm
        .generate_dm_config()
        .contains("DisplayManager.requestPort: 0"));

    // PAM Authentications
    // 1. Password auth
    let pass_req = PamAuthRequestV92 {
        username: "sigma_user".to_string(),
        password: Some("correct_horse_battery_staple".to_string()),
        totp_token: None,
        fido2_response: None,
        service_name: "login".to_string(),
    };
    let pass_resp = gdm.authenticate_pam(&pass_req);
    assert!(pass_resp.authenticated);
    assert_eq!(pass_resp.uid, 1000);
    assert_eq!(pass_resp.gid, 1000);
    assert_eq!(pass_resp.home_dir, "/home/sigma_user");
    assert_eq!(pass_resp.default_shell, "/bin/zsh");

    // 2. TOTP token auth
    let totp_req = PamAuthRequestV92 {
        username: "sigma_user".to_string(),
        password: None,
        totp_token: Some("123456".to_string()),
        fido2_response: None,
        service_name: "sudo".to_string(),
    };
    let totp_resp = gdm.authenticate_pam(&totp_req);
    assert!(totp_resp.authenticated);

    // 3. FIDO2 challenge-response auth
    let fido_req = PamAuthRequestV92 {
        username: "root".to_string(),
        password: None,
        totp_token: None,
        fido2_response: Some(vec![0xAA, 0xBB, 0xCC, 0xDD]),
        service_name: "gdm-webauthn".to_string(),
    };
    let fido_resp = gdm.authenticate_pam(&fido_req);
    assert!(fido_resp.authenticated);
    assert_eq!(fido_resp.uid, 0);

    // 4. Missing credentials failure
    let missing_req = PamAuthRequestV92 {
        username: "sigma_user".to_string(),
        password: None,
        totp_token: None,
        fido2_response: None,
        service_name: "login".to_string(),
    };
    let missing_resp = gdm.authenticate_pam(&missing_req);
    assert!(!missing_resp.authenticated);
    assert!(missing_resp.failure_reason.is_some());

    // 5. Unknown user failure
    let unknown_req = PamAuthRequestV92 {
        username: "attacker".to_string(),
        password: Some("secret".to_string()),
        totp_token: None,
        fido2_response: None,
        service_name: "login".to_string(),
    };
    let unknown_resp = gdm.authenticate_pam(&unknown_req);
    assert!(!unknown_resp.authenticated);
    assert!(unknown_resp
        .failure_reason
        .unwrap()
        .contains("not recognized in PAM database"));

    // 6. Empty username failure
    let empty_req = PamAuthRequestV92 {
        username: "".to_string(),
        password: Some("pass".to_string()),
        totp_token: None,
        fido2_response: None,
        service_name: "login".to_string(),
    };
    assert!(!gdm.authenticate_pam(&empty_req).authenticated);
}

#[test]
fn test_gvfs_gio_virtual_filesystem_engine_v92_integration() {
    let mut gvfs = SovereignGvfsGioFileSystemEngineV92::new();
    assert!(gvfs.network_connected);

    let mount_schemes = [
        (MountSchemeV92::Smb, "nas.lan", "/volume1/media"),
        (MountSchemeV92::Sftp, "dev.internal", "/home/developer"),
        (
            MountSchemeV92::WebDav,
            "cloud.example.org",
            "/remote.php/webdav",
        ),
        (MountSchemeV92::Afc, "iphone-device-uuid", "/DCIM"),
        (
            MountSchemeV92::Mtp,
            "android-usb-id",
            "/Internal shared storage",
        ),
        (MountSchemeV92::Ftp, "ftp.gnu.org", "/gnu"),
        (MountSchemeV92::Archive, "system_backup.tar.zst", "/"),
        (MountSchemeV92::Trash, "localhost", "/trash"),
        (MountSchemeV92::Nfs, "nfs.internal", "/exports/data"),
        (MountSchemeV92::CephFs, "mon1.ceph.cluster", "/sigmafs"),
    ];

    let mut mount_ids = Vec::new();
    for (scheme, host, path) in &mount_schemes {
        let mid = gvfs.mount_location(*scheme, host, path);
        assert!(gvfs.active_mounts.contains_key(&mid));
        let m = gvfs.active_mounts.get(&mid).unwrap();
        assert!(m.is_mounted);
        assert!(m.bytes_free > 0);
        assert!(m.mount_path.starts_with("/run/user/1000/gvfs/"));
        mount_ids.push(mid);
    }
    assert_eq!(gvfs.active_mounts.len(), 10);

    for mid in &mount_ids {
        assert!(gvfs.unmount_id(mid));
    }
    assert!(gvfs.active_mounts.is_empty());
    assert!(!gvfs.unmount_id("non_existent_mount"));

    // dconf database manipulation
    gvfs.set_dconf_value("/org/gnome/desktop/interface/font-name", "Inter 10");
    gvfs.set_dconf_value("/org/gnome/desktop/interface/cursor-theme", "Adwaita");
    assert_eq!(
        gvfs.get_dconf_value("/org/gnome/desktop/interface/font-name"),
        Some(&"Inter 10".to_string())
    );
    assert_eq!(
        gvfs.get_dconf_value("/org/gnome/desktop/interface/cursor-theme"),
        Some(&"Adwaita".to_string())
    );
    assert_eq!(gvfs.get_dconf_value("/non/existent/key"), None);
}

#[test]
fn test_policykit_authorization_engine_v92_integration() {
    let mut polkit = SovereignPolicyKitAuthorizationEngineV92::new();
    assert_eq!(polkit.rules.len(), 3);

    // Root is universally authorized
    assert!(polkit.evaluate_js_rule("org.freedesktop.policykit.exec", "root"));
    assert!(polkit.evaluate_js_rule("org.freedesktop.packagekit.system-update", "root"));

    // Normal user authorized for packagekit and udisks active rules
    assert!(polkit.evaluate_js_rule("org.freedesktop.packagekit.system-update", "sigma_user"));
    assert!(polkit.evaluate_js_rule("org.freedesktop.udisks2.filesystem-mount", "sigma_user"));

    // Normal user rejected for admin-only exec
    assert!(!polkit.evaluate_js_rule("org.freedesktop.policykit.exec", "sigma_user"));

    // PKLA parser
    let pkla_content = r#"
        [NetworkManager Modify]
        Identity=unix-group:wheel
        Action=org.freedesktop.NetworkManager.modify
        ResultAny=no
        ResultInactive=no
        ResultActive=yes

        [Color Manager]
        Identity=unix-user:*
        Action=org.freedesktop.color-manager.create-device
        ResultAny=no
        ResultActive=yes
    "#;
    assert_eq!(polkit.parse_pkla_file(pkla_content), 2);
}

#[test]
fn test_xdg_portals_and_utilities_engine_v92_integration() {
    let mut portals = SovereignXdgPortalsUtilitiesEngineV92::new();
    assert_eq!(
        portals.user_dirs.get("XDG_DESKTOP_DIR"),
        Some(&"$HOME/Desktop".to_string())
    );
    assert_eq!(
        portals.user_dirs.get("XDG_DOWNLOAD_DIR"),
        Some(&"$HOME/Downloads".to_string())
    );
    assert_eq!(
        portals.user_dirs.get("XDG_DOCUMENTS_DIR"),
        Some(&"$HOME/Documents".to_string())
    );

    // portals.conf parser
    let portals_conf = r#"
        [preferred]
        default=gtk;hyprland
        org.freedesktop.impl.portal.FileChooser=gtk
        org.freedesktop.impl.portal.ScreenCast=hyprland
    "#;
    let conf_count = portals.parse_portals_conf(portals_conf);
    assert_eq!(conf_count, 3);
    assert_eq!(
        portals.portals_config.get("default"),
        Some(&"gtk;hyprland".to_string())
    );
    assert_eq!(
        portals
            .portals_config
            .get("org.freedesktop.impl.portal.FileChooser"),
        Some(&"gtk".to_string())
    );

    // user-dirs.dirs parser
    let user_dirs_conf = r#"
        # Custom user-dirs config
        XDG_MUSIC_DIR="$HOME/Music"
        XDG_PICTURES_DIR="$HOME/Pictures"
        XDG_VIDEOS_DIR="$HOME/Videos"
    "#;
    let dirs_count = portals.parse_user_dirs(user_dirs_conf);
    assert_eq!(dirs_count, 3);
    assert_eq!(
        portals.user_dirs.get("XDG_MUSIC_DIR"),
        Some(&"$HOME/Music".to_string())
    );
    assert_eq!(
        portals.user_dirs.get("XDG_PICTURES_DIR"),
        Some(&"$HOME/Pictures".to_string())
    );
    assert_eq!(
        portals.user_dirs.get("XDG_VIDEOS_DIR"),
        Some(&"$HOME/Videos".to_string())
    );

    let uri_handler = portals.open_uri_handler("mailto:admin@sigmaos.org");
    assert!(uri_handler
        .contains("Launching browser or default handler for URI: mailto:admin@sigmaos.org"));
}

#[test]
fn test_font_text_layout_engine_v92_integration() {
    let font_engine = SovereignFontTextLayoutEngineV92::new();
    assert_eq!(font_engine.font_family, "Cantarell");
    assert_eq!(font_engine.hinting_mode, FreeTypeHintingV92::Slight);
    assert!(font_engine.subpixel_antialiasing);

    let xml = font_engine.generate_fontconfig_xml();
    assert!(xml.contains("<?xml version=\"1.0\"?>"));
    assert!(
        xml.contains("<edit name=\"hintstyle\" mode=\"assign\"><const>hintslight</const></edit>")
    );
    assert!(xml.contains("<edit name=\"rgba\" mode=\"assign\"><const>rgb</const></edit>"));
    assert!(
        xml.contains("<edit name=\"lcdfilter\" mode=\"assign\"><const>lcddefault</const></edit>")
    );

    let shaped = font_engine.shape_harfbuzz_text("SigmaOS v92", "Latn");
    assert_eq!(shaped.len(), 11);
    for glyph in &shaped {
        assert_eq!(glyph.x_advance, 640);
        assert_eq!(glyph.y_advance, 0);
        assert_eq!(glyph.x_offset, 0);
        assert_eq!(glyph.y_offset, 0);
    }
}

#[test]
fn test_input_method_framework_engine_v92_integration() {
    let mut ime = SovereignInputMethodFrameworkEngineV92::new(ImeEngineKindV92::Fcitx5);
    assert_eq!(ime.active_kind, ImeEngineKindV92::Fcitx5);
    assert_eq!(ime.current_input_engine, "pinyin");

    ime.update_preedit("nihao", vec!["你好".to_string(), "拟好".to_string()]);
    assert_eq!(ime.preedit_text, "nihao");
    assert_eq!(ime.candidate_list.len(), 2);

    let commit_msg = ime.handle_text_input_v3_commit("你好");
    assert_eq!(commit_msg, "Wayland text-input-v3 committed '你好'");
    assert!(ime.preedit_text.is_empty());
    assert!(ime.candidate_list.is_empty());

    let ibus_ime = SovereignInputMethodFrameworkEngineV92::new(ImeEngineKindV92::IBus);
    assert_eq!(ibus_ime.active_kind, ImeEngineKindV92::IBus);
}

#[test]
fn test_notification_daemon_engine_v92_integration() {
    let mut daemon = SovereignNotificationDaemonEngineV92::new();
    assert!(daemon.sound_feedback_enabled);

    let n1 = daemon.notify(
        "Browser",
        "Download Complete",
        "sigmaos_install.iso finished.",
        1,
        5000,
    );
    let n2 = daemon.notify("Battery", "Battery Low", "15% remaining", 2, 0);
    assert_eq!(n1, 1);
    assert_eq!(n2, 2);
    assert_eq!(daemon.active_notifications.len(), 2);

    let notif1 = daemon.active_notifications.get(&n1).unwrap();
    assert_eq!(notif1.app_name, "Browser");
    assert_eq!(notif1.urgency, 1);
    assert_eq!(notif1.expire_timeout_ms, 5000);

    let notif2 = daemon.active_notifications.get(&n2).unwrap();
    assert_eq!(notif2.app_name, "Battery");
    assert_eq!(notif2.urgency, 2);

    assert!(daemon.close_notification(n1));
    assert_eq!(daemon.active_notifications.len(), 1);
    assert!(!daemon.close_notification(999));
}

#[test]
fn test_screen_locker_and_clipboard_engine_v92_integration() {
    let mut locker = SovereignScreenLockerEngineV92::new();
    assert!(!locker.is_locked);
    assert!(!locker.ext_session_lock_active);
    assert_eq!(locker.failed_attempts, 0);

    locker.lock();
    assert!(locker.is_locked);
    assert!(locker.ext_session_lock_active);

    // Failed attempt triggers backoff
    assert!(!locker.unlock("incorrect_password"));
    assert_eq!(locker.failed_attempts, 1);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 2);

    assert!(!locker.unlock("another_wrong_password"));
    assert_eq!(locker.failed_attempts, 2);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 4);

    // Successful unlock resets failure count
    assert!(locker.unlock("user_secure_pass"));
    assert!(!locker.is_locked);
    assert!(!locker.ext_session_lock_active);
    assert_eq!(locker.failed_attempts, 0);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 0);

    // Clipboard tests
    let mut clipboard = SovereignClipboardManagerEngineV92::new();
    assert_eq!(clipboard.max_history_size, 100);

    let i1 = clipboard.push_item("text/plain", b"Normal text snippet", false);
    let i2 = clipboard.push_item("text/plain", b"SuperSecretApiKey123", true);
    let i3 = clipboard.push_item("image/png", &[0x89, 0x50, 0x4E, 0x47], false);
    assert_eq!(i1, 1);
    assert_eq!(i2, 2);
    assert_eq!(i3, 3);
    assert_eq!(clipboard.history.len(), 3);

    // Wipe secrets
    let wiped = clipboard.wipe_secrets();
    assert_eq!(wiped, 1);
    assert_eq!(clipboard.history.len(), 2);
    assert!(!clipboard.history.iter().any(|item| item.is_secret));
}

#[test]
fn test_master_pr_gateway_v92_integration() {
    let mut gateway = SovereignDesktopServicesV92PrGatewayEngine::new();
    assert!(gateway.proposals.is_empty());

    let count = gateway.generate_10_distro_prs();
    assert_eq!(count, 10);
    assert_eq!(gateway.proposals.len(), 10);

    let distros_expected = [
        "Ubuntu Desktop",
        "Fedora Workstation",
        "Arch Linux",
        "Debian GNU/Linux",
        "openSUSE Tumbleweed",
        "Gentoo Linux",
        "Alpine Linux",
        "FreeBSD Desktop",
        "Void Linux",
        "Manjaro Linux",
    ];

    for (i, expected_distro) in distros_expected.iter().enumerate() {
        let p = &gateway.proposals[i];
        assert_eq!(p.pr_id, 92100 + i);
        assert_eq!(&p.distro_target, expected_distro);
        assert_eq!(p.status, "Ready for Merge");
        assert!(p.pqc_signature.starts_with("dilithium5_sig_v92_"));
    }

    assert!(gateway.audit_all_proposals());
}
