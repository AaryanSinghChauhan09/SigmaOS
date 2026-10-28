// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V82
// (`tests/test_package_advancements_v82.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v82_pr.rs"]
mod sovereign_distro_package_advancements_v82_pr;

use sovereign_distro_package_advancements_v82_pr::sovereign_distro_package_advancements_v82::*;
use sovereign_distro_package_advancements_v82_pr::SovereignDistroPackagePrGatewayV82;
use sovereign_distro_package_advancements_v82_pr::sovereign_distro_package_advancements_v82::universal::{PackageFormat, PackageState};

#[test]
fn test_package_advancements_v82_full_cycle() {
    // 1. Universal Package Inspector V82
    let mut inspector = UniversalPackageFormatInspectorV82::new();
    assert_eq!(inspector.total_formats_supported, 250);

    let manifest = inspector
        .inspect_package("ripgrep-14.0.0-amd64.deb", b"!<arch>\ndebian-binary\n")
        .clone();
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV82::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // Test extended format inspection (.air, .bottle, .ipa, .aab, .hap, .eopkg, .lzm, .pup, .snap, etc.)
    let manifest_air = inspector.inspect_package("app.air", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_air.detected_format, PackageFormat::Air);

    let manifest_bottle = inspector.inspect_package("curl.bottle", b"tar_gz_bottle");
    assert_eq!(manifest_bottle.detected_format, PackageFormat::Bottle);

    let manifest_lzm = inspector.inspect_package("slax.lzm", b"squashfs_lzm");
    assert_eq!(manifest_lzm.detected_format, PackageFormat::Lzm);

    // 2. OOP Design Patterns V82
    let mut facade = SovereignPackageFacadeV82::new();
    let fac_res = facade
        .quick_install("curl.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. SAT Dependency Resolution V82
    let solver = SatDpllDependencySolverV82::new();
    let raw_deps = vec!["libssl3".to_string(), "glibc".to_string()];
    let solved = solver
        .solve_dependencies("ripgrep", &raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-openssl", "sovereign-libc", "sovereign-base-v82"]);

    // 4. Multi-Sandbox Security Governor V82
    let sandbox_gov = UniversalMultiSandboxGovernorV82::new();
    let rules = sandbox_gov.generate_sandbox_rules(PackageFormat::Flatpak);
    assert!(rules.pledge_promises.contains("stdio"));
    assert_eq!(rules.landlock_abi_version, 6);

    // 5. CAS Delta Store V82
    let mut cas_store = UniversalCasDeltaStoreGovernorV82::new();
    let delta = cas_store
        .apply_delta_patch("base_v82", "patch_v82")
        .expect("CAS delta patch failed");
    assert_eq!(delta.compression_savings_ratio, 95);

    // 6. Boot Env Snapshot Governor V82
    let mut boot_env = UniversalBootEnvSnapshotGovernorV82::new();
    let snap_id = boot_env.create_snapshot(BootEnvKindV82::FreeBsdBectl, "Pre-Update-v82");
    let rb_res = boot_env.rollback(snap_id).expect("Rollback failed");
    assert!(rb_res.contains("Pre-Update-v82"));

    // 7. CLI Command Router V82
    let router = UniversalPmCliRouterV82::new();
    let routed = router
        .route_command("pacman -S neovim --print")
        .expect("Routing failed");
    assert_eq!(routed.action, UniversalPmActionV82::Install);
    assert!(routed.dry_run);
    assert_eq!(routed.package_targets, vec!["neovim".to_string()]);

    // 8. PR Gateway V82
    let mut gateway = SovereignDistroPackagePrGatewayV82::new();
    gateway.submit_proposal(
        8201,
        "Universal Package Advancements V82",
        PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V82",
    );
    assert_eq!(gateway.total_proposals(), 1);

    // 9. Master Coordinator Suite V82
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV82::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v82-htop".to_string()));
}
