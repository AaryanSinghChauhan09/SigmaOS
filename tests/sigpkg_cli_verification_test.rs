// SigmaOS Universal Package Manager CLI & Adapter Integration Tests
extern crate alloc;

pub mod klib {
    pub mod collections {
        pub use alloc::collections::BTreeMap as HashMap;
    }
}

#[path = "../src/package/universal.rs"]
pub mod universal;

pub mod package {
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
    pub use crate::universal_adapter::*;

    pub use crate::universal_oop_system::{Dependency, Package, Version, VersionConstraint};
}

#[path = "../src/sigpkg/universal_adapter.rs"]
pub mod universal_adapter;

use sigpkg::*;

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

    // Distro Name Aliases
    let debian_act = dispatcher.dispatch_command("debian install nginx").unwrap();
    assert_eq!(debian_act.source_pm, "debian");
    assert_eq!(debian_act.operation, UniversalPmOperation::Install);
    assert_eq!(debian_act.target_packages, vec!["nginx"]);

    let fedora_act = dispatcher.dispatch_command("fedora remove httpd").unwrap();
    assert_eq!(fedora_act.source_pm, "fedora");
    assert_eq!(fedora_act.operation, UniversalPmOperation::Remove);

    let arch_act = dispatcher.dispatch_command("arch search ripgrep").unwrap();
    assert_eq!(arch_act.source_pm, "arch");
    assert_eq!(arch_act.operation, UniversalPmOperation::Search);

    let alpine_act = dispatcher.dispatch_command("alpine add musl-dev").unwrap();
    assert_eq!(alpine_act.source_pm, "alpine");
    assert_eq!(alpine_act.operation, UniversalPmOperation::Install);

    let freebsd_act = dispatcher.dispatch_command("freebsd install postgresql15").unwrap();
    assert_eq!(freebsd_act.source_pm, "freebsd");
    assert_eq!(freebsd_act.operation, UniversalPmOperation::Install);

    let void_act = dispatcher.dispatch_command("void install bash").unwrap();
    assert_eq!(void_act.source_pm, "void");
    assert_eq!(void_act.operation, UniversalPmOperation::Install);

    let gentoo_act = dispatcher.dispatch_command("gentoo install portage").unwrap();
    assert_eq!(gentoo_act.source_pm, "gentoo");
    assert_eq!(gentoo_act.operation, UniversalPmOperation::Install);

    let opensuse_act = dispatcher.dispatch_command("opensuse in gcc").unwrap();
    assert_eq!(opensuse_act.source_pm, "opensuse");
    assert_eq!(opensuse_act.operation, UniversalPmOperation::Install);

    let nixos_act = dispatcher.dispatch_command("nixos install firefox").unwrap();
    assert_eq!(nixos_act.source_pm, "nixos");
    assert_eq!(nixos_act.operation, UniversalPmOperation::Install);

    // Additional Distros & Modern Package Managers
    let urpmi_act = dispatcher.dispatch_command("urpmi install kernel").unwrap();
    assert_eq!(urpmi_act.source_pm, "urpmi");
    assert_eq!(urpmi_act.operation, UniversalPmOperation::Install);

    let pisi_act = dispatcher.dispatch_command("pisi it python3").unwrap();
    assert_eq!(pisi_act.source_pm, "pisi");
    assert_eq!(pisi_act.operation, UniversalPmOperation::Install);

    let spack_act = dispatcher.dispatch_command("spack install openmpi").unwrap();
    assert_eq!(spack_act.source_pm, "spack");
    assert_eq!(spack_act.operation, UniversalPmOperation::Install);

    let conan_act = dispatcher.dispatch_command("conan install boost").unwrap();
    assert_eq!(conan_act.source_pm, "conan");
    assert_eq!(conan_act.operation, UniversalPmOperation::Install);

    let kiss_act = dispatcher.dispatch_command("kiss build busybox").unwrap();
    assert_eq!(kiss_act.source_pm, "kiss");
    assert_eq!(kiss_act.operation, UniversalPmOperation::Install);
}

#[test]
fn test_universal_scriptlet_and_dependency_mapper() {
    let dep_mapper = UniversalDependencyMapper::new();
    assert_eq!(dep_mapper.to_canonical_name("libssl-dev"), "openssl");
    assert_eq!(dep_mapper.to_canonical_name("openssl-devel"), "openssl");
    assert_eq!(dep_mapper.to_canonical_name("libc6"), "libc");
    assert_eq!(dep_mapper.to_canonical_name("musl-dev"), "libc");
    assert_eq!(dep_mapper.to_canonical_name("python3-dev"), "python");
    assert_eq!(dep_mapper.to_canonical_name("zlib1g-dev"), "zlib");
    assert_eq!(dep_mapper.to_canonical_name("sys-apps/systemd"), "systemd");
    assert_eq!(dep_mapper.to_canonical_name("sys-apps/openrc"), "openrc");
    assert_eq!(dep_mapper.to_canonical_name("media-video/pipewire"), "pipewire");
    assert_eq!(dep_mapper.to_canonical_name("dev-libs/wayland"), "wayland");

    let scriptlet_conv = UniversalScriptletConverter::new();
    let hook = scriptlet_conv
        .convert_scriptlet(universal_engine::PackageFormat::Apt, "postinst", "echo post")
        .unwrap();
    assert_eq!(hook.hook_type, SigmaPkgHookType::PostInstall);

    let simulator = UniversalDryRunSimulator::new();
    let result = simulator
        .simulate_install(
            universal_engine::PackageFormat::Apt,
            b"Package: curl\nVersion: 8.2.1\nDepends: libssl-dev, libc6\n",
        )
        .unwrap();
    assert!(result.is_valid);
    assert_eq!(result.package_name, "curl");
    assert_eq!(result.resolved_dependencies.len(), 2);
}
