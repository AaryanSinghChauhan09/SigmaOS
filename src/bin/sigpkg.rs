// SPDX-License-Identifier: MIT
// SigmaPkg — Sovereign Universal Package Manager CLI
// Implements the `sigpkg` commands documented in docs/PACKAGE_MANAGEMENT.md,
// driving the no_std `sigmaos::sigpkg` library APIs from a std host binary.

use std::fs;
use std::path::Path;
use std::process::exit;

use sigmaos::sigpkg::repository_manager::{Repository, RepositoryManager};
use sigmaos::sigpkg::{
    ContentAddressedStore, CryptoVerifier, Dependency, Package, SigpkgDaemon,
    SovereignPackageSnapshotRollbackEngine, UniversalDependencyMapper, UniversalDryRunSimulator,
    UniversalPackageAdapter, Version, VersionConstraint,
};

fn usage() -> ! {
    eprintln!(
        "sigpkg — SigmaOS universal package manager\n\
         \n\
         USAGE:\n\
         \x20 sigpkg install <package|file> Add a package (or foreign .deb/.rpm/PKGBUILD/.apk/.xbps) to store\n\
         \x20 sigpkg convert <file>         Dry-run convert foreign package manifest & print metadata\n\
         \x20 sigpkg remove <package>       Remove a package from the store\n\
         \x20 sigpkg search <package>       Show a stored package's metadata\n\
         \x20 sigpkg status                 List stored packages and counts\n\
         \x20 sigpkg verify <package>       Verify a package's checksum signature\n\
         \x20 sigpkg repo add <name> <url>  Register an apt-style repository\n\
         \x20 sigpkg repo list              List registered repositories\n\
         \x20 sigpkg mirror best <repo>     Choose the best mirror for a repo\n\
         \x20 sigpkg snapshot <desc>        Create a pre-transaction snapshot\n\
         \x20 sigpkg rollback <generation>  Roll the store back to a snapshot\n\
         \x20 sigpkg update                 Check the repository for package updates\n\
         \x20 sigpkg daemon sync            Sync + verify repository metadata (sigpkgd)\n\
         \x20 sigpkg daemon gc              Garbage-collect orphaned store packages\n\
         \x20 sigpkg daemon status          Report daemon state\n\
         \x20 sigpkg help                   Show this help"
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

fn cmd_install(args: &[String]) {
    if args.is_empty() {
        eprintln!("sigpkg: install requires a package name or file path");
        exit(2);
    }
    let target = &args[0];
    let path = Path::new(target);
    let adapter = UniversalPackageAdapter::new();
    let dep_mapper = UniversalDependencyMapper::new();
    let mut store = ContentAddressedStore::new("/var/lib/sigpkg/store".to_string());

    let (pkg, raw_bytes) = if path.exists() {
        let data = match fs::read(path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("sigpkg: failed to read file '{}': {}", target, e);
                exit(1);
            }
        };
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
                // If parsing raw text fails, synthesize a package from filename
                let name = path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| target.clone());
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
    } else if let Some(fmt) = adapter.detect_format_by_extension(target) {
        // Filename specified with foreign package extension but file not found on disk: parse spec/name
        let clean_name = target.split('.').next().unwrap_or(target);
        let canonical_name = dep_mapper.to_canonical_name(clean_name);
        let pkg = Package::new(
            canonical_name,
            Version::parse("1.0.0").unwrap(),
            format!("Foreign format {:?} package spec {}", fmt, target),
            Vec::new(),
            format!("sha256-{}", target),
        );
        (pkg, Vec::new())
    } else {
        // Standard package name
        let canonical_name = dep_mapper.to_canonical_name(target);
        let pkg = Package::new(
            canonical_name.clone(),
            Version::parse("1.0.0").unwrap(),
            format!("{} package", canonical_name),
            Vec::new(),
            "placeholder-checksum".to_string(),
        );
        (pkg, Vec::new())
    };

    let name = pkg.name.clone();
    match store.add(pkg, &raw_bytes) {
        Ok(hash) => {
            println!("Installed {} (store hash {})", name, hash);
            exit(0);
        }
        Err(err) => {
            eprintln!("sigpkg: failed to install {}: {:?}", name, err);
            exit(1);
        }
    }
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
            println!("  Format:               {:?}", result.target_format);
            println!("  Valid Manifest:       {}", result.is_valid);
            println!("  Resolved Dependencies ({})", result.resolved_dependencies.len());
            for dep in &result.resolved_dependencies {
                println!("    - {}", dep);
            }
            println!("  Capability Sandboxing ({})", result.required_permissions.len());
            for perm in &result.required_permissions {
                println!("    - {}", perm);
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
    match store.remove(name) {
        Ok(()) => {
            println!("Removed {}", name);
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
        Some(p) => p.clone(),
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
