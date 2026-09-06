//! SigmaOS procfs — Sovereign /proc Virtual Filesystem
//!
//! Provides a Linux-compatible /proc interface for exposing
//! kernel state to userspace. Fully sovereign — no libc or
//! platform filesystem API required.
//!
//! # Structure
//! ```
//! /proc/
//! ├── cpuinfo          — CPU topology and features
//! ├── meminfo          — Memory usage statistics
//! ├── uptime           — System uptime in seconds
//! ├── loadavg          — 1/5/15-minute load averages
//! ├── stat             — CPU and I/O statistics
//! ├── mounts           — Mounted filesystems
//! ├── interrupts        — IRQ counts per CPU
//! ├── net/dev          — Network interface statistics
//! ├── sys/kernel/      — Kernel tunables (sysctl)
//! └── <pid>/
//!     ├── status       — Process status
//!     ├── stat         — Process statistics
//!     ├── cmdline      — Command line
//!     ├── environ      — Environment variables
//!     ├── maps         — Memory mappings
//!     ├── fd/          — File descriptors
//!     └── statm        — Memory usage
//! ```

#![allow(dead_code)]

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

// ============================================================
// Process State
// ============================================================

/// Process state as reported in /proc/<pid>/status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcProcessState {
    /// Running (R)
    Running,
    /// Sleeping in interruptible wait (S)
    Sleeping,
    /// Waiting in uninterruptible disk sleep (D)
    DiskSleep,
    /// Zombie (Z)
    Zombie,
    /// Stopped (T)
    Stopped,
    /// Tracing stop (t)
    TracingStop,
    /// Dead (X)
    Dead,
}

impl ProcProcessState {
    pub fn as_char(self) -> char {
        match self {
            Self::Running => 'R', Self::Sleeping => 'S', Self::DiskSleep => 'D',
            Self::Zombie => 'Z', Self::Stopped => 'T', Self::TracingStop => 't',
            Self::Dead => 'X',
        }
    }
}

// ============================================================
// Process Info
// ============================================================

/// Snapshot of a process's state for /proc/<pid>/ generation.
#[derive(Debug, Clone)]
pub struct ProcProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub state: ProcProcessState,
    pub uid: u32,
    pub gid: u32,
    pub vm_rss_kb: u64,
    pub vm_size_kb: u64,
    pub vm_peak_kb: u64,
    pub threads: u32,
    pub utime_ticks: u64,
    pub stime_ticks: u64,
    pub cmdline: String,
    pub start_time_ticks: u64,
    pub nice: i32,
    pub priority: i32,
    pub open_fds: Vec<u32>,
    pub mem_maps: Vec<ProcMemMap>,
}

/// A memory mapping entry for /proc/<pid>/maps.
#[derive(Debug, Clone)]
pub struct ProcMemMap {
    pub start: u64,
    pub end: u64,
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub shared: bool,
    pub offset: u64,
    pub name: String,
}

impl ProcMemMap {
    fn perms_str(&self) -> String {
        format!("{}{}{}{}",
            if self.readable { 'r' } else { '-' },
            if self.writable { 'w' } else { '-' },
            if self.executable { 'x' } else { '-' },
            if self.shared { 's' } else { 'p' },
        )
    }
}

// ============================================================
// CPU Info
// ============================================================

/// CPU information for /proc/cpuinfo generation.
#[derive(Debug, Clone)]
pub struct ProcCpuInfo {
    pub processor_id: u32,
    pub vendor_id: String,
    pub model_name: String,
    pub cpu_family: u32,
    pub model: u32,
    pub stepping: u32,
    pub mhz: f64,
    pub cache_size_kb: u32,
    pub cpu_cores: u32,
    pub siblings: u32,
    pub flags: Vec<String>,
}

impl Default for ProcCpuInfo {
    fn default() -> Self {
        Self {
            processor_id: 0,
            vendor_id: "SigmaOS".into(),
            model_name: "SigmaOS Sovereign Processor".into(),
            cpu_family: 6, model: 85, stepping: 7,
            mhz: 3600.0, cache_size_kb: 8192, cpu_cores: 4, siblings: 8,
            flags: vec![
                "fpu".into(), "vme".into(), "de".into(), "pse".into(),
                "tsc".into(), "msr".into(), "pae".into(), "mce".into(),
                "cx8".into(), "apic".into(), "sep".into(), "mtrr".into(),
                "pge".into(), "mca".into(), "cmov".into(), "pat".into(),
                "pse36".into(), "mmx".into(), "sse".into(), "sse2".into(),
                "sse4_2".into(), "avx".into(), "avx2".into(), "aes".into(),
            ],
        }
    }
}

// ============================================================
// Memory Info
// ============================================================

/// System memory statistics for /proc/meminfo.
#[derive(Debug, Clone, Default)]
pub struct ProcMemInfo {
    pub mem_total_kb: u64,
    pub mem_free_kb: u64,
    pub mem_available_kb: u64,
    pub buffers_kb: u64,
    pub cached_kb: u64,
    pub swap_cached_kb: u64,
    pub active_kb: u64,
    pub inactive_kb: u64,
    pub swap_total_kb: u64,
    pub swap_free_kb: u64,
    pub dirty_kb: u64,
    pub writeback_kb: u64,
    pub slab_kb: u64,
    pub page_tables_kb: u64,
    pub vmalloc_total_kb: u64,
    pub vmalloc_used_kb: u64,
    pub hugepages_total: u64,
    pub hugepages_free: u64,
    pub hugepage_size_kb: u64,
}

// ============================================================
// Kernel Stats
// ============================================================

/// System-wide statistics for /proc/stat.
#[derive(Debug, Clone, Default)]
pub struct ProcStat {
    /// CPU times: [user, nice, system, idle, iowait, irq, softirq, steal]
    pub cpu_times: [u64; 8],
    /// Per-CPU times
    pub per_cpu: Vec<[u64; 8]>,
    /// Total interrupts since boot
    pub intr_total: u64,
    /// Context switches since boot
    pub ctxt: u64,
    /// Boot time (Unix timestamp)
    pub btime: u64,
    /// Processes created since boot
    pub processes: u64,
    /// Currently running processes
    pub procs_running: u32,
    /// Processes blocked on I/O
    pub procs_blocked: u32,
}

// ============================================================
// ProcfsMount — The /proc Filesystem
// ============================================================

/// The SigmaOS /proc virtual filesystem.
///
/// Generates file contents dynamically from kernel data structures.
/// No physical storage — all content produced on read.
pub struct ProcfsMount {
    /// System uptime in seconds
    uptime_secs: f64,
    /// CPU information (one entry per logical CPU)
    cpuinfo: Vec<ProcCpuInfo>,
    /// Memory information
    meminfo: ProcMemInfo,
    /// System statistics
    stat: ProcStat,
    /// Process table
    processes: BTreeMap<u32, ProcProcessInfo>,
    /// Mount table entries
    mounts: Vec<ProcMount>,
    /// Kernel version string
    kernel_version: String,
    /// Hostname
    hostname: String,
    /// Load averages (1min, 5min, 15min)
    loadavg: (f64, f64, f64),
    /// Last PID assigned
    last_pid: u32,
}

/// A mount table entry for /proc/mounts.
#[derive(Debug, Clone)]
pub struct ProcMount {
    pub device: String,
    pub mount_point: String,
    pub fs_type: String,
    pub options: String,
}

impl ProcfsMount {
    /// Create a new procfs with default system info.
    pub fn new(hostname: &str, kernel_version: &str) -> Self {
        let mut mounts = Vec::new();
        mounts.push(ProcMount {
            device: "sysfs".into(), mount_point: "/sys".into(),
            fs_type: "sysfs".into(), options: "rw,nosuid,nodev,noexec,relatime".into(),
        });
        mounts.push(ProcMount {
            device: "proc".into(), mount_point: "/proc".into(),
            fs_type: "proc".into(), options: "rw,nosuid,nodev,noexec,relatime".into(),
        });
        mounts.push(ProcMount {
            device: "devtmpfs".into(), mount_point: "/dev".into(),
            fs_type: "devtmpfs".into(), options: "rw,nosuid,size=8192k,nr_inodes=4096".into(),
        });
        mounts.push(ProcMount {
            device: "/dev/sda1".into(), mount_point: "/".into(),
            fs_type: "sigma_ext".into(), options: "rw,relatime".into(),
        });
        mounts.push(ProcMount {
            device: "tmpfs".into(), mount_point: "/tmp".into(),
            fs_type: "tmpfs".into(), options: "rw,nosuid,nodev".into(),
        });
        Self {
            uptime_secs: 0.0,
            cpuinfo: vec![ProcCpuInfo::default()],
            meminfo: ProcMemInfo { mem_total_kb: 16 * 1024 * 1024, mem_free_kb: 8 * 1024 * 1024,
                mem_available_kb: 12 * 1024 * 1024, hugepage_size_kb: 2048, ..Default::default() },
            stat: ProcStat::default(),
            processes: BTreeMap::new(),
            mounts,
            kernel_version: kernel_version.into(),
            hostname: hostname.into(),
            loadavg: (0.0, 0.0, 0.0),
            last_pid: 1,
        }
    }

    /// Register a process in the process table.
    pub fn add_process(&mut self, info: ProcProcessInfo) {
        self.last_pid = self.last_pid.max(info.pid);
        self.processes.insert(info.pid, info);
    }

    /// Remove a process (it exited).
    pub fn remove_process(&mut self, pid: u32) {
        self.processes.remove(&pid);
    }

    /// Update system uptime.
    pub fn set_uptime(&mut self, secs: f64) { self.uptime_secs = secs; }

    /// Update load averages.
    pub fn set_loadavg(&mut self, a1: f64, a5: f64, a15: f64) {
        self.loadavg = (a1, a5, a15);
    }

    /// Read a /proc path and return its content as bytes.
    ///
    /// This is the main entry point — analogous to VFS read().
    pub fn read(&self, path: &str) -> Result<Vec<u8>, &'static str> {
        let path = path.trim_start_matches("/proc/").trim_start_matches('/');
        match path {
            "cpuinfo"    => Ok(self.gen_cpuinfo().into_bytes()),
            "meminfo"    => Ok(self.gen_meminfo().into_bytes()),
            "uptime"     => Ok(self.gen_uptime().into_bytes()),
            "loadavg"    => Ok(self.gen_loadavg().into_bytes()),
            "stat"       => Ok(self.gen_stat().into_bytes()),
            "mounts"     => Ok(self.gen_mounts().into_bytes()),
            "version"    => Ok(self.gen_version().into_bytes()),
            "hostname" | "sys/kernel/hostname" => Ok(format!("{}\n", self.hostname).into_bytes()),
            _ if path.starts_with("net/") => self.read_net(path),
            _ => {
                // Try to parse as PID path: "<pid>/..."
                let mut parts = path.splitn(2, '/');
                let pid_str = parts.next().unwrap_or("");
                let sub = parts.next().unwrap_or("status");
                if let Ok(pid) = pid_str.parse::<u32>() {
                    self.read_pid(pid, sub)
                } else {
                    Err("no such file or directory")
                }
            }
        }
    }

    /// List entries at a /proc path (directory listing).
    pub fn readdir(&self, path: &str) -> Result<Vec<String>, &'static str> {
        let path = path.trim_start_matches("/proc").trim_start_matches('/');
        match path {
            "" | "/" => {
                let mut entries = vec![
                    "cpuinfo".into(), "meminfo".into(), "uptime".into(),
                    "loadavg".into(), "stat".into(), "mounts".into(),
                    "version".into(), "net".into(), "sys".into(),
                ];
                for pid in self.processes.keys() {
                    entries.push(pid.to_string());
                }
                Ok(entries)
            }
            p if p.chars().all(|c| c.is_ascii_digit()) => {
                Ok(vec!["status".into(), "stat".into(), "cmdline".into(),
                        "environ".into(), "maps".into(), "fd".into(), "statm".into()])
            }
            _ => Err("not a directory"),
        }
    }

    // -- Generator methods --

    fn gen_cpuinfo(&self) -> String {
        let mut out = String::new();
        for cpu in &self.cpuinfo {
            out.push_str(&format!("processor\t: {}\n", cpu.processor_id));
            out.push_str(&format!("vendor_id\t: {}\n", cpu.vendor_id));
            out.push_str(&format!("cpu family\t: {}\n", cpu.cpu_family));
            out.push_str(&format!("model\t\t: {}\n", cpu.model));
            out.push_str(&format!("model name\t: {}\n", cpu.model_name));
            out.push_str(&format!("stepping\t: {}\n", cpu.stepping));
            out.push_str(&format!("cpu MHz\t\t: {:.3}\n", cpu.mhz));
            out.push_str(&format!("cache size\t: {} KB\n", cpu.cache_size_kb));
            out.push_str(&format!("cpu cores\t: {}\n", cpu.cpu_cores));
            out.push_str(&format!("siblings\t: {}\n", cpu.siblings));
            out.push_str(&format!("flags\t\t: {}\n", cpu.flags.join(" ")));
            out.push('\n');
        }
        out
    }

    fn gen_meminfo(&self) -> String {
        let m = &self.meminfo;
        format!(
            "MemTotal:       {:>10} kB\n\
             MemFree:        {:>10} kB\n\
             MemAvailable:   {:>10} kB\n\
             Buffers:        {:>10} kB\n\
             Cached:         {:>10} kB\n\
             SwapCached:     {:>10} kB\n\
             Active:         {:>10} kB\n\
             Inactive:       {:>10} kB\n\
             SwapTotal:      {:>10} kB\n\
             SwapFree:       {:>10} kB\n\
             Dirty:          {:>10} kB\n\
             Writeback:      {:>10} kB\n\
             Slab:           {:>10} kB\n\
             PageTables:     {:>10} kB\n\
             VmallocTotal:   {:>10} kB\n\
             VmallocUsed:    {:>10} kB\n\
             HugePages_Total:{:>10}\n\
             HugePages_Free: {:>10}\n\
             Hugepagesize:   {:>10} kB\n",
            m.mem_total_kb, m.mem_free_kb, m.mem_available_kb,
            m.buffers_kb, m.cached_kb, m.swap_cached_kb,
            m.active_kb, m.inactive_kb, m.swap_total_kb, m.swap_free_kb,
            m.dirty_kb, m.writeback_kb, m.slab_kb, m.page_tables_kb,
            m.vmalloc_total_kb, m.vmalloc_used_kb,
            m.hugepages_total, m.hugepages_free, m.hugepage_size_kb,
        )
    }

    fn gen_uptime(&self) -> String {
        format!("{:.2} {:.2}\n", self.uptime_secs, self.uptime_secs * 0.9)
    }

    fn gen_loadavg(&self) -> String {
        let (a1, a5, a15) = self.loadavg;
        let running = self.processes.values().filter(|p| p.state == ProcProcessState::Running).count();
        let total = self.processes.len();
        format!("{:.2} {:.2} {:.2} {}/{} {}\n", a1, a5, a15, running, total, self.last_pid)
    }

    fn gen_stat(&self) -> String {
        let s = &self.stat;
        let cpu = s.cpu_times;
        let mut out = format!(
            "cpu  {} {} {} {} {} {} {} {}\n",
            cpu[0], cpu[1], cpu[2], cpu[3], cpu[4], cpu[5], cpu[6], cpu[7]
        );
        for (i, c) in s.per_cpu.iter().enumerate() {
            out.push_str(&format!(
                "cpu{} {} {} {} {} {} {} {} {}\n",
                i, c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]
            ));
        }
        out.push_str(&format!("intr {}\n", s.intr_total));
        out.push_str(&format!("ctxt {}\n", s.ctxt));
        out.push_str(&format!("btime {}\n", s.btime));
        out.push_str(&format!("processes {}\n", s.processes));
        out.push_str(&format!("procs_running {}\n", s.procs_running));
        out.push_str(&format!("procs_blocked {}\n", s.procs_blocked));
        out
    }

    fn gen_mounts(&self) -> String {
        self.mounts.iter().map(|m| {
            format!("{} {} {} {} 0 0\n", m.device, m.mount_point, m.fs_type, m.options)
        }).collect()
    }

    fn gen_version(&self) -> String {
        format!("Linux version {} (sigma@sigmaos) (Rust compiler) #1 SMP\n", self.kernel_version)
    }

    fn read_net(&self, path: &str) -> Result<Vec<u8>, &'static str> {
        match path {
            "net/dev" => Ok(
                "Inter-|   Receive                                                |  Transmit\n \
                 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed\n\
                    lo:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0\n\
                  eth0: 1234567    9876    0    0    0     0          0         0   654321    6543    0    0    0     0       0          0\n"
                .as_bytes().to_vec()
            ),
            _ => Err("no such file"),
        }
    }

    fn read_pid(&self, pid: u32, sub: &str) -> Result<Vec<u8>, &'static str> {
        let info = self.processes.get(&pid).ok_or("no such process")?;
        match sub {
            "status" => Ok(self.gen_pid_status(info).into_bytes()),
            "stat"   => Ok(self.gen_pid_stat(info).into_bytes()),
            "cmdline" => Ok(info.cmdline.as_bytes().to_vec()),
            "statm"  => Ok(self.gen_pid_statm(info).into_bytes()),
            "maps"   => Ok(self.gen_pid_maps(info).into_bytes()),
            "fd"     => Ok(info.open_fds.iter().map(|f| format!("{}\n", f)).collect::<String>().into_bytes()),
            _ => Err("no such file"),
        }
    }

    fn gen_pid_status(&self, p: &ProcProcessInfo) -> String {
        format!(
            "Name:\t{}\nState:\t{} ({})\nPid:\t{}\nPPid:\t{}\nUid:\t{}\nGid:\t{}\n\
             VmPeak:\t{} kB\nVmSize:\t{} kB\nVmRSS:\t{} kB\nThreads:\t{}\n",
            p.name, p.state.as_char(),
            match p.state { ProcProcessState::Running => "running", ProcProcessState::Sleeping => "sleeping",
                _ => "other" },
            p.pid, p.ppid, p.uid, p.gid,
            p.vm_peak_kb, p.vm_size_kb, p.vm_rss_kb, p.threads
        )
    }

    fn gen_pid_stat(&self, p: &ProcProcessInfo) -> String {
        format!(
            "{} ({}) {} {} {} 0 0 0 0 0 0 {} {} {} {} {} {} 0 0 -1 0 {} 0 {} {} 0 0\n",
            p.pid, p.name, p.state.as_char(), p.ppid, p.pid,
            p.utime_ticks, p.stime_ticks, p.utime_ticks, p.stime_ticks,
            p.priority, p.nice, p.start_time_ticks,
            p.vm_size_kb * 1024, p.vm_rss_kb * 4
        )
    }

    fn gen_pid_statm(&self, p: &ProcProcessInfo) -> String {
        let pages = p.vm_size_kb / 4;
        let rss = p.vm_rss_kb / 4;
        format!("{} {} {} 0 0 {} 0\n", pages, rss, rss, pages)
    }

    fn gen_pid_maps(&self, p: &ProcProcessInfo) -> String {
        p.mem_maps.iter().map(|m| {
            format!("{:016x}-{:016x} {} {:08x} 00:00 0\t\t{}\n",
                m.start, m.end, m.perms_str(), m.offset, m.name)
        }).collect()
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_procfs() -> ProcfsMount {
        let mut pfs = ProcfsMount::new("sigmahost", "6.6.0-sigma");
        pfs.set_uptime(12345.67);
        pfs.set_loadavg(0.42, 0.35, 0.28);
        pfs.add_process(ProcProcessInfo {
            pid: 1, ppid: 0, name: "init".into(),
            state: ProcProcessState::Sleeping,
            uid: 0, gid: 0, vm_rss_kb: 4096, vm_size_kb: 16384,
            vm_peak_kb: 20000, threads: 1,
            utime_ticks: 100, stime_ticks: 50,
            cmdline: "/sbin/init".into(), start_time_ticks: 0,
            nice: 0, priority: 20, open_fds: vec![0, 1, 2],
            mem_maps: vec![
                ProcMemMap { start: 0x400000, end: 0x401000, readable: true,
                    writable: false, executable: true, shared: false,
                    offset: 0, name: "/sbin/init".into() }
            ],
        });
        pfs
    }

    #[test]
    fn test_cpuinfo() {
        let pfs = make_procfs();
        let content = String::from_utf8(pfs.read("/proc/cpuinfo").unwrap()).unwrap();
        assert!(content.contains("SigmaOS Sovereign Processor"));
        assert!(content.contains("cpu MHz"));
        assert!(content.contains("avx2"));
    }

    #[test]
    fn test_meminfo() {
        let pfs = make_procfs();
        let content = String::from_utf8(pfs.read("meminfo").unwrap()).unwrap();
        assert!(content.contains("MemTotal:"));
        assert!(content.contains("MemFree:"));
    }

    #[test]
    fn test_uptime() {
        let pfs = make_procfs();
        let content = String::from_utf8(pfs.read("uptime").unwrap()).unwrap();
        assert!(content.contains("12345.67"));
    }

    #[test]
    fn test_process_status() {
        let pfs = make_procfs();
        let content = String::from_utf8(pfs.read("1/status").unwrap()).unwrap();
        assert!(content.contains("Name:\tinit"));
        assert!(content.contains("Pid:\t1"));
    }

    #[test]
    fn test_mounts() {
        let pfs = make_procfs();
        let content = String::from_utf8(pfs.read("mounts").unwrap()).unwrap();
        assert!(content.contains("sigma_ext"));
        assert!(content.contains("/tmp"));
    }

    #[test]
    fn test_readdir_root() {
        let pfs = make_procfs();
        let entries = pfs.readdir("/proc").unwrap();
        assert!(entries.contains(&"cpuinfo".to_string()));
        assert!(entries.contains(&"1".to_string()));
    }
}
