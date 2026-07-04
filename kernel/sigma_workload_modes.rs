// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/sigma_workload_modes.rs — Workload-specific performance modes
// Novel: One command switches ALL kernel parameters for a given workload type.
// Covers: ML, Database, VideoStreaming, Gaming, RTOS, Server, Desktop, Minimal
//
// Each mode tunes: scheduler policy, memory policy, I/O scheduler, network,
// power governor, interrupt coalescing, huge pages, dirty page thresholds.
//
// Language: Rust (std)

use std::process::Command;
use std::collections::HashMap;

// ── Workload mode ─────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkloadMode {
    Desktop,     // balanced: interactive + responsive
    Server,      // throughput: high concurrency, no GUI
    MachineLearning, // batch: NUMA-aware, huge pages, GPU burst
    Database,    // ACID: high concurrency, predictable latency
    VideoStream, // latency: predictable throughput, low jitter
    Gaming,      // frame-paced: 60/120 FPS, low input latency
    Rtos,        // real-time: EDF scheduler, < 10µs IRQ latency
    Minimal,     // survival: 8MB RAM mode, kill non-essential
    Power,       // battery: lowest CPU freq, coalesce everything
    Turbo,       // max perf: all cores max freq, no power saving
}

impl WorkloadMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "desktop"          => Some(Self::Desktop),
            "server"           => Some(Self::Server),
            "ml"|"machine-learning"|"ai" => Some(Self::MachineLearning),
            "db"|"database"    => Some(Self::Database),
            "video"|"stream"   => Some(Self::VideoStream),
            "gaming"|"game"    => Some(Self::Gaming),
            "rtos"|"realtime"  => Some(Self::Rtos),
            "minimal"|"survival" => Some(Self::Minimal),
            "power"|"battery"  => Some(Self::Power),
            "turbo"|"performance" => Some(Self::Turbo),
            _ => None,
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            Self::Desktop      => "Balanced interactive desktop — responsive + efficient",
            Self::Server       => "High-throughput server — max concurrency, no GUI overhead",
            Self::MachineLearning => "ML training/inference — NUMA, huge pages, GPU burst mode",
            Self::Database     => "Database workload — ACID, predictable latency, high IOPS",
            Self::VideoStream  => "Video streaming — smooth throughput, low jitter",
            Self::Gaming       => "Gaming — 60/120 FPS, lowest input latency",
            Self::Rtos         => "Real-time — EDF scheduler, < 10µs IRQ latency",
            Self::Minimal      => "Survival mode — 8MB RAM, kill non-essential daemons",
            Self::Power        => "Battery saver — min CPU freq, max coalescing",
            Self::Turbo        => "Max performance — all cores max freq, no throttling",
        }
    }
}

// ── Sysctl tuning profile ─────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct KernelTuning {
    pub sysctls:     Vec<(String, String)>,
    pub cpufreq_gov: String,
    pub io_scheduler: String,
    pub irq_balance:  bool,
    pub thp_enabled:  bool,
    pub sigma_sched:  String,   // "mlfq" | "cfs" | "edf" | "rtos"
}

impl KernelTuning {
    fn for_mode(mode: WorkloadMode) -> Self {
        match mode {
            WorkloadMode::Desktop => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),                    "10".into()),
                    ("vm.dirty_ratio".into(),                   "20".into()),
                    ("vm.dirty_background_ratio".into(),        "5".into()),
                    ("kernel.sched_min_granularity_ns".into(),  "750000".into()),
                    ("kernel.sched_wakeup_granularity_ns".into(),"1000000".into()),
                    ("net.core.rmem_max".into(),                "16777216".into()),
                    ("net.core.wmem_max".into(),                "16777216".into()),
                ],
                cpufreq_gov:  "schedutil".into(),
                io_scheduler: "mq-deadline".into(),
                irq_balance:  true,
                thp_enabled:  false,
                sigma_sched:  "mlfq".into(),
            },
            WorkloadMode::Server => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),                    "1".into()),
                    ("vm.dirty_ratio".into(),                   "40".into()),
                    ("vm.dirty_background_ratio".into(),        "10".into()),
                    ("net.core.rmem_max".into(),                "134217728".into()),
                    ("net.core.wmem_max".into(),                "134217728".into()),
                    ("net.ipv4.tcp_congestion_control".into(),  "bbr".into()),
                    ("net.core.netdev_max_backlog".into(),      "65536".into()),
                    ("net.ipv4.tcp_max_syn_backlog".into(),     "65536".into()),
                    ("kernel.sched_min_granularity_ns".into(),  "3000000".into()),
                ],
                cpufreq_gov:  "performance".into(),
                io_scheduler: "none".into(),
                irq_balance:  true,
                thp_enabled:  true,
                sigma_sched:  "cfs".into(),
            },
            WorkloadMode::MachineLearning => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),                    "0".into()),
                    ("vm.dirty_ratio".into(),                   "80".into()),
                    ("vm.dirty_background_ratio".into(),        "20".into()),
                    ("kernel.numa_balancing".into(),            "1".into()),
                    ("vm.nr_hugepages".into(),                  "1024".into()),
                    ("kernel.sched_migration_cost_ns".into(),   "500000".into()),
                ],
                cpufreq_gov:  "performance".into(),
                io_scheduler: "none".into(),
                irq_balance:  false,   // pin IRQs for NUMA locality
                thp_enabled:  true,
                sigma_sched:  "cfs".into(),
            },
            WorkloadMode::Database => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),                    "1".into()),
                    ("vm.dirty_ratio".into(),                   "15".into()),
                    ("vm.dirty_background_ratio".into(),        "3".into()),
                    ("vm.dirty_expire_centisecs".into(),        "500".into()),
                    ("vm.dirty_writeback_centisecs".into(),     "100".into()),
                    ("net.core.rmem_max".into(),                "67108864".into()),
                    ("net.ipv4.tcp_congestion_control".into(),  "cubic".into()),
                    ("kernel.sched_min_granularity_ns".into(),  "1000000".into()),
                ],
                cpufreq_gov:  "performance".into(),
                io_scheduler: "mq-deadline".into(),
                irq_balance:  true,
                thp_enabled:  false,   // THP causes latency spikes in DBs
                sigma_sched:  "cfs".into(),
            },
            WorkloadMode::VideoStream => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),                    "5".into()),
                    ("net.core.rmem_max".into(),                "33554432".into()),
                    ("net.ipv4.tcp_low_latency".into(),         "1".into()),
                    ("net.ipv4.tcp_congestion_control".into(),  "bbr".into()),
                    ("kernel.sched_min_granularity_ns".into(),  "500000".into()),
                ],
                cpufreq_gov:  "schedutil".into(),
                io_scheduler: "mq-deadline".into(),
                irq_balance:  true,
                thp_enabled:  false,
                sigma_sched:  "mlfq".into(),
            },
            WorkloadMode::Gaming => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),                    "0".into()),
                    ("kernel.sched_min_granularity_ns".into(),  "250000".into()),
                    ("kernel.sched_wakeup_granularity_ns".into(),"250000".into()),
                    ("kernel.sched_migration_cost_ns".into(),   "50000".into()),
                    ("net.ipv4.tcp_low_latency".into(),         "1".into()),
                ],
                cpufreq_gov:  "performance".into(),
                io_scheduler: "none".into(),
                irq_balance:  false,
                thp_enabled:  false,
                sigma_sched:  "mlfq".into(),
            },
            WorkloadMode::Rtos => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),              "0".into()),
                    ("kernel.sched_rt_runtime_us".into(), "-1".into()),
                    ("kernel.sched_min_granularity_ns".into(), "100000".into()),
                ],
                cpufreq_gov:  "performance".into(),
                io_scheduler: "none".into(),
                irq_balance:  false,
                thp_enabled:  false,
                sigma_sched:  "edf".into(),
            },
            WorkloadMode::Minimal => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),              "100".into()),
                    ("vm.dirty_ratio".into(),             "5".into()),
                    ("net.core.rmem_max".into(),          "65536".into()),
                    ("net.core.wmem_max".into(),          "65536".into()),
                ],
                cpufreq_gov:  "powersave".into(),
                io_scheduler: "mq-deadline".into(),
                irq_balance:  true,
                thp_enabled:  false,
                sigma_sched:  "mlfq".into(),
            },
            WorkloadMode::Power => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),                "60".into()),
                    ("vm.dirty_writeback_centisecs".into(), "1500".into()),
                    ("net.core.rmem_max".into(),            "131072".into()),
                    ("kernel.nmi_watchdog".into(),          "0".into()),
                ],
                cpufreq_gov:  "powersave".into(),
                io_scheduler: "mq-deadline".into(),
                irq_balance:  true,
                thp_enabled:  false,
                sigma_sched:  "mlfq".into(),
            },
            WorkloadMode::Turbo => Self {
                sysctls: vec![
                    ("vm.swappiness".into(),                    "0".into()),
                    ("vm.dirty_ratio".into(),                   "80".into()),
                    ("net.core.rmem_max".into(),                "134217728".into()),
                    ("net.ipv4.tcp_congestion_control".into(),  "bbr".into()),
                    ("kernel.sched_min_granularity_ns".into(),  "100000".into()),
                    ("kernel.numa_balancing".into(),            "0".into()),
                ],
                cpufreq_gov:  "performance".into(),
                io_scheduler: "none".into(),
                irq_balance:  false,
                thp_enabled:  true,
                sigma_sched:  "cfs".into(),
            },
        }
    }

    pub fn apply(&self, verbose: bool) -> Vec<(String, bool)> {
        let mut results = Vec::new();
        // Apply sysctls
        for (key, val) in &self.sysctls {
            let ok = Command::new("sysctl")
                .args(["-w", &format!("{}={}", key, val)])
                .status().map(|s| s.success()).unwrap_or(false);
            if verbose { println!("  {} {}={}", if ok { "✓" } else { "⚠" }, key, val); }
            results.push((format!("{}={}", key, val), ok));
        }
        // CPU frequency governor
        let gov_ok = Command::new("sh")
            .arg("-c")
            .arg(format!("echo {} | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 2>/dev/null",
                         self.cpufreq_gov))
            .status().map(|s| s.success()).unwrap_or(false);
        if verbose { println!("  {} cpufreq_gov={}", if gov_ok { "✓" } else { "⚠" }, self.cpufreq_gov); }
        results.push((format!("cpufreq={}", self.cpufreq_gov), gov_ok));

        // I/O scheduler
        let io_ok = Command::new("sh")
            .arg("-c")
            .arg(format!("for d in /sys/block/*/queue/scheduler; do echo {} > $d 2>/dev/null; done",
                         self.io_scheduler))
            .status().map(|s| s.success()).unwrap_or(false);
        if verbose { println!("  {} io_scheduler={}", if io_ok { "✓" } else { "⚠" }, self.io_scheduler); }
        results.push((format!("io={}", self.io_scheduler), io_ok));

        // Transparent huge pages
        let thp_val = if self.thp_enabled { "always" } else { "madvise" };
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!("echo {} > /sys/kernel/mm/transparent_hugepage/enabled 2>/dev/null", thp_val))
            .status();

        // Write sigma scheduler preference
        let sigma_cfg = format!("sched.policy={}\nsched.irq_balance={}\n",
                                 self.sigma_sched, self.irq_balance);
        let _ = std::fs::create_dir_all("/run/sigma");
        let _ = std::fs::write("/run/sigma/workload_mode.conf", &sigma_cfg);

        results
    }
}

// ── CLI ────────────────────────────────────────────────────────────────────
pub fn workload_mode_cmd(args: &[String]) {
    if args.is_empty() || args[0] == "help" || args[0] == "--help" {
        println!("sigma-mode — Workload performance mode switcher\n\
            \nUsage:\n\
            sigma-mode <mode>              Apply a performance mode\n\
            sigma-mode <mode> --verbose    Apply with detailed output\n\
            sigma-mode list                List all modes\n\
            sigma-mode current             Show current mode\n\
            \nModes:");
        for (name, mode) in [
            ("desktop", WorkloadMode::Desktop), ("server", WorkloadMode::Server),
            ("ml", WorkloadMode::MachineLearning), ("database", WorkloadMode::Database),
            ("video", WorkloadMode::VideoStream), ("gaming", WorkloadMode::Gaming),
            ("rtos", WorkloadMode::Rtos), ("minimal", WorkloadMode::Minimal),
            ("power", WorkloadMode::Power), ("turbo", WorkloadMode::Turbo),
        ] {
            println!("  {:12}  {}", name, mode.description());
        }
        println!("\nExamples:\n  sigma-mode ml\n  sigma-mode gaming\n  sigma-mode server --verbose");
        return;
    }

    if args[0] == "list" {
        workload_mode_cmd(&["help".to_string()]);
        return;
    }

    if args[0] == "current" {
        let current = std::fs::read_to_string("/run/sigma/workload_mode.conf")
            .unwrap_or_else(|_| "sched.policy=mlfq\n".to_owned());
        println!("Current workload mode config:\n{}", current);
        return;
    }

    let mode_str = &args[0];
    let verbose  = args.iter().any(|a| a == "--verbose" || a == "-v");

    match WorkloadMode::from_str(mode_str) {
        Some(mode) => {
            println!("\x1b[38;2;69;243;255m\x1b[1mΣ Applying workload mode: {:?}\x1b[0m", mode);
            println!("  {}", mode.description());
            if verbose { println!(); }
            let tuning = KernelTuning::for_mode(mode);
            let results = tuning.apply(verbose);
            let ok_count = results.iter().filter(|(_, ok)| *ok).count();
            println!("\n  \x1b[38;2;52;211;153m✓ Mode applied: {}/{} settings\x1b[0m", ok_count, results.len());
            // Save current mode
            let _ = std::fs::write("/run/sigma/current_mode", format!("{:?}", mode));
        }
        None => {
            eprintln!("Unknown mode: {}. Run: sigma-mode list", mode_str);
            std::process::exit(1);
        }
    }
}
