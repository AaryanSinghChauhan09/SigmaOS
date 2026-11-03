// SPDX-License-Identifier: MIT
//! Integration Test Suite for Sovereign Printing & Scanning Subsystem Innovations V88 PR Engine
//! (`tests/test_printing_scanning_v88_pr.rs`)

#[path = "../src/printing/sovereign_printing_scanning_v88_pr.rs"]
mod sovereign_printing_scanning_v88_pr;

use sovereign_printing_scanning_v88_pr::*;

#[test]
fn test_integration_cups3_driverless_gateway_v88() {
    let mut gw = SovereignCups3DriverlessAppGatewayEngineV88::default();
    gw.register_printer_app("epson-app-v88", "ipp://localhost:8631/ipp/print/epson-app-v88");
    assert_eq!(gw.registered_apps.len(), 2);

    let attrs = gw.convert_ppd_to_ipp_attributes(&[("PageSize", "A4"), ("Duplex", "None")]);
    assert_eq!(attrs.get("ipp-ppd-pagesize").unwrap(), "A4");

    let txt = gw.generate_dns_sd_txt_records("Office-Printer-v88");
    assert!(txt.iter().any(|(k, v)| k == "rp" && v == "printers/Office-Printer-v88"));

    let route = gw.route_ipp_operation("Print-Job", "ipp://localhost/ipp/print").unwrap();
    assert!(route.contains("Print-Job"));
}

#[test]
fn test_integration_cups_filter_v2_graph_v88() {
    let graph = SovereignCupsFilterV2GraphEngineV88::default();
    let pipeline = graph.resolve_pipeline("text/plain", "image/pwg-raster").unwrap();
    assert_eq!(pipeline, vec!["texttopdf".to_string(), "pdftoraster".to_string()]);

    let exec = graph.execute_pipeline("text/plain", "image/pwg-raster", b"Hello V88").unwrap();
    let exec_str = String::from_utf8_lossy(&exec);
    assert!(exec_str.contains("texttopdf"));
}

#[test]
fn test_integration_ghostscript_distiller_v88() {
    let gs = SovereignGhostscriptDistillerEngineV88::default();
    assert!(gs.validate_pdfa_compliance(b"%PDF-1.7 Document").unwrap());
    assert_eq!(gs.substitute_font("Helvetica"), "Nimbus Sans L");

    let pdfa = gs.transpile_ps_to_pdfa(b"%!PS-Adobe-3.0 Content").unwrap();
    assert!(pdfa.starts_with(b"%%PDF/A-3b V88 Transpiled Stream"));
}

#[test]
fn test_integration_ghostscript_icc_tic_v88() {
    let icc = SovereignGhostscriptIccTicEngineV88::default();
    let (c, m, _y, _k) = icc.rgb_to_cmyk(255, 0, 0);
    assert!((c - 0.0).abs() < 1e-4);
    assert!((m - 1.0).abs() < 1e-4);

    let tic = icc.calculate_tic((0.8, 0.8, 0.8, 0.8));
    assert!((tic - 320.0).abs() < 1e-4);

    let adj = icc.enforce_tic_limit((0.8, 0.8, 0.8, 0.8));
    assert!(icc.calculate_tic(adj) <= 300.0);
}

#[test]
fn test_integration_poppler_acroform_v88() {
    let pop = SovereignPopplerAcroFormEngineV88::default();
    let fields = pop.extract_acroform_fields(b"%PDF-1.7 Form");
    assert_eq!(fields.len(), 3);
    assert!(pop.verify_pqc_signature(b"%PDF-1.7").unwrap());

    let svg = pop.render_svg_page(1);
    assert!(svg.contains("<svg"));
}

#[test]
fn test_integration_poppler_pdf_ua_search_v88() {
    let search = SovereignPopplerPdfUaSearchEngineV88::default();
    let hits = search.search_text_bbox(b"%PDF-1.7", "Sovereign");
    assert_eq!(hits.len(), 1);

    let svg = search.generate_highlight_svg(&hits);
    assert!(svg.contains("<rect"));
}

#[test]
fn test_integration_sane_net_multiplexer_v88() {
    let mut mux = SovereignSaneNetMultiplexerEngineV88::default();
    let hnd = mux.sane_open("genesys").unwrap();
    assert!(hnd.contains("SANE-V88-HANDLE-001"));

    let (bpl, ppl, lines) = mux.sane_start().unwrap();
    assert_eq!((bpl, ppl, lines), (2400, 800, 1000));

    let data = mux.sane_read(1024);
    assert_eq!(data.len(), 1024);

    mux.sane_close();
    assert!(!mux.is_open);
}

#[test]
fn test_integration_airscan_wsd_v88() {
    let mut airscan = SovereignAirScanWsdEngineV88::default();
    let soap = airscan.generate_wsd_probe_soap();
    assert!(soap.contains("Probe"));

    let caps = airscan.generate_escl_capabilities_xml();
    assert!(caps.contains("192.168.1.100"));

    let job_uri = airscan.create_escl_job(300);
    assert!(job_uri.contains("eSCL/ScanJobs/v88-job-300"));
}

#[test]
fn test_integration_adf_batch_ocr_v88() {
    let adf = SovereignAdfBatchOcrEngineV88::default();
    assert!(adf.detect_double_feed(-22.0));
    assert!(!adf.detect_double_feed(-10.0));

    let res = adf.process_batch_page(b"PAGE DATA", 1);
    assert!(res.contains("Deskew=0.35deg"));
}

#[test]
fn test_integration_sane_image_processing_v88() {
    let img = SovereignSaneImageProcessingEngineV88::default();
    let lut = img.generate_16bit_gamma_table(2.2);
    assert_eq!(lut.len(), 256);

    let desp = img.despeckle_background(&[100, 200, 250]);
    assert_eq!(desp[2], 255);
}

#[test]
fn test_integration_bsd_lpd_accounting_v88() {
    let lpd = SovereignBsdLpdAccountingEngineV88::default();
    assert!(lpd.check_quota("alice", 50));
    assert!(!lpd.check_quota("alice", 200));

    let ctrl = lpd.format_lpr_control_file(101, "myhost", "alice");
    assert!(ctrl.contains("Hmyhost"));
    assert!(ctrl.contains("Palice"));
}

#[test]
fn test_integration_print_scan_pr_gateway_v88() {
    let gw = SovereignPrintScanV88PrGatewayEngine::default();
    let audit = gw.run_12_point_diagnostic_audit();
    assert_eq!(audit.len(), 12);
    assert!(audit.iter().all(|(_, ok)| *ok));

    let diff = gw.generate_unified_diff();
    assert!(diff.contains("PQC-SIGNATURE: DILITHIUM5-OK"));

    let manifest = gw.generate_pr_manifest();
    assert!(manifest.contains("ALL 12 TESTS PASSED"));
}
