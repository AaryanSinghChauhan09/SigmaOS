// SPDX-License-Identifier: GPL-2.0-or-later
//! sigma_top — SigmaOS process monitor (htop-style)
//!
//! Usage:
//!   sigma_top [--sort cpu|mem|pid|name] [--filter <str>] [--count <n>]
//!             [--interval <sec>] [--once] [--json]

use std::env;
use std::process::exit;
use std::thread;
use std::time::Duration;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)   -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str)  -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)    -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str) -> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)   -> String { format!("\x1B[1m{}\x1B[0m", s) }
fn dim(s: &str)    -> String { format!("\x1B[2m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}", cyan("sigma-top"), VERSION);
    println!();
    println!("{}  sigma-top [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --sort     <field>  Sort by: cpu|mem|pid|name|io (default: cpu)");
    println!("  --filter   <str>    Show only processes matching <str>");
    println!("  --count    <n>      Show top N processes (default: 20)");
    println!("  --interval <sec>    Refresh interval in seconds (default: 2)");
    println!("  --once              Single snapshot then exit");
    println!("  --json              Machine-readable JSON output");
    println!("  --version, -V       Print version");
    println!("  --help,    -h       Show this help");
}

#[derive(Debug, Clone)]
struct Process {
    pid:      u32,
    ppid:     u32,
    user:     &'static str,
    name:     &'static str,
    state:    &'static str,
    cpu_pct:  f32,
    mem_mib:  f32,
    threads:  u16,
    vsz_mib:  u32,
    cmd:      &'static str,
}

fn sample_processes() -> Vec<Process> {
    vec![
        Process { pid:1,    ppid:0,   user:"root",  name:"sigma-init",    state:"S", cpu_pct:0.1, mem_mib:1.2,  threads:1,  vsz_mib:8,    cmd:"sigma-init --boot"},
        Process { pid:42,   ppid:1,   user:"sigma", name:"sigma-agent",   state:"S", cpu_pct:1.2, mem_mib:8.4,  threads:4,  vsz_mib:64,   cmd:"sigma-agent --daemon"},
        Process { pid:88,   ppid:1,   user:"root",  name:"sigma-healthd", state:"S", cpu_pct:0.3, mem_mib:3.1,  threads:2,  vsz_mib:16,   cmd:"sigma-healthd"},
        Process { pid:120,  ppid:1,   user:"root",  name:"sigma-netd",    state:"S", cpu_pct:0.5, mem_mib:4.8,  threads:3,  vsz_mib:32,   cmd:"sigma-netd eth0"},
        Process { pid:200,  ppid:1,   user:"sigma", name:"sigma-sh",      state:"S", cpu_pct:0.1, mem_mib:2.1,  threads:1,  vsz_mib:12,   cmd:"sigma-sh"},
        Process { pid:512,  ppid:1,   user:"sigma", name:"zenith-wm",     state:"R", cpu_pct:3.4, mem_mib:22.0, threads:6,  vsz_mib:128,  cmd:"zenith-wm --display :0"},
        Process { pid:600,  ppid:512, user:"sigma", name:"sigma-browser", state:"R", cpu_pct:12.1,mem_mib:182.4,threads:12, vsz_mib:1024, cmd:"sigma-browser --type=main"},
        Process { pid:612,  ppid:600, user:"sigma", name:"sigma-browser", state:"S", cpu_pct:2.3, mem_mib:48.2, threads:4,  vsz_mib:256,  cmd:"sigma-browser --type=renderer"},
        Process { pid:700,  ppid:1,   user:"sigma", name:"sigma-notes",   state:"S", cpu_pct:0.2, mem_mib:14.1, threads:3,  vsz_mib:64,   cmd:"sigma-notes"},
        Process { pid:800,  ppid:1,   user:"root",  name:"sigma-vaultd",  state:"S", cpu_pct:0.1, mem_mib:6.3,  threads:2,  vsz_mib:24,   cmd:"sigma-vaultd --secure"},
        Process { pid:900,  ppid:1,   user:"root",  name:"sigma-updaterd",state:"S", cpu_pct:0.0, mem_mib:5.1,  threads:1,  vsz_mib:20,   cmd:"sigma-updaterd --channel=stable"},
        Process { pid:1000, ppid:512, user:"sigma", name:"sigma-term",    state:"R", cpu_pct:0.4, mem_mib:11.2, threads:2,  vsz_mib:48,   cmd:"sigma-term"},
        Process { pid:1100, ppid:1,   user:"root",  name:"sigma-audiod",  state:"S", cpu_pct:0.8, mem_mib:9.7,  threads:4,  vsz_mib:40,   cmd:"sigma-audiod --pipewire"},
        Process { pid:1200, ppid:1,   user:"root",  name:"sigma-gpu-hal", state:"S", cpu_pct:1.5, mem_mib:18.3, threads:3,  vsz_mib:80,   cmd:"sigma-gpu-hal nvidia"},
        Process { pid:9999, ppid:0,   user:"root",  name:"[sigma-ksoftirq]",state:"S",cpu_pct:0.6,mem_mib:0.0,  threads:1, vsz_mib:0,    cmd:"[kernel thread]"},
    ]
}

fn read_sys_info() -> (f64, u64, u64) {
    // Returns (cpu_pct_total, mem_used_mib, mem_total_mib)
    #[cfg(target_os = "linux")]
    {
        if let Ok(mem) = std::fs::read_to_string("/proc/meminfo") {
            let mut total = 0u64; let mut available = 0u64;
            for line in mem.lines() {
                let mut p = line.split_whitespace();
                match p.next() {
                    Some("MemTotal:")     => total     = p.next().and_then(|s| s.parse().ok()).unwrap_or(0),
                    Some("MemAvailable:") => available = p.next().and_then(|s| s.parse().ok()).unwrap_or(0),
                    _ => {}
                }
            }
            let used_mib = (total - available) / 1024;
            let total_mib = total / 1024;
            return (22.5, used_mib, total_mib); // cpu simulated
        }
    }
    (22.5, 6400, 32768)
}

fn cpu_bar(pct: f64, width: usize) -> String {
    let filled = (pct / 100.0 * width as f64) as usize;
    let colour = if pct > 80.0 { "\x1B[1;31m" } else if pct > 50.0 { "\x1B[1;33m" } else { "\x1B[1;32m" };
    format!("{}{}{}{}{}",
        "[", colour, "█".repeat(filled), "\x1B[0m░".repeat(width-filled), "]"
    )
}

fn render(procs: &[Process], sort_by: &str, filter: Option<&str>, count: usize, json: bool) {
    let (cpu_total, mem_used, mem_total) = read_sys_info();

    let mut sorted = procs.to_vec();
    match sort_by {
        "mem"  => sorted.sort_by(|a,b| b.mem_mib.partial_cmp(&a.mem_mib).unwrap()),
        "pid"  => sorted.sort_by_key(|p| p.pid),
        "name" => sorted.sort_by_key(|p| p.name),
        _      => sorted.sort_by(|a,b| b.cpu_pct.partial_cmp(&a.cpu_pct).unwrap()),
    }

    let visible: Vec<&Process> = sorted.iter()
        .filter(|p| filter.map_or(true, |f| p.name.contains(f) || p.cmd.contains(f) || p.user.contains(f)))
        .take(count)
        .collect();

    if json {
        let cpu_sum: f32 = visible.iter().map(|p| p.cpu_pct).sum();
        let mem_sum: f32 = visible.iter().map(|p| p.mem_mib).sum();
        println!("{{\"system\":{{\"cpu_pct\":{:.1},\"mem_used_mib\":{},\"mem_total_mib\":{}}},\"processes\":[",
            cpu_total, mem_used, mem_total);
        for (i, p) in visible.iter().enumerate() {
            print!("  {{\"pid\":{},\"name\":\"{}\",\"user\":\"{}\",\"state\":\"{}\",\"cpu\":{:.1},\"mem_mib\":{:.1}}}",
                p.pid, p.name, p.user, p.state, p.cpu_pct, p.mem_mib);
            if i < visible.len()-1 { print!(","); }
            println!();
        }
        println!("]}}");
        return;
    }

    // System header
    let mem_pct = if mem_total > 0 { mem_used * 100 / mem_total } else { 0 };
    println!("{}  v{}    Sort: {}    Filter: {}", cyan("Σ sigma-top"), VERSION, sort_by, filter.unwrap_or("none"));
    println!("  CPU  {:3.1}%  {}", cpu_total, cpu_bar(cpu_total, 30));
    println!("  MEM  {}%  {} / {} MiB   {}", mem_pct, mem_used, mem_total, cpu_bar(mem_pct as f64, 30));
    println!();
    println!("  {:<6}  {:<6}  {:<10}  {:<18}  {:>5}  {:>8}  {:>7}  {}",
        "PID", "PPID", "USER", "NAME", "CPU%", "MEM MiB", "THREADS", "COMMAND");
    println!("  {}", "─".repeat(88));

    for p in &visible {
        let cpu_col = if p.cpu_pct > 50.0 { red(&format!("{:>5.1}", p.cpu_pct)) }
                      else if p.cpu_pct > 20.0 { yellow(&format!("{:>5.1}", p.cpu_pct)) }
                      else { format!("{:>5.1}", p.cpu_pct) };
        let mem_col = if p.mem_mib > 100.0 { yellow(&format!("{:>8.1}", p.mem_mib)) }
                      else { format!("{:>8.1}", p.mem_mib) };
        let name_col = if p.state == "R" { cyan(p.name) } else { p.name.to_string() };
        println!("  {:<6}  {:<6}  {:<10}  {:<26}  {}  {}  {:>7}  {}",
            p.pid, p.ppid, p.user, name_col, cpu_col, mem_col, p.threads, dim(p.cmd));
    }
    println!("\n  {} processes shown", visible.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h") { print_usage(); exit(0); }
    if args.iter().any(|a| a == "--version" || a == "-V") { println!("sigma-top {}", VERSION); exit(0); }

    let json     = args.iter().any(|a| a == "--json");
    let once     = args.iter().any(|a| a == "--once");
    let sort_by  = args.windows(2).find(|w| w[0] == "--sort").map(|w| w[1].as_str()).unwrap_or("cpu");
    let filter   = args.windows(2).find(|w| w[0] == "--filter").map(|w| w[1].as_str());
    let count    = args.windows(2).find(|w| w[0] == "--count").and_then(|w| w[1].parse().ok()).unwrap_or(20usize);
    let interval = args.windows(2).find(|w| w[0] == "--interval").and_then(|w| w[1].parse().ok()).unwrap_or(2u64);

    let procs = sample_processes();

    loop {
        if !json && !once { print!("\x1B[2J\x1B[1;1H"); }
        render(&procs, sort_by, filter, count, json);
        if once { break; }
        thread::sleep(Duration::from_secs(interval));
    }
}
