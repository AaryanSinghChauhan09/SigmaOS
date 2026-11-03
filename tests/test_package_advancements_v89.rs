// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V89
// (`tests/test_package_advancements_v89.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v89_pr.rs"]
mod sovereign_distro_package_advancements_v89_pr;

use sovereign_distro_package_advancements_v89_pr::sovereign_distro_package_advancements_v89::*;
use sovereign_distro_package_advancements_v89_pr::universal::{PackageFormat, PackageState};
use sovereign_distro_package_advancements_v89_pr::SovereignDistroPackagePrGatewayV89;

#[test]
fn test_package_advancements_v89_full_cycle() {
    // 1. Universal Package Inspector V89
    let mut inspector = UniversalPackageFormatInspectorV89::new();
    assert_eq!(inspector.total_formats_supported, 285);

    let manifest = inspector
        .inspect_package("ripgrep-14.2.0-amd64.deb", b"!<arch>\ndebian-binary\n")
        .clone();
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV89::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // Test extended format inspection across all user-specified formats:
    // .air, .bottle, .ipa, .ports, .pkg, .aab, .apk, AppImage, .eopkg, .nixpkg,
    // .portage, .deb, .tar.gz, .xz, .rpm, .ebuild, .pkg.tar.xz, Flatpak, .app,
    // .hap, .PiSi, .tgz, .superdeb, .lzm, pup, .snap, pacman, .tar, .pet
    let manifest_air = inspector.inspect_package("app.air", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_air.detected_format, PackageFormat::Air);

    let manifest_bottle = inspector.inspect_package("curl.bottle", &[0x1f, 0x8b]);
    assert_eq!(manifest_bottle.detected_format, PackageFormat::Bottle);

    let manifest_ipa = inspector.inspect_package("ios.ipa", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_ipa.detected_format, PackageFormat::Ipa);

    let manifest_ports = inspector.inspect_package("freebsd.ports", b"ports_content");
    assert_eq!(manifest_ports.detected_format, PackageFormat::Ports);

    let manifest_pkg = inspector.inspect_package("solaris.pkg", b"pkg_content");
    assert_eq!(manifest_pkg.detected_format, PackageFormat::Pkg);

    let manifest_aab = inspector.inspect_package("android.aab", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_aab.detected_format, PackageFormat::Aab);

    let manifest_apk = inspector.inspect_package("alpine.apk", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_apk.detected_format, PackageFormat::Apk);

    let manifest_appimage = inspector.inspect_package("tool.AppImage", b"hsqs_content");
    assert_eq!(manifest_appimage.detected_format, PackageFormat::AppImage);

    let manifest_eopkg = inspector.inspect_package("solus.eopkg", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_eopkg.detected_format, PackageFormat::Eopkg);

    let manifest_nixpkg = inspector.inspect_package("nix.nixpkg", b"nix_content");
    assert_eq!(manifest_nixpkg.detected_format, PackageFormat::Nixpkg);

    let manifest_portage = inspector.inspect_package("gentoo.portage", b"portage_spec");
    assert_eq!(manifest_portage.detected_format, PackageFormat::Portage);

    let manifest_deb = inspector.inspect_package("debian.deb", b"!<arch>\ndebian-binary\n");
    assert_eq!(manifest_deb.detected_format, PackageFormat::Deb);

    let manifest_targz = inspector.inspect_package("archive.tar.gz", &[0x1f, 0x8b]);
    assert_eq!(manifest_targz.detected_format, PackageFormat::TarGz);

    let manifest_xz =
        inspector.inspect_package("archive.xz", &[0xfd, b'7', b'z', b'X', b'Z', 0x00]);
    assert_eq!(manifest_xz.detected_format, PackageFormat::Xz);

    let manifest_rpm = inspector.inspect_package("redhat.rpm", &[0xed, 0xab, 0xee, 0xdb]);
    assert_eq!(manifest_rpm.detected_format, PackageFormat::Rpm);

    let manifest_ebuild = inspector.inspect_package("package.ebuild", b"ebuild_script");
    assert_eq!(manifest_ebuild.detected_format, PackageFormat::Ebuild);

    let manifest_pkgtarxz =
        inspector.inspect_package("arch.pkg.tar.xz", &[0xfd, b'7', b'z', b'X', b'Z', 0x00]);
    assert_eq!(manifest_pkgtarxz.detected_format, PackageFormat::Pacman);

    let manifest_flatpak = inspector.inspect_package("gnome.flatpak", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_flatpak.detected_format, PackageFormat::Flatpak);

    let manifest_app = inspector.inspect_package("macos.app", b"app_payload");
    assert_eq!(manifest_app.detected_format, PackageFormat::App);

    let manifest_hap = inspector.inspect_package("harmony.hap", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_hap.detected_format, PackageFormat::Hap);

    let manifest_pisi = inspector.inspect_package("pardus.PiSi", &[0x50, 0x4b, 0x03, 0x04]);
    assert_eq!(manifest_pisi.detected_format, PackageFormat::Eopkg);

    let manifest_tgz = inspector.inspect_package("slackware.tgz", &[0x1f, 0x8b]);
    assert_eq!(manifest_tgz.detected_format, PackageFormat::TarGz);

    let manifest_superdeb =
        inspector.inspect_package("deepin.superdeb", b"!<arch>\ndebian-binary\n");
    assert_eq!(manifest_superdeb.detected_format, PackageFormat::Superdeb);

    let manifest_lzm = inspector.inspect_package("slax.lzm", b"hsqs_slax_lzm");
    assert_eq!(manifest_lzm.detected_format, PackageFormat::Lzm);

    let manifest_pup = inspector.inspect_package("puppy.pup", &[0x1f, 0x8b]);
    assert_eq!(manifest_pup.detected_format, PackageFormat::Pup);

    let manifest_snap = inspector.inspect_package("canonical.snap", b"hsqs_snap");
    assert_eq!(manifest_snap.detected_format, PackageFormat::Snap);

    let manifest_pacman = inspector.inspect_package("package.pacman", &[0x28, 0xb5, 0x2f, 0xfd]);
    assert_eq!(manifest_pacman.detected_format, PackageFormat::Pacman);

    let mut tar_bytes = vec![0u8; 300];
    tar_bytes[257..262].copy_from_slice(b"ustar");
    let manifest_tar = inspector.inspect_package("archive.tar", &tar_bytes);
    assert_eq!(manifest_tar.detected_format, PackageFormat::Tar);

    let manifest_pet = inspector.inspect_package("puppy.pet", &tar_bytes);
    assert_eq!(manifest_pet.detected_format, PackageFormat::Pet);

    // 2. OOP Design Patterns V89
    let mut facade = SovereignPackageFacadeV89::new();
    let fac_res = facade
        .quick_install("curl.apk", &[0x50, 0x4b, 0x03, 0x04])
        .expect("Facade install failed");
    assert!(fac_res.contains("curl"));

    // 3. Upstream Distro Delta Pre-Absorption Engine V89
    let delta_engine = UpstreamDistroDeltaPreAbsorptionEngineV89::new();
    assert!(delta_engine.total_deltas_absorbed() >= 5);

    // 4. SAT DPLL Dependency Resolution V89
    let solver = SatDpllDependencySolverV89::new();
    let raw_deps = vec![
        "libssl3".to_string(),
        "glibc".to_string(),
        "zlib1g".to_string(),
    ];
    let solved = solver
        .solve_dependencies("ripgrep", &raw_deps)
        .expect("SAT solver failed");
    assert_eq!(
        solved,
        vec![
            "sovereign-openssl",
            "sovereign-libc",
            "sovereign-zlib",
            "sovereign-base-v89"
        ]
    );

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

    // 8. Foreign PM CLI Command Router V89
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
