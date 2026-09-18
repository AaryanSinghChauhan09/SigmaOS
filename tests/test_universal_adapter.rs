// Standalone Test Runner for SigmaOS Universal Package Format Adapter
extern crate alloc;
extern crate core;

pub mod klib {
    pub mod collections {
        pub use alloc::collections::BTreeMap as HashMap;
    }
}

#[path = "../src/package/universal.rs"]
pub mod universal;

pub mod package {
    pub use crate::universal;
    pub use crate::universal::*;
}

#[path = "../src/security/capability.rs"]
pub mod capability;

pub mod security {
    pub use super::capability::*;
}

#[path = "../src/sigpkg/universal_engine.rs"]
pub mod universal_engine;

#[path = "../src/sigpkg/universal_oop_system.rs"]
pub mod universal_oop_system;

pub mod sigpkg {
    pub use crate::security;
    pub use crate::universal_engine;
    pub use crate::universal_engine::PackageFormat;
    pub use crate::universal_oop_system;

    pub use crate::universal_oop_system::{Dependency, Package, Version, VersionConstraint};
}

#[path = "../src/sigpkg/universal_adapter.rs"]
pub mod universal_adapter;

#[test]
fn test_universal_adapter_all_formats() {
    use universal_adapter::{
        FreeBsdUclManifest, NetBsdPkgsrcManifest, OpenBsdContentsManifest,
        SigPkgUniversalBridgeEngine, SlackwarePkgManifest, UniversalPackageAdapter,
        UniversalPmCommandDispatcher, UniversalPmOperation, ZypperSpecManifest,
    };

    let adapter = UniversalPackageAdapter::new();

    // 1. FreeBSD UCL (+MANIFEST)
    let freebsd_data = "name: \"redis\"\nversion: \"7.0.11\"\ncomment: \"Persistent key-value store\"\ndeps { \"openssl\": { version: \"3.0.8\" } }\n";
    let ucl: FreeBsdUclManifest = adapter.parse_freebsd_ucl_manifest(freebsd_data).unwrap();
    assert_eq!(ucl.name, "redis");
    assert_eq!(ucl.version, "7.0.11");

    // 2. OpenBSD +CONTENTS
    let openbsd_data =
        "@name tmux-3.3a\n@comment Terminal multiplexer\n@depend libevent:libevent-2.1.12\n";
    let obsd: OpenBsdContentsManifest = adapter.parse_openbsd_contents(openbsd_data).unwrap();
    assert_eq!(obsd.pkgname, "tmux");
    assert_eq!(obsd.version, "3.3a");

    // 3. NetBSD pkgsrc
    let netbsd_data = "PKGNAME=git-base-2.41.0\nCOMMENT=Git SCM\nREQUIRES=openssl\n";
    let nbsd: NetBsdPkgsrcManifest = adapter.parse_netbsd_pkgsrc(netbsd_data).unwrap();
    assert_eq!(nbsd.pkgname, "git-base");

    // 4. openSUSE Zypper RPM spec
    let zypper_data = "Name: opensuse-kernel\nVersion: 6.5.0\nSummary: Kernel\nRequires: glibc\n";
    let zyp: ZypperSpecManifest = adapter.parse_zypper_spec(zypper_data).unwrap();
    assert_eq!(zyp.name, "opensuse-kernel");

    // 5. Slackware pkg / SlackBuild
    let slack_data = "PRGNAM=\"slack-tool\"\nVERSION=\"1.0\"\nSLACK_REQUIRED=\"glibc\"\n";
    let slk: SlackwarePkgManifest = adapter.parse_slackware_pkg(slack_data).unwrap();
    assert_eq!(slk.name, "slack-tool");

    // 6. Universal Bridge Engine Absorption
    let mut bridge = SigPkgUniversalBridgeEngine::new();
    let pkg_bsd = bridge
        .absorb_and_register("redis.pkg", freebsd_data.as_bytes())
        .unwrap();
    assert_eq!(pkg_bsd.name, "redis");
    assert_eq!(pkg_bsd.version, sigpkg::Version::new(7, 0, 11));
    assert!(bridge.is_package_registered("redis"));

    let pkg_obsd = bridge
        .absorb_and_register("tmux.openbsd.tgz", openbsd_data.as_bytes())
        .unwrap();
    assert_eq!(pkg_obsd.name, "tmux");
    assert_eq!(pkg_obsd.version, sigpkg::Version::new(3, 3, 0));
    assert!(bridge.is_package_registered("tmux"));

    // 7. Command Dispatcher
    let dispatcher = UniversalPmCommandDispatcher::new();
    let action = dispatcher.dispatch_command("apt install curl -y").unwrap();
    assert_eq!(action.source_pm, "apt");
    assert_eq!(action.operation, UniversalPmOperation::Install);
    assert_eq!(action.target_packages, vec!["curl"]);
}

#[test]
fn test_universal_adapter_extended_linux_bsd_formats() {
    use universal_adapter::{
        UniversalPackageAdapter, UniversalPmCommandDispatcher, UniversalPmOperation,
    };

    let adapter = UniversalPackageAdapter::new();

    // Test Extension Detection for Linux & BSD Formats
    assert_eq!(
        adapter.detect_format_by_extension("pkg.ipk"),
        Some(universal_engine::PackageFormat::Ipk)
    );
    assert_eq!(
        adapter.detect_format_by_extension("pkg.opkg"),
        Some(universal_engine::PackageFormat::Opkg)
    );
    assert_eq!(
        adapter.detect_format_by_extension("pkg.p5p"),
        Some(universal_engine::PackageFormat::SolarisIps)
    );
    assert_eq!(
        adapter.detect_format_by_extension("pkg.nar"),
        Some(universal_engine::PackageFormat::GuixNar)
    );
    assert_eq!(
        adapter.detect_format_by_extension("pkg.openbsd.tgz"),
        Some(universal_engine::PackageFormat::OpenBsdPkg)
    );
    assert_eq!(
        adapter.detect_format_by_extension("pkg.moss"),
        Some(universal_engine::PackageFormat::Moss)
    );
    assert_eq!(
        adapter.detect_format_by_extension("pkg.hpkg"),
        Some(universal_engine::PackageFormat::Hpkg)
    );

    // Test Magic Header Detection
    assert_eq!(
        adapter.detect_format_by_header(b"IPK!1234"),
        Some(universal_engine::PackageFormat::Ipk)
    );
    assert_eq!(
        adapter.detect_format_by_header(b"OPKG1234"),
        Some(universal_engine::PackageFormat::Opkg)
    );
    assert_eq!(
        adapter.detect_format_by_header(b"P5P!1234"),
        Some(universal_engine::PackageFormat::SolarisIps)
    );
    assert_eq!(
        adapter.detect_format_by_header(b"NARS1234"),
        Some(universal_engine::PackageFormat::GuixNar)
    );
    assert_eq!(
        adapter.detect_format_by_header(b"OBSD1234"),
        Some(universal_engine::PackageFormat::OpenBsdPkg)
    );

    // Test Command Dispatcher across multiple package managers
    let dispatcher = UniversalPmCommandDispatcher::new();

    let pacman_cmd = dispatcher.dispatch_command("pacman -S zsh").unwrap();
    assert_eq!(pacman_cmd.source_pm, "pacman");
    assert_eq!(pacman_cmd.operation, UniversalPmOperation::Install);
    assert_eq!(pacman_cmd.target_packages, vec!["zsh"]);

    let dnf_cmd = dispatcher.dispatch_command("dnf install htop").unwrap();
    assert_eq!(dnf_cmd.source_pm, "dnf");
    assert_eq!(dnf_cmd.operation, UniversalPmOperation::Install);
    assert_eq!(dnf_cmd.target_packages, vec!["htop"]);

    let apk_cmd = dispatcher.dispatch_command("apk add bash").unwrap();
    assert_eq!(apk_cmd.source_pm, "apk");
    assert_eq!(apk_cmd.operation, UniversalPmOperation::Install);
    assert_eq!(apk_cmd.target_packages, vec!["bash"]);
}

#[test]
fn test_all_prompt_package_formats() {
    use universal_adapter::UniversalPackageAdapter;
    use universal_engine::PackageFormat;

    let adapter = UniversalPackageAdapter::new();

    // Verify detection for all 29 Linux & BSD distro formats specified in prompt
    assert_eq!(
        adapter.detect_format_by_extension("app.air"),
        Some(PackageFormat::Air)
    );
    assert_eq!(
        adapter.detect_format_by_extension("pkg.bottle"),
        Some(PackageFormat::Bottle)
    );
    assert_eq!(
        adapter.detect_format_by_extension("app.ipa"),
        Some(PackageFormat::Ipa)
    );
    assert_eq!(
        adapter.detect_format_by_extension("bsd.ports"),
        Some(PackageFormat::Ports)
    );
    assert_eq!(
        adapter.detect_format_by_extension("mac.pkg"),
        Some(PackageFormat::Pkg)
    );
    assert_eq!(
        adapter.detect_format_by_extension("app.aab"),
        Some(PackageFormat::Aab)
    );
    assert_eq!(
        adapter.detect_format_by_extension("app.apk"),
        Some(PackageFormat::Apk)
    );
    assert_eq!(
        adapter.detect_format_by_extension("app.AppImage"),
        Some(PackageFormat::AppImage)
    );
    assert_eq!(
        adapter.detect_format_by_extension("solus.eopkg"),
        Some(PackageFormat::Eopkg)
    );
    assert_eq!(
        adapter.detect_format_by_extension("nix.nixpkg"),
        Some(PackageFormat::Nix)
    );
    assert_eq!(
        adapter.detect_format_by_extension("gentoo.portage"),
        Some(PackageFormat::Portage)
    );
    assert_eq!(
        adapter.detect_format_by_extension("debian.deb"),
        Some(PackageFormat::Deb)
    );
    assert_eq!(
        adapter.detect_format_by_extension("archive.tar.gz"),
        Some(PackageFormat::TarGz)
    );
    assert_eq!(
        adapter.detect_format_by_extension("compressed.xz"),
        Some(PackageFormat::TarXz)
    );
    assert_eq!(
        adapter.detect_format_by_extension("fedora.rpm"),
        Some(PackageFormat::Rpm)
    );
    assert_eq!(
        adapter.detect_format_by_extension("gentoo.ebuild"),
        Some(PackageFormat::Portage)
    );
    assert_eq!(
        adapter.detect_format_by_extension("arch.pkg.tar.xz"),
        Some(PackageFormat::Pacman)
    );
    assert_eq!(
        adapter.detect_format_by_extension("app.flatpak"),
        Some(PackageFormat::Flatpak)
    );
    assert_eq!(
        adapter.detect_format_by_extension("macos.app"),
        Some(PackageFormat::AppBundle)
    );
    assert_eq!(
        adapter.detect_format_by_extension("harmony.hap"),
        Some(PackageFormat::Hap)
    );
    assert_eq!(
        adapter.detect_format_by_extension("pardus.PiSi"),
        Some(PackageFormat::Pisi)
    );
    assert_eq!(
        adapter.detect_format_by_extension("archive.tgz"),
        Some(PackageFormat::TarGz)
    );
    assert_eq!(
        adapter.detect_format_by_extension("deepin.superdeb"),
        Some(PackageFormat::Superdeb)
    );
    assert_eq!(
        adapter.detect_format_by_extension("slax.lzm"),
        Some(PackageFormat::Lzm)
    );
    assert_eq!(
        adapter.detect_format_by_extension("puppy.pup"),
        Some(PackageFormat::Pup)
    );
    assert_eq!(
        adapter.detect_format_by_extension("canonical.snap"),
        Some(PackageFormat::Snap)
    );
    assert_eq!(
        adapter.detect_format_by_extension("pacman.pkg.tar.zst"),
        Some(PackageFormat::Pacman)
    );
    assert_eq!(
        adapter.detect_format_by_extension("plain.tar"),
        Some(PackageFormat::Tar)
    );
    assert_eq!(
        adapter.detect_format_by_extension("puppy.pet"),
        Some(PackageFormat::Pet)
    );
    assert_eq!(
        adapter.detect_format_by_extension("clearlinux.swupd"),
        Some(PackageFormat::Swupd)
    );
    assert_eq!(
        adapter.detect_format_by_extension("starling.starling"),
        Some(PackageFormat::Starling)
    );
    assert_eq!(
        adapter.detect_format_by_extension("cachy.cachyos"),
        Some(PackageFormat::Pacman)
    );
    assert_eq!(
        adapter.detect_format_by_extension("clearlinux.swupd"),
        Some(PackageFormat::Swupd)
    );
    assert_eq!(
        adapter.detect_format_by_extension("starling.starling"),
        Some(PackageFormat::Starling)
    );
    assert_eq!(
        adapter.detect_format_by_extension("cachy.cachyos"),
        Some(PackageFormat::Pacman)
    );
}

#[test]
fn test_all_unified_package_formats_resolution() {
    use universal_adapter::UniversalPackageAdapter;
    use universal_engine::PackageFormat;
    let adapter = UniversalPackageAdapter::new();
    let formats = vec![
        ("demo.swupd", PackageFormat::Swupd),
        ("demo.starling", PackageFormat::Starling),
        ("demo.cachyos", PackageFormat::Pacman),
        ("demo.deb", PackageFormat::Deb),
        ("demo.rpm", PackageFormat::Rpm),
        ("demo.apk", PackageFormat::Apk),
        ("demo.xbps", PackageFormat::Xbps),
    ];

    for (file, fmt) in formats {
        assert_eq!(adapter.detect_format_by_extension(file), Some(fmt));
        let manifest = if file.ends_with(".deb") {
            "Package: demo\nVersion: 1.0.0\n"
        } else if file.ends_with(".rpm") {
            "Name: demo\nVersion: 1.0.0\n"
        } else if file.ends_with(".apk") {
            "P:demo\nV:1.0.0\n"
        } else if file.ends_with(".xbps") {
            "pkgname=\"demo\"\nversion=\"1.0.0\"\n"
        } else if file.ends_with(".cachyos") {
            "pkgname=demo\npkgver=1.0.0\n"
        } else {
            "Package: demo\nVersion: 1.0.0\n"
        };
        let pkg = adapter
            .parse_and_translate_manifest(file, manifest)
            .expect("manifest parsing should succeed");
        assert_eq!(pkg.name, "demo");
    }
}
