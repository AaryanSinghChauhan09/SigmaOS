// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V89
// (`tests/test_package_advancements_v89.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v89_pr.rs"]
mod sovereign_distro_package_advancements_v89_pr;

use sovereign_distro_package_advancements_v89_pr::sovereign_distro_package_advancements_v89::*;
use sovereign_distro_package_advancements_v89_pr::SovereignDistroPackagePrGatewayV89;
use sovereign_distro_package_advancements_v89_pr::universal::{PackageFormat, PackageState};

#[test]
fn test_package_advancements_v89_full_cycle() {
    // 1. Universal Package Inspector V89
    let mut inspector = UniversalPackageFormatInspectorV89::new();
    assert_eq!(inspector.total_formats_supported, 285);

    let manifest = inspector
        .inspect_package("ripgrep-14.1.0-amd64.deb", b"!<arch>\ndebian-binary\n")
        .clone();
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV89::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // Test extended format inspection (.air, .bottle, .ipa, .aab, .hap, .eopkg, .lzm, .pup, .snap, .xbps, .ebuild, .pkg, etc.)
    let manifest_air = inspector.inspect_package("app.air", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_air.detected_format, PackageFormat::Air);

    let manifest_bottle = inspector.inspect_package("curl.bottle", b"tar_gz_bottle");
    assert_eq!(manifest_bottle.detected_format, PackageFormat::Bottle);

    let manifest_lzm = inspector.inspect_package("slax.lzm", b"squashfs_lzm");
    assert_eq!(manifest_lzm.detected_format, PackageFormat::Lzm);

    let manifest_xbps = inspector.inspect_package("void.xbps", b"xbps_data");
    assert_eq!(manifest_xbps.detected_format, PackageFormat::Xbps);

    let manifest_ebuild = inspector.inspect_package("gentoo.ebuild", b"ebuild_content");
    assert_eq!(manifest_ebuild.detected_format, PackageFormat::Ebuild);

    let manifest_bsdpkg = inspector.inspect_package("freebsd.pkg", b"pkg_content");
    assert_eq!(manifest_bsdpkg.detected_format, PackageFormat::Pkg);

    // 2. OOP Design Patterns V89
    let mut facade = SovereignPackageFacadeV89::new();
    let fac_res = facade
        .quick_install("curl.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. Upstream Distro Delta Pre-Absorption Engine V89
    let delta_engine = UpstreamDistroDeltaPreAbsorptionEngineV89::new();
    assert!(delta_engine.total_deltas_absorbed() >= 4);

    // 4. SAT Dependency Resolution V89
    let solver = SatDpllDependencySolverV89::new();
    let raw_deps = vec!["libssl3".to_string(), "glibc".to_string()];
    let solved = solver
        .solve_dependencies("ripgrep", &raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-openssl", "sovereign-libc", "sovereign-base-v89"]);

    // 5. Multi-Sandbox Security Governor V89
    let sandbox_gov = UniversalMultiSandboxGovernorV89::new();
    let rules = sandbox_gov.generate_sandbox_rules(PackageFormat::Flatpak);
    assert!(rules.pledge_promises.contains("inet"));
    assert_eq!(rules.landlock_abi_version, 6);

    // 6. CAS Delta Store V89
    let mut cas_store = UniversalCasDeltaStoreGovernorV89::new();
    let delta = cas_store
        .apply_delta_patch("base_v89", "patch_v89")
        .expect("CAS delta patch failed");
    assert_eq!(delta.compression_savings_ratio, 95);

    // 7. Boot Env Snapshot Governor V89
    let mut boot_env = UniversalBootEnvSnapshotGovernorV89::new();
    let snap_id = boot_env.create_snapshot(BootEnvKindV89::FreeBsdBectl, "Pre-Update-v89");
    let rb_res = boot_env.rollback(snap_id).expect("Rollback failed");
    assert!(rb_res.contains("Pre-Update-v89"));

    // 8. CLI Command Router V89
    let router = UniversalPmCliRouterV89::new();
    let routed = router
        .route_command("pacman -S neovim --print")
        .expect("Routing failed");
    assert_eq!(routed.action, UniversalPmActionV89::Install);
    assert!(routed.dry_run);
    assert_eq!(routed.package_targets, vec!["neovim".to_string()]);

    let routed_emerge = router
        .route_command("emerge -a sys-apps/ripgrep")
        .expect("Emerge routing failed");
    assert_eq!(routed_emerge.action, UniversalPmActionV89::Install);

    // 9. PR Gateway V89
    let mut gateway = SovereignDistroPackagePrGatewayV89::new();
    gateway.submit_proposal(
        8801,
        "Universal Package Advancements V89",
        PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V89",
    );
    assert_eq!(gateway.total_proposals(), 1);

    // 10. Master Coordinator Suite V89
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV89::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v89-htop".to_string()));
}
