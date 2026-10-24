// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements V69 Engine & PR Gateway

// The #[path]-included src files reference `crate::package::universal` in their
// non-standalone_test branch. Provide that path by re-exporting the compiled lib
// module (mirrors tests/test_universal_adapter.rs and sigpkg_cli_verification_test.rs).
mod package {
    pub use sigmaos::package::universal;
}

#[path = "../src/package/sovereign_distro_package_advancements_v69.rs"]
mod sovereign_distro_package_advancements_v69;

#[path = "../src/package/sovereign_distro_package_advancements_v69_pr.rs"]
mod sovereign_distro_package_advancements_v69_pr;

use package::universal::{PackageFormat, PackageState};
use sovereign_distro_package_advancements_v69::*;
use sovereign_distro_package_advancements_v69_pr::SovereignDistroPackagePrGatewayV69;

#[test]
fn test_package_advancements_v69_full_cycle() {
    let mut suite = SovereignDistroPackageAdvancementsSuiteV69::new();

    // 1. Process and install package across formats (.air, .bottle, .ipa, .ports, .pkg, .aab, .apk, AppImage, .eopkg, .nixpkg, .portage, .deb, .tar.gz, .xz, .rpm, .ebuild, .pkg.tar.xz, Flatpak, .app, .hap, .PiSi, .tgz, .superdeb, .lzm, pup, .snap, pacman, .tar, .pet, etc.)
    let formats_to_test = vec![
        (
            "ripgrep-13.0.0.deb",
            b"DEB_PAYLOAD"[..].to_vec(),
            PackageFormat::Deb,
        ),
        (
            "app-1.0.0.apk",
            b"APK_PAYLOAD"[..].to_vec(),
            PackageFormat::Apk,
        ),
        (
            "tool-2.1.0.superdeb",
            b"SUPERDEB_PAYLOAD"[..].to_vec(),
            PackageFormat::Superdeb,
        ),
        (
            "bundle-0.5.0.air",
            b"AIR_PAYLOAD"[..].to_vec(),
            PackageFormat::Air,
        ),
        (
            "formula-1.2.3.bottle",
            b"BOTTLE_PAYLOAD"[..].to_vec(),
            PackageFormat::Bottle,
        ),
        (
            "ios-app-1.0.ipa",
            b"IPA_PAYLOAD"[..].to_vec(),
            PackageFormat::Ipa,
        ),
        (
            "bsd-tool-1.0.ports",
            b"PORTS_PAYLOAD"[..].to_vec(),
            PackageFormat::Ports,
        ),
        (
            "bundle-1.0.aab",
            b"AAB_PAYLOAD"[..].to_vec(),
            PackageFormat::Aab,
        ),
        (
            "mac-app-1.0.app",
            b"APP_PAYLOAD"[..].to_vec(),
            PackageFormat::App,
        ),
        (
            "harmony-1.0.hap",
            b"HAP_PAYLOAD"[..].to_vec(),
            PackageFormat::Hap,
        ),
        (
            "pardus-1.0.pisi",
            b"PISI_PAYLOAD"[..].to_vec(),
            PackageFormat::Pisi,
        ),
        (
            "slax-1.0.lzm",
            b"LZM_PAYLOAD"[..].to_vec(),
            PackageFormat::Lzm,
        ),
        (
            "puppy-1.0.pup",
            b"PUP_PAYLOAD"[..].to_vec(),
            PackageFormat::Pup,
        ),
        (
            "pet-1.0.pet",
            b"PET_PAYLOAD"[..].to_vec(),
            PackageFormat::Pet,
        ),
    ];

    for (fname, payload, expected_fmt) in formats_to_test {
        let pkg = suite.process_and_install(fname, &payload).unwrap();
        assert_eq!(pkg.formats, vec![expected_fmt]);
        assert_eq!(pkg.state, PackageState::Installed);
    }

    assert_eq!(suite.installed_packages.len(), 14);

    // 2. Test PR Gateway V69
    let mut gateway = SovereignDistroPackagePrGatewayV69::new();
    gateway.submit_proposal(
        6901,
        "Universal Interop for Debian DEB in Sigma-pkg V69",
        PackageFormat::Deb,
        "ripgrep",
        "Added zero-copy format classification and PQC attestation verification",
    );

    gateway.submit_proposal(
        6902,
        "Universal Interop for Deepin Superdeb in Sigma-pkg V69",
        PackageFormat::Superdeb,
        "deepin-desktop",
        "Added PQC Dilithium5 signature verification and zero-copy inspection",
    );

    assert_eq!(gateway.total_proposals(), 2);
    let prop1 = gateway.get_proposal(6901).unwrap();
    assert_eq!(prop1.target_package, "ripgrep");
    assert_eq!(prop1.verified_slsa_level, 4);

    let prop2 = gateway.get_proposal(6902).unwrap();
    assert_eq!(prop2.target_package, "deepin-desktop");
    assert_eq!(prop2.source_format, PackageFormat::Superdeb);
}
