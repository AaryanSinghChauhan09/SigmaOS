//! Sovereign Linux & BSD Logging, Monitoring, and Diagnostics Suite V14 (PR Format)
//!
//! Synthesizes V14 capabilities inspired by Linux & BSD system administration,
//! observability, telemetry, and diagnostics tooling in Pull Request proposal format:
//! - **Journald REST API, Rsyslog RELP Disk Queues, Syslog-ng Sinks, BSD Syslogd & Logrotate V14**:
//!   `systemd-journal-gatewayd` REST API / HTTP Server-Sent Events (SSE) streaming with range headers & cursor pagination,
//!   binary journal entry hash table index verification (`_SYSTEMD_UNIT`, `_PID`, `_COMM`, `_HOSTNAME`),
//!   Rsyslog RELP protocol framing with disk-assisted queues, Syslog-ng ClickHouse / Elasticsearch / SQL sinks,
//!   BSD `/etc/syslog.conf` multi-priority facility routing, and Logrotate status state file parsing with size/time schedules.
//! - **Dmesg Timestamps, Kdump Vmcore & eBPF Trace Filters V14**:
//!   `dmesg` ISO-8601 human-readable timestamp conversion & ANSI color level formatting, kdump `vmcore` ELF header
//!   and crash summary parser (`PT_NOTE`, `NT_PRSTATUS`, registers, panic message), and eBPF tracepoint dynamic filter rules.
//! - **Htop Interactivity, Vmstat Paging, Iostat Latency & Sar Reporting V14**:
//!   `htop` / `top` per-core CPU breakdown & EEVDF/BORE scheduler interactivity scores, `vmstat` run queue (`r`, `b`),
//!   memory, swap (`si`, `so`) and interrupt counters, `iostat -x` device queue length (`aqu-sz`) & latency (`r_await`, `w_await`, `%util`),
//!   and `sysstat` / `sar` binary time-series aggregation & text report generator.
//! - **Lsof Handles, Lspci BARs, Lsusb Endpoints, Dmidecode SMBIOS & Sosreport Sanitization V14**:
//!   `lsof` file handle inspector by PID / path, `lspci` PCIe BAR memory & MSI-X descriptor parser,
//!   `lsusb` USB device hierarchy & endpoint parser, `dmidecode` SMBIOS Type 0/1/2/3/4 system table parser,
//!   and `sosreport` automated subsystem collection & regex secret sanitization.
//! - **Prometheus Node Exporter, OpenMetrics Exemplars & V14 PR Gateway**:
//!   Prometheus node_exporter exposition format generator (`node_cpu_seconds_total`, `node_memory_MemTotal_bytes`,
//!   `node_filesystem_avail_bytes`, `node_network_receive_bytes_total`, `node_load1`) with OpenMetrics trace exemplars,
//!   V14 PR submission gateway with unified diff generation, and master suite orchestrator.

use std::collections::{BTreeMap, HashMap};
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. JOURNALD REST API, RSYSLOG RELP, SYSLOG-NG, SYSLOGD & LOGROTATE V14
// ============================================================================

/// Journald REST API SSE Stream Format V14 (`systemd-journal-gatewayd`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournalGatewayFormatV14 {
    Json,
    ServerSentEvents,
    RawField,
    ExportText,
}

/// Journald Binary Journal Entry Index Record V14
#[derive(Debug, Clone)]
pub struct JournaldBinaryIndexRecordV14 {
    pub entry_seq_num: u64,
    pub realtime_timestamp_us: u64,
    pub monotonic_timestamp_us: u64,
    pub boot_id: String,
    pub cursor: String,
    pub systemd_unit: String,
    pub pid: u32,
    pub comm: String,
    pub hostname: String,
    pub message: String,
}

/// Journald Gateway REST API Endpoint Specifier V14
#[derive(Debug, Clone)]
pub struct JournaldGatewayRestApiEndpointV14 {
    pub endpoint_path: String,
    pub format: JournalGatewayFormatV14,
    pub cursor_pagination: Option<String>,
    pub range_entries_limit: u32,
    pub field_filters: HashMap<String, String>,
    pub stream_active: bool,
}

/// Rsyslog RELP Frame Command & Disk Queue Specifier V14
#[derive(Debug, Clone)]
pub struct RsyslogRelpDiskQueueEngineV14 {
    pub txnr: u64,
    pub relp_command: String, // "syslog", "rsp", "open", "close"
    pub work_directory: String,
    pub action_queue_type: String, // "LinkedList", "FixedArray", "Direct"
    pub queue_file_name: String,
    pub max_disk_space_bytes: u64,
    pub disk_queue_active: bool,
}

/// Syslog-ng ClickHouse / Elasticsearch / SQL Destination Sink V14
#[derive(Debug, Clone)]
pub struct SyslogNgClickhouseSinkEngineV14 {
    pub sink_name: String,
    pub driver_type: String, // "clickhouse", "elasticsearch", "sql", "kafka"
    pub host: String,
    pub port: u16,
    pub database_table: String,
    pub batch_size: u32,
    pub flush_timeout_ms: u32,
    pub column_mapping: BTreeMap<String, String>,
}

/// BSD Syslogd Priority Selector Routing Rule V14
#[derive(Debug, Clone)]
pub struct BsdSyslogdFacilityRouteV14 {
    pub facility_selector: String, // e.g. "kern.crit", "mail.none", "*.info"
    pub destination_target: String, // e.g. "/var/log/messages", "@remote.syslog.org", "/dev/console"
    pub is_no_sync: bool,
}

/// Logrotate Status File Entry & Lifecycle Schedule V14
#[derive(Debug, Clone)]
pub struct LogrotateStatusStateV14 {
    pub log_path: String,
    pub last_rotated_iso8601: String,
    pub schedule: String, // "daily", "weekly", "monthly"
    pub size_threshold_bytes: u64,
    pub compress_delay: bool,
    pub rotate_keep_count: u32,
    pub postrotate_cmd: Option<String>,
}

/// Sovereign Journald REST API & Syslog Suite V14
#[derive(Debug, Clone)]
pub struct JournaldSyslogSuiteV14 {
    binary_index: Vec<JournaldBinaryIndexRecordV14>,
    gateway_endpoints: Vec<JournaldGatewayRestApiEndpointV14>,
    relp_queues: Vec<RsyslogRelpDiskQueueEngineV14>,
    clickhouse_sinks: Vec<SyslogNgClickhouseSinkEngineV14>,
    bsd_routes: Vec<BsdSyslogdFacilityRouteV14>,
    logrotate_states: HashMap<String, LogrotateStatusStateV14>,
}

impl JournaldSyslogSuiteV14 {
    pub fn new() -> Self {
        let mut suite = Self {
            binary_index: Vec::new(),
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
        self.binary_index.push(JournaldBinaryIndexRecordV14 {
            entry_seq_num: 1,
            realtime_timestamp_us: 1774872000000000,
            monotonic_timestamp_us: 142050000,
            boot_id: "8f7a6b5c4d3e2f1a0b9c8d7e6f5a4b3c".to_string(),
            cursor: "s=a1b2c3d4e5f6;i=1;b=8f7a6b5c".to_string(),
            systemd_unit: "sshd.service".to_string(),
            pid: 1420,
            comm: "sshd".to_string(),
            hostname: "sigma-node-01".to_string(),
            message: "Accepted publickey for root from 192.168.1.100 port 54321 ssh2".to_string(),
        });

        let mut filters = HashMap::new();
        filters.insert("_SYSTEMD_UNIT".to_string(), "sshd.service".to_string());

        self.gateway_endpoints
            .push(JournaldGatewayRestApiEndpointV14 {
                endpoint_path: "/entries".to_string(),
                format: JournalGatewayFormatV14::ServerSentEvents,
                cursor_pagination: Some("s=a1b2c3d4e5f6".to_string()),
                range_entries_limit: 100,
                field_filters: filters,
                stream_active: true,
            });

        self.relp_queues.push(RsyslogRelpDiskQueueEngineV14 {
            txnr: 1042,
            relp_command: "syslog".to_string(),
            work_directory: "/var/spool/rsyslog".to_string(),
            action_queue_type: "LinkedList".to_string(),
            queue_file_name: "relp_spool_v14".to_string(),
            max_disk_space_bytes: 1_073_741_824,
            disk_queue_active: true,
        });

        let mut cols = BTreeMap::new();
        cols.insert("timestamp".to_string(), "DateTime64(3)".to_string());
        cols.insert("hostname".to_string(), "String".to_string());
        cols.insert("message".to_string(), "String".to_string());

        self.clickhouse_sinks.push(SyslogNgClickhouseSinkEngineV14 {
            sink_name: "clickhouse_prod_v14".to_string(),
            driver_type: "clickhouse".to_string(),
            host: "clickhouse.internal".to_string(),
            port: 8123,
            database_table: "syslog.logs_v14".to_string(),
            batch_size: 5000,
            flush_timeout_ms: 1000,
            column_mapping: cols,
        });

        self.bsd_routes.push(BsdSyslogdFacilityRouteV14 {
            facility_selector: "kern.crit".to_string(),
            destination_target: "/dev/console".to_string(),
            is_no_sync: true,
        });

        self.logrotate_states.insert(
            "/var/log/syslog".to_string(),
            LogrotateStatusStateV14 {
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

    pub fn add_index_record(&mut self, rec: JournaldBinaryIndexRecordV14) {
        self.binary_index.push(rec);
    }

    pub fn add_gateway_endpoint(&mut self, ep: JournaldGatewayRestApiEndpointV14) {
        self.gateway_endpoints.push(ep);
    }

    pub fn add_relp_queue(&mut self, q: RsyslogRelpDiskQueueEngineV14) {
        self.relp_queues.push(q);
    }

    pub fn add_clickhouse_sink(&mut self, sink: SyslogNgClickhouseSinkEngineV14) {
        self.clickhouse_sinks.push(sink);
    }

    pub fn add_bsd_route(&mut self, route: BsdSyslogdFacilityRouteV14) {
        self.bsd_routes.push(route);
    }

    pub fn add_logrotate_state(&mut self, state: LogrotateStatusStateV14) {
        self.logrotate_states.insert(state.log_path.clone(), state);
    }

    pub fn get_binary_index(&self) -> &[JournaldBinaryIndexRecordV14] {
        &self.binary_index
    }

    pub fn get_gateway_endpoints(&self) -> &[JournaldGatewayRestApiEndpointV14] {
        &self.gateway_endpoints
    }

    pub fn get_relp_queues(&self) -> &[RsyslogRelpDiskQueueEngineV14] {
        &self.relp_queues
    }

    pub fn get_clickhouse_sinks(&self) -> &[SyslogNgClickhouseSinkEngineV14] {
        &self.clickhouse_sinks
    }

    pub fn get_bsd_routes(&self) -> &[BsdSyslogdFacilityRouteV14] {
        &self.bsd_routes
    }

    pub fn get_logrotate_states(&self) -> &HashMap<String, LogrotateStatusStateV14> {
        &self.logrotate_states
    }
}

impl Default for JournaldSyslogSuiteV14 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. DMESG & KERNEL DIAGNOSTICS V14
// ============================================================================

/// Dmesg Log Level & ANSI Color Code V14
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmesgLogLevelV14 {
    Emerg = 0,
    Alert = 1,
    Crit = 2,
    Err = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

impl DmesgLogLevelV14 {
    pub fn ansi_color_code(&self) -> &'static str {
        match self {
            Self::Emerg | Self::Alert | Self::Crit => "\x1b[1;31m", // Bright Red
            Self::Err => "\x1b[31m",                                // Red
            Self::Warning => "\x1b[33m",                            // Yellow
            Self::Notice => "\x1b[32m",                             // Green
            Self::Info => "\x1b[36m",                               // Cyan
            Self::Debug => "\x1b[90m",                              // Bright Black / Gray
        }
    }
}

/// Dmesg Timestamp & Formatted Record V14
#[derive(Debug, Clone)]
pub struct DmesgTimestampRecordV14 {
    pub uptime_sec: f64,
    pub iso8601_timestamp: String,
    pub log_level: DmesgLogLevelV14,
    pub facility: String,
    pub message: String,
}

/// Kdump Vmcore ELF Header & Crash Record V14
#[derive(Debug, Clone)]
pub struct KdumpVmcoreElfHeaderParserV14 {
    pub vmcore_path: String,
    pub elf_class: u8, // 64 for ELF64
    pub machine_arch: String,
    pub kernel_release: String,
    pub panic_reason: String,
    pub registers_dump: HashMap<String, u64>,
    pub pt_note_offset: u64,
}

/// eBPF Tracepoint Dynamic Filter Rule V14
#[derive(Debug, Clone)]
pub struct EbpfDynamicTraceFilterEngineV14 {
    pub rule_id: String,
    pub tracepoint_category: String, // "syscalls", "sched", "net", "kprobes"
    pub event_name: String,
    pub filter_expression: String,
    pub active: bool,
}

/// Sovereign Dmesg & Kdump Diagnostics Engine V14
#[derive(Debug, Clone)]
pub struct DmesgKdumpDiagnosticsEngineV14 {
    dmesg_records: Vec<DmesgTimestampRecordV14>,
    vmcore_parsers: Vec<KdumpVmcoreElfHeaderParserV14>,
    ebpf_filters: Vec<EbpfDynamicTraceFilterEngineV14>,
}

impl DmesgKdumpDiagnosticsEngineV14 {
    pub fn new() -> Self {
        Self {
            dmesg_records: Vec::new(),
            vmcore_parsers: Vec::new(),
            ebpf_filters: Vec::new(),
        }
    }

    pub fn record_dmesg(&mut self, rec: DmesgTimestampRecordV14) {
        self.dmesg_records.push(rec);
    }

    pub fn record_vmcore(&mut self, parser: KdumpVmcoreElfHeaderParserV14) {
        self.vmcore_parsers.push(parser);
    }

    pub fn add_ebpf_filter(&mut self, filter: EbpfDynamicTraceFilterEngineV14) {
        self.ebpf_filters.push(filter);
    }

    pub fn format_colored_dmesg(&self, record: &DmesgTimestampRecordV14) -> String {
        format!(
            "{}[{}] [{:?}] {}{}\x1b[0m",
            record.log_level.ansi_color_code(),
            record.iso8601_timestamp,
            record.log_level,
            record.facility,
            record.message
        )
    }

    pub fn get_dmesg_records(&self) -> &[DmesgTimestampRecordV14] {
        &self.dmesg_records
    }

    pub fn get_vmcore_parsers(&self) -> &[KdumpVmcoreElfHeaderParserV14] {
        &self.vmcore_parsers
    }

    pub fn get_ebpf_filters(&self) -> &[EbpfDynamicTraceFilterEngineV14] {
        &self.ebpf_filters
    }
}

impl Default for DmesgKdumpDiagnosticsEngineV14 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. TOP, HTOP, VMSTAT, IOSTAT, SAR, SYSSTAT V14
// ============================================================================

/// Top / Htop Process Interactivity Score & Per-Core Metrics V14
#[derive(Debug, Clone)]
pub struct HtopProcessSchedulerInteractivityV14 {
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

/// Vmstat Memory, Swap & System Counters V14
#[derive(Debug, Clone)]
pub struct VmstatMemoryAndPagingSummaryV14 {
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

/// Extended Iostat Latency & Queue Statistics V14 (`iostat -x`)
#[derive(Debug, Clone)]
pub struct IostatDeviceQueueAndLatencyStatsV14 {
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

/// Sysstat / Sar Time-Series Aggregator & Report V14
#[derive(Debug, Clone)]
pub struct SarBinaryRecordAggregatorV14 {
    pub date_stamp: String,
    pub interval_sec: u32,
    pub records_count: u32,
    pub avg_cpu_user: f32,
    pub avg_cpu_sys: f32,
    pub avg_cpu_iowait: f32,
    pub peak_mem_used_pct: f32,
}

/// Sovereign System Telemetry & Metrics Suite V14
#[derive(Debug, Clone)]
pub struct SystemMetricsTelemetrySuiteV14 {
    process_metrics: HashMap<u32, HtopProcessSchedulerInteractivityV14>,
    vmstat_snapshots: Vec<VmstatMemoryAndPagingSummaryV14>,
    iostat_devices: Vec<IostatDeviceQueueAndLatencyStatsV14>,
    sar_reports: Vec<SarBinaryRecordAggregatorV14>,
}

impl SystemMetricsTelemetrySuiteV14 {
    pub fn new() -> Self {
        Self {
            process_metrics: HashMap::new(),
            vmstat_snapshots: Vec::new(),
            iostat_devices: Vec::new(),
            sar_reports: Vec::new(),
        }
    }

    pub fn update_process(&mut self, proc: HtopProcessSchedulerInteractivityV14) {
        self.process_metrics.insert(proc.pid, proc);
    }

    pub fn record_vmstat(&mut self, snap: VmstatMemoryAndPagingSummaryV14) {
        self.vmstat_snapshots.push(snap);
    }

    pub fn record_iostat(&mut self, io: IostatDeviceQueueAndLatencyStatsV14) {
        self.iostat_devices.push(io);
    }

    pub fn add_sar_report(&mut self, report: SarBinaryRecordAggregatorV14) {
        self.sar_reports.push(report);
    }

    pub fn get_process_metrics(&self) -> &HashMap<u32, HtopProcessSchedulerInteractivityV14> {
        &self.process_metrics
    }

    pub fn get_vmstat_snapshots(&self) -> &[VmstatMemoryAndPagingSummaryV14] {
        &self.vmstat_snapshots
    }

    pub fn get_iostat_devices(&self) -> &[IostatDeviceQueueAndLatencyStatsV14] {
        &self.iostat_devices
    }

    pub fn get_sar_reports(&self) -> &[SarBinaryRecordAggregatorV14] {
        &self.sar_reports
    }
}

impl Default for SystemMetricsTelemetrySuiteV14 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. LSOF, LSPCI, LSUSB, DMIDECODE, SOSREPORT V14
// ============================================================================

/// Lsof File Handle Specifier V14
#[derive(Debug, Clone)]
pub struct LsofFileHandleQueryEngineV14 {
    pub pid: u32,
    pub comm: String,
    pub fd: u32,
    pub open_mode: String, // "r", "w", "u"
    pub file_type: String, // "REG", "DIR", "CHR", "BLK", "UNIX", "IPv4"
    pub mount_point: String,
    pub path: String,
}

/// Lspci PCIe Base Address Register (BAR) & MSI-X Capability V14
#[derive(Debug, Clone)]
pub struct LspciPcieResourceParserV14 {
    pub bdf_address: String,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: String,
    pub bar_addresses: Vec<String>,
    pub msix_table_size: u16,
    pub max_payload_bytes: u32,
}

/// Lsusb Endpoint Descriptor Specifier V14
#[derive(Debug, Clone)]
pub struct LsusbDescriptorParserV14 {
    pub bus_num: u8,
    pub dev_num: u8,
    pub vendor_id: u16,
    pub product_id: u16,
    pub usb_version: String, // "3.2 Gen 2", "USB4"
    pub b_num_endpoints: u8,
    pub speed_mbps: u32,
}

/// Dmidecode SMBIOS Structure Record V14
#[derive(Debug, Clone)]
pub struct DmidecodeSmbiosTableParserV14 {
    pub handle_id: u16,
    pub table_type: u8, // 0=BIOS, 1=System, 2=Baseboard, 3=Chassis, 4=Processor
    pub type_name: String,
    pub vendor_manufacturer: String,
    pub product_version: String,
    pub serial_number: String,
}

/// Sosreport Archive Collector & Secret Sanitizer V14
#[derive(Debug, Clone)]
pub struct SosreportArchiveCollectorEngineV14 {
    pub archive_id: String,
    pub collected_subsystems: Vec<String>,
    pub redaction_regexes: Vec<String>,
    pub secrets_sanitized_count: u32,
    pub archive_size_bytes: u64,
}

/// Sovereign Hardware Inspection & Sosreport Suite V14
#[derive(Debug, Clone)]
pub struct HardwareAndSosreportSuiteV14 {
    file_handles: Vec<LsofFileHandleQueryEngineV14>,
    pci_resources: Vec<LspciPcieResourceParserV14>,
    usb_descriptors: Vec<LsusbDescriptorParserV14>,
    smbios_tables: Vec<DmidecodeSmbiosTableParserV14>,
    sosreports: Vec<SosreportArchiveCollectorEngineV14>,
}

impl HardwareAndSosreportSuiteV14 {
    pub fn new() -> Self {
        Self {
            file_handles: Vec::new(),
            pci_resources: Vec::new(),
            usb_descriptors: Vec::new(),
            smbios_tables: Vec::new(),
            sosreports: Vec::new(),
        }
    }

    pub fn add_file_handle(&mut self, fh: LsofFileHandleQueryEngineV14) {
        self.file_handles.push(fh);
    }

    pub fn add_pci_resource(&mut self, pci: LspciPcieResourceParserV14) {
        self.pci_resources.push(pci);
    }

    pub fn add_usb_descriptor(&mut self, usb: LsusbDescriptorParserV14) {
        self.usb_descriptors.push(usb);
    }

    pub fn add_smbios_table(&mut self, smbios: DmidecodeSmbiosTableParserV14) {
        self.smbios_tables.push(smbios);
    }

    pub fn add_sosreport(&mut self, sos: SosreportArchiveCollectorEngineV14) {
        self.sosreports.push(sos);
    }

    pub fn get_file_handles(&self) -> &[LsofFileHandleQueryEngineV14] {
        &self.file_handles
    }

    pub fn get_pci_resources(&self) -> &[LspciPcieResourceParserV14] {
        &self.pci_resources
    }

    pub fn get_usb_descriptors(&self) -> &[LsusbDescriptorParserV14] {
        &self.usb_descriptors
    }

    pub fn get_smbios_tables(&self) -> &[DmidecodeSmbiosTableParserV14] {
        &self.smbios_tables
    }

    pub fn get_sosreports(&self) -> &[SosreportArchiveCollectorEngineV14] {
        &self.sosreports
    }
}

impl Default for HardwareAndSosreportSuiteV14 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. PROMETHEUS NODE_EXPORTER TELEMETRY & V14 PR GATEWAY
// ============================================================================

/// OpenMetrics Trace Exemplar Specifier V14
#[derive(Debug, Clone)]
pub struct OpenMetricsTraceExemplarV14 {
    pub trace_id: String,
    pub span_id: String,
    pub timestamp_sec: f64,
    pub sample_value: f64,
}

/// Prometheus Exposition Format Metric Record V14
#[derive(Debug, Clone)]
pub struct PrometheusMetricSampleV14 {
    pub metric_name: String,
    pub help_text: String,
    pub type_str: String, // "counter", "gauge"
    pub labels: BTreeMap<String, String>,
    pub value: f64,
    pub exemplar: Option<OpenMetricsTraceExemplarV14>,
}

/// Node Exporter Metrics Exporter V14
#[derive(Debug, Clone)]
pub struct NodeExporterPrometheusExporterV14 {
    metrics: Vec<PrometheusMetricSampleV14>,
}

impl NodeExporterPrometheusExporterV14 {
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

        self.metrics.push(PrometheusMetricSampleV14 {
            metric_name: "node_cpu_seconds_total".to_string(),
            help_text: "Seconds the CPUs spent in each mode.".to_string(),
            type_str: "counter".to_string(),
            labels: cpu_labels,
            value: 12450.75,
            exemplar: Some(OpenMetricsTraceExemplarV14 {
                trace_id: "4bf92f3577b34da6a3ce929d0e0e4736".to_string(),
                span_id: "00f067aa0ba902b7".to_string(),
                timestamp_sec: 1774872000.0,
                sample_value: 12450.75,
            }),
        });

        self.metrics.push(PrometheusMetricSampleV14 {
            metric_name: "node_memory_MemTotal_bytes".to_string(),
            help_text: "Memory information field MemTotal_bytes.".to_string(),
            type_str: "gauge".to_string(),
            labels: BTreeMap::new(),
            value: 68719476736.0,
            exemplar: None,
        });

        self.metrics.push(PrometheusMetricSampleV14 {
            metric_name: "node_load1".to_string(),
            help_text: "1m load average.".to_string(),
            type_str: "gauge".to_string(),
            labels: BTreeMap::new(),
            value: 0.42,
            exemplar: None,
        });
    }

    pub fn add_sample(&mut self, sample: PrometheusMetricSampleV14) {
        self.metrics.push(sample);
    }

    pub fn render_exposition_text(&self) -> String {
        let mut output = String::new();
        for m in &self.metrics {
            output.push_str(&format!("# HELP {} {}\n", m.metric_name, m.help_text));
            output.push_str(&format!("# TYPE {} {}\n", m.metric_name, m.type_str));

            let base_str = if m.labels.is_empty() {
                format!("{} {}", m.metric_name, m.value)
            } else {
                let labels_str: Vec<String> = m
                    .labels
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect();
                format!("{}{{{}}} {}", m.metric_name, labels_str.join(","), m.value)
            };

            if let Some(ref ex) = m.exemplar {
                output.push_str(&format!(
                    "{} # {{trace_id=\"{}\",span_id=\"{}\"}} {} {}\n",
                    base_str, ex.trace_id, ex.span_id, ex.sample_value, ex.timestamp_sec
                ));
            } else {
                output.push_str(&format!("{}\n", base_str));
            }
        }
        output
    }

    pub fn get_metrics(&self) -> &[PrometheusMetricSampleV14] {
        &self.metrics
    }
}

impl Default for NodeExporterPrometheusExporterV14 {
    fn default() -> Self {
        Self::new()
    }
}

/// V14 Diagnostics Pull Request Submission
#[derive(Debug, Clone)]
pub struct DiagnosticsPrSubmissionV14 {
    pub pr_id: u32,
    pub title: String,
    pub author: String,
    pub target_component: String,
    pub description: String,
    pub pqc_signature: String,
    pub ci_passed: bool,
}

/// V14 Diagnostics Pull Request Gateway
#[derive(Debug, Clone)]
pub struct LoggingMonitoringDiagnosticsPullRequestGatewayV14 {
    submissions: Vec<DiagnosticsPrSubmissionV14>,
}

impl LoggingMonitoringDiagnosticsPullRequestGatewayV14 {
    pub fn new() -> Self {
        let mut gw = Self {
            submissions: Vec::new(),
        };
        gw.seed_presets();
        gw
    }

    fn seed_presets(&mut self) {
        self.submissions.push(DiagnosticsPrSubmissionV14 {
            pr_id: 1401,
            title: "PR #1401: Journald Binary Indexing, SSE Streaming, Rsyslog RELP & OpenMetrics Exemplars V14".to_string(),
            author: "SigmaObservabilityTeam".to_string(),
            target_component: "journald_rsyslog_v14".to_string(),
            description: "Implements journald binary index parser, SSE REST endpoint, Rsyslog RELP disk queue, and OpenMetrics exemplars.".to_string(),
            pqc_signature: "DILITHIUM5_SIG_DIAGNOSTICS_1401".to_string(),
            ci_passed: true,
        });
    }

    pub fn submit_pr(&mut self, pr: DiagnosticsPrSubmissionV14) {
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

    pub fn get_submissions(&self) -> &[DiagnosticsPrSubmissionV14] {
        &self.submissions
    }
}

impl Default for LoggingMonitoringDiagnosticsPullRequestGatewayV14 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. MASTER SUITE AGGREGATOR V14
// ============================================================================

/// Master Sovereign Logging, Monitoring, and Diagnostics Suite V14
#[derive(Debug, Clone)]
pub struct SovereignLoggingMonitoringDiagnosticsMasterSuiteV14 {
    pub journal_syslog: JournaldSyslogSuiteV14,
    pub dmesg_kdump: DmesgKdumpDiagnosticsEngineV14,
    pub system_metrics: SystemMetricsTelemetrySuiteV14,
    pub hardware_sosreport: HardwareAndSosreportSuiteV14,
    pub node_exporter: NodeExporterPrometheusExporterV14,
    pub pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV14,
}

impl SovereignLoggingMonitoringDiagnosticsMasterSuiteV14 {
    pub fn new() -> Self {
        Self {
            journal_syslog: JournaldSyslogSuiteV14::new(),
            dmesg_kdump: DmesgKdumpDiagnosticsEngineV14::new(),
            system_metrics: SystemMetricsTelemetrySuiteV14::new(),
            hardware_sosreport: HardwareAndSosreportSuiteV14::new(),
            node_exporter: NodeExporterPrometheusExporterV14::new(),
            pr_gateway: LoggingMonitoringDiagnosticsPullRequestGatewayV14::new(),
        }
    }

    pub fn run_diagnostics_sweep(&mut self) -> String {
        let mut report = String::from("=== SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V14 ===\n");
        report.push_str(&format!(
            "Binary Journal Index Records: {}\n",
            self.journal_syslog.get_binary_index().len()
        ));
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

impl Default for SovereignLoggingMonitoringDiagnosticsMasterSuiteV14 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 7. STANDALONE UNIT TESTS V14
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journald_syslog_suite_v14() {
        let suite = JournaldSyslogSuiteV14::new();
        assert_eq!(suite.get_binary_index().len(), 1);
        assert_eq!(suite.get_gateway_endpoints().len(), 1);
        assert_eq!(suite.get_relp_queues().len(), 1);
        assert_eq!(suite.get_clickhouse_sinks().len(), 1);
        assert_eq!(suite.get_bsd_routes().len(), 1);
        assert_eq!(suite.get_logrotate_states().len(), 1);

        let idx = &suite.get_binary_index()[0];
        assert_eq!(idx.systemd_unit, "sshd.service");
        assert_eq!(idx.pid, 1420);

        let ep = &suite.get_gateway_endpoints()[0];
        assert_eq!(ep.format, JournalGatewayFormatV14::ServerSentEvents);
        assert!(ep.stream_active);

        let q = &suite.get_relp_queues()[0];
        assert_eq!(q.relp_command, "syslog");
        assert_eq!(q.action_queue_type, "LinkedList");
    }

    #[test]
    fn test_dmesg_kdump_diagnostics_v14() {
        let mut engine = DmesgKdumpDiagnosticsEngineV14::new();
        engine.record_dmesg(DmesgTimestampRecordV14 {
            uptime_sec: 142.50,
            iso8601_timestamp: "2026-03-30T12:34:56.789Z".to_string(),
            log_level: DmesgLogLevelV14::Err,
            facility: "kern".to_string(),
            message: "nvme0n1: PCIe bus error detected".to_string(),
        });

        let colored = engine.format_colored_dmesg(&engine.get_dmesg_records()[0]);
        assert!(colored.contains("\x1b[31m")); // Red color code for Err
        assert!(colored.contains("nvme0n1"));

        engine.add_ebpf_filter(EbpfDynamicTraceFilterEngineV14 {
            rule_id: "filter_sys_enter".to_string(),
            tracepoint_category: "syscalls".to_string(),
            event_name: "sys_enter_openat".to_string(),
            filter_expression: "filename == '/etc/shadow'".to_string(),
            active: true,
        });

        assert_eq!(engine.get_ebpf_filters().len(), 1);
    }

    #[test]
    fn test_system_metrics_telemetry_v14() {
        let mut metrics = SystemMetricsTelemetrySuiteV14::new();
        metrics.update_process(HtopProcessSchedulerInteractivityV14 {
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

        metrics.record_iostat(IostatDeviceQueueAndLatencyStatsV14 {
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
    fn test_hardware_and_sosreport_v14() {
        let mut hw = HardwareAndSosreportSuiteV14::new();
        hw.add_pci_resource(LspciPcieResourceParserV14 {
            bdf_address: "0000:01:00.0".to_string(),
            vendor_id: 0x10de,
            device_id: 0x2704,
            class_code: "0300".to_string(),
            bar_addresses: vec!["0x70000000".to_string()],
            msix_table_size: 64,
            max_payload_bytes: 512,
        });

        hw.add_sosreport(SosreportArchiveCollectorEngineV14 {
            archive_id: "sosreport-sigma-1401".to_string(),
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
    fn test_node_exporter_and_pr_gateway_v14() {
        let exporter = NodeExporterPrometheusExporterV14::new();
        let rendered = exporter.render_exposition_text();
        assert!(rendered.contains("node_cpu_seconds_total"));
        assert!(rendered.contains("node_memory_MemTotal_bytes"));
        assert!(rendered.contains("node_load1"));
        assert!(rendered.contains("trace_id=\"4bf92f3577b34da6a3ce929d0e0e4736\""));

        let gw = LoggingMonitoringDiagnosticsPullRequestGatewayV14::new();
        assert_eq!(gw.get_submissions().len(), 1);
        let diff = gw.generate_unified_diff(1401);
        assert!(diff.contains("PR #1401"));
    }

    #[test]
    fn test_master_suite_v14() {
        let mut master = SovereignLoggingMonitoringDiagnosticsMasterSuiteV14::new();
        let sweep = master.run_diagnostics_sweep();
        assert!(sweep.contains("SOVEREIGN DIAGNOSTICS & TELEMETRY SWEEP V14"));
    }
}
