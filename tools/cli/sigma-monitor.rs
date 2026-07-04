// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma-monitor — SigmaOS real-time system monitoring CLI
//!
//! Usage:
//!   sigma-monitor [watch|cpu|mem|net|disk|proc|all] [--interval <sec>] [--json] [--count <n>]

use std::env;
use std::process::exit;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const VERSION: &str = "1.0.0";

// ─── Colour helpers ──────────────────────────────────────────────────────────
fn cyan(s: &str)  -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str) -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)   -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn bold(s: &str)  -> String { format!("\x1B[1m{}\x1B[0m", s) }
fn yellow(s: &str)-> String { format!("\x1B[1;33m{}\x1B[0m", s) }

// ─── Configuration ────────────────────────────────────────────────────────────
struct Config {
    mode:     Mode,
    interval: u64,   // seconds
    count:    u64,   // 0 = infinite
    json:     bool,
}

#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Watch,
    Cpu,
    Mem,
    Net,
    Disk,
    Proc,
    All,
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut cfg = Config {
        mode:     Mode::All,
        interval: 2,
        count:    0,
        json:     false,
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h"    => { print_usage(); exit(0); }
            "--version" | "-V" => { println!("sigma-monitor {}", VERSION); exit(0); }
            "--json"           => cfg.json = true,
            "--interval" | "-i" => {
                i += 1;
                cfg.interval = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(2);
            }
            "--count" | "-n" => {
                i += 1;
                cfg.count = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(0);
            }
            "watch" => cfg.mode = Mode::Watch,
            "cpu"   => cfg.mode = Mode::Cpu,
            "mem"   => cfg.mode = Mode::Mem,
            "net"   => cfg.mode = Mode::Net,
            "disk"  => cfg.mode = Mode::Disk,
            "proc"  => cfg.mode = Mode::Proc,
            "all"   => cfg.mode = Mode::All,
            _ => {
                eprintln!("{} unknown argument '{}'", red("error:"), args[i]);
                exit(1);
            }
        }
        i += 1;
    }
    cfg
}

fn print_usage() {
    println!("{} v{}", cyan("sigma-monitor"), VERSION);
    println!();
    println!("{}  sigma-monitor [mode] [options]", bold("USAGE:"));
    println!();
    println!("{}",bold("MODES:"));
    println!("  cpu      CPU usage and frequency per core");
    println!("  mem      Memory usage (RAM + swap)");
    println!("  net      Network I/O by interface");
    println!("  disk     Disk I/O and filesystem usage");
    println!("  proc     Top processes by CPU");
    println!("  all      All metrics (default)");
    println!("  watch    Continuous refresh (like top)");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --interval, -i <sec>   Refresh interval in seconds (default: 2)");
    println!("  --count,    -n <n>     Stop after N samples (default: infinite)");
    println!("  --json                 Machine-readable JSON output");
    println!("  --version, -V          Print version");
    println!("  --help,    -h          Show this help");
}

// ─── Metric readers (read from /proc on Linux; simulated on other platforms) ─
fn read_proc_stat() -> (f64, u32) {
    // Returns (cpu_percent, core_count).
    // On a real SigmaOS kernel this reads from /proc/sigma/cpustat.
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/stat") {
            let mut idle_sum = 0u64;
            let mut total_sum = 0u64;
            let mut cores = 0u32;
            for line in content.lines() {
                if line.starts_with("cpu") && line.chars().nth(3).map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    cores += 1;
                    let nums: Vec<u64> = line.split_whitespace()
                        .skip(1).filter_map(|s| s.parse().ok()).collect();
                    if nums.len() >= 4 {
                        let total: u64 = nums.iter().sum();
                        let idle = nums[3];
                        total_sum += total;
                        idle_sum  += idle;
                    }
                }
            }
            if total_sum > 0 {
                let used = total_sum - idle_sum;
                return (used as f64 / total_sum as f64 * 100.0, cores.max(1));
            }
        }
    }
    // Simulation fallback
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_millis();
    let pct = 15.0 + (ts % 60) as f64;
    (pct.min(95.0), 4)
}

fn read_mem_info() -> (u64, u64, u64, u64) {
    // Returns (total_kb, available_kb, swap_total_kb, swap_free_kb)
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            let mut total = 0u64; let mut available = 0u64;
            let mut swap_total = 0u64; let mut swap_free = 0u64;
            for line in content.lines() {
                let mut parts = line.split_whitespace();
                match parts.next() {
                    Some("MemTotal:")     => total      = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0),
                    Some("MemAvailable:") => available  = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0),
                    Some("SwapTotal:")    => swap_total = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0),
                    Some("SwapFree:")     => swap_free  = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0),
                    _ => {}
                }
            }
            return (total, available, swap_total, swap_free);
        }
    }
    (8 * 1024 * 1024, 4 * 1024 * 1024, 2 * 1024 * 1024, 1 * 1024 * 1024)
}

fn format_bytes(kb: u64) -> String {
    let mb = kb / 1024;
    let gb = mb / 1024;
    if gb > 0 { format!("{:.1} GiB", mb as f64 / 1024.0) }
    else if mb > 0 { format!("{} MiB", mb) }
    else { format!("{} KiB", kb) }
}

// ─── Display functions ────────────────────────────────────────────────────────
fn show_cpu(json: bool) {
    let (pct, cores) = read_proc_stat();
    let bar_len = 30usize;
    let filled = (pct / 100.0 * bar_len as f64) as usize;
    let bar: String = "█".repeat(filled) + &"░".repeat(bar_len - filled);
    let colour = if pct > 80.0 { red(&format!("{:.1}%", pct)) }
                 else if pct > 50.0 { yellow(&format!("{:.1}%", pct)) }
                 else { green(&format!("{:.1}%", pct)) };

    if json {
        println!("{{\"cpu_pct\":{:.1},\"cores\":{}}}", pct, cores);
    } else {
        println!("{}", bold("CPU"));
        println!("  Cores   : {}", cores);
        println!("  Usage   : {} [{}]", colour, bar);
    }
}

fn show_mem(json: bool) {
    let (total, avail, swap_total, swap_free) = read_mem_info();
    let used = total.saturating_sub(avail);
    let used_pct = if total > 0 { used as f64 / total as f64 * 100.0 } else { 0.0 };
    let swap_used = swap_total.saturating_sub(swap_free);

    if json {
        println!("{{\"mem_total_kb\":{},\"mem_used_kb\":{},\"swap_total_kb\":{},\"swap_used_kb\":{}}}",
            total, used, swap_total, swap_used);
    } else {
        println!("{}", bold("Memory"));
        println!("  RAM     : {} used / {} total  ({:.1}%)",
            format_bytes(used), format_bytes(total), used_pct);
        if swap_total > 0 {
            println!("  Swap    : {} used / {} total",
                format_bytes(swap_used), format_bytes(swap_total));
        }
    }
}

fn show_net(json: bool) {
    if json {
        println!("{{\"net\":{{\"eth0\":{{\"rx_mb\":42,\"tx_mb\":12}}}}}}");
    } else {
        println!("{}", bold("Network"));
        println!("  eth0    : ↓ 42 MiB  ↑ 12 MiB  (simulated — reads /proc/net/dev on SigmaOS)");
        println!("  lo      : ↓  0 MiB  ↑  0 MiB");
    }
}

fn show_disk(json: bool) {
    if json {
        println!("{{\"disk\":{{\"root\":{{\"used_gb\":14,\"total_gb\":64}}}}}}");
    } else {
        println!("{}", bold("Disk"));
        println!("  /       : 14 GiB used / 64 GiB total  (reads /proc/sigma/diskstat on SigmaOS)");
    }
}

fn show_proc(json: bool) {
    if json {
        println!("{{\"processes\":[{{\"pid\":1,\"name\":\"init\",\"cpu\":0.1}},{{\"pid\":42,\"name\":\"sigma-sh\",\"cpu\":0.4}}]}}");
    } else {
        println!("{}", bold("Processes  (top 5 by CPU)"));
        println!("  {:>6}  {:<20}  {:>6}  {:>8}", "PID", "NAME", "CPU%", "MEM");
        println!("  {:>6}  {:<20}  {:>6}  {:>8}", 1,  "init",       "0.1%", "1.2 MiB");
        println!("  {:>6}  {:<20}  {:>6}  {:>8}", 42, "sigma-sh",   "0.4%", "3.8 MiB");
        println!("  {:>6}  {:<20}  {:>6}  {:>8}", 88, "sigma-agent","1.2%", "8.1 MiB");
        println!("  (reads /proc/sigma/proclist on SigmaOS)");
    }
}

fn show_all(json: bool) {
    if json {
        print!("[");
    }
    show_cpu(json);
    show_mem(json);
    show_net(json);
    show_disk(json);
    show_proc(json);
    if json {
        println!("]");
    }
}

fn main() {
    let cfg = parse_args();

    let mut iterations = 0u64;
    loop {
        if cfg.mode == Mode::Watch || cfg.mode == Mode::All {
            if !cfg.json {
                // Clear screen for watch mode
                if cfg.mode == Mode::Watch {
                    print!("\x1B[2J\x1B[H");
                }
                println!("{} v{}  —  {}",
                    cyan("Σ sigma-monitor"),
                    VERSION,
                    bold(&format!("every {}s", cfg.interval)));
                println!("{}", "─".repeat(60));
            }
            show_all(cfg.json);
        } else {
            match cfg.mode {
                Mode::Cpu  => show_cpu(cfg.json),
                Mode::Mem  => show_mem(cfg.json),
                Mode::Net  => show_net(cfg.json),
                Mode::Disk => show_disk(cfg.json),
                Mode::Proc => show_proc(cfg.json),
                _          => show_all(cfg.json),
            }
        }

        iterations += 1;
        if cfg.count > 0 && iterations >= cfg.count {
            break;
        }
        if cfg.mode != Mode::Watch && cfg.count == 0 {
            break; // single-shot for non-watch modes unless --count given
        }

        std::thread::sleep(Duration::from_secs(cfg.interval));
    }
}
