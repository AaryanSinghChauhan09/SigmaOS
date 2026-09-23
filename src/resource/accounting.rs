/// Sovereign Process & System Accounting Subsystem (SigmaAccounting)
/// Inspired by Linux Process Accounting (`acct_v3` / `/var/log/pacct`),
/// BSD Resource Accounting (`getrusage` / `sa(8)` / `lastcomm`),
/// BSD User Session Accounting (`utmp` / `wtmp` / `lastlog` / `btmp`),
/// and Linux Netlink Taskstats & Delay Accounting.


use std::vec::Vec;
use std::string::String;
#[cfg(all(not(feature = "standalone_test"), not(test)))]
use crate::klib::HashMap;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap as HashMap;

pub type Pid = usize;
pub type UserID = u32;
pub type GroupID = u32;

// ─────────────────────────────────────────────────────────────────────────────
// 1. LINUX PROCESS ACCOUNTING (acct_v3 / pacct)
// ─────────────────────────────────────────────────────────────────────────────

pub mod acct_flags {
    pub const AFORK: u8 = 1 << 0; // Process was created by fork but not exec'd
    pub const ASU: u8   = 1 << 1; // Process used superuser privileges
    pub const ACORE: u8 = 1 << 2; // Process dumped core
    pub const AXSIG: u8 = 1 << 3; // Process was killed by a signal
}

#[derive(Debug, Clone)]
pub struct AcctV3Record {
    pub pid: Pid,
    pub ppid: Pid,
    pub uid: UserID,
    pub gid: GroupID,
    pub command_name: String,
    pub utime_ms: u64,       // User CPU time in milliseconds
    pub stime_ms: u64,       // System CPU time in milliseconds
    pub etime_ms: u64,       // Elapsed wall-clock time in milliseconds
    pub start_time_sec: u64, // Process start time (POSIX epoch)
    pub io_bytes_read: u64,  // Character I/O bytes read
    pub io_bytes_written: u64, // Character I/O bytes written
    pub page_faults_minor: u64,
    pub page_faults_major: u64,
    pub exit_code: u32,
    pub flags: u8,            // AC_AFORK, AC_ASU, AC_ACORE, AC_AXSIG
}

impl AcctV3Record {
    pub fn new(pid: Pid, ppid: Pid, uid: UserID, gid: GroupID, comm: &str) -> Self {
        Self {
            pid,
            ppid,
            uid,
            gid,
            command_name: String::from(comm),
            utime_ms: 0,
            stime_ms: 0,
            etime_ms: 0,
            start_time_sec: 0,
            io_bytes_read: 0,
            io_bytes_written: 0,
            page_faults_minor: 0,
            page_faults_major: 0,
            exit_code: 0,
            flags: 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. BSD HIGH-PRECISION RESOURCE ACCOUNTING (getrusage / sa(8))
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RusageAccounting {
    pub user_cpu_us: u64,          // ru_utime in microseconds
    pub system_cpu_us: u64,        // ru_stime in microseconds
    pub max_rss_bytes: u64,        // ru_maxrss peak resident set size
    pub minor_page_faults: u64,    // ru_minflt page reclaims
    pub major_page_faults: u64,    // ru_majflt page faults requiring I/O
    pub block_input_ops: u64,      // ru_inblock filesystem reads
    pub block_output_ops: u64,     // ru_oublock filesystem writes
    pub vol_context_switches: u64,  // ru_nvcsw voluntary switches
    pub invol_context_switches: u64,// ru_nivcsw involuntary switches
    pub signals_delivered: u64,    // ru_nsignals signals received
}

impl RusageAccounting {
    pub fn new() -> Self {
        Self::default()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. BSD SESSION ACCOUNTING (utmp / wtmp / lastlog / btmp)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    Empty = 0,
    RunLvl = 1,
    BootTime = 2,
    NewTime = 3,
    OldTime = 4,
    InitProcess = 5,
    LoginProcess = 6,
    UserProcess = 7,
    DeadProcess = 8,
    FailedLogin = 9, // btmp entry
}

#[derive(Debug, Clone)]
pub struct UtmpSessionRecord {
    pub session_type: SessionType,
    pub pid: Pid,
    pub line_tty: String,
    pub user_name: String,
    pub host_name: String,
    pub ip_address: [u8; 4],
    pub timestamp_sec: u64,
}

impl UtmpSessionRecord {
    pub fn new(session_type: SessionType, pid: Pid, tty: &str, user: &str, host: &str, timestamp: u64) -> Self {
        Self {
            session_type,
            pid,
            line_tty: String::from(tty),
            user_name: String::from(user),
            host_name: String::from(host),
            ip_address: [0, 0, 0, 0],
            timestamp_sec: timestamp,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. LINUX TASKSTATS & DELAY ACCOUNTING
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TaskstatsAccount {
    pub cpu_delay_ns: u64,       // Time waiting for CPU
    pub block_io_delay_ns: u64,  // Time waiting for Block I/O completion
    pub swap_in_delay_ns: u64,   // Time waiting for Swap-in page faults
    pub memory_reclaim_delay_ns: u64, // Time waiting for memory page reclaim
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. LINUX & BSD NETWORK & DISK I/O ACCOUNTING (netstat -i / diskstats / gstat)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SovereignNetworkInterfaceAccounting {
    pub interface_name: String,
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errors: u64,
    pub rx_dropped: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errors: u64,
    pub tx_dropped: u64,
    pub tcp_retransmits: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SovereignStorageIoAccounting {
    pub device_name: String,
    pub reads_completed: u64,
    pub read_sectors: u64,
    pub read_time_ms: u64,
    pub writes_completed: u64,
    pub write_sectors: u64,
    pub write_time_ms: u64,
    pub io_in_progress_queue_depth: u32,
    pub io_total_time_ms: u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. SOVEREIGN SYSTEM ACCOUNTING ENGINE
// ─────────────────────────────────────────────────────────────────────────────

pub struct CommandSummaryStats {
    pub command_name: String,
    pub total_calls: usize,
    pub total_utime_ms: u64,
    pub total_stime_ms: u64,
    pub total_io_bytes: u64,
}

pub struct SovereignAccountingEngine {
    pub process_pacct_log: Vec<AcctV3Record>,
    pub active_rusage_table: HashMap<Pid, RusageAccounting>,
    pub user_wtmp_log: Vec<UtmpSessionRecord>,
    pub failed_btmp_log: Vec<UtmpSessionRecord>,
    pub delay_accounting_table: HashMap<Pid, TaskstatsAccount>,
    pub net_interface_accounting_table: HashMap<String, SovereignNetworkInterfaceAccounting>,
    pub disk_io_accounting_table: HashMap<String, SovereignStorageIoAccounting>,
    pub accounting_enabled: bool,
}

impl SovereignAccountingEngine {
    pub fn new() -> Self {
        Self {
            process_pacct_log: Vec::new(),
            active_rusage_table: HashMap::new(),
            user_wtmp_log: Vec::new(),
            failed_btmp_log: Vec::new(),
            delay_accounting_table: HashMap::new(),
            net_interface_accounting_table: HashMap::new(),
            disk_io_accounting_table: HashMap::new(),
            accounting_enabled: true,
        }
    }

    /// Records network interface statistics
    pub fn update_net_interface_stats(&mut self, stats: SovereignNetworkInterfaceAccounting) {
        if self.accounting_enabled {
            self.net_interface_accounting_table.insert(stats.interface_name.clone(), stats);
        }
    }

    /// Records storage device I/O diskstats statistics
    pub fn update_disk_io_stats(&mut self, stats: SovereignStorageIoAccounting) {
        if self.accounting_enabled {
            self.disk_io_accounting_table.insert(stats.device_name.clone(), stats);
        }
    }

    /// Records process completion to pacct log file (`acct(2)` behavior)
    pub fn record_process_exit(&mut self, record: AcctV3Record) {
        if self.accounting_enabled {
            self.process_pacct_log.push(record);
        }
    }

    /// Records user session event to wtmp or btmp log
    pub fn record_session_event(&mut self, record: UtmpSessionRecord) {
        if !self.accounting_enabled {
            return;
        }

        if record.session_type == SessionType::FailedLogin {
            self.failed_btmp_log.push(record);
        } else {
            self.user_wtmp_log.push(record);
        }
    }

    /// Updates live rusage metrics for process
    pub fn update_rusage(&mut self, pid: Pid, rusage: RusageAccounting) {
        self.active_rusage_table.insert(pid, rusage);
    }

    /// Generates process accounting summary report (BSD `sa(8)` command equivalent)
    pub fn generate_sa_summary(&self) -> Vec<CommandSummaryStats> {
        // Bolt performance optimization: Key map by borrowed string slice `&str` during aggregation
        // to avoid allocating heap `String` clones for every log record in `process_pacct_log`.
        let mut map: HashMap<&str, (usize, u64, u64, u64)> = HashMap::new();

        for rec in &self.process_pacct_log {
            let entry = map.entry(rec.command_name.as_str()).or_insert((0, 0, 0, 0));
            entry.0 += 1;
            entry.1 += rec.utime_ms;
            entry.2 += rec.stime_ms;
            entry.3 += rec.io_bytes_read + rec.io_bytes_written;
        }

        let mut summaries = Vec::with_capacity(map.len());
        for (name, (calls, utime, stime, io_bytes)) in map {
            summaries.push(CommandSummaryStats {
                command_name: String::from(name),
                total_calls: calls,
                total_utime_ms: utime,
                total_stime_ms: stime,
                total_io_bytes: io_bytes,
            });
        }

        summaries
    }
}

impl Default for SovereignAccountingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_acct_v3_record() {
        let mut rec = AcctV3Record::new(101, 1, 1000, 1000, "cargo");
        rec.utime_ms = 1200;
        rec.stime_ms = 300;
        rec.io_bytes_written = 1024 * 1024;
        rec.flags |= acct_flags::ASU;

        assert_eq!(rec.pid, 101);
        assert_eq!(rec.command_name, "cargo");
        assert_eq!(rec.flags & acct_flags::ASU, acct_flags::ASU);
    }

    #[test]
    fn test_session_accounting_wtmp_btmp() {
        let mut engine = SovereignAccountingEngine::new();

        let login = UtmpSessionRecord::new(SessionType::UserProcess, 1001, "tty1", "aaryan", "local_console", 1700000000);
        let failed = UtmpSessionRecord::new(SessionType::FailedLogin, 1002, "tty2", "attacker", "192.168.1.50", 1700000005);

        engine.record_session_event(login);
        engine.record_session_event(failed);

        assert_eq!(engine.user_wtmp_log.len(), 1);
        assert_eq!(engine.failed_btmp_log.len(), 1);
        assert_eq!(engine.failed_btmp_log[0].user_name, "attacker");
    }

    #[test]
    fn test_bsd_sa_summary_generation() {
        let mut engine = SovereignAccountingEngine::new();

        let mut rec1 = AcctV3Record::new(10, 1, 1000, 1000, "rustc");
        rec1.utime_ms = 500;
        rec1.io_bytes_read = 2048;

        let mut rec2 = AcctV3Record::new(11, 1, 1000, 1000, "rustc");
        rec2.utime_ms = 700;
        rec2.io_bytes_read = 4096;

        engine.record_process_exit(rec1);
        engine.record_process_exit(rec2);

        let summaries = engine.generate_sa_summary();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].command_name, "rustc");
        assert_eq!(summaries[0].total_calls, 2);
        assert_eq!(summaries[0].total_utime_ms, 1200);
        assert_eq!(summaries[0].total_io_bytes, 6144);
    }

    #[test]
    fn test_network_interface_accounting() {
        let mut engine = SovereignAccountingEngine::new();
        let net_stat = SovereignNetworkInterfaceAccounting {
            interface_name: "eth0".to_string(),
            rx_bytes: 1048576,
            rx_packets: 1024,
            rx_errors: 0,
            rx_dropped: 2,
            tx_bytes: 2097152,
            tx_packets: 2048,
            tx_errors: 0,
            tx_dropped: 0,
            tcp_retransmits: 1,
        };

        engine.update_net_interface_stats(net_stat);
        assert_eq!(engine.net_interface_accounting_table.len(), 1);
        let eth0 = engine.net_interface_accounting_table.get("eth0").unwrap();
        assert_eq!(eth0.rx_bytes, 1048576);
        assert_eq!(eth0.rx_dropped, 2);
        assert_eq!(eth0.tcp_retransmits, 1);
    }

    #[test]
    fn test_storage_io_accounting() {
        let mut engine = SovereignAccountingEngine::new();
        let disk_stat = SovereignStorageIoAccounting {
            device_name: "nvme0n1".to_string(),
            reads_completed: 5000,
            read_sectors: 40000,
            read_time_ms: 120,
            writes_completed: 2500,
            write_sectors: 20000,
            write_time_ms: 80,
            io_in_progress_queue_depth: 3,
            io_total_time_ms: 200,
        };

        engine.update_disk_io_stats(disk_stat);
        assert_eq!(engine.disk_io_accounting_table.len(), 1);
        let nvme = engine.disk_io_accounting_table.get("nvme0n1").unwrap();
        assert_eq!(nvme.reads_completed, 5000);
        assert_eq!(nvme.io_in_progress_queue_depth, 3);
    }
}
