// Integration Test Suite for Sovereign Printing & Scanning Advancements V88 PR
// Testing all 12 sub-engines and end-to-end pull request gateway audit

#[path = "../src/printing/sovereign_printing_scanning_v88_pr.rs"]
mod sovereign_printing_scanning_v88_pr;

use sovereign_printing_scanning_v88_pr::*;

#[test]
fn test_integration_cups3_driverless_gateway_v88() {
    let mut gw = SovereignCups3DriverlessAppGatewayEngineV88::default();
    assert_eq!(gw.service_mode, SovereignCups3ServiceModeV88::LocalServer);
    assert_eq!(gw.ipp_version, "2.0");

    gw.register_printer_app("canon-mf-app", "ipp://192.168.1.50:8631/ipp/print/canon-mf-app");
    assert_eq!(gw.registered_apps.len(), 2);

    let attrs = gw.convert_ppd_to_ipp_attributes(&[
        ("PageSize", "A4"),
        ("Resolution", "600dpi"),
        ("Duplex", "DuplexNoTumble"),
    ]);
    assert_eq!(attrs.get("ipp-ppd-pagesize").unwrap(), "A4");
    assert_eq!(attrs.get("ipp-ppd-resolution").unwrap(), "600dpi");
    assert_eq!(attrs.get("ipp-ppd-duplex").unwrap(), "DuplexNoTumble");

    let txt_records = gw.generate_dns_sd_txt_records("OfficeMFP");
    assert!(txt_records.iter().any(|(k, v)| k == "rp" && v == "printers/OfficeMFP"));
    assert!(txt_records.iter().any(|(k, v)| k == "note" && v.contains("V88")));

    let routed = gw.route_ipp_operation("Get-Printer-Attributes", "ipp://localhost/ipp/print/generic-ipp-app-v88").unwrap();
    assert!(routed.contains("CUPS3-GATEWAY-V88"));
    assert!(routed.contains("routed to 'ipp://localhost/ipp/print/generic-ipp-app-v88'"));
}

#[test]
fn test_integration_cups_filter_v2_graph_v88() {
    let graph = SovereignCupsFilterV2GraphEngineV88::default();
    assert!(!graph.nodes.is_empty());

    let pipeline = graph.resolve_pipeline("text/plain", "image/pwg-raster");
    assert!(pipeline.is_ok());
    let path = pipeline.unwrap();
    assert_eq!(path, vec!["texttopdf".to_string(), "pdftoraster".to_string()]);

    let output = graph.execute_pipeline("text/plain", "image/pwg-raster", b"Hello SigmaOS");
    assert!(output.is_ok());
    let bytes = output.unwrap();
    let exec_str = String::from_utf8_lossy(&bytes);
    assert!(exec_str.contains("%%CUPS-FILTER-V88-EXEC"));
    assert!(exec_str.contains("texttopdf"));
}

#[test]
fn test_integration_ghostscript_distiller_v88() {
    let distiller = SovereignGhostscriptDistillerEngineV88::default();
    assert_eq!(distiller.target_pdfa_part, 3);

    let ps = b"%!PS-Adobe-3.0 Content";
    let transpiled = distiller.transpile_ps_to_pdfa(ps).unwrap();
    assert!(transpiled.starts_with(b"%%PDF/A-3b V88 Transpiled Stream"));

    assert!(distiller.validate_pdfa_compliance(b"%PDF-1.7 Valid Stream").unwrap());
    assert!(distiller.validate_pdfa_compliance(b"INVALID RAW PDF").is_err());
    assert!(distiller.validate_pdfa_compliance(b"").is_err());

    assert_eq!(distiller.substitute_font("Helvetica"), "Nimbus Sans L");
    assert_eq!(distiller.substitute_font("Times-Roman"), "Nimbus Roman No9 L");
    assert_eq!(distiller.substitute_font("Ryumin-Light"), "Kochi-Mincho");
    assert_eq!(distiller.substitute_font("CustomFont"), "FreeMono");
}

#[test]
fn test_integration_ghostscript_icc_tic_v88() {
    let icc = SovereignGhostscriptIccTicEngineV88::default();
    assert_eq!(icc.tic_limit_percent, 300.0);

    let (c, m, y, k) = icc.rgb_to_cmyk(255, 0, 0);
    assert!((c - 0.0).abs() < 1e-4);
    assert!((m - 1.0).abs() < 1e-4);
    assert!((y - 1.0).abs() < 1e-4);
    assert!((k - 0.0).abs() < 1e-4);

    let tic_red = icc.calculate_tic((c, m, y, k));
    assert!((tic_red - 200.0).abs() < 1e-4);

    let (c2, m2, y2, k2) = (0.8, 0.8, 0.8, 0.8);
    let tic_heavy = icc.calculate_tic((c2, m2, y2, k2));
    assert!((tic_heavy - 320.0).abs() < 1e-4);

    let enforced = icc.enforce_tic_limit((c2, m2, y2, k2));
    let enforced_tic = icc.calculate_tic(enforced);
    assert!(enforced_tic <= 300.0);
}

#[test]
fn test_integration_poppler_acroform_and_pqc_v88() {
    let acro = SovereignPopplerAcroFormEngineV88::default();
    assert!(acro.cairo_subpixel);
    assert!(acro.pqc_verification);

    let pdf_bytes = b"%PDF-1.7 AcroForm document";
    let fields = acro.extract_acroform_fields(pdf_bytes);
    assert_eq!(fields.len(), 3);
    assert!(fields.iter().any(|(name, _, val)| name == "Signature" && val == "PQC-Dilithium5-Verified"));

    assert!(acro.verify_pqc_signature(pdf_bytes).unwrap());
    assert!(acro.verify_pqc_signature(b"").is_err());

    let svg = acro.render_svg_page(1);
    assert!(svg.contains("<svg"));
    assert!(svg.contains("Sovereign Poppler V88 Render Page 1"));
}

#[test]
fn test_integration_poppler_pdf_ua_search_v88() {
    let search = SovereignPopplerPdfUaSearchEngineV88::default();
    assert!(search.enforce_iso14289);

    let pdf = b"%PDF-1.7 Sovereign Universal Print Scan Specification V88 Architecture";
    let hits = search.search_text_bbox(pdf, "Sovereign");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].word, "Sovereign");
    assert_eq!(hits[0].page_num, 1);

    let empty_hits = search.search_text_bbox(pdf, "");
    assert!(empty_hits.is_empty());

    let highlight_svg = search.generate_highlight_svg(&hits);
    assert!(highlight_svg.contains("<rect"));
    assert!(highlight_svg.contains("fill=\"yellow\""));
    assert!(highlight_svg.contains("opacity=\"0.4\""));
}

#[test]
fn test_integration_sane_net_multiplexer_v88() {
    let mut sane = SovereignSaneNetMultiplexerEngineV88::default();
    assert_eq!(sane.protocol_version, 2);

    let dev = sane.sane_open("genesys");
    assert!(dev.is_ok());
    let handle = dev.unwrap();
    assert!(handle.contains("SANE-V88-HANDLE-001"));
    assert!(sane.is_open);

    let start = sane.sane_start();
    assert!(start.is_ok());
    let (bpl, ppl, lines) = start.unwrap();
    assert_eq!((bpl, ppl, lines), (2400, 800, 1000));

    let data = sane.sane_read(1024);
    assert_eq!(data.len(), 1024);

    sane.sane_close();
    assert!(!sane.is_open);
}

#[test]
fn test_integration_airscan_wsd_v88() {
    let mut airscan = SovereignAirScanWsdEngineV88::default();
    assert_eq!(airscan.scanner_ip, "192.168.1.100");
    assert_eq!(airscan.port, 8080);

    let soap = airscan.generate_wsd_probe_soap();
    assert!(soap.contains("s:Envelope"));
    assert!(soap.contains("Probe"));

    let escl_caps = airscan.generate_escl_capabilities_xml();
    assert!(escl_caps.contains("scan:ScannerCapabilities"));
    assert!(escl_caps.contains("Sovereign AirScan V88"));

    let job_uri = airscan.create_escl_job(600);
    assert!(job_uri.contains("eSCL/ScanJobs/v88-job-600"));
}

#[test]
fn test_integration_adf_batch_ocr_v88() {
    let adf = SovereignAdfBatchOcrEngineV88::default();
    assert_eq!(adf.ultrasonic_threshold_db, -18.5);

    assert!(adf.detect_double_feed(-25.0));
    assert!(!adf.detect_double_feed(-10.0));

    let angle = adf.calculate_deskew_angle(b"HORIZONTAL TEXT LINE DATA");
    assert!((angle - 0.35).abs() < 1e-4);

    let processed = adf.process_batch_page(b"PAGE SCAN CONTENT", 3);
    assert!(processed.contains("PAGE 3: Deskew=0.35deg, OCR='Sovereign Document V88 Page 3'"));
}

#[test]
fn test_integration_sane_image_processing_v88() {
    let img = SovereignSaneImageProcessingEngineV88::default();
    assert_eq!(img.lut_size, 256);

    let gamma_table = img.generate_16bit_gamma_table(2.2);
    assert_eq!(gamma_table.len(), 256);
    assert_eq!(gamma_table[0], 0);
    assert_eq!(gamma_table[255], 65535);

    let noisy_pixels = [10, 50, 248, 251, 100];
    let cleaned = img.despeckle_background(&noisy_pixels);
    assert_eq!(cleaned[2], 255);
    assert_eq!(cleaned[3], 255);
    assert_eq!(cleaned[0], 10);
}

#[test]
fn test_integration_bsd_lpd_accounting_v88() {
    let mut lpd = SovereignBsdLpdAccountingEngineV88::default();
    assert_eq!(lpd.queue_name, "lp");
    assert_eq!(lpd.spool_directory, "/var/spool/lpd/lp");

    assert!(lpd.check_quota("alice", 50));
    assert!(!lpd.check_quota("alice", 150));
    // User without explicit quota defaults to true (unlimited)
    assert!(lpd.check_quota("bob", 10));

    lpd.user_quotas.insert("carol".to_string(), 100);
    assert!(lpd.check_quota("carol", 50));
    assert!(!lpd.check_quota("carol", 150));

    let ctrl_file = lpd.format_lpr_control_file(42, "workstation01", "alice");
    assert!(ctrl_file.contains("Hworkstation01\n"));
    assert!(ctrl_file.contains("Palice\n"));
    assert!(ctrl_file.contains("JPrint Job #42\n"));
    assert!(ctrl_file.contains("UdfA042workstation01\n"));
}

#[test]
fn test_integration_print_scan_pr_gateway_v88() {
    let gw = SovereignPrintScanV88PrGatewayEngine::default();
    assert_eq!(gw.pr_id, 88001);
    assert!(gw.title.contains("V88 Innovations"));
    assert_eq!(gw.author, "Sovereign Core Team");

    let audit = gw.run_12_point_diagnostic_audit();
    assert_eq!(audit.len(), 12);
    for (name, passed) in &audit {
        assert!(passed, "Audit checkpoint '{}' failed!", name);
    }

    let diff = gw.generate_unified_diff();
    assert!(diff.contains("PQC-SIGNATURE: DILITHIUM5-OK"));
    assert!(diff.contains("Sovereign Printing & Scanning V88 Innovations"));

    let manifest = gw.generate_pr_manifest();
    assert!(manifest.contains("PR-MANIFEST-V88: PR #88001"));
    assert!(manifest.contains("ALL 12 TESTS PASSED"));
}
