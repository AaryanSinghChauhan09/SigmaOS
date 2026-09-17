// Standalone Test Runner for SigmaOS Universal Package Format Adapter
extern crate alloc;

pub mod klib {
    pub mod collections {
        pub use alloc::collections::BTreeMap as HashMap;
    }
}

#[path = "../src/security/capability.rs"]
pub mod capability;

pub mod security {
    pub use super::capability::*;
}

pub mod package {
    use alloc::string::String;
    use alloc::vec::Vec;
    use crate::universal_adapter::PackagePriority;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AptDebManifest {
        pub package: String,
        pub version: String,
        pub depends: Vec<String>,
        pub description: String,
        pub priority: PackagePriority,
    }
}

#[path = "../src/sigpkg/universal_engine.rs"]
pub mod universal_engine;

pub use universal_engine::PackageFormat;

#[path = "../src/sigpkg/universal_oop_system.rs"]
pub mod universal_oop_system;

pub mod sigpkg {
    use alloc::string::String;
    use alloc::vec::Vec;

    pub use crate::security;
    pub use crate::universal_engine;
    pub use crate::universal_oop_system;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Version {
        pub major: u64,
        pub minor: u64,
        pub patch: u64,
    }

    impl Version {
        pub fn new(major: u64, minor: u64, patch: u64) -> Self {
            Self { major, minor, patch }
        }

        pub fn parse(version_str: &str) -> Result<Self, &'static str> {
            let clean = version_str.split('-').next().unwrap_or(version_str);
            let mut parts = clean.split('.');

            let major_str = parts.next().unwrap_or("0");
            let minor_str = parts.next().unwrap_or("0");
            let patch_str = parts.next().unwrap_or("0");

            let major = major_str.parse::<u64>().unwrap_or(0);
            let minor = minor_str.parse::<u64>().unwrap_or(0);
            let patch = patch_str.parse::<u64>().unwrap_or(0);

            Ok(Self { major, minor, patch })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum VersionConstraint {
        Any,
        Exact(Version),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Dependency {
        pub name: String,
        pub version_constraint: VersionConstraint,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Package {
        pub name: String,
        pub version: Version,
        pub description: String,
        pub dependencies: Vec<Dependency>,
        pub checksum: String,
    }

    impl Package {
        pub fn new(
            name: String,
            version: Version,
            description: String,
            dependencies: Vec<Dependency>,
            checksum: String,
        ) -> Self {
            Self {
                name,
                version,
                description,
                dependencies,
                checksum,
            }
        }
    }
}

#[path = "../src/sigpkg/universal_adapter.rs"]
pub mod universal_adapter;

#[test]
fn test_universal_adapter_all_formats() {
    use universal_adapter::{
        AppImageContainer, FreeBsdUclManifest, OpenBsdContentsManifest, SigPkgUniversalBridgeEngine,
        SlackwarePkgManifest, UniversalPackageAdapter, UniversalPmCommandDispatcher,
        UniversalPmOperation,
    };

    let adapter = UniversalPackageAdapter::new();

    // 1. Apt Control parsing (.deb)
    let deb_control = "Package: nginx\nVersion: 1.24.0\nDepends: libc6, libssl3\nDescription: Web server\n";
    let deb = adapter.parse_apt_control(deb_control).unwrap();
    assert_eq!(deb.package, "nginx");
    assert_eq!(deb.version, "1.24.0");

    // 2. FreeBSD UCL +MANIFEST
    let freebsd_data = "name: \"redis\"\nversion: \"7.0.11\"\ncomment: In-memory DB\ndeps {\n  \"openssl\": {origin: \"security/openssl\"}\n}\n";
    let bsd: FreeBsdUclManifest = adapter.parse_freebsd_ucl_manifest(freebsd_data).unwrap();
    assert_eq!(bsd.name, "redis");
    assert_eq!(bsd.deps.len(), 1);

    // 3. OpenBSD +CONTENTS
    let openbsd_data = "@name tmux-3.3a\n@comment Terminal multiplexer\n@depend devel/libevent\n";
    let obsd: OpenBsdContentsManifest = adapter.parse_openbsd_contents(openbsd_data).unwrap();
    assert_eq!(obsd.pkgname, "tmux");

    // 4. AppImage single-file container
    let mut appimage = AppImageContainer::new("Vlc.AppImage", "vlc");
    let exec = appimage.mount_and_run("/tmp/vlc_mount").unwrap();
    assert_eq!(exec, "/tmp/vlc_mount/vlc");

    // 5. Slackware pkg / SlackBuild
    let slack_data = "PRGNAM=\"slack-tool\"\nVERSION=\"1.0\"\nSLACK_REQUIRED=\"glibc\"\n";
    let slk: SlackwarePkgManifest = adapter.parse_slackware_pkg(slack_data).unwrap();
    assert_eq!(slk.name, "slack-tool");

    // 6. Universal Bridge Engine Absorption
    let mut bridge = SigPkgUniversalBridgeEngine::new();
    let pkg_bsd = bridge.absorb_and_register("redis.pkg", freebsd_data.as_bytes()).unwrap();
    assert_eq!(pkg_bsd.name, "redis");
    assert_eq!(pkg_bsd.version, sigpkg::Version::new(7, 0, 11));
    assert!(bridge.is_package_registered("redis"));

    let pkg_obsd = bridge.absorb_and_register("tmux.tgz", openbsd_data.as_bytes()).unwrap();
    assert_eq!(pkg_obsd.name, "tmux");
    assert_eq!(pkg_obsd.version, sigpkg::Version::new(3, 0, 0));
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
    use universal_adapter::UniversalPackageAdapter;

    let adapter = UniversalPackageAdapter::new();

    // Verify format detection by filename extensions
    assert_eq!(adapter.detect_format_by_extension("router.ipk"), Some(PackageFormat::Ipk));
    assert_eq!(adapter.detect_format_by_extension("embedded.opkg"), Some(PackageFormat::Opkg));
    assert_eq!(adapter.detect_format_by_extension("solaris.p5p"), Some(PackageFormat::SolarisIps));
    assert_eq!(adapter.detect_format_by_extension("store.nar"), Some(PackageFormat::GuixNar));
    assert_eq!(adapter.detect_format_by_extension("base.openbsd.tgz"), Some(PackageFormat::OpenBsdPkg));

    // Verify header byte signatures
    assert_eq!(adapter.detect_format_by_header(b"IPK!1234"), Some(PackageFormat::Ipk));
    assert_eq!(adapter.detect_format_by_header(b"OPKG1234"), Some(PackageFormat::Opkg));
    assert_eq!(adapter.detect_format_by_header(b"P5P!1234"), Some(PackageFormat::SolarisIps));
    assert_eq!(adapter.detect_format_by_header(b"NARS1234"), Some(PackageFormat::GuixNar));
    assert_eq!(adapter.detect_format_by_header(b"OBSD1234"), Some(PackageFormat::OpenBsdPkg));
}

#[test]
fn test_all_prompt_package_formats() {
    use universal_adapter::UniversalPackageAdapter;
    use crate::universal_engine::PackageFormat;

    let adapter = UniversalPackageAdapter::new();

    // Verify detection for all 29 Linux & BSD distro formats specified in prompt
    assert_eq!(adapter.detect_format_by_extension("app.air"), Some(PackageFormat::Air));
    assert_eq!(adapter.detect_format_by_extension("pkg.bottle"), Some(PackageFormat::Bottle));
    assert_eq!(adapter.detect_format_by_extension("app.ipa"), Some(PackageFormat::Ipa));
    assert_eq!(adapter.detect_format_by_extension("bsd.ports"), Some(PackageFormat::Ports));
    assert_eq!(adapter.detect_format_by_extension("mac.pkg"), Some(PackageFormat::Pkg));
    assert_eq!(adapter.detect_format_by_extension("app.aab"), Some(PackageFormat::Aab));
    assert_eq!(adapter.detect_format_by_extension("app.apk"), Some(PackageFormat::Apk));
    assert_eq!(adapter.detect_format_by_extension("app.AppImage"), Some(PackageFormat::AppImage));
    assert_eq!(adapter.detect_format_by_extension("solus.eopkg"), Some(PackageFormat::Eopkg));
    assert_eq!(adapter.detect_format_by_extension("nix.nixpkg"), Some(PackageFormat::Nix));
    assert_eq!(adapter.detect_format_by_extension("gentoo.portage"), Some(PackageFormat::Portage));
    assert_eq!(adapter.detect_format_by_extension("debian.deb"), Some(PackageFormat::Apt));
    assert_eq!(adapter.detect_format_by_extension("archive.tar.gz"), Some(PackageFormat::TarGz));
    assert_eq!(adapter.detect_format_by_extension("archive.tar .gz"), Some(PackageFormat::TarGz));
    assert_eq!(adapter.detect_format_by_extension("compressed.xz"), Some(PackageFormat::TarXz));
    assert_eq!(adapter.detect_format_by_extension("fedora.rpm"), Some(PackageFormat::Yum));
    assert_eq!(adapter.detect_format_by_extension("gentoo.ebuild"), Some(PackageFormat::Portage));
    assert_eq!(adapter.detect_format_by_extension("arch.pkg.tar.xz"), Some(PackageFormat::Pacman));
    assert_eq!(adapter.detect_format_by_extension("app.flatpak"), Some(PackageFormat::Flatpak));
    assert_eq!(adapter.detect_format_by_extension("macos.app"), Some(PackageFormat::AppBundle));
    assert_eq!(adapter.detect_format_by_extension("harmony.hap"), Some(PackageFormat::Hap));
    assert_eq!(adapter.detect_format_by_extension("pardus.PiSi"), Some(PackageFormat::Pisi));
    assert_eq!(adapter.detect_format_by_extension("archive.tgz"), Some(PackageFormat::TarGz));
    assert_eq!(adapter.detect_format_by_extension("deepin.superdeb"), Some(PackageFormat::Superdeb));
    assert_eq!(adapter.detect_format_by_extension("slax.lzm"), Some(PackageFormat::Lzm));
    assert_eq!(adapter.detect_format_by_extension("puppy.pup"), Some(PackageFormat::Pup));
    assert_eq!(adapter.detect_format_by_extension("canonical.snap"), Some(PackageFormat::Snap));
    assert_eq!(adapter.detect_format_by_extension("plain.tar"), Some(PackageFormat::Tar));
    assert_eq!(adapter.detect_format_by_extension("puppy.pet"), Some(PackageFormat::Pet));
}
