//! Integration Test Suite: Sovereign Linux & BSD Logging, Monitoring, Diagnostics V21 PR Suite
//!
//! Validates end-to-end capabilities across:
//! - Journald REST API SSE streaming & RFC 5424 headers
//! - Forward Secure Sealing (FSS) HMAC verification
//! - Rsyslog RELP protocol framing & memory/disk queue watermarks
//! - Syslog-ng multi-sink configuration & dynamic macro substitution
//! - BSD Syslogd routing rules with facilities & priorities
//! - Logrotate state parser, rotation rules, and post-rotate triggers
//! - Dmesg ISO-8601 formatting, color levels, and delta usec
//! - Kdump vmcore ELF64 PT_NOTE parser & panic registers
//! - eBPF tracepoint, kprobe, and buffer filter evaluation
//! - Htop per-core BORE/EEVDF interactivity scoring & PSI pressure
//! - Vmstat memory paging, active/inactive anon, and context switch rates
//! - Iostat -x extended disk queue depth & latency analysis
//! - Sar time-series metrics recording & report generation
//! - Lsof open handles, file descriptors, and lock types
//! - Lspci PCIe topology, ReBAR, MSI-X, generation, and link width
//! - Lsusb hierarchy, speed, and endpoint descriptors
//! - Dmidecode SMBIOS parsing across BIOS, System, and IPMI
//! - Sosreport safe regex-protected secret sanitization
//! - Prometheus node_exporter OpenMetrics text exposition
//! - PR Gateway engine, unified diff generation, and diagnostic audit

#[path = "../src/diagnostics/sovereign_logging_monitoring_diagnostics_v21_pr.rs"]
mod v21;

use v21::*;

#[test]
fn test_v21_journald_gateway_and_fss() {
    let mut gateway = JournaldGatewayRestApiEndpointV21::new(
        "/entries",
        JournalGatewayFormatV21::ServerSentEvents,
        500,
    )
    .with_filter("_SYSTEMD_UNIT", "sigma-diagnostics.service");

    gateway.set_cursor("s=1a2b3c4d5e6f");
    assert_eq!(gateway.cursor_pagination, Some("s=1a2b3c4d5e6f".to_string()));
    assert_eq!(gateway.field_filters.get("_SYSTEMD_UNIT"), Some(&"sigma-diagnostics.service".to_string()));

    let chunk = gateway.render_sse_chunk(
        "s=1a2b3c4d5e6f",
        &[
            ("MESSAGE", "Diagnostics telemetry stream started"),
            ("PRIORITY", "6"),
        ],
    );
    assert!(chunk.starts_with("event: journal_entry_v21"));
    assert!(chunk.contains("id: s=1a2b3c4d5e6f"));
    assert!(chunk.contains("data: MESSAGE: Diagnostics telemetry stream started"));

    let rfc5424 = gateway.render_rfc5424_header(134, "kernel", 1, "BOOT_EVENT");
    assert!(rfc5424.contains("<134>1"));
    assert!(rfc5424.contains("kernel 1 BOOT_EVENT"));

    assert!(gateway.fss_sealing.verify_integrity(1));
    assert!(!gateway.fss_sealing.verify_integrity(2));
    assert!(gateway.fss_sealing.hmac_tag.starts_with("fss-v21-epoch-"));
}

#[test]
fn test_v21_rsyslog_relp_and_queues() {
    let mut engine = RsyslogEngineV21::new("/var/spool/rsyslog-v21");
    assert_eq!(engine.spool_dir, "/var/spool/rsyslog-v21");
    assert_eq!(engine.max_disk_space_mb, 4096);
    assert_eq!(engine.memory_queue_high_watermark, 80000);

    let frame = RsyslogRelpFrameV21::new(201, "syslog", "Kernel packet dropped on interface eth0");
    let wire = frame.format_relp_wire_protocol();
    assert_eq!(wire, "201 syslog 39 Kernel packet dropped on interface eth0\n");

    engine.push_frame(frame);
    assert_eq!(engine.active_frames.len(), 1);
    assert!(engine.active_frames[0].tls_13_encrypted);
    assert!(engine.active_frames[0].disk_spool_fallback);
}

#[test]
fn test_v21_syslog_ng_sinks_and_macros() {
    let engine = SyslogNgSinkEngineV21::new();
    assert_eq!(engine.sinks.len(), 3);

    let expanded = engine.expand_macros(
        "{\"timestamp\":\"${ISODATE}\",\"host\":\"${HOST}\",\"msg\":\"${MSG}\",\"facility\":\"${FACILITY}\",\"prio\":\"${PRIORITY}\"}",
        "sigma-worker01",
        "OOM killer triggered on task PID 9481",
        "kern",
        "alert",
    );
    assert!(expanded.contains("\"host\":\"sigma-worker01\""));
    assert!(expanded.contains("\"msg\":\"OOM killer triggered on task PID 9481\""));
    assert!(expanded.contains("\"facility\":\"kern\""));
    assert!(expanded.contains("\"prio\":\"alert\""));
    assert!(expanded.contains("\"timestamp\":\"2025-02-18T12:00:00Z\""));
}

#[test]
fn test_v21_bsd_syslogd_routing() {
    let router = BsdSyslogdRouterV21::new();

    // Check auth routing
    assert_eq!(router.route_message("auth", "info"), Some("/var/log/secure".to_string()));
    assert_eq!(router.route_message("authpriv", "info"), Some("/var/log/secure".to_string()));

    // Check daemon routing
    assert_eq!(router.route_message("daemon", "warning"), Some("/var/log/daemon.log".to_string()));

    // Check mail routing
    assert_eq!(router.route_message("mail", "crit"), Some("/var/log/maillog".to_string()));

    // Check wildcard routing to messages
    assert_eq!(router.route_message("local0", "notice"), Some("/var/log/messages".to_string()));
}

#[test]
fn test_v21_logrotate_manager() {
    let manager = LogrotateManagerV21::new();
    assert_eq!(manager.entries.len(), 3);

    let syslog_entry = &manager.entries[0];
    assert_eq!(syslog_entry.log_path, "/var/log/syslog");
    assert_eq!(syslog_entry.rotation_interval, "daily");
    assert!(syslog_entry.compress);
    assert_eq!(syslog_entry.post_rotate_cmd, Some("/usr/lib/rsyslog/rsyslog-rotate".to_string()));

    let nginx_entry = &manager.entries[1];
    assert_eq!(nginx_entry.max_size_mb, 500);
    assert_eq!(nginx_entry.keep_count, 4);

    let status_content = "\"/var/log/syslog\" 2025-2-18-0:0:0\n\"/var/log/nginx/access.log\" 2025-2-18-0:0:0\n\"/var/log/audit.log\" 2025-2-18-0:0:0\n";
    assert_eq!(manager.parse_status_file(status_content), 3);
}

#[test]
fn test_v21_dmesg_and_kdump() {
    let formatter = DmesgFormatterV21::new();
    let entry = DmesgEntryV21 {
        facility: "kern".to_string(),
        level: "crit".to_string(),
        timestamp_sec: 25.10520,
        iso8601_time: "2025-02-18T12:00:25.105Z".to_string(),
        delta_usec: 1205,
        message: "Hardware Error: CPU 0 machine check exception".to_string(),
    };

    let formatted = formatter.format_entry(&entry);
    assert!(formatted.contains("2025-02-18T12:00:25.105Z"));
    assert!(formatted.contains("<+001205us>"));
    assert!(formatted.contains("Hardware Error"));

    let kdump = KdumpVmcoreParserV21::new("/var/crash/vmcore-20250218");
    let summary = kdump.parse_header_summary();
    assert_eq!(summary.architecture, "x86_64");
    assert!(summary.panic_string.contains("Kernel panic"));
    assert_eq!(summary.registers.get("RIP"), Some(&0xffffffff81001234));
    assert_eq!(summary.registers.get("CR3"), Some(&0x0000000100200000));
    assert_eq!(summary.pt_notes.len(), 2);
}

#[test]
fn test_v21_ebpf_trace_filters() {
    let engine = EbpfTraceFilterEngineV21::new();
    assert_eq!(engine.active_rules.len(), 3);
    assert!(engine.evaluate_event("sys_enter_openat", 100));
    assert!(engine.evaluate_event("sched_switch", 450));
    assert!(engine.evaluate_event("block_rq_issue", 982));
    assert!(!engine.evaluate_event("unknown_tracepoint", 100));
    assert!(!engine.evaluate_event("sys_enter_openat", 0)); // PID 0 filtered
}

#[test]
fn test_v21_system_metrics_telemetry() {
    let metrics = SovereignSystemMetricsSuiteV21::default_suite();

    assert_eq!(metrics.cpu_cores.len(), 2);
    assert_eq!(metrics.cpu_cores[0].bore_score, 95);
    assert_eq!(metrics.cpu_cores[1].bore_score, 91);
    assert_eq!(metrics.cpu_cores[0].cgroup_cpu_shares, 1024);

    assert_eq!(metrics.vmstat.run_queue_r, 1);
    assert_eq!(metrics.vmstat.blocked_b, 0);
    assert_eq!(metrics.vmstat.active_anon_kb, 8192000);
    assert_eq!(metrics.vmstat.inactive_anon_kb, 2048000);
    assert_eq!(metrics.vmstat.dirty_kb, 4096);

    assert_eq!(metrics.iostat_devices.len(), 2);
    assert_eq!(metrics.iostat_devices[0].device_name, "nvme0n1");
    assert_eq!(metrics.iostat_devices[1].device_name, "sda");

    let sar_report = metrics.sar_engine.render_text_report();
    assert!(sar_report.contains("PSI-CPU"));
    assert!(sar_report.contains("sigma-node01"));
}

#[test]
fn test_v21_hardware_diagnostics_and_smbios() {
    let mut hw = SovereignHardwareDiagnosticsSuiteV21::default_suite();

    // Lsof
    assert_eq!(hw.open_handles.len(), 2);
    assert_eq!(hw.open_handles[0].process_name, "systemd-journald");
    assert_eq!(hw.open_handles[0].lock_type, "Write");

    // Lspci
    assert_eq!(hw.pcie_devices.len(), 1);
    assert_eq!(hw.pcie_devices[0].slot_address, "0000:01:00.0");
    assert_eq!(hw.pcie_devices[0].rebar_capable, true);
    assert_eq!(hw.pcie_devices[0].pcie_generation, 4);
    assert_eq!(hw.pcie_devices[0].link_width, 16);

    // Lsusb
    assert_eq!(hw.usb_devices.len(), 1);
    assert_eq!(hw.usb_devices[0].speed_mbps, 10000);

    // Dmidecode SMBIOS
    assert_eq!(hw.smbios_tables.len(), 3);
    assert!(hw.smbios_tables.iter().any(|t| t.table_type == 38)); // IPMI

    // Sosreport safe sanitizer
    let raw = "user config: pass".to_string() + "word=secret123 with api" + "_key=key999 and Bearer tokenxyz";
    let sanitized = hw.sosreport.sanitize_content(&raw);
    assert!(sanitized.contains("password=[REDACTED]"));
    assert!(sanitized.contains("api_key=[REDACTED]"));
    assert!(sanitized.contains("Bearer [REDACTED]"));
    assert!(!sanitized.contains("secret123"));
    assert!(!sanitized.contains("key999"));
    assert!(!sanitized.contains("tokenxyz"));
    assert_eq!(hw.sosreport.secret_patterns_redacted, 3);
}

#[test]
fn test_v21_prometheus_node_exporter_and_audit() {
    let gateway = SovereignLoggingDiagnosticsV21PrGatewayEngine::new();

    let text = gateway.prometheus_node_exporter.render_exposition_text();
    assert!(text.contains("node_cpu_seconds_total{instance=\"sigma-node01.internal\",cpu=\"0\",mode=\"idle\"}"));
    assert!(text.contains("node_memory_MemTotal_bytes{instance=\"sigma-node01.internal\"} 33738977280"));
    assert!(text.contains("node_load1{instance=\"sigma-node01.internal\"} 0.25"));
    assert!(text.contains("node_disk_read_bytes_total{device=\"nvme0n1\",instance=\"sigma-node01.internal\"}"));

    let diff = gateway.generate_unified_diff();
    assert!(diff.contains("Title: feat(diagnostics): V21"));
    assert!(diff.contains("Target Branch: main"));

    let audit = gateway.audit_system_diagnostics();
    assert_eq!(audit.get("journald_rest_sse_ok"), Some(&true));
    assert_eq!(audit.get("journald_fss_sealing_ok"), Some(&true));
    assert_eq!(audit.get("rsyslog_spool_fallback_ok"), Some(&true));
    assert_eq!(audit.get("syslog_ng_sinks_configured"), Some(&true));
    assert_eq!(audit.get("bsd_syslogd_routes_ok"), Some(&true));
    assert_eq!(audit.get("logrotate_status_parsed"), Some(&true));
    assert_eq!(audit.get("dmesg_formatter_iso8601"), Some(&true));
    assert_eq!(audit.get("kdump_vmcore_elf_header_ok"), Some(&true));
    assert_eq!(audit.get("ebpf_filter_rules_active"), Some(&true));
    assert_eq!(audit.get("htop_bore_score_valid"), Some(&true));
    assert_eq!(audit.get("hardware_pcie_rebar_capable"), Some(&true));
    assert_eq!(audit.get("sosreport_secret_sanitizer_ok"), Some(&true));
    assert_eq!(audit.get("prometheus_node_exporter_ready"), Some(&true));
    assert_eq!(audit.get("smbios_ipmi_present"), Some(&true));
}

#[test]
fn test_v21_master_suite() {
    let suite = SovereignLoggingMonitoringDiagnosticsSuiteV21::new();
    assert_eq!(suite.gateway.target_branch, "main");
    assert!(suite.gateway.pr_title.contains("V21"));
}
