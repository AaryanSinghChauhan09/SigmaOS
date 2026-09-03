extern crate alloc;

use alloc::string::ToString;
use alloc::vec;

use sigmaos::distro_inspirations::{
    AppStreamModuleStream, BlackarchCategory, BlackarchRepository, BlackmanBuild,
    CaineForensicsEnforcer, ClearLinuxAutoOptimizer, ElevateMigration, FlatcarImmutableRootfs,
    ForensicsDevicePolicy, FormFactor, FreePolicyVerdict, GamescopeCompositor, InterfaceFlag,
    IsolationKind, KaliMetapackage, KaliToolGroup, MicroarchTier, NebraskaUpdateServer,
    PhoshConvergence, PressureVessel, PuppySaveSession, PureosFreePolicy, RancherOsCloudConfig,
    RaspiConfigTool, ReleaseChannel, RescuezillaBackupEngine, SaveMode, SigRepository,
    SteamABImageUpdate, TinyCoreExtensionManager, TorStreamIsolation, UpdateStrategy, WhonixSplit,
    WoofCeLayer, ZincatiUpdateAgent,
};

#[test]
fn blackarch_repository_organizes_tools_by_category() {
    let mut repo = BlackarchRepository::new();
    repo.add_tool(
        "aircrack-ng",
        vec![BlackarchCategory::Base, BlackarchCategory::Wireless],
        "wireless cracker",
    );
    repo.add_tool(
        "sqlmap",
        vec![BlackarchCategory::Base, BlackarchCategory::Webapp],
        "sql injection",
    );
    repo.add_tool(
        "hashcat",
        vec![BlackarchCategory::Base, BlackarchCategory::Cracker],
        "password cracker",
    );
    assert_eq!(repo.total_tools(), 3);
    assert!(repo
        .tools_in(BlackarchCategory::Webapp)
        .contains(&"sqlmap".to_string()));
    assert!(repo.validate_base_membership().is_ok());

    let build = BlackmanBuild::new(repo);
    assert_eq!(
        build.category_members("blackarch-wireless"),
        vec!["aircrack-ng".to_string()]
    );
}

#[test]
fn blackarch_rejects_tool_missing_base_group() {
    let mut repo = BlackarchRepository::new();
    repo.add_tool(
        "stray",
        vec![BlackarchCategory::Cracker],
        "missing base membership",
    );
    assert!(repo.validate_base_membership().is_err());
}

#[test]
fn whonix_enforces_stream_isolation_per_application() {
    let mut split = WhonixSplit::new();
    split.isolation = IsolationKind::SocksStream;
    split.bind_app("browser", 9050);
    split.bind_app("thunderbird", 9102);
    split.bind_app("git", 9104);
    assert!(split.all_streams_isolated());
    assert!(split.requires_stream_isolation());

    let mut iso = TorStreamIsolation::new();
    assert!(iso.add("browser", 9050).is_ok());
    assert!(iso.add("thunderbird", 9102).is_ok());
    // Two apps on the same port would share a circuit -> rejected.
    assert!(iso.add("git", 9050).is_err());
    assert_eq!(iso.distinct_circuits(), 2);
    assert_eq!(iso.circuit_for("browser"), Some(9050));
}

#[test]
fn whonix_detects_shared_socks_port() {
    let mut split = WhonixSplit::new();
    split.bind_app("browser", 9050);
    split.bind_app("other", 9050);
    assert!(!split.all_streams_isolated());
}

#[test]
fn pureos_free_policy_refuses_nonfree_license() {
    let policy = PureosFreePolicy::new();
    assert_eq!(policy.classify("MIT"), FreePolicyVerdict::Free);
    assert_eq!(policy.classify("Proprietary"), FreePolicyVerdict::NonFree);
    assert!(!policy.refuses(FreePolicyVerdict::Free));
    assert!(policy.refuses(FreePolicyVerdict::NonFree));

    let mut phosh = PhoshConvergence::new();
    assert!(!phosh.is_desktop_mode());
    phosh.attach_external_display();
    assert_eq!(phosh.form_factor, FormFactor::Desktop);
}

#[test]
fn nebraska_updates_target_outdated_channel() {
    let mut nebraska = NebraskaUpdateServer::new();
    nebraska.register("node-a", ReleaseChannel::Stable, "1.0");
    nebraska.register("node-b", ReleaseChannel::Stable, "1.1");
    nebraska.register("node-c", ReleaseChannel::Beta, "1.1");
    let outdated = nebraska.channel_outdated(ReleaseChannel::Stable, "1.1");
    assert_eq!(outdated, vec!["node-a".to_string()]);
    assert_eq!(nebraska.instance_count(), 3);

    let mut flatcar = FlatcarImmutableRootfs::new();
    assert!(flatcar.is_immutable());
    flatcar.mount_state();
    assert!(flatcar.state_mounted);
}

#[test]
fn zincati_is_rollback_aware_and_respects_strategy() {
    let mut agent = ZincatiUpdateAgent::new();
    agent.strategy = UpdateStrategy::Immediate;
    assert!(agent.accept("34.20210904.1.0").is_ok());
    // Re-applying the same version is refused (rollback awareness).
    assert!(agent.accept("34.20210904.1.0").is_err());

    agent.note_rollback_boot();
    assert!(agent.accept("34.20210901.1.0").is_err());

    let mut agent2 = ZincatiUpdateAgent::new();
    agent2.strategy = UpdateStrategy::Disabled;
    assert!(agent2.accept("x").is_err());
}

#[test]
fn steam_gamescope_and_ab_update_work() {
    let mut gc = GamescopeCompositor::new();
    assert!(gc.nested);
    gc.cap_frame_rate(60);
    assert_eq!(gc.effective_frame_rate(144), 60);
    assert_eq!(gc.effective_frame_rate(30), 30);

    let mut pv = PressureVessel::new();
    pv.push_layer("gamescope");
    pv.push_layer("vulkan");
    assert_eq!(pv.layer_count(), 2);
    assert!(pv.shader_cache_enabled);

    let mut ab = SteamABImageUpdate::new(2);
    assert_eq!(ab.stage_and_switch().unwrap(), 1);
    assert_eq!(ab.stage_and_switch().unwrap(), 0);
}

#[test]
fn rancheros_cloud_config_and_elevate_migration_work() {
    let mut cfg = RancherOsCloudConfig::new();
    assert!(cfg.has_dual_daemons());
    cfg.set("ssh_authorized_keys", "abc");
    assert_eq!(
        cfg.get("ssh_authorized_keys").map(|s: &alloc::string::String| s.as_str()),
        Some("abc")
    );

    let mut migrate = ElevateMigration::new("centos-7", "almalinux-8");
    migrate.add_readiness_check("repositories configured");
    assert!(migrate.preflight_ok());
    assert_eq!(migrate.target(), "centos-7 -> almalinux-8");
}

#[test]
fn centos_stream_sig_and_module_streams_work() {
    let sig = SigRepository::new("hyperscale");
    assert!(sig.enabled("hyperscale"));
    assert!(!sig.enabled("cloud-sig"));

    let mut module = AppStreamModuleStream::new("nodejs", "18");
    module.add_stream("20");
    assert!(module.switch_stream("20").is_ok());
    assert_eq!(module.active_stream, "20");
    assert!(module.switch_stream("99").is_err());
}

#[test]
fn kali_metapackage_groups_tools() {
    let mut top10 = KaliMetapackage::new(KaliToolGroup::Top10);
    top10.add("burpsuite");
    top10.add("sqlmap");
    assert_eq!(top10.member_count(), 2);
    assert_eq!(KaliToolGroup::Top10.name(), "kali-tools-top10");
    assert_eq!(KaliToolGroup::Everything.name(), "kali-tools-everything");
}

#[test]
fn raspi_config_and_puppy_save_session_work() {
    let mut config = RaspiConfigTool::new();
    config.enable(InterfaceFlag::Ssh);
    config.disable(InterfaceFlag::Camera);
    assert!(config.is_enabled(InterfaceFlag::Ssh));
    assert!(!config.is_enabled(InterfaceFlag::Camera));

    let mut save = PuppySaveSession::new(SaveMode::UsbSaveFile);
    assert!(!save.is_persisted());
    save.save_ram_to_disk();
    assert!(save.is_persisted());

    let mut woof = WoofCeLayer::new();
    woof.add_layer("base");
    woof.add_layer("drivers");
    assert_eq!(woof.layer_count(), 2);
}

#[test]
fn tiny_core_and_caine_forensics_work() {
    let mut tce = TinyCoreExtensionManager::new();
    assert!(tce
        .load_extension_on_demand("openssh.tcz", "/dev/loop0", true)
        .is_ok());
    assert!(tce.is_mounted("openssh.tcz"));
    assert_eq!(tce.mounted_count(), 1);

    let mut caine = CaineForensicsEnforcer::new(ForensicsDevicePolicy::StrictReadOnly);
    assert!(caine.intercept_io("/dev/sda", false).is_ok());
    assert!(caine.intercept_io("/dev/sda", true).is_err());
    assert_eq!(caine.audit_events_count(), 1);
}

#[test]
fn rescuezilla_and_clear_linux_auto_optimization_work() {
    let mut rescue = RescuezillaBackupEngine::new();
    rescue.create_partition_backup("root_clone", "/dev/nvme0n1p2", "zstd", 1024 * 1024 * 500);
    assert!(rescue.verify_image("root_clone"));

    let optimizer = ClearLinuxAutoOptimizer::detect_hardware(true, true);
    assert_eq!(optimizer.microarch_tier, MicroarchTier::X86_64V4);
    assert_eq!(optimizer.active_binary_suffix(), ".v4-avx512");
}
