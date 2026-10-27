// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V75
// (`tests/test_package_advancements_v75.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v75.rs"]
mod sovereign_distro_package_advancements_v75;

use sovereign_distro_package_advancements_v75::*;
use sovereign_distro_package_advancements_v75::universal::{PackageFormat, PackageState};

#[test]
fn test_package_advancements_v75_full_cycle() {
    // 1. Universal Package Inspector V75
    let mut inspector = UniversalPackageFormatInspectorV75::new();
    assert_eq!(inspector.total_formats_supported, 250);

    let manifest = inspector.inspect_package("ripgrep-13.0.0-amd64.deb", b"!<arch>\ndebian-binary\n");
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV75::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // 2. OOP Design Patterns V75
    let mut facade = SovereignPackageFacadeV75::new();
    let fac_res = facade
        .quick_install("curl.apk", b"APK_PAYLOAD")
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. SAT Dependency Resolution V75
    let solver = UniversalCrossDistroSatSolverV75::new();
    let raw_deps = vec!["libssl-dev".to_string(), "glibc".to_string()];
    let solved = solver
        .solve_dependencies(&raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-openssl", "sovereign-libc"]);

    // 4. Multi-Sandbox Security Governor V75
    let sandbox_gov = UniversalMultiSandboxGovernorV75::new();
    let rules = sandbox_gov.generate_sandbox_rules(PackageFormat::Flatpak);
    assert!(rules.pledge_promises.contains("inet"));

    // 5. CAS Delta Store V75
    let mut cas_store = UniversalCasDeltaStoreGovernorV75::new();
    let delta = cas_store
        .apply_delta_patch("base_v75", "patch_v75")
        .expect("CAS delta patch failed");
    assert_eq!(delta.compression_savings_ratio, 94);

    // 6. Boot Env Snapshot Governor V75
    let mut boot_env = UniversalBootEnvSnapshotGovernorV75::new();
    let snap_id = boot_env.create_snapshot(BootEnvKindV75::FreeBsdBectl, "Pre-Update-v75");
    let rb_res = boot_env.rollback(snap_id).expect("Rollback failed");
    assert!(rb_res.contains("Pre-Update-v75"));

    // 7. CLI Command Router V75
    let router = UniversalPmCliRouterV75::new();
    let routed = router
        .route_command("pacman -S neovim --print")
        .expect("Routing failed");
    assert_eq!(routed.action, UniversalPmActionV75::Install);
    assert!(routed.dry_run);
    assert_eq!(routed.package_targets, vec!["neovim".to_string()]);

    // 8. Master Coordinator Suite V75
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV75::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", b"PK\x03\x04_APK_PAYLOAD")
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v75-htop".to_string()));
}
