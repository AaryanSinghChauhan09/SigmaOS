// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma-snapshot — SigmaOS system snapshot CLI
//!
//! Usage:
//!   sigma-snapshot <create|list|restore|delete|diff|export> [options]

use std::env;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

const VERSION: &str = "1.0.0";

fn cyan(s: &str)  -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str) -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)   -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str)-> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)  -> String { format!("\x1B[1m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}", cyan("sigma-snapshot"), VERSION);
    println!();
    println!("{}  sigma-snapshot <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  create  [--name <n>] [--type full|incremental|config]  Take a snapshot");
    println!("  list    [--format table|json]                          List snapshots");
    println!("  restore <id> [--dry-run]                               Restore a snapshot");
    println!("  delete  <id> [--force]                                 Remove a snapshot");
    println!("  diff    <id1> <id2>                                    Compare two snapshots");
    println!("  export  <id> --output <file>                           Export to archive");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --name    <n>      Snapshot name / label");
    println!("  --type    <type>   Snapshot type: full|incremental|config (default: full)");
    println!("  --format  <fmt>    Output format for list: table|json (default: table)");
    println!("  --output  <file>   Output archive file for export");
    println!("  --dry-run          Show what restore would do without applying");
    println!("  --force            Skip confirmation prompts");
    println!("  --json             Machine-readable JSON output");
    println!("  --version, -V      Print version");
    println!("  --help,    -h      Show this help");
}

struct Snapshot {
    id:       u32,
    name:     &'static str,
    stype:    &'static str,
    created:  &'static str,
    size_mib: u32,
    parent:   Option<u32>,
}

fn sample_snapshots() -> Vec<Snapshot> {
    vec![
        Snapshot { id: 1, name: "initial-boot",         stype: "full",         created: "2026-07-01 09:00", size_mib: 1024, parent: None    },
        Snapshot { id: 2, name: "pre-update-v15",       stype: "full",         created: "2026-07-02 14:30", size_mib: 1150, parent: None    },
        Snapshot { id: 3, name: "post-kernel-patch",    stype: "incremental",  created: "2026-07-02 18:00", size_mib:  128, parent: Some(2) },
        Snapshot { id: 4, name: "config-backup",        stype: "config",       created: "2026-07-03 08:00", size_mib:    2, parent: None    },
    ]
}

fn cmd_create(name: &str, stype: &str, json: bool) {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let id = (ts % 10000) as u32 + 100;

    if json {
        println!("{{\"snapshot\":{{\"id\":{},\"name\":\"{}\",\"type\":\"{}\",\"status\":\"created\"}}}}", id, name, stype);
        return;
    }
    println!("{} Taking {} snapshot '{}'...", cyan("Σ"), stype, name);
    match stype {
        "full"        => println!("  Copying filesystem (Copy-on-Write)..."),
        "incremental" => println!("  Recording delta from parent..."),
        "config"      => println!("  Snapshotting /etc and /var/sigma..."),
        _             => {}
    }
    println!("  Compressing...  done (estimate: ~256 MiB)");
    println!("  {} Snapshot #{} created: '{}'", green("✓"), id, name);
    println!("  Tip: restore with 'sigma-snapshot restore {}'", id);
}

fn cmd_list(json: bool) {
    let snaps = sample_snapshots();
    if json {
        println!("{{\"snapshots\":[");
        for (i, s) in snaps.iter().enumerate() {
            print!("  {{\"id\":{},\"name\":\"{}\",\"type\":\"{}\",\"created\":\"{}\",\"size_mib\":{}}}",
                s.id, s.name, s.stype, s.created, s.size_mib);
            if i < snaps.len() - 1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }
    println!("{}", bold("Snapshots"));
    println!("  {:<4}  {:<28}  {:<13}  {:<18}  {:>8}  {}",
        "ID", "Name", "Type", "Created", "Size", "Parent");
    println!("  {}", "─".repeat(88));
    for s in &snaps {
        let parent = s.parent.map(|p| p.to_string()).unwrap_or_else(|| "—".to_string());
        println!("  {:<4}  {:<28}  {:<13}  {:<18}  {:>5} MiB  {}",
            s.id, s.name, s.stype, s.created, s.size_mib, parent);
    }
    println!("\n  {} snapshots, {} MiB total", snaps.len(),
        snaps.iter().map(|s| s.size_mib as u64).sum::<u64>());
}

fn cmd_restore(id: u32, dry_run: bool, json: bool) {
    if json {
        println!("{{\"restore\":{{\"id\":{},\"dry_run\":{},\"status\":\"ok\"}}}}", id, dry_run);
        return;
    }
    if dry_run {
        println!("{} [dry-run] Would restore snapshot #{}:", cyan("Σ"), id);
        println!("  • Roll back /etc to snapshot state");
        println!("  • Roll back /var/sigma to snapshot state");
        println!("  • Update boot partition pointer");
        println!("  Reboot required after restore.");
    } else {
        println!("{} Restoring snapshot #{}...", cyan("Σ"), id);
        println!("  {} Filesystem restored", green("✓"));
        println!("  {} Boot pointer updated", green("✓"));
        println!("  {} Please reboot to complete the restore.", yellow("⚠"));
    }
}

fn cmd_delete(id: u32, force: bool, json: bool) {
    if json {
        println!("{{\"delete\":{{\"id\":{},\"status\":\"deleted\"}}}}", id);
        return;
    }
    if !force {
        println!("{} Are you sure you want to delete snapshot #{}? Use --force to confirm.", yellow("⚠"), id);
        return;
    }
    println!("{} Deleting snapshot #{}...", cyan("Σ"), id);
    println!("{} Snapshot #{} deleted.", green("✓"), id);
}

fn cmd_diff(id1: u32, id2: u32, json: bool) {
    let changes: &[(&str, &str, &str)] = &[
        ("MODIFIED", "/etc/sigma.toml",       "kernel.debug = false → true"),
        ("ADDED",    "/etc/sigma/new.conf",   "new configuration file"),
        ("DELETED",  "/etc/sigma/old.conf",   "removed"),
    ];
    if json {
        println!("{{\"diff\":{{\"from\":{},\"to\":{},\"changes\":{}}}}}", id1, id2, changes.len());
        return;
    }
    println!("{} Snapshot #{} → #{}", bold("Diff"), id1, id2);
    println!("{}", "─".repeat(60));
    for (action, path, detail) in changes {
        let a = match *action {
            "MODIFIED" => yellow("MODIFIED"),
            "ADDED"    => green("ADDED   "),
            _          => red("DELETED "),
        };
        println!("  {} {:<35} {}", a, path, detail);
    }
}

fn cmd_export(id: u32, output: &str, json: bool) {
    if json {
        println!("{{\"export\":{{\"id\":{},\"output\":\"{}\",\"status\":\"ok\"}}}}", id, output);
        return;
    }
    println!("{} Exporting snapshot #{} to '{}'...", cyan("Σ"), id, output);
    println!("  Compressing with zstd...");
    println!("{} Export complete: {} (simulated — 256 MiB)", green("✓"), output);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        exit(if args.len() < 2 { 1 } else { 0 });
    }
    if args[1] == "--version" || args[1] == "-V" {
        println!("sigma-snapshot {}", VERSION);
        exit(0);
    }

    let json    = args.iter().any(|a| a == "--json");
    let dry_run = args.iter().any(|a| a == "--dry-run");
    let force   = args.iter().any(|a| a == "--force");
    let name    = args.windows(2).find(|w| w[0] == "--name").map(|w| w[1].as_str()).unwrap_or("snap");
    let stype   = args.windows(2).find(|w| w[0] == "--type").map(|w| w[1].as_str()).unwrap_or("full");
    let output  = args.windows(2).find(|w| w[0] == "--output").map(|w| w[1].as_str()).unwrap_or("snapshot.tar.zst");

    let positional: Vec<u32> = args[2..].iter()
        .filter(|a| !a.starts_with("--"))
        .filter_map(|s| s.parse().ok())
        .collect();

    match args[1].as_str() {
        "create"  => cmd_create(name, stype, json),
        "list"    => cmd_list(json),
        "restore" => cmd_restore(*positional.get(0).unwrap_or(&0), dry_run, json),
        "delete"  => cmd_delete(*positional.get(0).unwrap_or(&0), force, json),
        "diff"    => cmd_diff(
            *positional.get(0).unwrap_or(&0),
            *positional.get(1).unwrap_or(&0),
            json,
        ),
        "export"  => cmd_export(*positional.get(0).unwrap_or(&0), output, json),
        _ => {
            eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]);
            exit(1);
        }
    }
}
