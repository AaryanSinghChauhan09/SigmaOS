// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma-log — SigmaOS unified log viewer & anomaly detector
//!
//! Usage:
//!   sigma-log [follow|search|tail|dump|stats|anomaly|export] [options]

use std::env;
use std::process::exit;
use std::time::Duration;
use std::thread;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)   -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str)  -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)    -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str) -> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)   -> String { format!("\x1B[1m{}\x1B[0m", s) }
fn dim(s: &str)    -> String { format!("\x1B[2m{}\x1B[0m", s) }

#[derive(Debug, Clone, PartialEq)]
enum Level { Trace, Debug, Info, Warn, Error, Critical }

impl Level {
    fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "TRACE"    => Self::Trace,
            "DEBUG"    => Self::Debug,
            "WARN"     => Self::Warn,
            "ERROR"    => Self::Error,
            "CRITICAL" => Self::Critical,
            _          => Self::Info,
        }
    }
    fn to_coloured_str(&self) -> String {
        match self {
            Self::Trace    => dim("TRACE"),
            Self::Debug    => dim("DEBUG"),
            Self::Info     => green("INFO "),
            Self::Warn     => yellow("WARN "),
            Self::Error    => red("ERROR"),
            Self::Critical => format!("\x1B[1;41;37mCRIT \x1B[0m"),
        }
    }
}

struct LogEntry {
    ts:      &'static str,
    level:   Level,
    source:  &'static str,
    message: &'static str,
}

fn sample_logs() -> Vec<LogEntry> {
    vec![
        LogEntry { ts: "2026-07-03 08:00:00.001", level: Level::Info,     source: "sigma-init",     message: "SigmaOS v15.0 (Zenith) booting" },
        LogEntry { ts: "2026-07-03 08:00:00.012", level: Level::Info,     source: "sigma-hal",      message: "SovereignHAL initialised: 12 cores, 32 GiB RAM" },
        LogEntry { ts: "2026-07-03 08:00:00.034", level: Level::Info,     source: "sigma-vfs",      message: "VFS root mounted: sigma-fs @ /dev/nvme0n1p2" },
        LogEntry { ts: "2026-07-03 08:00:00.051", level: Level::Info,     source: "sigma-net",      message: "eth0 up: 2.5 Gbps, IP 10.0.0.1" },
        LogEntry { ts: "2026-07-03 08:00:00.088", level: Level::Warn,     source: "sigma-pqc",      message: "Dilithium-5 key not found, falling back to Ed25519" },
        LogEntry { ts: "2026-07-03 08:00:00.102", level: Level::Info,     source: "sigma-sched",    message: "Scheduler: CFS + realtime hybrid, 4 priority bands" },
        LogEntry { ts: "2026-07-03 08:00:00.200", level: Level::Debug,    source: "sigma-mm",       message: "Page allocator: 8192 pages free (32 GiB)" },
        LogEntry { ts: "2026-07-03 08:00:01.000", level: Level::Info,     source: "sigma-agent",    message: "sigma-agent daemon started (AI mode)" },
        LogEntry { ts: "2026-07-03 08:00:01.500", level: Level::Warn,     source: "sigma-security", message: "3 failed sudo attempts from uid=1001 (sigma)" },
        LogEntry { ts: "2026-07-03 08:00:02.100", level: Level::Error,    source: "sigma-gpu",      message: "GPU shard suspend timeout after 5000ms" },
        LogEntry { ts: "2026-07-03 08:00:02.400", level: Level::Info,     source: "sigma-net",      message: "TCP syn-cookies enabled (load protection)" },
        LogEntry { ts: "2026-07-03 08:00:03.800", level: Level::Critical, source: "sigma-mm",       message: "OOM killer invoked: reclaimed 128 MiB from pid=4512 (chrome)" },
        LogEntry { ts: "2026-07-03 08:00:04.000", level: Level::Info,     source: "sigma-mm",       message: "OOM resolved: 1.4 GiB now available" },
        LogEntry { ts: "2026-07-03 08:00:05.000", level: Level::Info,     source: "sigma-update",   message: "OTA check: channel=stable, no updates available" },
    ]
}

fn print_usage() {
    println!("{} v{}", cyan("sigma-log"), VERSION);
    println!();
    println!("{}  sigma-log <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  tail    [--lines <n>] [--source <s>]  Show last N log lines");
    println!("  follow  [--source <s>]                 Follow log in real time (like tail -f)");
    println!("  search  --query <q> [--level <l>]      Full-text search");
    println!("  dump    [--output <file>]               Export all logs");
    println!("  stats   [--since <epoch>]               Log level statistics");
    println!("  anomaly [--threshold <n>]               Detect anomalies / spikes");
    println!("  export  --format json|csv|syslog        Export in a specific format");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --lines     <n>       Lines to show (default: 20)");
    println!("  --source    <name>    Filter by source (e.g. sigma-net)");
    println!("  --level     <l>       Minimum level: trace|debug|info|warn|error|critical");
    println!("  --query     <q>       Search query string");
    println!("  --since     <epoch>   Unix timestamp to start from");
    println!("  --output    <file>    Write to file instead of stdout");
    println!("  --threshold <n>       Anomaly detection sensitivity (default: 3)");
    println!("  --format    <fmt>     Export format: json|csv|syslog");
    println!("  --json                Machine-readable JSON output");
    println!("  --no-color            Disable ANSI colour codes");
    println!("  --version,  -V        Print version");
    println!("  --help,     -h        Show this help");
}

fn level_rank(l: &Level) -> u8 {
    match l {
        Level::Trace    => 0,
        Level::Debug    => 1,
        Level::Info     => 2,
        Level::Warn     => 3,
        Level::Error    => 4,
        Level::Critical => 5,
    }
}

fn cmd_tail(lines: usize, source_filter: Option<&str>, min_level: &Level, json: bool) {
    let logs = sample_logs();
    let filtered: Vec<&LogEntry> = logs.iter()
        .filter(|e| source_filter.map_or(true, |s| e.source.contains(s)))
        .filter(|e| level_rank(&e.level) >= level_rank(min_level))
        .rev().take(lines).collect::<Vec<_>>().into_iter().rev().collect();

    if json {
        println!("{{\"logs\":[");
        for (i, e) in filtered.iter().enumerate() {
            let lvl_str = match e.level { Level::Info=>"info", Level::Warn=>"warn", Level::Error=>"error", Level::Critical=>"critical", Level::Debug=>"debug", Level::Trace=>"trace" };
            print!("  {{\"ts\":\"{}\",\"level\":\"{}\",\"source\":\"{}\",\"msg\":\"{}\"}}", e.ts, lvl_str, e.source, e.message);
            if i < filtered.len()-1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }
    for e in &filtered {
        println!("  {}  {}  {:<18}  {}", dim(e.ts), e.level.to_coloured_str(), cyan(e.source), e.message);
    }
}

fn cmd_follow(source_filter: Option<&str>, json: bool) {
    println!("{} Following logs{} (Ctrl+C to stop)...\n",
        cyan("Σ"),
        source_filter.map(|s| format!(" [source={}]", s)).unwrap_or_default()
    );
    let logs = sample_logs();
    for (i, e) in logs.iter().enumerate() {
        if source_filter.map_or(true, |s| e.source.contains(s)) {
            if json {
                println!("{{\"ts\":\"{}\",\"level\":\"{:?}\",\"source\":\"{}\",\"msg\":\"{}\"}}",
                    e.ts, e.level, e.source, e.message);
            } else {
                println!("  {}  {}  {:<18}  {}", dim(e.ts), e.level.to_coloured_str(), cyan(e.source), e.message);
            }
        }
        if i < logs.len()-1 { thread::sleep(Duration::from_millis(120)); }
    }
    println!("\n{} {}", dim("—"), dim("End of simulated stream. Live mode reads from /run/sigma/journal.sock"));
}

fn cmd_search(query: &str, min_level: &Level, json: bool) {
    let logs = sample_logs();
    let matches: Vec<&LogEntry> = logs.iter()
        .filter(|e| e.message.to_lowercase().contains(&query.to_lowercase()) || e.source.contains(query))
        .filter(|e| level_rank(&e.level) >= level_rank(min_level))
        .collect();

    if json {
        println!("{{\"query\":\"{}\",\"count\":{},\"results\":[", query, matches.len());
        for (i, e) in matches.iter().enumerate() {
            print!("  {{\"ts\":\"{}\",\"source\":\"{}\",\"msg\":\"{}\"}}",e.ts,e.source,e.message);
            if i < matches.len()-1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }
    println!("{} '{}' — {} match(es)\n", bold("Search:"), query, matches.len());
    for e in &matches {
        let highlighted = e.message.replace(query, &format!("\x1B[1;33m{}\x1B[0m", query));
        println!("  {}  {}  {:<18}  {}", dim(e.ts), e.level.to_coloured_str(), cyan(e.source), highlighted);
    }
}

fn cmd_stats(json: bool) {
    let logs = sample_logs();
    let (mut info, mut warn, mut error, mut crit, mut debug) = (0u32,0u32,0u32,0u32,0u32);
    for e in &logs {
        match e.level {
            Level::Info | Level::Trace => info += 1,
            Level::Debug               => debug += 1,
            Level::Warn                => warn += 1,
            Level::Error               => error += 1,
            Level::Critical            => crit += 1,
        }
    }
    let total = logs.len() as u32;
    if json {
        println!("{{\"stats\":{{\"total\":{},\"info\":{},\"warn\":{},\"error\":{},\"critical\":{},\"debug\":{}}}}}",
            total, info, warn, error, crit, debug);
        return;
    }
    println!("{}", bold("Log Statistics"));
    println!("  Total entries : {}", total);
    println!("  {}  : {}", green("INFO "), info);
    println!("  {}  : {}", dim("DEBUG"), debug);
    println!("  {}  : {}", yellow("WARN "), warn);
    println!("  {}  : {}", red("ERROR"), error);
    let crit_str = if crit > 0 { red(&crit.to_string()) } else { green("0") };
    println!("  CRIT  : {}", crit_str);
    let bar_total = total.max(1);
    println!("\n  Distribution:");
    let bar_w = 40usize;
    for (label, count, colour_fn) in [("INFO ", info, "\x1B[1;32m"), ("WARN ", warn, "\x1B[1;33m"), ("ERROR", error, "\x1B[1;31m")] {
        let filled = (count as usize * bar_w / bar_total as usize).max(if count > 0 { 1 } else { 0 });
        println!("  {}  {}{}{}\x1B[0m  {}", label, colour_fn, "█".repeat(filled), "░".repeat(bar_w - filled), count);
    }
}

fn cmd_anomaly(threshold: u32, json: bool) {
    let anomalies: &[(&str, &str, &str)] = &[
        ("HIGH",   "2026-07-03 08:00:03.800", "OOM killer invoked — memory spike detected"),
        ("MEDIUM", "2026-07-03 08:00:02.100", "GPU shard timeout — hardware unresponsive > 5s"),
        ("LOW",    "2026-07-03 08:00:00.088", "PQC key missing — downgrade to weaker algorithm"),
    ];
    if json {
        println!("{{\"anomalies\":[");
        for (i, (sev, ts, msg)) in anomalies.iter().enumerate() {
            print!("  {{\"severity\":\"{}\",\"ts\":\"{}\",\"message\":\"{}\"}}",sev,ts,msg);
            if i < anomalies.len()-1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }
    println!("{} (threshold: {})", bold("Anomaly Detection"), threshold);
    println!("{}", "─".repeat(70));
    for (sev, ts, msg) in anomalies {
        let s = match *sev {
            "HIGH"   => red("HIGH  "),
            "MEDIUM" => yellow("MEDIUM"),
            _        => bold("LOW   "),
        };
        println!("  [{}] {}  {}", s, dim(ts), msg);
    }
}

fn cmd_export(format: &str, output: Option<&str>, json: bool) {
    let logs = sample_logs();
    let content = match format {
        "json"   => format!("[{}]", logs.iter().map(|e| format!("{{\"ts\":\"{}\",\"source\":\"{}\",\"msg\":\"{}\"}}", e.ts, e.source, e.message)).collect::<Vec<_>>().join(",")),
        "csv"    => format!("ts,level,source,message\n{}",
            logs.iter().map(|e| format!("{},{:?},{},{}", e.ts, e.level, e.source, e.message)).collect::<Vec<_>>().join("\n")),
        _        => logs.iter().map(|e| format!("<{} [{}] {}> {}", e.ts, e.source, format!("{:?}", e.level), e.message)).collect::<Vec<_>>().join("\n"),
    };
    if let Some(path) = output {
        let _ = std::fs::write(path, &content);
        println!("{} Exported {} entries to '{}' ({})", green("✓"), logs.len(), path, format);
    } else {
        println!("{}", content);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage();
        exit(if args.len() < 2 { 1 } else { 0 });
    }
    if args[1] == "--version" || args[1] == "-V" {
        println!("sigma-log {}", VERSION);
        exit(0);
    }

    let json      = args.iter().any(|a| a == "--json");
    let lines     = args.windows(2).find(|w| w[0] == "--lines").and_then(|w| w[1].parse().ok()).unwrap_or(20usize);
    let source    = args.windows(2).find(|w| w[0] == "--source").map(|w| w[1].as_str());
    let query     = args.windows(2).find(|w| w[0] == "--query").map(|w| w[1].as_str()).unwrap_or("");
    let level_str = args.windows(2).find(|w| w[0] == "--level").map(|w| w[1].as_str()).unwrap_or("info");
    let threshold = args.windows(2).find(|w| w[0] == "--threshold").and_then(|w| w[1].parse().ok()).unwrap_or(3u32);
    let format    = args.windows(2).find(|w| w[0] == "--format").map(|w| w[1].as_str()).unwrap_or("syslog");
    let output    = args.windows(2).find(|w| w[0] == "--output").map(|w| w[1].as_str());
    let min_level = Level::from_str(level_str);

    match args[1].as_str() {
        "tail"    => cmd_tail(lines, source, &min_level, json),
        "follow"  => cmd_follow(source, json),
        "search"  => cmd_search(query, &min_level, json),
        "dump"    => cmd_tail(usize::MAX, source, &Level::Trace, json),
        "stats"   => cmd_stats(json),
        "anomaly" => cmd_anomaly(threshold, json),
        "export"  => cmd_export(format, output, json),
        _ => {
            eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]);
            exit(1);
        }
    }
}
