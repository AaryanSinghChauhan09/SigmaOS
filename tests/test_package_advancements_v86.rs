// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V86
// (`tests/test_package_advancements_v86.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v86_pr.rs"]
mod sovereign_distro_package_advancements_v86_pr;

use sovereign_distro_package_advancements_v86_pr::sovereign_distro_package_advancements_v86::*;
use sovereign_distro_package_advancements_v86_pr::SovereignDistroPackagePrGatewayV86;
use sovereign_distro_package_advancements_v86_pr::universal::{PackageFormat, PackageState};

#[test]
fn test_package_advancements_v86_full_cycle() {
    // 1. Universal Package Inspector V86
    let mut inspector = UniversalPackageFormatInspectorV86::new();
    assert_eq!(inspector.total_formats_supported, 260);

    let manifest = inspector
        .inspect_package("ripgrep-14.1.1-amd64.deb", b"!<arch>\ndebian-binary\n")
        .clone();
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV86::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // Test extended format inspection (.air, .bottle, .ipa, .aab, .hap, .eopkg, .lzm, .pup, .snap, etc.)
    let manifest_air = inspector.inspect_package("app.air", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_air.detected_format, PackageFormat::Air);

    let manifest_bottle = inspector.inspect_package("curl.bottle", b"tar_gz_bottle");
    assert_eq!(manifest_bottle.detected_format, PackageFormat::Bottle);

    let manifest_lzm = inspector.inspect_package("slax.lzm", b"squashfs_lzm");
    assert_eq!(manifest_lzm.detected_format, PackageFormat::Lzm);

    // 2. OOP Design Patterns V86
    let mut facade = SovereignPackageFacadeV86::new();
    let fac_res = facade
        .quick_install("curl.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. Upstream Distro Delta Pre-Absorption Engine V86
    let delta_engine = UpstreamDistroDeltaPreAbsorptionEngineV86::new();
    assert!(delta_engine.total_deltas_absorbed() >= 3);

    // 4. SAT Dependency Resolution V86
    let solver = SatDpllDependencySolverV86::new();
    let raw_deps = vec!["libssl3".to_string(), "glibc".to_string()];
    let solved = solver
        .solve_dependencies("ripgrep", &raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-openssl", "sovereign-libc", "sovereign-base-v86"]);

    // 5. Multi-Sandbox Security Governor V86
    let sandbox_gov = UniversalMultiSandboxGovernorV86::new();
    let rules = sandbox_gov.generate_sandbox_rules(PackageFormat::Flatpak);
    assert!(rules.pledge_promises.contains("inet"));
    assert_eq!(rules.landlock_abi_version, 7);

    // 6. CAS Delta Store V86
    let mut cas_store = UniversalCasDeltaStoreGovernorV86::new();
    let delta = cas_store
        .apply_delta_patch("base_v86", "patch_v86")
        .expect("CAS delta patch failed");
    assert_eq!(delta.compression_savings_ratio, 96);

    // 7. Boot Env Snapshot Governor V86
    let mut boot_env = UniversalBootEnvSnapshotGovernorV86::new();
    let snap_id = boot_env.create_snapshot(BootEnvKindV86::FreeBsdBectl, "Pre-Update-v86");
    let rb_res = boot_env.rollback(snap_id).expect("Rollback failed");
    assert!(rb_res.contains("Pre-Update-v86"));

    // 8. CLI Command Router V86
    let router = UniversalPmCliRouterV86::new();
    let routed = router
        .route_command("pacman -S neovim --print")
        .expect("Routing failed");
    assert_eq!(routed.action, UniversalPmActionV86::Install);
    assert!(routed.dry_run);
    assert_eq!(routed.package_targets, vec!["neovim".to_string()]);

    // 9. PR Gateway V86
    let mut gateway = SovereignDistroPackagePrGatewayV86::new();
    gateway.submit_proposal(
        8601,
        "Universal Package Advancements V86",
        PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V86",
    );
    assert_eq!(gateway.total_proposals(), 1);

    // 10. Master Coordinator Suite V86
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV86::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v86-htop".to_string()));
}
