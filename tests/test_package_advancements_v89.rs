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

    // Test extended format inspection across all requested formats:
    // .air, .bottle, .ipa, .ports, .pkg, .aab, .apk, AppImage, .eopkg, .nixpkg, .portage, .deb,
    // .tar.gz, .xz, .rpm, .ebuild, .pkg.tar.xz, Flatpak, .app, .hap, .PiSi, .tgz, .superdeb, .lzm, pup, .snap, pacman, .tar, .pet
    let manifest_air = inspector.inspect_package("app.air", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_air.detected_format, PackageFormat::Air);

    let manifest_bottle = inspector.inspect_package("curl.bottle", b"tar_gz_bottle");
    assert_eq!(manifest_bottle.detected_format, PackageFormat::Bottle);

    let manifest_ipa = inspector.inspect_package("mobile.ipa", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_ipa.detected_format, PackageFormat::Ipa);

    let manifest_ports = inspector.inspect_package("bsd.ports", b"ports_data");
    assert_eq!(manifest_ports.detected_format, PackageFormat::Pkg);

    let manifest_bsdpkg = inspector.inspect_package("freebsd.pkg", b"pkg_content");
    assert_eq!(manifest_bsdpkg.detected_format, PackageFormat::Pkg);

    let manifest_aab = inspector.inspect_package("app.aab", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_aab.detected_format, PackageFormat::Aab);

    let manifest_apk = inspector.inspect_package("tool.apk", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_apk.detected_format, PackageFormat::Apk);

    let manifest_appimage = inspector.inspect_package("tool.AppImage", b"hsqs_appimage");
    assert_eq!(manifest_appimage.detected_format, PackageFormat::AppImage);

    let manifest_eopkg = inspector.inspect_package("solus.eopkg", b"zip_xml_data");
    assert_eq!(manifest_eopkg.detected_format, PackageFormat::Eopkg);

    let manifest_nixpkg = inspector.inspect_package("system.nixpkg", b"nix_nar_data");
    assert_eq!(manifest_nixpkg.detected_format, PackageFormat::Nixpkg);

    let manifest_portage = inspector.inspect_package("sys-apps.portage", b"portage_data");
    assert_eq!(manifest_portage.detected_format, PackageFormat::Ebuild);

    let manifest_deb = inspector.inspect_package("sys.deb", b"!<arch>\ndebian");
    assert_eq!(manifest_deb.detected_format, PackageFormat::Deb);

    let manifest_targz = inspector.inspect_package("archive.tar.gz", &[0x1f, 0x8b, 0x08, 0x00]);
    assert_eq!(manifest_targz.detected_format, PackageFormat::TarGz);

    let manifest_xz = inspector.inspect_package("archive.xz", &[0xfd, b'7', b'z', b'X', b'Z', 0x00]);
    assert_eq!(manifest_xz.detected_format, PackageFormat::Pacman);

    let manifest_rpm = inspector.inspect_package("fedora.rpm", &[0xed, 0xab, 0xee, 0xdb]);
    assert_eq!(manifest_rpm.detected_format, PackageFormat::Rpm);

    let manifest_ebuild = inspector.inspect_package("gentoo.ebuild", b"ebuild_content");
    assert_eq!(manifest_ebuild.detected_format, PackageFormat::Ebuild);

    let manifest_pkgtarxz = inspector.inspect_package("arch.pkg.tar.xz", &[0xfd, b'7', b'z', b'X', b'Z', 0x00]);
    assert_eq!(manifest_pkgtarxz.detected_format, PackageFormat::Pacman);

    let manifest_flatpak = inspector.inspect_package("app.flatpak", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_flatpak.detected_format, PackageFormat::Flatpak);

    let manifest_app = inspector.inspect_package("macos.app", b"bundle_content");
    assert_eq!(manifest_app.detected_format, PackageFormat::App);

    let manifest_hap = inspector.inspect_package("harmony.hap", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_hap.detected_format, PackageFormat::Hap);

    let manifest_pisi = inspector.inspect_package("solus.PiSi", b"zip_xml_data");
    assert_eq!(manifest_pisi.detected_format, PackageFormat::Eopkg);

    let manifest_tgz = inspector.inspect_package("source.tgz", &[0x1f, 0x8b, 0x08, 0x00]);
    assert_eq!(manifest_tgz.detected_format, PackageFormat::TarGz);

    let manifest_superdeb = inspector.inspect_package("deepin.superdeb", b"ar_superdeb_data");
    assert_eq!(manifest_superdeb.detected_format, PackageFormat::Superdeb);

    let manifest_lzm = inspector.inspect_package("slax.lzm", b"squashfs_lzm");
    assert_eq!(manifest_lzm.detected_format, PackageFormat::Lzm);

    let manifest_pup = inspector.inspect_package("puppy.pup", &[0x1f, 0x8b, 0x08, 0x00]);
    assert_eq!(manifest_pup.detected_format, PackageFormat::Pup);

    let manifest_snap = inspector.inspect_package("canonical.snap", b"hsqs_snap_data");
    assert_eq!(manifest_snap.detected_format, PackageFormat::Snap);

    let manifest_pacman = inspector.inspect_package("neovim.pacman", &[0x28, 0xb5, 0x2f, 0xfd]);
    assert_eq!(manifest_pacman.detected_format, PackageFormat::Pacman);

    let manifest_tar = inspector.inspect_package("archive.tar", b"ustar  data");
    assert_eq!(manifest_tar.detected_format, PackageFormat::Tar);

    let manifest_pet = inspector.inspect_package("puppy.pet", &[0x1f, 0x8b, 0x08, 0x00]);
    assert_eq!(manifest_pet.detected_format, PackageFormat::Pup);

    let manifest_xbps = inspector.inspect_package("void.xbps", b"xbps_data");
    assert_eq!(manifest_xbps.detected_format, PackageFormat::Xbps);

    // 2. OOP Design Patterns V89
    let mut facade = SovereignPackageFacadeV89::new();
    let fac_res = facade
        .quick_install("curl.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    let mut pkg = UnifiedPackage::new("libuv".to_string(), "1.48.0".to_string())
        .with_format(PackageFormat::Deb);
    let sec_dec = SecurityHardeningDecoratorV89;
    sec_dec.decorate(&mut pkg);
    assert_eq!(pkg.name, "hardened-v89-libuv");

    let strat = PackageLifecycleFactoryV89::create_strategy(PackageFormat::Rpm);
    let st_res = strat.install(&pkg).unwrap();
    assert!(st_res.contains("V89 transactional strategy"));

    let mut obs = PackageAuditObserverV89::new();
    obs.on_event("PreHookFired");
    assert_eq!(obs.event_log.len(), 1);

    let mut cmd = InstallCommandV89 {
        pkg_name: "neofetch".to_string(),
        installed: false,
    };
    cmd.execute().unwrap();
    assert!(cmd.installed);
    cmd.undo().unwrap();
    assert!(!cmd.installed);

    // 3. User-Defined Functions (UDFs) Scriptlet Sandboxing Across 14 Hooks V89
    let mut udf_engine = UdfScriptletEngineV89::new();
    udf_engine.register_hook(UdfExecutionContextV89 {
        hook_type: UdfHookTypeV89::PreInstall,
        package_name: "ripgrep".to_string(),
        scriptlet_code: "useradd -r ripgrep_user".to_string(),
        environment_vars: std::collections::BTreeMap::new(),
    });
    udf_engine.register_hook(UdfExecutionContextV89 {
        hook_type: UdfHookTypeV89::RollbackHook,
        package_name: "ripgrep".to_string(),
        scriptlet_code: "userdel ripgrep_user".to_string(),
        environment_vars: std::collections::BTreeMap::new(),
    });
    udf_engine.register_hook(UdfExecutionContextV89 {
        hook_type: UdfHookTypeV89::SecurityAudit,
        package_name: "ripgrep".to_string(),
        scriptlet_code: "auditctl -w /usr/bin/rg -p rwa -k ripgrep".to_string(),
        environment_vars: std::collections::BTreeMap::new(),
    });

    let executed = udf_engine
        .execute_hooks_for_phase("ripgrep", UdfHookTypeV89::PreInstall)
        .expect("UDF execution failed");
    assert_eq!(executed, 1);
    let exec_rb = udf_engine
        .execute_hooks_for_phase("ripgrep", UdfHookTypeV89::RollbackHook)
        .expect("Rollback UDF failed");
    assert_eq!(exec_rb, 1);

    // 4. SAT DPLL Dependency Solver V89
    let solver = SatDpllDependencySolverV89::new();
    let raw_deps = vec!["libc6".to_string(), "openssl".to_string()];
    let solved = solver
        .solve_dependencies("ripgrep", &raw_deps)
        .expect("SAT solver failed");
    assert_eq!(solved, vec!["sovereign-libc", "sovereign-openssl", "sovereign-base-v89"]);

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
        8901,
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
