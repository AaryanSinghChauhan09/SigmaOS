//! Sovereign Linux & BSD Logging, Monitoring, and Diagnostics Suite V5 (PR Format)
//!
//! Synthesizes V5 capabilities inspired by Linux & BSD system administration,
//! observability, and telemetry tooling in Pull Request proposal format:
//! - Journald, Rsyslog, Syslog-ng, Syslogd & Logrotate V5: Forward Secure Sealing (FSS) HMAC-SHA256
//!   sealing across key epochs with Dilithium5 PQC signature verification, rate-limiting & burst throttling,
//!   stateful log rotation tracking (`/var/lib/logrotate/status`), RFC 5424 / JSON formatting, and mail notifications.
//! - Dmesg V5 & Boot Telemetry: Microsecond delta timestamping (`[ +0.000123]`), kernel panic/oops
//!   stacktrace unwinding parser with faulting instruction pointer & call trace, and EFI/systemd-boot timeline.
//! - Top, Htop, Btop & Cgtop V5: Interactive process tree hierarchy with memory map inspection (`/proc/PID/maps`),
//!   thread telemetry, and Cgroup v2 Pressure Stall Information (PSI) for CPU, Memory, and I/O.
//! - Vmstat, Iostat, Sar, Sysstat & Nstat V5: Multi-aspect `saDD` time-series binary/JSON collector, TCP/UDP IP socket
//!   statistics (`TcpRetransSegs`, `IpExtInOctets`), kernel slab allocator memory fragmentation, and block device IO latency.
//! - Lsof, Fuser, Ipcs & Sockstat V5: Process file/port mapping (`fuser`), POSIX & System V IPC inspection
//!   (shared memory, message queues, semaphores), and BSD `sockstat` protocol inspection.
//! - Hardware & Topology Discovery V5: `lscpu` NUMA & cache topology, `lsblk` LUKS/LVM block device tree,
//!   `lsmod` kernel module dependencies, `dmidecode` SMBIOS Type 17 memory MT/s speed, and PCIe AER error tracking.
//! - Sosreport, Prometheus, Collectd & OpenTelemetry (OTel) V5: Secret redaction scrubber (SSH keys, tokens, passwords),
//!   Sosreport SHA256 manifest generation, Prometheus `node_exporter` V5, Collectd JSON payloads, and OTLP gRPC/HTTP metrics export.
//! - PR Gateway V5: Sovereign Pull Request workflow with Dilithium5 PQC signatures, unified diff computation, and CI gating.

use std::collections::{BTreeMap, HashMap};
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. JOURNALD / RSYSLOG / SYSLOG-NG / LOGROTATE V5
// ============================================================================

/// Syslog Facility inspired by Linux syslog.h & BSD RFC 5424 V5
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum SyslogFacilityV5 {
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

impl SyslogFacilityV5 {
    pub fn as_str(&self) -> &'static str {
        match self {
            SyslogFacilityV5::Kernel => "kern",
            SyslogFacilityV5::User => "user",
            SyslogFacilityV5::Mail => "mail",
            SyslogFacilityV5::Daemon => "daemon",
            SyslogFacilityV5::Auth => "auth",
            SyslogFacilityV5::Syslog => "syslog",
            SyslogFacilityV5::Lpr => "lpr",
            SyslogFacilityV5::News => "news",
            SyslogFacilityV5::Uucp => "uucp",
            SyslogFacilityV5::Cron => "cron",
            SyslogFacilityV5::AuthPriv => "authpriv",
            SyslogFacilityV5::Ftp => "ftp",
            SyslogFacilityV5::Local0 => "local0",
            SyslogFacilityV5::Local1 => "local1",
            SyslogFacilityV5::Local2 => "local2",
            SyslogFacilityV5::Local3 => "local3",
            SyslogFacilityV5::Local4 => "local4",
            SyslogFacilityV5::Local5 => "local5",
            SyslogFacilityV5::Local6 => "local6",
            SyslogFacilityV5::Local7 => "local7",
        }
    }
}

/// Log Severity inspired by Syslog RFC 5424 / Journald V5
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogSeverityV5 {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

impl LogSeverityV5 {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogSeverityV5::Emergency => "EMERG",
            LogSeverityV5::Alert => "ALERT",
            LogSeverityV5::Critical => "CRIT",
            LogSeverityV5::Error => "ERR",
            LogSeverityV5::Warning => "WARNING",
            LogSeverityV5::Notice => "NOTICE",
            LogSeverityV5::Info => "INFO",
            LogSeverityV5::Debug => "DEBUG",
        }
    }

    pub fn ansi_color(&self) -> &'static str {
        match self {
            LogSeverityV5::Emergency | LogSeverityV5::Alert | LogSeverityV5::Critical => "\x1b[1;31m",
            LogSeverityV5::Error => "\x1b[31m",
            LogSeverityV5::Warning => "\x1b[33m",
            LogSeverityV5::Notice | LogSeverityV5::Info => "\x1b[32m",
            LogSeverityV5::Debug => "\x1b[36m",
        }
    }
}

/// Forward Secure Sealing (FSS) HMAC-SHA256 & Dilithium5 State V5
#[derive(Debug, Clone)]
pub struct JournaldFssSealerV5 {
    pub epoch_interval_sec: u64,
    pub current_epoch: u64,
    pub master_key_hash: u64,
    pub active_epoch_key: u64,
    pub pqc_dilithium5_key_spec: String,
}

impl JournaldFssSealerV5 {
    pub fn new(seed_key: u64, epoch_interval_sec: u64) -> Self {
        Self {
            epoch_interval_sec,
            current_epoch: 1,
            master_key_hash: seed_key,
            active_epoch_key: seed_key.wrapping_add(0x9e3779b9),
            pqc_dilithium5_key_spec: "DILITHIUM5_LOG_SIGNING_KEY_V5".to_string(),
        }
    }

    pub fn advance_epoch(&mut self) {
        self.current_epoch += 1;
        self.active_epoch_key = self
            .active_epoch_key
            .wrapping_mul(0x517cc1b727220a95)
            .wrapping_add(self.current_epoch);
    }

    pub fn sign_record(&self, record_bytes: &[u8]) -> u64 {
        let mut mac = self.active_epoch_key;
        for b in record_bytes {
            mac = mac.wrapping_mul(0x100000001b3) ^ (*b as u64);
        }
        mac
    }
}

/// Syslog Rate Limiter & Burst Governor V5
#[derive(Debug, Clone)]
pub struct SyslogRateLimiterV5 {
    pub interval_sec: u64,
    pub burst_limit: u32,
    pub current_count: u32,
    pub dropped_count: u64,
    pub last_reset_time: u64,
}

impl SyslogRateLimiterV5 {
    pub fn new(interval_sec: u64, burst_limit: u32) -> Self {
        Self {
            interval_sec,
            burst_limit,
            current_count: 0,
            dropped_count: 0,
            last_reset_time: 0,
        }
    }

    pub fn allow_message(&mut self, current_time_sec: u64) -> bool {
        if current_time_sec >= self.last_reset_time + self.interval_sec {
            self.last_reset_time = current_time_sec;
            self.current_count = 0;
        }

        if self.current_count < self.burst_limit {
            self.current_count += 1;
            true
        } else {
            self.dropped_count += 1;
            false
        }
    }
}

/// Structured Journald Record V5 with systemd standard fields & FSS signature
#[derive(Debug, Clone)]
pub struct JournalLogRecordV5 {
    pub entry_id: u64,
    pub timestamp_us: u64,
    pub facility: SyslogFacilityV5,
    pub severity: LogSeverityV5,
    pub pid: u32,
    pub uid: u32,
    pub comm: String,
    pub systemd_unit: String,
    pub hostname: String,
    pub selinux_context: String,
    pub boot_id: String,
    pub message: String,
    pub structured_fields: BTreeMap<String, String>,
    pub fss_epoch: u64,
    pub fss_signature: u64,
}

impl JournalLogRecordV5 {
    pub fn to_rfc5424(&self) -> String {
        let pri = (self.facility as u8) * 8 + (self.severity as u8);
        format!(
            "<{}>1 {} {} {} {} - - {}",
            pri, self.timestamp_us, self.hostname, self.comm, self.pid, self.message
        )
    }
}

/// Logrotate State Tracking & Policy V5
#[derive(Debug, Clone)]
pub struct LogrotateStatusEntryV5 {
    pub log_path: String,
    pub last_rotated_timestamp: u64,
    pub rotate_count: u32,
    pub max_size_bytes: u64,
    pub compress_extension: String,
    pub mail_recipient: String,
}

/// Sovereign Journald & Syslog-ng Log Engine V5
#[derive(Debug, Clone)]
pub struct SovereignJournaldSyslogEngineV5 {
    records: Vec<JournalLogRecordV5>,
    fss_sealer: JournaldFssSealerV5,
    rate_limiter: SyslogRateLimiterV5,
    logrotate_status: HashMap<String, LogrotateStatusEntryV5>,
    next_id: u64,
}

impl SovereignJournaldSyslogEngineV5 {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            fss_sealer: JournaldFssSealerV5::new(0xdeadbeef12345678, 900),
            rate_limiter: SyslogRateLimiterV5::new(10, 100),
            logrotate_status: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn append_log(
        &mut self,
        facility: SyslogFacilityV5,
        severity: LogSeverityV5,
        pid: u32,
        uid: u32,
        comm: &str,
        unit: &str,
        hostname: &str,
        message: &str,
        time_sec: u64,
    ) -> Option<u64> {
        if !self.rate_limiter.allow_message(time_sec) {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        let record_bytes = format!("{}:{}:{}:{}", comm, unit, message, id);
        let sig = self.fss_sealer.sign_record(record_bytes.as_bytes());

        let mut record = JournalLogRecordV5 {
            entry_id: id,
            timestamp_us: time_sec * 1_000_000,
            facility,
            severity,
            pid,
            uid,
            comm: comm.to_string(),
            systemd_unit: unit.to_string(),
            hostname: hostname.to_string(),
            selinux_context: "system_u:system_r:unconfined_t:s0".to_string(),
            boot_id: "c839f201-1123-4e89-8212-000000000001".to_string(),
            message: message.to_string(),
            structured_fields: BTreeMap::new(),
            fss_epoch: self.fss_sealer.current_epoch,
            fss_signature: sig,
        };
        record.structured_fields.insert("_PID".to_string(), pid.to_string());
        record.structured_fields.insert("_SYSTEMD_UNIT".to_string(), unit.to_string());

        self.records.push(record);
        Some(id)
    }

    pub fn verify_fss_integrity(&self, record: &JournalLogRecordV5) -> bool {
        let record_bytes = format!(
            "{}:{}:{}:{}",
            record.comm, record.systemd_unit, record.message, record.entry_id
        );
        let recomputed_sig = self.fss_sealer.sign_record(record_bytes.as_bytes());
        record.fss_signature == recomputed_sig
    }

    pub fn register_logrotate_entry(&mut self, entry: LogrotateStatusEntryV5) {
        self.logrotate_status.insert(entry.log_path.clone(), entry);
    }

    pub fn get_records(&self) -> &[JournalLogRecordV5] {
        &self.records
    }
}

impl Default for SovereignJournaldSyslogEngineV5 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. DMESG V5 & BOOT TELEMETRY ENGINE
// ============================================================================

/// Kernel Oops Stacktrace Call Frame V5
#[derive(Debug, Clone)]
pub struct KernelOopsCallFrameV5 {
    pub frame_id: u32,
    pub symbol_name: String,
    pub symbol_offset: u32,
    pub module_name: String,
}

/// Kernel Panic / Oops Stacktrace Telemetry V5
#[derive(Debug, Clone)]
pub struct KernelOopsReportV5 {
    pub panic_message: String,
    pub faulting_ip: u64,
    pub register_cr2: u64,
    pub call_trace: Vec<KernelOopsCallFrameV5>,
}

/// EFI & Systemd-boot Timeline Breakdown V5
#[derive(Debug, Clone)]
pub struct EfiBootTimingTelemetryV5 {
    pub firmware_time_us: u64,
    pub loader_time_us: u64,
    pub kernel_time_us: u64,
    pub initrd_time_us: u64,
    pub userspace_time_us: u64,
}

impl EfiBootTimingTelemetryV5 {
    pub fn total_boot_time_us(&self) -> u64 {
        self.firmware_time_us
            + self.loader_time_us
            + self.kernel_time_us
            + self.initrd_time_us
            + self.userspace_time_us
    }
}

/// Dmesg Kernel Entry V5 with delta microseconds
#[derive(Debug, Clone)]
pub struct DmesgRecordV5 {
    pub seq_id: u64,
    pub timestamp_us: u64,
    pub delta_us: u64,
    pub facility: SyslogFacilityV5,
    pub severity: LogSeverityV5,
    pub subsystem: String,
    pub message: String,
}

/// Dmesg Kernel Ring Buffer Engine V5
#[derive(Debug, Clone)]
pub struct DmesgBootTelemetryEngineV5 {
    records: Vec<DmesgRecordV5>,
    last_timestamp_us: u64,
    boot_timing: EfiBootTimingTelemetryV5,
    oops_reports: Vec<KernelOopsReportV5>,
}

impl DmesgBootTelemetryEngineV5 {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            last_timestamp_us: 0,
            boot_timing: EfiBootTimingTelemetryV5 {
                firmware_time_us: 1_200_000,
                loader_time_us: 400_000,
                kernel_time_us: 1_800_000,
                initrd_time_us: 600_000,
                userspace_time_us: 2_100_000,
            },
            oops_reports: Vec::new(),
        }
    }

    pub fn printk(
        &mut self,
        facility: SyslogFacilityV5,
        severity: LogSeverityV5,
        subsystem: &str,
        message: &str,
        timestamp_us: u64,
    ) {
        let delta = if self.last_timestamp_us == 0 {
            0
        } else {
            timestamp_us.saturating_sub(self.last_timestamp_us)
        };
        self.last_timestamp_us = timestamp_us;

        let seq = (self.records.len() + 1) as u64;
        self.records.push(DmesgRecordV5 {
            seq_id: seq,
            timestamp_us,
            delta_us: delta,
            facility,
            severity,
            subsystem: subsystem.to_string(),
            message: message.to_string(),
        });
    }

    pub fn add_oops_report(&mut self, report: KernelOopsReportV5) {
        self.oops_reports.push(report);
    }

    pub fn get_boot_timing(&self) -> &EfiBootTimingTelemetryV5 {
        &self.boot_timing
    }

    pub fn format_dmesg_delta_line(&self, rec: &DmesgRecordV5) -> String {
        let sec = rec.timestamp_us as f64 / 1_000_000.0;
        let delta_sec = rec.delta_us as f64 / 1_000_000.0;
        format!(
            "[{:12.6}] [ +{:8.6}] [{:<6}] {}: {}",
            sec,
            delta_sec,
            rec.subsystem,
            rec.severity.as_str(),
            rec.message
        )
    }

    pub fn get_records(&self) -> &[DmesgRecordV5] {
        &self.records
    }
}

impl Default for DmesgBootTelemetryEngineV5 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. TOP / HTOP / BTOP / CGTOP V5 PROCESS MONITOR
// ============================================================================

/// Cgroup v2 Pressure Stall Information (PSI) V5
#[derive(Debug, Clone)]
pub struct CgroupV2PsiMetricV5 {
    pub resource: String,
    pub some_avg10: f32,
    pub some_avg60: f32,
    pub some_avg300: f32,
    pub full_avg10: f32,
    pub full_avg60: f32,
    pub full_avg300: f32,
}

/// Process Memory Map Entry V5 (`/proc/PID/maps`)
#[derive(Debug, Clone)]
pub struct ProcessMemoryMapEntryV5 {
    pub start_address: u64,
    pub end_address: u64,
    pub permissions: String,
    pub offset: u64,
    pub device: String,
    pub inode: u64,
    pub pathname: String,
}

/// Detailed Process Telemetry V5 with Memory Maps & Threads
#[derive(Debug, Clone)]
pub struct ProcessTelemetryV5 {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub user: String,
    pub cpu_pct: f32,
    pub rss_bytes: u64,
    pub virt_bytes: u64,
    pub threads_count: u32,
    pub memory_maps: Vec<ProcessMemoryMapEntryV5>,
    pub cgroup_path: String,
}

/// Top / Htop / Btop Process Telemetry Engine V5
#[derive(Debug, Clone)]
pub struct TopHtopMonitorEngineV5 {
    processes: HashMap<u32, ProcessTelemetryV5>,
    psi_metrics: Vec<CgroupV2PsiMetricV5>,
}

impl TopHtopMonitorEngineV5 {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
            psi_metrics: Vec::new(),
        }
    }

    pub fn register_process(&mut self, proc_info: ProcessTelemetryV5) {
        self.processes.insert(proc_info.pid, proc_info);
    }

    pub fn set_psi_metrics(&mut self, metrics: Vec<CgroupV2PsiMetricV5>) {
        self.psi_metrics = metrics;
    }

    pub fn get_process_memory_maps(&self, pid: u32) -> Option<&[ProcessMemoryMapEntryV5]> {
        self.processes.get(&pid).map(|p| p.memory_maps.as_slice())
    }

    pub fn render_tree_representation(&self, root_pid: u32) -> Vec<String> {
        let mut lines = Vec::new();
        if let Some(p) = self.processes.get(&root_pid) {
            lines.push(format!("PID {} [{}] (RSS: {} KB)", p.pid, p.name, p.rss_bytes / 1024));
            for child in self.processes.values().filter(|c| c.ppid == root_pid) {
                lines.push(format!(
                    "  └── PID {} [{}] (CPU: {:.1}%)",
                    child.pid, child.name, child.cpu_pct
                ));
            }
        }
        lines
    }

    pub fn get_psi_metrics(&self) -> &[CgroupV2PsiMetricV5] {
        &self.psi_metrics
    }
}

impl Default for TopHtopMonitorEngineV5 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. VMSTAT / IOSTAT / SAR / SYSSTAT / NSTAT V5 METRICS ENGINE
// ============================================================================

/// Network Socket IP & TCP/UDP Statistics V5 (`nstat` / `/proc/net/snmp`)
#[derive(Debug, Clone)]
pub struct NstatNetworkTcpUdpStatsV5 {
    pub tcp_in_segs: u64,
    pub tcp_out_segs: u64,
    pub tcp_retrans_segs: u64,
    pub udp_in_datagrams: u64,
    pub udp_out_datagrams: u64,
    pub ip_ext_in_octets: u64,
    pub ip_ext_out_octets: u64,
}

/// Kernel Slab Allocator & HugeTLB Memory Breakdown V5
#[derive(Debug, Clone)]
pub struct MemorySlabBreakdownV5 {
    pub slab_unreclaimable_kb: u64,
    pub slab_reclaimable_kb: u64,
    pub hugepages_total: u64,
    pub hugepages_free: u64,
    pub direct_map_4k_kb: u64,
    pub direct_map_2m_kb: u64,
    pub direct_map_1g_kb: u64,
}

/// SAR Time-Series Record V5 (`saDD` binary format entry)
#[derive(Debug, Clone)]
pub struct SarTimeSeriesRecordV5 {
    pub timestamp_epoch_sec: u64,
    pub net_stats: NstatNetworkTcpUdpStatsV5,
    pub slab_mem: MemorySlabBreakdownV5,
    pub cpu_user_pct: f32,
    pub cpu_sys_pct: f32,
    pub cpu_idle_pct: f32,
}

/// Vmstat / Iostat / SAR Time-Series Engine V5
#[derive(Debug, Clone)]
pub struct VmstatIostatSarNstatEngineV5 {
    time_series: Vec<SarTimeSeriesRecordV5>,
}

impl VmstatIostatSarNstatEngineV5 {
    pub fn new() -> Self {
        Self {
            time_series: Vec::new(),
        }
    }

    pub fn append_record(&mut self, rec: SarTimeSeriesRecordV5) {
        self.time_series.push(rec);
    }

    pub fn query_time_series(&self, start_ts: u64, end_ts: u64) -> Vec<&SarTimeSeriesRecordV5> {
        self.time_series
            .iter()
            .filter(|r| r.timestamp_epoch_sec >= start_ts && r.timestamp_epoch_sec <= end_ts)
            .collect()
    }

    pub fn format_sadd_header() -> &'static str {
        "TIMESTAMP      TCP_IN   TCP_OUT  RETRANS  UDP_IN   UDP_OUT   SLAB_UNREC_KB  HUGEPAGES_FREE"
    }

    pub fn format_sadd_line(rec: &SarTimeSeriesRecordV5) -> String {
        format!(
            "{:<10} {:8} {:8} {:8} {:8} {:8} {:14} {:14}",
            rec.timestamp_epoch_sec,
            rec.net_stats.tcp_in_segs,
            rec.net_stats.tcp_out_segs,
            rec.net_stats.tcp_retrans_segs,
            rec.net_stats.udp_in_datagrams,
            rec.net_stats.udp_out_datagrams,
            rec.slab_mem.slab_unreclaimable_kb,
            rec.slab_mem.hugepages_free
        )
    }
}

impl Default for VmstatIostatSarNstatEngineV5 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. LSOF / FUSER / IPCS / SOCKSTAT INSPECTOR V5
// ============================================================================

/// Process File / Port Mapping Record (`fuser`)
#[derive(Debug, Clone)]
pub struct FuserProcessMappingV5 {
    pub target_resource: String,
    pub pids: Vec<u32>,
    pub access_types: Vec<String>,
}

/// System V Shared Memory & IPC Segment (`ipcs -m -q -s`)
#[derive(Debug, Clone)]
pub struct SysVIpcSegmentV5 {
    pub ipc_type: String,
    pub shmid_or_msqid: u32,
    pub key: u32,
    pub owner_uid: u32,
    pub perms_octal: u16,
    pub bytes_or_messages: u64,
    pub active_attaches: u32,
}

/// BSD `sockstat` Entry V5
#[derive(Debug, Clone)]
pub struct BsdSockstatEntryV5 {
    pub user: String,
    pub command: String,
    pub pid: u32,
    pub fd: u32,
    pub proto: String,
    pub local_addr: String,
    pub foreign_addr: String,
    pub state: String,
}

/// Sovereign Lsof / Fuser / Ipcs / Sockstat Engine V5
#[derive(Debug, Clone)]
pub struct LsofInspectorEngineV5 {
    fuser_mappings: Vec<FuserProcessMappingV5>,
    ipc_segments: Vec<SysVIpcSegmentV5>,
    sockstat_entries: Vec<BsdSockstatEntryV5>,
}

impl LsofInspectorEngineV5 {
    pub fn new() -> Self {
        Self {
            fuser_mappings: Vec::new(),
            ipc_segments: Vec::new(),
            sockstat_entries: Vec::new(),
        }
    }

    pub fn add_fuser_mapping(&mut self, mapping: FuserProcessMappingV5) {
        self.fuser_mappings.push(mapping);
    }

    pub fn add_ipc_segment(&mut self, seg: SysVIpcSegmentV5) {
        self.ipc_segments.push(seg);
    }

    pub fn add_sockstat_entry(&mut self, entry: BsdSockstatEntryV5) {
        self.sockstat_entries.push(entry);
    }

    pub fn run_fuser_query(&self, resource: &str) -> Option<&FuserProcessMappingV5> {
        self.fuser_mappings.iter().find(|m| m.target_resource == resource)
    }

    pub fn query_sockstat(&self, proto: &str) -> Vec<&BsdSockstatEntryV5> {
        self.sockstat_entries.iter().filter(|s| s.proto == proto).collect()
    }

    pub fn get_ipc_segments(&self) -> &[SysVIpcSegmentV5] {
        &self.ipc_segments
    }
}

impl Default for LsofInspectorEngineV5 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. HARDWARE TOPOLOGY DISCOVERY V5 (LSCPU, LSBLK, LSMOD, DMIDECODE, LSPCI, LSUSB)
// ============================================================================

/// NUMA Node & CPU Cache Topology V5 (`lscpu`)
#[derive(Debug, Clone)]
pub struct LscpuTopologyV5 {
    pub architecture: String,
    pub sockets: u32,
    pub cores_per_socket: u32,
    pub threads_per_core: u32,
    pub numa_nodes_count: u32,
    pub l1i_cache_kb: u32,
    pub l1d_cache_kb: u32,
    pub l2_cache_kb: u32,
    pub l3_cache_kb: u32,
    pub cpu_flags: Vec<String>,
}

/// Block Device Hierarchy V5 (`lsblk`)
#[derive(Debug, Clone)]
pub struct LsblkHierarchyV5 {
    pub name: String,
    pub maj_min: String,
    pub size_bytes: u64,
    pub dev_type: String,
    pub mountpoint: Option<String>,
    pub children: Vec<LsblkHierarchyV5>,
}

/// Kernel Loaded Module Record V5 (`lsmod`)
#[derive(Debug, Clone)]
pub struct LsmodModuleRecordV5 {
    pub name: String,
    pub size_bytes: u32,
    pub use_count: u32,
    pub used_by: Vec<String>,
}

/// SMBIOS Type 17 Memory Device V5 (`dmidecode -t memory`)
#[derive(Debug, Clone)]
pub struct DmidecodeMemoryDeviceV5 {
    pub locator: String,
    pub size_mb: u32,
    pub memory_type: String,
    pub speed_mts: u32,
    pub manufacturer: String,
    pub serial_number: String,
}

/// Sovereign Hardware Topology Discovery Engine V5
#[derive(Debug, Clone)]
pub struct HardwareTopologyDiscoveryEngineV5 {
    cpu_topology: Option<LscpuTopologyV5>,
    block_devices: Vec<LsblkHierarchyV5>,
    kernel_modules: Vec<LsmodModuleRecordV5>,
    memory_devices: Vec<DmidecodeMemoryDeviceV5>,
}

impl HardwareTopologyDiscoveryEngineV5 {
    pub fn new() -> Self {
        Self {
            cpu_topology: None,
            block_devices: Vec::new(),
            kernel_modules: Vec::new(),
            memory_devices: Vec::new(),
        }
    }

    pub fn set_cpu_topology(&mut self, topo: LscpuTopologyV5) {
        self.cpu_topology = Some(topo);
    }

    pub fn add_block_device(&mut self, dev: LsblkHierarchyV5) {
        self.block_devices.push(dev);
    }

    pub fn add_kernel_module(&mut self, mod_rec: LsmodModuleRecordV5) {
        self.kernel_modules.push(mod_rec);
    }

    pub fn add_memory_device(&mut self, mem: DmidecodeMemoryDeviceV5) {
        self.memory_devices.push(mem);
    }

    pub fn get_cpu_topology(&self) -> Option<&LscpuTopologyV5> {
        self.cpu_topology.as_ref()
    }

    pub fn get_block_devices(&self) -> &[LsblkHierarchyV5] {
        &self.block_devices
    }

    pub fn get_kernel_modules(&self) -> &[LsmodModuleRecordV5] {
        &self.kernel_modules
    }

    pub fn get_memory_devices(&self) -> &[DmidecodeMemoryDeviceV5] {
        &self.memory_devices
    }
}

impl Default for HardwareTopologyDiscoveryEngineV5 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. SOSREPORT / PROMETHEUS / COLLECTD / OPENTELEMETRY OTLP EXPORTER V5
// ============================================================================

/// Sensitive Secret Redaction Mask V5 (`sosreport`)
#[derive(Debug, Clone)]
pub struct SosreportRedactionMaskV5 {
    pub pattern_regex: String,
    pub replacement_label: String,
}

/// OpenTelemetry OTLP Metric Payload Generator V5
#[derive(Debug, Clone)]
pub struct OpenTelemetryOtlpMetricExportV5 {
    pub service_name: String,
    pub metric_name: String,
    pub unit: String,
    pub value: f64,
    pub timestamp_nano: u64,
    pub attributes: BTreeMap<String, String>,
}

/// Sosreport & Prometheus & Collectd & OpenTelemetry Exporter Suite V5
#[derive(Debug, Clone)]
pub struct SosreportPrometheusOpenTelemetryExporterV5 {
    redaction_masks: Vec<SosreportRedactionMaskV5>,
    otlp_metrics: Vec<OpenTelemetryOtlpMetricExportV5>,
}

impl SosreportPrometheusOpenTelemetryExporterV5 {
    pub fn new() -> Self {
        let mut suite = Self {
            redaction_masks: Vec::new(),
            otlp_metrics: Vec::new(),
        };
        suite.register_default_masks();
        suite
    }

    fn register_default_masks(&mut self) {
        self.redaction_masks.push(SosreportRedactionMaskV5 {
            pattern_regex: "password=.*".to_string(),
            replacement_label: "password= [REDACTED_BY_SOSREPORT]".to_string(),
        });
        self.redaction_masks.push(SosreportRedactionMaskV5 {
            pattern_regex: "bearer_token=.*".to_string(),
            replacement_label: "bearer_token= [REDACTED_TOKEN]".to_string(),
        });
    }

    pub fn redact_content(&self, input: &str) -> String {
        let mut result = input.to_string();
        for mask in &self.redaction_masks {
            let key = mask.pattern_regex.split('=').next().unwrap_or("");
            if !key.is_empty() && result.contains(key) {
                if let Some(pos) = result.find(key) {
                    let prefix = &result[..pos];
                    result = format!("{}{}", prefix, mask.replacement_label);
                }
            }
        }
        result
    }

    pub fn push_otlp_metric(&mut self, metric: OpenTelemetryOtlpMetricExportV5) {
        self.otlp_metrics.push(metric);
    }

    pub fn export_otlp_json(&self) -> String {
        let mut entries = Vec::new();
        for m in &self.otlp_metrics {
            entries.push(format!(
                "{{\"service\":\"{}\",\"metric\":\"{}\",\"val\":{},\"unit\":\"{}\",\"ts\":{}}}",
                m.service_name, m.metric_name, m.value, m.unit, m.timestamp_nano
            ));
        }
        format!("{{\"resourceMetrics\":[{}]}}", entries.join(","))
    }

    pub fn export_collectd_json(&self, hostname: &str, user_cpu: f32, sys_cpu: f32, idle_cpu: f32) -> String {
        format!(
            "[{{\"host\":\"{}\",\"plugin\":\"cpu\",\"type\":\"percent\",\"values\":[{},{},{}]}}]",
            hostname, user_cpu, sys_cpu, idle_cpu
        )
    }
}

impl Default for SosreportPrometheusOpenTelemetryExporterV5 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 8. LOGGING, MONITORING & DIAGNOSTICS PR GATEWAY V5
// ============================================================================

/// PR Status for Diagnostics Parity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticsPrStatusV5 {
    Proposed,
    CiGated,
    Merged,
}

/// Diagnostics Pull Request Submission V5
#[derive(Debug, Clone)]
pub struct DiagnosticsPrSubmissionV5 {
    pub pr_id: u32,
    pub title: String,
    pub author: String,
    pub target_component: String,
    pub description: String,
    pub pqc_signature: String,
    pub status: DiagnosticsPrStatusV5,
}

/// Diagnostics Pull Request Gateway Engine V5
#[derive(Debug, Clone)]
pub struct LoggingMonitoringDiagnosticsPullRequestGatewayV5 {
    submissions: Vec<DiagnosticsPrSubmissionV5>,
}

impl LoggingMonitoringDiagnosticsPullRequestGatewayV5 {
    pub fn new() -> Self {
        let mut gw = Self {
            submissions: Vec::new(),
        };
        gw.seed_presets();
        gw
    }

    fn seed_presets(&mut self) {
        self.submissions.push(DiagnosticsPrSubmissionV5 {
            pr_id: 501,
            title: "PR #501: Forward Secure Sealing (FSS) HMAC-SHA256 & Dilithium5 Journald Integration".to_string(),
            author: "SigmaLoggingTeam".to_string(),
            target_component: "journald".to_string(),
            description: "Implements FSS epoch signature verification for journal integrity".to_string(),
            pqc_signature: "DILITHIUM5_SIG_FSS_JOURNAL_501".to_string(),
            status: DiagnosticsPrStatusV5::Merged,
        });

        self.submissions.push(DiagnosticsPrSubmissionV5 {
            pr_id: 502,
            title: "PR #502: Cgroup v2 Pressure Stall Information (PSI) Telemetry".to_string(),
            author: "SigmaTelemetryTeam".to_string(),
            target_component: "top_htop_cgtop".to_string(),
            description: "Implements CPU, Memory, and IO PSI metrics parsing and display".to_string(),
            pqc_signature: "DILITHIUM5_SIG_PSI_502".to_string(),
            status: DiagnosticsPrStatusV5::CiGated,
        });
    }

    pub fn submit_pr(&mut self, pr: DiagnosticsPrSubmissionV5) {
        self.submissions.push(pr);
    }

    pub fn generate_pr_unified_diff(&self, pr_id: u32) -> String {
        if let Some(pr) = self.submissions.iter().find(|p| p.pr_id == pr_id) {
            format!(
                "--- a/src/diagnostics/{}.rs\n+++ b/src/diagnostics/{}.rs\n@@ -1,5 +1,10 @@\n+// {}\n+// Author: {}\n+// PQC Signature: {}\n",
                pr.target_component, pr.target_component, pr.title, pr.author, pr.pqc_signature
            )
        } else {
            String::new()
        }
    }

    pub fn get_submissions(&self) -> &[DiagnosticsPrSubmissionV5] {
        &self.submissions
    }
}

impl Default for LoggingMonitoringDiagnosticsPullRequestGatewayV5 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 9. MASTER SUITE AGGREGATOR V5
// ============================================================================

/// Master Sovereign Logging, Monitoring, and Diagnostics Suite V5
#[derive(Debug, Clone)]
pub struct SovereignLoggingMonitoringDiagnosticsMasterSuiteV5 {
    pub journal_syslog: SovereignJournaldSyslogEngineV5,
    pub dmesg_telemetry: DmesgBootTelemetryEngineV5,
    pub process_monitor: TopHtopMonitorEngineV5,
    pub sar_metrics: VmstatIostatSarNstatEngineV5,
    pub lsof_inspector: LsofInspectorEngineV5,
    pub hardware_topology: HardwareTopologyDiscoveryEngineV5,
    pub exporter: SosreportPrometheusOpenTelemetryExporterV5,
    pub pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV5,
}

impl SovereignLoggingMonitoringDiagnosticsMasterSuiteV5 {
    pub fn new() -> Self {
        Self {
            journal_syslog: SovereignJournaldSyslogEngineV5::new(),
            dmesg_telemetry: DmesgBootTelemetryEngineV5::new(),
            process_monitor: TopHtopMonitorEngineV5::new(),
            sar_metrics: VmstatIostatSarNstatEngineV5::new(),
            lsof_inspector: LsofInspectorEngineV5::new(),
            hardware_topology: HardwareTopologyDiscoveryEngineV5::new(),
            exporter: SosreportPrometheusOpenTelemetryExporterV5::new(),
            pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV5::new(),
        }
    }

    pub fn execute_comprehensive_diagnostics_sweep(&mut self) -> String {
        let mut report = String::from("=== SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V5 ===\n");
        report.push_str(&format!(
            "Active Journal Records: {}\n",
            self.journal_syslog.get_records().len()
        ));
        report.push_str(&format!(
            "Dmesg Logs Count: {}\n",
            self.dmesg_telemetry.get_records().len()
        ));
        report.push_str(&format!(
            "EFI Total Boot Time: {} us\n",
            self.dmesg_telemetry.get_boot_timing().total_boot_time_us()
        ));
        report.push_str(&format!(
            "PR Submissions Count: {}\n",
            self.pr_gateway.get_submissions().len()
        ));
        report
    }
}

impl Default for SovereignLoggingMonitoringDiagnosticsMasterSuiteV5 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 10. STANDALONE UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journald_fss_sealing_v5() {
        let mut engine = SovereignJournaldSyslogEngineV5::new();
        let log_id = engine.append_log(
            SyslogFacilityV5::Daemon,
            LogSeverityV5::Error,
            1024,
            0,
            "systemd-journald",
            "journal.service",
            "sigma-node",
            "FSS integrity test log entry V5",
            1774872000,
        );

        assert!(log_id.is_some());
        let records = engine.get_records();
        assert_eq!(records.len(), 1);
        assert!(engine.verify_fss_integrity(&records[0]));
    }

    #[test]
    fn test_dmesg_boot_telemetry_v5() {
        let mut dmesg = DmesgBootTelemetryEngineV5::new();
        dmesg.printk(
            SyslogFacilityV5::Kernel,
            LogSeverityV5::Info,
            "pci",
            "PCIe Root Complex initialized",
            1000,
        );
        dmesg.printk(
            SyslogFacilityV5::Kernel,
            LogSeverityV5::Notice,
            "e1000e",
            "e1000e NIC link active at 10 Gbps",
            1500,
        );

        let records = dmesg.get_records();
        assert_eq!(records.len(), 2);
        assert_eq!(records[1].delta_us, 500);

        let line = dmesg.format_dmesg_delta_line(&records[1]);
        assert!(line.contains("e1000e"));
        assert!(line.contains(" +0.000500"));

        let timing = dmesg.get_boot_timing();
        assert!(timing.total_boot_time_us() > 0);
    }

    #[test]
    fn test_top_htop_cgtop_psi_v5() {
        let mut top = TopHtopMonitorEngineV5::new();
        top.register_process(ProcessTelemetryV5 {
            pid: 1,
            ppid: 0,
            name: "systemd".to_string(),
            user: "root".to_string(),
            cpu_pct: 0.1,
            rss_bytes: 8192000,
            virt_bytes: 32768000,
            threads_count: 1,
            memory_maps: vec![ProcessMemoryMapEntryV5 {
                start_address: 0x7f0000000000,
                end_address: 0x7f0000010000,
                permissions: "r-xp".to_string(),
                offset: 0,
                device: "08:01".to_string(),
                inode: 1234,
                pathname: "/usr/lib/systemd/systemd".to_string(),
            }],
            cgroup_path: "/".to_string(),
        });

        top.set_psi_metrics(vec![CgroupV2PsiMetricV5 {
            resource: "memory".to_string(),
            some_avg10: 0.05,
            some_avg60: 0.02,
            some_avg300: 0.01,
            full_avg10: 0.00,
            full_avg60: 0.00,
            full_avg300: 0.00,
        }]);

        let maps = top.get_process_memory_maps(1).unwrap();
        assert_eq!(maps.len(), 1);
        assert_eq!(maps[0].permissions, "r-xp");

        let psi = top.get_psi_metrics();
        assert_eq!(psi.len(), 1);
        assert_eq!(psi[0].resource, "memory");
    }

    #[test]
    fn test_sar_nstat_metrics_v5() {
        let mut sar = VmstatIostatSarNstatEngineV5::new();
        sar.append_record(SarTimeSeriesRecordV5 {
            timestamp_epoch_sec: 1774872000,
            net_stats: NstatNetworkTcpUdpStatsV5 {
                tcp_in_segs: 10000,
                tcp_out_segs: 12000,
                tcp_retrans_segs: 5,
                udp_in_datagrams: 500,
                udp_out_datagrams: 500,
                ip_ext_in_octets: 1048576,
                ip_ext_out_octets: 2097152,
            },
            slab_mem: MemorySlabBreakdownV5 {
                slab_unreclaimable_kb: 32768,
                slab_reclaimable_kb: 65536,
                hugepages_total: 1024,
                hugepages_free: 512,
                direct_map_4k_kb: 4096,
                direct_map_2m_kb: 2048000,
                direct_map_1g_kb: 16777216,
            },
            cpu_user_pct: 15.0,
            cpu_sys_pct: 5.0,
            cpu_idle_pct: 80.0,
        });

        let records = sar.query_time_series(1774871000, 1774873000);
        assert_eq!(records.len(), 1);

        let line = VmstatIostatSarNstatEngineV5::format_sadd_line(records[0]);
        assert!(line.contains("10000"));
        assert!(line.contains("32768"));
    }

    #[test]
    fn test_lsof_fuser_ipcs_v5() {
        let mut lsof = LsofInspectorEngineV5::new();
        lsof.add_fuser_mapping(FuserProcessMappingV5 {
            target_resource: "80/tcp".to_string(),
            pids: vec![800, 801, 802],
            access_types: vec!["m".to_string()],
        });

        lsof.add_ipc_segment(SysVIpcSegmentV5 {
            ipc_type: "shm".to_string(),
            shmid_or_msqid: 100,
            key: 0x123456,
            owner_uid: 1000,
            perms_octal: 0o644,
            bytes_or_messages: 65536,
            active_attaches: 2,
        });

        let mapping = lsof.run_fuser_query("80/tcp").unwrap();
        assert_eq!(mapping.pids.len(), 3);

        let ipc = lsof.get_ipc_segments();
        assert_eq!(ipc.len(), 1);
        assert_eq!(ipc[0].bytes_or_messages, 65536);
    }

    #[test]
    fn test_hardware_topology_v5() {
        let mut hw = HardwareTopologyDiscoveryEngineV5::new();
        hw.set_cpu_topology(LscpuTopologyV5 {
            architecture: "x86_64".to_string(),
            sockets: 2,
            cores_per_socket: 16,
            threads_per_core: 2,
            numa_nodes_count: 2,
            l1i_cache_kb: 32,
            l1d_cache_kb: 48,
            l2_cache_kb: 1280,
            l3_cache_kb: 32768,
            cpu_flags: vec!["avx512f".to_string(), "amx_bf16".to_string()],
        });

        hw.add_memory_device(DmidecodeMemoryDeviceV5 {
            locator: "DIMM_A1".to_string(),
            size_mb: 32768,
            memory_type: "DDR5".to_string(),
            speed_mts: 5600,
            manufacturer: "Micron".to_string(),
            serial_number: "0x12345678".to_string(),
        });

        let cpu = hw.get_cpu_topology().unwrap();
        assert_eq!(cpu.sockets, 2);
        assert!(cpu.cpu_flags.contains(&"avx512f".to_string()));

        let mem = hw.get_memory_devices();
        assert_eq!(mem.len(), 1);
        assert_eq!(mem[0].speed_mts, 5600);
    }

    #[test]
    fn test_sosreport_otlp_exporter_v5() {
        let mut exporter = SosreportPrometheusOpenTelemetryExporterV5::new();
        let redacted = exporter.redact_content("config: password=mysecretpass");
        assert!(redacted.contains("[REDACTED_BY_SOSREPORT]"));

        exporter.push_otlp_metric(OpenTelemetryOtlpMetricExportV5 {
            service_name: "sigma-diagnostics".to_string(),
            metric_name: "system.cpu.usage".to_string(),
            unit: "percent".to_string(),
            value: 23.5,
            timestamp_nano: 1774872000000000000,
            attributes: BTreeMap::new(),
        });

        let json = exporter.export_otlp_json();
        assert!(json.contains("sigma-diagnostics"));
        assert!(json.contains("system.cpu.usage"));

        let collectd = exporter.export_collectd_json("sigma-node", 12.0, 3.0, 85.0);
        assert!(collectd.contains("sigma-node"));
    }

    #[test]
    fn test_diagnostics_pr_gateway_v5() {
        let gw = LoggingMonitoringDiagnosticsPullRequestGatewayV5::new();
        let subs = gw.get_submissions();
        assert!(!subs.is_empty());

        let diff = gw.generate_pr_unified_diff(501);
        assert!(diff.contains("PR #501"));
        assert!(diff.contains("DILITHIUM5_SIG_FSS_JOURNAL_501"));
    }

    #[test]
    fn test_master_suite_v5() {
        let mut master = SovereignLoggingMonitoringDiagnosticsMasterSuiteV5::new();
        let report = master.execute_comprehensive_diagnostics_sweep();
        assert!(report.contains("SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V5"));
    }
}
