// Integration Test Suite for Sovereign Desktop Services and Toolkits Advancements V92 PR
// Verifying all 11 sub-engines, cross-distro PR gateway, and end-to-end integration workflows.

#[path = "../src/desktop/sovereign_desktop_toolkits_and_services_v92_pr.rs"]
mod sovereign_desktop_toolkits_and_services_v92_pr;

use sovereign_desktop_toolkits_and_services_v92_pr::*;

#[test]
fn test_gtk_qt_toolkit_engine_v92_integration() {
    let mut toolkit = SovereignGtkQtToolkitEngineV92::new();
    assert_eq!(toolkit.adw_color_scheme, AdwColorSchemeV92::PreferDark);
    assert_eq!(toolkit.qt_style, Qt6PlatformStyleV92::Kvantum);
    assert_eq!(toolkit.gtk_render_pipeline, GtkRenderPipelineV92::Vulkan);

    toolkit.set_accent_color("#e66100");
    assert_eq!(toolkit.active_accent_hex, "#e66100");

    let css = r#"
        @define-color accent_bg_color #e66100;
        window.background {
            background-color: #1e1e1e;
        }
        button.suggested-action {
            background-color: @accent_bg_color;
        }
        entry {
            border-radius: 6px;
        }
        headerbar {
            background-color: #2e2e2e;
        }
    "#;
    let qss = toolkit.convert_css_to_qss(css);
    assert!(qss.contains("/* Palette Variable: accent_bg_color -> #e66100 */"));
    assert!(qss.contains("QMainWindow, QDialog"));
    assert!(qss.contains("QPushButton[accent=\"true\"]"));
    assert!(qss.contains("QLineEdit"));
    assert!(qss.contains("QHeaderView"));

    let kvantum_config = toolkit.generate_kvantum_config();
    assert!(kvantum_config.contains("theme=SovereignDarkV92"));
    assert!(kvantum_config.contains("blur=true"));
    assert!(kvantum_config.contains("translucent=true"));
    assert!(kvantum_config.contains("shadow_size=12"));
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

    let xdm = SovereignDisplayManagerSuiteEngineV92::new(DisplayManagerKindV92::Xdm);
    let xdm_cfg = xdm.generate_dm_config();
    assert!(xdm_cfg.contains("DisplayManager.requestPort: 0"));

    // PAM authentication tests
    let pw_req = PamAuthRequestV92 {
        username: "sigma_user".to_string(),
        password: Some("sigma_pass_2026".to_string()),
        totp_token: None,
        fido2_response: None,
        service_name: "login".to_string(),
    };
    let pw_resp = gdm.authenticate_pam(&pw_req);
    assert!(pw_resp.authenticated);
    assert_eq!(pw_resp.uid, 1000);
    assert_eq!(pw_resp.default_shell, "/bin/zsh");

    let totp_req = PamAuthRequestV92 {
        username: "root".to_string(),
        password: None,
        totp_token: Some("123456".to_string()),
        fido2_response: None,
        service_name: "sudo".to_string(),
    };
    let totp_resp = gdm.authenticate_pam(&totp_req);
    assert!(totp_resp.authenticated);
    assert_eq!(totp_resp.uid, 0);

    let fido_req = PamAuthRequestV92 {
        username: "sigma_user".to_string(),
        password: None,
        totp_token: None,
        fido2_response: Some(vec![0x01, 0x02, 0x03]),
        service_name: "polkit-1".to_string(),
    };
    let fido_resp = gdm.authenticate_pam(&fido_req);
    assert!(fido_resp.authenticated);

    let invalid_req = PamAuthRequestV92 {
        username: "nonexistent".to_string(),
        password: Some("pass".to_string()),
        totp_token: None,
        fido2_response: None,
        service_name: "login".to_string(),
    };
    let invalid_resp = gdm.authenticate_pam(&invalid_req);
    assert!(!invalid_resp.authenticated);
}

#[test]
fn test_gvfs_gio_engine_v92_integration() {
    let mut gvfs = SovereignGvfsGioFileSystemEngineV92::new();
    assert!(gvfs.network_connected);

    let smb_id = gvfs.mount_location(MountSchemeV92::Smb, "nas.local", "media");
    let sftp_id = gvfs.mount_location(MountSchemeV92::Sftp, "dev.server", "var/www");
    let webdav_id = gvfs.mount_location(MountSchemeV92::WebDav, "nextcloud.local", "remote.php/webdav");

    assert_eq!(gvfs.active_mounts.len(), 3);
    assert!(gvfs.active_mounts.contains_key(&smb_id));
    assert!(gvfs.active_mounts.contains_key(&sftp_id));
    assert!(gvfs.active_mounts.contains_key(&webdav_id));

    // dconf database integration
    gvfs.set_dconf_value("/org/gnome/desktop/interface/font-name", "Cantarell 11");
    gvfs.set_dconf_value("/org/gnome/desktop/wm/preferences/button-layout", "appmenu:close");

    assert_eq!(
        gvfs.get_dconf_value("/org/gnome/desktop/interface/font-name"),
        Some(&"Cantarell 11".to_string())
    );
    assert_eq!(
        gvfs.get_dconf_value("/org/gnome/desktop/wm/preferences/button-layout"),
        Some(&"appmenu:close".to_string())
    );

    assert!(gvfs.unmount_id(&smb_id));
    assert_eq!(gvfs.active_mounts.len(), 2);
}

#[test]
fn test_polkit_and_portals_v92_integration() {
    let mut polkit = SovereignPolicyKitAuthorizationEngineV92::new();
    assert!(polkit.evaluate_js_rule("org.freedesktop.policykit.exec", "root"));
    assert!(!polkit.evaluate_js_rule("org.freedesktop.policykit.exec", "regular_user"));
    assert!(polkit.evaluate_js_rule("org.freedesktop.packagekit.system-update", "regular_user"));

    let pkla_data = r#"
        [Allow Admin Users]
        Identity=unix-group:wheel;unix-group:sudo
        Action=org.freedesktop.NetworkManager.*
        ResultAny=yes
        ResultInactive=no
        ResultActive=yes

        [Allow PackageKit Updates]
        Identity=unix-group:users
        Action=org.freedesktop.packagekit.system-update
        ResultAny=yes
    "#;
    let pkla_rules = polkit.parse_pkla_file(pkla_data);
    assert_eq!(pkla_rules, 2);

    let mut portals = SovereignXdgPortalsUtilitiesEngineV92::new();
    let portals_conf = r#"
        # XDG Desktop Portals Configuration
        [preferred]
        default=gtk;hyprland
        org.freedesktop.impl.portal.FileChooser=gtk
        org.freedesktop.impl.portal.ScreenCast=hyprland
    "#;
    let parsed_confs = portals.parse_portals_conf(portals_conf);
    assert_eq!(parsed_confs, 3);
    assert_eq!(
        portals.portals_config.get("org.freedesktop.impl.portal.FileChooser"),
        Some(&"gtk".to_string())
    );

    let user_dirs_content = r#"
        XDG_DESKTOP_DIR="$HOME/Desktop"
        XDG_DOWNLOAD_DIR="$HOME/Downloads"
        XDG_TEMPLATES_DIR="$HOME/Templates"
        XDG_PUBLICSHARE_DIR="$HOME/Public"
        XDG_DOCUMENTS_DIR="$HOME/Documents"
        XDG_MUSIC_DIR="$HOME/Music"
        XDG_PICTURES_DIR="$HOME/Pictures"
        XDG_VIDEOS_DIR="$HOME/Videos"
    "#;
    let dir_count = portals.parse_user_dirs(user_dirs_content);
    assert_eq!(dir_count, 8);
    assert_eq!(portals.user_dirs.get("XDG_PICTURES_DIR"), Some(&"$HOME/Pictures".to_string()));

    let handler_msg = portals.open_uri_handler("file:///home/user/document.pdf");
    assert!(handler_msg.contains("file:///home/user/document.pdf"));
}

#[test]
fn test_font_and_text_layout_v92_integration() {
    let font_engine = SovereignFontTextLayoutEngineV92::new();
    assert_eq!(font_engine.font_family, "Cantarell");
    assert_eq!(font_engine.hinting_mode, FreeTypeHintingV92::Slight);
    assert!(font_engine.subpixel_antialiasing);

    let fontconfig_xml = font_engine.generate_fontconfig_xml();
    assert!(fontconfig_xml.contains("hintslight"));
    assert!(fontconfig_xml.contains("rgb"));
    assert!(fontconfig_xml.contains("lcddefault"));

    let shaped_glyphs = font_engine.shape_harfbuzz_text("SovereignOS", "Latn");
    assert_eq!(shaped_glyphs.len(), 11);
    for glyph in &shaped_glyphs {
        assert_eq!(glyph.x_advance, 640);
        assert_eq!(glyph.y_advance, 0);
    }
}

#[test]
fn test_input_method_v92_integration() {
    let mut fcitx5 = SovereignInputMethodFrameworkEngineV92::new(ImeEngineKindV92::Fcitx5);
    fcitx5.update_preedit("nihao", vec!["你好".to_string(), "拟好".to_string()]);
    assert_eq!(fcitx5.preedit_text, "nihao");
    assert_eq!(fcitx5.candidate_list.len(), 2);

    let commit_result = fcitx5.handle_text_input_v3_commit("你好");
    assert!(commit_result.contains("committed '你好'"));
    assert!(fcitx5.preedit_text.is_empty());
    assert!(fcitx5.candidate_list.is_empty());

    let ibus = SovereignInputMethodFrameworkEngineV92::new(ImeEngineKindV92::IBus);
    assert_eq!(ibus.active_kind, ImeEngineKindV92::IBus);
}

#[test]
fn test_notification_daemon_v92_integration() {
    let mut notif = SovereignNotificationDaemonEngineV92::new();
    let n1 = notif.notify("Browser", "Download Complete", "sigmaos_installer.iso", 1, 5000);
    let n2 = notif.notify("Battery", "Low Battery Warning", "15% remaining", 2, 0);

    assert_eq!(notif.active_notifications.len(), 2);
    assert_eq!(notif.active_notifications.get(&n1).unwrap().urgency, 1);
    assert_eq!(notif.active_notifications.get(&n2).unwrap().urgency, 2);

    assert!(notif.close_notification(n1));
    assert_eq!(notif.active_notifications.len(), 1);
    assert!(!notif.close_notification(n1));
}

#[test]
fn test_screen_locker_v92_integration() {
    let mut locker = SovereignScreenLockerEngineV92::new();
    assert!(!locker.is_locked);
    assert!(!locker.ext_session_lock_active);

    locker.set_password("correct_horse_battery_staple");
    locker.lock();
    assert!(locker.is_locked);
    assert!(locker.ext_session_lock_active);

    // Failed unlock attempts and exponential backoff
    assert!(!locker.unlock("wrong_password_1"));
    assert_eq!(locker.failed_attempts, 1);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 2); // 1 << 1

    assert!(!locker.unlock("wrong_password_2"));
    assert_eq!(locker.failed_attempts, 2);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 4); // 1 << 2

    // Successful unlock resets backoff
    assert!(locker.unlock("correct_horse_battery_staple"));
    assert!(!locker.is_locked);
    assert!(!locker.ext_session_lock_active);
    assert_eq!(locker.failed_attempts, 0);
    assert_eq!(locker.calculate_pam_backoff_seconds(), 0);
}

#[test]
fn test_clipboard_manager_v92_integration() {
    let mut clip = SovereignClipboardManagerEngineV92::new();
    clip.max_history_size = 5;

    clip.push_item("text/plain", b"item 1", false);
    clip.push_item("text/html", b"<p>item 2</p>", false);
    clip.push_item("text/plain", b"pass: topsecret_vault_key", true);
    clip.push_item("image/png", b"\x89PNG\r\n\x1a\n", false);
    clip.push_item("text/plain", b"token: sk-test-12345", true);

    assert_eq!(clip.history.len(), 5);

    let wiped = clip.wipe_secrets();
    assert_eq!(wiped, 2);
    assert_eq!(clip.history.len(), 3);
    for item in &clip.history {
        assert!(!item.is_secret);
    }
}

#[test]
fn test_distro_services_pr_gateway_v92_integration() {
    let mut pr_gateway = SovereignDesktopServicesV92PrGatewayEngine::new();
    let count = pr_gateway.generate_10_distro_prs();
    assert_eq!(count, 10);
    assert_eq!(pr_gateway.proposals.len(), 10);

    let targets: Vec<String> = pr_gateway.proposals.iter().map(|p| p.distro_target.clone()).collect();
    assert!(targets.contains(&"Ubuntu Desktop".to_string()));
    assert!(targets.contains(&"Fedora Workstation".to_string()));
    assert!(targets.contains(&"Arch Linux".to_string()));
    assert!(targets.contains(&"Debian GNU/Linux".to_string()));
    assert!(targets.contains(&"openSUSE Tumbleweed".to_string()));
    assert!(targets.contains(&"Gentoo Linux".to_string()));
    assert!(targets.contains(&"Alpine Linux".to_string()));
    assert!(targets.contains(&"FreeBSD Desktop".to_string()));
    assert!(targets.contains(&"Void Linux".to_string()));
    assert!(targets.contains(&"Manjaro Linux".to_string()));

    assert!(pr_gateway.audit_all_proposals());
}
