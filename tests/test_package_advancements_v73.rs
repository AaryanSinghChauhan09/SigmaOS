// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V73
// (`tests/test_package_advancements_v73.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v73.rs"]
mod sovereign_distro_package_advancements_v73;

#[path = "../src/package/sovereign_distro_package_advancements_v73_pr.rs"]
mod sovereign_distro_package_advancements_v73_pr;

use sovereign_distro_package_advancements_v73::universal::{PackageFormat, PackageState};
use sovereign_distro_package_advancements_v73::*;
use sovereign_distro_package_advancements_v73_pr::SovereignDistroPackagePrGatewayV73;

#[test]
fn test_package_advancements_v73_full_cycle() {
    // 1. Universal Package Inspector V73
    let mut inspector = UniversalPackageFormatInspectorV73::new();
    assert_eq!(inspector.total_formats_supported, 210);

    let manifest = inspector
        .inspect_package("ripgrep-13.0.0-amd64.deb", b"DEB_HEADER_PAYLOAD")
        .expect("Failed to inspect DEB package");
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV73::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // 2. OOP Design Patterns V73
    let mut facade = SovereignPackageFacadeV73::new();
    let fac_res = facade
        .quick_install("curl.apk", b"APK_PAYLOAD")
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. SAT Dependency Resolution V73
    let solver = UniversalCrossDistroSatSolverV73::new();
    let raw_deps = vec!["libssl-dev".to_string(), "glibc".to_string()];
    let solved = solver
        .solve_dependencies(&raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-openssl", "sovereign-libc"]);

    // 4. Multi-Sandbox Security Governor V73
    let sandbox_gov = UniversalMultiSandboxGovernorV73::new();
    let rules = sandbox_gov.generate_sandbox_rules(PackageFormat::Flatpak);
    assert!(rules.pledge_promises.contains("inet"));

    // 5. CAS Delta Store V73
    let mut cas_store = UniversalCasDeltaStoreGovernorV73::new();
    let delta = cas_store
        .apply_delta_patch("base_v73", "patch_v73")
        .expect("CAS delta patch failed");
    assert_eq!(delta.compression_savings_ratio, 90);

    // 6. Boot Env Snapshot Governor V73
    let mut boot_env = UniversalBootEnvSnapshotGovernorV73::new();
    let snap_id = boot_env.create_snapshot(BootEnvKindV73::FreeBsdBectl, "Pre-Update-v73");
    let rb_res = boot_env.rollback(snap_id).expect("Rollback failed");
    assert!(rb_res.contains("Pre-Update-v73"));

    // 7. CLI Command Router V73
    let router = UniversalPmCliRouterV73::new();
    let routed = router
        .route_command("pacman -S neovim --print")
        .expect("Routing failed");
    assert_eq!(routed.action, UniversalPmActionV73::Install);
    assert!(routed.dry_run);
    assert_eq!(routed.package_targets, vec!["neovim".to_string()]);

    // 8. PR Gateway V73
    let mut gateway = SovereignDistroPackagePrGatewayV73::new();
    gateway.submit_proposal(
        7301,
        "Universal Package Advancements V73",
        PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V73",
    );
    assert_eq!(gateway.total_proposals(), 1);

    // 9. Master Coordinator Suite V73
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV73::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", b"APK_PAYLOAD")
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v73-htop".to_string()));
}
