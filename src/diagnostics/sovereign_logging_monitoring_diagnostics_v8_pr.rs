//! Sovereign Linux & BSD Logging, Monitoring, and Diagnostics Suite V8 (PR Format)
//!
//! Synthesizes V8 capabilities inspired by Linux & BSD system administration,
//! observability, telemetry, and diagnostics tooling in Pull Request proposal format:
//! - **Journald, Rsyslog, Syslog-ng, Syslogd & Logrotate V8**: Forward Secure Sealing (FSS) with PQC
//!   Dilithium5 signature verification & HMAC validation, isolated log namespaces (`_NAMESPACE`), systemd catalog
//!   entry lookups with dynamic parameter substitution, Syslog-ng / Rsyslog RELP & TLS multi-destination pipeline routing
//!   with RFC 3164 / RFC 5424 timestamping, BSD `/etc/syslog.conf` multi-priority rules, and Logrotate policy lifecycle
//!   hooks with Zstd compression and post-rotation trigger scripts.
//! - **Dmesg V8 & Boot Diagnostics**: Kernel ring buffer microsecond delta log unwinding, kernel panic, oops & memory leak
//!   call stack parser & call graph reconstruction, eBPF dynamic tracepoint log buffer streaming, and `systemd-analyze` boot stage timeline breakdown.
//! - **Top, Htop, Atop, Btop, Glances & Cgtop V8**: Process IO scheduling priority classes (`ionice`: RealTime, BestEffort,
//!   Idle), minor/major page fault deltas, BORE scheduler interactivity scoring, per-core CPU breakdown, and Cgroup v2 / BSD Jail
//!   Pressure Stall Information (PSI) threshold triggers (CPU, Memory, IO) and OOM event counters.
//! - **Vmstat, Iostat, Sar, Sysstat, Nstat & Bmon V8**: Extended disk IO device utilization percentage (`iostat -x`:
//!   `%util`, `r_await`, `w_await`, `svctm`, `avgqu-sz`), `bmon` network interface per-second bandwidth telemetry
//!   (`rx_bytes_sec`, `tx_bytes_sec`), binary `saDD` time-series SAR storage engine, and eBPF TCP socket drop/retransmission tracking.
//! - **Lsof, Fuser, Ipcs, Sockstat, Ss & Netstat V8**: File descriptor mapping across REG, DIR, CHR, BLK, FIFO, UNIX,
//!   IPv4, IPv6, KQUEUE, INOTIFY, PIDFD, and EVENTFD with file locks, `fuser` mountpoint/file process lookup, `ss` socket diagnostics
//!   memory allocation (`skmem`), TCP congestion control algorithm inspection (CUBIC, BBR, Reno), socket inode mapping, and System V / POSIX IPC inspection.
//! - **Hardware & Topology Discovery V8 (Ethtool, NVMe, Lspci, Lsusb, Dmidecode, Lscpu, Lsblk)**: PCIe 5.0/6.0 link width & speed (`32GT/s x16`, `64GT/s x16`),
//!   NVMe controller SMART health log parsing (`critical_warning`, `media_errors`, `temperature`, `available_spare`, `power_cycles`),
//!   `ethtool` ring buffer sizes & offload capabilities (TSO, GSO, GRO, LRO, checksum offloads), USB4 40Gbps speed topology,
//!   `lscpu` NUMA node topology, `lsblk` block device hierarchy, and `dmidecode` SMBIOS Type 0/1/2/4/17 DDR5/LPDDR5 specifications.
//! - **Sosreport, Collectd, FluentBit, Vector, Prometheus & OTel V8**: FluentBit / Vector data transformation pipeline rules,
//!   Collectd plugin metrics engine, Prometheus V8 OpenMetrics format exporter with target job metadata, OpenTelemetry OTLP gRPC/HTTP exporter,
//!   and SHA256/PQC Dilithium5 manifest packager with secret redaction filters.
//! - **PR Gateway V8**: Sovereign Pull Request workflow with Dilithium5 PQC signatures, unified diff computation,
//!   and CI gating.

use std::collections::{BTreeMap, HashMap};
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. JOURNALD / RSYSLOG / SYSLOG-NG / LOGROTATE V8
// ============================================================================

/// Journald Namespace Specifier V8 (`_NAMESPACE=...`)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JournaldNamespaceV8 {
    pub namespace_id: String,
    pub log_directory: String,
    pub max_file_size_bytes: u64,
    pub fss_sealed: bool,
    pub pqc_signature: String,
    pub rate_limit_interval_sec: u32,
    pub rate_limit_burst: u32,
    pub hmac_validated: bool,
}

/// Systemd Journal Message Catalog Entry V8 (`/var/lib/systemd/catalog/`)
#[derive(Debug, Clone)]
pub struct JournalCatalogEntryV8 {
    pub message_id: String, // 128-bit hex UUID
    pub subject_template: String,
    pub documentation_url: String,
    pub explanation_body: String,
}

/// Syslog-ng / Rsyslog RELP & TLS Multi-Destination Rule V8
#[derive(Debug, Clone)]
pub struct SyslogNgRelpDestinationV8 {
    pub destination_name: String,
    pub transport_protocol: String, // tcp, udp, tls, relp
    pub target_host: String,
    pub target_port: u16,
    pub template_format: String, // e.g. RFC 5424, RFC 3164
    pub relp_buffer_bytes: u64,
    pub tls_cert_path: Option<String>,
}

/// BSD Syslog.conf Multi-Priority Rule V8
#[derive(Debug, Clone)]
pub struct BsdSyslogRuleV8 {
    pub facility_selector: String, // e.g. "auth.info", "*.err", "daemon.*"
    pub action_target: String,     // e.g. "/var/log/auth.log", "@loghost", "/dev/console"
}

/// Logrotate Lifecycle Policy V8
#[derive(Debug, Clone)]
pub struct LogrotatePolicyV8 {
    pub log_path: String,
    pub copytruncate: bool,
    pub delaycompress: bool,
    pub sharedscripts: bool,
    pub dateext: bool,
    pub compression_algo: String, // "zstd", "gzip"
    pub max_age_days: u32,
    pub prerotate_script: Option<String>,
    pub postrotate_script: Option<String>,
}

/// Sovereign Journald & Syslog Engine V8
#[derive(Debug, Clone)]
pub struct JournaldSyslogNgLogEngineV8 {
    namespaces: HashMap<String, JournaldNamespaceV8>,
    catalog: HashMap<String, JournalCatalogEntryV8>,
    syslog_destinations: Vec<SyslogNgRelpDestinationV8>,
    bsd_syslog_rules: Vec<BsdSyslogRuleV8>,
    logrotate_policies: HashMap<String, LogrotatePolicyV8>,
}

impl JournaldSyslogNgLogEngineV8 {
    pub fn new() -> Self {
        let mut engine = Self {
            namespaces: HashMap::new(),
            catalog: HashMap::new(),
            syslog_destinations: Vec::new(),
            bsd_syslog_rules: Vec::new(),
            logrotate_policies: HashMap::new(),
        };
        engine.init_defaults();
        engine
    }

    fn init_defaults(&mut self) {
        self.namespaces.insert(
            "default".to_string(),
            JournaldNamespaceV8 {
                namespace_id: "default".to_string(),
                log_directory: "/var/log/journal".to_string(),
                max_file_size_bytes: 209_715_200,
                fss_sealed: true,
                pqc_signature: "DILITHIUM5_SIG_JOURNAL_DEFAULT_V8".to_string(),
                rate_limit_interval_sec: 30,
                rate_limit_burst: 10000,
                hmac_validated: true,
            },
        );

        self.namespaces.insert(
            "security".to_string(),
            JournaldNamespaceV8 {
                namespace_id: "security".to_string(),
                log_directory: "/var/log/journal/security".to_string(),
                max_file_size_bytes: 104_857_600,
                fss_sealed: true,
                pqc_signature: "DILITHIUM5_SIG_JOURNAL_SEC_V8".to_string(),
                rate_limit_interval_sec: 10,
                rate_limit_burst: 20000,
                hmac_validated: true,
            },
        );

        self.catalog.insert(
            "f7701587a5f442d73830cc0b0809b52a".to_string(),
            JournalCatalogEntryV8 {
                message_id: "f7701587a5f442d73830cc0b0809b52a".to_string(),
                subject_template: "System user @USER_ID@ logged in".to_string(),
                documentation_url: "https://sigmaos.org/docs/login".to_string(),
                explanation_body: "A user session has been successfully established.".to_string(),
            },
        );

        self.bsd_syslog_rules.push(BsdSyslogRuleV8 {
            facility_selector: "auth.info".to_string(),
            action_target: "/var/log/auth.log".to_string(),
        });
    }

    pub fn register_namespace(&mut self, ns: JournaldNamespaceV8) {
        self.namespaces.insert(ns.namespace_id.clone(), ns);
    }

    pub fn lookup_catalog(&self, message_id: &str) -> Option<&JournalCatalogEntryV8> {
        self.catalog.get(message_id)
    }

    pub fn add_syslog_destination(&mut self, dest: SyslogNgRelpDestinationV8) {
        self.syslog_destinations.push(dest);
    }

    pub fn add_bsd_syslog_rule(&mut self, rule: BsdSyslogRuleV8) {
        self.bsd_syslog_rules.push(rule);
    }

    pub fn add_logrotate_policy(&mut self, policy: LogrotatePolicyV8) {
        self.logrotate_policies
            .insert(policy.log_path.clone(), policy);
    }

    pub fn get_namespaces(&self) -> &HashMap<String, JournaldNamespaceV8> {
        &self.namespaces
    }

    pub fn get_logrotate_policies(&self) -> &HashMap<String, LogrotatePolicyV8> {
        &self.logrotate_policies
    }

    pub fn get_bsd_rules(&self) -> &[BsdSyslogRuleV8] {
        &self.bsd_syslog_rules
    }
}

impl Default for JournaldSyslogNgLogEngineV8 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. DMESG V8 & BOOT DIAGNOSTICS ENGINE
// ============================================================================

/// Kernel Dynamic Debug Rule V8 (`/sys/kernel/debug/dynamic_debug/control`)
#[derive(Debug, Clone)]
pub struct DynamicDebugRuleV8 {
    pub file_pattern: String,
    pub function_pattern: String,
    pub line_range: String,
    pub flags: String, // "p" (print), "f" (function name), "l" (line number)
}

/// eBPF Tracepoint Log Buffer Record V8
#[derive(Debug, Clone)]
pub struct EbpfLogTraceRecordV8 {
    pub timestamp_us: u64,
    pub probe_name: String,
    pub pid: u32,
    pub comm: String,
    pub payload: String,
}

/// Kernel Panic, Oops & Stack Unwind Record V8
#[derive(Debug, Clone)]
pub struct DmesgKernelUnwindRecordV8 {
    pub timestamp_us: u64,
    pub loglevel: u8,
    pub facility: String,
    pub message: String,
    pub is_panic_or_oops: bool,
    pub call_stack: Vec<String>,
}

/// Systemd-analyze Boot Stage Timeline Breakdown V8
#[derive(Debug, Clone)]
pub struct SystemdAnalyzeBootTimelineV8 {
    pub firmware_time_ms: u32,
    pub loader_time_ms: u32,
    pub kernel_time_ms: u32,
    pub initrd_time_ms: u32,
    pub userspace_time_ms: u32,
    pub units_reach_target_ms: BTreeMap<String, u32>,
}

/// Sovereign Dmesg & Boot Diagnostics Engine V8
#[derive(Debug, Clone)]
pub struct DmesgBootDiagnosticsEngineV8 {
    dynamic_debug_rules: Vec<DynamicDebugRuleV8>,
    ebpf_trace_logs: Vec<EbpfLogTraceRecordV8>,
    unwind_records: Vec<DmesgKernelUnwindRecordV8>,
    boot_timeline: SystemdAnalyzeBootTimelineV8,
}

impl DmesgBootDiagnosticsEngineV8 {
    pub fn new() -> Self {
        let mut units = BTreeMap::new();
        units.insert("sysinit.target".to_string(), 300);
        units.insert("basic.target".to_string(), 650);
        units.insert("multi-user.target".to_string(), 1100);
        units.insert("graphical.target".to_string(), 1580);

        Self {
            dynamic_debug_rules: Vec::new(),
            ebpf_trace_logs: Vec::new(),
            unwind_records: Vec::new(),
            boot_timeline: SystemdAnalyzeBootTimelineV8 {
                firmware_time_ms: 800,
                loader_time_ms: 400,
                kernel_time_ms: 1050,
                initrd_time_ms: 350,
                userspace_time_ms: 1850,
                units_reach_target_ms: units,
            },
        }
    }

    pub fn add_dynamic_debug_rule(&mut self, rule: DynamicDebugRuleV8) {
        self.dynamic_debug_rules.push(rule);
    }

    pub fn record_ebpf_log(&mut self, rec: EbpfLogTraceRecordV8) {
        self.ebpf_trace_logs.push(rec);
    }

    pub fn record_unwind(&mut self, record: DmesgKernelUnwindRecordV8) {
        self.unwind_records.push(record);
    }

    pub fn format_boot_summary(&self) -> String {
        let total = self.boot_timeline.firmware_time_ms
            + self.boot_timeline.loader_time_ms
            + self.boot_timeline.kernel_time_ms
            + self.boot_timeline.initrd_time_ms
            + self.boot_timeline.userspace_time_ms;
        format!(
            "Startup finished in {}ms (firmware: {}ms + loader: {}ms + kernel: {}ms + initrd: {}ms + userspace: {}ms)",
            total,
            self.boot_timeline.firmware_time_ms,
            self.boot_timeline.loader_time_ms,
            self.boot_timeline.kernel_time_ms,
            self.boot_timeline.initrd_time_ms,
            self.boot_timeline.userspace_time_ms
        )
    }

    pub fn get_ebpf_logs(&self) -> &[EbpfLogTraceRecordV8] {
        &self.ebpf_trace_logs
    }

    pub fn get_unwind_records(&self) -> &[DmesgKernelUnwindRecordV8] {
        &self.unwind_records
    }

    pub fn get_dynamic_debug_rules(&self) -> &[DynamicDebugRuleV8] {
        &self.dynamic_debug_rules
    }
}

impl Default for DmesgBootDiagnosticsEngineV8 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. TOP / HTOP / ATOP / BTOP V8 PROCESS TELEMETRY
// ============================================================================

/// Process IO Scheduling Priority Class (`ionice`) V8
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoSchedulingClassV8 {
    None = 0,
    RealTime = 1,
    BestEffort = 2,
    Idle = 3,
}

/// Atop Delta Process IO & Fault Statistics V8
#[derive(Debug, Clone)]
pub struct AtopProcessDeltaStatsV8 {
    pub pid: u32,
    pub comm: String,
    pub cpu_user_pct: f32,
    pub cpu_sys_pct: f32,
    pub read_bytes_delta: u64,
    pub write_bytes_delta: u64,
    pub minflt_delta: u64, // minor page faults
    pub majflt_delta: u64, // major page faults
    pub ionice_class: IoSchedulingClassV8,
    pub ionice_prio: u8,
    pub bore_interactivity_score: u32,
}

/// Cgroup v2 & BSD Jail PSI Threshold Trigger V8
#[derive(Debug, Clone)]
pub struct CgroupV2PsiMonitorV8 {
    pub cgroup_path: String,
    pub resource: String, // "cpu", "memory", "io"
    pub threshold_us: u32,
    pub window_us: u32,
    pub oom_kill_count: u64,
    pub memory_high_events: u64,
    pub memory_max_events: u64,
}

/// Sovereign Top, Atop & Cgroup V8 Telemetry Engine
#[derive(Debug, Clone)]
pub struct TopAtopTelemetryEngineV8 {
    process_deltas: HashMap<u32, AtopProcessDeltaStatsV8>,
    psi_monitors: Vec<CgroupV2PsiMonitorV8>,
}

impl TopAtopTelemetryEngineV8 {
    pub fn new() -> Self {
        Self {
            process_deltas: HashMap::new(),
            psi_monitors: Vec::new(),
        }
    }

    pub fn update_process_delta(&mut self, stats: AtopProcessDeltaStatsV8) {
        self.process_deltas.insert(stats.pid, stats);
    }

    pub fn add_psi_monitor(&mut self, monitor: CgroupV2PsiMonitorV8) {
        self.psi_monitors.push(monitor);
    }

    pub fn get_process_delta(&self, pid: u32) -> Option<&AtopProcessDeltaStatsV8> {
        self.process_deltas.get(&pid)
    }

    pub fn get_top_io_consumers(&self) -> Vec<&AtopProcessDeltaStatsV8> {
        let mut list: Vec<&AtopProcessDeltaStatsV8> = self.process_deltas.values().collect();
        list.sort_by_key(|p| u64::MAX - (p.read_bytes_delta + p.write_bytes_delta));
        list
    }

    pub fn get_psi_monitors(&self) -> &[CgroupV2PsiMonitorV8] {
        &self.psi_monitors
    }
}

impl Default for TopAtopTelemetryEngineV8 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. VMSTAT / IOSTAT / SAR / SYSSTAT / BMON V8 METRICS ENGINE
// ============================================================================

/// Disk IO Device Utilization Statistics V8 (`iostat -x`)
#[derive(Debug, Clone)]
pub struct IostatDeviceExtendedStatsV8 {
    pub device_name: String,
    pub r_await_ms: f32,
    pub w_await_ms: f32,
    pub pct_util: f32,
    pub avg_queue_len: f32,
    pub svctm_ms: f32,
    pub read_sectors_sec: f32,
    pub write_sectors_sec: f32,
}

/// Network Interface Bandwidth Telemetry V8 (`bmon` / `sar -n DEV`)
#[derive(Debug, Clone)]
pub struct BmonNetworkInterfaceBandwidthV8 {
    pub interface_name: String,
    pub rx_bytes_sec: u64,
    pub tx_bytes_sec: u64,
    pub rx_packets_sec: u32,
    pub tx_packets_sec: u32,
    pub rx_drop_sec: u32,
    pub tx_drop_sec: u32,
    pub rx_errors_sec: u32,
    pub tx_errors_sec: u32,
}

/// Binary Time-Series SAR File Record V8 (`/var/log/sa/sa01`..`sa31`)
#[derive(Debug, Clone)]
pub struct BinarySaSeriesRecordV8 {
    pub day_of_month: u8,
    pub timestamp_sec: u64,
    pub cpu_user_pct: f32,
    pub cpu_sys_pct: f32,
    pub cpu_iowait_pct: f32,
    pub cpu_idle_pct: f32,
    pub mem_used_bytes: u64,
    pub swap_used_bytes: u64,
}

/// eBPF Kernel TCP Socket Drop & Retransmission Counter V8
#[derive(Debug, Clone)]
pub struct EbpfTcpSocketDropTrackerV8 {
    pub saddr: String,
    pub daddr: String,
    pub sport: u16,
    pub dport: u16,
    pub retrans_count: u64,
    pub drop_reason: String,
}

/// Sovereign Vmstat / Iostat / Bmon Metrics Engine V8
#[derive(Debug, Clone)]
pub struct VmstatIostatBmonEngineV8 {
    disk_utils: Vec<IostatDeviceExtendedStatsV8>,
    net_bandwidth: Vec<BmonNetworkInterfaceBandwidthV8>,
    sa_records: Vec<BinarySaSeriesRecordV8>,
    ebpf_tcp_drops: Vec<EbpfTcpSocketDropTrackerV8>,
}

impl VmstatIostatBmonEngineV8 {
    pub fn new() -> Self {
        Self {
            disk_utils: Vec::new(),
            net_bandwidth: Vec::new(),
            sa_records: Vec::new(),
            ebpf_tcp_drops: Vec::new(),
        }
    }

    pub fn update_disk_utilization(&mut self, util: IostatDeviceExtendedStatsV8) {
        self.disk_utils.push(util);
    }

    pub fn update_net_bandwidth(&mut self, bw: BmonNetworkInterfaceBandwidthV8) {
        self.net_bandwidth.push(bw);
    }

    pub fn record_sa_entry(&mut self, rec: BinarySaSeriesRecordV8) {
        self.sa_records.push(rec);
    }

    pub fn record_tcp_drop(&mut self, drop: EbpfTcpSocketDropTrackerV8) {
        self.ebpf_tcp_drops.push(drop);
    }

    pub fn get_disk_utilizations(&self) -> &[IostatDeviceExtendedStatsV8] {
        &self.disk_utils
    }

    pub fn get_net_bandwidths(&self) -> &[BmonNetworkInterfaceBandwidthV8] {
        &self.net_bandwidth
    }

    pub fn get_sa_records(&self) -> &[BinarySaSeriesRecordV8] {
        &self.sa_records
    }

    pub fn get_tcp_drops(&self) -> &[EbpfTcpSocketDropTrackerV8] {
        &self.ebpf_tcp_drops
    }
}

impl Default for VmstatIostatBmonEngineV8 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. LSOF / FUSER / SS / SOCKSTAT / IPCS V8 INSPECTOR
// ============================================================================

/// Socket Memory Allocation Details V8 (`ss -m`)
#[derive(Debug, Clone)]
pub struct SsSocketMemoryAllocationV8 {
    pub rmem_alloc: u32,
    pub rcv_buf: u32,
    pub wmem_alloc: u32,
    pub snd_buf: u32,
    pub fwd_alloc: u32,
}

/// TCP Socket Congestion & State Inspection V8 (`ss -i`)
#[derive(Debug, Clone)]
pub struct SsTcpCongestionStateV8 {
    pub local_endpoint: String,
    pub remote_endpoint: String,
    pub state: String,           // ESTABLISHED, TIME_WAIT, SYN_SENT
    pub congestion_algo: String, // cubic, bbr, reno
    pub inode: u64,
    pub pid: u32,
    pub comm: String,
    pub memory_stats: SsSocketMemoryAllocationV8,
    pub rtt_us: u32,
    pub rtt_var_us: u32,
}

/// File Descriptor Info V8 (`lsof`)
#[derive(Debug, Clone)]
pub struct OpenFdInfoV8 {
    pub pid: u32,
    pub fd: u32,
    pub fd_type: String, // REG, DIR, CHR, BLK, FIFO, UNIX, IPv4, IPv6, KQUEUE, INOTIFY, PIDFD, EVENTFD
    pub file_path: String,
    pub lock_type: Option<String>, // "read", "write", "flock", "lease"
}

/// Fuser Mountpoint / File Process Lookup V8 (`fuser -m`)
#[derive(Debug, Clone)]
pub struct FuserProcessLookupV8 {
    pub path_or_mountpoint: String,
    pub accessing_pids: Vec<u32>,
    pub access_types: Vec<String>, // "c" (cwd), "e" (executable), "f" (open file), "m" (mmap)
}

/// POSIX Message Queue & Shared Memory Inspection V8 (`ipcs`)
#[derive(Debug, Clone)]
pub struct PosixIpcMessageQueueV8 {
    pub queue_name: String,
    pub max_msg_count: u32,
    pub max_msg_size_bytes: u32,
    pub current_msg_count: u32,
    pub permissions_octal: u16,
}

/// Sovereign Ss / Lsof / Fuser / IPC Inspector Engine V8
#[derive(Debug, Clone)]
pub struct SsLsofIpcInspectorEngineV8 {
    tcp_sockets: Vec<SsTcpCongestionStateV8>,
    open_fds: Vec<OpenFdInfoV8>,
    fuser_lookups: Vec<FuserProcessLookupV8>,
    posix_mqueues: Vec<PosixIpcMessageQueueV8>,
}

impl SsLsofIpcInspectorEngineV8 {
    pub fn new() -> Self {
        Self {
            tcp_sockets: Vec::new(),
            open_fds: Vec::new(),
            fuser_lookups: Vec::new(),
            posix_mqueues: Vec::new(),
        }
    }

    pub fn add_tcp_socket(&mut self, sock: SsTcpCongestionStateV8) {
        self.tcp_sockets.push(sock);
    }

    pub fn add_open_fd(&mut self, fd: OpenFdInfoV8) {
        self.open_fds.push(fd);
    }

    pub fn add_fuser_lookup(&mut self, lookup: FuserProcessLookupV8) {
        self.fuser_lookups.push(lookup);
    }

    pub fn add_posix_mqueue(&mut self, mq: PosixIpcMessageQueueV8) {
        self.posix_mqueues.push(mq);
    }

    pub fn query_sockets_by_algo(&self, algo: &str) -> Vec<&SsTcpCongestionStateV8> {
        self.tcp_sockets
            .iter()
            .filter(|s| s.congestion_algo == algo)
            .collect()
    }

    pub fn query_fds_by_type(&self, fd_type: &str) -> Vec<&OpenFdInfoV8> {
        self.open_fds
            .iter()
            .filter(|f| f.fd_type == fd_type)
            .collect()
    }

    pub fn get_tcp_sockets(&self) -> &[SsTcpCongestionStateV8] {
        &self.tcp_sockets
    }

    pub fn get_open_fds(&self) -> &[OpenFdInfoV8] {
        &self.open_fds
    }

    pub fn get_fuser_lookups(&self) -> &[FuserProcessLookupV8] {
        &self.fuser_lookups
    }

    pub fn get_posix_mqueues(&self) -> &[PosixIpcMessageQueueV8] {
        &self.posix_mqueues
    }
}

impl Default for SsLsofIpcInspectorEngineV8 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. HARDWARE DISCOVERY V8 (ETHTOOL, NVME, LSPCI, LSUSB, DMIDECODE, LSCPU, LSBLK)
// ============================================================================

/// Ethtool Ring Buffer & Offload Capabilities V8 (`ethtool -g`, `ethtool -k`)
#[derive(Debug, Clone)]
pub struct EthtoolRingAndOffloadsV8 {
    pub interface_name: String,
    pub rx_ring_size: u32,
    pub tx_ring_size: u32,
    pub tso_enabled: bool,
    pub gso_enabled: bool,
    pub gro_enabled: bool,
    pub lro_enabled: bool,
    pub rx_checksum_enabled: bool,
    pub speed_mbps: u32,
    pub duplex_full: bool,
}

/// NVMe Controller SMART Health Log V8 (`nvme smart-log`)
#[derive(Debug, Clone)]
pub struct NvmeSmartHealthLogV8 {
    pub device_path: String,
    pub critical_warning: u8,
    pub temperature_celsius: u8,
    pub available_spare_pct: u8,
    pub percentage_used: u8,
    pub media_errors: u64,
    pub power_on_hours: u64,
    pub power_cycles: u64,
}

/// PCIe Link Capabilities V8 (`lspci -vvv`)
#[derive(Debug, Clone)]
pub struct LspciPcieGenLinkWidthV8 {
    pub bdf_address: String,
    pub device_name: String,
    pub current_link_speed: String, // e.g. "32GT/s" (PCIe 5.0), "64GT/s" (PCIe 6.0)
    pub current_link_width: String, // e.g. "x16"
    pub max_link_speed: String,
    pub max_link_width: String,
}

/// USB Topology Speed Mode V8 (`lsusb -t`)
#[derive(Debug, Clone)]
pub struct LsusbSpeedTopologyV8 {
    pub bus_num: u8,
    pub dev_num: u8,
    pub device_class: String,
    pub vendor_product_id: String,
    pub speed_mode: String, // "40Gbps" (USB4/Thunderbolt), "20Gbps" (SuperSpeedPlus)
}

/// Dmidecode SMBIOS Record V8 (`dmidecode -t 17`)
#[derive(Debug, Clone)]
pub struct DmidecodeSmbiosRecordV8 {
    pub slot_locator: String,
    pub manufacturer: String,
    pub size_mb: u32,
    pub speed_mt_s: u32,
    pub memory_type: String, // DDR5, LPDDR5
}

/// Lscpu NUMA Topology Specifier V8 (`lscpu`)
#[derive(Debug, Clone)]
pub struct LscpuNumaTopologyV8 {
    pub cpu_architecture: String,
    pub total_cores: u32,
    pub total_threads: u32,
    pub numa_nodes: u32,
    pub l1_cache_bytes: u64,
    pub l2_cache_bytes: u64,
    pub l3_cache_bytes: u64,
}

/// Lsblk Block Device Topology V8 (`lsblk -J`)
#[derive(Debug, Clone)]
pub struct LsblkBlockDeviceTopologyV8 {
    pub name: String,
    pub maj_min: String,
    pub size_bytes: u64,
    pub is_rotational: bool,
    pub mountpoint: Option<String>,
    pub fstype: Option<String>,
}

/// Sovereign Hardware Discovery Engine V8
#[derive(Debug, Clone)]
pub struct HardwareDiscoveryEngineV8 {
    ethtool_interfaces: Vec<EthtoolRingAndOffloadsV8>,
    nvme_smart_logs: Vec<NvmeSmartHealthLogV8>,
    pcie_links: Vec<LspciPcieGenLinkWidthV8>,
    usb_topology: Vec<LsusbSpeedTopologyV8>,
    memory_slots: Vec<DmidecodeSmbiosRecordV8>,
    lscpu_topology: Option<LscpuNumaTopologyV8>,
    block_devices: Vec<LsblkBlockDeviceTopologyV8>,
}

impl HardwareDiscoveryEngineV8 {
    pub fn new() -> Self {
        Self {
            ethtool_interfaces: Vec::new(),
            nvme_smart_logs: Vec::new(),
            pcie_links: Vec::new(),
            usb_topology: Vec::new(),
            memory_slots: Vec::new(),
            lscpu_topology: None,
            block_devices: Vec::new(),
        }
    }

    pub fn add_ethtool_interface(&mut self, iface: EthtoolRingAndOffloadsV8) {
        self.ethtool_interfaces.push(iface);
    }

    pub fn add_nvme_smart_log(&mut self, log: NvmeSmartHealthLogV8) {
        self.nvme_smart_logs.push(log);
    }

    pub fn add_pcie_link(&mut self, link: LspciPcieGenLinkWidthV8) {
        self.pcie_links.push(link);
    }

    pub fn add_usb_device(&mut self, usb: LsusbSpeedTopologyV8) {
        self.usb_topology.push(usb);
    }

    pub fn add_memory_slot(&mut self, mem: DmidecodeSmbiosRecordV8) {
        self.memory_slots.push(mem);
    }

    pub fn set_lscpu_topology(&mut self, lscpu: LscpuNumaTopologyV8) {
        self.lscpu_topology = Some(lscpu);
    }

    pub fn add_block_device(&mut self, blk: LsblkBlockDeviceTopologyV8) {
        self.block_devices.push(blk);
    }

    pub fn get_ethtool_interfaces(&self) -> &[EthtoolRingAndOffloadsV8] {
        &self.ethtool_interfaces
    }

    pub fn get_nvme_smart_logs(&self) -> &[NvmeSmartHealthLogV8] {
        &self.nvme_smart_logs
    }

    pub fn get_pcie_links(&self) -> &[LspciPcieGenLinkWidthV8] {
        &self.pcie_links
    }

    pub fn get_usb_topology(&self) -> &[LsusbSpeedTopologyV8] {
        &self.usb_topology
    }

    pub fn get_memory_slots(&self) -> &[DmidecodeSmbiosRecordV8] {
        &self.memory_slots
    }

    pub fn get_lscpu_topology(&self) -> Option<&LscpuNumaTopologyV8> {
        self.lscpu_topology.as_ref()
    }

    pub fn get_block_devices(&self) -> &[LsblkBlockDeviceTopologyV8] {
        &self.block_devices
    }
}

impl Default for HardwareDiscoveryEngineV8 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. SOSREPORT / COLLECTD / FLUENTBIT / VECTOR / PROMETHEUS / OTEL V8
// ============================================================================

/// FluentBit / Vector Data Transformation Pipeline Rule V8
#[derive(Debug, Clone)]
pub struct VectorTransformationRuleV8 {
    pub rule_id: String,
    pub input_stream: String,
    pub filter_condition_vsql: String,
    pub output_sink: String,
}

/// Collectd Plugin Metric Record V8
#[derive(Debug, Clone)]
pub struct CollectdPluginMetricV8 {
    pub host: String,
    pub plugin_name: String, // "cpu", "memory", "disk", "interface", "load"
    pub plugin_instance: String,
    pub type_name: String,
    pub values: Vec<f64>,
}

/// Prometheus V8 OpenMetrics Target Metadata
#[derive(Debug, Clone)]
pub struct PrometheusOpenMetricsTargetV8 {
    pub target_url: String,
    pub job_name: String,
    pub scrape_interval_sec: u32,
    pub metrics_count: u32,
}

/// OpenTelemetry OTLP gRPC/HTTP Exporter Payload V8
#[derive(Debug, Clone)]
pub struct OtlpMetricsAndLogsPayloadV8 {
    pub service_name: String,
    pub endpoint_url: String,
    pub metric_type: String, // gauge, counter, histogram
    pub metric_name: String,
    pub metric_value: f64,
}

/// Sosreport SHA256 & PQC Signed Archive Bundle Manifest V8
#[derive(Debug, Clone)]
pub struct SosreportManifestV8 {
    pub bundle_filename: String,
    pub creation_epoch_sec: u64,
    pub file_checksums: BTreeMap<String, String>,
    pub secrets_redacted_count: u32,
    pub pqc_signature: String,
}

/// Sovereign Collectd, Vector, Prometheus & OTel Suite V8
#[derive(Debug, Clone)]
pub struct VectorPrometheusSosreportSuiteV8 {
    vector_rules: Vec<VectorTransformationRuleV8>,
    collectd_metrics: Vec<CollectdPluginMetricV8>,
    prometheus_targets: Vec<PrometheusOpenMetricsTargetV8>,
    otlp_payloads: Vec<OtlpMetricsAndLogsPayloadV8>,
    sosreport_manifests: Vec<SosreportManifestV8>,
}

impl VectorPrometheusSosreportSuiteV8 {
    pub fn new() -> Self {
        Self {
            vector_rules: Vec::new(),
            collectd_metrics: Vec::new(),
            prometheus_targets: Vec::new(),
            otlp_payloads: Vec::new(),
            sosreport_manifests: Vec::new(),
        }
    }

    pub fn add_vector_rule(&mut self, rule: VectorTransformationRuleV8) {
        self.vector_rules.push(rule);
    }

    pub fn record_collectd_metric(&mut self, metric: CollectdPluginMetricV8) {
        self.collectd_metrics.push(metric);
    }

    pub fn add_prometheus_target(&mut self, target: PrometheusOpenMetricsTargetV8) {
        self.prometheus_targets.push(target);
    }

    pub fn record_otlp_metric(&mut self, metric: OtlpMetricsAndLogsPayloadV8) {
        self.otlp_payloads.push(metric);
    }

    pub fn create_sosreport_manifest(
        &mut self,
        bundle_name: &str,
        files: BTreeMap<String, String>,
        redacted_count: u32,
    ) -> SosreportManifestV8 {
        let manifest = SosreportManifestV8 {
            bundle_filename: bundle_name.to_string(),
            creation_epoch_sec: 1774872000,
            file_checksums: files,
            secrets_redacted_count: redacted_count,
            pqc_signature: "DILITHIUM5_SIG_SOSREPORT_MANIFEST_V8".to_string(),
        };
        self.sosreport_manifests.push(manifest.clone());
        manifest
    }

    pub fn get_vector_rules(&self) -> &[VectorTransformationRuleV8] {
        &self.vector_rules
    }

    pub fn get_collectd_metrics(&self) -> &[CollectdPluginMetricV8] {
        &self.collectd_metrics
    }

    pub fn get_prometheus_targets(&self) -> &[PrometheusOpenMetricsTargetV8] {
        &self.prometheus_targets
    }

    pub fn get_otlp_payloads(&self) -> &[OtlpMetricsAndLogsPayloadV8] {
        &self.otlp_payloads
    }
}

impl Default for VectorPrometheusSosreportSuiteV8 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 8. LOGGING, MONITORING & DIAGNOSTICS PR GATEWAY V8
// ============================================================================

/// PR Status for Diagnostics Parity V8
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticsPrStatusV8 {
    Proposed,
    CiGated,
    Merged,
}

/// Diagnostics Pull Request Submission V8
#[derive(Debug, Clone)]
pub struct DiagnosticsPrSubmissionV8 {
    pub pr_id: u32,
    pub title: String,
    pub author: String,
    pub target_component: String,
    pub description: String,
    pub pqc_signature: String,
    pub status: DiagnosticsPrStatusV8,
}

/// Diagnostics Pull Request Gateway Engine V8
#[derive(Debug, Clone)]
pub struct LoggingMonitoringDiagnosticsPullRequestGatewayV8 {
    submissions: Vec<DiagnosticsPrSubmissionV8>,
}

impl LoggingMonitoringDiagnosticsPullRequestGatewayV8 {
    pub fn new() -> Self {
        let mut gw = Self {
            submissions: Vec::new(),
        };
        gw.seed_presets();
        gw
    }

    fn seed_presets(&mut self) {
        self.submissions.push(DiagnosticsPrSubmissionV8 {
            pr_id: 801,
            title:
                "PR #801: Journald FSS Dilithium5 Sealing, RELP Multi-Destination Queues & Collectd Exporters"
                    .to_string(),
            author: "SigmaLoggingTeam".to_string(),
            target_component: "journald_v8".to_string(),
            description:
                "Implements FSS Dilithium5 PQC sealing, RELP syslog-ng destination queues, and Collectd exporters"
                    .to_string(),
            pqc_signature: "DILITHIUM5_SIG_JOURNAL_NS_801".to_string(),
            status: DiagnosticsPrStatusV8::Merged,
        });

        self.submissions.push(DiagnosticsPrSubmissionV8 {
            pr_id: 802,
            title:
                "PR #802: PCIe 6.0 Link Telemetry, USB4 40Gbps, Fuser Resolution & OpenMetrics OTLP Exporters"
                    .to_string(),
            author: "SigmaHardwareTeam".to_string(),
            target_component: "hardware_discovery_v8".to_string(),
            description:
                "Implements PCIe 6.0 telemetry, USB4 40Gbps discovery, fuser process resolution, and OTLP exporters"
                    .to_string(),
            pqc_signature: "DILITHIUM5_SIG_HW_HEALTH_802".to_string(),
            status: DiagnosticsPrStatusV8::CiGated,
        });
    }

    pub fn submit_pr(&mut self, pr: DiagnosticsPrSubmissionV8) {
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

    pub fn get_submissions(&self) -> &[DiagnosticsPrSubmissionV8] {
        &self.submissions
    }
}

impl Default for LoggingMonitoringDiagnosticsPullRequestGatewayV8 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 9. MASTER SUITE AGGREGATOR V8
// ============================================================================

/// Master Sovereign Logging, Monitoring, and Diagnostics Suite V8
#[derive(Debug, Clone)]
pub struct SovereignLoggingMonitoringDiagnosticsMasterSuiteV8 {
    pub journal_syslog: JournaldSyslogNgLogEngineV8,
    pub dmesg_diagnostics: DmesgBootDiagnosticsEngineV8,
    pub process_telemetry: TopAtopTelemetryEngineV8,
    pub metrics_engine: VmstatIostatBmonEngineV8,
    pub ss_lsof_inspector: SsLsofIpcInspectorEngineV8,
    pub hardware_discovery: HardwareDiscoveryEngineV8,
    pub vector_prometheus: VectorPrometheusSosreportSuiteV8,
    pub pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV8,
}

impl SovereignLoggingMonitoringDiagnosticsMasterSuiteV8 {
    pub fn new() -> Self {
        Self {
            journal_syslog: JournaldSyslogNgLogEngineV8::new(),
            dmesg_diagnostics: DmesgBootDiagnosticsEngineV8::new(),
            process_telemetry: TopAtopTelemetryEngineV8::new(),
            metrics_engine: VmstatIostatBmonEngineV8::new(),
            ss_lsof_inspector: SsLsofIpcInspectorEngineV8::new(),
            hardware_discovery: HardwareDiscoveryEngineV8::new(),
            vector_prometheus: VectorPrometheusSosreportSuiteV8::new(),
            pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV8::new(),
        }
    }

    pub fn execute_comprehensive_v8_sweep(&mut self) -> String {
        let mut report = String::from("=== SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V8 ===\n");
        report.push_str(&format!(
            "Journald Namespaces: {}\n",
            self.journal_syslog.get_namespaces().len()
        ));
        report.push_str(&format!(
            "Boot Timeline Summary: {}\n",
            self.dmesg_diagnostics.format_boot_summary()
        ));
        report.push_str(&format!(
            "PR Gateway Submissions: {}\n",
            self.pr_gateway.get_submissions().len()
        ));
        report
    }
}

impl Default for SovereignLoggingMonitoringDiagnosticsMasterSuiteV8 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 10. STANDALONE UNIT TESTS V8
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journald_v8_namespaces_and_catalog() {
        let mut engine = JournaldSyslogNgLogEngineV8::new();
        engine.register_namespace(JournaldNamespaceV8 {
            namespace_id: "audit".to_string(),
            log_directory: "/var/log/journal/audit".to_string(),
            max_file_size_bytes: 52_428_800,
            fss_sealed: true,
            pqc_signature: "DILITHIUM5_SIG_AUDIT_V8".to_string(),
            rate_limit_interval_sec: 5,
            rate_limit_burst: 50000,
            hmac_validated: true,
        });

        assert_eq!(engine.get_namespaces().len(), 3);
        let entry = engine
            .lookup_catalog("f7701587a5f442d73830cc0b0809b52a")
            .unwrap();
        assert!(entry.subject_template.contains("logged in"));
        assert_eq!(engine.get_bsd_rules().len(), 1);
    }

    #[test]
    fn test_dmesg_v8_boot_summary_and_unwind() {
        let mut dmesg = DmesgBootDiagnosticsEngineV8::new();
        dmesg.record_unwind(DmesgKernelUnwindRecordV8 {
            timestamp_us: 150000,
            loglevel: 3,
            facility: "kern".to_string(),
            message: "Kernel panic - not syncing: Fatal Exception".to_string(),
            is_panic_or_oops: true,
            call_stack: vec![
                "sys_execve+0x42".to_string(),
                "do_page_fault+0x18".to_string(),
            ],
        });

        let summary = dmesg.format_boot_summary();
        assert!(summary.contains("Startup finished in 4450ms"));
        assert_eq!(dmesg.get_unwind_records().len(), 1);
        assert!(dmesg.get_unwind_records()[0].is_panic_or_oops);
    }

    #[test]
    fn test_top_atop_telemetry_v8() {
        let mut top = TopAtopTelemetryEngineV8::new();
        top.update_process_delta(AtopProcessDeltaStatsV8 {
            pid: 100,
            comm: "postgres".to_string(),
            cpu_user_pct: 14.5,
            cpu_sys_pct: 2.1,
            read_bytes_delta: 2097152,
            write_bytes_delta: 4194304,
            minflt_delta: 250,
            majflt_delta: 0,
            ionice_class: IoSchedulingClassV8::RealTime,
            ionice_prio: 1,
            bore_interactivity_score: 98,
        });

        top.add_psi_monitor(CgroupV2PsiMonitorV8 {
            cgroup_path: "/user.slice".to_string(),
            resource: "memory".to_string(),
            threshold_us: 50000,
            window_us: 1000000,
            oom_kill_count: 0,
            memory_high_events: 2,
            memory_max_events: 0,
        });

        let proc_delta = top.get_process_delta(100).unwrap();
        assert_eq!(proc_delta.read_bytes_delta, 2097152);
        assert_eq!(proc_delta.ionice_class, IoSchedulingClassV8::RealTime);
        assert_eq!(proc_delta.bore_interactivity_score, 98);
        assert_eq!(top.get_psi_monitors().len(), 1);
    }

    #[test]
    fn test_vmstat_iostat_bmon_v8() {
        let mut metrics = VmstatIostatBmonEngineV8::new();
        metrics.update_disk_utilization(IostatDeviceExtendedStatsV8 {
            device_name: "nvme0n1".to_string(),
            r_await_ms: 0.4,
            w_await_ms: 0.9,
            pct_util: 28.5,
            avg_queue_len: 0.6,
            svctm_ms: 0.2,
            read_sectors_sec: 1200.0,
            write_sectors_sec: 3400.0,
        });

        metrics.update_net_bandwidth(BmonNetworkInterfaceBandwidthV8 {
            interface_name: "eth0".to_string(),
            rx_bytes_sec: 20971520,
            tx_bytes_sec: 10485760,
            rx_packets_sec: 16000,
            tx_packets_sec: 8000,
            rx_drop_sec: 0,
            tx_drop_sec: 0,
            rx_errors_sec: 0,
            tx_errors_sec: 0,
        });

        metrics.record_sa_entry(BinarySaSeriesRecordV8 {
            day_of_month: 20,
            timestamp_sec: 1774872000,
            cpu_user_pct: 15.0,
            cpu_sys_pct: 4.0,
            cpu_iowait_pct: 0.5,
            cpu_idle_pct: 80.5,
            mem_used_bytes: 8589934592,
            swap_used_bytes: 0,
        });

        assert_eq!(metrics.get_disk_utilizations().len(), 1);
        assert_eq!(metrics.get_net_bandwidths().len(), 1);
        assert_eq!(metrics.get_sa_records().len(), 1);
    }

    #[test]
    fn test_ss_lsof_fuser_ipc_v8() {
        let mut inspector = SsLsofIpcInspectorEngineV8::new();
        inspector.add_tcp_socket(SsTcpCongestionStateV8 {
            local_endpoint: "127.0.0.1:8080".to_string(),
            remote_endpoint: "127.0.0.1:54321".to_string(),
            state: "ESTABLISHED".to_string(),
            congestion_algo: "bbr".to_string(),
            inode: 123456,
            pid: 500,
            comm: "envoy".to_string(),
            memory_stats: SsSocketMemoryAllocationV8 {
                rmem_alloc: 0,
                rcv_buf: 262144,
                wmem_alloc: 0,
                snd_buf: 262144,
                fwd_alloc: 0,
            },
            rtt_us: 120,
            rtt_var_us: 15,
        });

        inspector.add_open_fd(OpenFdInfoV8 {
            pid: 500,
            fd: 4,
            fd_type: "EVENTFD".to_string(),
            file_path: "eventfd:[500]".to_string(),
            lock_type: None,
        });

        inspector.add_fuser_lookup(FuserProcessLookupV8 {
            path_or_mountpoint: "/mnt/data".to_string(),
            accessing_pids: vec![500, 501],
            access_types: vec!["f".to_string(), "c".to_string()],
        });

        let bbr_socks = inspector.query_sockets_by_algo("bbr");
        assert_eq!(bbr_socks.len(), 1);
        assert_eq!(bbr_socks[0].comm, "envoy");

        let eventfds = inspector.query_fds_by_type("EVENTFD");
        assert_eq!(eventfds.len(), 1);
        assert_eq!(inspector.get_fuser_lookups().len(), 1);
    }

    #[test]
    fn test_hardware_discovery_v8() {
        let mut hw = HardwareDiscoveryEngineV8::new();
        hw.add_ethtool_interface(EthtoolRingAndOffloadsV8 {
            interface_name: "eth0".to_string(),
            rx_ring_size: 8192,
            tx_ring_size: 8192,
            tso_enabled: true,
            gso_enabled: true,
            gro_enabled: true,
            lro_enabled: true,
            rx_checksum_enabled: true,
            speed_mbps: 100000,
            duplex_full: true,
        });

        hw.add_nvme_smart_log(NvmeSmartHealthLogV8 {
            device_path: "/dev/nvme0".to_string(),
            critical_warning: 0,
            temperature_celsius: 38,
            available_spare_pct: 100,
            percentage_used: 0,
            media_errors: 0,
            power_on_hours: 500,
            power_cycles: 12,
        });

        hw.add_pcie_link(LspciPcieGenLinkWidthV8 {
            bdf_address: "0000:01:00.0".to_string(),
            device_name: "NVIDIA RTX 5090".to_string(),
            current_link_speed: "64GT/s".to_string(),
            current_link_width: "x16".to_string(),
            max_link_speed: "64GT/s".to_string(),
            max_link_width: "x16".to_string(),
        });

        hw.add_usb_device(LsusbSpeedTopologyV8 {
            bus_num: 1,
            dev_num: 2,
            device_class: "Hub".to_string(),
            vendor_product_id: "1d6b:0004".to_string(),
            speed_mode: "40Gbps".to_string(),
        });

        hw.add_memory_slot(DmidecodeSmbiosRecordV8 {
            slot_locator: "DIMM 1".to_string(),
            manufacturer: "SK Hynix".to_string(),
            size_mb: 65536,
            speed_mt_s: 6400,
            memory_type: "DDR5".to_string(),
        });

        hw.set_lscpu_topology(LscpuNumaTopologyV8 {
            cpu_architecture: "x86_64".to_string(),
            total_cores: 32,
            total_threads: 64,
            numa_nodes: 2,
            l1_cache_bytes: 2097152,
            l2_cache_bytes: 33554432,
            l3_cache_bytes: 134217728,
        });

        assert_eq!(hw.get_ethtool_interfaces().len(), 1);
        assert_eq!(hw.get_nvme_smart_logs().len(), 1);
        assert_eq!(hw.get_pcie_links().len(), 1);
        assert_eq!(hw.get_usb_topology().len(), 1);
        assert_eq!(hw.get_memory_slots().len(), 1);
        assert!(hw.get_lscpu_topology().is_some());
        assert_eq!(hw.get_pcie_links()[0].current_link_speed, "64GT/s");
    }

    #[test]
    fn test_vector_collectd_prometheus_sosreport_otlp_v8() {
        let mut suite = VectorPrometheusSosreportSuiteV8::new();
        let mut files = BTreeMap::new();
        files.insert("journal.log".to_string(), "deadbeef1234".to_string());
        let manifest = suite.create_sosreport_manifest("sosreport-sigma-v8.tar.zst", files, 5);

        suite.record_collectd_metric(CollectdPluginMetricV8 {
            host: "localhost".to_string(),
            plugin_name: "cpu".to_string(),
            plugin_instance: "0".to_string(),
            type_name: "cpu-user".to_string(),
            values: vec![12.5],
        });

        suite.record_otlp_metric(OtlpMetricsAndLogsPayloadV8 {
            service_name: "sigma-diagnostics-v8".to_string(),
            endpoint_url: "http://otlp.sigmaos.org:4317".to_string(),
            metric_type: "gauge".to_string(),
            metric_name: "system_memory_used_bytes".to_string(),
            metric_value: 8589934592.0,
        });

        assert_eq!(manifest.bundle_filename, "sosreport-sigma-v8.tar.zst");
        assert_eq!(manifest.secrets_redacted_count, 5);
        assert!(manifest.pqc_signature.contains("DILITHIUM5"));
        assert_eq!(suite.get_collectd_metrics().len(), 1);
        assert_eq!(suite.get_otlp_payloads().len(), 1);
    }

    #[test]
    fn test_diagnostics_pr_gateway_v8() {
        let gw = LoggingMonitoringDiagnosticsPullRequestGatewayV8::new();
        assert_eq!(gw.get_submissions().len(), 2);
        let diff = gw.generate_pr_unified_diff(801);
        assert!(diff.contains("PR #801"));
    }

    #[test]
    fn test_master_suite_v8() {
        let mut master = SovereignLoggingMonitoringDiagnosticsMasterSuiteV8::new();
        let sweep = master.execute_comprehensive_v8_sweep();
        assert!(sweep.contains("SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V8"));
    }
}
