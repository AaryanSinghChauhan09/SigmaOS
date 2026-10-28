// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V75
// (`tests/test_package_advancements_v75.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v75.rs"]
mod sovereign_distro_package_advancements_v75;

#[path = "../src/package/sovereign_distro_package_advancements_v75_pr.rs"]
mod sovereign_distro_package_advancements_v75_pr;

use sovereign_distro_package_advancements_v75::universal::{PackageFormat, PackageState};
use sovereign_distro_package_advancements_v75::*;
use sovereign_distro_package_advancements_v75_pr::SovereignDistroPackagePrGatewayV75;

#[test]
fn test_package_advancements_v75_full_cycle() {
    // 1. Universal Package Inspector V75 across requested package formats
    let mut inspector = UniversalPackageFormatInspectorV75::new();
    assert_eq!(inspector.total_formats_supported, 230);

    let formats_to_verify = vec![
        (
            "ripgrep-13.0.0-amd64.deb",
            b"!<arch>".as_slice(),
            PackageFormat::Deb,
        ),
        ("package.air", b"AIR_PAYLOAD".as_slice(), PackageFormat::Air),
        (
            "package.bottle",
            b"BOTTLE_PAYLOAD".as_slice(),
            PackageFormat::Bottle,
        ),
        ("app.ipa", b"IPA_PAYLOAD".as_slice(), PackageFormat::Ipa),
        (
            "bsd.ports",
            b"PORTS_PAYLOAD".as_slice(),
            PackageFormat::Ports,
        ),
        ("freebsd.pkg", b"PKG_PAYLOAD".as_slice(), PackageFormat::Pkg),
        ("app.aab", b"AAB_PAYLOAD".as_slice(), PackageFormat::Aab),
        ("app.apk", b"APK_PAYLOAD".as_slice(), PackageFormat::Apk),
        ("app.appimage", b"hsqs".as_slice(), PackageFormat::AppImage),
        (
            "solus.eopkg",
            b"EOPKG_PAYLOAD".as_slice(),
            PackageFormat::Eopkg,
        ),
        (
            "store.nixpkg",
            b"NIXPKG_PAYLOAD".as_slice(),
            PackageFormat::Nixpkg,
        ),
        (
            "gcc.ebuild",
            b"EBUILD_PAYLOAD".as_slice(),
            PackageFormat::Ebuild,
        ),
        (
            "archive.tar.gz",
            b"\x1f\x8bPAYLOAD".as_slice(),
            PackageFormat::TarGz,
        ),
        (
            "archive.tar .gz",
            b"\x1f\x8bPAYLOAD".as_slice(),
            PackageFormat::TarGz,
        ),
        (
            "archive.xz",
            b"\xfd\x37\x7a\x58\x5a\x00".as_slice(),
            PackageFormat::Xz,
        ),
        (
            "package.rpm",
            b"\xed\xab\xee\xdb".as_slice(),
            PackageFormat::Rpm,
        ),
        (
            "app.flatpak",
            b"FLATPAK_PAYLOAD".as_slice(),
            PackageFormat::Flatpak,
        ),
        ("app.app", b"APP_PAYLOAD".as_slice(), PackageFormat::App),
        ("app.hap", b"HAP_PAYLOAD".as_slice(), PackageFormat::Hap),
        (
            "package.PiSi",
            b"PISI_PAYLOAD".as_slice(),
            PackageFormat::Pisi,
        ),
        (
            "archive.tgz",
            b"\x1f\x8bPAYLOAD".as_slice(),
            PackageFormat::TarGz,
        ),
        (
            "deepin.superdeb",
            b"SUPERDEB_PAYLOAD".as_slice(),
            PackageFormat::Superdeb,
        ),
        ("slax.lzm", b"LZM_PAYLOAD".as_slice(), PackageFormat::Lzm),
        ("pup", b"PUP_PAYLOAD".as_slice(), PackageFormat::Pup),
        ("app.snap", b"b'hsqs'".as_slice(), PackageFormat::Snap),
        (
            "pacman",
            b"\x28\xb5\x2f\xfd".as_slice(),
            PackageFormat::Pacman,
        ),
        ("archive.tar", b"TAR_PAYLOAD".as_slice(), PackageFormat::Tar),
        ("package.pet", b"PET_PAYLOAD".as_slice(), PackageFormat::Pet),
    ];

    for (filename, payload, expected_fmt) in formats_to_verify {
        let manifest = inspector
            .inspect_package(filename, payload)
            .expect("Failed to inspect package format");
        assert_eq!(
            manifest.detected_format, expected_fmt,
            "Format mismatch for {}",
            filename
        );
        assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);
    }

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
    assert_eq!(delta.compression_savings_ratio, 90);

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

    // 8. PR Gateway V75
    let mut gateway = SovereignDistroPackagePrGatewayV75::new();
    gateway.submit_proposal(
        7501,
        "Universal Package Advancements V75",
        sovereign_distro_package_advancements_v75_pr::universal::PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V75",
    );
    assert_eq!(gateway.total_proposals(), 1);

    // 9. Master Coordinator Suite V75
    let mut master_suite = SovereignDistroPackageAdvancementsSuiteV75::new();
    let installed_pkg = master_suite
        .process_and_install("htop-3.3.0.apk", b"APK_PAYLOAD")
        .expect("Master suite installation failed");
    assert_eq!(installed_pkg.name, "htop");
    assert_eq!(installed_pkg.state, PackageState::Installed);
    assert!(master_suite
        .installed_packages
        .contains(&"sigpkg-v75-htop".to_string()));
}
