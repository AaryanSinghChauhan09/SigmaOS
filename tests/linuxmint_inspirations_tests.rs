extern crate alloc;

use sigmaos::linuxmint_inspirations::{
    AdBlockLevel, BulkyRenamer, CaptainInstaller, DebPackage, FsFormat, HypnotixIptvPlayer,
    IsolationMode, LanWarpEngine, MintNannyFilter, NannyDecision, NetworkPolicy, ProviderType,
    PwaDisplayMode, RenameRule, ThingyRecentDocs, WebEngineKind, WebappCategory, WebappConfig,
    WebappManager, WebappNavigationMode, WebappProfileMode, WebappSecurityPolicy, WARP_AUTH_PORT,
    WARP_TRANSFER_PORT,
};

#[test]
fn warpinator_secure_lan_transfer() {
    let mut w = LanWarpEngine::new("sigma-desktop", "192.168.1.100", IsolationMode::Landlock);
    assert_eq!(WARP_TRANSFER_PORT, 42000);
    assert_eq!(WARP_AUTH_PORT, 42001);
    assert!(!w.secure_mode);
    assert_eq!(w.secure_mode_restrictions().len(), 3);

    assert!(w.set_group_code("my-sigma-lan-9").is_ok());
    assert!(w.secure_mode);
    assert!(w.discover_peer("laptop", "192.168.1.20", "my-sigma-lan-9"));
    assert!(!w.discover_peer("rogue", "192.168.1.99", "default-code"));
    assert_eq!(w.peer_count(), 1);

    w.compression_enabled = true;
    let payload = [0u8; 1000];
    let out = w.send_file("192.168.1.20", "report.txt", &payload);
    match out {
        sigmaos::linuxmint_inspirations::TransferOutcome::Completed { bytes } => {
            assert!(bytes < 1000, "compression should shrink payload");
        }
        _ => panic!("expected completed transfer"),
    }
}

#[test]
fn thingy_recent_and_favourites() {
    let mut t = ThingyRecentDocs::new(3);
    t.open("/home/u/a.txt", 1);
    t.open("/home/u/b.txt", 2);
    t.open("/home/u/c.txt", 3);
    t.open("/home/u/d.txt", 4);
    // Only the last 3 recent entries are retained.
    assert_eq!(t.recent().len(), 3);
    t.toggle_favourite("/home/u/b.txt");
    assert_eq!(t.favourites().len(), 1);
    assert_eq!(t.favourites()[0], "/home/u/b.txt");
}

#[test]
fn webapp_manager_registers_isolated_apps() {
    let mut m = WebappManager::new();
    m.add_webapp("Mail", "https://mail.example", WebEngineKind::Gecko);
    m.add_webapp("Docs", "docs.example", WebEngineKind::Chromium);
    assert_eq!(m.app_count(), 2);
    let docs = m.launch("Docs").unwrap();
    assert!(docs.url.starts_with("https://"));
}

#[test]
fn webapp_manager_advanced_innovations() {
    let mut m = WebappManager::new();
    let cfg = WebappConfig {
        name: "ProtonMail".into(),
        url: "proton.me".into(),
        engine: WebEngineKind::LibreWolf,
        category: WebappCategory::Office,
        nav_mode: WebappNavigationMode::AppFrameOnly,
        profile_mode: WebappProfileMode::IncognitoEphemeral,
        security_policy: WebappSecurityPolicy {
            network: NetworkPolicy::DomainRestricted(vec!["proton.me".into(), "protonmail.com".into()]),
            adblock: AdBlockLevel::Strict,
            capsicum_sandboxed: true,
            isolate_storage: true,
        },
        custom_user_agent: None,
        custom_css: Some("body { background-color: #000; }".into()),
        icon_path: Some("proton-icon".into()),
        isolated: true,
        desktop_shortcut: true,
        pinned: true,
        force_https: true,
    };

    m.add_webapp_full(cfg);
    assert_eq!(m.app_count(), 1);

    let app = m.launch("ProtonMail").unwrap();
    assert_eq!(app.url, "https://proton.me");
    assert_eq!(app.category, WebappCategory::Office);

    let cmd = m.generate_launch_command("ProtonMail").unwrap();
    assert!(cmd.contains("librewolf --profile"));
    assert!(cmd.contains("--private-window"));

    assert!(m.evaluate_domain_access("ProtonMail", "proton.me"));
    assert!(!m.evaluate_domain_access("ProtonMail", "google-analytics.com"));

    let json_export = m.export_config();
    assert!(json_export.contains("ProtonMail"));

    assert!(m.remove_webapp("ProtonMail"));
    assert_eq!(m.app_count(), 0);
}

#[test]
fn captain_deb_and_apt_url_install() {
    let mut c = CaptainInstaller::new();
    c.seed_repo(vec![
        DebPackage { name: "libfoo".into(), version: "1.0".into(), depends: vec![] },
        DebPackage { name: "app".into(), version: "2.0".into(), depends: vec!["libfoo".into()] },
    ]);
    assert!(c.install_from_apt_url("apt://app").is_ok());
    // Missing dependency blocks the .deb install.
    let missing = DebPackage { name: "x".into(), version: "1.0".into(), depends: vec!["no-such".into()] };
    assert!(c.install_deb(missing).is_err());
}

#[test]
fn hypnotix_ingests_m3u_by_country() {
    let mut h = HypnotixIptvPlayer::new();
    h.add_provider("Free-TV", ProviderType::M3uUrl, "https://iptv.example/feed.m3u", false);
    h.add_provider("Adult", ProviderType::XtreamApi, "https://x.example/api", true);
    h.select_free_provider();
    assert_eq!(h.provider_count(), 1);

    h.ingest_m3u("Free-TV", &["bbc#GB", "cnn#US", "dw#DE"]);
    assert_eq!(h.channels_by_country("GB").len(), 1);
    assert_eq!(h.channels.len(), 3);
}

#[test]
fn bulky_renames_multiple_files() {
    let mut b = BulkyRenamer::new();
    b.add_file("photo (1).jpg");
    b.add_file("photo (2).jpg");
    b.add_rule(RenameRule::FindReplace { find: " (".into(), replace: "_".into() });
    b.add_rule(RenameRule::FindReplace { find: ")".into(), replace: "".into() });
    let renamed = b.execute();
    assert_eq!(renamed.len(), 2);
    assert_eq!(renamed[0].renamed, "photo_1.jpg");
    assert_eq!(renamed[1].renamed, "photo_2.jpg");
}

#[test]
fn mint_nanny_allowlist_wins() {
    let mut n = MintNannyFilter::new();
    n.block("adult.example");
    n.allow("docs.example");
    assert_eq!(n.evaluate("https://media.adult.example/a"), NannyDecision::Block);
    assert_eq!(n.evaluate("https://docs.example/guide"), NannyDecision::Allow);
    // Allow-list is evaluated before the block-list.
    n.allow("safe.example");
    assert_eq!(n.evaluate("https://safe.example/resource"), NannyDecision::Allow);
}

#[test]
fn mint_stick_formats_and_restores() {
    let mut f = sigmaos::linuxmint_inspirations::MintStickFormatter::new();
    let dev = sigmaos::linuxmint_inspirations::UsbDevice {
        path: "/dev/sdb".into(),
        label: "BOOT".into(),
        size_mb: 8192,
        writable: true,
    };
    assert!(f.format(&dev, FsFormat::Fat32).is_ok());
    assert_eq!(f.format_history.len(), 1);
    let ro = sigmaos::linuxmint_inspirations::UsbDevice {
        path: "/dev/sdc".into(),
        label: "LOCKED".into(),
        size_mb: 1024,
        writable: false,
    };
    assert!(f.format(&ro, FsFormat::Ext4).is_err());
    assert!(f.restore_from_iso(&dev, 4096).is_ok());
    assert!(f.restore_from_iso(&dev, 99999).is_err());
}

#[test]
fn mint_welcome_and_config_hub_flow() {
    let mut w = sigmaos::linuxmint_inspirations::MintWelcomeFlow::new();
    w.mark_done("update");
    assert_eq!(w.remaining().len(), 3);
    assert!(!w.is_complete());

    let mut hub = sigmaos::linuxmint_inspirations::MintConfigHub::new();
    assert!(hub.set("update", "auto-refresh", "off"));
    assert_eq!(hub.get("update", "auto-refresh"), Some("off"));
    assert_eq!(hub.get("welcome", "onboarding-version"), Some("1"));
}

#[test]
fn xapp_theme_engine_follows_system() {
    let theme = sigmaos::linuxmint_inspirations::XAppThemeEngine::new();
    use sigmaos::linuxmint_inspirations::AppTheme;
    assert_eq!(theme.effective_theme(true), AppTheme::Dark);
    assert_eq!(theme.effective_theme(false), AppTheme::Light);
}
