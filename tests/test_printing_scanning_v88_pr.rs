// Integration Test Suite for Sovereign Printing & Scanning Advancements (v88 PR Suite)

extern crate alloc;

#[path = "../src/printing/sovereign_printing_scanning_v88_pr.rs"]
mod sovereign_printing_scanning_v88_pr;

use sovereign_printing_scanning_v88_pr::*;

#[test]
fn test_integration_printing_scanning_v88_suite() {
    let gateway = SovereignPrintScanV88PrGatewayEngine::default();
    let audit = gateway.run_12_point_diagnostic_audit();

    assert_eq!(audit.len(), 12);
    for (name, passed) in &audit {
        assert!(passed, "Audit failed for: {}", name);
    }

    let diff = gateway.generate_unified_diff();
    assert!(diff.contains("PQC-SIGNATURE: DILITHIUM5-OK"));
    assert!(diff.contains("PR #88001"));

    let manifest = gateway.generate_pr_manifest();
    assert!(manifest.contains("ALL 12 TESTS PASSED"));
    assert!(manifest.contains("PR-MANIFEST-V88"));
}

#[test]
fn test_integration_cups_subsystem_v88() {
    let mut cups_gw = SovereignCups3DriverlessAppGatewayEngineV88::new(
        SovereignCups3ServiceModeV88::PrinterApplication,
    );
    cups_gw.register_printer_app("laserjet-m600", "ipp://192.168.1.50:631/ipp/print");
    let txt_records = cups_gw.generate_dns_sd_txt_records("laserjet-m600");
    assert!(txt_records.iter().any(|(k, v)| k == "note" && v.contains("V88")));

    let ppd_opts = vec![("PageSize", "A4"), ("Duplex", "DuplexNoTumble")];
    let attrs = cups_gw.convert_ppd_to_ipp_attributes(&ppd_opts);
    assert_eq!(attrs.get("ipp-ppd-pagesize").unwrap(), "A4");
    assert_eq!(attrs.get("printer-is-accepting-jobs").unwrap(), "true");

    let route = cups_gw.route_ipp_operation("Print-Job", "ipp://192.168.1.50:631/ipp/print").unwrap();
    assert!(route.contains("Print-Job"));
    assert!(route.contains("PrinterApplication"));

    let mut filter_graph = SovereignCupsFilterV2GraphEngineV88::default();
    filter_graph.register_node(
        "application/pdf",
        "application/vnd.hp-PCLm",
        "pdftopclm",
        25,
    );
    let plan = filter_graph.resolve_pipeline("application/pdf", "application/vnd.hp-PCLm").unwrap();
    assert_eq!(plan, vec!["pdftopclm"]);
}

#[test]
fn test_integration_ghostscript_subsystem_v88() {
    let gs = SovereignGhostscriptDistillerEngineV88::default();
    assert!(gs.validate_pdfa_compliance(b"%PDF-1.7 Document").unwrap());
    assert_eq!(gs.substitute_font("Helvetica"), "Nimbus Sans L");
    let pdfa = gs.transpile_ps_to_pdfa(b"%!PS-Adobe-3.0 Content").unwrap();
    assert!(pdfa.starts_with(b"%%PDF/A-3b V88 Transpiled Stream"));

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
fn test_integration_poppler_subsystem_v88() {
    let pop = SovereignPopplerAcroFormEngineV88::default();
    let fields = pop.extract_acroform_fields(b"%PDF-1.7 Form");
    assert_eq!(fields.len(), 3);
    assert!(pop.verify_pqc_signature(b"%PDF-1.7").unwrap());

    let svg = pop.render_svg_page(1);
    assert!(svg.contains("<svg"));

    let search = SovereignPopplerPdfUaSearchEngineV88::default();
    let hits = search.search_text_bbox(b"%PDF-1.7", "Sovereign");
    assert_eq!(hits.len(), 1);

    let highlight_svg = search.generate_highlight_svg(&hits);
    assert!(highlight_svg.contains("<rect"));
}

#[test]
fn test_integration_sane_and_adf_subsystem_v88() {
    let mut sane_net = SovereignSaneNetMultiplexerEngineV88::default();
    let handle = sane_net.sane_open("genesys").unwrap();
    assert!(handle.contains("genesys"));
    let (bpl, ppl, lines) = sane_net.sane_start().unwrap();
    assert_eq!((bpl, ppl, lines), (2400, 800, 1000));
    let data = sane_net.sane_read(2048);
    assert_eq!(data.len(), 2048);
    sane_net.sane_close();

    let mut airscan = SovereignAirScanWsdEngineV88::default();
    let soap = airscan.generate_wsd_probe_soap();
    assert!(soap.contains("Probe"));
    let caps = airscan.generate_escl_capabilities_xml();
    assert!(caps.contains("192.168.1.100"));
    let job_uri = airscan.create_escl_job(600);
    assert!(job_uri.contains("eSCL/ScanJobs/v88-job-600"));

    let adf = SovereignAdfBatchOcrEngineV88::default();
    assert!(adf.detect_double_feed(-22.0));
    assert!(!adf.detect_double_feed(-10.0));
    let batch_res = adf.process_batch_page(b"PAGE_DATA", 2);
    assert!(batch_res.contains("Deskew=0.35deg"));

    let img = SovereignSaneImageProcessingEngineV88::default();
    let lut = img.generate_16bit_gamma_table(2.2);
    assert_eq!(lut.len(), 256);
    let desp = img.despeckle_background(&[100, 200, 250]);
    assert_eq!(desp[2], 255);

    let mut lpd = SovereignBsdLpdAccountingEngineV88::default();
    lpd.user_quotas.insert("bob".to_string(), 50);
    assert!(lpd.check_quota("bob", 20));
    assert!(!lpd.check_quota("bob", 200));
    let ctrl = lpd.format_lpr_control_file(42, "daemonhost", "bob");
    assert!(ctrl.contains("Hdaemonhost"));
    assert!(ctrl.contains("Pbob"));
}
