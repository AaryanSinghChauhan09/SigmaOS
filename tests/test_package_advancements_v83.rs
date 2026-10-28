// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V83
// (`tests/test_package_advancements_v83.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v83.rs"]
mod sovereign_distro_package_advancements_v83;

#[path = "../src/package/sovereign_distro_package_advancements_v83_pr.rs"]
mod sovereign_distro_package_advancements_v83_pr;

use sovereign_distro_package_advancements_v83::*;
use sovereign_distro_package_advancements_v83_pr::SovereignDistroPackagePrGatewayV83;
use sovereign_distro_package_advancements_v83::universal::{PackageFormat, PackageState};

#[test]
fn test_package_advancements_v83_full_cycle() {
    // 1. Universal Package Inspector V83
    let mut inspector = UniversalPackageFormatInspectorV83::new();
    assert_eq!(inspector.total_formats_supported, 250);

    let manifest = inspector
        .inspect_package("ripgrep-13.0.0-amd64.deb", b"!<arch>\ndebian-binary\n")
        .clone();
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV83::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // Test extended format inspection (.air, .bottle, .ipa, .aab, .hap, .eopkg, .lzm, .pup, .snap, .pet, etc.)
    let manifest_air = inspector.inspect_package("app.air", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_air.detected_format, PackageFormat::Air);

    let manifest_bottle = inspector.inspect_package("curl.bottle", b"tar_gz_bottle");
    assert_eq!(manifest_bottle.detected_format, PackageFormat::Bottle);

    let manifest_lzm = inspector.inspect_package("slax.lzm", b"squashfs_lzm");
    assert_eq!(manifest_lzm.detected_format, PackageFormat::Lzm);

    let manifest_pet = inspector.inspect_package("puppy.pet", b"puppy_pet_archive");
    assert_eq!(manifest_pet.detected_format, PackageFormat::Pet);

    // 2. OOP Design Patterns V83
    let mut facade = SovereignPackageFacadeV83::new();
    let fac_res = facade
        .quick_install("curl.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. SAT Dependency Resolution V83
    let solver = SatDpllDependencySolverV83::new();
    let raw_deps = vec!["libssl3".to_string(), "glibc".to_string()];
    let solved = solver
        .solve_dependencies("ripgrep", &raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-openssl", "sovereign-libc", "sovereign-base-v83"]);

    // 4. Multi-Sandbox Security Governor V83
    let sandbox_gov = UniversalMultiSandboxGovernorV83::new();
    let rules = sandbox_gov.generate_sandbox_rules(PackageFormat::Flatpak);
    assert!(rules.pledge_promises.contains("inet"));
    assert_eq!(rules.landlock_abi_version, 6);

    // 5. CAS Delta Store V83
    let mut cas_store = UniversalCasDeltaStoreGovernorV83::new();
    let delta = cas_store
        .apply_delta_patch("base_v83", "patch_v83")
        .expect("CAS delta patch failed");
    assert_eq!(delta.compression_savings_ratio, 95);

    // 6. Boot Env Snapshot Governor V83
    let mut boot_env = UniversalBootEnvSnapshotGovernorV83::new();
    let snap_id = boot_env.create_snapshot(BootEnvKindV83::FreeBsdBectl, "Pre-Update-v83");
    let rb_res = boot_env.rollback(snap_id).expect("Rollback failed");
    assert!(rb_res.contains("Pre-Update-v83"));

    // 7. CLI Command Router V83
    let router = UniversalPmCliRouterV83::new();
    let routed = router
        .route_command("pacman -S neovim --print")
        .expect("Routing failed");
    assert_eq!(routed.action, UniversalPmActionV83::Install);
    assert!(routed.dry_run);
    assert_eq!(routed.package_targets, vec!["neovim".to_string()]);

    // 8. PR Gateway V83
    let mut gateway = SovereignDistroPackagePrGatewayV83::new();
    gateway.submit_proposal(
        8301,
        "Universal Package Advancements V83",
        sovereign_distro_package_advancements_v83_pr::PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V83",
    );
    assert_eq!(gateway.total_proposals(), 1);

    // 9. Master Coordinator Suite V83
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV83::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v83-htop".to_string()));
}
