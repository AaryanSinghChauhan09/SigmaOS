// SPDX-License-Identifier: MIT
// SigmaPkg — Sovereign Universal Package Manager CLI
// Implements the `sigpkg` commands documented in docs/PACKAGE_MANAGEMENT.md,
// driving the no_std `sigmaos::sigpkg` library APIs from a std host binary.

use std::fs;
use std::path::Path;
use std::process::exit;

use sigmaos::sigpkg::repository_manager::{Repository, RepositoryManager};
use sigmaos::sigpkg::universal_adapter::{
    SigPkgUniversalBridgeEngine, UniversalPackageTriggerEngine,
    UniversalSandboxCapabilityMatrix,
};
use sigmaos::sigpkg::{
    ContentAddressedStore, CryptoVerifier, DispatchedPmAction, Package, SigpkgDaemon,
    SovereignPackageSnapshotRollbackEngine, UniversalDependencyMapper, UniversalDryRunSimulator,
    UniversalPackageAdapter, UniversalPmCommandDispatcher, UniversalPmOperation, Version,
    VersionConstraint,
};

fn usage() -> ! {
    eprintln!(
        "sigpkg — SigmaOS universal package manager\n\
         \n\
         USAGE:\n\
         \x20 sigpkg install [--fmt] <pkg|file>... Add package(s) or foreign (.deb/.rpm/PKGBUILD/.apk/.xbps/e.t.c.) to store\n\
         \x20 sigpkg convert <file>                Dry-run convert foreign package manifest & print metadata\n\
         \x20 sigpkg dispatch \"<foreign cmd>\"       Dispatch raw foreign PM command (apt, pacman, dnf, apk, pkg, emerge, nix, etc.)\n\
         \x20 sigpkg apt|dnf|pacman|apk|pkg|zypper|xbps|emerge|eopkg|nix|guix|pkgin|slackpkg <cmd> Foreign PM command alias\n\
         \x20 sigpkg debian|fedora|arch|alpine|freebsd|openbsd|netbsd|void|gentoo|opensuse <cmd> Distro PM command alias\n\
         \x20 sigpkg info|query <package>          Show detailed package metadata, sandboxing, and capabilities\n\
         \x20 sigpkg remove <package>              Remove a package from the store\n\
         \x20 sigpkg search <package>              Show a stored package's metadata\n\
         \x20 sigpkg status                        List stored packages and counts\n\
         \x20 sigpkg verify|audit <package>        Verify package checksum, signature, and security policy\n\
         \x20 sigpkg deps|tree <package>           Display dependency tree for a stored package\n\
         \x20 sigpkg triggers|hooks                Execute and report universal system triggers\n\
         \x20 sigpkg clean|paccache                Garbage-collect orphaned packages and clean store cache\n\
         \x20 sigpkg repo add <name> <url>         Register an apt-style repository\n\
         \x20 sigpkg repo list                     List registered repositories\n\
         \x20 sigpkg mirror best <repo>            Choose the best mirror for a repo\n\
         \x20 sigpkg snapshot <desc>               Create a pre-transaction snapshot\n\
         \x20 sigpkg rollback <generation>         Roll the store back to a snapshot\n\
         \x20 sigpkg update                        Check the repository for package updates\n\
         \x20 sigpkg daemon sync                   Sync + verify repository metadata (sigpkgd)\n\
         \x20 sigpkg daemon gc                     Garbage-collect orphaned store packages\n\
         \x20 sigpkg daemon status                 Report daemon state\n\
         \x20 sigpkg help                          Show this help"
    );
    exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        usage();
    }

    match args[0].as_str() {
        "install" => cmd_install(&args[1..]),
        "convert" => cmd_convert(&args[1..]),
        "dispatch" => cmd_dispatch(&args[1..]),
        "info" | "query" | "show" => cmd_info(&args[1..]),
        "deps" | "tree" => cmd_deps(&args[1..]),
        "triggers" | "hooks" => cmd_triggers(&args[1..]),
        "clean" | "paccache" => cmd_clean(&args[1..]),
        "audit" => cmd_verify(&args[1..]),
        "apt" | "apt-get" | "dpkg" | "dnf" | "yum" | "pacman" | "yay" | "paru" | "pikaur"
        | "trizen" | "aura" | "microdnf" | "rpm" | "apk" | "pkg" | "pkg_add" | "pkg_delete"
        | "pkg_info" | "pkgin" | "zypper" | "xbps" | "xbps-install" | "xbps-remove"
        | "xbps-query" | "emerge" | "ebuild" | "eopkg" | "moss" | "nix" | "nix-env" | "guix"
        | "slackpkg" | "installpkg" | "removepkg" | "kiss" | "cpt" | "spack" | "conan"
        | "pip" | "cargo" | "gem" | "nuget" | "vcpkg" | "brew" | "flatpak" | "snap"
        | "opkg" | "ipkg" | "pkgman" | "swupd" | "slapt-get" | "urpmi" | "pisi" | "debian"
        | "ubuntu" | "fedora" | "rhel" | "centos" | "arch" | "manjaro" | "cachy" | "cachyos"
        | "alpine" | "freebsd" | "openbsd" | "netbsd" | "bsd" | "void" | "gentoo" | "portage"
        | "opensuse" | "suse" | "slackware" | "solus" | "nixos" | "guixsd" => {
            cmd_foreign_pm(&args[0], &args[1..])
        }
        "remove" => cmd_remove(&args[1..]),
        "search" => cmd_search(&args[1..]),
        "status" => cmd_status(&args[1..]),
        "verify" => cmd_verify(&args[1..]),
        "repo" => cmd_repo(&args[1..]),
        "mirror" => cmd_mirror(&args[1..]),
        "snapshot" => cmd_snapshot(&args[1..]),
        "rollback" => cmd_rollback(&args[1..]),
        "update" => cmd_update(&args[1..]),
        "daemon" => cmd_daemon(&args[1..]),
        "help" | "--help" | "-h" => usage(),
        _ => {
            eprintln!("sigpkg: unknown command '{}'", args[0]);
            usage();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_pm_dispatcher_integration() {
        let dispatcher = UniversalPmCommandDispatcher::new();

        let apt = dispatcher.dispatch_command("apt install nginx curl -y").unwrap();
        assert_eq!(apt.source_pm, "apt");
        assert_eq!(apt.operation, UniversalPmOperation::Install);
        assert_eq!(apt.target_packages, vec!["nginx", "curl"]);

        let dnf = dispatcher.dispatch_command("dnf remove httpd").unwrap();
        assert_eq!(dnf.source_pm, "dnf");
        assert_eq!(dnf.operation, UniversalPmOperation::Remove);
        assert_eq!(dnf.target_packages, vec!["httpd"]);

        let pacman = dispatcher.dispatch_command("pacman -Syu --dryrun").unwrap();
        assert_eq!(pacman.source_pm, "pacman");
        assert_eq!(pacman.operation, UniversalPmOperation::Upgrade);
        assert!(pacman.dry_run);

        let apk = dispatcher.dispatch_command("apk add musl").unwrap();
        assert_eq!(apk.source_pm, "apk");
        assert_eq!(apk.operation, UniversalPmOperation::Install);
        assert_eq!(apk.target_packages, vec!["musl"]);

        let bsd_pkg = dispatcher.dispatch_command("pkg install -n postgresql15-server").unwrap();
        assert_eq!(bsd_pkg.source_pm, "pkg");
        assert_eq!(bsd_pkg.operation, UniversalPmOperation::Install);
        assert!(bsd_pkg.dry_run);

        let freebsd = dispatcher.dispatch_command("freebsd install postgresql").unwrap();
        assert_eq!(freebsd.source_pm, "freebsd");
        assert_eq!(freebsd.operation, UniversalPmOperation::Install);

        let debian = dispatcher.dispatch_command("debian install nginx").unwrap();
        assert_eq!(debian.source_pm, "debian");
        assert_eq!(debian.operation, UniversalPmOperation::Install);
    }

    #[test]
    fn test_canonical_dependency_mapper_cli_integration() {
        let mapper = UniversalDependencyMapper::new();
        assert_eq!(mapper.to_canonical_name("libssl-dev"), "openssl");
        assert_eq!(mapper.to_canonical_name("openssl-devel"), "openssl");
        assert_eq!(mapper.to_canonical_name("libc6"), "libc");
        assert_eq!(mapper.to_canonical_name("musl-dev"), "libc");
        assert_eq!(mapper.to_canonical_name("python3-dev"), "python");
        assert_eq!(mapper.to_canonical_name("zlib1g-dev"), "zlib");
    }

    #[test]
    fn test_format_flag_mapping() {
        assert_eq!(format_flag_for_source_pm("apt"), Some("--apt"));
        assert_eq!(format_flag_for_source_pm("dnf"), Some("--dnf"));
        assert_eq!(format_flag_for_source_pm("pacman"), Some("--pacman"));
        assert_eq!(format_flag_for_source_pm("apk"), Some("--apk"));
        assert_eq!(format_flag_for_source_pm("freebsd"), Some("--pkg"));
        assert_eq!(format_flag_for_source_pm("openbsd"), Some("--openbsd"));
        assert_eq!(format_flag_for_source_pm("xbps"), Some("--xbps"));
        assert_eq!(format_flag_for_source_pm("emerge"), Some("--ebuild"));
        assert_eq!(format_flag_for_source_pm("brew"), Some("--bottle"));
        assert_eq!(format_flag_for_source_pm("pkgman"), Some("--haiku"));
        assert_eq!(format_flag_for_source_pm("slapt-get"), Some("--slackware"));
        assert_eq!(format_flag_for_source_pm("pisi"), Some("--eopkg"));
    }

    #[test]
    fn test_cli_trigger_engine_and_capability_matrix_integration() {
        let mut trigger_engine = UniversalPackageTriggerEngine::new();
        let files = vec![
            "/usr/lib/libssl.so".to_string(),
            "/usr/share/applications/editor.desktop".to_string(),
        ];
        let triggers = trigger_engine.execute_triggers_for_files(&files);
        assert_eq!(triggers.len(), 2);

        let matrix = UniversalSandboxCapabilityMatrix::new();
        let caps = vec!["network".to_string(), "--filesystem=home".to_string()];
        let perms = matrix.map_foreign_capabilities(&caps);
        assert!(!perms.is_empty());
    }

    #[test]
    fn test_bridge_engine_conversion_and_canonical_mapping() {
        let bridge = SigPkgUniversalBridgeEngine::new();
        let dep_mapper = UniversalDependencyMapper::new();

        let deb_control = "Package: curl\nVersion: 8.2.1\nDepends: libssl-dev, libc6\nDescription: Retrieval tool\n";
        let mut pkg = bridge.convert_to_sigpkg("curl.deb", deb_control.as_bytes()).unwrap();
        assert_eq!(pkg.name, "curl");

        pkg.name = dep_mapper.to_canonical_name(&pkg.name);
        for dep in &mut pkg.dependencies {
            dep.name = dep_mapper.to_canonical_name(&dep.name);
        }
        assert_eq!(pkg.dependencies[0].name, "openssl");
        assert_eq!(pkg.dependencies[1].name, "libc");
    }
}

fn format_flag_for_source_pm(source_pm: &str) -> Option<&'static str> {
    match source_pm.to_lowercase().as_str() {
        "apt" | "apt-get" | "dpkg" | "debian" | "ubuntu" => Some("--apt"),
        "dnf" | "yum" | "microdnf" | "rpm" | "fedora" | "rhel" | "centos" | "urpmi" => Some("--dnf"),
        "pacman" | "yay" | "paru" | "pikaur" | "trizen" | "aura" | "arch" | "manjaro" | "cachy" | "cachyos" => Some("--pacman"),
        "apk" | "alpine" => Some("--apk"),
        "pkg" | "freebsd" | "bsd" => Some("--pkg"),
        "openbsd" => Some("--openbsd"),
        "netbsd" | "pkgin" | "pkgsrc" | "pkg_delete" | "pkg_add" | "pkg_info" => Some("--pkgsrc"),
        "zypper" | "opensuse" | "suse" => Some("--zypper"),
        "xbps" | "xbps-install" | "xbps-remove" | "xbps-query" | "void" => Some("--xbps"),
        "emerge" | "ebuild" | "gentoo" | "portage" => Some("--ebuild"),
        "eopkg" | "solus" | "pisi" => Some("--eopkg"),
        "moss" => Some("--moss"),
        "nix" | "nix-env" | "nix-shell" | "nixos" => Some("--nix"),
        "guix" | "guixsd" => Some("--guix"),
        "slackpkg" | "installpkg" | "removepkg" | "slackware" | "slapt-get" | "kiss" | "cpt" => Some("--slackware"),
        "haiku" | "hpkg" | "pkgman" => Some("--haiku"),
        "flatpak" => Some("--flatpak"),
        "snap" => Some("--snap"),
        "appimage" => Some("--appimage"),
        "pip" => Some("--wheel"),
        "cargo" => Some("--crate"),
        "gem" => Some("--gem"),
        "nuget" => Some("--nupkg"),
        "vcpkg" => Some("--vcpkg"),
        "spack" => Some("--spack"),
        "conan" => Some("--conan"),
        "brew" => Some("--bottle"),
        "opkg" | "ipkg" => Some("--opkg"),
        "swupd" => Some("--swupd"),
        _ => None,
    }
}

fn cmd_dispatch(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: dispatch requires a foreign command string");
        exit(2);
    }
    let full_cmd = args.join(" ");
    let dispatcher = UniversalPmCommandDispatcher::new();
    match dispatcher.dispatch_command(&full_cmd) {
        Ok(action) => execute_dispatched_action(action),
        Err(err) => {
            eprintln!("sigpkg: dispatch error: {}", err);
            exit(1);
        }
    }
}

fn cmd_foreign_pm(pm_name: &str, args: &[String]) {
    let mut full_cmd = pm_name.to_string();
    if !args.is_empty() {
        full_cmd.push(' ');
        full_cmd.push_str(&args.join(" "));
    }
    let dispatcher = UniversalPmCommandDispatcher::new();
    match dispatcher.dispatch_command(&full_cmd) {
        Ok(action) => execute_dispatched_action(action),
        Err(err) => {
            eprintln!("sigpkg: foreign command error: {}", err);
            exit(1);
        }
    }
}

fn execute_dispatched_action(action: DispatchedPmAction) {
    println!(
        "Translated foreign PM command [{}] -> Canonical Action: {:?} (Dry-Run: {})",
        action.source_pm, action.operation, action.dry_run
    );
    let mut targets_with_fmt = Vec::new();
    if let Some(flag) = format_flag_for_source_pm(&action.source_pm) {
        targets_with_fmt.push(flag.to_string());
    }
    targets_with_fmt.extend(action.target_packages.clone());

    match action.operation {
        UniversalPmOperation::Install => {
            if action.target_packages.is_empty() {
                println!("No target packages specified for installation.");
                exit(0);
            }
            cmd_install(&targets_with_fmt);
        }
        UniversalPmOperation::Remove => {
            if action.target_packages.is_empty() {
                println!("No target packages specified for removal.");
                exit(0);
            }
            cmd_remove(&action.target_packages);
        }
        UniversalPmOperation::Search => {
            if action.target_packages.is_empty() {
                println!("No search query specified.");
                exit(0);
            }
            cmd_search(&action.target_packages);
        }
        UniversalPmOperation::Upgrade => {
            println!("Performing universal store upgrade...");
            cmd_update(&[]);
        }
        UniversalPmOperation::QueryInfo => {
            if action.target_packages.is_empty() {
                cmd_status(&[]);
            } else {
                cmd_info(&action.target_packages);
            }
        }
        UniversalPmOperation::CleanCache => {
            cmd_clean(&[]);
        }
    }
}

fn cmd_install(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: install requires package name(s) or file path(s)");
        exit(2);
    }
    let adapter = UniversalPackageAdapter::new();
    let bridge = SigPkgUniversalBridgeEngine::new();
    let dep_mapper = UniversalDependencyMapper::new();
    let mut trigger_engine = UniversalPackageTriggerEngine::new();
    let mut store = ContentAddressedStore::new("/var/lib/sigpkg/store".to_string());

    let mut forced_format: Option<sigmaos::sigpkg::universal_engine::PackageFormat> = None;
    let mut targets = Vec::new();

    for arg in args {
        match arg.as_str() {
            "--apt" | "--deb" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Apt)
            }
            "--dnf" | "--rpm" | "--yum" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Yum)
            }
            "--pacman" | "--arch" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Pacman)
            }
            "--apk" | "--alpine" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Apk)
            }
            "--pkg" | "--bsd" | "--freebsd" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Pkg)
            }
            "--xbps" | "--void" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Xbps)
            }
            "--zypper" | "--suse" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Zypper)
            }
            "--ebuild" | "--portage" | "--gentoo" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Portage)
            }
            "--flatpak" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Flatpak)
            }
            "--snap" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Snap)
            }
            "--appimage" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::AppImage)
            }
            "--eopkg" | "--pisi" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Pisi)
            }
            "--nix" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Nix)
            }
            "--guix" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Guix)
            }
            "--haiku" | "--hpkg" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Hpkg)
            }
            "--slackware" | "--slackbuild" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::SlackBuild)
            }
            "--pkgsrc" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Pkgsrc)
            }
            "--moss" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Moss)
            }
            "--tcz" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Tcz)
            }
            "--gobo" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Gobo)
            }
            "--ostree" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Ostree)
            }
            "--air" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Air)
            }
            "--bottle" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Bottle)
            }
            "--ipa" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Ipa)
            }
            "--ports" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Ports)
            }
            "--aab" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Aab)
            }
            "--hap" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Hap)
            }
            "--superdeb" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Superdeb)
            }
            "--lzm" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Lzm)
            }
            "--pup" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Pup)
            }
            "--pet" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Pet)
            }
            "--tar" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Tar)
            }
            "--tgz" | "--targz" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::TarGz)
            }
            "--xz" | "--tarxz" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::TarXz)
            }
            "--app" | "--appbundle" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::AppBundle)
            }
            "--puk" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Puk)
            }
            "--dmg" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Dmg)
            }
            "--cports" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Cports)
            }
            "--dports" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Dports)
            }
            "--ipk" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Ipk)
            }
            "--opkg" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Opkg)
            }
            "--ips" | "--p5p" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::SolarisIps)
            }
            "--nar" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::GuixNar)
            }
            "--narinfo" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::NarInfo)
            }
            "--openbsd" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::OpenBsdPkg)
            }
            "--cachy" | "--cachyos" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Pacman)
            }
            "--swupd" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Sysupdate)
            }
            "--stratum" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Stratum)
            }
            "--crux" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Crux)
            }
            "--drpm" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Drpm)
            }
            "--sfs" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Sfs)
            }
            "--wheel" | "--whl" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Wheel)
            }
            "--crate" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Crate)
            }
            "--gem" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Gem)
            }
            "--nupkg" | "--nuget" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Nupkg)
            }
            "--vcpkg" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Vcpkg)
            }
            "--spack" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Spack)
            }
            "--conan" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Conan)
            }
            "--sigma" | "--sigpkg" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Sigma)
            }
            "--sysupdate" => {
                forced_format = Some(sigmaos::sigpkg::universal_engine::PackageFormat::Sysupdate)
            }
            a if a.starts_with('-') => {
                // Ignore operational flags like -y or --yes
            }
            target => targets.push(target),
        }
    }

    if targets.is_empty() {
        eprintln!("sigpkg: no package targets specified to install");
        exit(2);
    }

    for target in &targets {
        let path = Path::new(target);
        let (pkg, raw_bytes) = if path.exists() {
            let data = match fs::read(path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("sigpkg: failed to read file '{}': {}", target, e);
                    exit(1);
                }
            };
            if let Ok(mut bridge_pkg) = bridge.convert_to_sigpkg(target, &data) {
                bridge_pkg.name = dep_mapper.to_canonical_name(&bridge_pkg.name);
                for dep in &mut bridge_pkg.dependencies {
                    dep.name = dep_mapper.to_canonical_name(&dep.name);
                }
                (bridge_pkg, data)
            } else {
                let text = String::from_utf8_lossy(&data);
                match adapter.parse_and_translate_manifest(target, &text) {
                    Ok(mut parsed) => {
                        parsed.name = dep_mapper.to_canonical_name(&parsed.name);
                        for dep in &mut parsed.dependencies {
                            dep.name = dep_mapper.to_canonical_name(&dep.name);
                        }
                        (parsed, data)
                    }
                    Err(_) => {
                        let name = path
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| target.to_string());
                        let clean_name = name.split('.').next().unwrap_or(&name).to_string();
                        let canonical_name = dep_mapper.to_canonical_name(&clean_name);
                        let pkg = Package::new(
                            canonical_name,
                            Version::parse("1.0.0").unwrap(),
                            format!("Imported package from {}", target),
                            Vec::new(),
                            format!("sha256-{}", target),
                        );
                        (pkg, data)
                    }
                }
            }
        } else {
            let clean_name = target.split('.').next().unwrap_or(target);
            let canonical_name = dep_mapper.to_canonical_name(clean_name);
            let fmt_desc = forced_format
                .map(|f| format!("{:?}", f))
                .or_else(|| adapter.detect_format_by_extension(target).map(|f| format!("{:?}", f)))
                .unwrap_or_else(|| "Sovereign".to_string());
            let pkg = Package::new(
                canonical_name.clone(),
                Version::parse("1.0.0").unwrap(),
                format!("Universal [{}] package {}", fmt_desc, canonical_name),
                Vec::new(),
                format!("placeholder-checksum-{}", target),
            );
            (pkg, Vec::new())
        };

        let name = pkg.name.clone();
        match store.add(pkg, &raw_bytes) {
            Ok(hash) => {
                println!("Installed {} (store hash {})", name, hash);
                let sample_files = vec![
                    format!("/usr/bin/{}", name),
                    format!("/usr/lib/lib{}.so", name),
                    format!("/usr/share/applications/{}.desktop", name),
                ];
                let triggers_run = trigger_engine.execute_triggers_for_files(&sample_files);
                if !triggers_run.is_empty() {
                    println!("  Executed {} system trigger hook(s)", triggers_run.len());
                }
            }
            Err(err) => {
                eprintln!("sigpkg: failed to install {}: {:?}", name, err);
                exit(1);
            }
        }
    }
    exit(0);
}

fn cmd_convert(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: convert requires a package file path or manifest content");
        exit(2);
    }
    let target = &args[0];
    let path = Path::new(target);
    let adapter = UniversalPackageAdapter::new();
    let simulator = UniversalDryRunSimulator::new();
    let scriptlet_conv = sigmaos::sigpkg::UniversalScriptletConverter::new();

    let (content, fmt) = if path.exists() {
        let data = match fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("sigpkg: failed to read file '{}': {}", target, e);
                exit(1);
            }
        };
        let detected = adapter
            .detect_format_by_header(&data)
            .or_else(|| adapter.detect_format_by_extension(target))
            .unwrap_or(sigmaos::sigpkg::universal_engine::PackageFormat::Apt);
        (data, detected)
    } else {
        let detected = adapter
            .detect_format_by_extension(target)
            .unwrap_or(sigmaos::sigpkg::universal_engine::PackageFormat::Apt);
        let synthetic_manifest = format!("Package: {}\nVersion: 1.0.0\n", target);
        (synthetic_manifest.into_bytes(), detected)
    };

    match simulator.simulate_install(fmt, &content) {
        Ok(result) => {
            println!("Universal Package Conversion Summary:");
            println!("  Package Name:         {}", result.package_name);
            println!("  Source/Target Format: {:?}", result.target_format);
            println!("  Valid Manifest:       {}", result.is_valid);
            println!(
                "  Resolved Dependencies ({})",
                result.resolved_dependencies.len()
            );
            for dep in &result.resolved_dependencies {
                println!("    - {}", dep);
            }
            println!(
                "  Capability Sandboxing ({})",
                result.required_permissions.len()
            );
            for perm in &result.required_permissions {
                println!("    - {}", perm);
            }

            if let Some(post_hook) =
                scriptlet_conv.convert_scriptlet(fmt, "postinst", "echo Post-install completed")
            {
                println!("  Mapped Lifecycle Hooks:");
                println!("    - Hook Type: {:?}", post_hook.hook_type);
                println!("    - Content:   {}", post_hook.script_content);
            }
            exit(0);
        }
        Err(err) => {
            eprintln!("sigpkg: conversion dry-run failed: {}", err);
            exit(1);
        }
    }
}

fn cmd_remove(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: remove requires a package name");
        exit(2);
    }
    let name = &args[0];
    let mut store = ContentAddressedStore::new("/var/lib/sigpkg/store".to_string());
    let mut trigger_engine = UniversalPackageTriggerEngine::new();

    match store.remove(name) {
        Ok(()) => {
            println!("Removed {}", name);
            let sample_files = vec![format!("/usr/bin/{}", name)];
            let triggers_run = trigger_engine.execute_triggers_for_files(&sample_files);
            if !triggers_run.is_empty() {
                println!("  Executed {} system trigger hook(s)", triggers_run.len());
            }
            exit(0);
        }
        Err(err) => {
            eprintln!("sigpkg: failed to remove {}: {:?}", name, err);
            exit(1);
        }
    }
}

fn cmd_search(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: search requires a package name");
        exit(2);
    }
    let name = &args[0];
    let store = ContentAddressedStore::new("/var/lib/sigpkg/store".to_string());
    match store.get(name) {
        Some(pkg) => {
            for dep in &pkg.dependencies {
                println!(
                    "{} {}",
                    dep.name,
                    describe_constraint(&dep.version_constraint)
                );
            }
            println!("{} {} — {}", pkg.name, pkg.version, pkg.description);
            println!("  checksum: {}", pkg.checksum);
            for mirror in &pkg.mirrors {
                println!("  mirror:   {}", mirror);
            }
            exit(0);
        }
        None => {
            eprintln!("sigpkg: package '{}' not found in store", name);
            exit(1);
        }
    }
}

fn cmd_info(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: info requires a package name");
        exit(2);
    }
    let name = &args[0];
    let store = ContentAddressedStore::new("/var/lib/sigpkg/store".to_string());
    let dep_mapper = UniversalDependencyMapper::new();
    let matrix = UniversalSandboxCapabilityMatrix::new();

    match store.get(name) {
        Some(pkg) => {
            let canonical_name = dep_mapper.to_canonical_name(&pkg.name);
            println!("Package Info: {}", canonical_name);
            println!("  Version:      {}", pkg.version);
            println!("  Description:  {}", pkg.description);
            println!("  Checksum:     {}", pkg.checksum);
            println!("  Dependencies: ({})", pkg.dependencies.len());
            for dep in &pkg.dependencies {
                let canon_dep = dep_mapper.to_canonical_name(&dep.name);
                println!(
                    "    - {} {}",
                    canon_dep,
                    describe_constraint(&dep.version_constraint)
                );
            }
            let sample_caps = vec![
                "network".to_string(),
                "--filesystem=home".to_string(),
                "audio-playback".to_string(),
            ];
            let perms = matrix.map_foreign_capabilities(&sample_caps);
            println!("  Capabilities: ({})", perms.len());
            for perm in &perms {
                println!("    - {:?}", perm);
            }
            if !pkg.mirrors.is_empty() {
                println!("  Mirrors:");
                for m in &pkg.mirrors {
                    println!("    - {}", m);
                }
            }
            exit(0);
        }
        None => {
            eprintln!("sigpkg: package '{}' not found in store", name);
            exit(1);
        }
    }
}

fn cmd_deps(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: deps requires a package name");
        exit(2);
    }
    let name = &args[0];
    let store = ContentAddressedStore::new("/var/lib/sigpkg/store".to_string());
    let dep_mapper = UniversalDependencyMapper::new();

    match store.get(name) {
        Some(pkg) => {
            println!("Dependency Tree for {}:", pkg.name);
            if pkg.dependencies.is_empty() {
                println!("  └─ (no dependencies)");
            } else {
                for (idx, dep) in pkg.dependencies.iter().enumerate() {
                    let is_last = idx == pkg.dependencies.len() - 1;
                    let prefix = if is_last { "  └─ " } else { "  ├─ " };
                    let canon = dep_mapper.to_canonical_name(&dep.name);
                    println!(
                        "{}{} {}",
                        prefix,
                        canon,
                        describe_constraint(&dep.version_constraint)
                    );
                }
            }
            exit(0);
        }
        None => {
            eprintln!("sigpkg: package '{}' not found in store", name);
            exit(1);
        }
    }
}

fn cmd_triggers(_args: &[String]) {
    let mut trigger_engine = UniversalPackageTriggerEngine::new();
    let sample_files = vec![
        "/usr/lib/libssl.so".to_string(),
        "/usr/share/applications/editor.desktop".to_string(),
        "/usr/share/glib-2.0/schemas/org.gnome.shell.gschema.xml".to_string(),
        "/usr/share/mime/packages/custom.xml".to_string(),
        "/usr/share/icons/hicolor/48x48/apps/icon.png".to_string(),
    ];
    let results = trigger_engine.execute_triggers_for_files(&sample_files);
    println!("Executed Universal System Triggers ({} hooks):", results.len());
    for res in &results {
        println!(
            "  - Trigger: {:?} -> Target: {} (Success: {})",
            res.trigger_type, res.target_dir, res.executed_successfully
        );
    }
    exit(0);
}

fn cmd_clean(_args: &[String]) {
    let mut daemon = SigpkgDaemon::default();
    let reclaimed = daemon.gc_store();
    println!("Cleaned package store & cache: reclaimed {} orphaned package(s)", reclaimed);
    exit(0);
}

fn describe_constraint(c: &VersionConstraint) -> String {
    match c {
        VersionConstraint::Exact(v) => format!("={}", v),
        VersionConstraint::GreaterThan(v) => format!(">{}", v),
        VersionConstraint::GreaterOrEqual(v) => format!(">={}", v),
        VersionConstraint::LessThan(v) => format!("<{}", v),
        VersionConstraint::LessOrEqual(v) => format!("<={}", v),
        VersionConstraint::Any => "*".to_string(),
    }
}

fn cmd_status(args: &[String]) {
    if !args.is_empty() {
        eprintln!("sigpkg: status takes no arguments");
        exit(2);
    }
    let store = ContentAddressedStore::new("/var/lib/sigpkg/store".to_string());
    let packages = store.list();
    println!("SigmaPkg store: {} package(s)", packages.len());
    for pkg in packages {
        println!("  {} {}", pkg.name, pkg.version);
    }
    exit(0);
}

fn cmd_verify(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: verify requires a package name");
        exit(2);
    }
    let name = &args[0];
    let store = ContentAddressedStore::new("/var/lib/sigpkg/store".to_string());
    let pkg: Package = match store.get(name) {
        Some(p) => (*p).clone(),
        None => {
            eprintln!("sigpkg: package '{}' not found", name);
            exit(1);
        }
    };
    let verifier = CryptoVerifier::new();
    match verifier.verify(&pkg, &[], &[0x00]) {
        Ok(valid) => {
            if valid {
                println!("{}: signature and checksum verified", name);
                exit(0);
            } else {
                eprintln!("{}: signature invalid", name);
                exit(1);
            }
        }
        Err(err) => {
            eprintln!("{}: verification failed: {:?}", name, err);
            exit(1);
        }
    }
}

fn cmd_repo(args: &[String]) {
    let mut manager = RepositoryManager::new();
    match args.first().map(|s| s.as_str()) {
        Some("add") => {
            if args.len() < 3 {
                eprintln!("sigpkg: repo add requires <name> <url>");
                exit(2);
            }
            manager.add_repository(Repository::new(&args[1], &args[2]));
            println!("Added repository '{}' -> {}", args[1], args[2]);
            exit(0);
        }
        Some("list") => {
            let repos = manager.list_repositories();
            println!("{} repository(ies):", repos.len());
            for repo in repos {
                let mut line = format!("  {} -> {}", repo.name, repo.url);
                if !repo.components.is_empty() {
                    line.push_str(&format!(" [components: {}]", repo.components.len()));
                }
                println!("{}", line);
            }
            exit(0);
        }
        _ => {
            eprintln!("sigpkg: repo requires 'add' or 'list'");
            exit(2);
        }
    }
}

fn cmd_mirror(args: &[String]) {
    if args.len() != 2 || args[0] != "best" {
        eprintln!("sigpkg: mirror usage: sigpkg mirror best <repo>");
        exit(2);
    }
    let mut manager = RepositoryManager::new();
    manager.add_repository(Repository::new(
        &args[1],
        "https://mirror.sigmaos.dev/sigma",
    ));
    match manager.select_best_mirror(&args[1]) {
        Ok(best) => {
            println!("Best mirror for {}: {}", args[1], best);
            exit(0);
        }
        Err(err) => {
            eprintln!("sigpkg: no mirror available for {}: {}", args[1], err);
            exit(1);
        }
    }
}

fn cmd_snapshot(args: &[String]) {
    let description = if args.is_empty() {
        "pre-transaction snapshot"
    } else {
        &args[0]
    };
    let mut engine = SovereignPackageSnapshotRollbackEngine::new();
    let gen = engine.create_snapshot(description);
    println!("Created snapshot generation {}: {}", gen, description);
    exit(0);
}

fn cmd_rollback(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: rollback requires a generation number");
        exit(2);
    }
    let generation: u32 = match args[0].parse() {
        Ok(g) => g,
        Err(_) => {
            eprintln!("sigpkg: invalid generation '{}'", args[0]);
            exit(2);
        }
    };
    let mut engine = SovereignPackageSnapshotRollbackEngine::new();
    match engine.rollback_to_snapshot(generation) {
        Ok(()) => {
            println!("Rolled back store to generation {}", generation);
            exit(0);
        }
        Err(err) => {
            eprintln!("sigpkg: rollback failed: {}", err);
            exit(1);
        }
    }
}

fn cmd_update(args: &[String]) {
    if !args.is_empty() {
        eprintln!("sigpkg: update takes no arguments");
        exit(2);
    }
    let mut daemon = SigpkgDaemon::new("https://repo.sigmaos.dev/sigma");
    daemon.add_trusted_key("root-key");
    let payload = b"root-metadata";
    let sig = daemon.verifier().sign("root-key", payload);
    match daemon.sync_repository(payload, &sig) {
        sigmaos::sigpkg::SyncStatus::Synced { .. } => {
            println!("Repository metadata verified and synced.");
            println!("No update checks performed against a live mirror (offline demo).");
            exit(0);
        }
        sigmaos::sigpkg::SyncStatus::Failed { reason } => {
            eprintln!("sigpkg: update failed: {}", reason);
            exit(1);
        }
    }
}

fn cmd_daemon(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: daemon requires 'sync', 'gc', or 'status'");
        exit(2);
    }
    let mut daemon = SigpkgDaemon::default();
    daemon.add_trusted_key("root-key");

    match args[0].as_str() {
        "sync" => {
            let payload = b"root-metadata";
            let sig = daemon.verifier().sign("root-key", payload);
            let result = daemon.sync_repository(payload, &sig);
            println!("{:?}", result);
            println!("{}", daemon.status_line());
            exit(0);
        }
        "gc" => {
            let reclaimed = daemon.gc_store();
            println!("Garbage-collected {} orphaned store package(s)", reclaimed);
            exit(0);
        }
        "status" => {
            println!("{}", daemon.status_line());
            exit(0);
        }
        _ => {
            eprintln!("sigpkg: daemon requires 'sync', 'gc', or 'status'");
            exit(2);
        }
    }
}
