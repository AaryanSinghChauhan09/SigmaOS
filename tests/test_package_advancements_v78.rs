// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V78
// (`tests/test_package_advancements_v78.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v78.rs"]
mod sovereign_distro_package_advancements_v78;

#[path = "../src/package/sovereign_distro_package_advancements_v78_pr.rs"]
mod sovereign_distro_package_advancements_v78_pr;

use sovereign_distro_package_advancements_v78::*;
use sovereign_distro_package_advancements_v78_pr::SovereignDistroPackagePrGatewayV78;
use sovereign_distro_package_advancements_v78::universal::{PackageFormat, PackageState};

#[test]
fn test_package_advancements_v78_full_cycle() {
    // 1. Universal Package Inspector V78
    let mut inspector = UniversalPackageFormatInspectorV78::new();
    assert_eq!(inspector.total_formats_supported, 240);

    let manifest = inspector
        .inspect_package("ripgrep-13.0.0-amd64.deb", b"!<arch>\ndebian-binary\n")
        .clone();
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV78::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // Test extended format inspection (.air, .bottle, .ipa, .aab, .hap, .eopkg, .lzm, .pup, .snap, etc.)
    let manifest_air = inspector.inspect_package("app.air", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_air.detected_format, PackageFormat::Air);

    let manifest_bottle = inspector.inspect_package("curl.bottle", b"tar_gz_bottle");
    assert_eq!(manifest_bottle.detected_format, PackageFormat::Bottle);

    let manifest_lzm = inspector.inspect_package("slax.lzm", b"squashfs_lzm");
    assert_eq!(manifest_lzm.detected_format, PackageFormat::Lzm);

    // 2. OOP Design Patterns V78
    let mut facade = SovereignPackageFacadeV78::new();
    let fac_res = facade
        .quick_install("curl.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. SAT Dependency Resolution V78
    let solver = SatDpllDependencySolverV78::new();
    let raw_deps = vec!["libssl3".to_string(), "glibc".to_string()];
    let solved = solver
        .solve_dependencies("ripgrep", &raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-openssl", "sovereign-libc", "sovereign-base-v78"]);

    // 4. Multi-Sandbox Security Governor V78
    let sandbox_gov = UniversalMultiSandboxGovernorV78::new();
    let rules = sandbox_gov.generate_sandbox_rules(PackageFormat::Flatpak);
    assert!(rules.pledge_promises.contains("inet"));
    assert_eq!(rules.landlock_abi_version, 6);

    // 5. CAS Delta Store V78
    let mut cas_store = UniversalCasDeltaStoreGovernorV78::new();
    let delta = cas_store
        .apply_delta_patch("base_v78", "patch_v78")
        .expect("CAS delta patch failed");
    assert_eq!(delta.compression_savings_ratio, 94);

    // 6. Boot Env Snapshot Governor V78
    let mut boot_env = UniversalBootEnvSnapshotGovernorV78::new();
    let snap_id = boot_env.create_snapshot(BootEnvKindV78::FreeBsdBectl, "Pre-Update-v78");
    let rb_res = boot_env.rollback(snap_id).expect("Rollback failed");
    assert!(rb_res.contains("Pre-Update-v78"));

    // 7. CLI Command Router V78
    let router = UniversalPmCliRouterV78::new();
    let routed = router
        .route_command("pacman -S neovim --print")
        .expect("Routing failed");
    assert_eq!(routed.action, UniversalPmActionV78::Install);
    assert!(routed.dry_run);
    assert_eq!(routed.package_targets, vec!["neovim".to_string()]);

    // 8. PR Gateway V78
    let mut gateway = SovereignDistroPackagePrGatewayV78::new();
    gateway.submit_proposal(
        7801,
        "Universal Package Advancements V78",
        PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V78",
    );
    assert_eq!(gateway.total_proposals(), 1);

    // 9. Master Coordinator Suite V78
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV78::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v78-htop".to_string()));
}
