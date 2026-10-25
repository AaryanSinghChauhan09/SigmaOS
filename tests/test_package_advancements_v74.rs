// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V74
// (`tests/test_package_advancements_v74.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v74_pr.rs"]
mod sovereign_distro_package_advancements_v74_pr;

use sovereign_distro_package_advancements_v74_pr::sovereign_distro_package_advancements_v74::*;
use sovereign_distro_package_advancements_v74_pr::SovereignDistroPackagePrGatewayV74;
use sovereign_distro_package_advancements_v74_pr::universal::{PackageFormat, PackageState};

#[test]
fn test_package_advancements_v74_full_cycle() {
    // 1. Universal Package Inspector V74
    let mut inspector = UniversalPackageFormatInspectorV74::new();
    assert_eq!(inspector.total_formats_supported, 220);

    let manifest = inspector
        .inspect_package("ripgrep-14.0.0-amd64.deb", b"DEB_HEADER_PAYLOAD")
        .expect("Failed to inspect DEB package");
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV74::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // 2. OOP Design Patterns V74
    let mut facade = SovereignPackageFacadeV74::new();
    let fac_res = facade
        .quick_install("curl.apk", b"APK_PAYLOAD")
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. SAT Dependency Resolution V74
    let solver = UniversalCrossDistroSatSolverV74::new();
    let raw_deps = vec!["libssl-dev".to_string(), "glibc".to_string()];
    let solved = solver
        .solve_dependencies(&raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-openssl", "sovereign-libc"]);

    // 4. Multi-Sandbox Security Governor V74
    let sandbox_gov = UniversalMultiSandboxGovernorV74::new();
    let rules = sandbox_gov.generate_sandbox_rules(PackageFormat::Flatpak);
    assert!(rules.pledge_promises.contains("inet"));

    // 5. CAS Delta Store V74
    let mut cas_store = UniversalCasDeltaStoreGovernorV74::new();
    let delta = cas_store
        .apply_delta_patch("base_v74", "patch_v74")
        .expect("CAS delta patch failed");
    assert_eq!(delta.compressed_size, 1024);

    // 6. Boot Env Snapshot Governor V74
    let mut boot_env = UniversalBootEnvSnapshotGovernorV74::new();
    let snap_id = boot_env.create_snapshot(BootEnvKindV74::FreeBsdBectl, "Pre-Update-v74");
    let rb_res = boot_env.rollback(snap_id).expect("Rollback failed");
    assert!(rb_res.contains("snapshot #1"));

    // 7. CLI Command Router V74
    let router = UniversalPmCliRouterV74::new();
    let routed = router
        .route_command("pacman -S neovim --print")
        .expect("Routing failed");
    assert_eq!(routed.action, UniversalPmActionV74::Install);
    assert!(routed.dry_run);
    assert_eq!(routed.package_targets, vec!["neovim".to_string()]);

    // 8. PR Gateway V74
    let mut gateway = SovereignDistroPackagePrGatewayV74::new();
    gateway.submit_proposal(
        7401,
        "Universal Package Advancements V74",
        PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V74",
    );
    assert_eq!(gateway.total_proposals(), 1);

    // 9. Master Coordinator Suite V74
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV74::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", b"APK_PAYLOAD")
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v74-htop".to_string()));
}
