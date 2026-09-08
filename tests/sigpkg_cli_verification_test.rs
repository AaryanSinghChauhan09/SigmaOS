// SigmaOS Universal Package Manager CLI & Adapter Integration Tests
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

#[path = "../src/sigpkg/universal_oop_system.rs"]
pub mod universal_oop_system;

#[path = "../src/sigpkg/universal_adapter.rs"]
pub mod universal_adapter;

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

    impl core::fmt::Display for Version {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }

    impl Version {
        pub fn new(major: u64, minor: u64, patch: u64) -> Self {
            Self {
                major,
                minor,
                patch,
            }
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

            let major = major_clean.parse::<u64>().unwrap_or(0);
            let minor = minor_clean.parse::<u64>().unwrap_or(0);
            let patch = patch_clean.parse::<u64>().unwrap_or(0);

            Ok(Self {
                major,
                minor,
                patch,
            })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum VersionConstraint {
        Any,
        Exact(Version),
        Min(Version),
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

use universal_adapter::{
    SigmaPkgHookType, UniversalDependencyMapper, UniversalDryRunSimulator,
    UniversalFormatConverter, UniversalPackageAdapter, UniversalPmCommandDispatcher,
    UniversalPmOperation, UniversalSandboxCapabilityMatrix, UniversalScriptletConverter,
};
use universal_engine::PackageFormat;

#[test]
fn test_bsd_and_linux_manifest_parsers() {
    let adapter = UniversalPackageAdapter::new();

    // FreeBSD UCL +MANIFEST
    let freebsd_ucl = r#"
        name: "nginx"
        origin: "www/nginx"
        version: "1.24.0"
        comment: "Robust HTTP server and reverse proxy"
        deps {
            "openssl": { origin: "security/openssl", version: "3.0.8" }
        }
    "#;
    let ucl_parsed = adapter.parse_freebsd_ucl_manifest(freebsd_ucl).unwrap();
    assert_eq!(ucl_parsed.name, "nginx");
    assert_eq!(ucl_parsed.version, "1.24.0");
    assert_eq!(ucl_parsed.deps.len(), 1);

    // OpenBSD +CONTENTS
    let openbsd_contents = r#"
        @name rsync-3.2.7p0
        @comment Remote file copy tool
        @depend net/rsync:rsync-3.2.7
        @exec echo "Installing rsync..."
        @unexec echo "Removing rsync..."
    "#;
    let obs_parsed = adapter.parse_openbsd_contents(openbsd_contents).unwrap();
    assert_eq!(obs_parsed.pkgname, "rsync");
    assert_eq!(obs_parsed.version, "3.2.7p0");
    assert_eq!(obs_parsed.depends.len(), 1);

    // NetBSD pkgsrc
    let netbsd_pkgsrc = r#"
        PKGNAME=git-2.41.0
        COMMENT=Fast distributed revision control system
        DEPENDS=security/openssl
    "#;
    let net_parsed = adapter.parse_netbsd_pkgsrc(netbsd_pkgsrc).unwrap();
    assert_eq!(net_parsed.pkgname, "git");
    assert_eq!(net_parsed.version, "2.41.0");

    // Slackware
    let slack_pkg = r#"
        PRGNAM=htop
        VERSION=3.2.2
        SHORT_DESCRIPTION=Interactive process viewer
        SLACK_REQUIRED=ncurses,zlib
    "#;
    let slack_parsed = adapter.parse_slackware_pkg(slack_pkg).unwrap();
    assert_eq!(slack_parsed.name, "htop");
    assert_eq!(slack_parsed.slack_required.len(), 2);
}

#[test]
fn test_universal_pm_command_dispatcher_all_distros() {
    let dispatcher = UniversalPmCommandDispatcher::new();

    // APT
    let apt_act = dispatcher
        .dispatch_command("apt install nginx curl -y")
        .unwrap();
    assert_eq!(apt_act.source_pm, "apt");
    assert_eq!(apt_act.operation, UniversalPmOperation::Install);
    assert_eq!(apt_act.target_packages, vec!["nginx", "curl"]);

    // DNF
    let dnf_act = dispatcher.dispatch_command("dnf remove httpd").unwrap();
    assert_eq!(dnf_act.source_pm, "dnf");
    assert_eq!(dnf_act.operation, UniversalPmOperation::Remove);

    // Pacman
    let pac_act = dispatcher.dispatch_command("pacman -Syu --print").unwrap();
    assert_eq!(pac_act.source_pm, "pacman");
    assert_eq!(pac_act.operation, UniversalPmOperation::Upgrade);
    assert!(pac_act.dry_run);

    // APK
    let apk_act = dispatcher.dispatch_command("apk add musl-dev").unwrap();
    assert_eq!(apk_act.source_pm, "apk");
    assert_eq!(apk_act.operation, UniversalPmOperation::Install);

    // PKG (FreeBSD)
    let pkg_act = dispatcher
        .dispatch_command("pkg install -n postgresql15")
        .unwrap();
    assert_eq!(pkg_act.source_pm, "pkg");
    assert_eq!(pkg_act.operation, UniversalPmOperation::Install);
    assert!(pkg_act.dry_run);

    // Zypper
    let zyp_act = dispatcher.dispatch_command("zypper in gcc").unwrap();
    assert_eq!(zyp_act.source_pm, "zypper");
    assert_eq!(zyp_act.operation, UniversalPmOperation::Install);

    // XBPS
    let xbps_act = dispatcher.dispatch_command("xbps-install -S bash").unwrap();
    assert_eq!(xbps_act.source_pm, "xbps-install");
    assert_eq!(xbps_act.operation, UniversalPmOperation::Install);
}

#[test]
fn test_universal_scriptlet_and_dependency_mapper() {
    let dep_mapper = UniversalDependencyMapper::new();
    assert_eq!(dep_mapper.to_canonical_name("libssl-dev"), "openssl");
    assert_eq!(dep_mapper.to_canonical_name("openssl-devel"), "openssl");
    assert_eq!(dep_mapper.to_canonical_name("libc6"), "libc");

    let scriptlet_conv = UniversalScriptletConverter::new();
    let hook = scriptlet_conv
        .convert_scriptlet(PackageFormat::Apt, "postinst", "echo post")
        .unwrap();
    assert_eq!(hook.hook_type, SigmaPkgHookType::PostInstall);

    let simulator = UniversalDryRunSimulator::new();
    let result = simulator
        .simulate_install(
            PackageFormat::Apt,
            b"Package: curl\nVersion: 8.2.1\nDepends: libssl-dev, libc6\n",
        )
        .unwrap();
    assert!(result.is_valid);
    assert_eq!(result.package_name, "curl");
    assert_eq!(result.resolved_dependencies.len(), 2);
}
