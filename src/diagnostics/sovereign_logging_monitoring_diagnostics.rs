//! Sovereign Linux & BSD Logging, Monitoring, and Diagnostics Engine for SigmaOS
//!
//! Synthesizes capabilities inspired by Linux & BSD system administration, observability,
//! and telemetry tooling:
//! - Journald, Rsyslog, Syslog-ng, Syslogd & Logrotate: Multi-facility structured logging,
//!   RFC 5424 / RFC 3164 syslog framing, action queues, pattern parsing, BSD /etc/syslog.conf
//!   selector-action rules, CRC entry integrity, JSON exports, and automated log rotation policies.
//! - Dmesg: Kernel ring buffer logger with microsecond boot/ISO/delta timestamps, severity levels,
//!   console loglevel control, colorized log rendering simulation, and subsystem filter matrix.
//! - Top & Htop: Process activity telemetry, thread counts, process states (R, S, D, T, Z, I),
//!   CPU/RAM utilization, per-CPU core breakdown (%user, %sys, %iowait, %irq, %softirq, %steal, %idle),
//!   process tree hierarchy, nice/priority adjustment, and signal sending simulation (SIGTERM, SIGKILL, SIGHUP).
//! - Vmstat, Iostat, Sar & Sysstat: System memory (total, free, active, inactive, wired, buffers, cached, slab),
//!   swap (si, so), block I/O (bi, bo), interrupts & context switches, disk device IOPS/latency/queue depth,
//!   and historical time-series sampling buffers (`saDD` daily datasets).
//! - Lsof: Open file descriptor inspector across processes, paths, socket ports, descriptor types
//!   (REG, DIR, CHR, BLK, FIFO, UNIX, IPv4, IPv6, KQUEUE, INOTIFY), and TCP state (LISTEN, ESTABLISHED, TIME_WAIT).
//! - Lspci, Lsusb, Dmidecode & Lsscsi: Comprehensive hardware discovery for PCI/PCIe topology,
//!   USB speeds & drivers, SMBIOS/DMI tables (BIOS, System, Board, CPU, RAM DIMMs), and SCSI target/LUN devices.
//! - Sosreport, Prometheus node_exporter & Collectd: Automated diagnostic archive bundle generation
//!   with sensitive data scrubbing, full Prometheus exposition format, and collectd protocol metrics exporter.

use std::collections::{BTreeMap, HashMap};
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. JOURNALD / RSYSLOG / SYSLOG-NG / SYSLOGD / LOGROTATE LOG ENGINE
// ============================================================================

/// Syslog Facility inspired by Linux syslog.h & BSD RFC 5424
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SyslogFacility {
    Kernel = 0,
    User = 1,
    Mail = 2,
    Daemon = 3,
    Auth = 4,
    Syslog = 5,
    Lpr = 6,
    News = 7,
    Uucp = 8,
    Cron = 9,
    AuthPriv = 10,
    Ftp = 11,
    Local0 = 16,
    Local1 = 17,
    Local2 = 18,
    Local3 = 19,
    Local4 = 20,
    Local5 = 21,
    Local6 = 22,
    Local7 = 23,
}

impl SyslogFacility {
    pub fn as_str(&self) -> &'static str {
        match self {
            SyslogFacility::Kernel => "kern",
            SyslogFacility::User => "user",
            SyslogFacility::Mail => "mail",
            SyslogFacility::Daemon => "daemon",
            SyslogFacility::Auth => "auth",
            SyslogFacility::Syslog => "syslog",
            SyslogFacility::Lpr => "lpr",
            SyslogFacility::News => "news",
            SyslogFacility::Uucp => "uucp",
            SyslogFacility::Cron => "cron",
            SyslogFacility::AuthPriv => "authpriv",
            SyslogFacility::Ftp => "ftp",
            SyslogFacility::Local0 => "local0",
            SyslogFacility::Local1 => "local1",
            SyslogFacility::Local2 => "local2",
            SyslogFacility::Local3 => "local3",
            SyslogFacility::Local4 => "local4",
            SyslogFacility::Local5 => "local5",
            SyslogFacility::Local6 => "local6",
            SyslogFacility::Local7 => "local7",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "kern" | "kernel" => Some(SyslogFacility::Kernel),
            "user" => Some(SyslogFacility::User),
            "mail" => Some(SyslogFacility::Mail),
            "daemon" => Some(SyslogFacility::Daemon),
            "auth" => Some(SyslogFacility::Auth),
            "syslog" => Some(SyslogFacility::Syslog),
            "lpr" => Some(SyslogFacility::Lpr),
            "news" => Some(SyslogFacility::News),
            "uucp" => Some(SyslogFacility::Uucp),
            "cron" => Some(SyslogFacility::Cron),
            "authpriv" => Some(SyslogFacility::AuthPriv),
            "ftp" => Some(SyslogFacility::Ftp),
            "local0" => Some(SyslogFacility::Local0),
            "local1" => Some(SyslogFacility::Local1),
            "local2" => Some(SyslogFacility::Local2),
            "local3" => Some(SyslogFacility::Local3),
            "local4" => Some(SyslogFacility::Local4),
            "local5" => Some(SyslogFacility::Local5),
            "local6" => Some(SyslogFacility::Local6),
            "local7" => Some(SyslogFacility::Local7),
            _ => None,
        }
    }
}

/// Log Severity inspired by Syslog RFC 5424 / Journald
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogSeverity {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

impl LogSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogSeverity::Emergency => "EMERG",
            LogSeverity::Alert => "ALERT",
            LogSeverity::Critical => "CRIT",
            LogSeverity::Error => "ERR",
            LogSeverity::Warning => "WARNING",
            LogSeverity::Notice => "NOTICE",
            LogSeverity::Info => "INFO",
            LogSeverity::Debug => "DEBUG",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "emerg" | "emergency" | "0" => Some(LogSeverity::Emergency),
            "alert" | "1" => Some(LogSeverity::Alert),
            "crit" | "critical" | "2" => Some(LogSeverity::Critical),
            "err" | "error" | "3" => Some(LogSeverity::Error),
            "warning" | "warn" | "4" => Some(LogSeverity::Warning),
            "notice" | "5" => Some(LogSeverity::Notice),
            "info" | "6" => Some(LogSeverity::Info),
            "debug" | "7" => Some(LogSeverity::Debug),
            _ => None,
        }
    }
}

/// Structured Journald Log Entry with systemd-style key-value fields and CRC integrity check
#[derive(Debug, Clone)]
pub struct JournalLogEntry {
    pub id: u64,
    pub timestamp_us: u64,
    pub facility: SyslogFacility,
    pub severity: LogSeverity,
    pub pid: u32,
    pub uid: u32,
    pub app_name: String,
    pub systemd_unit: String,
    pub selinux_context: String,
    pub boot_id: String,
    pub message: String,
    pub metadata: BTreeMap<String, String>,
    pub checksum: u32,
}

impl JournalLogEntry {
    pub fn syslog_pri(&self) -> u8 {
        (self.facility as u8) * 8 + (self.severity as u8)
    }

    /// Calculate simple CRC32-like checksum for entry integrity validation
    pub fn calculate_checksum(&self) -> u32 {
        let mut hash: u32 = 0x811c9dc5;
        let bytes = format!(
            "{}:{}:{}:{}:{}:{}",
            self.id, self.timestamp_us, self.app_name, self.systemd_unit, self.pid, self.message
        );
        for b in bytes.bytes() {
            hash ^= b as u32;
            hash = hash.wrapping_mul(0x01000193);
        }
        hash
    }

    /// Format entry as RFC 5424 Syslog message
    pub fn to_rfc5424(&self, hostname: &str) -> String {
        let pri = self.syslog_pri();
        let mut sd = String::from("[sigmaos@53828");
        sd.push_str(&format!(" _PID=\"{}\"", self.pid));
        sd.push_str(&format!(" _UID=\"{}\"", self.uid));
        if !self.systemd_unit.is_empty() {
            sd.push_str(&format!(" _SYSTEMD_UNIT=\"{}\"", self.systemd_unit));
        }
        for (k, v) in &self.metadata {
            sd.push_str(&format!(" {}=\"{}\"", k, v));
        }
        sd.push(']');

        format!(
            "<{}>1 {} {} {} {} - {} {}",
            pri, self.timestamp_us, hostname, self.app_name, self.pid, sd, self.message
        )
    }

    /// Format entry as classic RFC 3164 BSD Syslog message
    pub fn to_rfc3164(&self, hostname: &str) -> String {
        let pri = self.syslog_pri();
        format!(
            "<{}> Jan 1 00:00:00 {} {}[{}]: {}",
            pri, hostname, self.app_name, self.pid, self.message
        )
    }

    /// Export entry as JSON string (inspired by `journalctl -o json-pretty`)
    pub fn to_json(&self, hostname: &str) -> String {
        let mut meta_str = String::new();
        for (k, v) in &self.metadata {
            meta_str.push_str(&format!(",\"{}\":\"{}\"", k, v));
        }

        format!(
            "{{\"__CURSOR\":\"{}\",\"_REALTIME_TIMESTAMP\":\"{}\",\"PRIORITY\":\"{}\",\"SYSLOG_FACILITY\":\"{}\",\"_HOSTNAME\":\"{}\",\"_APP_NAME\":\"{}\",\"_PID\":\"{}\",\"_UID\":\"{}\",\"_SYSTEMD_UNIT\":\"{}\",\"MESSAGE\":\"{}\",\"CRC32\":\"{:08x}\"{}}}",
            self.id, self.timestamp_us, self.severity as u8, self.facility.as_str(), hostname, self.app_name, self.pid, self.uid, self.systemd_unit, self.message, self.checksum, meta_str
        )
    }

    pub fn verify_checksum(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }
}

/// Rsyslog Action Queue Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RsyslogQueueMode {
    Direct,
    InMemory,
    DiskAssisted,
}

/// Expression-based Rsyslog Filter Rule
#[derive(Debug, Clone)]
pub struct RsyslogFilterRule {
    pub rule_id: String,
    pub facility: Option<SyslogFacility>,
    pub min_severity: Option<LogSeverity>,
    pub message_contains: Option<String>,
    pub action_target: String,
}

/// Syslog-ng Destination Driver
#[derive(Debug, Clone)]
pub enum SyslogNgDestination {
    File(String),
    SyslogTcp(String, u16),
    SyslogUdp(String, u16),
    NamedPipe(String),
    Elasticsearch(String),
}

/// Pattern DB Extraction Rule (`syslog-ng` patterndb)
#[derive(Debug, Clone)]
pub struct PatternDbRule {
    pub pattern_id: String,
    pub keyword: String,
    pub extracted_key: String,
}

impl PatternDbRule {
    pub fn extract(&self, log_msg: &str) -> Option<(String, String)> {
        if log_msg.contains(&self.keyword) {
            let parts: Vec<&str> = log_msg.split_whitespace().collect();
            if let Some(last) = parts.last() {
                return Some((self.extracted_key.clone(), last.to_string()));
            }
        }
        None
    }
}

/// Classic BSD `/etc/syslog.conf` Selector and Action Rule
#[derive(Debug, Clone)]
pub struct BsdSyslogRule {
    pub selector_facility: String, // e.g., "auth", "kern", "*"
    pub selector_priority: String, // e.g., "notice", "emerg", "*"
    pub action_target: String,     // e.g., "/var/log/auth.log", "@loghost", "*"
}

impl BsdSyslogRule {
    pub fn matches(&self, facility: SyslogFacility, severity: LogSeverity) -> bool {
        let fac_match = self.selector_facility == "*"
            || self.selector_facility.to_lowercase() == facility.as_str();
        let sev_match = if self.selector_priority == "*" {
            true
        } else if let Some(req_sev) = LogSeverity::from_str(&self.selector_priority) {
            severity <= req_sev
        } else {
            false
        };
        fac_match && sev_match
    }
}

/// Log Rotation Frequency Strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogRotateFrequency {
    Daily,
    Weekly,
    Monthly,
    SizeBased(u64),
}

/// Extended Log Rotation Policy (`logrotate`)
#[derive(Debug, Clone)]
pub struct LogRotateConfig {
    pub frequency: LogRotateFrequency,
    pub max_entries_per_file: usize,
    pub max_rotated_generations: usize,
    pub compress_zstd: bool,
    pub copytruncate: bool,
    pub delaycompress: bool,
    pub prerotate_cmd: String,
    pub postrotate_cmd: String,
}

/// Unified Journald, Rsyslog, Syslog-ng, Syslogd & Logrotate Engine
#[derive(Debug, Clone)]
pub struct JournaldRsyslogLogEngine {
    pub hostname: String,
    pub boot_id: String,
    pub entries: Vec<JournalLogEntry>,
    pub rotated_archives: Vec<Vec<JournalLogEntry>>,
    pub config: LogRotateConfig,
    pub rsyslog_queue_mode: RsyslogQueueMode,
    pub rsyslog_rules: Vec<RsyslogFilterRule>,
    pub syslog_ng_destinations: Vec<SyslogNgDestination>,
    pub bsd_syslog_rules: Vec<BsdSyslogRule>,
    pub pattern_db: Vec<PatternDbRule>,
    pub next_id: u64,
    pub rate_limit_burst: usize,
    pub current_burst_count: usize,
}

impl JournaldRsyslogLogEngine {
    pub fn new(hostname: &str) -> Self {
        let mut engine = Self {
            hostname: hostname.to_string(),
            boot_id: "a1b2c3d4-e5f6-7890-abcd-1234567890ab".to_string(),
            entries: Vec::new(),
            rotated_archives: Vec::new(),
            config: LogRotateConfig {
                frequency: LogRotateFrequency::Daily,
                max_entries_per_file: 500,
                max_rotated_generations: 4,
                compress_zstd: true,
                copytruncate: true,
                delaycompress: true,
                prerotate_cmd: "/bin/systemctl stop syslog-flush".to_string(),
                postrotate_cmd: "/bin/systemctl reload journald".to_string(),
            },
            rsyslog_queue_mode: RsyslogQueueMode::DiskAssisted,
            rsyslog_rules: Vec::new(),
            syslog_ng_destinations: Vec::new(),
            bsd_syslog_rules: Vec::new(),
            pattern_db: Vec::new(),
            next_id: 1,
            rate_limit_burst: 100,
            current_burst_count: 0,
        };
        engine.populate_default_rules();
        engine
    }

    fn populate_default_rules(&mut self) {
        // Rsyslog default rules
        self.rsyslog_rules.push(RsyslogFilterRule {
            rule_id: "auth-rule".to_string(),
            facility: Some(SyslogFacility::Auth),
            min_severity: Some(LogSeverity::Warning),
            message_contains: None,
            action_target: "/var/log/auth.log".to_string(),
        });

        // Syslog-ng default destinations
        self.syslog_ng_destinations
            .push(SyslogNgDestination::File("/var/log/messages".to_string()));
        self.syslog_ng_destinations.push(SyslogNgDestination::SyslogTcp(
            "logserver.internal".to_string(),
            514,
        ));

        // BSD syslog.conf rules
        self.bsd_syslog_rules.push(BsdSyslogRule {
            selector_facility: "authpriv".to_string(),
            selector_priority: "info".to_string(),
            action_target: "/var/log/secure".to_string(),
        });
        self.bsd_syslog_rules.push(BsdSyslogRule {
            selector_facility: "*".to_string(),
            selector_priority: "emerg".to_string(),
            action_target: "*".to_string(),
        });

        // Pattern DB extraction rules
        self.pattern_db.push(PatternDbRule {
            pattern_id: "pat-oom".to_string(),
            keyword: "Kill process".to_string(),
            extracted_key: "killed_pid".to_string(),
        });
    }

    pub fn log(
        &mut self,
        facility: SyslogFacility,
        severity: LogSeverity,
        pid: u32,
        app_name: &str,
        message: &str,
        mut metadata: BTreeMap<String, String>,
    ) -> u64 {
        if self.current_burst_count >= self.rate_limit_burst {
            return 0; // Rate limit exceeded
        }
        self.current_burst_count += 1;

        let entry_id = self.next_id;
        self.next_id += 1;

        // Run pattern DB extraction
        for pat in &self.pattern_db {
            if let Some((k, v)) = pat.extract(message) {
                metadata.insert(k, v);
            }
        }

        let mut entry = JournalLogEntry {
            id: entry_id,
            timestamp_us: entry_id * 10_000,
            facility,
            severity,
            pid,
            uid: 0,
            app_name: app_name.to_string(),
            systemd_unit: format!("{}.service", app_name),
            selinux_context: "system_u:system_r:daemon_t:s0".to_string(),
            boot_id: self.boot_id.clone(),
            message: message.to_string(),
            metadata,
            checksum: 0,
        };

        entry.checksum = entry.calculate_checksum();
        self.entries.push(entry);

        // Check logrotate trigger
        if self.entries.len() >= self.config.max_entries_per_file {
            self.rotate_logs();
        }

        entry_id
    }

    pub fn rotate_logs(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let archive = std::mem::take(&mut self.entries);
        self.rotated_archives.push(archive);

        if self.rotated_archives.len() > self.config.max_rotated_generations {
            self.rotated_archives.remove(0);
        }
    }

    pub fn reset_rate_limit(&mut self) {
        self.current_burst_count = 0;
    }

    pub fn query_journal(
        &self,
        app_name_filter: Option<&str>,
        min_severity: Option<LogSeverity>,
    ) -> Vec<&JournalLogEntry> {
        self.entries
            .iter()
            .filter(|e| {
                if let Some(app) = app_name_filter {
                    if e.app_name != app {
                        return false;
                    }
                }
                if let Some(sev) = min_severity {
                    if e.severity > sev {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn export_json_journal(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|e| e.to_json(&self.hostname))
            .collect()
    }

    pub fn match_bsd_syslog_rules(
        &self,
        facility: SyslogFacility,
        severity: LogSeverity,
    ) -> Vec<String> {
        self.bsd_syslog_rules
            .iter()
            .filter(|r| r.matches(facility, severity))
            .map(|r| r.action_target.clone())
            .collect()
    }
}

// ============================================================================
// 2. DMESG KERNEL RING BUFFER ENGINE
// ============================================================================

/// Timestamp Formatting Mode for `dmesg`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmesgTimestampFormat {
    MicrosecondBoot,
    Iso8601,
    DeltaFromPrev,
    HumanCtime,
}

/// Single Kernel Ring Buffer Record
#[derive(Debug, Clone)]
pub struct DmesgRecord {
    pub sequence: u64,
    pub timestamp_us: u64,
    pub severity: LogSeverity,
    pub facility: SyslogFacility,
    pub subsystem: String,
    pub message: String,
    pub caller_cpu: u32,
}

impl DmesgRecord {
    pub fn format_record(&self, fmt: DmesgTimestampFormat) -> String {
        let ts_str = match fmt {
            DmesgTimestampFormat::MicrosecondBoot => {
                format!("[{:10.6}]", self.timestamp_us as f64 / 1_000_000.0)
            }
            DmesgTimestampFormat::Iso8601 => {
                format!("[2025-02-20T12:00:{:02}Z]", (self.timestamp_us / 1_000_000) % 60)
            }
            DmesgTimestampFormat::DeltaFromPrev => {
                format!("[+{:0.06}]", 0.001234)
            }
            DmesgTimestampFormat::HumanCtime => "[Thu Feb 20 12:00:00 2025]".to_string(),
        };

        format!(
            "{} [{}] <{}> cpu{:02}: {}",
            ts_str,
            self.subsystem,
            self.severity.as_str(),
            self.caller_cpu,
            self.message
        )
    }
}

/// Dmesg Kernel Ring Buffer Engine
#[derive(Debug, Clone)]
pub struct DmesgKernelRingBufferEngine {
    pub buffer: Vec<DmesgRecord>,
    pub max_capacity: usize,
    pub sequence_counter: u64,
    pub console_loglevel: u8, // 1..8 (dmesg -n level)
}

impl DmesgKernelRingBufferEngine {
    pub fn new(capacity: usize) -> Self {
        let mut engine = Self {
            buffer: Vec::new(),
            max_capacity: capacity,
            sequence_counter: 0,
            console_loglevel: 7,
        };
        engine.populate_initial_kernel_boot_sequence();
        engine
    }

    fn populate_initial_kernel_boot_sequence(&mut self) {
        let boot_msgs: &[(&str, LogSeverity, &str, u64, u32)] = &[
            (
                "Linux/BSD Sovereign Microkernel initializing...",
                LogSeverity::Info,
                "kernel",
                0,
                0,
            ),
            (
                "Command line: root=/dev/sda1 ro quiet splash",
                LogSeverity::Info,
                "kernel",
                120,
                0,
            ),
            (
                "BIOS-provided physical RAM map: 64 GB detected",
                LogSeverity::Info,
                "memory",
                1200,
                0,
            ),
            (
                "ACPI: RSDP at 0x000F0240, parsing FADT and MADT tables...",
                LogSeverity::Info,
                "acpi",
                4500,
                1,
            ),
            (
                "SMP: 64 logical CPU cores online",
                LogSeverity::Info,
                "smp",
                12400,
                0,
            ),
            (
                "PCI: Host bridge initialized at 0000:00",
                LogSeverity::Info,
                "pci",
                25000,
                2,
            ),
            (
                "usbcore: registered new interface driver usbfs",
                LogSeverity::Info,
                "usb",
                88000,
                3,
            ),
            (
                "SovereignFS: Mounting root filesystem...",
                LogSeverity::Info,
                "fs",
                150000,
                0,
            ),
            (
                "systemd[1]: Reached target Graphical Interface",
                LogSeverity::Notice,
                "init",
                250000,
                0,
            ),
        ];

        for &(msg, sev, subsys, ts, cpu) in boot_msgs {
            self.sequence_counter += 1;
            self.buffer.push(DmesgRecord {
                sequence: self.sequence_counter,
                timestamp_us: ts,
                severity: sev,
                facility: SyslogFacility::Kernel,
                subsystem: subsys.to_string(),
                message: msg.to_string(),
                caller_cpu: cpu,
            });
        }
    }

    pub fn print_klog(&mut self, subsystem: &str, severity: LogSeverity, message: &str) {
        self.sequence_counter += 1;
        let ts = self.sequence_counter * 1_000;

        if self.buffer.len() >= self.max_capacity {
            self.buffer.remove(0); // Evict oldest entry in ring buffer
        }

        self.buffer.push(DmesgRecord {
            sequence: self.sequence_counter,
            timestamp_us: ts,
            severity,
            facility: SyslogFacility::Kernel,
            subsystem: subsystem.to_string(),
            message: message.to_string(),
            caller_cpu: 0,
        });
    }

    pub fn filter(
        &self,
        min_severity: Option<LogSeverity>,
        subsystem: Option<&str>,
    ) -> Vec<&DmesgRecord> {
        self.buffer
            .iter()
            .filter(|rec| {
                if let Some(sev) = min_severity {
                    if rec.severity > sev {
                        return false;
                    }
                }
                if let Some(sub) = subsystem {
                    if rec.subsystem != sub {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn set_console_loglevel(&mut self, level: u8) {
        self.console_loglevel = level;
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

// ============================================================================
// 3. TOP & HTOP PROCESS & CPU CORE MONITOR ENGINE
// ============================================================================

/// Extended Process Execution State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    DiskSleep,
    Stopped,
    Zombie,
    IdleKernelThread,
}

impl ProcessState {
    pub fn as_char(&self) -> char {
        match self {
            ProcessState::Running => 'R',
            ProcessState::Sleeping => 'S',
            ProcessState::DiskSleep => 'D',
            ProcessState::Stopped => 'T',
            ProcessState::Zombie => 'Z',
            ProcessState::IdleKernelThread => 'I',
        }
    }
}

/// Process Signal inspired by Linux / POSIX
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProcessSignal {
    Sighup = 1,
    Sigint = 2,
    Sigkill = 9,
    Sigusr1 = 10,
    Sigusr2 = 12,
    Sigterm = 15,
    Sigstop = 19,
    Sigcont = 18,
}

/// Process Activity Telemetry Record (`top`/`htop`)
#[derive(Debug, Clone)]
pub struct ProcessTelemetry {
    pub pid: u32,
    pub ppid: u32,
    pub user: String,
    pub name: String,
    pub state: ProcessState,
    pub nice: i8,
    pub priority: i32,
    pub cpu_usage_pct: f32,
    pub mem_usage_pct: f32,
    pub virt_mem_kb: u64,
    pub res_mem_kb: u64,
    pub num_threads: u32,
    pub tree_depth: u32,
    pub read_bytes_sec: u64,
    pub write_bytes_sec: u64,
}

/// Per-CPU Core Utilization Breakdown
#[derive(Debug, Clone)]
pub struct CpuCoreTelemetry {
    pub core_id: u32,
    pub user_pct: f32,
    pub sys_pct: f32,
    pub iowait_pct: f32,
    pub irq_pct: f32,
    pub softirq_pct: f32,
    pub steal_pct: f32,
    pub idle_pct: f32,
}

/// Top & Htop Process Telemetry Monitor
#[derive(Debug, Clone)]
pub struct TopHtopProcessMonitorEngine {
    pub processes: BTreeMap<u32, ProcessTelemetry>,
    pub per_cpu_cores: Vec<CpuCoreTelemetry>,
    pub total_cpu_user_pct: f32,
    pub total_cpu_sys_pct: f32,
    pub total_cpu_iowait_pct: f32,
    pub total_cpu_idle_pct: f32,
}

impl TopHtopProcessMonitorEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            processes: BTreeMap::new(),
            per_cpu_cores: Vec::new(),
            total_cpu_user_pct: 12.5,
            total_cpu_sys_pct: 3.2,
            total_cpu_iowait_pct: 0.5,
            total_cpu_idle_pct: 83.8,
        };
        engine.populate_sample_data();
        engine
    }

    fn populate_sample_data(&mut self) {
        for i in 0..4 {
            self.per_cpu_cores.push(CpuCoreTelemetry {
                core_id: i,
                user_pct: 10.0 + i as f32 * 2.0,
                sys_pct: 3.0,
                iowait_pct: 0.5,
                irq_pct: 0.1,
                softirq_pct: 0.2,
                steal_pct: 0.0,
                idle_pct: 86.2 - i as f32 * 2.0,
            });
        }

        let samples = vec![
            ProcessTelemetry {
                pid: 1,
                ppid: 0,
                user: "root".to_string(),
                name: "systemd".to_string(),
                state: ProcessState::Sleeping,
                nice: 0,
                priority: 20,
                cpu_usage_pct: 0.1,
                mem_usage_pct: 0.2,
                virt_mem_kb: 168000,
                res_mem_kb: 12400,
                num_threads: 1,
                tree_depth: 0,
                read_bytes_sec: 1024,
                write_bytes_sec: 512,
            },
            ProcessTelemetry {
                pid: 102,
                ppid: 1,
                user: "root".to_string(),
                name: "journald".to_string(),
                state: ProcessState::Sleeping,
                nice: -5,
                priority: 15,
                cpu_usage_pct: 0.5,
                mem_usage_pct: 0.4,
                virt_mem_kb: 84000,
                res_mem_kb: 8100,
                num_threads: 2,
                tree_depth: 1,
                read_bytes_sec: 4096,
                write_bytes_sec: 16384,
            },
            ProcessTelemetry {
                pid: 540,
                ppid: 1,
                user: "sigma".to_string(),
                name: "zenith-desktop".to_string(),
                state: ProcessState::Running,
                nice: 0,
                priority: 20,
                cpu_usage_pct: 4.8,
                mem_usage_pct: 2.5,
                virt_mem_kb: 512000,
                res_mem_kb: 164000,
                num_threads: 8,
                tree_depth: 1,
                read_bytes_sec: 32768,
                write_bytes_sec: 8192,
            },
        ];

        for proc in samples {
            self.processes.insert(proc.pid, proc);
        }
    }

    pub fn update_process(&mut self, proc: ProcessTelemetry) {
        self.processes.insert(proc.pid, proc);
    }

    pub fn sort_by_cpu(&self) -> Vec<&ProcessTelemetry> {
        let mut list: Vec<&ProcessTelemetry> = self.processes.values().collect();
        list.sort_by(|a, b| b.cpu_usage_pct.partial_cmp(&a.cpu_usage_pct).unwrap());
        list
    }

    pub fn sort_by_mem(&self) -> Vec<&ProcessTelemetry> {
        let mut list: Vec<&ProcessTelemetry> = self.processes.values().collect();
        list.sort_by(|a, b| b.mem_usage_pct.partial_cmp(&a.mem_usage_pct).unwrap());
        list
    }

    pub fn sort_by_io(&self) -> Vec<&ProcessTelemetry> {
        let mut list: Vec<&ProcessTelemetry> = self.processes.values().collect();
        list.sort_by(|a, b| {
            (b.read_bytes_sec + b.write_bytes_sec)
                .cmp(&(a.read_bytes_sec + a.write_bytes_sec))
        });
        list
    }

    pub fn build_process_tree(&self) -> HashMap<u32, Vec<u32>> {
        let mut tree: HashMap<u32, Vec<u32>> = HashMap::new();
        for proc in self.processes.values() {
            tree.entry(proc.ppid).or_default().push(proc.pid);
        }
        tree
    }

    pub fn send_signal(&mut self, pid: u32, signal: ProcessSignal) -> bool {
        if let Some(proc) = self.processes.get_mut(&pid) {
            match signal {
                ProcessSignal::Sigkill | ProcessSignal::Sigterm => {
                    proc.state = ProcessState::Stopped;
                }
                ProcessSignal::Sigstop => {
                    proc.state = ProcessState::Stopped;
                }
                ProcessSignal::Sigcont => {
                    proc.state = ProcessState::Running;
                }
                _ => {}
            }
            return true;
        }
        false
    }
}

impl Default for TopHtopProcessMonitorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. VMSTAT / IOSTAT / SAR METRICS COLLECTOR ENGINE
// ============================================================================

/// Vmstat System Memory & CPU Activity Snapshot
#[derive(Debug, Clone, Copy)]
pub struct VmstatSnapshot {
    pub timestamp_sec: u64,
    pub procs_running: u32,
    pub procs_blocked: u32,
    pub swpd_kb: u64,
    pub free_kb: u64,
    pub active_kb: u64,
    pub inactive_kb: u64,
    pub wired_kb: u64,
    pub buff_kb: u64,
    pub cache_kb: u64,
    pub slab_kb: u64,
    pub swap_in_kb_s: u32,
    pub swap_out_kb_s: u32,
    pub blocks_in_s: u32,
    pub blocks_out_s: u32,
    pub interrupts_s: u32,
    pub context_switches_s: u32,
    pub cpu_user_pct: u8,
    pub cpu_sys_pct: u8,
    pub cpu_idle_pct: u8,
    pub cpu_wait_pct: u8,
    pub cpu_steal_pct: u8,
}

/// Extended Iostat Block Device I/O Statistics
#[derive(Debug, Clone)]
pub struct IostatDeviceSnapshot {
    pub device_name: String,
    pub tps: f32,
    pub read_kb_sec: f32,
    pub write_kb_sec: f32,
    pub rrqm_sec: f32,
    pub wrqm_sec: f32,
    pub r_await_ms: f32,
    pub w_await_ms: f32,
    pub avg_await_ms: f32,
    pub avg_queue_size: f32,
    pub util_pct: f32,
}

/// Daily Historical `saDD` Dataset Buffer (`sysstat` / `sar`)
#[derive(Debug, Clone)]
pub struct SarDailyDataset {
    pub day_number: u8,
    pub snapshots: Vec<VmstatSnapshot>,
}

/// Sar Time-Series Historical Performance Buffer (`sysstat`)
#[derive(Debug, Clone)]
pub struct VmstatIostatSarMetricsCollectorEngine {
    pub vmstat_history: Vec<VmstatSnapshot>,
    pub iostat_history: Vec<IostatDeviceSnapshot>,
    pub sar_daily_datasets: BTreeMap<u8, SarDailyDataset>,
    pub max_history_capacity: usize,
}

impl VmstatIostatSarMetricsCollectorEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            vmstat_history: Vec::new(),
            iostat_history: Vec::new(),
            sar_daily_datasets: BTreeMap::new(),
            max_history_capacity: 1000,
        };
        engine.populate_initial_snapshots();
        engine
    }

    fn populate_initial_snapshots(&mut self) {
        let snap1 = VmstatSnapshot {
            timestamp_sec: 1700000000,
            procs_running: 2,
            procs_blocked: 0,
            swpd_kb: 0,
            free_kb: 16384000,
            active_kb: 8192000,
            inactive_kb: 4096000,
            wired_kb: 2048000,
            buff_kb: 512000,
            cache_kb: 12288000,
            slab_kb: 1024000,
            swap_in_kb_s: 0,
            swap_out_kb_s: 0,
            blocks_in_s: 128,
            blocks_out_s: 256,
            interrupts_s: 1450,
            context_switches_s: 3200,
            cpu_user_pct: 12,
            cpu_sys_pct: 3,
            cpu_idle_pct: 84,
            cpu_wait_pct: 1,
            cpu_steal_pct: 0,
        };

        self.vmstat_history.push(snap1);

        self.iostat_history.push(IostatDeviceSnapshot {
            device_name: "sda".to_string(),
            tps: 45.2,
            read_kb_sec: 1024.0,
            write_kb_sec: 2048.0,
            rrqm_sec: 2.1,
            wrqm_sec: 12.5,
            r_await_ms: 1.2,
            w_await_ms: 2.5,
            avg_await_ms: 1.85,
            avg_queue_size: 0.15,
            util_pct: 8.2,
        });

        self.iostat_history.push(IostatDeviceSnapshot {
            device_name: "nvme0n1".to_string(),
            tps: 1250.0,
            read_kb_sec: 32768.0,
            write_kb_sec: 16384.0,
            rrqm_sec: 0.0,
            wrqm_sec: 0.0,
            r_await_ms: 0.12,
            w_await_ms: 0.18,
            avg_await_ms: 0.15,
            avg_queue_size: 0.08,
            util_pct: 14.5,
        });

        let mut sa20 = SarDailyDataset {
            day_number: 20,
            snapshots: Vec::new(),
        };
        sa20.snapshots.push(snap1);
        self.sar_daily_datasets.insert(20, sa20);
    }

    pub fn record_vmstat(&mut self, snap: VmstatSnapshot) {
        if self.vmstat_history.len() >= self.max_history_capacity {
            self.vmstat_history.remove(0);
        }
        self.vmstat_history.push(snap);
    }

    pub fn record_iostat(&mut self, snap: IostatDeviceSnapshot) {
        if let Some(existing) = self.iostat_history.iter_mut().find(|i| i.device_name == snap.device_name) {
            *existing = snap;
        } else {
            self.iostat_history.push(snap);
        }
    }

    pub fn calculate_sar_average_cpu(&self) -> (f32, f32, f32) {
        if self.vmstat_history.is_empty() {
            return (0.0, 0.0, 100.0);
        }

        let total = self.vmstat_history.len() as f32;
        let sum_user: f32 = self.vmstat_history.iter().map(|s| s.cpu_user_pct as f32).sum();
        let sum_sys: f32 = self.vmstat_history.iter().map(|s| s.cpu_sys_pct as f32).sum();
        let sum_idle: f32 = self.vmstat_history.iter().map(|s| s.cpu_idle_pct as f32).sum();

        (sum_user / total, sum_sys / total, sum_idle / total)
    }

    pub fn query_time_range(&self, start_sec: u64, end_sec: u64) -> Vec<&VmstatSnapshot> {
        self.vmstat_history
            .iter()
            .filter(|s| s.timestamp_sec >= start_sec && s.timestamp_sec <= end_sec)
            .collect()
    }
}

impl Default for VmstatIostatSarMetricsCollectorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. LSOF OPEN FILES & SOCKET INSPECTOR ENGINE
// ============================================================================

/// Open File Descriptor Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdType {
    Regular,
    Directory,
    Character,
    Block,
    Fifo,
    UnixSocket,
    IPv4Socket,
    IPv6Socket,
    Kqueue,
    Inotify,
}

/// Open File Access Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdAccessMode {
    Read,
    Write,
    ReadWrite,
}

/// Network TCP Connection State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    Listen,
    Established,
    TimeWait,
    CloseWait,
    SynSent,
    None,
}

/// Open File Record (`lsof`)
#[derive(Debug, Clone)]
pub struct OpenFileRecord {
    pub pid: u32,
    pub process_name: String,
    pub fd: u32,
    pub fd_type: FdType,
    pub access_mode: FdAccessMode,
    pub inode: u64,
    pub offset_bytes: u64,
    pub size_bytes: u64,
    pub path_or_endpoint: String,
    pub tcp_state: TcpState,
}

/// Lsof Open File & Socket Inspector Engine
#[derive(Debug, Clone)]
pub struct LsofOpenFilesInspectorEngine {
    pub open_files: Vec<OpenFileRecord>,
}

impl LsofOpenFilesInspectorEngine {
    pub fn new() -> Self {
        let mut inspector = Self {
            open_files: Vec::new(),
        };
        inspector.populate_sample_files();
        inspector
    }

    fn populate_sample_files(&mut self) {
        let samples = vec![
            OpenFileRecord {
                pid: 1,
                process_name: "systemd".to_string(),
                fd: 0,
                fd_type: FdType::Character,
                access_mode: FdAccessMode::ReadWrite,
                inode: 12,
                offset_bytes: 0,
                size_bytes: 0,
                path_or_endpoint: "/dev/null".to_string(),
                tcp_state: TcpState::None,
            },
            OpenFileRecord {
                pid: 102,
                process_name: "journald".to_string(),
                fd: 4,
                fd_type: FdType::Regular,
                access_mode: FdAccessMode::Write,
                inode: 55420,
                offset_bytes: 1048576,
                size_bytes: 4194304,
                path_or_endpoint: "/var/log/journal/system.journal".to_string(),
                tcp_state: TcpState::None,
            },
            OpenFileRecord {
                pid: 540,
                process_name: "zenith-desktop".to_string(),
                fd: 12,
                fd_type: FdType::UnixSocket,
                access_mode: FdAccessMode::ReadWrite,
                inode: 98124,
                offset_bytes: 0,
                size_bytes: 0,
                path_or_endpoint: "@wayland-0".to_string(),
                tcp_state: TcpState::None,
            },
            OpenFileRecord {
                pid: 880,
                process_name: "sshd".to_string(),
                fd: 3,
                fd_type: FdType::IPv4Socket,
                access_mode: FdAccessMode::ReadWrite,
                inode: 10420,
                offset_bytes: 0,
                size_bytes: 0,
                path_or_endpoint: "0.0.0.0:22".to_string(),
                tcp_state: TcpState::Listen,
            },
        ];

        for f in samples {
            self.open_files.push(f);
        }
    }

    pub fn find_by_pid(&self, pid: u32) -> Vec<&OpenFileRecord> {
        self.open_files.iter().filter(|f| f.pid == pid).collect()
    }

    pub fn find_by_path(&self, substring: &str) -> Vec<&OpenFileRecord> {
        self.open_files
            .iter()
            .filter(|f| f.path_or_endpoint.contains(substring))
            .collect()
    }

    pub fn find_by_type(&self, fd_type: FdType) -> Vec<&OpenFileRecord> {
        self.open_files
            .iter()
            .filter(|f| f.fd_type == fd_type)
            .collect()
    }

    pub fn find_by_tcp_state(&self, state: TcpState) -> Vec<&OpenFileRecord> {
        self.open_files
            .iter()
            .filter(|f| f.tcp_state == state)
            .collect()
    }
}

impl Default for LsofOpenFilesInspectorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. HARDWARE DISCOVERY INSPECTOR ENGINE (LSPCI / LSUSB / DMIDECODE / LSSCSI)
// ============================================================================

/// PCI / PCIe Device Record (`lspci`)
#[derive(Debug, Clone)]
pub struct PciDeviceRecord {
    pub domain_bus_dev_fn: String, // e.g. "0000:00:02.0"
    pub vendor_id: u16,
    pub vendor_name: String,
    pub device_id: u16,
    pub device_name: String,
    pub class_code_desc: String,
    pub pcie_link_gen: String,  // e.g., "Gen4 x16"
    pub pcie_link_width: String,
    pub bars: Vec<String>,
}

/// USB Device Record (`lsusb`)
#[derive(Debug, Clone)]
pub struct UsbDeviceRecord {
    pub bus_dev_address: String, // e.g. "Bus 001 Dev 002"
    pub vendor_id: u16,
    pub vendor_name: String,
    pub product_id: u16,
    pub product_name: String,
    pub speed_mode: String, // e.g. "High-Speed 480Mbps", "SuperSpeed 5Gbps"
    pub bound_driver: String,
}

/// RAM Memory Device (`dmidecode -t memory`)
#[derive(Debug, Clone)]
pub struct DmiRamDevice {
    pub slot_locator: String,
    pub capacity_mb: u64,
    pub speed_mhz: u32,
    pub memory_type: String, // DDR4, DDR5
    pub manufacturer: String,
}

/// SMBIOS / DMI System Information (`dmidecode`)
#[derive(Debug, Clone)]
pub struct DmiDecodeInfo {
    pub bios_vendor: String,
    pub bios_version: String,
    pub sys_manufacturer: String,
    pub sys_product_name: String,
    pub sys_serial_number: String,
    pub cpu_model: String,
    pub cpu_cores: u32,
    pub ram_devices: Vec<DmiRamDevice>,
}

/// SCSI / SATA Target Device (`lsscsi`)
#[derive(Debug, Clone)]
pub struct ScsiDeviceRecord {
    pub scsi_address: String, // e.g. "[0:0:0:0]"
    pub dev_type: String,     // disk, cd/dvd, tape
    pub vendor: String,
    pub model: String,
    pub revision: String,
    pub dev_node: String,     // /dev/sda
}

/// Hardware Discovery Inspector Engine (`lspci`, `lsusb`, `dmidecode`, `lsscsi`)
#[derive(Debug, Clone)]
pub struct HardwareDiscoveryInspectorEngine {
    pub pci_devices: Vec<PciDeviceRecord>,
    pub usb_devices: Vec<UsbDeviceRecord>,
    pub dmi_info: DmiDecodeInfo,
    pub scsi_devices: Vec<ScsiDeviceRecord>,
}

impl HardwareDiscoveryInspectorEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            pci_devices: Vec::new(),
            usb_devices: Vec::new(),
            dmi_info: DmiDecodeInfo {
                bios_vendor: "American Megatrends Inc.".to_string(),
                bios_version: "2.40".to_string(),
                sys_manufacturer: "SigmaWorkstation".to_string(),
                sys_product_name: "Pro-Studio-X64".to_string(),
                sys_serial_number: "SIGMA-9988-7766".to_string(),
                cpu_model: "AMD EPYC 9654 96-Core Processor".to_string(),
                cpu_cores: 96,
                ram_devices: Vec::new(),
            },
            scsi_devices: Vec::new(),
        };
        engine.populate_hardware_tables();
        engine
    }

    fn populate_hardware_tables(&mut self) {
        self.pci_devices.push(PciDeviceRecord {
            domain_bus_dev_fn: "0000:00:00.0".to_string(),
            vendor_id: 0x8086,
            vendor_name: "Intel Corporation".to_string(),
            device_id: 0x9b43,
            device_name: "Host bridge".to_string(),
            class_code_desc: "Bridge - Host bridge".to_string(),
            pcie_link_gen: "Gen4".to_string(),
            pcie_link_width: "x16".to_string(),
            bars: vec!["0x00000000".to_string()],
        });

        self.pci_devices.push(PciDeviceRecord {
            domain_bus_dev_fn: "0000:01:00.0".to_string(),
            vendor_id: 0x10de,
            vendor_name: "NVIDIA Corporation".to_string(),
            device_id: 0x2684,
            device_name: "AD102 [GeForce RTX 4090]".to_string(),
            class_code_desc: "VGA compatible controller".to_string(),
            pcie_link_gen: "Gen4".to_string(),
            pcie_link_width: "x16".to_string(),
            bars: vec!["0xde000000".to_string(), "0xc0000000".to_string()],
        });

        self.usb_devices.push(UsbDeviceRecord {
            bus_dev_address: "Bus 001 Dev 002".to_string(),
            vendor_id: 0x046d,
            vendor_name: "Logitech, Inc.".to_string(),
            product_id: 0xc52b,
            product_name: "Unifying Receiver".to_string(),
            speed_mode: "Full-Speed 12Mbps".to_string(),
            bound_driver: "usbhid".to_string(),
        });

        self.dmi_info.ram_devices.push(DmiRamDevice {
            slot_locator: "DIMM_A1".to_string(),
            capacity_mb: 32768,
            speed_mhz: 5600,
            memory_type: "DDR5".to_string(),
            manufacturer: "Kingston".to_string(),
        });

        self.scsi_devices.push(ScsiDeviceRecord {
            scsi_address: "[0:0:0:0]".to_string(),
            dev_type: "disk".to_string(),
            vendor: "Samsung".to_string(),
            model: "SSD 980 PRO 2TB".to_string(),
            revision: "5B2Q".to_string(),
            dev_node: "/dev/sda".to_string(),
        });
    }

    pub fn find_pci_by_class(&self, class_desc: &str) -> Vec<&PciDeviceRecord> {
        self.pci_devices
            .iter()
            .filter(|p| p.class_code_desc.to_lowercase().contains(&class_desc.to_lowercase()))
            .collect()
    }

    pub fn find_usb_by_speed(&self, speed: &str) -> Vec<&UsbDeviceRecord> {
        self.usb_devices
            .iter()
            .filter(|u| u.speed_mode.to_lowercase().contains(&speed.to_lowercase()))
            .collect()
    }

    pub fn find_scsi_by_dev(&self, dev_node: &str) -> Option<&ScsiDeviceRecord> {
        self.scsi_devices.iter().find(|s| s.dev_node == dev_node)
    }
}

impl Default for HardwareDiscoveryInspectorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. SOSREPORT & TELEMETRY EXPORTERS (PROMETHEUS & COLLECTD)
// ============================================================================

/// Sosreport Diagnostic Archive Bundle Manifest
#[derive(Debug, Clone)]
pub struct SosreportArchiveBundle {
    pub report_id: String,
    pub timestamp_sec: u64,
    pub hostname: String,
    pub os_release: String,
    pub kernel_version: String,
    pub captured_files: BTreeMap<String, String>,
    pub sanitized_items_count: u32,
    pub tar_zst_manifest: Vec<String>,
}

/// Sosreport & Prometheus Node Exporter Engine
#[derive(Debug, Clone)]
pub struct SosreportPrometheusExporterEngine {
    pub hostname: String,
    pub anonymize_ip_mac_enabled: bool,
}

impl SosreportPrometheusExporterEngine {
    pub fn new(hostname: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            anonymize_ip_mac_enabled: true,
        }
    }

    /// Sanitize sensitive credentials and tokens in diagnostic string
    pub fn sanitize_sensitive_data(&self, text: &str) -> (String, u32) {
        let mut sanitized = text.to_string();
        let mut count = 0;

        let sensitive_keywords = ["password=", "secret=", "bearer ", "api_key="];
        for kw in &sensitive_keywords {
            if sanitized.contains(kw) {
                sanitized = sanitized.replace(kw, "[REDACTED_SECRET]=");
                count += 1;
            }
        }
        (sanitized, count)
    }

    /// Generate an automated `sosreport` diagnostic archive bundle
    pub fn generate_sosreport(
        &self,
        journal: &JournaldRsyslogLogEngine,
        dmesg: &DmesgKernelRingBufferEngine,
        hw: &HardwareDiscoveryInspectorEngine,
    ) -> SosreportArchiveBundle {
        let mut files = BTreeMap::new();
        let mut total_sanitized = 0;

        // Include dmesg log
        let mut dmesg_txt = String::new();
        for rec in &dmesg.buffer {
            dmesg_txt.push_str(&rec.format_record(DmesgTimestampFormat::MicrosecondBoot));
            dmesg_txt.push('\n');
        }
        let (clean_dmesg, c1) = self.sanitize_sensitive_data(&dmesg_txt);
        total_sanitized += c1;
        files.insert("var/log/dmesg".to_string(), clean_dmesg);

        // Include syslog / journal log
        let mut syslog_txt = String::new();
        for entry in &journal.entries {
            syslog_txt.push_str(&entry.to_rfc5424(&self.hostname));
            syslog_txt.push('\n');
        }
        let (clean_syslog, c2) = self.sanitize_sensitive_data(&syslog_txt);
        total_sanitized += c2;
        files.insert("var/log/syslog".to_string(), clean_syslog);

        // Include DMI / hardware discovery
        let dmi_txt = format!(
            "BIOS Vendor: {}\nBIOS Version: {}\nSystem: {} {}\nSerial: {}\nCPU: {} ({} cores)\n",
            hw.dmi_info.bios_vendor,
            hw.dmi_info.bios_version,
            hw.dmi_info.sys_manufacturer,
            hw.dmi_info.sys_product_name,
            hw.dmi_info.sys_serial_number,
            hw.dmi_info.cpu_model,
            hw.dmi_info.cpu_cores
        );
        files.insert("sys/class/dmi/id/bios".to_string(), dmi_txt);

        let mut manifest = Vec::new();
        for k in files.keys() {
            manifest.push(format!("sosreport-{}/{}", self.hostname, k));
        }

        SosreportArchiveBundle {
            report_id: format!("sosreport-{}-202502", self.hostname),
            timestamp_sec: 1700000000,
            hostname: self.hostname.clone(),
            os_release: "SigmaOS 1.0 (Sovereign)".to_string(),
            kernel_version: "6.12.0-sigmaos".to_string(),
            captured_files: files,
            sanitized_items_count: total_sanitized,
            tar_zst_manifest: manifest,
        }
    }

    /// Export metrics in Prometheus `node_exporter` exposition text format
    pub fn export_prometheus_metrics(
        &self,
        top: &TopHtopProcessMonitorEngine,
        vmstat: &VmstatIostatSarMetricsCollectorEngine,
    ) -> String {
        let mut out = String::new();
        out.push_str("# HELP node_cpu_seconds_total Seconds the CPUs spent in each mode.\n");
        out.push_str("# TYPE node_cpu_seconds_total counter\n");
        out.push_str(&format!(
            "node_cpu_seconds_total{{mode=\"user\"}} {}\n",
            top.total_cpu_user_pct
        ));
        out.push_str(&format!(
            "node_cpu_seconds_total{{mode=\"system\"}} {}\n",
            top.total_cpu_sys_pct
        ));
        out.push_str(&format!(
            "node_cpu_seconds_total{{mode=\"iowait\"}} {}\n",
            top.total_cpu_iowait_pct
        ));
        out.push_str(&format!(
            "node_cpu_seconds_total{{mode=\"idle\"}} {}\n",
            top.total_cpu_idle_pct
        ));

        if let Some(vm) = vmstat.vmstat_history.last() {
            out.push_str("# HELP node_memory_MemTotal_bytes Memory information field MemTotal_bytes.\n");
            out.push_str("# TYPE node_memory_MemTotal_bytes gauge\n");
            out.push_str(&format!("node_memory_MemTotal_bytes {}\n", (vm.free_kb + vm.active_kb + vm.inactive_kb) * 1024));

            out.push_str("# HELP node_memory_MemFree_bytes Memory information field MemFree_bytes.\n");
            out.push_str("# TYPE node_memory_MemFree_bytes gauge\n");
            out.push_str(&format!("node_memory_MemFree_bytes {}\n", vm.free_kb * 1024));

            out.push_str("# HELP node_memory_Cached_bytes Memory information field Cached_bytes.\n");
            out.push_str("# TYPE node_memory_Cached_bytes gauge\n");
            out.push_str(&format!("node_memory_Cached_bytes {}\n", vm.cache_kb * 1024));

            out.push_str("# HELP node_context_switches_total Total number of context switches.\n");
            out.push_str("# TYPE node_context_switches_total counter\n");
            out.push_str(&format!("node_context_switches_total {}\n", vm.context_switches_s));
        }

        out
    }

    /// Export metrics in `collectd` JSON protocol format
    pub fn export_collectd_metrics(
        &self,
        top: &TopHtopProcessMonitorEngine,
        vmstat: &VmstatIostatSarMetricsCollectorEngine,
    ) -> String {
        let mut json = String::from("{\"collectd\":[");
        json.push_str(&format!(
            "{{\"host\":\"{}\",\"plugin\":\"cpu\",\"type\":\"percent\",\"type_instance\":\"user\",\"values\":[{}]}},",
            self.hostname, top.total_cpu_user_pct
        ));
        json.push_str(&format!(
            "{{\"host\":\"{}\",\"plugin\":\"cpu\",\"type\":\"percent\",\"type_instance\":\"system\",\"values\":[{}]}}",
            self.hostname, top.total_cpu_sys_pct
        ));

        if let Some(vm) = vmstat.vmstat_history.last() {
            json.push_str(&format!(
                ",{{\"host\":\"{}\",\"plugin\":\"memory\",\"type\":\"memory\",\"type_instance\":\"free\",\"values\":[{}]}}",
                self.hostname, vm.free_kb * 1024
            ));
        }
        json.push_str("]}");
        json
    }
}

// ============================================================================
// MASTER LOGGING, MONITORING & DIAGNOSTICS SUITE
// ============================================================================

/// Master Sovereign Linux & BSD Logging, Monitoring, and Diagnostics Suite
#[derive(Debug, Clone)]
pub struct SovereignLinuxBsdLoggingMonitoringDiagnosticsSuite {
    pub journal: JournaldRsyslogLogEngine,
    pub dmesg: DmesgKernelRingBufferEngine,
    pub top_monitor: TopHtopProcessMonitorEngine,
    pub vmstat_collector: VmstatIostatSarMetricsCollectorEngine,
    pub lsof_inspector: LsofOpenFilesInspectorEngine,
    pub hardware: HardwareDiscoveryInspectorEngine,
    pub exporter: SosreportPrometheusExporterEngine,
}

impl SovereignLinuxBsdLoggingMonitoringDiagnosticsSuite {
    pub fn new(hostname: &str) -> Self {
        Self {
            journal: JournaldRsyslogLogEngine::new(hostname),
            dmesg: DmesgKernelRingBufferEngine::new(1000),
            top_monitor: TopHtopProcessMonitorEngine::new(),
            vmstat_collector: VmstatIostatSarMetricsCollectorEngine::new(),
            lsof_inspector: LsofOpenFilesInspectorEngine::new(),
            hardware: HardwareDiscoveryInspectorEngine::new(),
            exporter: SosreportPrometheusExporterEngine::new(hostname),
        }
    }

    /// Run full system health diagnostic check
    pub fn run_system_diagnostic(&self) -> HashMap<String, String> {
        let mut diag = HashMap::new();
        diag.insert("hostname".to_string(), self.journal.hostname.clone());
        diag.insert(
            "journal_entries".to_string(),
            self.journal.entries.len().to_string(),
        );
        diag.insert(
            "dmesg_records".to_string(),
            self.dmesg.buffer.len().to_string(),
        );
        diag.insert(
            "monitored_processes".to_string(),
            self.top_monitor.processes.len().to_string(),
        );
        diag.insert(
            "open_file_descriptors".to_string(),
            self.lsof_inspector.open_files.len().to_string(),
        );
        diag.insert(
            "pci_devices_count".to_string(),
            self.hardware.pci_devices.len().to_string(),
        );
        diag.insert(
            "usb_devices_count".to_string(),
            self.hardware.usb_devices.len().to_string(),
        );
        diag.insert(
            "bios_vendor".to_string(),
            self.hardware.dmi_info.bios_vendor.clone(),
        );
        diag.insert(
            "scsi_devices_count".to_string(),
            self.hardware.scsi_devices.len().to_string(),
        );
        diag
    }

    /// Export Prometheus and Collectd telemetry metrics payload
    pub fn export_all_telemetry(&self) -> String {
        let prom = self.exporter.export_prometheus_metrics(&self.top_monitor, &self.vmstat_collector);
        let collectd = self.exporter.export_collectd_metrics(&self.top_monitor, &self.vmstat_collector);
        format!("--- PROMETHEUS METRICS ---\n{}\n--- COLLECTD METRICS ---\n{}\n", prom, collectd)
    }
}

// ============================================================================
// STANDALONE UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journald_rsyslog_log_engine() {
        let mut journal = JournaldRsyslogLogEngine::new("sigma-node-1");
        let mut meta = BTreeMap::new();
        meta.insert("subsystem".to_string(), "kernel".to_string());

        let id = journal.log(
            SyslogFacility::Kernel,
            LogSeverity::Error,
            1,
            "kernel",
            "Out of memory: Kill process 1024",
            meta,
        );

        assert_eq!(id, 1);
        assert_eq!(journal.entries.len(), 1);

        let entry = &journal.entries[0];
        assert!(entry.verify_checksum());

        let rfc5424 = entry.to_rfc5424(&journal.hostname);
        assert!(rfc5424.contains("sigma-node-1"));
        assert!(rfc5424.contains("Out of memory"));

        let json = entry.to_json(&journal.hostname);
        assert!(json.contains("PRIORITY"));
        assert!(json.contains("killed_pid"));

        let queried = journal.query_journal(Some("kernel"), Some(LogSeverity::Error));
        assert_eq!(queried.len(), 1);

        let bsd_targets = journal.match_bsd_syslog_rules(SyslogFacility::AuthPriv, LogSeverity::Info);
        assert!(bsd_targets.contains(&"/var/log/secure".to_string()));
    }

    #[test]
    fn test_dmesg_ring_buffer() {
        let mut dmesg = DmesgKernelRingBufferEngine::new(10);
        assert!(!dmesg.buffer.is_empty());

        dmesg.print_klog(
            "usb",
            LogSeverity::Warning,
            "Overcurrent condition on port 1",
        );
        let filtered = dmesg.filter(Some(LogSeverity::Warning), Some("usb"));
        assert!(!filtered.is_empty());

        let formatted = filtered[0].format_record(DmesgTimestampFormat::MicrosecondBoot);
        assert!(formatted.contains("[usb]"));
    }

    #[test]
    fn test_top_htop_process_monitor() {
        let mut monitor = TopHtopProcessMonitorEngine::new();
        assert!(!monitor.processes.is_empty());
        assert_eq!(monitor.per_cpu_cores.len(), 4);

        let top_cpu = monitor.sort_by_cpu();
        assert!(top_cpu[0].cpu_usage_pct >= top_cpu[1].cpu_usage_pct);

        let tree = monitor.build_process_tree();
        assert!(tree.contains_key(&1));

        let signaled = monitor.send_signal(540, ProcessSignal::Sigstop);
        assert!(signaled);
        assert_eq!(monitor.processes.get(&540).unwrap().state, ProcessState::Stopped);
    }

    #[test]
    fn test_vmstat_iostat_sar() {
        let collector = VmstatIostatSarMetricsCollectorEngine::new();
        assert!(!collector.vmstat_history.is_empty());
        assert!(!collector.iostat_history.is_empty());
        assert!(collector.sar_daily_datasets.contains_key(&20));

        let (user, sys, idle) = collector.calculate_sar_average_cpu();
        assert!(user > 0.0);
        assert!(sys >= 0.0);
        assert!(idle > 0.0);

        let range = collector.query_time_range(1600000000, 1800000000);
        assert_eq!(range.len(), 1);
    }

    #[test]
    fn test_lsof_inspector() {
        let lsof = LsofOpenFilesInspectorEngine::new();
        let sys_files = lsof.find_by_pid(1);
        assert_eq!(sys_files.len(), 1);

        let socket_files = lsof.find_by_path("@wayland-0");
        assert_eq!(socket_files.len(), 1);

        let tcp_listens = lsof.find_by_tcp_state(TcpState::Listen);
        assert_eq!(tcp_listens.len(), 1);
    }

    #[test]
    fn test_hardware_discovery() {
        let hw = HardwareDiscoveryInspectorEngine::new();
        assert_eq!(hw.pci_devices.len(), 2);
        assert_eq!(hw.usb_devices.len(), 1);
        assert_eq!(hw.scsi_devices.len(), 1);
        assert_eq!(hw.dmi_info.bios_vendor, "American Megatrends Inc.");

        let gpus = hw.find_pci_by_class("VGA");
        assert_eq!(gpus.len(), 1);

        let scsi_disk = hw.find_scsi_by_dev("/dev/sda");
        assert!(scsi_disk.is_some());
    }

    #[test]
    fn test_sosreport_and_prometheus_export() {
        let suite = SovereignLinuxBsdLoggingMonitoringDiagnosticsSuite::new("test-host");
        let sos = suite
            .exporter
            .generate_sosreport(&suite.journal, &suite.dmesg, &suite.hardware);
        assert!(sos.captured_files.contains_key("var/log/dmesg"));
        assert!(!sos.tar_zst_manifest.is_empty());

        let prom = suite
            .exporter
            .export_prometheus_metrics(&suite.top_monitor, &suite.vmstat_collector);
        assert!(prom.contains("node_cpu_seconds_total"));
        assert!(prom.contains("node_memory_MemFree_bytes"));

        let collectd = suite.exporter.export_collectd_metrics(&suite.top_monitor, &suite.vmstat_collector);
        assert!(collectd.contains("collectd"));

        let telemetry = suite.export_all_telemetry();
        assert!(telemetry.contains("PROMETHEUS METRICS"));
        assert!(telemetry.contains("COLLECTD METRICS"));
    }
}
