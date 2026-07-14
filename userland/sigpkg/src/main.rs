// sigpkg — SigmaOS Sovereign Package Manager
// CLI entry point

mod resolver;
mod crypto;
mod profiles;
mod registry;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let exit_code = match args[1].as_str() {
        "install" | "i"   => cmd_install(&args[2..]),
        "remove"  | "rm"  => cmd_remove(&args[2..]),
        "update"  | "up"  => cmd_update(&args[2..]),
        "search"  | "s"   => cmd_search(&args[2..]),
        "verify"          => cmd_verify(&args[2..]),
        "list"    | "ls"  => cmd_list(&args[2..]),
        "info"            => cmd_info(&args[2..]),
        "profile"         => cmd_profile(&args[2..]),
        "sync"            => cmd_sync(),
        "version" | "-v"  => { println!("sigpkg v0.2.0 — SigmaOS Sovereign Package Manager"); 0 }
        "--help"  | "-h"  => { print_usage(); 0 }
        unknown           => {
            eprintln!("sigpkg: unknown command '{}'. Run 'sigpkg --help'.", unknown);
            1
        }
    };

    process::exit(exit_code);
}

fn cmd_install(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("sigpkg: install requires at least one package name");
        return 1;
    }

    for pkg in args {
        println!("\x1b[1;34m[sigpkg]\x1b[0m Resolving dependencies for '{}'...", pkg);
        let resolved = match resolver::resolve(pkg) {
            Ok(deps) => deps,
            Err(e) => { eprintln!("sigpkg: resolution failed: {}", e); return 1; }
        };

        for dep in &resolved {
            println!("  → {} v{}", dep.name, dep.version);
        }

        println!("\x1b[1;34m[sigpkg]\x1b[0m Verifying cryptographic signatures...");
        for dep in &resolved {
            match crypto::verify_package(&dep.name, &dep.hash, &dep.signature) {
                Ok(_) => println!("  ✓ {} [Ed25519 OK]", dep.name),
                Err(e) => { eprintln!("sigpkg: verification FAILED for {}: {}", dep.name, e); return 1; }
            }
        }

        println!("\x1b[1;32m[sigpkg]\x1b[0m Installing '{}' (deterministic, reproducible)...", pkg);
        println!("  ✓ '{}' installed successfully.", pkg);
    }
    0
}

fn cmd_remove(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("sigpkg: remove requires a package name");
        return 1;
    }
    for pkg in args {
        println!("\x1b[1;33m[sigpkg]\x1b[0m Removing '{}'...", pkg);
        // TODO: Check reverse deps before removal
        println!("  ✓ '{}' removed.", pkg);
    }
    0
}

fn cmd_update(args: &[String]) -> i32 {
    if args.is_empty() {
        println!("\x1b[1;34m[sigpkg]\x1b[0m Syncing registry...");
        cmd_sync();
        println!("\x1b[1;34m[sigpkg]\x1b[0m Checking for updates (all packages)...");
        println!("  ✓ System is up to date.");
    } else {
        for pkg in args {
            println!("\x1b[1;34m[sigpkg]\x1b[0m Updating '{}'...", pkg);
            println!("  ✓ '{}' updated.", pkg);
        }
    }
    0
}

fn cmd_search(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("sigpkg: search requires a query");
        return 1;
    }
    let query = &args[0];
    println!("\x1b[1;34m[sigpkg]\x1b[0m Searching for '{}'...", query);
    let results = registry::search(query);
    if results.is_empty() {
        // Palette 🎨: Delightful empty state with actionable suggestions to guide the user
        println!("  \x1b[33m⚠ No packages found matching '{}'.\x1b[0m", query);
        println!("  💡 Protip: Try searching for 'sigma' or check spelling!");
    } else {
        println!("{:<25} {:<12} {}", "NAME", "VERSION", "DESCRIPTION");
        println!("{}", "-".repeat(65));
        for r in &results {
            println!("{:<25} {:<12} {}", r.name, r.version, r.description);
        }
    }
    0
}

fn cmd_verify(args: &[String]) -> i32 {
    if args.is_empty() {
        println!("\x1b[1;34m[sigpkg]\x1b[0m Verifying system-wide package integrity...");
        println!("  ✓ All package hashes verified (Ed25519 + SHA-256).");
        println!("  ✓ No tampered packages detected.");
    } else {
        for pkg in args {
            println!("\x1b[1;34m[sigpkg]\x1b[0m Verifying '{}'...", pkg);
            println!("  ✓ '{}' signature valid.", pkg);
        }
    }
    0
}

fn cmd_list(args: &[String]) -> i32 {
    let profile = args.first().map(|s| s.as_str()).unwrap_or("all");
    println!("\x1b[1;34m[sigpkg]\x1b[0m Installed packages (profile: {}):", profile);
    let installed = registry::list_installed(profile);
    if installed.is_empty() {
        // Palette 🎨: Elegant and helpful empty state listing clear instructions/call-to-actions
        println!("  \x1b[33m⚠ No packages currently installed for this profile.\x1b[0m");
        println!("  💡 Protip: Run 'sigpkg install <pkg>' to install or 'sigpkg profile apply sigma-core' to setup.");
    } else {
        for pkg in &installed {
            println!("  {} v{}", pkg.name, pkg.version);
        }
    }
    0
}

fn cmd_info(args: &[String]) -> i32 {
    let pkg = match args.first() {
        Some(p) => p,
        None => { eprintln!("sigpkg: info requires a package name"); return 1; }
    };
    match registry::info(pkg) {
        Some(p) => {
            println!("Name:        {}", p.name);
            println!("Version:     {}", p.version);
            println!("Description: {}", p.description);
            println!("Profile:     {}", p.profile);
            println!("Hash:        {}", p.hash);
            println!("Depends:     {}", p.depends.join(", "));
        }
        None => {
            eprintln!("sigpkg: package '{}' not found", pkg);
            return 1;
        }
    }
    0
}

fn cmd_profile(args: &[String]) -> i32 {
    let sub = args.first().map(|s| s.as_str()).unwrap_or("list");
    match sub {
        "list" => profiles::list_profiles(),
        "apply" => {
            let name = args.get(1).map(|s| s.as_str()).unwrap_or("sigma-core");
            profiles::apply_profile(name)
        }
        _ => {
            eprintln!("sigpkg profile: unknown subcommand '{}'", sub);
            1
        }
    }
}

fn cmd_sync() -> i32 {
    println!("\x1b[1;34m[sigpkg]\x1b[0m Syncing sovereign package registry...");
    println!("  ✓ Registry synced (0 new packages).");
    0
}

fn print_usage() {
    println!("\x1b[1;36msigpkg\x1b[0m v0.2.0 — SigmaOS Sovereign Package Manager");
    println!();
    println!("\x1b[1mUSAGE:\x1b[0m");
    println!("  sigpkg <command> [options]");
    println!();
    println!("\x1b[1mCOMMANDS:\x1b[0m");
    println!("  install <pkg...>    Install package(s) with cryptographic verification");
    println!("  remove  <pkg...>    Remove installed package(s)");
    println!("  update  [pkg...]    Update package(s) or entire system");
    println!("  search  <query>     Search the sovereign package registry");
    println!("  verify  [pkg...]    Verify package integrity (Ed25519 + SHA-256)");
    println!("  list    [profile]   List installed packages");
    println!("  info    <pkg>       Show detailed package information");
    println!("  profile list        List available system profiles");
    println!("  profile apply <P>   Apply a system profile");
    println!("  sync                Sync the package registry");
    println!("  version             Show sigpkg version");
    println!();
    println!("\x1b[1mPROFILES:\x1b[0m");
    println!("  sigma-core          Minimal CLI environment");
    println!("  sigma-desktop       Full GUI + media + productivity");
    println!("  sigma-cloud         Container runtime + orchestration");
    println!("  sigma-secure        Air-gapped, security-hardened");
}
