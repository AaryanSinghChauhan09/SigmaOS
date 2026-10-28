// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V85
// (`tests/test_package_advancements_v85.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v85_pr.rs"]
mod sovereign_distro_package_advancements_v85_pr;

use sovereign_distro_package_advancements_v85_pr::sovereign_distro_package_advancements_v85::*;
use sovereign_distro_package_advancements_v85_pr::SovereignDistroPackagePrGatewayV85;
use sovereign_distro_package_advancements_v85_pr::universal::{PackageFormat, PackageState};

#[test]
fn test_package_advancements_v85_full_cycle() {
    // 1. Universal Package Inspector V85
    let mut inspector = UniversalPackageFormatInspectorV85::new();
    assert_eq!(inspector.total_formats_supported, 250);

    let manifest = inspector
        .inspect_package("ripgrep-14.1.0-amd64.deb", b"!<arch>\ndebian-binary\n")
        .clone();
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV85::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // Test extended format inspection (.air, .bottle, .ipa, .aab, .hap, .eopkg, .lzm, .pup, .snap, etc.)
    let manifest_air = inspector.inspect_package("app.air", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_air.detected_format, PackageFormat::Air);

    let manifest_bottle = inspector.inspect_package("curl.bottle", b"tar_gz_bottle");
    assert_eq!(manifest_bottle.detected_format, PackageFormat::Bottle);

    let manifest_lzm = inspector.inspect_package("slax.lzm", b"squashfs_lzm");
    assert_eq!(manifest_lzm.detected_format, PackageFormat::Lzm);

    // 2. OOP Design Patterns V85
    let mut facade = SovereignPackageFacadeV85::new();
    let fac_res = facade
        .quick_install("curl.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. Upstream Distro Delta Pre-Absorption Engine V85
    let delta_engine = UpstreamDistroDeltaPreAbsorptionEngineV85::new();
    assert!(delta_engine.total_deltas_absorbed() >= 3);

    // 4. SAT Dependency Resolution V85
    let solver = SatDpllDependencySolverV85::new();
    let raw_deps = vec!["libssl3".to_string(), "glibc".to_string()];
    let solved = solver
        .solve_dependencies("ripgrep", &raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-openssl", "sovereign-libc", "sovereign-base-v85"]);

    // 5. Multi-Sandbox Security Governor V85
    let sandbox_gov = UniversalMultiSandboxGovernorV85::new();
    let rules = sandbox_gov.generate_sandbox_rules(PackageFormat::Flatpak);
    assert!(rules.pledge_promises.contains("inet"));
    assert_eq!(rules.landlock_abi_version, 6);

    // 6. CAS Delta Store V85
    let mut cas_store = UniversalCasDeltaStoreGovernorV85::new();
    let delta = cas_store
        .apply_delta_patch("base_v85", "patch_v85")
        .expect("CAS delta patch failed");
    assert_eq!(delta.compression_savings_ratio, 95);

    // 7. Boot Env Snapshot Governor V85
    let mut boot_env = UniversalBootEnvSnapshotGovernorV85::new();
    let snap_id = boot_env.create_snapshot(BootEnvKindV85::FreeBsdBectl, "Pre-Update-v85");
    let rb_res = boot_env.rollback(snap_id).expect("Rollback failed");
    assert!(rb_res.contains("Pre-Update-v85"));

    // 8. CLI Command Router V85
    let router = UniversalPmCliRouterV85::new();
    let routed = router
        .route_command("pacman -S neovim --print")
        .expect("Routing failed");
    assert_eq!(routed.action, UniversalPmActionV85::Install);
    assert!(routed.dry_run);
    assert_eq!(routed.package_targets, vec!["neovim".to_string()]);

    // 9. PR Gateway V85
    let mut gateway = SovereignDistroPackagePrGatewayV85::new();
    gateway.submit_proposal(
        8501,
        "Universal Package Advancements V85",
        PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V85",
    );
    assert_eq!(gateway.total_proposals(), 1);

    // 10. Master Coordinator Suite V85
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV85::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v85-htop".to_string()));
}
