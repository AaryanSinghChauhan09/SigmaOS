// SigmaOS Comprehensive OS Components Integration & Unit Test Suite
// Verifies sovereign subsystem capabilities, compatibility layers, drivers, security, and tools.

use sigmaos::community::toolkit::{
    CommunityHandbookCatalog, ReproduciblePackageRecipeManager,
    SecurityProfileTemplateStore,
};
use sigmaos::dashboard::statutory_compliance::StatutoryGovernanceLayer;
use sigmaos::filesystem::{
    Blake3BlockDeduplicationEngine, PfsType, PseudoFilesystemNamespace,
};
use sigmaos::memory::segmentation_paging::{
    AslrEntropyConfig, CpuRing, RandomizedAddressSpace, SegmentSelector, SegmentationPagingEngine,
    SpaceProtectionFlags,
};
use sigmaos::process::activity_manager::ActivityManager;
use sigmaos::security::{UnveilPermissions, UnveilState};
use sigmaos::tools::{
    AlmeidaCmosRtc, SovereignDpkgEtcher, SovereignIPCalculator, SovereignJsonPrettifier,
    SovereignPasswordGenerator,
};

#[test]
fn test_segmentation_paging_and_aslr() {
    let engine = SegmentationPagingEngine::new(SpaceProtectionFlags::strict_hardening());
    let sel_ring0 = SegmentSelector::new(1, false, CpuRing::Ring0Kernel);
    let linear = engine.translate_logical_to_linear(sel_ring0, 0x1000, CpuRing::Ring0Kernel).unwrap();
    assert_eq!(linear, 0x1000);

    let config = AslrEntropyConfig::linux_default();
    let aslr = RandomizedAddressSpace::compute_aslr_layout(0x0000_0000_0040_0000, config, 12345);
    assert!(aslr.text_base >= 0x0000_0000_0040_0000);
}

#[test]
fn test_hammer2_pfs_namespaces_and_blake3_dedup() {
    let mut pfs = PseudoFilesystemNamespace::new("root_master", PfsType::Master);
    pfs.file_map.insert("/etc/hostname".to_string(), "blake3-hash1".to_string());

    let snap = PseudoFilesystemNamespace::snapshot("root_snap_1", "root_master", pfs.file_map.clone());
    assert!(snap.is_read_only);
    assert_eq!(snap.parent_snapshot_id.unwrap(), "root_master");

    let mut dedup = Blake3BlockDeduplicationEngine::new();
    let hash1 = dedup.store_block(b"SOVEREIGN_SYSTEM_BLOCK_DATA");
    let hash2 = dedup.store_block(b"SOVEREIGN_SYSTEM_BLOCK_DATA");
    assert_eq!(hash1, hash2);
    assert_eq!(*dedup.ref_counts.get(&hash1).unwrap(), 2);

    assert!(!dedup.release_block(&hash1));
    assert!(dedup.release_block(&hash1));
    assert!(dedup.read_block(&hash1).is_none());
}

#[test]
fn test_process_activity_manager() {
    let mut pam = ActivityManager::new();
    pam.register_process(500, 0, "chrome", 0);
    assert_eq!(pam.activities.len(), 1);
}

#[test]
fn test_unveil_sandboxing_and_landlock() {
    let mut state = UnveilState::new();
    state.unveil(std::path::PathBuf::from("/usr/bin"), "rwx").unwrap();
    state.unveil(std::path::PathBuf::from("/etc/nginx"), "r").unwrap();

    assert!(state.check_access(std::path::Path::new("/usr/bin/cargo"), UnveilPermissions::Read).is_ok());
    assert!(state.check_access(std::path::Path::new("/usr/bin/cargo"), UnveilPermissions::Execute).is_ok());
    assert!(state.check_access(std::path::Path::new("/etc/nginx/nginx.conf"), UnveilPermissions::Read).is_ok());
    assert!(state.check_access(std::path::Path::new("/etc/nginx/nginx.conf"), UnveilPermissions::Write).is_err());
}

#[test]
fn test_sigmatools_suite() {
    let mut etcher = SovereignDpkgEtcher::new("/dev/nvme0n1p1".to_string());
    assert!(etcher.flash_iso_image(&[0x7F, b'E', b'L', b'F']).is_ok());

    let calc = SovereignIPCalculator;
    let (net, bcast, hosts) = calc.calculate_subnet_details("10.0.0.50", 24).unwrap();
    assert_eq!(net, "10.0.0.0");
    assert_eq!(bcast, "10.0.0.255");
    assert_eq!(hosts, 254);

    let prettifier = SovereignJsonPrettifier;
    let pretty = prettifier.prettify_json("{\"kernel\":\"sigmaos\",\"version\":1}");
    assert!(pretty.contains("\n"));

    let gen = SovereignPasswordGenerator;
    let pass = gen.generate_secure_password(24, true);
    assert_eq!(pass.len(), 24);

    let rtc = AlmeidaCmosRtc::decode_cmos_values(0x00, 0x30, 0x14, 0x15, 0x08, 0x26, true);
    assert_eq!(rtc.format_timestamp(), "2026-08-15 14:30:00");
}

#[test]
fn test_statutory_compliance_overlay_and_community_toolkit() {
    let mut gov = StatutoryGovernanceLayer::new();
    let score = gov.evaluate_compliance_posture(1000);
    assert_eq!(score, 100);

    let handbook = CommunityHandbookCatalog::new();
    let articles = handbook.search_articles("FreeBSD");
    assert!(!articles.is_empty());

    let recipe_mgr = ReproduciblePackageRecipeManager::new();
    assert!(recipe_mgr.verify_checksum("zenith-desktop", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855").unwrap());

    let sec = SecurityProfileTemplateStore::new();
    assert!(sec.templates.contains_key("browser_sandboxed"));
}
