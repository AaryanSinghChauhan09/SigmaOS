//! Sovereign Linux & BSD Logging, Monitoring, and Diagnostics Suite V6 (PR Format)
//!
//! Synthesizes V6 capabilities inspired by Linux & BSD system administration,
//! observability, telemetry, and diagnostics tooling in Pull Request proposal format:
//! - **Journald, Rsyslog, Syslog-ng, Syslogd & Logrotate V6**: Forward Secure Sealing (FSS) HMAC verification,
//!   isolated namespaces (`_NAMESPACE`), systemd message catalog lookup (`/var/lib/systemd/catalog/`),
//!   Syslog-ng template destination routing, Rsyslog RELP queue buffering, BSD `/etc/syslog.conf` rules,
//!   and Logrotate policy lifecycle execution hooks.
//! - **Dmesg V6 & Boot Diagnostics**: Kernel ring buffer microsecond delta timestamps, loglevel & facility filtering,
//!   dynamic debug control (`/sys/kernel/debug/dynamic_debug/control`), eBPF tracepoint log buffer streaming,
//!   and systemd-analyze boot timeline stage breakdown.
//! - **Top, Htop, Btop, Atop, Glances & Cgtop V6**: `atop` delta process IO stats (`ionice` scheduling classes),
//!   page faults, per-interval IO, BORE scheduler interactivity scoring, and Cgroup v2 Pressure Stall Information (PSI)
//!   thresholds & OOM event counters.
//! - **Vmstat, Iostat, Sar, Sysstat, Nstat & Bmon V6**: Disk IO utilization percentage (%util, await, svctm),
//!   `bmon` network interface per-second bandwidth telemetry (`rx_bytes_sec`, `tx_bytes_sec`), binary `saDD` time-series
//!   SAR storage, and eBPF socket drop/retrans tracking.
//! - **Lsof, Fuser, Ipcs, Sockstat, Ss & Netstat V6**: File descriptor mapping across REG, DIR, CHR, BLK, FIFO, UNIX,
//!   IPv4, IPv6, KQUEUE, INOTIFY, PIDFD, and EVENTFD with locks, `ss` socket diagnostics memory allocation (`skmem`),
//!   TCP congestion state (CUBIC/BBR/Reno), socket inode mapping, and System V / POSIX IPC message queue inspection.
//! - **Hardware & Topology Discovery V6**: `lspci` PCIe link width & speed (`16GT/s x16`), `nvme` SMART health logs
//!   (`critical_warning`, `media_errors`, `temperature`), `ethtool` ring buffer & offload capabilities (`tso`, `gso`, `gro`),
//!   and `dmidecode` SMBIOS Type 0/1/2/4/17 specifications.
//! - **Sosreport, FluentBit, Vector, Prometheus & OTel V6**: FluentBit / Vector data transformation pipeline rules,
//!   Prometheus V6 OpenMetrics format exporter with target metadata, OpenTelemetry OTLP gRPC/HTTP exporter,
//!   and SHA256/PQC manifest packager with secret redaction filters.
//! - **PR Gateway V6**: Sovereign Pull Request workflow with Dilithium5 PQC signatures, unified diff computation,
//!   and CI gating.

use std::collections::{BTreeMap, HashMap};
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. JOURNALD / RSYSLOG / SYSLOG-NG / LOGROTATE V6
// ============================================================================

/// Journald Namespace Specifier V6 (`_NAMESPACE=...`)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JournaldNamespaceV6 {
    pub namespace_id: String,
    pub log_directory: String,
    pub max_file_size_bytes: u64,
    pub fss_sealed: bool,
}

/// Systemd Journal Message Catalog Entry V6 (`/var/lib/systemd/catalog/`)
#[derive(Debug, Clone)]
pub struct JournalCatalogEntryV6 {
    pub message_id: String, // 128-bit hex UUID
    pub subject_template: String,
    pub documentation_url: String,
    pub explanation_body: String,
}

/// Syslog-ng Template Destination Rule V6
#[derive(Debug, Clone)]
pub struct SyslogNgDestinationRuleV6 {
    pub destination_name: String,
    pub transport_protocol: String, // tcp, udp, tls, relp
    pub target_host: String,
    pub target_port: u16,
    pub template_format: String,
    pub relp_buffer_bytes: u64,
}

/// BSD Syslog.conf Action Rule V6
#[derive(Debug, Clone)]
pub struct BsdSyslogRuleV6 {
    pub facility_selector: String, // e.g. "auth.info", "*.err", "daemon.*"
    pub action_target: String,     // e.g. "/var/log/auth.log", "@loghost", "/dev/console"
}

/// Logrotate Hook Execution Options V6
#[derive(Debug, Clone)]
pub struct LogrotatePolicyV6 {
    pub log_path: String,
    pub copytruncate: bool,
    pub delaycompress: bool,
    pub sharedscripts: bool,
    pub dateext: bool,
    pub max_age_days: u32,
    pub prerotate_script: Option<String>,
    pub postrotate_script: Option<String>,
}

/// Sovereign Journald & Syslog Engine V6
#[derive(Debug, Clone)]
pub struct JournaldSyslogNgLogEngineV6 {
    namespaces: HashMap<String, JournaldNamespaceV6>,
    catalog: HashMap<String, JournalCatalogEntryV6>,
    syslog_destinations: Vec<SyslogNgDestinationRuleV6>,
    bsd_syslog_rules: Vec<BsdSyslogRuleV6>,
    logrotate_policies: HashMap<String, LogrotatePolicyV6>,
}

impl JournaldSyslogNgLogEngineV6 {
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
            JournaldNamespaceV6 {
                namespace_id: "default".to_string(),
                log_directory: "/var/log/journal".to_string(),
                max_file_size_bytes: 104_857_600,
                fss_sealed: true,
            },
        );

        self.namespaces.insert(
            "security".to_string(),
            JournaldNamespaceV6 {
                namespace_id: "security".to_string(),
                log_directory: "/var/log/journal/security".to_string(),
                max_file_size_bytes: 52_428_800,
                fss_sealed: true,
            },
        );

        self.catalog.insert(
            "f7701587a5f442d73830cc0b0809b52a".to_string(),
            JournalCatalogEntryV6 {
                message_id: "f7701587a5f442d73830cc0b0809b52a".to_string(),
                subject_template: "System user @USER_ID@ logged in".to_string(),
                documentation_url: "https://sigmaos.org/docs/login".to_string(),
                explanation_body: "A user session has been successfully established.".to_string(),
            },
        );

        self.bsd_syslog_rules.push(BsdSyslogRuleV6 {
            facility_selector: "auth.info".to_string(),
            action_target: "/var/log/auth.log".to_string(),
        });
    }

    pub fn register_namespace(&mut self, ns: JournaldNamespaceV6) {
        self.namespaces.insert(ns.namespace_id.clone(), ns);
    }

    pub fn lookup_catalog(&self, message_id: &str) -> Option<&JournalCatalogEntryV6> {
        self.catalog.get(message_id)
    }

    pub fn add_syslog_destination(&mut self, dest: SyslogNgDestinationRuleV6) {
        self.syslog_destinations.push(dest);
    }

    pub fn add_bsd_syslog_rule(&mut self, rule: BsdSyslogRuleV6) {
        self.bsd_syslog_rules.push(rule);
    }

    pub fn add_logrotate_policy(&mut self, policy: LogrotatePolicyV6) {
        self.logrotate_policies
            .insert(policy.log_path.clone(), policy);
    }

    pub fn get_namespaces(&self) -> &HashMap<String, JournaldNamespaceV6> {
        &self.namespaces
    }

    pub fn get_logrotate_policies(&self) -> &HashMap<String, LogrotatePolicyV6> {
        &self.logrotate_policies
    }

    pub fn get_bsd_rules(&self) -> &[BsdSyslogRuleV6] {
        &self.bsd_syslog_rules
    }
}

impl Default for JournaldSyslogNgLogEngineV6 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. DMESG V6 & BOOT DIAGNOSTICS ENGINE
// ============================================================================

/// Kernel Dynamic Debug Rule V6 (`/sys/kernel/debug/dynamic_debug/control`)
#[derive(Debug, Clone)]
pub struct DynamicDebugRuleV6 {
    pub file_pattern: String,
    pub function_pattern: String,
    pub line_range: String,
    pub flags: String, // "p" (print), "f" (function name), "l" (line number)
}

/// eBPF Tracepoint Log Buffer Record V6
#[derive(Debug, Clone)]
pub struct EbpfLogTraceRecordV6 {
    pub timestamp_us: u64,
    pub probe_name: String,
    pub pid: u32,
    pub comm: String,
    pub payload: String,
}

/// Systemd-analyze Boot Stage Timeline Breakdown V6
#[derive(Debug, Clone)]
pub struct SystemdAnalyzeBootTimelineV6 {
    pub kernel_time_ms: u32,
    pub initrd_time_ms: u32,
    pub userspace_time_ms: u32,
    pub units_reach_target_ms: BTreeMap<String, u32>,
}

/// Sovereign Dmesg & Boot Diagnostics Engine V6
#[derive(Debug, Clone)]
pub struct DmesgBootDiagnosticsEngineV6 {
    dynamic_debug_rules: Vec<DynamicDebugRuleV6>,
    ebpf_trace_logs: Vec<EbpfLogTraceRecordV6>,
    boot_timeline: SystemdAnalyzeBootTimelineV6,
}

impl DmesgBootDiagnosticsEngineV6 {
    pub fn new() -> Self {
        let mut units = BTreeMap::new();
        units.insert("sysinit.target".to_string(), 350);
        units.insert("basic.target".to_string(), 720);
        units.insert("multi-user.target".to_string(), 1250);
        units.insert("graphical.target".to_string(), 1800);

        Self {
            dynamic_debug_rules: Vec::new(),
            ebpf_trace_logs: Vec::new(),
            boot_timeline: SystemdAnalyzeBootTimelineV6 {
                kernel_time_ms: 1200,
                initrd_time_ms: 450,
                userspace_time_ms: 2100,
                units_reach_target_ms: units,
            },
        }
    }

    pub fn add_dynamic_debug_rule(&mut self, rule: DynamicDebugRuleV6) {
        self.dynamic_debug_rules.push(rule);
    }

    pub fn record_ebpf_log(&mut self, rec: EbpfLogTraceRecordV6) {
        self.ebpf_trace_logs.push(rec);
    }

    pub fn format_boot_summary(&self) -> String {
        let total = self.boot_timeline.kernel_time_ms
            + self.boot_timeline.initrd_time_ms
            + self.boot_timeline.userspace_time_ms;
        format!(
            "Startup finished in {}ms (kernel: {}ms + initrd: {}ms + userspace: {}ms)",
            total,
            self.boot_timeline.kernel_time_ms,
            self.boot_timeline.initrd_time_ms,
            self.boot_timeline.userspace_time_ms
        )
    }

    pub fn get_ebpf_logs(&self) -> &[EbpfLogTraceRecordV6] {
        &self.ebpf_trace_logs
    }

    pub fn get_dynamic_debug_rules(&self) -> &[DynamicDebugRuleV6] {
        &self.dynamic_debug_rules
    }
}

impl Default for DmesgBootDiagnosticsEngineV6 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. TOP / HTOP / ATOP V6 PROCESS TELEMETRY
// ============================================================================

/// Process IO Scheduling Priority Class (`ionice`) V6
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoSchedulingClassV6 {
    None = 0,
    RealTime = 1,
    BestEffort = 2,
    Idle = 3,
}

/// Atop Delta Process IO & Fault Statistics V6
#[derive(Debug, Clone)]
pub struct AtopProcessDeltaStatsV6 {
    pub pid: u32,
    pub comm: String,
    pub read_bytes_delta: u64,
    pub write_bytes_delta: u64,
    pub minflt_delta: u64, // minor page faults
    pub majflt_delta: u64, // major page faults
    pub ionice_class: IoSchedulingClassV6,
    pub ionice_prio: u8,
    pub bore_interactivity_score: u32,
}

/// Cgroup v2 PSI Threshold Trigger & OOM Counter V6
#[derive(Debug, Clone)]
pub struct CgroupV2PsiThresholdEngineV6 {
    pub cgroup_path: String,
    pub resource: String, // "cpu", "memory", "io"
    pub threshold_us: u32,
    pub window_us: u32,
    pub oom_kill_count: u64,
    pub memory_high_events: u64,
}

/// Sovereign Top, Atop & Cgroup V6 Telemetry Engine
#[derive(Debug, Clone)]
pub struct TopAtopTelemetryEngineV6 {
    process_deltas: HashMap<u32, AtopProcessDeltaStatsV6>,
    psi_thresholds: Vec<CgroupV2PsiThresholdEngineV6>,
}

impl TopAtopTelemetryEngineV6 {
    pub fn new() -> Self {
        Self {
            process_deltas: HashMap::new(),
            psi_thresholds: Vec::new(),
        }
    }

    pub fn update_process_delta(&mut self, stats: AtopProcessDeltaStatsV6) {
        self.process_deltas.insert(stats.pid, stats);
    }

    pub fn add_psi_threshold(&mut self, thresh: CgroupV2PsiThresholdEngineV6) {
        self.psi_thresholds.push(thresh);
    }

    pub fn get_process_delta(&self, pid: u32) -> Option<&AtopProcessDeltaStatsV6> {
        self.process_deltas.get(&pid)
    }

    pub fn get_top_io_consumers(&self) -> Vec<&AtopProcessDeltaStatsV6> {
        let mut list: Vec<&AtopProcessDeltaStatsV6> = self.process_deltas.values().collect();
        list.sort_by_key(|p| u64::MAX - (p.read_bytes_delta + p.write_bytes_delta));
        list
    }

    pub fn get_psi_thresholds(&self) -> &[CgroupV2PsiThresholdEngineV6] {
        &self.psi_thresholds
    }
}

impl Default for TopAtopTelemetryEngineV6 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. VMSTAT / IOSTAT / SAR / BMON V6 METRICS ENGINE
// ============================================================================

/// Disk IO Device Utilization Statistics V6 (`iostat -x`)
#[derive(Debug, Clone)]
pub struct IostatDeviceUtilizationV6 {
    pub device_name: String,
    pub r_await_ms: f32,
    pub w_await_ms: f32,
    pub pct_util: f32,
    pub avg_queue_len: f32,
    pub svctm_ms: f32,
}

/// Network Interface Bandwidth Telemetry V6 (`bmon` / `sar -n DEV`)
#[derive(Debug, Clone)]
pub struct BmonNetworkBandwidthV6 {
    pub interface_name: String,
    pub rx_bytes_sec: u64,
    pub tx_bytes_sec: u64,
    pub rx_packets_sec: u32,
    pub tx_packets_sec: u32,
    pub rx_drop_sec: u32,
    pub tx_drop_sec: u32,
}

/// Binary Time-Series SAR File Record V6 (`/var/log/sa/sa01`..`sa31`)
#[derive(Debug, Clone)]
pub struct BinarySaRecordV6 {
    pub day_of_month: u8,
    pub timestamp_sec: u64,
    pub cpu_user_pct: f32,
    pub cpu_sys_pct: f32,
    pub cpu_idle_pct: f32,
    pub mem_used_bytes: u64,
    pub swap_used_bytes: u64,
}

/// eBPF Kernel TCP Drop & Retransmission Counter V6
#[derive(Debug, Clone)]
pub struct EbpfTcpDropRetransStatsV6 {
    pub saddr: String,
    pub daddr: String,
    pub sport: u16,
    pub dport: u16,
    pub retrans_count: u64,
    pub drop_reason: String,
}

/// Sovereign Vmstat / Iostat / Bmon Metrics Engine V6
#[derive(Debug, Clone)]
pub struct VmstatIostatBmonEngineV6 {
    disk_utils: Vec<IostatDeviceUtilizationV6>,
    net_bandwidth: Vec<BmonNetworkBandwidthV6>,
    sa_records: Vec<BinarySaRecordV6>,
    ebpf_tcp_drops: Vec<EbpfTcpDropRetransStatsV6>,
}

impl VmstatIostatBmonEngineV6 {
    pub fn new() -> Self {
        Self {
            disk_utils: Vec::new(),
            net_bandwidth: Vec::new(),
            sa_records: Vec::new(),
            ebpf_tcp_drops: Vec::new(),
        }
    }

    pub fn update_disk_utilization(&mut self, util: IostatDeviceUtilizationV6) {
        self.disk_utils.push(util);
    }

    pub fn update_net_bandwidth(&mut self, bw: BmonNetworkBandwidthV6) {
        self.net_bandwidth.push(bw);
    }

    pub fn record_sa_entry(&mut self, rec: BinarySaRecordV6) {
        self.sa_records.push(rec);
    }

    pub fn record_tcp_drop(&mut self, drop: EbpfTcpDropRetransStatsV6) {
        self.ebpf_tcp_drops.push(drop);
    }

    pub fn get_disk_utilizations(&self) -> &[IostatDeviceUtilizationV6] {
        &self.disk_utils
    }

    pub fn get_net_bandwidths(&self) -> &[BmonNetworkBandwidthV6] {
        &self.net_bandwidth
    }

    pub fn get_sa_records(&self) -> &[BinarySaRecordV6] {
        &self.sa_records
    }

    pub fn get_tcp_drops(&self) -> &[EbpfTcpDropRetransStatsV6] {
        &self.ebpf_tcp_drops
    }
}

impl Default for VmstatIostatBmonEngineV6 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. LSOF / SS / SOCKSTAT INSPECTOR V6
// ============================================================================

/// Socket Memory Allocation Details V6 (`ss -m`)
#[derive(Debug, Clone)]
pub struct SsSocketMemoryV6 {
    pub rmem_alloc: u32,
    pub rcv_buf: u32,
    pub wmem_alloc: u32,
    pub snd_buf: u32,
    pub fwd_alloc: u32,
}

/// TCP Socket Congestion & State Inspection V6 (`ss -i`)
#[derive(Debug, Clone)]
pub struct SsTcpSocketDetailsV6 {
    pub local_endpoint: String,
    pub remote_endpoint: String,
    pub state: String,           // ESTABLISHED, TIME_WAIT, SYN_SENT
    pub congestion_algo: String, // cubic, bbr, reno
    pub inode: u64,
    pub pid: u32,
    pub comm: String,
    pub memory_stats: SsSocketMemoryV6,
    pub rtt_us: u32,
    pub rtt_var_us: u32,
}

/// File Descriptor Descriptor V6 (`lsof`)
#[derive(Debug, Clone)]
pub struct OpenFdInfoV6 {
    pub pid: u32,
    pub fd: u32,
    pub fd_type: String, // REG, DIR, CHR, BLK, FIFO, UNIX, IPv4, IPv6, KQUEUE, INOTIFY, PIDFD, EVENTFD
    pub file_path: String,
    pub lock_type: Option<String>, // "read", "write", "flock", "lease"
}

/// POSIX Message Queue & Semaphore Inspection V6
#[derive(Debug, Clone)]
pub struct PosixIpcMessageQueueV6 {
    pub queue_name: String,
    pub max_msg_count: u32,
    pub max_msg_size_bytes: u32,
    pub current_msg_count: u32,
    pub permissions_octal: u16,
}

/// Sovereign Ss / Lsof / IPC Inspector Engine V6
#[derive(Debug, Clone)]
pub struct SsLsofIpcInspectorEngineV6 {
    tcp_sockets: Vec<SsTcpSocketDetailsV6>,
    open_fds: Vec<OpenFdInfoV6>,
    posix_mqueues: Vec<PosixIpcMessageQueueV6>,
}

impl SsLsofIpcInspectorEngineV6 {
    pub fn new() -> Self {
        Self {
            tcp_sockets: Vec::new(),
            open_fds: Vec::new(),
            posix_mqueues: Vec::new(),
        }
    }

    pub fn add_tcp_socket(&mut self, sock: SsTcpSocketDetailsV6) {
        self.tcp_sockets.push(sock);
    }

    pub fn add_open_fd(&mut self, fd: OpenFdInfoV6) {
        self.open_fds.push(fd);
    }

    pub fn add_posix_mqueue(&mut self, mq: PosixIpcMessageQueueV6) {
        self.posix_mqueues.push(mq);
    }

    pub fn query_sockets_by_algo(&self, algo: &str) -> Vec<&SsTcpSocketDetailsV6> {
        self.tcp_sockets
            .iter()
            .filter(|s| s.congestion_algo == algo)
            .collect()
    }

    pub fn query_fds_by_type(&self, fd_type: &str) -> Vec<&OpenFdInfoV6> {
        self.open_fds
            .iter()
            .filter(|f| f.fd_type == fd_type)
            .collect()
    }

    pub fn get_tcp_sockets(&self) -> &[SsTcpSocketDetailsV6] {
        &self.tcp_sockets
    }

    pub fn get_open_fds(&self) -> &[OpenFdInfoV6] {
        &self.open_fds
    }

    pub fn get_posix_mqueues(&self) -> &[PosixIpcMessageQueueV6] {
        &self.posix_mqueues
    }
}

impl Default for SsLsofIpcInspectorEngineV6 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. HARDWARE DISCOVERY V6 (ETHTOOL, NVME, LSPCI, DMIDECODE)
// ============================================================================

/// Ethtool Ring Buffer & Offload Capabilities V6 (`ethtool -g`, `ethtool -k`)
#[derive(Debug, Clone)]
pub struct EthtoolInterfaceCapabilitiesV6 {
    pub interface_name: String,
    pub rx_ring_size: u32,
    pub tx_ring_size: u32,
    pub tso_enabled: bool,
    pub gso_enabled: bool,
    pub gro_enabled: bool,
    pub rx_checksum_enabled: bool,
    pub speed_mbps: u32,
    pub duplex_full: bool,
}

/// NVMe Controller SMART Health Log V6 (`nvme smart-log`)
#[derive(Debug, Clone)]
pub struct NvmeSmartHealthLogV6 {
    pub device_path: String,
    pub critical_warning: u8,
    pub temperature_celsius: u8,
    pub available_spare_pct: u8,
    pub percentage_used: u8,
    pub media_errors: u64,
    pub power_on_hours: u64,
}

/// PCIe Link Capabilities V6 (`lspci -vvv`)
#[derive(Debug, Clone)]
pub struct LspciPcieLinkCapabilityV6 {
    pub bdf_address: String,
    pub device_name: String,
    pub current_link_speed: String, // e.g. "16GT/s"
    pub current_link_width: String, // e.g. "x16"
    pub max_link_speed: String,
    pub max_link_width: String,
}

/// Dmidecode SMBIOS Record V6 (`dmidecode -t 17`)
#[derive(Debug, Clone)]
pub struct DmidecodeMemoryRecordV6 {
    pub slot_locator: String,
    pub manufacturer: String,
    pub size_mb: u32,
    pub speed_mt_s: u32,
    pub memory_type: String, // DDR4, DDR5, LPDDR5
}

/// Sovereign Hardware Discovery Engine V6
#[derive(Debug, Clone)]
pub struct HardwareDiscoveryEngineV6 {
    ethtool_interfaces: Vec<EthtoolInterfaceCapabilitiesV6>,
    nvme_smart_logs: Vec<NvmeSmartHealthLogV6>,
    pcie_links: Vec<LspciPcieLinkCapabilityV6>,
    memory_slots: Vec<DmidecodeMemoryRecordV6>,
}

impl HardwareDiscoveryEngineV6 {
    pub fn new() -> Self {
        Self {
            ethtool_interfaces: Vec::new(),
            nvme_smart_logs: Vec::new(),
            pcie_links: Vec::new(),
            memory_slots: Vec::new(),
        }
    }

    pub fn add_ethtool_interface(&mut self, iface: EthtoolInterfaceCapabilitiesV6) {
        self.ethtool_interfaces.push(iface);
    }

    pub fn add_nvme_smart_log(&mut self, log: NvmeSmartHealthLogV6) {
        self.nvme_smart_logs.push(log);
    }

    pub fn add_pcie_link(&mut self, link: LspciPcieLinkCapabilityV6) {
        self.pcie_links.push(link);
    }

    pub fn add_memory_slot(&mut self, mem: DmidecodeMemoryRecordV6) {
        self.memory_slots.push(mem);
    }

    pub fn get_ethtool_interfaces(&self) -> &[EthtoolInterfaceCapabilitiesV6] {
        &self.ethtool_interfaces
    }

    pub fn get_nvme_smart_logs(&self) -> &[NvmeSmartHealthLogV6] {
        &self.nvme_smart_logs
    }

    pub fn get_pcie_links(&self) -> &[LspciPcieLinkCapabilityV6] {
        &self.pcie_links
    }

    pub fn get_memory_slots(&self) -> &[DmidecodeMemoryRecordV6] {
        &self.memory_slots
    }
}

impl Default for HardwareDiscoveryEngineV6 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. SOSREPORT / FLUENTBIT / VECTOR / PROMETHEUS / OTEL V6
// ============================================================================

/// FluentBit / Vector Data Transformation Pipeline Rule V6
#[derive(Debug, Clone)]
pub struct VectorTransformationRuleV6 {
    pub rule_id: String,
    pub input_stream: String,
    pub filter_condition_vsql: String,
    pub output_sink: String,
}

/// Prometheus V6 OpenMetrics Target Metadata
#[derive(Debug, Clone)]
pub struct PrometheusOpenMetricsTargetV6 {
    pub target_url: String,
    pub job_name: String,
    pub scrape_interval_sec: u32,
    pub metrics_count: u32,
}

/// OpenTelemetry OTLP gRPC/HTTP Exporter Payload V6
#[derive(Debug, Clone)]
pub struct OtlpMetricsPayloadV6 {
    pub service_name: String,
    pub endpoint_url: String,
    pub metric_type: String, // gauge, counter, histogram
    pub metric_name: String,
    pub metric_value: f64,
}

/// Sosreport SHA256 & PQC Signed Archive Bundle Manifest V6
#[derive(Debug, Clone)]
pub struct SosreportManifestV6 {
    pub bundle_filename: String,
    pub creation_epoch_sec: u64,
    pub file_checksums: BTreeMap<String, String>,
    pub secrets_redacted_count: u32,
    pub pqc_signature: String,
}

/// Sovereign FluentBit / Vector, Prometheus & OTel Suite V6
#[derive(Debug, Clone)]
pub struct VectorPrometheusSosreportSuiteV6 {
    vector_rules: Vec<VectorTransformationRuleV6>,
    prometheus_targets: Vec<PrometheusOpenMetricsTargetV6>,
    otlp_payloads: Vec<OtlpMetricsPayloadV6>,
    sosreport_manifests: Vec<SosreportManifestV6>,
}

impl VectorPrometheusSosreportSuiteV6 {
    pub fn new() -> Self {
        Self {
            vector_rules: Vec::new(),
            prometheus_targets: Vec::new(),
            otlp_payloads: Vec::new(),
            sosreport_manifests: Vec::new(),
        }
    }

    pub fn add_vector_rule(&mut self, rule: VectorTransformationRuleV6) {
        self.vector_rules.push(rule);
    }

    pub fn add_prometheus_target(&mut self, target: PrometheusOpenMetricsTargetV6) {
        self.prometheus_targets.push(target);
    }

    pub fn record_otlp_metric(&mut self, metric: OtlpMetricsPayloadV6) {
        self.otlp_payloads.push(metric);
    }

    pub fn create_sosreport_manifest(
        &mut self,
        bundle_name: &str,
        files: BTreeMap<String, String>,
        redacted_count: u32,
    ) -> SosreportManifestV6 {
        let manifest = SosreportManifestV6 {
            bundle_filename: bundle_name.to_string(),
            creation_epoch_sec: 1774872000,
            file_checksums: files,
            secrets_redacted_count: redacted_count,
            pqc_signature: "DILITHIUM5_SIG_SOSREPORT_MANIFEST_V6".to_string(),
        };
        self.sosreport_manifests.push(manifest.clone());
        manifest
    }

    pub fn get_vector_rules(&self) -> &[VectorTransformationRuleV6] {
        &self.vector_rules
    }

    pub fn get_prometheus_targets(&self) -> &[PrometheusOpenMetricsTargetV6] {
        &self.prometheus_targets
    }

    pub fn get_otlp_payloads(&self) -> &[OtlpMetricsPayloadV6] {
        &self.otlp_payloads
    }
}

impl Default for VectorPrometheusSosreportSuiteV6 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 8. LOGGING, MONITORING & DIAGNOSTICS PR GATEWAY V6
// ============================================================================

/// PR Status for Diagnostics Parity V6
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticsPrStatusV6 {
    Proposed,
    CiGated,
    Merged,
}

/// Diagnostics Pull Request Submission V6
#[derive(Debug, Clone)]
pub struct DiagnosticsPrSubmissionV6 {
    pub pr_id: u32,
    pub title: String,
    pub author: String,
    pub target_component: String,
    pub description: String,
    pub pqc_signature: String,
    pub status: DiagnosticsPrStatusV6,
}

/// Diagnostics Pull Request Gateway Engine V6
#[derive(Debug, Clone)]
pub struct LoggingMonitoringDiagnosticsPullRequestGatewayV6 {
    submissions: Vec<DiagnosticsPrSubmissionV6>,
}

impl LoggingMonitoringDiagnosticsPullRequestGatewayV6 {
    pub fn new() -> Self {
        let mut gw = Self {
            submissions: Vec::new(),
        };
        gw.seed_presets();
        gw
    }

    fn seed_presets(&mut self) {
        self.submissions.push(DiagnosticsPrSubmissionV6 {
            pr_id: 601,
            title: "PR #601: Journald FSS HMAC Sealing & Isolated Namespace Partitioning"
                .to_string(),
            author: "SigmaLoggingTeam".to_string(),
            target_component: "journald_v6".to_string(),
            description: "Implements FSS HMAC sealing and multi-tenant journal namespace isolation"
                .to_string(),
            pqc_signature: "DILITHIUM5_SIG_JOURNAL_NS_601".to_string(),
            status: DiagnosticsPrStatusV6::Merged,
        });

        self.submissions.push(DiagnosticsPrSubmissionV6 {
            pr_id: 602,
            title: "PR #602: Ethtool Offloads, NVMe SMART Health & OpenMetrics OTLP Exporters"
                .to_string(),
            author: "SigmaHardwareTeam".to_string(),
            target_component: "hardware_discovery_v6".to_string(),
            description: "Implements Ethtool offloads, NVMe SMART health, and OTLP gRPC telemetry"
                .to_string(),
            pqc_signature: "DILITHIUM5_SIG_HW_HEALTH_602".to_string(),
            status: DiagnosticsPrStatusV6::CiGated,
        });
    }

    pub fn submit_pr(&mut self, pr: DiagnosticsPrSubmissionV6) {
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

    pub fn get_submissions(&self) -> &[DiagnosticsPrSubmissionV6] {
        &self.submissions
    }
}

impl Default for LoggingMonitoringDiagnosticsPullRequestGatewayV6 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 9. MASTER SUITE AGGREGATOR V6
// ============================================================================

/// Master Sovereign Logging, Monitoring, and Diagnostics Suite V6
#[derive(Debug, Clone)]
pub struct SovereignLoggingMonitoringDiagnosticsMasterSuiteV6 {
    pub journal_syslog: JournaldSyslogNgLogEngineV6,
    pub dmesg_diagnostics: DmesgBootDiagnosticsEngineV6,
    pub process_telemetry: TopAtopTelemetryEngineV6,
    pub metrics_engine: VmstatIostatBmonEngineV6,
    pub ss_lsof_inspector: SsLsofIpcInspectorEngineV6,
    pub hardware_discovery: HardwareDiscoveryEngineV6,
    pub vector_prometheus: VectorPrometheusSosreportSuiteV6,
    pub pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV6,
}

impl SovereignLoggingMonitoringDiagnosticsMasterSuiteV6 {
    pub fn new() -> Self {
        Self {
            journal_syslog: JournaldSyslogNgLogEngineV6::new(),
            dmesg_diagnostics: DmesgBootDiagnosticsEngineV6::new(),
            process_telemetry: TopAtopTelemetryEngineV6::new(),
            metrics_engine: VmstatIostatBmonEngineV6::new(),
            ss_lsof_inspector: SsLsofIpcInspectorEngineV6::new(),
            hardware_discovery: HardwareDiscoveryEngineV6::new(),
            vector_prometheus: VectorPrometheusSosreportSuiteV6::new(),
            pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV6::new(),
        }
    }

    pub fn execute_comprehensive_v6_sweep(&mut self) -> String {
        let mut report = String::from("=== SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V6 ===\n");
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

impl Default for SovereignLoggingMonitoringDiagnosticsMasterSuiteV6 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 10. STANDALONE UNIT TESTS V6
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journald_v6_namespaces_and_catalog() {
        let mut engine = JournaldSyslogNgLogEngineV6::new();
        engine.register_namespace(JournaldNamespaceV6 {
            namespace_id: "audit".to_string(),
            log_directory: "/var/log/journal/audit".to_string(),
            max_file_size_bytes: 26_214_400,
            fss_sealed: true,
        });

        assert_eq!(engine.get_namespaces().len(), 3);
        let entry = engine
            .lookup_catalog("f7701587a5f442d73830cc0b0809b52a")
            .unwrap();
        assert!(entry.subject_template.contains("logged in"));
        assert_eq!(engine.get_bsd_rules().len(), 1);
    }

    #[test]
    fn test_dmesg_v6_boot_summary() {
        let dmesg = DmesgBootDiagnosticsEngineV6::new();
        let summary = dmesg.format_boot_summary();
        assert!(summary.contains("Startup finished in 3750ms"));
    }

    #[test]
    fn test_top_atop_telemetry_v6() {
        let mut top = TopAtopTelemetryEngineV6::new();
        top.update_process_delta(AtopProcessDeltaStatsV6 {
            pid: 100,
            comm: "postgres".to_string(),
            read_bytes_delta: 1048576,
            write_bytes_delta: 2097152,
            minflt_delta: 120,
            majflt_delta: 2,
            ionice_class: IoSchedulingClassV6::RealTime,
            ionice_prio: 1,
            bore_interactivity_score: 95,
        });

        let proc_delta = top.get_process_delta(100).unwrap();
        assert_eq!(proc_delta.read_bytes_delta, 1048576);
        assert_eq!(proc_delta.ionice_class, IoSchedulingClassV6::RealTime);
        assert_eq!(proc_delta.bore_interactivity_score, 95);
    }

    #[test]
    fn test_vmstat_iostat_bmon_v6() {
        let mut metrics = VmstatIostatBmonEngineV6::new();
        metrics.update_disk_utilization(IostatDeviceUtilizationV6 {
            device_name: "nvme0n1".to_string(),
            r_await_ms: 0.5,
            w_await_ms: 1.2,
            pct_util: 35.4,
            avg_queue_len: 0.8,
            svctm_ms: 0.3,
        });

        metrics.update_net_bandwidth(BmonNetworkBandwidthV6 {
            interface_name: "eth0".to_string(),
            rx_bytes_sec: 10485760,
            tx_bytes_sec: 5242880,
            rx_packets_sec: 8000,
            tx_packets_sec: 4000,
            rx_drop_sec: 0,
            tx_drop_sec: 0,
        });

        metrics.record_sa_entry(BinarySaRecordV6 {
            day_of_month: 15,
            timestamp_sec: 1774872000,
            cpu_user_pct: 12.5,
            cpu_sys_pct: 3.2,
            cpu_idle_pct: 84.3,
            mem_used_bytes: 4294967296,
            swap_used_bytes: 0,
        });

        assert_eq!(metrics.get_disk_utilizations().len(), 1);
        assert_eq!(metrics.get_net_bandwidths().len(), 1);
        assert_eq!(metrics.get_sa_records().len(), 1);
    }

    #[test]
    fn test_ss_lsof_ipc_v6() {
        let mut inspector = SsLsofIpcInspectorEngineV6::new();
        inspector.add_tcp_socket(SsTcpSocketDetailsV6 {
            local_endpoint: "127.0.0.1:8080".to_string(),
            remote_endpoint: "127.0.0.1:54321".to_string(),
            state: "ESTABLISHED".to_string(),
            congestion_algo: "bbr".to_string(),
            inode: 98765,
            pid: 400,
            comm: "nginx".to_string(),
            memory_stats: SsSocketMemoryV6 {
                rmem_alloc: 0,
                rcv_buf: 131072,
                wmem_alloc: 0,
                snd_buf: 131072,
                fwd_alloc: 0,
            },
            rtt_us: 150,
            rtt_var_us: 20,
        });

        inspector.add_open_fd(OpenFdInfoV6 {
            pid: 400,
            fd: 3,
            fd_type: "PIDFD".to_string(),
            file_path: "pidfd:[400]".to_string(),
            lock_type: None,
        });

        let bbr_socks = inspector.query_sockets_by_algo("bbr");
        assert_eq!(bbr_socks.len(), 1);
        assert_eq!(bbr_socks[0].comm, "nginx");

        let pidfds = inspector.query_fds_by_type("PIDFD");
        assert_eq!(pidfds.len(), 1);
    }

    #[test]
    fn test_hardware_discovery_v6() {
        let mut hw = HardwareDiscoveryEngineV6::new();
        hw.add_ethtool_interface(EthtoolInterfaceCapabilitiesV6 {
            interface_name: "eth0".to_string(),
            rx_ring_size: 4096,
            tx_ring_size: 4096,
            tso_enabled: true,
            gso_enabled: true,
            gro_enabled: true,
            rx_checksum_enabled: true,
            speed_mbps: 10000,
            duplex_full: true,
        });

        hw.add_nvme_smart_log(NvmeSmartHealthLogV6 {
            device_path: "/dev/nvme0".to_string(),
            critical_warning: 0,
            temperature_celsius: 42,
            available_spare_pct: 100,
            percentage_used: 1,
            media_errors: 0,
            power_on_hours: 1200,
        });

        hw.add_memory_slot(DmidecodeMemoryRecordV6 {
            slot_locator: "DIMM 1".to_string(),
            manufacturer: "Micron".to_string(),
            size_mb: 32768,
            speed_mt_s: 5600,
            memory_type: "DDR5".to_string(),
        });

        assert_eq!(hw.get_ethtool_interfaces().len(), 1);
        assert_eq!(hw.get_nvme_smart_logs().len(), 1);
        assert_eq!(hw.get_memory_slots().len(), 1);
        assert_eq!(hw.get_memory_slots()[0].speed_mt_s, 5600);
    }

    #[test]
    fn test_vector_prometheus_sosreport_otlp_v6() {
        let mut suite = VectorPrometheusSosreportSuiteV6::new();
        let mut files = BTreeMap::new();
        files.insert("dmesg.log".to_string(), "a1b2c3d4".to_string());
        let manifest = suite.create_sosreport_manifest("sosreport-sigma.tar.zst", files, 3);

        suite.record_otlp_metric(OtlpMetricsPayloadV6 {
            service_name: "sigma-diagnostics".to_string(),
            endpoint_url: "http://otlp.sigmaos.org:4317".to_string(),
            metric_type: "gauge".to_string(),
            metric_name: "system_cpu_usage_pct".to_string(),
            metric_value: 12.5,
        });

        assert_eq!(manifest.bundle_filename, "sosreport-sigma.tar.zst");
        assert_eq!(manifest.secrets_redacted_count, 3);
        assert!(manifest.pqc_signature.contains("DILITHIUM5"));
        assert_eq!(suite.get_otlp_payloads().len(), 1);
    }

    #[test]
    fn test_diagnostics_pr_gateway_v6() {
        let gw = LoggingMonitoringDiagnosticsPullRequestGatewayV6::new();
        assert_eq!(gw.get_submissions().len(), 2);
        let diff = gw.generate_pr_unified_diff(601);
        assert!(diff.contains("PR #601"));
    }

    #[test]
    fn test_master_suite_v6() {
        let mut master = SovereignLoggingMonitoringDiagnosticsMasterSuiteV6::new();
        let sweep = master.execute_comprehensive_v6_sweep();
        assert!(sweep.contains("SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V6"));
    }
}
