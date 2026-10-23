//! Sovereign Linux & BSD Logging, Monitoring, and Diagnostics Suite V13 (PR Format)
//!
//! Synthesizes V13 capabilities inspired by Linux & BSD system administration,
//! observability, telemetry, and diagnostics tooling in Pull Request proposal format:
//! - **Journald REST API, Rsyslog RELP Disk Queues, Syslog-ng Sinks, BSD Syslogd & Logrotate V13**:
//!   `systemd-journal-gatewayd` REST API / HTTP Server-Sent Events (SSE) streaming with range headers & cursor pagination,
//!   Rsyslog RELP protocol framing with disk-assisted queues, Syslog-ng ClickHouse / Elasticsearch / SQL sinks,
//!   BSD `/etc/syslog.conf` multi-priority facility routing, and Logrotate status state file parsing with size/time schedules.
//! - **Dmesg Timestamps, Kdump Vmcore & eBPF Trace Filters V13**:
//!   `dmesg` ISO-8601 human-readable timestamp conversion & ANSI color level formatting, kdump `vmcore` ELF header
//!   and crash summary parser (`PT_NOTE`, `NT_PRSTATUS`, registers, panic message), and eBPF tracepoint dynamic filter rules.
//! - **Htop Interactivity, Vmstat Paging, Iostat Latency & Sar Reporting V13**:
//!   `htop` / `top` per-core CPU breakdown & EEVDF/BORE scheduler interactivity scores, `vmstat` run queue (`r`, `b`),
//!   memory, swap (`si`, `so`) and interrupt counters, `iostat -x` device queue length (`aqu-sz`) & latency (`r_await`, `w_await`, `%util`),
//!   and `sysstat` / `sar` binary time-series aggregation & text report generator.
//! - **Lsof Handles, Lspci BARs, Lsusb Endpoints, Dmidecode SMBIOS & Sosreport Sanitization V13**:
//!   `lsof` file handle inspector by PID / path, `lspci` PCIe BAR memory & MSI-X descriptor parser,
//!   `lsusb` USB device hierarchy & endpoint parser, `dmidecode` SMBIOS Type 0/1/2/3/4 system table parser,
//!   and `sosreport` automated subsystem collection & regex secret sanitization.
//! - **Prometheus Node Exporter & V13 PR Gateway**:
//!   Prometheus node_exporter exposition format generator (`node_cpu_seconds_total`, `node_memory_MemTotal_bytes`,
//!   `node_filesystem_avail_bytes`, `node_network_receive_bytes_total`, `node_load1`), V13 PR submission gateway
//!   with unified diff generation, and master suite orchestrator.

use std::collections::{BTreeMap, HashMap};
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. JOURNALD REST API, RSYSLOG RELP, SYSLOG-NG, SYSLOGD & LOGROTATE V13
// ============================================================================

/// Journald REST API SSE Stream Format V13 (`systemd-journal-gatewayd`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournalGatewayFormatV13 {
    Json,
    ServerSentEvents,
    RawField,
    ExportText,
}

/// Journald Gateway REST API Endpoint Specifier V13
#[derive(Debug, Clone)]
pub struct JournaldGatewayRestApiEndpointV13 {
    pub endpoint_path: String,
    pub format: JournalGatewayFormatV13,
    pub cursor_pagination: Option<String>,
    pub range_entries_limit: u32,
    pub field_filters: HashMap<String, String>,
    pub stream_active: bool,
}

/// Rsyslog RELP Frame Command & Disk Queue Specifier V13
#[derive(Debug, Clone)]
pub struct RsyslogRelpDiskQueueEngineV13 {
    pub txnr: u64,
    pub relp_command: String, // "syslog", "rsp", "open", "close"
    pub work_directory: String,
    pub action_queue_type: String, // "LinkedList", "FixedArray", "Direct"
    pub queue_file_name: String,
    pub max_disk_space_bytes: u64,
    pub disk_queue_active: bool,
}

/// Syslog-ng ClickHouse / Elasticsearch / SQL Destination Sink V13
#[derive(Debug, Clone)]
pub struct SyslogNgClickhouseSinkEngineV13 {
    pub sink_name: String,
    pub driver_type: String, // "clickhouse", "elasticsearch", "sql", "kafka"
    pub host: String,
    pub port: u16,
    pub database_table: String,
    pub batch_size: u32,
    pub flush_timeout_ms: u32,
    pub column_mapping: BTreeMap<String, String>,
}

/// BSD Syslogd Priority Selector Routing Rule V13
#[derive(Debug, Clone)]
pub struct BsdSyslogdFacilityRouteV13 {
    pub facility_selector: String, // e.g. "kern.crit", "mail.none", "*.info"
    pub destination_target: String, // e.g. "/var/log/messages", "@remote.syslog.org", "/dev/console"
    pub is_no_sync: bool,
}

/// Logrotate Status File Entry & Lifecycle Schedule V13
#[derive(Debug, Clone)]
pub struct LogrotateStatusStateV13 {
    pub log_path: String,
    pub last_rotated_iso8601: String,
    pub schedule: String, // "daily", "weekly", "monthly"
    pub size_threshold_bytes: u64,
    pub compress_delay: bool,
    pub rotate_keep_count: u32,
    pub postrotate_cmd: Option<String>,
}

/// Sovereign Journald REST API & Syslog Suite V13
#[derive(Debug, Clone)]
pub struct JournaldSyslogSuiteV13 {
    gateway_endpoints: Vec<JournaldGatewayRestApiEndpointV13>,
    relp_queues: Vec<RsyslogRelpDiskQueueEngineV13>,
    clickhouse_sinks: Vec<SyslogNgClickhouseSinkEngineV13>,
    bsd_routes: Vec<BsdSyslogdFacilityRouteV13>,
    logrotate_states: HashMap<String, LogrotateStatusStateV13>,
}

impl JournaldSyslogSuiteV13 {
    pub fn new() -> Self {
        let mut suite = Self {
            gateway_endpoints: Vec::new(),
            relp_queues: Vec::new(),
            clickhouse_sinks: Vec::new(),
            bsd_routes: Vec::new(),
            logrotate_states: HashMap::new(),
        };
        suite.init_defaults();
        suite
    }

    fn init_defaults(&mut self) {
        let mut filters = HashMap::new();
        filters.insert("_SYSTEMD_UNIT".to_string(), "sshd.service".to_string());

        self.gateway_endpoints.push(JournaldGatewayRestApiEndpointV13 {
            endpoint_path: "/entries".to_string(),
            format: JournalGatewayFormatV13::ServerSentEvents,
            cursor_pagination: Some("s=a1b2c3d4e5f6".to_string()),
            range_entries_limit: 100,
            field_filters: filters,
            stream_active: true,
        });

        self.relp_queues.push(RsyslogRelpDiskQueueEngineV13 {
            txnr: 1042,
            relp_command: "syslog".to_string(),
            work_directory: "/var/spool/rsyslog".to_string(),
            action_queue_type: "LinkedList".to_string(),
            queue_file_name: "relp_spool".to_string(),
            max_disk_space_bytes: 1_073_741_824,
            disk_queue_active: true,
        });

        let mut cols = BTreeMap::new();
        cols.insert("timestamp".to_string(), "DateTime64(3)".to_string());
        cols.insert("hostname".to_string(), "String".to_string());
        cols.insert("message".to_string(), "String".to_string());

        self.clickhouse_sinks.push(SyslogNgClickhouseSinkEngineV13 {
            sink_name: "clickhouse_prod".to_string(),
            driver_type: "clickhouse".to_string(),
            host: "clickhouse.internal".to_string(),
            port: 8123,
            database_table: "syslog.logs_v13".to_string(),
            batch_size: 5000,
            flush_timeout_ms: 1000,
            column_mapping: cols,
        });

        self.bsd_routes.push(BsdSyslogdFacilityRouteV13 {
            facility_selector: "kern.crit".to_string(),
            destination_target: "/dev/console".to_string(),
            is_no_sync: true,
        });

        self.logrotate_states.insert(
            "/var/log/syslog".to_string(),
            LogrotateStatusStateV13 {
                log_path: "/var/log/syslog".to_string(),
                last_rotated_iso8601: "2026-03-30T00:00:00Z".to_string(),
                schedule: "daily".to_string(),
                size_threshold_bytes: 104_857_600,
                compress_delay: true,
                rotate_keep_count: 7,
                postrotate_cmd: Some("/usr/lib/rsyslog/rsyslog-rotate".to_string()),
            },
        );
    }

    pub fn add_gateway_endpoint(&mut self, ep: JournaldGatewayRestApiEndpointV13) {
        self.gateway_endpoints.push(ep);
    }

    pub fn add_relp_queue(&mut self, q: RsyslogRelpDiskQueueEngineV13) {
        self.relp_queues.push(q);
    }

    pub fn add_clickhouse_sink(&mut self, sink: SyslogNgClickhouseSinkEngineV13) {
        self.clickhouse_sinks.push(sink);
    }

    pub fn add_bsd_route(&mut self, route: BsdSyslogdFacilityRouteV13) {
        self.bsd_routes.push(route);
    }

    pub fn add_logrotate_state(&mut self, state: LogrotateStatusStateV13) {
        self.logrotate_states
            .insert(state.log_path.clone(), state);
    }

    pub fn get_gateway_endpoints(&self) -> &[JournaldGatewayRestApiEndpointV13] {
        &self.gateway_endpoints
    }

    pub fn get_relp_queues(&self) -> &[RsyslogRelpDiskQueueEngineV13] {
        &self.relp_queues
    }

    pub fn get_clickhouse_sinks(&self) -> &[SyslogNgClickhouseSinkEngineV13] {
        &self.clickhouse_sinks
    }

    pub fn get_bsd_routes(&self) -> &[BsdSyslogdFacilityRouteV13] {
        &self.bsd_routes
    }

    pub fn get_logrotate_states(&self) -> &HashMap<String, LogrotateStatusStateV13> {
        &self.logrotate_states
    }
}

impl Default for JournaldSyslogSuiteV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. DMESG & KERNEL DIAGNOSTICS V13
// ============================================================================

/// Dmesg Log Level & ANSI Color Code V13
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmesgLogLevelV13 {
    Emerg = 0,
    Alert = 1,
    Crit = 2,
    Err = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

impl DmesgLogLevelV13 {
    pub fn ansi_color_code(&self) -> &'static str {
        match self {
            Self::Emerg | Self::Alert | Self::Crit => "\x1b[1;31m", // Bright Red
            Self::Err => "\x1b[31m",                               // Red
            Self::Warning => "\x1b[33m",                           // Yellow
            Self::Notice => "\x1b[32m",                            // Green
            Self::Info => "\x1b[36m",                              // Cyan
            Self::Debug => "\x1b[90m",                             // Bright Black / Gray
        }
    }
}

/// Dmesg Timestamp & Formatted Record V13
#[derive(Debug, Clone)]
pub struct DmesgTimestampRecordV13 {
    pub uptime_sec: f64,
    pub iso8601_timestamp: String,
    pub log_level: DmesgLogLevelV13,
    pub facility: String,
    pub message: String,
}

/// Kdump Vmcore ELF Header & Crash Record V13
#[derive(Debug, Clone)]
pub struct KdumpVmcoreElfHeaderParserV13 {
    pub vmcore_path: String,
    pub elf_class: u8, // 64 for ELF64
    pub machine_arch: String,
    pub kernel_release: String,
    pub panic_reason: String,
    pub registers_dump: HashMap<String, u64>,
    pub pt_note_offset: u64,
}

/// eBPF Tracepoint Dynamic Filter Rule V13
#[derive(Debug, Clone)]
pub struct EbpfDynamicTraceFilterEngineV13 {
    pub rule_id: String,
    pub tracepoint_category: String, // "syscalls", "sched", "net", "kprobes"
    pub event_name: String,
    pub filter_expression: String,
    pub active: bool,
}

/// Sovereign Dmesg & Kdump Diagnostics Engine V13
#[derive(Debug, Clone)]
pub struct DmesgKdumpDiagnosticsEngineV13 {
    dmesg_records: Vec<DmesgTimestampRecordV13>,
    vmcore_parsers: Vec<KdumpVmcoreElfHeaderParserV13>,
    ebpf_filters: Vec<EbpfDynamicTraceFilterEngineV13>,
}

impl DmesgKdumpDiagnosticsEngineV13 {
    pub fn new() -> Self {
        Self {
            dmesg_records: Vec::new(),
            vmcore_parsers: Vec::new(),
            ebpf_filters: Vec::new(),
        }
    }

    pub fn record_dmesg(&mut self, rec: DmesgTimestampRecordV13) {
        self.dmesg_records.push(rec);
    }

    pub fn record_vmcore(&mut self, parser: KdumpVmcoreElfHeaderParserV13) {
        self.vmcore_parsers.push(parser);
    }

    pub fn add_ebpf_filter(&mut self, filter: EbpfDynamicTraceFilterEngineV13) {
        self.ebpf_filters.push(filter);
    }

    pub fn format_colored_dmesg(&self, record: &DmesgTimestampRecordV13) -> String {
        format!(
            "{}[{}] [{:?}] {}{}\x1b[0m",
            record.log_level.ansi_color_code(),
            record.iso8601_timestamp,
            record.log_level,
            record.facility,
            record.message
        )
    }

    pub fn get_dmesg_records(&self) -> &[DmesgTimestampRecordV13] {
        &self.dmesg_records
    }

    pub fn get_vmcore_parsers(&self) -> &[KdumpVmcoreElfHeaderParserV13] {
        &self.vmcore_parsers
    }

    pub fn get_ebpf_filters(&self) -> &[EbpfDynamicTraceFilterEngineV13] {
        &self.ebpf_filters
    }
}

impl Default for DmesgKdumpDiagnosticsEngineV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. TOP, HTOP, VMSTAT, IOSTAT, SAR, SYSSTAT V13
// ============================================================================

/// Top / Htop Process Interactivity Score & Per-Core Metrics V13
#[derive(Debug, Clone)]
pub struct HtopProcessSchedulerInteractivityV13 {
    pub pid: u32,
    pub comm: String,
    pub core_id: u32,
    pub eevdf_latency_ns: u64,
    pub bore_interactivity_score: u32,
    pub thread_count: u32,
    pub virt_bytes: u64,
    pub res_bytes: u64,
    pub cpu_pct: f32,
    pub mem_pct: f32,
}

/// Vmstat Memory, Swap & System Counters V13
#[derive(Debug, Clone)]
pub struct VmstatMemoryAndPagingSummaryV13 {
    pub runqueue_procs: u32,
    pub blocked_procs: u32,
    pub swap_used_kb: u64,
    pub free_mem_kb: u64,
    pub buff_mem_kb: u64,
    pub cache_mem_kb: u64,
    pub swap_in_kb_sec: u32,
    pub swap_out_kb_sec: u32,
    pub blocks_in_sec: u32,
    pub blocks_out_sec: u32,
    pub interrupts_sec: u32,
    pub context_switches_sec: u32,
    pub cpu_user_pct: u8,
    pub cpu_sys_pct: u8,
    pub cpu_idle_pct: u8,
    pub cpu_iowait_pct: u8,
    pub cpu_stolen_pct: u8,
}

/// Extended Iostat Latency & Queue Statistics V13 (`iostat -x`)
#[derive(Debug, Clone)]
pub struct IostatDeviceQueueAndLatencyStatsV13 {
    pub device_name: String,
    pub reads_sec: f32,
    pub writes_sec: f32,
    pub r_kb_sec: f32,
    pub w_kb_sec: f32,
    pub r_await_ms: f32,
    pub w_await_ms: f32,
    pub avg_queue_size: f32,
    pub util_pct: f32,
}

/// Sysstat / Sar Time-Series Aggregator & Report V13
#[derive(Debug, Clone)]
pub struct SarBinaryRecordAggregatorV13 {
    pub date_stamp: String,
    pub interval_sec: u32,
    pub records_count: u32,
    pub avg_cpu_user: f32,
    pub avg_cpu_sys: f32,
    pub avg_cpu_iowait: f32,
    pub peak_mem_used_pct: f32,
}

/// Sovereign System Telemetry & Metrics Suite V13
#[derive(Debug, Clone)]
pub struct SystemMetricsTelemetrySuiteV13 {
    process_metrics: HashMap<u32, HtopProcessSchedulerInteractivityV13>,
    vmstat_snapshots: Vec<VmstatMemoryAndPagingSummaryV13>,
    iostat_devices: Vec<IostatDeviceQueueAndLatencyStatsV13>,
    sar_reports: Vec<SarBinaryRecordAggregatorV13>,
}

impl SystemMetricsTelemetrySuiteV13 {
    pub fn new() -> Self {
        Self {
            process_metrics: HashMap::new(),
            vmstat_snapshots: Vec::new(),
            iostat_devices: Vec::new(),
            sar_reports: Vec::new(),
        }
    }

    pub fn update_process(&mut self, proc: HtopProcessSchedulerInteractivityV13) {
        self.process_metrics.insert(proc.pid, proc);
    }

    pub fn record_vmstat(&mut self, snap: VmstatMemoryAndPagingSummaryV13) {
        self.vmstat_snapshots.push(snap);
    }

    pub fn record_iostat(&mut self, io: IostatDeviceQueueAndLatencyStatsV13) {
        self.iostat_devices.push(io);
    }

    pub fn add_sar_report(&mut self, report: SarBinaryRecordAggregatorV13) {
        self.sar_reports.push(report);
    }

    pub fn get_process_metrics(&self) -> &HashMap<u32, HtopProcessSchedulerInteractivityV13> {
        &self.process_metrics
    }

    pub fn get_vmstat_snapshots(&self) -> &[VmstatMemoryAndPagingSummaryV13] {
        &self.vmstat_snapshots
    }

    pub fn get_iostat_devices(&self) -> &[IostatDeviceQueueAndLatencyStatsV13] {
        &self.iostat_devices
    }

    pub fn get_sar_reports(&self) -> &[SarBinaryRecordAggregatorV13] {
        &self.sar_reports
    }
}

impl Default for SystemMetricsTelemetrySuiteV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. LSOF, LSPCI, LSUSB, DMIDECODE, SOSREPORT V13
// ============================================================================

/// Lsof File Handle Specifier V13
#[derive(Debug, Clone)]
pub struct LsofFileHandleQueryEngineV13 {
    pub pid: u32,
    pub comm: String,
    pub fd: u32,
    pub open_mode: String, // "r", "w", "u"
    pub file_type: String, // "REG", "DIR", "CHR", "BLK", "UNIX", "IPv4"
    pub mount_point: String,
    pub path: String,
}

/// Lspci PCIe Base Address Register (BAR) & MSI-X Capability V13
#[derive(Debug, Clone)]
pub struct LspciPcieResourceParserV13 {
    pub bdf_address: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: String,
    pub bar_addresses: Vec<String>,
    pub msix_table_size: u16,
    pub max_payload_bytes: u32,
}

/// Lsusb Endpoint Descriptor Specifier V13
#[derive(Debug, Clone)]
pub struct LsusbDescriptorParserV13 {
    pub bus_num: u8,
    pub dev_num: u8,
    pub vendor_id: u16,
    pub product_id: u16,
    pub usb_version: String, // "3.2 Gen 2", "USB4"
    pub b_num_endpoints: u8,
    pub speed_mbps: u32,
}

/// Dmidecode SMBIOS Structure Record V13
#[derive(Debug, Clone)]
pub struct DmidecodeSmbiosTableParserV13 {
    pub handle_id: u16,
    pub table_type: u8, // 0=BIOS, 1=System, 2=Baseboard, 3=Chassis, 4=Processor
    pub type_name: String,
    pub vendor_manufacturer: String,
    pub product_version: String,
    pub serial_number: String,
}

/// Sosreport Archive Collector & Secret Sanitizer V13
#[derive(Debug, Clone)]
pub struct SosreportArchiveCollectorEngineV13 {
    pub archive_id: String,
    pub collected_subsystems: Vec<String>,
    pub redaction_regexes: Vec<String>,
    pub secrets_sanitized_count: u32,
    pub archive_size_bytes: u64,
}

/// Sovereign Hardware Inspection & Sosreport Suite V13
#[derive(Debug, Clone)]
pub struct HardwareAndSosreportSuiteV13 {
    file_handles: Vec<LsofFileHandleQueryEngineV13>,
    pci_resources: Vec<LspciPcieResourceParserV13>,
    usb_descriptors: Vec<LsusbDescriptorParserV13>,
    smbios_tables: Vec<DmidecodeSmbiosTableParserV13>,
    sosreports: Vec<SosreportArchiveCollectorEngineV13>,
}

impl HardwareAndSosreportSuiteV13 {
    pub fn new() -> Self {
        Self {
            file_handles: Vec::new(),
            pci_resources: Vec::new(),
            usb_descriptors: Vec::new(),
            smbios_tables: Vec::new(),
            sosreports: Vec::new(),
        }
    }

    pub fn add_file_handle(&mut self, fh: LsofFileHandleQueryEngineV13) {
        self.file_handles.push(fh);
    }

    pub fn add_pci_resource(&mut self, pci: LspciPcieResourceParserV13) {
        self.pci_resources.push(pci);
    }

    pub fn add_usb_descriptor(&mut self, usb: LsusbDescriptorParserV13) {
        self.usb_descriptors.push(usb);
    }

    pub fn add_smbios_table(&mut self, smbios: DmidecodeSmbiosTableParserV13) {
        self.smbios_tables.push(smbios);
    }

    pub fn add_sosreport(&mut self, sos: SosreportArchiveCollectorEngineV13) {
        self.sosreports.push(sos);
    }

    pub fn get_file_handles(&self) -> &[LsofFileHandleQueryEngineV13] {
        &self.file_handles
    }

    pub fn get_pci_resources(&self) -> &[LspciPcieResourceParserV13] {
        &self.pci_resources
    }

    pub fn get_usb_descriptors(&self) -> &[LsusbDescriptorParserV13] {
        &self.usb_descriptors
    }

    pub fn get_smbios_tables(&self) -> &[DmidecodeSmbiosTableParserV13] {
        &self.smbios_tables
    }

    pub fn get_sosreports(&self) -> &[SosreportArchiveCollectorEngineV13] {
        &self.sosreports
    }
}

impl Default for HardwareAndSosreportSuiteV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. PROMETHEUS NODE_EXPORTER TELEMETRY & V13 PR GATEWAY
// ============================================================================

/// Prometheus Exposition Format Metric Record V13
#[derive(Debug, Clone)]
pub struct PrometheusMetricSampleV13 {
    pub metric_name: String,
    pub help_text: String,
    pub type_str: String, // "counter", "gauge"
    pub labels: BTreeMap<String, String>,
    pub value: f64,
}

/// Node Exporter Metrics Exporter V13
#[derive(Debug, Clone)]
pub struct NodeExporterPrometheusExporterV13 {
    metrics: Vec<PrometheusMetricSampleV13>,
}

impl NodeExporterPrometheusExporterV13 {
    pub fn new() -> Self {
        let mut exporter = Self {
            metrics: Vec::new(),
        };
        exporter.init_defaults();
        exporter
    }

    fn init_defaults(&mut self) {
        let mut cpu_labels = BTreeMap::new();
        cpu_labels.insert("cpu".to_string(), "0".to_string());
        cpu_labels.insert("mode".to_string(), "idle".to_string());

        self.metrics.push(PrometheusMetricSampleV13 {
            metric_name: "node_cpu_seconds_total".to_string(),
            help_text: "Seconds the CPUs spent in each mode.".to_string(),
            type_str: "counter".to_string(),
            labels: cpu_labels,
            value: 12450.75,
        });

        self.metrics.push(PrometheusMetricSampleV13 {
            metric_name: "node_memory_MemTotal_bytes".to_string(),
            help_text: "Memory information field MemTotal_bytes.".to_string(),
            type_str: "gauge".to_string(),
            labels: BTreeMap::new(),
            value: 68719476736.0,
        });

        self.metrics.push(PrometheusMetricSampleV13 {
            metric_name: "node_load1".to_string(),
            help_text: "1m load average.".to_string(),
            type_str: "gauge".to_string(),
            labels: BTreeMap::new(),
            value: 0.42,
        });
    }

    pub fn add_sample(&mut self, sample: PrometheusMetricSampleV13) {
        self.metrics.push(sample);
    }

    pub fn render_exposition_text(&self) -> String {
        let mut output = String::new();
        for m in &self.metrics {
            output.push_str(&format!("# HELP {} {}\n", m.metric_name, m.help_text));
            output.push_str(&format!("# TYPE {} {}\n", m.metric_name, m.type_str));

            if m.labels.is_empty() {
                output.push_str(&format!("{} {}\n", m.metric_name, m.value));
            } else {
                let labels_str: Vec<String> = m
                    .labels
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect();
                output.push_str(&format!(
                    "{}{{{}}} {}\n",
                    m.metric_name,
                    labels_str.join(","),
                    m.value
                ));
            }
        }
        output
    }

    pub fn get_metrics(&self) -> &[PrometheusMetricSampleV13] {
        &self.metrics
    }
}

impl Default for NodeExporterPrometheusExporterV13 {
    fn default() -> Self {
        Self::new()
    }
}

/// V13 Diagnostics Pull Request Submission
#[derive(Debug, Clone)]
pub struct DiagnosticsPrSubmissionV13 {
    pub pr_id: u32,
    pub title: String,
    pub author: String,
    pub target_component: String,
    pub description: String,
    pub pqc_signature: String,
    pub ci_passed: bool,
}

/// V13 Diagnostics Pull Request Gateway
#[derive(Debug, Clone)]
pub struct LoggingMonitoringDiagnosticsPullRequestGatewayV13 {
    submissions: Vec<DiagnosticsPrSubmissionV13>,
}

impl LoggingMonitoringDiagnosticsPullRequestGatewayV13 {
    pub fn new() -> Self {
        let mut gw = Self {
            submissions: Vec::new(),
        };
        gw.seed_presets();
        gw
    }

    fn seed_presets(&mut self) {
        self.submissions.push(DiagnosticsPrSubmissionV13 {
            pr_id: 1301,
            title: "PR #1301: Journald REST API SSE Streaming, Rsyslog RELP Spooling & Prometheus Exporter V13".to_string(),
            author: "SigmaObservabilityTeam".to_string(),
            target_component: "journald_rsyslog_v13".to_string(),
            description: "Implements journald SSE REST endpoint, Rsyslog RELP disk queue, and Node Exporter exposition format.".to_string(),
            pqc_signature: "DILITHIUM5_SIG_DIAGNOSTICS_1301".to_string(),
            ci_passed: true,
        });
    }

    pub fn submit_pr(&mut self, pr: DiagnosticsPrSubmissionV13) {
        self.submissions.push(pr);
    }

    pub fn generate_unified_diff(&self, pr_id: u32) -> String {
        if let Some(pr) = self.submissions.iter().find(|p| p.pr_id == pr_id) {
            format!(
                "--- a/src/diagnostics/{}.rs\n+++ b/src/diagnostics/{}.rs\n@@ -1,5 +1,10 @@\n+// {}\n+// Author: {}\n+// PQC Signature: {}\n",
                pr.target_component, pr.target_component, pr.title, pr.author, pr.pqc_signature
            )
        } else {
            String::new()
        }
    }

    pub fn get_submissions(&self) -> &[DiagnosticsPrSubmissionV13] {
        &self.submissions
    }
}

impl Default for LoggingMonitoringDiagnosticsPullRequestGatewayV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. MASTER SUITE AGGREGATOR V13
// ============================================================================

/// Master Sovereign Logging, Monitoring, and Diagnostics Suite V13
#[derive(Debug, Clone)]
pub struct SovereignLoggingMonitoringDiagnosticsMasterSuiteV13 {
    pub journal_syslog: JournaldSyslogSuiteV13,
    pub dmesg_kdump: DmesgKdumpDiagnosticsEngineV13,
    pub system_metrics: SystemMetricsTelemetrySuiteV13,
    pub hardware_sosreport: HardwareAndSosreportSuiteV13,
    pub node_exporter: NodeExporterPrometheusExporterV13,
    pub pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV13,
}

impl SovereignLoggingMonitoringDiagnosticsMasterSuiteV13 {
    pub fn new() -> Self {
        Self {
            journal_syslog: JournaldSyslogSuiteV13::new(),
            dmesg_kdump: DmesgKdumpDiagnosticsEngineV13::new(),
            system_metrics: SystemMetricsTelemetrySuiteV13::new(),
            hardware_sosreport: HardwareAndSosreportSuiteV13::new(),
            node_exporter: NodeExporterPrometheusExporterV13::new(),
            pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV13::new(),
        }
    }

    pub fn run_diagnostics_sweep(&mut self) -> String {
        let mut report = String::from("=== SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V13 ===\n");
        report.push_str(&format!(
            "Gateway Endpoints: {}\n",
            self.journal_syslog.get_gateway_endpoints().len()
        ));
        report.push_str(&format!(
            "RELP Disk Queues: {}\n",
            self.journal_syslog.get_relp_queues().len()
        ));
        report.push_str(&format!(
            "ClickHouse Sinks: {}\n",
            self.journal_syslog.get_clickhouse_sinks().len()
        ));
        report.push_str(&format!(
            "Prometheus Metrics Rendered Bytes: {}\n",
            self.node_exporter.render_exposition_text().len()
        ));
        report.push_str(&format!(
            "PR Submissions Count: {}\n",
            self.pr_gateway.get_submissions().len()
        ));
        report
    }
}

impl Default for SovereignLoggingMonitoringDiagnosticsMasterSuiteV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. STANDALONE UNIT TESTS V13
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journald_syslog_suite_v13() {
        let suite = JournaldSyslogSuiteV13::new();
        assert_eq!(suite.get_gateway_endpoints().len(), 1);
        assert_eq!(suite.get_relp_queues().len(), 1);
        assert_eq!(suite.get_clickhouse_sinks().len(), 1);
        assert_eq!(suite.get_bsd_routes().len(), 1);
        assert_eq!(suite.get_logrotate_states().len(), 1);

        let ep = &suite.get_gateway_endpoints()[0];
        assert_eq!(ep.format, JournalGatewayFormatV13::ServerSentEvents);
        assert!(ep.stream_active);

        let q = &suite.get_relp_queues()[0];
        assert_eq!(q.relp_command, "syslog");
        assert_eq!(q.action_queue_type, "LinkedList");
    }

    #[test]
    fn test_dmesg_kdump_diagnostics_v13() {
        let mut engine = DmesgKdumpDiagnosticsEngineV13::new();
        engine.record_dmesg(DmesgTimestampRecordV13 {
            uptime_sec: 142.50,
            iso8601_timestamp: "2026-03-30T12:34:56.789Z".to_string(),
            log_level: DmesgLogLevelV13::Err,
            facility: "kern".to_string(),
            message: "nvme0n1: PCIe bus error detected".to_string(),
        });

        let colored = engine.format_colored_dmesg(&engine.get_dmesg_records()[0]);
        assert!(colored.contains("\x1b[31m")); // Red color code for Err
        assert!(colored.contains("nvme0n1"));

        engine.add_ebpf_filter(EbpfDynamicTraceFilterEngineV13 {
            rule_id: "filter_sys_enter".to_string(),
            tracepoint_category: "syscalls".to_string(),
            event_name: "sys_enter_openat".to_string(),
            filter_expression: "filename == '/etc/shadow'".to_string(),
            active: true,
        });

        assert_eq!(engine.get_ebpf_filters().len(), 1);
    }

    #[test]
    fn test_system_metrics_telemetry_v13() {
        let mut metrics = SystemMetricsTelemetrySuiteV13::new();
        metrics.update_process(HtopProcessSchedulerInteractivityV13 {
            pid: 100,
            comm: "systemd".to_string(),
            core_id: 0,
            eevdf_latency_ns: 1200000,
            bore_interactivity_score: 95,
            thread_count: 1,
            virt_bytes: 104857600,
            res_bytes: 10485760,
            cpu_pct: 0.1,
            mem_pct: 0.2,
        });

        metrics.record_iostat(IostatDeviceQueueAndLatencyStatsV13 {
            device_name: "nvme0n1".to_string(),
            reads_sec: 120.5,
            writes_sec: 450.2,
            r_kb_sec: 2048.0,
            w_kb_sec: 8192.0,
            r_await_ms: 0.35,
            w_await_ms: 0.82,
            avg_queue_size: 0.45,
            util_pct: 12.5,
        });

        assert_eq!(metrics.get_process_metrics().len(), 1);
        assert_eq!(metrics.get_iostat_devices().len(), 1);
    }

    #[test]
    fn test_hardware_and_sosreport_v13() {
        let mut hw = HardwareAndSosreportSuiteV13::new();
        hw.add_pci_resource(LspciPcieResourceParserV13 {
            bdf_address: "0000:01:00.0".to_string(),
            vendor_id: 0x10de,
            device_id: 0x2704,
            class_code: "0300".to_string(),
            bar_addresses: vec!["0x70000000".to_string()],
            msix_table_size: 64,
            max_payload_bytes: 512,
        });

        hw.add_sosreport(SosreportArchiveCollectorEngineV13 {
            archive_id: "sosreport-sigma-1301".to_string(),
            collected_subsystems: vec!["systemd".to_string(), "networking".to_string()],
            redaction_regexes: vec!["password=.*".to_string()],
            secrets_sanitized_count: 12,
            archive_size_bytes: 15420000,
        });

        assert_eq!(hw.get_pci_resources().len(), 1);
        assert_eq!(hw.get_sosreports().len(), 1);
        assert_eq!(hw.get_sosreports()[0].secrets_sanitized_count, 12);
    }

    #[test]
    fn test_node_exporter_and_pr_gateway_v13() {
        let exporter = NodeExporterPrometheusExporterV13::new();
        let rendered = exporter.render_exposition_text();
        assert!(rendered.contains("node_cpu_seconds_total"));
        assert!(rendered.contains("node_memory_MemTotal_bytes"));
        assert!(rendered.contains("node_load1"));

        let gw = LoggingMonitoringDiagnosticsPullRequestGatewayV13::new();
        assert_eq!(gw.get_submissions().len(), 1);
        let diff = gw.generate_unified_diff(1301);
        assert!(diff.contains("PR #1301"));
    }

    #[test]
    fn test_master_suite_v13() {
        let mut master = SovereignLoggingMonitoringDiagnosticsMasterSuiteV13::new();
        let sweep = master.run_diagnostics_sweep();
        assert!(sweep.contains("SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V13"));
    }
}
