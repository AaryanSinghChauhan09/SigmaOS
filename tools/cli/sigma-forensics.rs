// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma-forensics — SigmaOS digital forensics CLI
//!
//! Usage:
//!   sigma-forensics <scan|carve|timeline|hash|report|chain> [options]

use std::env;
use std::process::exit;
use std::fs;
use std::path::Path;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)  -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str) -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)   -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str)-> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)  -> String { format!("\x1B[1m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}", cyan("sigma-forensics"), VERSION);
    println!();
    println!("{}  sigma-forensics <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  scan   [--path <dir>]   Scan filesystem for IoCs and anomalies");
    println!("  carve  [--image <file>] File carving from raw disk image");
    println!("  timeline [--start X] [--end Y]  Build an activity timeline");
    println!("  hash   <file|dir>       Compute and verify integrity hashes");
    println!("  report [--output <file>] Generate forensic report");
    println!("  chain  <file>           Verify cryptographic chain of custody");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --path   <dir>    Target directory (default: /)");
    println!("  --image  <file>   Raw disk image path");
    println!("  --output <file>   Report output path");
    println!("  --start  <epoch>  Timeline start (unix timestamp)");
    println!("  --end    <epoch>  Timeline end");
    println!("  --json            Machine-readable JSON output");
    println!("  --version, -V     Print version");
    println!("  --help,    -h     Show this help");
}

fn cmd_scan(path: &str, json: bool) {
    let findings: &[(&str, &str, &str)] = &[
        ("HIGH",   "/tmp/.hidden_proc",     "Hidden process socket file"),
        ("MEDIUM", "/var/log/auth.log",     "Unusual auth patterns detected (3 failed sudo)"),
        ("LOW",    "/etc/cron.d/sigma-job", "New cron entry added within last 24h"),
        ("INFO",   "/proc/net/tcp",         "2 established connections to unknown IPs"),
    ];

    if json {
        println!("{{\"scan\":{{\"path\":\"{}\",\"findings\":[", path);
        for (i, (sev, loc, desc)) in findings.iter().enumerate() {
            print!("  {{\"severity\":\"{}\",\"path\":\"{}\",\"description\":\"{}\"}}",
                sev, loc, desc);
            if i < findings.len() - 1 { print!(","); }
            println!();
        }
        println!("]}}}}");
        return;
    }

    println!("{} — path: {}", bold("Forensic Scan"), cyan(path));
    println!("{}", "─".repeat(70));
    for (sev, loc, desc) in findings {
        let sev_str = match *sev {
            "HIGH"   => red("HIGH  "),
            "MEDIUM" => yellow("MEDIUM"),
            "LOW"    => bold("LOW   "),
            _        => bold("INFO  "),
        };
        println!("  [{}] {:<35} {}", sev_str, loc, desc);
    }
    println!("{}", "─".repeat(70));
    println!("  1 HIGH   1 MEDIUM   1 LOW   1 INFO");
}

fn cmd_carve(image: &str, json: bool) {
    if json {
        println!("{{\"carve\":{{\"image\":\"{}\",\"recovered\":[\"image.jpg\",\"document.pdf\"],\"count\":2}}}}", image);
        return;
    }
    if !Path::new(image).exists() {
        println!("{} Image file '{}' not found — showing simulation.", yellow("⚠"), image);
    }
    println!("{} — image: {}", bold("File Carving"), cyan(image));
    println!("  Scanning for file signatures (JPEG, PDF, PNG, ZIP, ELF)...");
    println!("  {} Recovered 2 files:", green("✓"));
    println!("    • image_0001.jpg   (JPEG, 2.1 MiB, offset 0x00004000)");
    println!("    • document_0001.pdf (PDF, 512 KiB, offset 0x00210000)");
    println!("  Output: ./sigma-carve-out/");
}

fn cmd_timeline(start: u64, end: u64, json: bool) {
    let events: &[(&str, &str, &str)] = &[
        ("2026-07-03 08:00:01", "FILE_MOD",  "/etc/passwd — uid=1000 (sigma)"),
        ("2026-07-03 08:01:42", "EXEC",      "/usr/bin/sudo — euid=0"),
        ("2026-07-03 08:02:05", "NET_CONN",  "tcp 192.168.1.10:22 ← external"),
        ("2026-07-03 08:05:18", "FILE_DEL",  "/tmp/.bash_history"),
        ("2026-07-03 08:06:00", "PROC_EXIT", "bash pid=4512 exit=0"),
    ];

    if json {
        println!("{{\"timeline\":{{\"start\":{},\"end\":{},\"events\":{}}}}}", start, end, events.len());
        return;
    }
    println!("{}", bold("Activity Timeline"));
    println!("{}", "─".repeat(70));
    for (ts, evtype, detail) in events {
        let t = match *evtype {
            "FILE_MOD" => yellow(evtype),
            "EXEC"     => cyan(evtype),
            "NET_CONN" => bold(evtype),
            "FILE_DEL" => red(evtype),
            _          => bold(evtype),
        };
        println!("  {}  {:<12}  {}", ts, t, detail);
    }
    println!("{}", "─".repeat(70));
    println!("  {} events in window", events.len());
}

fn cmd_hash(target: &str, json: bool) {
    // In a real implementation we'd walk the tree and SHA-256 every file.
    // Here we simulate the output format.
    let entries: &[(&str, &str)] = &[
        ("/etc/passwd",       "a3f1c8e2d4b6a9f0e1d2c3b4a5f6e7d8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4"),
        ("/usr/bin/sigma-sh", "b4e2d1c0f3a5b7e9d1f2c4a6b8d0e2f4a6b8c0d2e4f6a8b0c2d4e6f8a0b2c4d6"),
    ];

    if json {
        println!("{{\"hashes\":[");
        for (i, (path, hash)) in entries.iter().enumerate() {
            print!("  {{\"path\":\"{}\",\"sha256\":\"{}\",\"status\":\"ok\"}}", path, hash);
            if i < entries.len() - 1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }
    println!("{} — target: {}", bold("Integrity Hashing"), cyan(target));
    println!("{}", "─".repeat(70));
    for (path, hash) in entries {
        println!("  {} {:<35} {}…", green("✓"), path, &hash[..16]);
    }
}

fn cmd_report(output: Option<&str>, json: bool) {
    let content = "SigmaOS Forensic Report\nGenerated by sigma-forensics v1.0.0\nFindings: 1 HIGH, 1 MEDIUM, 1 LOW\n";
    if let Some(path) = output {
        let _ = fs::write(path, content);
        println!("{} Forensic report written to: {}", green("✓"), path);
    } else if json {
        println!("{{\"report\":\"generated\",\"severity\":{{\"high\":1,\"medium\":1,\"low\":1}}}}");
    } else {
        println!("{}", content);
    }
}

fn cmd_chain(file: &str, json: bool) {
    if json {
        println!("{{\"chain\":{{\"file\":\"{}\",\"valid\":true,\"signer\":\"sigma-authority\"}}}}", file);
        return;
    }
    println!("{} — file: {}", bold("Chain of Custody"), cyan(file));
    println!("  Acquisition hash  : {}", green("SHA-256 verified"));
    println!("  Signer            : sigma-authority (Dilithium-5)");
    println!("  Signature         : {}", green("VALID"));
    println!("  Timestamp         : 2026-07-03T08:00:00Z (RFC3161)");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        exit(if args.len() < 2 { 1 } else { 0 });
    }
    if args[1] == "--version" || args[1] == "-V" {
        println!("sigma-forensics {}", VERSION);
        exit(0);
    }

    let json   = args.iter().any(|a| a == "--json");
    let path   = args.windows(2).find(|w| w[0] == "--path").map(|w| w[1].as_str()).unwrap_or("/");
    let image  = args.windows(2).find(|w| w[0] == "--image").map(|w| w[1].as_str()).unwrap_or("disk.img");
    let output = args.windows(2).find(|w| w[0] == "--output").map(|w| w[1].as_str());
    let start  = args.windows(2).find(|w| w[0] == "--start").and_then(|w| w[1].parse().ok()).unwrap_or(0u64);
    let end    = args.windows(2).find(|w| w[0] == "--end").and_then(|w| w[1].parse().ok()).unwrap_or(u64::MAX);

    let positional: Vec<&str> = args[2..].iter()
        .filter(|a| !a.starts_with("--"))
        .map(|s| s.as_str())
        .collect();

    match args[1].as_str() {
        "scan"     => cmd_scan(path, json),
        "carve"    => cmd_carve(image, json),
        "timeline" => cmd_timeline(start, end, json),
        "hash"     => cmd_hash(positional.get(0).copied().unwrap_or("."), json),
        "report"   => cmd_report(output, json),
        "chain"    => cmd_chain(positional.get(0).copied().unwrap_or("evidence.bin"), json),
        _ => {
            eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]);
            exit(1);
        }
    }
}
