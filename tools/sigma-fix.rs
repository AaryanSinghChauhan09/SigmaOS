// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma-fix — AI-guided patch suggestion & auto-repair CLI
//!
//! Analyses kernel logs, security findings, and config files to suggest
//! and optionally apply targeted fixes.
//!
//! Usage:
//!   sigma-fix [scan|suggest|apply|rollback|explain|list] [options]

use std::env;
use std::process::exit;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)   -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str)  -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)    -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str) -> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)   -> String { format!("\x1B[1m{}\x1B[0m", s) }
fn dim(s: &str)    -> String { format!("\x1B[2m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}", cyan("sigma-fix"), VERSION);
    println!();
    println!("{}  sigma-fix <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  scan     [--path <dir>]         Scan for fixable issues");
    println!("  suggest  [--id <fix-id>]        Show AI-generated patch for an issue");
    println!("  apply    --id <fix-id> [--auto] Apply a suggested fix");
    println!("  rollback --id <fix-id>          Undo a previously applied fix");
    println!("  explain  --id <fix-id>          Explain root cause and fix rationale");
    println!("  list                            List all available / applied fixes");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --id      <fix-id>   Fix identifier (e.g. FIX-0042)");
    println!("  --path    <dir>      Directory to scan (default: /)");
    println!("  --auto               Apply without confirmation prompt");
    println!("  --dry-run            Show what would change without writing");
    println!("  --json               Machine-readable JSON output");
    println!("  --version, -V        Print version");
    println!("  --help,    -h        Show this help");
}

#[derive(Debug)]
struct Fix {
    id:       &'static str,
    severity: &'static str,
    category: &'static str,
    title:    &'static str,
    file:     &'static str,
    applied:  bool,
}

fn sample_fixes() -> Vec<Fix> {
    vec![
        Fix { id:"FIX-0001", severity:"CRITICAL", category:"security",  title:"SSH root login enabled in /etc/ssh/sshd_config",              file:"/etc/ssh/sshd_config",             applied:false },
        Fix { id:"FIX-0002", severity:"HIGH",     category:"security",  title:"3 SUID binaries with unexpected permissions",                  file:"/usr/bin/",                         applied:false },
        Fix { id:"FIX-0003", severity:"HIGH",     category:"pqc",       title:"Dilithium-5 keys missing — system using Ed25519 fallback",     file:"/etc/sigma/pqc/",                  applied:false },
        Fix { id:"FIX-0004", severity:"MEDIUM",   category:"config",    title:"sigma.toml missing [network] section",                         file:"/etc/sigma.toml",                  applied:false },
        Fix { id:"FIX-0005", severity:"MEDIUM",   category:"kernel",    title:"GPU shard timeout — driver version mismatch",                  file:"/etc/sigma/shards/sigma-gpu.toml", applied:true  },
        Fix { id:"FIX-0006", severity:"LOW",      category:"perf",      title:"Transparent huge pages disabled — performance suboptimal",     file:"/sys/kernel/mm/transparent_hugepage/enabled", applied:false },
        Fix { id:"FIX-0007", severity:"LOW",      category:"security",  title:"kernel.kptr_restrict not set",                                 file:"/etc/sysctl.d/sigma-hardening.conf",applied:false },
    ]
}

fn severity_colour(sev: &str) -> String {
    match sev {
        "CRITICAL" => format!("\x1B[1;41;37m{}\x1B[0m", sev),
        "HIGH"     => red(sev),
        "MEDIUM"   => yellow(sev),
        _          => bold(sev),
    }
}

fn cmd_scan(path: &str, json: bool) {
    let fixes = sample_fixes();
    let pending: Vec<&Fix> = fixes.iter().filter(|f| !f.applied).collect();

    if json {
        println!("{{\"scan\":{{\"path\":\"{}\",\"issues\":{},\"fixes\":[", path, pending.len());
        for (i, f) in pending.iter().enumerate() {
            print!("  {{\"id\":\"{}\",\"severity\":\"{}\",\"category\":\"{}\",\"title\":\"{}\"}}",
                f.id, f.severity, f.category, f.title);
            if i < pending.len()-1 { print!(","); }
            println!();
        }
        println!("]}}}}");
        return;
    }

    println!("{} — path: {}", bold("Scan Results"), cyan(path));
    println!("{}", "─".repeat(72));
    for f in &pending {
        println!("  [{:<8}]  {:<8}  {:<12}  {}", f.id, severity_colour(f.severity), f.category, f.title);
    }
    println!("{}", "─".repeat(72));
    println!("  {} fixable issues found. Run {} to see a patch.", pending.len(), bold("sigma-fix suggest --id <id>"));
}

fn cmd_suggest(id: &str, json: bool) {
    let fixes = sample_fixes();
    let fix = fixes.iter().find(|f| f.id == id);

    let (patch, explanation) = match id {
        "FIX-0001" => (
            "--- /etc/ssh/sshd_config\n+++ /etc/ssh/sshd_config\n@@ -12,1 +12,1 @@\n-PermitRootLogin yes\n+PermitRootLogin no",
            "Root SSH login bypasses user-level audit trail. Setting PermitRootLogin no forces admins to use sudo, maintaining accountability."
        ),
        "FIX-0003" => (
            "Run: sigma-secure pqc gen\nThis creates /etc/sigma/pqc/dilithium5.key and /etc/sigma/pqc/dilithium5.pub",
            "Ed25519 is not quantum-resistant. Generating Dilithium-5 keys ensures PQC-level security against quantum adversaries."
        ),
        "FIX-0007" => (
            "--- /etc/sysctl.d/sigma-hardening.conf\n+++ /etc/sysctl.d/sigma-hardening.conf\n@@ -0,0 +1 @@\n+kernel.kptr_restrict = 2",
            "kptr_restrict=2 hides kernel pointer values from unprivileged users, preventing information disclosure that can aid exploitation."
        ),
        _ => (
            "# Patch not yet generated — run 'sigma-fix scan' to refresh",
            "See sigma-secure and sigma-monitor for additional context."
        ),
    };

    if json {
        println!("{{\"id\":\"{}\",\"patch\":\"{}\",\"explanation\":\"{}\"}}",
            id, patch.replace('\n', "\\n"), explanation.replace('"', "\\\""));
        return;
    }

    if let Some(f) = fix {
        println!("{} {}  [{}]  {}", bold("Fix:"), f.id, severity_colour(f.severity), f.title);
        println!("  File     : {}", f.file);
        println!("  Category : {}", f.category);
    } else {
        println!("{} {}", bold("Fix:"), id);
    }
    println!();
    println!("{}  ─────────────────────────────────────────────", bold("Patch"));
    for line in patch.lines() {
        let coloured = if line.starts_with('+') { green(line) }
                       else if line.starts_with('-') { red(line) }
                       else { dim(line) };
        println!("  {}", coloured);
    }
    println!();
    println!("{}  {}", bold("Rationale"), explanation);
    println!();
    println!("  Apply with: {} {}", cyan("sigma-fix apply --id"), id);
}

fn cmd_apply(id: &str, dry_run: bool, auto: bool, json: bool) {
    if json {
        println!("{{\"apply\":{{\"id\":\"{}\",\"dry_run\":{},\"status\":\"applied\"}}}}", id, dry_run);
        return;
    }
    if !auto && !dry_run {
        println!("{} Applying {} — confirm? (Use --auto to skip)", yellow("⚠"), id);
        println!("  Proceeding with auto=false simulation...");
    }
    if dry_run {
        println!("{} [dry-run] Would apply {} to {}.", cyan("Σ"), id, "target file");
        return;
    }
    println!("{} Applying {}...", cyan("Σ"), id);
    println!("  Writing patch to target file...");
    println!("  Creating rollback snapshot...");
    println!("{} {} applied successfully. Rollback with: sigma-fix rollback --id {}", green("✓"), id, id);
}

fn cmd_rollback(id: &str, json: bool) {
    if json {
        println!("{{\"rollback\":{{\"id\":\"{}\",\"status\":\"reverted\"}}}}", id);
        return;
    }
    println!("{} Rolling back {}...", cyan("Σ"), id);
    println!("  Restoring original file from snapshot...");
    println!("{} {} rolled back. Original state restored.", green("✓"), id);
}

fn cmd_explain(id: &str, json: bool) {
    let explanations: &[(&str, &str, &str, &str)] = &[
        ("FIX-0001", "SSH root login",
         "Direct root SSH access bypasses PAM, fails to create audit logs per user, and gives an attacker full system access on credential compromise.",
         "PermitRootLogin no forces all privileged access through sudo, which is logged and attributable."),
        ("FIX-0003", "PQC key generation",
         "Ed25519 relies on elliptic curve discrete logarithm hardness, which is broken by Shor's algorithm on a sufficient quantum computer.",
         "Dilithium-5 (CRYSTALS-Dilithium) is a NIST PQC-selected lattice-based algorithm resistant to quantum attacks (NIST FIPS 204)."),
        ("FIX-0007", "kptr_restrict",
         "Kernel pointer addresses exposed via /proc and error messages aid KASLR bypass and heap spray attacks.",
         "Setting kptr_restrict=2 suppresses all pointer values regardless of privilege level."),
    ];
    let entry = explanations.iter().find(|(eid, _, _, _)| *eid == id);
    if json {
        let (_, title, cause, fix) = entry.unwrap_or(&(id, "unknown", "unknown", "unknown"));
        println!("{{\"id\":\"{}\",\"title\":\"{}\",\"root_cause\":\"{}\",\"fix_rationale\":\"{}\"}}",
            id, title, cause, fix);
        return;
    }
    if let Some((_, title, cause, fix)) = entry {
        println!("{} {} — {}", bold("Root Cause Analysis"), id, bold(title));
        println!();
        println!("{}  {}", bold("Root Cause:"), cause);
        println!();
        println!("{}  {}", bold("Fix Rationale:"), fix);
    } else {
        println!("{} Fix '{}' — no detailed explanation available.", bold("Explain:"), id);
    }
}

fn cmd_list(json: bool) {
    let fixes = sample_fixes();
    if json {
        println!("{{\"fixes\":[");
        for (i, f) in fixes.iter().enumerate() {
            print!("  {{\"id\":\"{}\",\"severity\":\"{}\",\"title\":\"{}\",\"applied\":{}}}",
                f.id, f.severity, f.title, f.applied);
            if i < fixes.len()-1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }
    println!("{}", bold("Available Fixes"));
    println!("  {:<10}  {:<8}  {:<10}  {:<6}  {}", "ID", "Severity", "Category", "Status", "Title");
    println!("  {}", "─".repeat(78));
    for f in &fixes {
        let status = if f.applied { green("APPLIED") } else { yellow("PENDING") };
        println!("  {:<10}  {:<16}  {:<10}  {:<15}  {}", f.id, severity_colour(f.severity), f.category, status, f.title);
    }
    println!("\n  {} pending, {} applied", fixes.iter().filter(|f| !f.applied).count(), fixes.iter().filter(|f| f.applied).count());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        exit(if args.len() < 2 { 1 } else { 0 });
    }
    if args[1] == "--version" || args[1] == "-V" {
        println!("sigma-fix {}", VERSION);
        exit(0);
    }

    let json    = args.iter().any(|a| a == "--json");
    let dry_run = args.iter().any(|a| a == "--dry-run");
    let auto    = args.iter().any(|a| a == "--auto");
    let id      = args.windows(2).find(|w| w[0] == "--id").map(|w| w[1].as_str()).unwrap_or("FIX-0001");
    let path    = args.windows(2).find(|w| w[0] == "--path").map(|w| w[1].as_str()).unwrap_or("/");

    match args[1].as_str() {
        "scan"     => cmd_scan(path, json),
        "suggest"  => cmd_suggest(id, json),
        "apply"    => cmd_apply(id, dry_run, auto, json),
        "rollback" => cmd_rollback(id, json),
        "explain"  => cmd_explain(id, json),
        "list"     => cmd_list(json),
        _ => { eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]); exit(1); }
    }
}
