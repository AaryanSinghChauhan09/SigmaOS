/// cli/main.rs — sigmactl: SigmaOS Unified CLI (Rust native binary)
/// Zero external dependencies — uses only std + core/orchestrator.rs
///
/// Build: cargo build -p sigma-cli --release
/// Usage: sigmactl <command> [args...]

mod orchestrator {
    include!("../core/orchestrator.rs");
}
mod config {
    include!("../core/config.rs");
}

use orchestrator::ShardManager;
use std::env;

const VERSION: &str = "sigmactl v2.0 (SigmaOS Sovereign CLI)";

fn print_help() {
    println!("{VERSION}");
    println!();
    println!("USAGE:");
    println!("  sigmactl <command> [args...]");
    println!();
    println!("COMMANDS:");
    println!("  build                   Build all shards (Rust + C)");
    println!("  build <shard>           Build a specific shard");
    println!("  sync                    Sync with GitHub (fetch + push)");
    println!("  shard add <name>        Scaffold and register a new shard");
    println!("  shard remove <name>     Remove a shard");
    println!("  shard ls                List all registered shards");
    println!("  profile set <name>      Apply personalization profile");
    println!("  profile list            List available profiles");
    println!("  status                  Show system status");
    println!("  version                 Show version");
    println!("  help                    Show this help");
    println!();
    println!("EXAMPLES:");
    println!("  sigmactl build");
    println!("  sigmactl shard add analytics");
    println!("  sigmactl profile set developer");
    println!("  sigmactl status");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = env::var("SIGMA_ROOT")
        .unwrap_or_else(|_| ".".to_string());

    let mut mgr = ShardManager::with_root(&root);

    if args.len() < 2 {
        print_help();
        std::process::exit(0);
    }

    match args[1].as_str() {

        "build" => {
            if args.len() > 2 {
                match mgr.build_shard(&args[2]) {
                    Ok(()) => println!("Σ [OK] Built shard: {}", args[2]),
                    Err(e)  => { eprintln!("Σ [ERR] {e}"); std::process::exit(1); }
                }
            } else {
                match mgr.build_all() {
                    Ok(())  => println!("Σ [OK] All shards built."),
                    Err(e)  => { eprintln!("Σ [ERR] {e}"); std::process::exit(1); }
                }
            }
        }

        "sync" => {
            match mgr.sync_github() {
                Ok(msg)  => println!("Σ [OK] {msg}"),
                Err(e)   => { eprintln!("Σ [ERR] {e}"); std::process::exit(1); }
            }
        }

        "shard" => {
            match args.get(2).map(|s| s.as_str()) {
                Some("ls") | Some("list") => {
                    let shards = mgr.list_shards();
                    println!("{:<24} {:<10} {:<10}", "Shard", "Lang", "State");
                    println!("{}", "─".repeat(44));
                    for s in shards {
                        let st = match &s.state {
                            orchestrator::ShardState::Active   => "ACTIVE",
                            orchestrator::ShardState::Inactive => "INACTIVE",
                            orchestrator::ShardState::Error(_) => "ERROR",
                        };
                        println!("  {:<22} {:<10} {:<10}", s.name, s.lang, st);
                    }
                }
                Some("add") => {
                    let name = args.get(3).map(|s| s.as_str()).unwrap_or_else(|| {
                        eprintln!("Usage: sigmactl shard add <name>"); std::process::exit(1);
                    });
                    match mgr.add_shard(name) {
                        Ok(()) => println!("Σ [OK] Shard '{name}' scaffolded."),
                        Err(e) => { eprintln!("Σ [ERR] {e}"); std::process::exit(1); }
                    }
                }
                Some("remove") => {
                    let name = args.get(3).map(|s| s.as_str()).unwrap_or_else(|| {
                        eprintln!("Usage: sigmactl shard remove <name>"); std::process::exit(1);
                    });
                    let _ = mgr.remove_shard(name);
                    println!("Σ [OK] Shard '{name}' removed.");
                }
                _ => {
                    eprintln!("Usage: sigmactl shard [ls|add|remove] [name]");
                    std::process::exit(1);
                }
            }
        }

        "profile" => {
            match args.get(2).map(|s| s.as_str()) {
                Some("set") | Some("switch") => {
                    let name = args.get(3).map(|s| s.as_str()).unwrap_or_else(|| {
                        eprintln!("Usage: sigmactl profile set <name>"); std::process::exit(1);
                    });
                    match mgr.apply_profile(name) {
                        Ok(()) => println!("Σ [OK] Profile '{name}' applied."),
                        Err(e) => { eprintln!("Σ [ERR] {e}"); std::process::exit(1); }
                    }
                }
                Some("list") | Some("ls") => {
                    let profiles_dir = std::path::Path::new(&root).join("profiles");
                    if let Ok(entries) = std::fs::read_dir(profiles_dir) {
                        println!("Available profiles:");
                        for e in entries.flatten() {
                            if let Some(name) = e.path().file_stem() {
                                println!("  - {}", name.to_string_lossy());
                            }
                        }
                    }
                }
                _ => {
                    eprintln!("Usage: sigmactl profile [set|list] [name]");
                    std::process::exit(1);
                }
            }
        }

        "status" => {
            println!("{}", mgr.status());
        }

        "version" => println!("{VERSION}"),
        "help" | "--help" | "-h" => print_help(),
        unknown => {
            eprintln!("Σ [ERR] Unknown command: '{unknown}'. Run 'sigmactl help'.");
            std::process::exit(1);
        }
    }
}
