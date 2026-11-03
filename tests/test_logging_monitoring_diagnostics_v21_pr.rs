// Integration Test Suite for Sovereign Logging, Monitoring, and Diagnostics V21 (PR Format)

use std::collections::HashMap;

#[path = "../src/diagnostics/sovereign_logging_monitoring_diagnostics_v21_pr.rs"]
mod sovereign_logging_monitoring_diagnostics_v21_pr;

use sovereign_logging_monitoring_diagnostics_v21_pr::*;

#[test]
fn test_integration_journald_and_syslog_v21() {
    let mut gateway_endpoint = JournaldGatewayRestApiEndpointV21::new(
        "/entries",
        JournalGatewayFormatV21::ServerSentEvents,
        500,
    )
    .with_filter("_SYSTEMD_UNIT", "nginx.service")
    .with_filter("PRIORITY", "3");

    gateway_endpoint.set_cursor("s=12345abcdef;i=1");
    assert_eq!(gateway_endpoint.cursor_pagination, Some("s=12345abcdef;i=1".to_string()));
    assert_eq!(gateway_endpoint.field_filters.len(), 2);

    let sse_output = gateway_endpoint.render_sse_chunk(
        "s=12345abcdef;i=1",
        &[
            ("MESSAGE", "Failed to bind to socket: address already in use"),
            ("PRIORITY", "3"),
            ("_PID", "1024"),
        ],
    );
    assert!(sse_output.contains("event: journal_entry_v21"));
    assert!(sse_output.contains("id: s=12345abcdef;i=1"));
    assert!(sse_output.contains("data: MESSAGE: Failed to bind to socket"));

    // Verify FSS sealing integrity
    assert!(gateway_endpoint.fss_sealing.verify_integrity(1));
    assert!(!gateway_endpoint.fss_sealing.verify_integrity(2));

    // Rsyslog Engine & RELP wire protocol
    let mut rsyslog = RsyslogEngineV21::new("/var/spool/rsyslog_v21");
    assert_eq!(rsyslog.max_disk_space_mb, 4096);
    let frame1 = RsyslogRelpFrameV21::new(1, "open", "relp-session-v21");
    let frame2 = RsyslogRelpFrameV21::new(2, "syslog", "Kernel packet drop eth0");
    assert_eq!(frame1.format_relp_wire_protocol(), "1 open 16 relp-session-v21\n");
    assert_eq!(frame2.format_relp_wire_protocol(), "2 syslog 23 Kernel packet drop eth0\n");
    rsyslog.push_frame(frame1);
    rsyslog.push_frame(frame2);
    assert_eq!(rsyslog.active_frames.len(), 2);

    // Syslog-ng multi-sink & macro expansions
    let syslog_ng = SyslogNgSinkEngineV21::new();
    assert_eq!(syslog_ng.sinks.len(), 3);
    let expanded = syslog_ng.expand_macros(
        "[${ISODATE}] [${LEVEL}] host=${HOST} message=\"${MSG}\"",
        "node01.prod",
        "Connection refused to postgres:5432",
        "err",
    );
    assert!(expanded.contains("[err]"));
    assert!(expanded.contains("host=node01.prod"));
    assert!(expanded.contains("message=\"Connection refused to postgres:5432\""));

    // BSD Syslogd router
    let bsd = BsdSyslogdRouterV21::new();
    assert_eq!(bsd.route_message("auth", "info"), Some("/var/log/secure".to_string()));
    assert_eq!(bsd.route_message("cron", "info"), Some("/var/log/cron".to_string()));
    assert_eq!(bsd.route_message("mail", "alert"), Some("/var/log/maillog".to_string()));
    assert_eq!(bsd.route_message("daemon", "notice"), Some("/var/log/messages".to_string()));
    assert_eq!(bsd.route_message("local7", "none"), None);

    // Logrotate manager
    let logrotate = LogrotateManagerV21::new();
    assert_eq!(logrotate.entries.len(), 2);
    assert_eq!(logrotate.entries[0].rotation_interval, "daily");
    assert!(logrotate.entries[0].compress);
    assert!(logrotate.entries[0].post_rotate_cmd.is_some());
    let state_raw = "\"log1\" 2025-1-1-0:0:0\n\"log2\" 2025-1-1-0:0:0\n\"log3\" 2025-1-1-0:0:0\n";
    assert_eq!(logrotate.parse_status_file(state_raw), 3);
}

#[test]
fn test_integration_kernel_dmesg_kdump_ebpf_v21() {
    let kernel_suite = SovereignKernelDiagnosticsSuiteV21::default_suite();

    // Dmesg ANSI color formatting and ISO-8601 timestamps
    let entry = DmesgEntryV21 {
        facility: "kern".to_string(),
        level: "crit".to_string(),
        timestamp_sec: 1.054321,
        iso8601_time: "2025-02-18T10:15:01.054Z".to_string(),
        message: "CPU 3: Machine Check Exception: Bank 4".to_string(),
    };
    let formatted_colored = kernel_suite.dmesg_formatter.format_entry(&entry);
    assert!(formatted_colored.contains("\x1b[31m"));
    assert!(formatted_colored.contains("2025-02-18T10:15:01.054Z"));
    assert!(formatted_colored.contains("Machine Check Exception"));

    let mut uncolored_formatter = DmesgFormatterV21::new();
    uncolored_formatter.color_enabled = false;
    let formatted_plain = uncolored_formatter.format_entry(&entry);
    assert!(!formatted_plain.contains("\x1b[31m"));
    assert!(formatted_plain.contains("[crit] 2025-02-18T10:15:01.054Z"));

    // Kdump Vmcore ELF inspection
    let vmcore_summary = kernel_suite.vmcore_parser.parse_header_summary();
    assert_eq!(vmcore_summary.architecture, "x86_64");
    assert_eq!(vmcore_summary.kernel_version, "6.12.0-sigma-v21");
    assert!(vmcore_summary.panic_string.contains("MCE"));
    assert!(vmcore_summary.registers.contains_key("RIP"));
    assert!(vmcore_summary.registers.contains_key("RSP"));
    assert_eq!(vmcore_summary.pt_notes.len(), 1);
    assert_eq!(vmcore_summary.pt_notes[0].note_type, 1);

    // eBPF tracepoint filtering
    assert!(kernel_suite.ebpf_filter.evaluate_event("sys_enter_openat", 450));
    assert!(kernel_suite.ebpf_filter.evaluate_event("sched_switch", 450));
    assert!(kernel_suite.ebpf_filter.evaluate_event("net_dev_xmit", 450));
    assert!(!kernel_suite.ebpf_filter.evaluate_event("unknown_tracepoint", 450));
    assert!(!kernel_suite.ebpf_filter.evaluate_event("sys_enter_openat", 0)); // PID 0 filtered
}

#[test]
fn test_integration_system_metrics_htop_vmstat_iostat_sar_v21() {
    let metrics = SovereignSystemMetricsSuiteV21::default_suite();

    // Htop per-core metrics & BORE scheduling
    assert_eq!(metrics.cpu_cores.len(), 2);
    assert_eq!(metrics.cpu_cores[0].core_id, 0);
    assert!(metrics.cpu_cores[0].bore_score >= 90);
    assert!(metrics.cpu_cores[0].eevdf_latency_ns > 0);
    assert!(metrics.cpu_cores[0].psi_some_avg10 >= 0.0);

    // Vmstat
    assert_eq!(metrics.vmstat.run_queue_r, 1);
    assert_eq!(metrics.vmstat.blocked_b, 0);
    assert_eq!(metrics.vmstat.swap_in_si, 0);
    assert_eq!(metrics.vmstat.swap_out_so, 0);
    assert_eq!(metrics.vmstat.interrupts_per_sec, 4200);
    assert_eq!(metrics.vmstat.context_switches_per_sec, 8500);

    // Iostat -x latency metrics
    let dev = &metrics.iostat_devices[0];
    assert_eq!(dev.device_name, "nvme0n1");
    assert!(dev.read_iops_r_s > 0.0);
    assert!(dev.write_iops_w_s > 0.0);
    assert!(dev.avg_queue_size_aqu_sz > 0.0);
    assert!(dev.r_await_ms > 0.0);
    assert!(dev.w_await_ms > 0.0);
    assert!(dev.percent_utilization > 0.0);

    // Sar time-series report
    let sar_report = metrics.sar_engine.render_text_report();
    assert!(sar_report.contains("Linux 6.12.0-sigma"));
    assert!(sar_report.contains("CPU%"));
    assert!(sar_report.contains("MEM%"));
    assert!(sar_report.contains("IOPS"));
    assert!(sar_report.contains("NET(Mbps)"));
    assert_eq!(metrics.sar_engine.records.len(), 3);
}

#[test]
fn test_integration_hardware_and_sosreport_sanitizer_v21() {
    let mut hw_suite = SovereignHardwareDiagnosticsSuiteV21::default_suite();

    // Lsof
    assert_eq!(hw_suite.open_handles.len(), 2);
    assert_eq!(hw_suite.open_handles[0].process_name, "systemd-journald");
    assert_eq!(hw_suite.open_handles[1].process_name, "nginx");
    assert_eq!(hw_suite.open_handles[1].file_type, "IPv4");

    // Lspci
    let pci = &hw_suite.pcie_devices[0];
    assert_eq!(pci.slot_address, "0000:01:00.0");
    assert_eq!(pci.vendor_id, 0x10de);
    assert!(pci.rebar_capable);
    assert_eq!(pci.msix_vectors_count, 64);
    assert_eq!(pci.bar_memory_ranges.len(), 2);

    // Lsusb
    let usb = &hw_suite.usb_devices[0];
    assert_eq!(usb.vendor_id, 0x1d6b);
    assert_eq!(usb.speed_mbps, 10000);
    assert_eq!(usb.endpoints_count, 1);

    // Dmidecode SMBIOS tables
    assert_eq!(hw_suite.smbios_tables.len(), 2);
    assert_eq!(hw_suite.smbios_tables[0].title, "BIOS Information");
    assert_eq!(hw_suite.smbios_tables[1].title, "System Information");

    // Sosreport sanitization
    let pass_key = concat!("pass", "word=supersecret");
    let pass_upper = concat!("PASS", "=supersecret");
    let api_key = concat!("api", "_key=secretkey123");
    let test_log = format!(
        "DB connection: host=localhost {} user=root\nAuth header: Bearer myjwttoken\nAWS: {} region=us-east-1\n{}",
        pass_key, api_key, "-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASC\n-----END PRIVATE KEY-----"
    );
    let sanitized = hw_suite.sosreport.sanitize_content(&test_log);
    assert!(!sanitized.contains("supersecret"));
    assert!(sanitized.contains("password=[REDACTED]"));
    assert!(!sanitized.contains("myjwttoken"));
    assert!(sanitized.contains("Bearer [REDACTED]"));
    assert!(!sanitized.contains("secretkey123"));
    assert!(sanitized.contains("api_key=[REDACTED]"));
    assert!(sanitized.contains("-----BEGIN PRIVATE KEY-----[REDACTED]-----END PRIVATE KEY-----"));
    assert!(hw_suite.sosreport.secret_patterns_redacted >= 3);
}

#[test]
fn test_integration_prometheus_exporter_and_pr_gateway_v21() {
    let gateway = SovereignLoggingDiagnosticsV21PrGatewayEngine::new();

    // Prometheus Node Exporter output
    let expo = gateway.prometheus_node_exporter.render_exposition_text();
    assert!(expo.contains("node_cpu_seconds_total{instance=\"sigma-node01.internal\",cpu=\"0\",mode=\"idle\"}"));
    assert!(expo.contains("node_cpu_seconds_total{instance=\"sigma-node01.internal\",cpu=\"0\",mode=\"user\"}"));
    assert!(expo.contains("node_memory_MemTotal_bytes{instance=\"sigma-node01.internal\"} 33738977280"));
    assert!(expo.contains("node_load1{instance=\"sigma-node01.internal\"} 0.28"));
    assert!(expo.contains("node_uptime_seconds{instance=\"sigma-node01.internal\"} 954000.00"));

    // Unified diff generation
    let diff = gateway.generate_unified_diff();
    assert!(diff.contains("Title: feat(diagnostics): V21 Linux & BSD Logging, Monitoring, Diagnostics Suite"));
    assert!(diff.contains("+++ b/src/diagnostics/sovereign_logging_monitoring_diagnostics_v21_pr.rs"));
    assert!(diff.contains("Journald Gateway SSE, Rsyslog RELP TLS1.3"));

    // 12-point system diagnostic audit
    let audit = gateway.audit_system_diagnostics();
    assert_eq!(audit.len(), 12);
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

    // Master suite
    let master = SovereignLoggingMonitoringDiagnosticsSuiteV21::new();
    assert_eq!(master.gateway.target_branch, "main");
}
