// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma-cluster — SigmaOS cluster management CLI
//!
//! Usage:
//!   sigma-cluster <status|enroll|drain|evict|upgrade|logs|metrics> [options]

use std::env;
use std::process::exit;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)  -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str) -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)   -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str)-> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)  -> String { format!("\x1B[1m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}", cyan("sigma-cluster"), VERSION);
    println!();
    println!("{}  sigma-cluster <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  status  [--node <n>]   Show cluster and node health");
    println!("  enroll  --node <addr>  Add a new node to the cluster");
    println!("  drain   --node <name>  Cordon and drain a node");
    println!("  evict   --node <name>  Force-remove a node");
    println!("  upgrade [--node <n>] [--channel stable|nightly]  Rolling upgrade");
    println!("  logs    --node <name> [--tail <n>]                Node logs");
    println!("  metrics [--node <n>]   Show cluster metrics");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --node     <name>  Target node (default: all)");
    println!("  --channel  <ch>    Update channel (default: stable)");
    println!("  --tail     <n>     Show last N log lines");
    println!("  --json             Machine-readable JSON output");
    println!("  --version, -V      Print version");
    println!("  --help,    -h      Show this help");
}

struct Node {
    name:    &'static str,
    addr:    &'static str,
    status:  &'static str,
    version: &'static str,
    cpu_pct: u8,
    mem_pct: u8,
    shards:  u8,
}

fn sample_nodes() -> Vec<Node> {
    vec![
        Node { name: "sigma-node-01", addr: "10.0.0.1", status: "Ready",       version: "v15.0", cpu_pct: 22, mem_pct: 45, shards: 8  },
        Node { name: "sigma-node-02", addr: "10.0.0.2", status: "Ready",       version: "v15.0", cpu_pct: 61, mem_pct: 70, shards: 12 },
        Node { name: "sigma-node-03", addr: "10.0.0.3", status: "NotReady",    version: "v14.9", cpu_pct:  0, mem_pct: 10, shards:  0 },
        Node { name: "sigma-node-04", addr: "10.0.0.4", status: "Cordoned",    version: "v15.0", cpu_pct: 10, mem_pct: 20, shards:  3 },
    ]
}

fn cmd_status(node_filter: Option<&str>, json: bool) {
    let nodes = sample_nodes();
    let filtered: Vec<&Node> = if let Some(f) = node_filter {
        nodes.iter().filter(|n| n.name == f || n.addr == f).collect()
    } else {
        nodes.iter().collect()
    };

    if json {
        println!("{{\"nodes\":[");
        for (i, n) in filtered.iter().enumerate() {
            print!("  {{\"name\":\"{}\",\"addr\":\"{}\",\"status\":\"{}\",\"version\":\"{}\",\"cpu\":{},\"mem\":{},\"shards\":{}}}",
                n.name, n.addr, n.status, n.version, n.cpu_pct, n.mem_pct, n.shards);
            if i < filtered.len() - 1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }

    println!("{}", bold("Cluster Status"));
    println!("  {:<18}  {:<12}  {:<12}  {:<7}  {:>4}  {:>4}  {:>6}",
        "Node", "Address", "Status", "Version", "CPU%", "MEM%", "Shards");
    println!("  {}", "─".repeat(72));
    for n in &filtered {
        let status_str = match n.status {
            "Ready"    => green(n.status),
            "NotReady" => red(n.status),
            _          => yellow(n.status),
        };
        println!("  {:<18}  {:<12}  {:<20}  {:<7}  {:>4}%  {:>4}%  {:>6}",
            n.name, n.addr, status_str, n.version, n.cpu_pct, n.mem_pct, n.shards);
    }
    let ready = filtered.iter().filter(|n| n.status == "Ready").count();
    println!("\n  {}/{} nodes ready", ready, filtered.len());
}

fn cmd_enroll(addr: &str, json: bool) {
    if json {
        println!("{{\"enroll\":{{\"addr\":\"{}\",\"status\":\"enrolled\"}}}}", addr);
        return;
    }
    println!("{} Enrolling node at {}...", cyan("Σ"), addr);
    println!("  Exchanging TLS certificates...");
    println!("  Provisioning sigma-agent...");
    println!("  Registering in cluster ledger...");
    println!("{} Node {} enrolled successfully.", green("✓"), addr);
}

fn cmd_drain(node: &str, json: bool) {
    if json {
        println!("{{\"drain\":{{\"node\":\"{}\",\"status\":\"drained\"}}}}", node);
        return;
    }
    println!("{} Draining node '{}'...", cyan("Σ"), node);
    println!("  Cordoning (no new shards)...");
    println!("  Migrating 3 active shards to other nodes...");
    println!("{} Node '{}' drained. Safe to maintain.", green("✓"), node);
}

fn cmd_evict(node: &str, json: bool) {
    if json {
        println!("{{\"evict\":{{\"node\":\"{}\",\"status\":\"evicted\"}}}}", node);
        return;
    }
    println!("{} Force-evicting node '{}'...", yellow("⚠"), node);
    println!("  Rescheduling orphaned shards...");
    println!("{} Node '{}' removed from cluster.", green("✓"), node);
}

fn cmd_upgrade(node: Option<&str>, channel: &str, json: bool) {
    let target = node.unwrap_or("all nodes");
    if json {
        println!("{{\"upgrade\":{{\"target\":\"{}\",\"channel\":\"{}\",\"status\":\"ok\"}}}}", target, channel);
        return;
    }
    println!("{} Rolling upgrade — target: {}  channel: {}", cyan("Σ"), bold(target), cyan(channel));
    if node.is_some() {
        println!("  Draining {} before upgrade...", target);
        println!("  Applying new sigma kernel image...");
        println!("  Rebooting {}...", target);
        println!("{} {} upgraded to {}", green("✓"), target, channel);
    } else {
        println!("  Upgrading nodes one by one (rolling)...");
        for n in &sample_nodes() {
            println!("  {} {}", green("✓"), n.name);
        }
        println!("{} All nodes upgraded.", green("✓"));
    }
}

fn cmd_logs(node: &str, tail: usize, json: bool) {
    let lines: &[&str] = &[
        "2026-07-03 08:00:01 INFO  sigma-agent: heartbeat ok",
        "2026-07-03 08:00:30 INFO  shard-engine: shard 7 healthy",
        "2026-07-03 08:01:00 WARN  scheduler: node load 61% — consider rebalancing",
        "2026-07-03 08:01:30 INFO  sigma-agent: heartbeat ok",
        "2026-07-03 08:02:00 INFO  sigma-net: peer mesh refresh complete",
    ];
    let shown: Vec<&&str> = lines.iter().rev().take(tail).collect::<Vec<_>>().into_iter().rev().collect();

    if json {
        println!("{{\"logs\":{{\"node\":\"{}\",\"lines\":{}}}}}", node, shown.len());
        return;
    }
    println!("{} — {}", bold("Logs"), cyan(node));
    for line in shown {
        if line.contains("WARN") {
            println!("  {}", yellow(line));
        } else {
            println!("  {}", line);
        }
    }
}

fn cmd_metrics(node_filter: Option<&str>, json: bool) {
    if json {
        println!("{{\"metrics\":{{\"cluster_cpu_avg\":31,\"cluster_mem_avg\":36,\"total_shards\":23}}}}");
        return;
    }
    println!("{}", bold("Cluster Metrics"));
    println!("  Cluster CPU avg  : {}%", 31);
    println!("  Cluster MEM avg  : {}%", 36);
    println!("  Total shards     : 23");
    println!("  Messages/sec     : 1,420");
    println!("  Avg latency      : 0.8 ms");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        exit(if args.len() < 2 { 1 } else { 0 });
    }
    if args[1] == "--version" || args[1] == "-V" {
        println!("sigma-cluster {}", VERSION);
        exit(0);
    }

    let json    = args.iter().any(|a| a == "--json");
    let node    = args.windows(2).find(|w| w[0] == "--node").map(|w| w[1].as_str());
    let channel = args.windows(2).find(|w| w[0] == "--channel").map(|w| w[1].as_str()).unwrap_or("stable");
    let tail    = args.windows(2).find(|w| w[0] == "--tail").and_then(|w| w[1].parse().ok()).unwrap_or(20usize);

    match args[1].as_str() {
        "status"  => cmd_status(node, json),
        "enroll"  => cmd_enroll(node.unwrap_or("127.0.0.1"), json),
        "drain"   => cmd_drain(node.unwrap_or("sigma-node-01"), json),
        "evict"   => cmd_evict(node.unwrap_or("sigma-node-01"), json),
        "upgrade" => cmd_upgrade(node, channel, json),
        "logs"    => cmd_logs(node.unwrap_or("sigma-node-01"), tail, json),
        "metrics" => cmd_metrics(node, json),
        _ => {
            eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]);
            exit(1);
        }
    }
}
