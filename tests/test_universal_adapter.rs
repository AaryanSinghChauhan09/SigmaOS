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

#[path = "../src/sigpkg/universal_engine.rs"]
pub mod universal_engine;

#[path = "../src/sigpkg/universal_adapter.rs"]
pub mod universal_adapter;

pub use universal_adapter::universal_oop_system;

pub mod sigpkg {
    use alloc::string::String;
    use alloc::vec::Vec;

    pub use crate::security;
    pub use crate::universal_adapter;
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

            let major_clean: String = major_str.chars().filter(|c| c.is_ascii_digit()).collect();
            let minor_clean: String = minor_str.chars().filter(|c| c.is_ascii_digit()).collect();
            let patch_clean: String = patch_str.chars().filter(|c| c.is_ascii_digit()).collect();

            let major = if major_clean.is_empty() { 0 } else { major_clean.parse::<u64>().unwrap_or(0) };
            let minor = if minor_clean.is_empty() { 0 } else { minor_clean.parse::<u64>().unwrap_or(0) };
            let patch = if patch_clean.is_empty() { 0 } else { patch_clean.parse::<u64>().unwrap_or(0) };

            Ok(Version::new(major, minor, patch))
        }
    }

    #[derive(Debug, Clone)]
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

    #[derive(Debug, Clone)]
    pub struct Dependency {
        pub name: String,
        pub version_constraint: VersionConstraint,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum VersionConstraint {
        Exact(Version),
        GreaterThan(Version),
        LessThan(Version),
        GreaterOrEqual(Version),
        LessOrEqual(Version),
        Any,
    }
}

#[test]
fn test_universal_adapter_all_formats() {
    use universal_adapter::{
        UniversalPackageAdapter, SigPkgUniversalBridgeEngine,
        UniversalPmCommandDispatcher, UniversalPmOperation,
        FreeBsdUclManifest, OpenBsdContentsManifest, NetBsdPkgsrcManifest,
        ZypperSpecManifest, SlackwarePkgManifest,
    };
    use sigpkg::Version;

    let adapter = UniversalPackageAdapter::new();

    // 1. FreeBSD UCL (+MANIFEST)
    let freebsd_data = "name: \"redis\"\nversion: \"7.0.11\"\ncomment: \"Persistent key-value store\"\ndeps { \"openssl\": { version: \"3.0.8\" } }\n";
    let ucl: FreeBsdUclManifest = adapter.parse_freebsd_ucl_manifest(freebsd_data).unwrap();
    assert_eq!(ucl.name, "redis");
    assert_eq!(ucl.version, "7.0.11");

    // 2. OpenBSD +CONTENTS
    let openbsd_data = "@name tmux-3.3a\n@comment Terminal multiplexer\n@depend libevent:libevent-2.1.12\n";
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
    let pkg_bsd = bridge.absorb_and_register("redis.pkg", freebsd_data.as_bytes()).unwrap();
    assert_eq!(pkg_bsd.name, "redis");
    assert_eq!(pkg_bsd.version, universal_adapter::Version::new(7, 0, 11));
    assert!(bridge.is_package_registered("redis"));

    let pkg_obsd = bridge.absorb_and_register("tmux.tgz", openbsd_data.as_bytes()).unwrap();
    assert_eq!(pkg_obsd.name, "tmux");
    assert_eq!(pkg_obsd.version, universal_adapter::Version::new(3, 3, 0));
    assert!(bridge.is_package_registered("tmux"));

    // 7. Command Dispatcher Multi-Distro Suite
    let dispatcher = UniversalPmCommandDispatcher::new();

    // APT
    let action = dispatcher.dispatch_command("apt install curl -y").unwrap();
    assert_eq!(action.source_pm, "apt");
    assert_eq!(action.operation, UniversalPmOperation::Install);
    assert_eq!(action.target_packages, vec!["curl"]);

    // Pacman
    let action_pacman = dispatcher.dispatch_command("pacman -S --noconfirm firefox").unwrap();
    assert_eq!(action_pacman.source_pm, "pacman");
    assert_eq!(action_pacman.operation, UniversalPmOperation::Install);
    assert_eq!(action_pacman.target_packages, vec!["firefox"]);

    // DNF / YUM
    let action_dnf = dispatcher.dispatch_command("dnf install htop").unwrap();
    assert_eq!(action_dnf.source_pm, "dnf");
    assert_eq!(action_dnf.operation, UniversalPmOperation::Install);
    assert_eq!(action_dnf.target_packages, vec!["htop"]);

    // Zypper
    let action_zypper = dispatcher.dispatch_command("zypper in --no-confirm vlc").unwrap();
    assert_eq!(action_zypper.source_pm, "zypper");
    assert_eq!(action_zypper.operation, UniversalPmOperation::Install);
    assert_eq!(action_zypper.target_packages, vec!["vlc"]);

    // APK (Alpine)
    let action_apk = dispatcher.dispatch_command("apk add musl-dev").unwrap();
    assert_eq!(action_apk.source_pm, "apk");
    assert_eq!(action_apk.operation, UniversalPmOperation::Install);
    assert_eq!(action_apk.target_packages, vec!["musl-dev"]);

    // XBPS (Void)
    let action_xbps = dispatcher.dispatch_command("xbps-install -S bash").unwrap();
    assert_eq!(action_xbps.source_pm, "xbps-install");
    assert_eq!(action_xbps.operation, UniversalPmOperation::Install);
    assert_eq!(action_xbps.target_packages, vec!["bash"]);

    // FreeBSD / OpenBSD pkg
    let action_pkg = dispatcher.dispatch_command("pkg install nginx").unwrap();
    assert_eq!(action_pkg.source_pm, "pkg");
    assert_eq!(action_pkg.operation, UniversalPmOperation::Install);
    assert_eq!(action_pkg.target_packages, vec!["nginx"]);

    // Gentoo emerge
    let action_emerge = dispatcher.dispatch_command("emerge -a sys-apps/portage").unwrap();
    assert_eq!(action_emerge.source_pm, "emerge");
    assert_eq!(action_emerge.target_packages, vec!["sys-apps/portage"]);
    assert!(action_emerge.dry_run);

    // Nix / Guix
    let action_nix = dispatcher.dispatch_command("nix-env -iA nixpkgs.git").unwrap();
    assert_eq!(action_nix.source_pm, "nix-env");
    assert_eq!(action_nix.operation, UniversalPmOperation::Install);
    assert_eq!(action_nix.target_packages, vec!["nixpkgs.git"]);

    // Flatpak & Snap
    let action_flatpak = dispatcher.dispatch_command("flatpak install org.gimp.GIMP").unwrap();
    assert_eq!(action_flatpak.source_pm, "flatpak");
    assert_eq!(action_flatpak.operation, UniversalPmOperation::Install);
    assert_eq!(action_flatpak.target_packages, vec!["org.gimp.GIMP"]);

    let action_snap = dispatcher.dispatch_command("snap install code --classic").unwrap();
    assert_eq!(action_snap.source_pm, "snap");
    assert_eq!(action_snap.operation, UniversalPmOperation::Install);
    assert_eq!(action_snap.target_packages, vec!["code"]);

    // Arch AUR helper (yay)
    let action_yay = dispatcher.dispatch_command("yay -S visual-studio-code-bin").unwrap();
    assert_eq!(action_yay.source_pm, "yay");
    assert_eq!(action_yay.operation, UniversalPmOperation::Install);
    assert_eq!(action_yay.target_packages, vec!["visual-studio-code-bin"]);
}
