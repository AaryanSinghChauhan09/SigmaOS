//! Sovereign Linux & BSD Logging, Monitoring, and Diagnostics Engine for SigmaOS
//!
//! Synthesizes capabilities inspired by Linux & BSD system administration, observability,
//! and telemetry tooling:
//! - Journald, Rsyslog, Syslog-ng, Syslogd & Logrotate: Multi-facility structured logging,
<<<<<<< HEAD
//!   RFC 5424 / RFC 3164 syslog framing, rate limiting, TCP/UDP/TLS network forwarding,
//!   JSON/CEF output, and automated log rotation policies.
//! - Dmesg: Kernel ring buffer logger with microsecond boot timestamps, severity levels,
//!   kmsg formatting, and facility tag filtering.
//! - Top, Htop, Btop, Atop & Iotop, Nethogs: Process activity telemetry, thread counts,
//!   CPU/RAM utilization, per-process disk IOPS/bandwidth, per-process network sockets,
//!   and process tree hierarchy.
//! - Vmstat, Iostat, Sar, Sysstat, Mpstat & Pidstat: System memory, swap, context switches,
//!   disk IOPS, multi-core CPU utilization breakdown, and time-series performance metrics.
//! - Lsof, Fuser & Lslocks: Open file descriptor inspection across processes, paths, sockets,
//!   POSIX/FLOCK file locks, and active mount point process identification.
//! - Lspci, Lsusb, Dmidecode, Lscpu, Lsblk, Smartctl & Sensors: Comprehensive hardware
//!   discovery for PCI, USB, SMBIOS/DMI tables, CPU ISA microarchitecture topology,
//!   NVMe/SATA SMART health, and thermal/fan sensors.
//! - Sosreport, Prometheus node_exporter, OpenTelemetry & Loki: Diagnostic archive bundle
//!   generation, Prometheus metrics exposition, OpenTelemetry spans, and Loki log streams.
=======
//!   RFC 5424 syslog framing, rate limiting, Syslog-ng log pipelines, journalctl cursor
//!   pagination & binary sealing verification, and logrotate policy parsing/execution.
//! - Dmesg: Kernel ring buffer logger with microsecond boot timestamps, severity levels,
//!   facility tag filtering, ANSI coloring, and `klogctl` / `kernel.printk` console mask control.
//! - Top & Htop: Process activity telemetry, thread counts, CPU/RAM utilization,
//!   CachyOS BORE / EEVDF interactivity scoring, signal handling, renicing, and process tree hierarchy.
//! - Vmstat, Iostat, Sar & Sysstat: System memory, swap, context switches, disk queue lengths,
//!   await times (`r_await`/`w_await`), utilization (%util), and historical sar binary data serialization.
//! - Lsof: Open file descriptor inspection across processes, paths, mount points,
//!   TCP/UDP socket states, and IPC handles.
//! - Lspci, Lsusb & Dmidecode: Comprehensive hardware discovery for PCI Express link speeds/widths,
//!   USB speeds (480Mbps to 40Gbps USB4), SMBIOS table structure types, and driver binding queries.
//! - Sosreport, Prometheus node_exporter & Collectd: Diagnostic archive bundle generation,
//!   system config capturing (`/etc/fstab`, `/etc/sysctl.conf`), process & netdev stats,
//!   Prometheus text exposition, and Collectd RRD metric payloads.
//! - Diagnostics Pull Request Gateway: Sovereign PR gateway supporting diagnostic plugins,
//!   metric exporters, log parsers, and hardware probes with automated CI gating, PQC signatures,
//!   and unified diff generation.
>>>>>>> origin/main

use std::collections::{BTreeMap, HashMap};
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. JOURNALD / RSYSLOG / SYSLOG-NG / LOGROTATE LOG ENGINE
// ============================================================================

/// Syslog Facility inspired by Linux syslog.h & BSD RFC 5424
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
}

/// Log Severity inspired by Syslog RFC 5424 / Journald
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn ansi_color_code(&self) -> &'static str {
        match self {
            LogSeverity::Emergency | LogSeverity::Alert | LogSeverity::Critical => "\x1b[1;31m", // Bright Red
            LogSeverity::Error => "\x1b[31m",                                                      // Red
            LogSeverity::Warning => "\x1b[33m",                                                    // Yellow
            LogSeverity::Notice => "\x1b[32m",                                                     // Green
            LogSeverity::Info => "\x1b[36m",                                                       // Cyan
            LogSeverity::Debug => "\x1b[35m",                                                      // Magenta
        }
    }
}

/// Structured Journald Log Entry
#[derive(Debug, Clone)]
pub struct JournalLogEntry {
    pub id: u64,
    pub timestamp_us: u64,
    pub boot_id: String,
    pub facility: SyslogFacility,
    pub severity: LogSeverity,
    pub pid: u32,
    pub app_name: String,
    pub systemd_unit: Option<String>,
    pub message: String,
    pub metadata: BTreeMap<String, String>,
    pub entry_hash: String,
}

impl JournalLogEntry {
    pub fn syslog_pri(&self) -> u8 {
        (self.facility as u8) * 8 + (self.severity as u8)
    }

    /// Format entry as RFC 5424 Syslog message
    pub fn to_rfc5424(&self, hostname: &str) -> String {
        let pri = self.syslog_pri();
        let mut sd = String::from("[sigmaos@53828");
        for (k, v) in &self.metadata {
            sd.push_str(&format!(" {}=\"{}\"", k, v));
        }
        if let Some(unit) = &self.systemd_unit {
            sd.push_str(&format!(" systemd_unit=\"{}\"", unit));
        }
        sd.push(']');

        format!(
            "<{}>1 {} {} {} {} - {} {}",
            pri, self.timestamp_us, hostname, self.app_name, self.pid, sd, self.message
        )
    }

    /// Calculate FSP (Forward Secure Sealing) entry verification checksum
    pub fn compute_seal_checksum(&self) -> String {
        let raw = format!(
            "{}:{}:{}:{}:{}:{}",
            self.id, self.timestamp_us, self.boot_id, self.app_name, self.pid, self.message
        );
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in raw.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("{:016x}", hash)
    }
}

/// Syslog-ng Log Pipeline Destination Rule
#[derive(Debug, Clone)]
pub struct SyslogNgDestinationRule {
    pub name: String,
    pub filter_facility: Option<SyslogFacility>,
    pub min_severity: Option<LogSeverity>,
    pub app_filter: Option<String>,
    pub target_file: String,
}

/// Log Rotation Frequency (`logrotate`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogRotateFrequency {
    Daily,
    Weekly,
    Monthly,
}

/// Log Rotation Policy (`logrotate`)
#[derive(Debug, Clone)]
pub struct LogRotateConfig {
    pub max_entries_per_file: usize,
    pub size_threshold_bytes: u64,
    pub max_rotated_generations: usize,
    pub compress_old_logs: bool,
    pub frequency: LogRotateFrequency,
    pub create_mode: String,
}

/// Journald, Rsyslog, & Syslog-ng Unified Logging Engine
#[derive(Debug, Clone)]
pub struct JournaldRsyslogLogEngine {
    pub hostname: String,
    pub current_boot_id: String,
    pub entries: Vec<JournalLogEntry>,
    pub rotated_archives: Vec<Vec<JournalLogEntry>>,
    pub config: LogRotateConfig,
    pub syslog_ng_rules: Vec<SyslogNgDestinationRule>,
    pub next_id: u64,
    pub rate_limit_burst: usize,
    pub current_burst_count: usize,
    pub is_binary_sealed: bool,
}

impl JournaldRsyslogLogEngine {
    pub fn new(hostname: &str) -> Self {
        let mut engine = Self {
            hostname: hostname.to_string(),
            current_boot_id: "a1b2c3d4e5f647898091a2b3c4d5e6f7".to_string(),
            entries: Vec::new(),
            rotated_archives: Vec::new(),
            config: LogRotateConfig {
                max_entries_per_file: 500,
                size_threshold_bytes: 10 * 1024 * 1024,
                max_rotated_generations: 4,
                compress_old_logs: true,
                frequency: LogRotateFrequency::Daily,
                create_mode: "0640 root adm".to_string(),
            },
            syslog_ng_rules: Vec::new(),
            next_id: 1,
            rate_limit_burst: 100,
            current_burst_count: 0,
            is_binary_sealed: true,
        };

        engine.syslog_ng_rules.push(SyslogNgDestinationRule {
            name: "d_auth".to_string(),
            filter_facility: Some(SyslogFacility::Auth),
            min_severity: None,
            app_filter: None,
            target_file: "/var/log/auth.log".to_string(),
        });

        engine.syslog_ng_rules.push(SyslogNgDestinationRule {
            name: "d_syslog".to_string(),
            filter_facility: None,
            min_severity: Some(LogSeverity::Info),
            app_filter: None,
            target_file: "/var/log/syslog".to_string(),
        });

        engine
    }

    pub fn log(
        &mut self,
        facility: SyslogFacility,
        severity: LogSeverity,
        pid: u32,
        app_name: &str,
        systemd_unit: Option<&str>,
        message: &str,
        metadata: BTreeMap<String, String>,
    ) -> u64 {
        if self.current_burst_count >= self.rate_limit_burst {
            return 0; // Throttled by rate limiting
        }
        self.current_burst_count += 1;

        let entry_id = self.next_id;
        self.next_id += 1;

        let mut entry = JournalLogEntry {
            id: entry_id,
            timestamp_us: entry_id * 10_000,
            boot_id: self.current_boot_id.clone(),
            facility,
            severity,
            pid,
            app_name: app_name.to_string(),
            systemd_unit: systemd_unit.map(|u| u.to_string()),
            message: message.to_string(),
            metadata,
            entry_hash: String::new(),
        };

        entry.entry_hash = entry.compute_seal_checksum();
        self.entries.push(entry);

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

    pub fn query_journalctl(
        &self,
        unit: Option<&str>,
        app_name: Option<&str>,
        min_severity: Option<LogSeverity>,
        boot_id: Option<&str>,
        limit: usize,
    ) -> Vec<&JournalLogEntry> {
        self.entries
            .iter()
            .filter(|e| {
                if let Some(u) = unit {
                    if e.systemd_unit.as_deref() != Some(u) {
                        return false;
                    }
                }
                if let Some(app) = app_name {
                    if e.app_name != app {
                        return false;
                    }
                }
                if let Some(sev) = min_severity {
                    if e.severity > sev {
                        return false;
                    }
                }
                if let Some(b) = boot_id {
                    if e.boot_id != b {
                        return false;
                    }
                }
                true
            })
            .take(limit)
            .collect()
    }

    pub fn verify_journal_sealing_integrity(&self) -> bool {
        for entry in &self.entries {
            if entry.entry_hash != entry.compute_seal_checksum() {
                return false;
            }
        }
        true
    }
}

// ============================================================================
// 1B. SYSLOG-NG / RSYSLOG NETWORK ROUTING & FORMATTING ENGINE
// ============================================================================

/// Syslog-ng Network Transport Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyslogNgTransportMode {
    Udp,
    Tcp,
    Tls,
    Relp,
    LocalPipe,
    File,
}

/// Output Formatting Specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormatSpec {
    Rfc5424,
    Rfc3164,
    Json,
    Cef,
}

/// Destination Rule for Network Forwarding
#[derive(Debug, Clone)]
pub struct LogDestinationRule {
    pub destination_id: String,
    pub transport: SyslogNgTransportMode,
    pub target_endpoint: String,
    pub format: LogFormatSpec,
    pub filter_facility: Option<SyslogFacility>,
    pub min_severity: Option<LogSeverity>,
}

/// Syslog-ng & Rsyslog Advanced Routing Engine
#[derive(Debug, Clone)]
pub struct SyslogNgRsyslogRoutingEngine {
    pub rules: Vec<LogDestinationRule>,
}

impl SyslogNgRsyslogRoutingEngine {
    pub fn new() -> Self {
        Self {
            rules: vec![
                LogDestinationRule {
                    destination_id: "sec_siem_cef".to_string(),
                    transport: SyslogNgTransportMode::Tls,
                    target_endpoint: "10.0.0.50:6514".to_string(),
                    format: LogFormatSpec::Cef,
                    filter_facility: Some(SyslogFacility::AuthPriv),
                    min_severity: Some(LogSeverity::Warning),
                },
                LogDestinationRule {
                    destination_id: "central_json_log".to_string(),
                    transport: SyslogNgTransportMode::Tcp,
                    target_endpoint: "10.0.0.100:5140".to_string(),
                    format: LogFormatSpec::Json,
                    filter_facility: None,
                    min_severity: Some(LogSeverity::Info),
                },
            ],
        }
    }

    pub fn add_rule(&mut self, rule: LogDestinationRule) {
        self.rules.push(rule);
    }

    pub fn format_json(&self, entry: &JournalLogEntry, hostname: &str) -> String {
        let mut json = String::new();
        json.push_str("{\"id\":");
        json.push_str(&entry.id.to_string());
        json.push_str(",\"timestamp_us\":");
        json.push_str(&entry.timestamp_us.to_string());
        json.push_str(",\"hostname\":\"");
        json.push_str(hostname);
        json.push_str("\",\"facility\":\"");
        json.push_str(entry.facility.as_str());
        json.push_str("\",\"severity\":\"");
        json.push_str(entry.severity.as_str());
        json.push_str("\",\"pid\":");
        json.push_str(&entry.pid.to_string());
        json.push_str(",\"app\":\"");
        json.push_str(&entry.app_name);
        json.push_str("\",\"message\":\"");
        json.push_str(&entry.message.replace('"', "\\\""));
        json.push_str("\"}");
        json
    }

    pub fn format_cef(&self, entry: &JournalLogEntry, hostname: &str) -> String {
        format!(
            "CEF:0|SigmaOS|SovereignKernel|1.0|{:?}|{}|{}|src={} app={} pid={}",
            entry.facility,
            entry.severity.as_str(),
            entry.message,
            hostname,
            entry.app_name,
            entry.pid
        )
    }

    pub fn route_entry(&self, entry: &JournalLogEntry, hostname: &str) -> Vec<(String, String)> {
        let mut dispatched = Vec::new();
        for rule in &self.rules {
            if let Some(fac) = rule.filter_facility {
                if entry.facility != fac {
                    continue;
                }
            }
            if let Some(sev) = rule.min_severity {
                if entry.severity > sev {
                    continue;
                }
            }

            let formatted = match rule.format {
                LogFormatSpec::Rfc5424 => entry.to_rfc5424(hostname),
                LogFormatSpec::Rfc3164 => format!(
                    "<{}> {} {}[{}]: {}",
                    entry.syslog_pri(),
                    hostname,
                    entry.app_name,
                    entry.pid,
                    entry.message
                ),
                LogFormatSpec::Json => self.format_json(entry, hostname),
                LogFormatSpec::Cef => self.format_cef(entry, hostname),
            };

            dispatched.push((rule.destination_id.clone(), formatted));
        }
        dispatched
    }
}

impl Default for SyslogNgRsyslogRoutingEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. DMESG KERNEL RING BUFFER ENGINE
// ============================================================================

/// Single Kernel Ring Buffer Record
#[derive(Debug, Clone)]
pub struct DmesgRecord {
    pub sequence: u64,
    pub timestamp_us: u64,
    pub facility: SyslogFacility,
    pub severity: LogSeverity,
    pub subsystem: String,
    pub message: String,
}

impl DmesgRecord {
    pub fn format_ansi(&self) -> String {
        format!(
            "{}[{:12.6}] {}<{}>: {}\x1b[0m",
            self.severity.ansi_color_code(),
            self.timestamp_us as f64 / 1_000_000.0,
            self.subsystem,
            self.severity.as_str(),
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
    pub console_loglevel_mask: LogSeverity,
}

impl DmesgKernelRingBufferEngine {
    pub fn new(capacity: usize) -> Self {
        let mut engine = Self {
            buffer: Vec::new(),
            max_capacity: capacity,
            sequence_counter: 0,
            console_loglevel_mask: LogSeverity::Debug,
        };
        engine.populate_initial_kernel_boot_sequence();
        engine
    }

    fn populate_initial_kernel_boot_sequence(&mut self) {
        let boot_msgs: &[(&str, SyslogFacility, LogSeverity, &str, u64)] = &[
            (
                "Linux/BSD Sovereign Microkernel initializing...",
                SyslogFacility::Kernel,
                LogSeverity::Info,
                "kernel",
                0,
            ),
            (
                "Command line: root=/dev/sda1 ro quiet splash",
                SyslogFacility::Kernel,
                LogSeverity::Info,
                "kernel",
                120,
            ),
            (
                "BIOS-provided physical RAM map: 64 GB detected",
                SyslogFacility::Kernel,
                LogSeverity::Info,
                "memory",
                1200,
            ),
            (
                "ACPI: RSDP at 0x000F0240, parsing FADT and MADT tables...",
                SyslogFacility::Kernel,
                LogSeverity::Info,
                "acpi",
                4500,
            ),
            (
                "SMP: 64 logical CPU cores online",
                SyslogFacility::Kernel,
                LogSeverity::Info,
                "smp",
                12400,
            ),
            (
                "PCI: Host bridge initialized at 0000:00",
                SyslogFacility::Kernel,
                LogSeverity::Info,
                "pci",
                25000,
            ),
            (
                "usbcore: registered new interface driver usbfs",
                SyslogFacility::Kernel,
                LogSeverity::Info,
                "usb",
                88000,
            ),
            (
                "SovereignFS: Mounting root filesystem...",
                SyslogFacility::Kernel,
                LogSeverity::Info,
                "fs",
                150000,
            ),
            (
                "systemd[1]: Reached target Graphical Interface",
                SyslogFacility::Daemon,
                LogSeverity::Notice,
                "init",
                250000,
            ),
        ];

        for &(msg, fac, sev, subsys, ts) in boot_msgs {
            self.sequence_counter += 1;
            self.buffer.push(DmesgRecord {
                sequence: self.sequence_counter,
                timestamp_us: ts,
                facility: fac,
                severity: sev,
                subsystem: subsys.to_string(),
                message: msg.to_string(),
            });
        }
    }

    pub fn print_klog(
        &mut self,
        facility: SyslogFacility,
        severity: LogSeverity,
        subsystem: &str,
        message: &str,
    ) {
        self.sequence_counter += 1;
        let ts = self.sequence_counter * 1_000;

        if self.buffer.len() >= self.max_capacity {
            self.buffer.remove(0);
        }

        self.buffer.push(DmesgRecord {
            sequence: self.sequence_counter,
            timestamp_us: ts,
            facility,
            severity,
            subsystem: subsystem.to_string(),
            message: message.to_string(),
        });
    }

    pub fn filter(
        &self,
        min_severity: Option<LogSeverity>,
        facility: Option<SyslogFacility>,
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
                if let Some(fac) = facility {
                    if rec.facility != fac {
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

    pub fn set_console_loglevel(&mut self, level: LogSeverity) {
        self.console_loglevel_mask = level;
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

// ============================================================================
// 3. TOP & HTOP PROCESS MONITOR ENGINE
// ============================================================================

/// Process Execution State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    DiskSleep,
    Stopped,
    Zombie,
}

impl ProcessState {
    pub fn as_char(&self) -> char {
        match self {
            ProcessState::Running => 'R',
            ProcessState::Sleeping => 'S',
            ProcessState::DiskSleep => 'D',
            ProcessState::Stopped => 'T',
            ProcessState::Zombie => 'Z',
        }
    }
}

/// Process Activity Telemetry Record (`top`/`htop`)
#[derive(Debug, Clone)]
pub struct ProcessTelemetry {
    pub pid: u32,
    pub ppid: u32,
    pub user: String,
    pub name: String,
    pub state: ProcessState,
    pub nice_priority: i32,
    pub cpu_usage_pct: f32,
    pub mem_usage_pct: f32,
    pub virt_mem_kb: u64,
    pub res_mem_kb: u64,
    pub num_threads: u32,
    pub read_bytes_sec: u64,
    pub write_bytes_sec: u64,
    pub bore_interactivity_score: u32, // CachyOS BORE scheduler interactivity score
}

/// Top & Htop Process Telemetry Monitor
#[derive(Debug, Clone)]
pub struct TopHtopProcessMonitorEngine {
    pub processes: BTreeMap<u32, ProcessTelemetry>,
    pub total_cpu_user_pct: f32,
    pub total_cpu_sys_pct: f32,
    pub total_cpu_idle_pct: f32,
}

impl TopHtopProcessMonitorEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            processes: BTreeMap::new(),
            total_cpu_user_pct: 12.5,
            total_cpu_sys_pct: 3.2,
            total_cpu_idle_pct: 84.3,
        };
        engine.populate_sample_processes();
        engine
    }

    fn populate_sample_processes(&mut self) {
        let samples = vec![
            ProcessTelemetry {
                pid: 1,
                ppid: 0,
                user: "root".to_string(),
                name: "systemd".to_string(),
                state: ProcessState::Sleeping,
                nice_priority: 0,
                cpu_usage_pct: 0.1,
                mem_usage_pct: 0.2,
                virt_mem_kb: 168000,
                res_mem_kb: 12400,
                num_threads: 1,
                read_bytes_sec: 1024,
                write_bytes_sec: 512,
                bore_interactivity_score: 95,
            },
            ProcessTelemetry {
                pid: 102,
                ppid: 1,
                user: "root".to_string(),
                name: "journald".to_string(),
                state: ProcessState::Sleeping,
                nice_priority: -5,
                cpu_usage_pct: 0.5,
                mem_usage_pct: 0.4,
                virt_mem_kb: 84000,
                res_mem_kb: 8100,
                num_threads: 2,
                read_bytes_sec: 4096,
                write_bytes_sec: 16384,
                bore_interactivity_score: 88,
            },
            ProcessTelemetry {
                pid: 540,
                ppid: 1,
                user: "sigma".to_string(),
                name: "zenith-desktop".to_string(),
                state: ProcessState::Running,
                nice_priority: 0,
                cpu_usage_pct: 4.8,
                mem_usage_pct: 2.5,
                virt_mem_kb: 512000,
                res_mem_kb: 164000,
                num_threads: 8,
                read_bytes_sec: 32768,
                write_bytes_sec: 8192,
                bore_interactivity_score: 99,
            },
        ];

        for proc in samples {
            self.processes.insert(proc.pid, proc);
        }
    }

    pub fn update_process(&mut self, proc: ProcessTelemetry) {
        self.processes.insert(proc.pid, proc);
    }

    pub fn renice_process(&mut self, pid: u32, new_nice: i32) -> bool {
        if let Some(proc) = self.processes.get_mut(&pid) {
            proc.nice_priority = new_nice.clamp(-20, 19);
            true
        } else {
            false
        }
    }

    pub fn send_signal(&mut self, pid: u32, signal: &str) -> bool {
        if let Some(proc) = self.processes.get_mut(&pid) {
            match signal {
                "SIGTERM" | "SIGKILL" => {
                    self.processes.remove(&pid);
                }
                "SIGSTOP" => {
                    proc.state = ProcessState::Stopped;
                }
                "SIGCONT" => {
                    proc.state = ProcessState::Running;
                }
                _ => {}
            }
            true
        } else {
            false
        }
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

    pub fn sort_by_interactivity(&self) -> Vec<&ProcessTelemetry> {
        let mut list: Vec<&ProcessTelemetry> = self.processes.values().collect();
        list.sort_by(|a, b| b.bore_interactivity_score.cmp(&a.bore_interactivity_score));
        list
    }

    pub fn build_process_tree(&self) -> HashMap<u32, Vec<u32>> {
        let mut tree: HashMap<u32, Vec<u32>> = HashMap::new();
        for proc in self.processes.values() {
            tree.entry(proc.ppid).or_default().push(proc.pid);
        }
        tree
    }
}

impl Default for TopHtopProcessMonitorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3B. IOTOP & NETHOGS PROCESS I/O AND NETWORK MONITOR
// ============================================================================

/// Process Disk I/O Rate (`iotop`)
#[derive(Debug, Clone)]
pub struct ProcessIoTelemetry {
    pub pid: u32,
    pub command: String,
    pub read_bytes_sec: u64,
    pub write_bytes_sec: u64,
    pub swapin_delay_pct: f32,
    pub io_priority: u8, // 0..7 (be/realtime/idle)
}

/// Process Network Socket Throughput (`nethogs` / `iftop`)
#[derive(Debug, Clone)]
pub struct ProcessNetTelemetry {
    pub pid: u32,
    pub command: String,
    pub rx_bytes_sec: u64,
    pub tx_bytes_sec: u64,
    pub active_sockets_count: u32,
}

/// Iotop & Nethogs Monitor
#[derive(Debug, Clone)]
pub struct IotopNethogsProcessIoNetMonitor {
    pub io_stats: BTreeMap<u32, ProcessIoTelemetry>,
    pub net_stats: BTreeMap<u32, ProcessNetTelemetry>,
}

impl IotopNethogsProcessIoNetMonitor {
    pub fn new() -> Self {
        let mut monitor = Self {
            io_stats: BTreeMap::new(),
            net_stats: BTreeMap::new(),
        };
        monitor.populate_samples();
        monitor
    }

    fn populate_samples(&mut self) {
        self.io_stats.insert(
            540,
            ProcessIoTelemetry {
                pid: 540,
                command: "zenith-desktop".to_string(),
                read_bytes_sec: 1_048_576,
                write_bytes_sec: 524_288,
                swapin_delay_pct: 0.1,
                io_priority: 4,
            },
        );

        self.net_stats.insert(
            540,
            ProcessNetTelemetry {
                pid: 540,
                command: "zenith-desktop".to_string(),
                rx_bytes_sec: 262_144,
                tx_bytes_sec: 65_536,
                active_sockets_count: 5,
            },
        );
    }

    pub fn update_io(&mut self, io: ProcessIoTelemetry) {
        self.io_stats.insert(io.pid, io);
    }

    pub fn update_net(&mut self, net: ProcessNetTelemetry) {
        self.net_stats.insert(net.pid, net);
    }

    pub fn top_io_consumers(&self) -> Vec<&ProcessIoTelemetry> {
        let mut list: Vec<&ProcessIoTelemetry> = self.io_stats.values().collect();
        list.sort_by(|a, b| {
            (b.read_bytes_sec + b.write_bytes_sec)
                .cmp(&(a.read_bytes_sec + a.write_bytes_sec))
        });
        list
    }

    pub fn top_net_consumers(&self) -> Vec<&ProcessNetTelemetry> {
        let mut list: Vec<&ProcessNetTelemetry> = self.net_stats.values().collect();
        list.sort_by(|a, b| {
            (b.rx_bytes_sec + b.tx_bytes_sec).cmp(&(a.rx_bytes_sec + a.tx_bytes_sec))
        });
        list
    }
}

impl Default for IotopNethogsProcessIoNetMonitor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. VMSTAT / IOSTAT / SAR / MPSTAT / PIDSTAT METRICS COLLECTOR ENGINE
// ============================================================================

/// Vmstat System Memory & CPU Activity Snapshot
#[derive(Debug, Clone, Copy)]
pub struct VmstatSnapshot {
    pub timestamp_sec: u64,
    pub procs_running: u32,
    pub procs_blocked: u32,
    pub swpd_kb: u64,
    pub free_kb: u64,
    pub buff_kb: u64,
    pub cache_kb: u64,
    pub swap_in_kb_s: u32,
    pub swap_out_kb_s: u32,
    pub blocks_in_s: u32,
    pub blocks_out_s: u32,
    pub interrupts_s: u32,
    pub context_switches_s: u32,
    pub page_faults_minor_s: u32,
    pub page_faults_major_s: u32,
    pub cpu_user_pct: u8,
    pub cpu_sys_pct: u8,
    pub cpu_idle_pct: u8,
    pub cpu_wait_pct: u8,
}

/// Iostat Block Device I/O Statistics
#[derive(Debug, Clone)]
pub struct IostatDeviceSnapshot {
    pub device_name: String,
    pub tps: f32,
    pub read_kb_sec: f32,
    pub write_kb_sec: f32,
    pub avg_queue_len: f32, // avgqu-sz
    pub read_await_ms: f32, // r_await
    pub write_await_ms: f32, // w_await
    pub avg_await_ms: f32,
    pub util_pct: f32,
}

/// Per-CPU Core Activity Snapshot (`mpstat`)
#[derive(Debug, Clone)]
pub struct CpuCoreSnapshot {
    pub core_id: u32,
    pub user_pct: f32,
    pub sys_pct: f32,
    pub iowait_pct: f32,
    pub irq_pct: f32,
    pub softirq_pct: f32,
    pub idle_pct: f32,
}

/// Per-Process Detailed Resource Snapshot (`pidstat`)
#[derive(Debug, Clone)]
pub struct PidstatSnapshot {
    pub pid: u32,
    pub command: String,
    pub cpu_usr_pct: f32,
    pub cpu_sys_pct: f32,
    pub minflt_s: f32,
    pub majflt_s: f32,
    pub kb_rd_s: f32,
    pub kb_wr_s: f32,
    pub voluntary_ctxt_s: f32,
    pub nonvoluntary_ctxt_s: f32,
}

/// Sar Time-Series Historical Performance Buffer (`sysstat`)
#[derive(Debug, Clone)]
pub struct VmstatIostatSarMetricsCollectorEngine {
    pub vmstat_history: Vec<VmstatSnapshot>,
    pub iostat_history: Vec<IostatDeviceSnapshot>,
    pub core_snapshots: Vec<CpuCoreSnapshot>,
    pub pidstat_snapshots: Vec<PidstatSnapshot>,
    pub max_history_capacity: usize,
}

impl VmstatIostatSarMetricsCollectorEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            vmstat_history: Vec::new(),
            iostat_history: Vec::new(),
            core_snapshots: Vec::new(),
            pidstat_snapshots: Vec::new(),
            max_history_capacity: 120,
        };
        engine.record_sample_snapshot();
        engine
    }

    fn record_sample_snapshot(&mut self) {
        self.vmstat_history.push(VmstatSnapshot {
            timestamp_sec: 100,
            procs_running: 2,
            procs_blocked: 0,
            swpd_kb: 0,
            free_kb: 48_000_000,
            buff_kb: 1_200_000,
            cache_kb: 12_400_000,
            swap_in_kb_s: 0,
            swap_out_kb_s: 0,
            blocks_in_s: 128,
            blocks_out_s: 64,
            interrupts_s: 1450,
            context_switches_s: 3200,
            page_faults_minor_s: 850,
            page_faults_major_s: 2,
            cpu_user_pct: 10,
            cpu_sys_pct: 3,
            cpu_idle_pct: 86,
            cpu_wait_pct: 1,
        });

        self.iostat_history.push(IostatDeviceSnapshot {
            device_name: "sda".to_string(),
            tps: 42.5,
            read_kb_sec: 1024.0,
            write_kb_sec: 512.0,
            avg_queue_len: 0.12,
            read_await_ms: 0.72,
            write_await_ms: 0.98,
            avg_await_ms: 0.85,
            util_pct: 3.2,
        });

        self.core_snapshots.push(CpuCoreSnapshot {
            core_id: 0,
            user_pct: 12.0,
            sys_pct: 4.0,
            iowait_pct: 0.5,
            irq_pct: 0.1,
            softirq_pct: 0.2,
            idle_pct: 83.2,
        });

        self.pidstat_snapshots.push(PidstatSnapshot {
            pid: 540,
            command: "zenith-desktop".to_string(),
            cpu_usr_pct: 4.2,
            cpu_sys_pct: 0.6,
            minflt_s: 120.0,
            majflt_s: 0.0,
            kb_rd_s: 1024.0,
            kb_wr_s: 512.0,
            voluntary_ctxt_s: 450.0,
            nonvoluntary_ctxt_s: 12.0,
        });
    }

    pub fn push_vmstat(&mut self, snapshot: VmstatSnapshot) {
        if self.vmstat_history.len() >= self.max_history_capacity {
            self.vmstat_history.remove(0);
        }
        self.vmstat_history.push(snapshot);
    }

    pub fn push_iostat(&mut self, device_stat: IostatDeviceSnapshot) {
        self.iostat_history.push(device_stat);
    }

    pub fn calculate_sar_average_cpu(&self) -> (f32, f32, f32) {
        if self.vmstat_history.is_empty() {
            return (0.0, 0.0, 0.0);
        }
        let total = self.vmstat_history.len() as f32;
        let user: f32 = self
            .vmstat_history
            .iter()
            .map(|v| v.cpu_user_pct as f32)
            .sum();
        let sys: f32 = self
            .vmstat_history
            .iter()
            .map(|v| v.cpu_sys_pct as f32)
            .sum();
        let idle: f32 = self
            .vmstat_history
            .iter()
            .map(|v| v.cpu_idle_pct as f32)
            .sum();

        (user / total, sys / total, idle / total)
    }

    pub fn export_sar_binary_payload(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"SAR_BIN_V1\n");
        buf.extend_from_slice(format!("RECORDS:{}\n", self.vmstat_history.len()).as_bytes());
        for rec in &self.vmstat_history {
            buf.extend_from_slice(
                format!(
                    "{},{},{},{},{},{}\n",
                    rec.timestamp_sec,
                    rec.cpu_user_pct,
                    rec.cpu_sys_pct,
                    rec.cpu_idle_pct,
                    rec.interrupts_s,
                    rec.context_switches_s
                )
                .as_bytes(),
            );
        }
        buf
    }
}

impl Default for VmstatIostatSarMetricsCollectorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. LSOF, FUSER & LSLOCKS INSPECTOR ENGINE
// ============================================================================

/// Open File Descriptor Type (`lsof`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenFileType {
    Regular,
    Directory,
    Character,
    Block,
    Fifo,
    UnixSocket,
    Ipv4Socket,
    Ipv6Socket,
}

/// Socket State (`lsof` / `ss`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Established,
    Listen,
    TimeWait,
    CloseWait,
    SynSent,
    None,
}

/// Open File Descriptor Record (`lsof`)
#[derive(Debug, Clone)]
pub struct OpenFileRecord {
    pub pid: u32,
    pub command: String,
    pub user: String,
    pub fd_name: String, // cwd, rtd, txt, 0u, 1w, 2w, etc.
    pub file_type: OpenFileType,
    pub socket_state: SocketState,
    pub device: String,
    pub node_inode: u64,
    pub path_name: String,
}

/// Lsof Open Files Inspector Engine
#[derive(Debug, Clone)]
pub struct LsofOpenFilesInspectorEngine {
    pub open_files: Vec<OpenFileRecord>,
}

impl LsofOpenFilesInspectorEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            open_files: Vec::new(),
        };
        engine.populate_sample_open_files();
        engine
    }

    fn populate_sample_open_files(&mut self) {
        self.open_files.extend(vec![
            OpenFileRecord {
                pid: 1,
                command: "systemd".to_string(),
                user: "root".to_string(),
                fd_name: "cwd".to_string(),
                file_type: OpenFileType::Directory,
                socket_state: SocketState::None,
                device: "8,1".to_string(),
                node_inode: 2,
                path_name: "/".to_string(),
            },
            OpenFileRecord {
                pid: 540,
                command: "zenith-desktop".to_string(),
                user: "sigma".to_string(),
                fd_name: "3u".to_string(),
                file_type: OpenFileType::UnixSocket,
                socket_state: SocketState::Established,
                device: "0,12".to_string(),
                node_inode: 15420,
                path_name: "@wayland-0".to_string(),
            },
            OpenFileRecord {
                pid: 540,
                command: "zenith-desktop".to_string(),
                user: "sigma".to_string(),
                fd_name: "4u".to_string(),
                file_type: OpenFileType::Ipv4Socket,
                socket_state: SocketState::Listen,
                device: "0,14".to_string(),
                node_inode: 18910,
                path_name: "127.0.0.1:8080".to_string(),
            },
        ]);
    }

    pub fn register_open_file(&mut self, record: OpenFileRecord) {
        self.open_files.push(record);
    }

    pub fn find_by_pid(&self, pid: u32) -> Vec<&OpenFileRecord> {
        self.open_files.iter().filter(|f| f.pid == pid).collect()
    }

    pub fn find_by_path(&self, path_substring: &str) -> Vec<&OpenFileRecord> {
        self.open_files
            .iter()
            .filter(|f| f.path_name.contains(path_substring))
            .collect()
    }

    pub fn find_sockets_by_state(&self, state: SocketState) -> Vec<&OpenFileRecord> {
        self.open_files
            .iter()
            .filter(|f| f.socket_state == state)
            .collect()
    }
}

impl Default for LsofOpenFilesInspectorEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// System-Wide File Lock Type (`lslocks`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileLockType {
    Posix,
    Flock,
    Ofd,
    Lease,
}

/// System-Wide File Lock Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileLockMode {
    Read,
    Write,
}

/// File Lock Record (`lslocks`)
#[derive(Debug, Clone)]
pub struct FileLockRecord {
    pub pid: u32,
    pub command: String,
    pub lock_type: FileLockType,
    pub lock_mode: FileLockMode,
    pub mandatory: bool,
    pub inode: u64,
    pub start_byte: u64,
    pub end_byte: u64,
    pub path_name: String,
}

/// Fuser & Lslocks Inspector
#[derive(Debug, Clone)]
pub struct FuserLslocksInspector {
    pub locks: Vec<FileLockRecord>,
}

impl FuserLslocksInspector {
    pub fn new() -> Self {
        Self {
            locks: vec![FileLockRecord {
                pid: 102,
                command: "journald".to_string(),
                lock_type: FileLockType::Posix,
                lock_mode: FileLockMode::Write,
                mandatory: false,
                inode: 40210,
                start_byte: 0,
                end_byte: 0, // 0 = EOF
                path_name: "/var/log/journal/system.journal".to_string(),
            }],
        }
    }

    pub fn add_lock(&mut self, lock: FileLockRecord) {
        self.locks.push(lock);
    }

    pub fn find_locks_for_path(&self, path: &str) -> Vec<&FileLockRecord> {
        self.locks.iter().filter(|l| l.path_name == path).collect()
    }

    pub fn find_processes_holding_mount(
        &self,
        lsof: &LsofOpenFilesInspectorEngine,
        mount_path: &str,
    ) -> Vec<u32> {
        lsof.open_files
            .iter()
            .filter(|f| f.path_name.starts_with(mount_path))
            .map(|f| f.pid)
            .collect()
    }
}

impl Default for FuserLslocksInspector {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. HARDWARE DISCOVERY ENGINE (LSPCI, LSUSB, DMIDECODE, LSCPU, LSBLK, SMARTCTL, SENSORS)
// ============================================================================

/// PCI Device Info (`lspci`)
#[derive(Debug, Clone)]
pub struct PciDeviceRecord {
    pub slot: String, // 00:1f.3
    pub class_name: String,
    pub vendor_name: String,
    pub device_name: String,
    pub revision: u8,
    pub pcie_gen: String,   // Gen1..Gen5
    pub link_width: String, // x1..x16
    pub driver_in_use: String,
    pub kernel_modules: Vec<String>,
}

/// USB Device Info (`lsusb`)
#[derive(Debug, Clone)]
pub struct UsbDeviceRecord {
    pub bus_num: u8,
    pub device_num: u8,
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: String,
    pub product: String,
    pub speed_descriptor: String, // e.g. "480 Mbps", "5 Gbps", "10 Gbps", "40 Gbps (USB4)"
    pub speed_mbps: u32,
}

/// SMBIOS Table Structure Type (`dmidecode`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmbiosStructureType {
    BiosInfo = 0,
    SystemInfo = 1,
    BaseboardInfo = 2,
    ChassisInfo = 3,
    ProcessorInfo = 4,
    MemoryDevice = 17,
}

/// DMI/SMBIOS System Info (`dmidecode`)
#[derive(Debug, Clone)]
pub struct DmiDecodeInfo {
    pub bios_vendor: String,
    pub bios_version: String,
    pub bios_release_date: String,
    pub sys_manufacturer: String,
    pub sys_product_name: String,
    pub sys_serial_number: String,
    pub chassis_type: String,
    pub processor_family: String,
    pub memory_slots_count: usize,
}

/// CPU Architecture Topology (`lscpu`)
#[derive(Debug, Clone)]
pub struct CpuTopologyInfo {
    pub architecture: String,
    pub online_cpus: u32,
    pub sockets: u32,
    pub cores_per_socket: u32,
    pub threads_per_core: u32,
    pub numa_nodes: u32,
    pub l1_cache_kb: u32,
    pub l2_cache_kb: u32,
    pub l3_cache_kb: u32,
    pub isa_extensions: Vec<String>,
}

/// Block Storage Device Info (`lsblk` / `smartctl`)
#[derive(Debug, Clone)]
pub struct BlockDeviceTopology {
    pub name: String,
    pub size_bytes: u64,
    pub rotational: bool,
    pub mountpoint: Option<String>,
    pub smart_passed: bool,
    pub temperature_celsius: u32,
    pub power_on_hours: u32,
    pub wear_level_pct: u8,
}

/// Thermal & Environmental Sensors (`sensors`)
#[derive(Debug, Clone)]
pub struct ThermalSensorRecord {
    pub sensor_name: String,
    pub temp_celsius: f32,
    pub crit_temp_celsius: f32,
    pub fan_rpm: Option<u32>,
}

/// Hardware Discovery Inspector Engine
#[derive(Debug, Clone)]
pub struct HardwareDiscoveryInspectorEngine {
    pub pci_devices: Vec<PciDeviceRecord>,
    pub usb_devices: Vec<UsbDeviceRecord>,
    pub dmi_info: DmiDecodeInfo,
    pub cpu_topology: CpuTopologyInfo,
    pub block_devices: Vec<BlockDeviceTopology>,
    pub thermal_sensors: Vec<ThermalSensorRecord>,
}

impl HardwareDiscoveryInspectorEngine {
    pub fn new() -> Self {
        Self {
            pci_devices: vec![
                PciDeviceRecord {
                    slot: "00:00.0".to_string(),
                    class_name: "Host bridge".to_string(),
                    vendor_name: "Intel Corporation".to_string(),
                    device_name: "13th Gen Core Host Bridge".to_string(),
                    revision: 0x02,
                    pcie_gen: "Gen4".to_string(),
                    link_width: "x16".to_string(),
                    driver_in_use: "pci-host".to_string(),
                    kernel_modules: vec!["pci-host".to_string()],
                },
                PciDeviceRecord {
                    slot: "00:02.0".to_string(),
                    class_name: "VGA compatible controller".to_string(),
                    vendor_name: "NVIDIA Corporation".to_string(),
                    device_name: "AD104 [GeForce RTX 4070]".to_string(),
                    revision: 0xa1,
                    pcie_gen: "Gen4".to_string(),
                    link_width: "x16".to_string(),
                    driver_in_use: "nvidia".to_string(),
                    kernel_modules: vec!["nvidia".to_string(), "nouveau".to_string()],
                },
            ],
            usb_devices: vec![UsbDeviceRecord {
                bus_num: 1,
                device_num: 2,
                vendor_id: 0x046d,
                product_id: 0xc52b,
                manufacturer: "Logitech".to_string(),
                product: "USB Receiver".to_string(),
                speed_descriptor: "12 Mbps (Full Speed)".to_string(),
                speed_mbps: 12,
            }],
            dmi_info: DmiDecodeInfo {
                bios_vendor: "American Megatrends Inc.".to_string(),
                bios_version: "2.40".to_string(),
                bios_release_date: "2024-06-15".to_string(),
                sys_manufacturer: "SigmaOS Sovereign Systems".to_string(),
                sys_product_name: "Sovereign Workstation V34".to_string(),
                sys_serial_number: "SIGMA-2025-X1".to_string(),
                chassis_type: "Desktop".to_string(),
                processor_family: "x86-64 v4".to_string(),
                memory_slots_count: 4,
            },
            cpu_topology: CpuTopologyInfo {
                architecture: "x86_64".to_string(),
                online_cpus: 64,
                sockets: 2,
                cores_per_socket: 16,
                threads_per_core: 2,
                numa_nodes: 2,
                l1_cache_kb: 32,
                l2_cache_kb: 1024,
                l3_cache_kb: 32768,
                isa_extensions: vec![
                    "avx2".to_string(),
                    "avx512f".to_string(),
                    "sha_ni".to_string(),
                    "aes".to_string(),
                ],
            },
            block_devices: vec![BlockDeviceTopology {
                name: "nvme0n1".to_string(),
                size_bytes: 2_000_398_934_016,
                rotational: false,
                mountpoint: Some("/".to_string()),
                smart_passed: true,
                temperature_celsius: 38,
                power_on_hours: 1250,
                wear_level_pct: 2,
            }],
            thermal_sensors: vec![ThermalSensorRecord {
                sensor_name: "cpu_package_thermal".to_string(),
                temp_celsius: 42.5,
                crit_temp_celsius: 100.0,
                fan_rpm: Some(1200),
            }],
        }
    }

    pub fn add_pci_device(&mut self, device: PciDeviceRecord) {
        self.pci_devices.push(device);
    }

    pub fn add_usb_device(&mut self, device: UsbDeviceRecord) {
        self.usb_devices.push(device);
    }

    pub fn query_pci_by_driver(&self, driver: &str) -> Vec<&PciDeviceRecord> {
        self.pci_devices
            .iter()
            .filter(|d| d.driver_in_use == driver)
            .collect()
    }
}

impl Default for HardwareDiscoveryInspectorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
<<<<<<< HEAD
// 7. SOSREPORT, PROMETHEUS, OPENTELEMETRY & LOKI EXPORTER ENGINE
=======
// 7. SOSREPORT, PROMETHEUS NODE_EXPORTER & COLLECTD EXPORTER ENGINE
>>>>>>> origin/main
// ============================================================================

/// Sosreport Diagnostic Archive Bundle
#[derive(Debug, Clone)]
pub struct SosreportArchiveBundle {
    pub report_id: String,
    pub timestamp_sec: u64,
    pub hostname: String,
    pub os_release: String,
    pub kernel_version: String,
    pub captured_files: BTreeMap<String, String>,
}

<<<<<<< HEAD
/// OpenTelemetry Span Record
#[derive(Debug, Clone)]
pub struct OtelSpanRecord {
    pub trace_id: String,
    pub span_id: String,
    pub name: String,
    pub start_time_us: u64,
    pub duration_us: u64,
    pub attributes: BTreeMap<String, String>,
}

/// Sosreport, Prometheus, OpenTelemetry & Loki Exporter
=======
/// Sosreport, Prometheus & Collectd Exporter Engine
>>>>>>> origin/main
#[derive(Debug, Clone)]
pub struct SosreportPrometheusExporterEngine {
    pub hostname: String,
}

impl SosreportPrometheusExporterEngine {
    pub fn new(hostname: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
        }
    }

    /// Generate an automated `sosreport` diagnostic archive bundle
    pub fn generate_sosreport(
        &self,
        journal: &JournaldRsyslogLogEngine,
        dmesg: &DmesgKernelRingBufferEngine,
        hw: &HardwareDiscoveryInspectorEngine,
    ) -> SosreportArchiveBundle {
        let mut files = BTreeMap::new();

        // Include dmesg log
        let mut dmesg_txt = String::new();
        for rec in &dmesg.buffer {
            dmesg_txt.push_str(&rec.format_ansi());
            dmesg_txt.push('\n');
        }
        files.insert("var/log/dmesg".to_string(), dmesg_txt);

        // Include syslog / journal log
        let mut syslog_txt = String::new();
        for entry in &journal.entries {
            syslog_txt.push_str(&entry.to_rfc5424(&self.hostname));
            syslog_txt.push('\n');
        }
        files.insert("var/log/syslog".to_string(), syslog_txt);

        // Include system configs
        files.insert(
            "etc/fstab".to_string(),
            "/dev/sda1 / ext4 defaults 0 1\n/dev/sda2 swap swap defaults 0 0\n".to_string(),
        );
        files.insert(
            "etc/sysctl.conf".to_string(),
            "vm.swappiness=10\nnet.ipv4.ip_forward=1\n".to_string(),
        );

        // Include DMI / hardware discovery
        let dmi_txt = format!(
            "BIOS Vendor: {}\nBIOS Version: {}\nSystem: {} {}\nSerial: {}\nCPU Arch: {}\n",
            hw.dmi_info.bios_vendor,
            hw.dmi_info.bios_version,
            hw.dmi_info.sys_manufacturer,
            hw.dmi_info.sys_product_name,
            hw.dmi_info.sys_serial_number,
            hw.cpu_topology.architecture
        );
        files.insert("sys/class/dmi/id/bios".to_string(), dmi_txt);

        SosreportArchiveBundle {
            report_id: format!("sosreport-{}-202502", self.hostname),
            timestamp_sec: 1700000000,
            hostname: self.hostname.clone(),
            os_release: "SigmaOS 1.0 (Sovereign)".to_string(),
            kernel_version: "6.12.0-sigmaos".to_string(),
            captured_files: files,
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
            "node_cpu_seconds_total{{mode=\"idle\"}} {}\n",
            top.total_cpu_idle_pct
        ));

        if let Some(vm) = vmstat.vmstat_history.last() {
            out.push_str(
                "# HELP node_memory_MemFree_bytes Memory information field MemFree_bytes.\n",
            );
            out.push_str("# TYPE node_memory_MemFree_bytes gauge\n");
            out.push_str(&format!(
                "node_memory_MemFree_bytes {}\n",
                vm.free_kb * 1024
            ));

            out.push_str(
                "# HELP node_memory_Cached_bytes Memory information field Cached_bytes.\n",
            );
            out.push_str("# TYPE node_memory_Cached_bytes gauge\n");
            out.push_str(&format!(
                "node_memory_Cached_bytes {}\n",
                vm.cache_kb * 1024
            ));
        }

        out
    }

<<<<<<< HEAD
    /// Export Loki Log Stream JSON payload
    pub fn export_loki_log_stream(&self, journal: &JournaldRsyslogLogEngine) -> String {
        let mut payload = String::from("{\"streams\":[{\"stream\":{\"host\":\"");
        payload.push_str(&self.hostname);
        payload.push_str("\"},\"values\":[");

        for (idx, entry) in journal.entries.iter().enumerate() {
            if idx > 0 {
                payload.push(',');
            }
            payload.push_str(&format!(
                "[\"{}\",\"[{}] [{}] {}\"]",
                entry.timestamp_us * 1000,
                entry.app_name,
                entry.severity.as_str(),
                entry.message.replace('"', "\\\"")
            ));
        }

        payload.push_str("]}]}");
        payload
=======
    /// Export metrics in Collectd binary / RRD-style metric payload format
    pub fn export_collectd_metrics(
        &self,
        top: &TopHtopProcessMonitorEngine,
        vmstat: &VmstatIostatSarMetricsCollectorEngine,
    ) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "PUTVAL \"{}/cpu-0/cpu-user\" interval=10 N:{}\n",
            self.hostname, top.total_cpu_user_pct
        ));
        out.push_str(&format!(
            "PUTVAL \"{}/cpu-0/cpu-system\" interval=10 N:{}\n",
            self.hostname, top.total_cpu_sys_pct
        ));
        if let Some(vm) = vmstat.vmstat_history.last() {
            out.push_str(&format!(
                "PUTVAL \"{}/memory/memory-free\" interval=10 N:{}\n",
                self.hostname,
                vm.free_kb * 1024
            ));
        }
        out
    }
}

// ============================================================================
// 8. DIAGNOSTICS PULL REQUEST GATEWAY ENGINE
// ============================================================================

/// Diagnostics PR Format Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticsPrFormat {
    DiagnosticPlugin,
    MetricExporter,
    LogParserRule,
    HardwareProbe,
}

/// Diagnostics PR Submission
#[derive(Debug, Clone)]
pub struct DiagnosticsPrSubmission {
    pub pr_id: String,
    pub author: String,
    pub title: String,
    pub format: DiagnosticsPrFormat,
    pub description: String,
    pub code_patch: String,
    pub pqc_signature: String,
    pub is_gated: bool,
    pub is_merged: bool,
}

/// Logging, Monitoring & Diagnostics PR Gateway Engine
#[derive(Debug, Clone, Default)]
pub struct LoggingMonitoringDiagnosticsPullRequestEngine {
    pub submissions: Vec<DiagnosticsPrSubmission>,
}

impl LoggingMonitoringDiagnosticsPullRequestEngine {
    pub fn new() -> Self {
        Self {
            submissions: Vec::new(),
        }
    }

    pub fn submit_pr(
        &mut self,
        author: &str,
        title: &str,
        format: DiagnosticsPrFormat,
        description: &str,
        code_patch: &str,
        pqc_signature: &str,
    ) -> String {
        let pr_id = format!("diag-pr-{}", self.submissions.len() + 1);
        let submission = DiagnosticsPrSubmission {
            pr_id: pr_id.clone(),
            author: author.to_string(),
            title: title.to_string(),
            format,
            description: description.to_string(),
            code_patch: code_patch.to_string(),
            pqc_signature: pqc_signature.to_string(),
            is_gated: false,
            is_merged: false,
        };
        self.submissions.push(submission);
        pr_id
    }

    pub fn run_automated_ci_pr_gate(&mut self, pr_id: &str) -> bool {
        if let Some(sub) = self.submissions.iter_mut().find(|s| s.pr_id == pr_id) {
            if sub.pqc_signature.is_empty() || sub.code_patch.is_empty() {
                sub.is_gated = false;
                return false;
            }
            sub.is_gated = true;
            true
        } else {
            false
        }
    }

    pub fn merge_pr(&mut self, pr_id: &str) -> bool {
        if let Some(sub) = self.submissions.iter_mut().find(|s| s.pr_id == pr_id) {
            if sub.is_gated {
                sub.is_merged = true;
                return true;
            }
        }
        false
>>>>>>> origin/main
    }
}

// ============================================================================
// MASTER LOGGING, MONITORING & DIAGNOSTICS SUITE
// ============================================================================

/// Master Sovereign Linux & BSD Logging, Monitoring, and Diagnostics Suite
#[derive(Debug, Clone)]
pub struct SovereignLinuxBsdLoggingMonitoringDiagnosticsSuite {
    pub journal: JournaldRsyslogLogEngine,
    pub routing: SyslogNgRsyslogRoutingEngine,
    pub dmesg: DmesgKernelRingBufferEngine,
    pub top_monitor: TopHtopProcessMonitorEngine,
    pub io_net_monitor: IotopNethogsProcessIoNetMonitor,
    pub vmstat_collector: VmstatIostatSarMetricsCollectorEngine,
    pub lsof_inspector: LsofOpenFilesInspectorEngine,
    pub file_locks: FuserLslocksInspector,
    pub hardware: HardwareDiscoveryInspectorEngine,
    pub exporter: SosreportPrometheusExporterEngine,
    pub pr_gateway: LoggingMonitoringDiagnosticsPullRequestEngine,
}

impl SovereignLinuxBsdLoggingMonitoringDiagnosticsSuite {
    pub fn new(hostname: &str) -> Self {
        Self {
            journal: JournaldRsyslogLogEngine::new(hostname),
            routing: SyslogNgRsyslogRoutingEngine::new(),
            dmesg: DmesgKernelRingBufferEngine::new(1000),
            top_monitor: TopHtopProcessMonitorEngine::new(),
            io_net_monitor: IotopNethogsProcessIoNetMonitor::new(),
            vmstat_collector: VmstatIostatSarMetricsCollectorEngine::new(),
            lsof_inspector: LsofOpenFilesInspectorEngine::new(),
            file_locks: FuserLslocksInspector::new(),
            hardware: HardwareDiscoveryInspectorEngine::new(),
            exporter: SosreportPrometheusExporterEngine::new(hostname),
            pr_gateway: LoggingMonitoringDiagnosticsPullRequestEngine::new(),
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
<<<<<<< HEAD
            "routing_destinations".to_string(),
            self.routing.rules.len().to_string(),
=======
            "journal_seal_valid".to_string(),
            self.journal.verify_journal_sealing_integrity().to_string(),
>>>>>>> origin/main
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
            "monitored_io_processes".to_string(),
            self.io_net_monitor.io_stats.len().to_string(),
        );
        diag.insert(
            "open_file_descriptors".to_string(),
            self.lsof_inspector.open_files.len().to_string(),
        );
        diag.insert(
            "active_file_locks".to_string(),
            self.file_locks.locks.len().to_string(),
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
            "cpu_online_cores".to_string(),
            self.hardware.cpu_topology.online_cpus.to_string(),
        );
        diag.insert(
            "bios_vendor".to_string(),
            self.hardware.dmi_info.bios_vendor.clone(),
        );
        diag
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
            Some("kernel.service"),
            "Out of memory: Kill process 1024",
            meta,
        );

        assert_eq!(id, 1);
        assert_eq!(journal.entries.len(), 1);
        assert!(journal.verify_journal_sealing_integrity());

        let rfc5424 = journal.entries[0].to_rfc5424(&journal.hostname);
        assert!(rfc5424.contains("sigma-node-1"));
        assert!(rfc5424.contains("Out of memory"));

        let queried = journal.query_journalctl(
            Some("kernel.service"),
            Some("kernel"),
            Some(LogSeverity::Error),
            None,
            10,
        );
        assert_eq!(queried.len(), 1);
    }

    #[test]
    fn test_syslog_ng_routing_engine() {
        let routing = SyslogNgRsyslogRoutingEngine::new();
        let entry = JournalLogEntry {
            id: 1,
            timestamp_us: 1000000,
            facility: SyslogFacility::AuthPriv,
            severity: LogSeverity::Error,
            pid: 400,
            app_name: "sshd".to_string(),
            message: "Failed password for root".to_string(),
            metadata: BTreeMap::new(),
        };

        let routed = routing.route_entry(&entry, "sigma-node-1");
        assert!(!routed.is_empty());
        let json_fmt = routing.format_json(&entry, "sigma-node-1");
        assert!(json_fmt.contains("sshd"));
        assert!(json_fmt.contains("Failed password"));
    }

    #[test]
    fn test_dmesg_ring_buffer() {
        let mut dmesg = DmesgKernelRingBufferEngine::new(10);
        assert!(!dmesg.buffer.is_empty());

        dmesg.print_klog(
            SyslogFacility::Kernel,
            LogSeverity::Warning,
            "usb",
            "Overcurrent condition on port 1",
        );
        let filtered = dmesg.filter(
            Some(LogSeverity::Warning),
            Some(SyslogFacility::Kernel),
            Some("usb"),
        );
        assert!(!filtered.is_empty());
        assert!(filtered[0].format_ansi().contains("usb"));
    }

    #[test]
    fn test_top_htop_process_monitor() {
        let mut monitor = TopHtopProcessMonitorEngine::new();
        assert!(!monitor.processes.is_empty());

        let top_cpu = monitor.sort_by_cpu();
        assert!(top_cpu[0].cpu_usage_pct >= top_cpu[1].cpu_usage_pct);

        assert!(monitor.renice_process(540, -10));
        assert_eq!(monitor.processes.get(&540).unwrap().nice_priority, -10);

        let tree = monitor.build_process_tree();
        assert!(tree.contains_key(&1));
    }

    #[test]
    fn test_iotop_nethogs_monitor() {
        let io_net = IotopNethogsProcessIoNetMonitor::new();
        let top_io = io_net.top_io_consumers();
        assert_eq!(top_io.len(), 1);
        assert_eq!(top_io[0].pid, 540);

        let top_net = io_net.top_net_consumers();
        assert_eq!(top_net.len(), 1);
        assert_eq!(top_net[0].pid, 540);
    }

    #[test]
    fn test_vmstat_iostat_sar() {
        let collector = VmstatIostatSarMetricsCollectorEngine::new();
        assert!(!collector.vmstat_history.is_empty());
        assert!(!collector.iostat_history.is_empty());
        assert!(!collector.core_snapshots.is_empty());

        let (user, sys, idle) = collector.calculate_sar_average_cpu();
        assert!(user > 0.0);
        assert!(sys >= 0.0);
        assert!(idle > 0.0);

        let sar_bin = collector.export_sar_binary_payload();
        assert!(sar_bin.starts_with(b"SAR_BIN_V1"));
    }

    #[test]
    fn test_lsof_fuser_lslocks() {
        let lsof = LsofOpenFilesInspectorEngine::new();
        let sys_files = lsof.find_by_pid(1);
        assert_eq!(sys_files.len(), 1);

<<<<<<< HEAD
        let locks = FuserLslocksInspector::new();
        let journal_locks = locks.find_locks_for_path("/var/log/journal/system.journal");
        assert_eq!(journal_locks.len(), 1);

        let mount_pids = locks.find_processes_holding_mount(&lsof, "/");
        assert!(mount_pids.contains(&1));
=======
        let socket_files = lsof.find_by_path("@wayland-0");
        assert_eq!(socket_files.len(), 1);

        let listen_sockets = lsof.find_sockets_by_state(SocketState::Listen);
        assert_eq!(listen_sockets.len(), 1);
>>>>>>> origin/main
    }

    #[test]
    fn test_hardware_discovery() {
        let hw = HardwareDiscoveryInspectorEngine::new();
        assert_eq!(hw.pci_devices.len(), 2);
        assert_eq!(hw.usb_devices.len(), 1);
        assert_eq!(hw.dmi_info.bios_vendor, "American Megatrends Inc.");
<<<<<<< HEAD
        assert_eq!(hw.cpu_topology.online_cpus, 64);
        assert_eq!(hw.block_devices[0].name, "nvme0n1");
        assert!(hw.block_devices[0].smart_passed);
        assert_eq!(hw.thermal_sensors[0].sensor_name, "cpu_package_thermal");
=======

        let nvidia = hw.query_pci_by_driver("nvidia");
        assert_eq!(nvidia.len(), 1);
>>>>>>> origin/main
    }

    #[test]
    fn test_sosreport_prometheus_loki_export() {
        let suite = SovereignLinuxBsdLoggingMonitoringDiagnosticsSuite::new("test-host");
        let sos = suite
            .exporter
            .generate_sosreport(&suite.journal, &suite.dmesg, &suite.hardware);
        assert!(sos.captured_files.contains_key("var/log/dmesg"));
        assert!(sos.captured_files.contains_key("etc/fstab"));

        let prom = suite
            .exporter
            .export_prometheus_metrics(&suite.top_monitor, &suite.vmstat_collector);
        assert!(prom.contains("node_cpu_seconds_total"));
        assert!(prom.contains("node_memory_MemFree_bytes"));

<<<<<<< HEAD
        let loki = suite.exporter.export_loki_log_stream(&suite.journal);
        assert!(loki.contains("test-host"));
=======
        let collectd = suite
            .exporter
            .export_collectd_metrics(&suite.top_monitor, &suite.vmstat_collector);
        assert!(collectd.contains("PUTVAL \"test-host/cpu-0/cpu-user\""));
    }

    #[test]
    fn test_diagnostics_pr_gateway() {
        let mut pr_engine = LoggingMonitoringDiagnosticsPullRequestEngine::new();
        let pr_id = pr_engine.submit_pr(
            "dev@sigmaos.org",
            "Add eBPF XDP Packet Counter Exporter",
            DiagnosticsPrFormat::MetricExporter,
            "Adds eBPF XDP kernel metrics exporter to Prometheus node exporter",
            "+ fn export_xdp_metrics() {}",
            "PQC-SIG-DILITHIUM5-39201",
        );

        assert!(pr_engine.run_automated_ci_pr_gate(&pr_id));
        assert!(pr_engine.merge_pr(&pr_id));
>>>>>>> origin/main
    }
}
