// SPDX-License-Identifier: MIT
// Test Suite for Pdfcraft ArtCraft Missing Components V73 PR Engine & Integration

#[path = "../src/creative/pdfcraft_artcraft_missing_components_v73_pr.rs"]
mod pdfcraft_artcraft_missing_components_v73_pr;

use pdfcraft_artcraft_missing_components_v73_pr::*;

#[test]
fn test_v73_pdfcraft_individual_engines() {
    let mut neural_layout = PdfcraftAiNeuralLayoutInferenceEngineV73::new();
    let (text, cols, conf) = neural_layout.extract_text_and_layout(b"Sample PDF page content");
    assert_eq!(cols, 2);
    assert!(conf > 0.9);
    assert!(!text.is_empty());

    let mut crdt_collab = PdfcraftCrdtMultiUserCollaborationSyncEngineV73::new();
    crdt_collab.register_collaborator(1, "Editor1");
    assert_eq!(crdt_collab.apply_annotation_delta("Strikeout"), 1);

    let mut pqc_sig = PdfcraftPqcPostQuantumDigitalSignatureEngineV73::new();
    assert!(pqc_sig.verify_signature(b"hash", b"sig"));
    assert_eq!(pqc_sig.verified_signatures_count, 1);

    let mut vector_spline = PdfcraftVectorPathSplineSubdivisionEngineV73::new();
    assert!(vector_spline.insert_bezier_spline("M 0 0 C 10 10, 20 20, 30 30 Z"));
    assert_eq!(vector_spline.active_path_layers, 1);

    let cmyk = PdfcraftCmykSpotColorSeparationPreflightEngineV73::new();
    let (c, m, y, k, coverage) = cmyk.compute_ink_density_cmyk(64, 128, 192);
    assert!(coverage > 0.0);
    assert!(c >= 0.0 && m >= 0.0 && y >= 0.0 && k >= 0.0);

    let pii = PdfcraftIrreversiblePiiRedactionSanitizationEngineV73::new();
    let redacted = pii.redact_pii_pattern("Sensitive CARD: 4000-1234-5678-9010");
    assert!(redacted.contains("[REDACTED_PII]"));

    let imposition = PdfcraftPageImpositionBookletAssemblyEngineV73::new();
    assert_eq!(imposition.calculate_imposition_sheets(16, 4), 4);

    let mut acroforms = PdfcraftAcroFormsXfaDynamicCalculationEngineV73::new();
    acroforms.set_field_data("TaxId", "XX-XXXXXXX");
    let xml = acroforms.export_xfdf();
    assert!(xml.contains("XX-XXXXXXX"));

    let pdfa = PdfcraftPdfaArchivalPdfxPrintComplianceEngineV73::new();
    let (valid, bytes_len) = pdfa.validate_and_repair_pdf(b"%PDF-1.7 standard header");
    assert!(valid);
    assert_eq!(bytes_len, 24);

    let mut mesh3d = PdfcraftSpatial3dMeshU3dPrcViewerEngineV73::new();
    assert!(mesh3d.embed_u3d_scene(b"U3D_HEADER_BINARY_STREAM"));
    assert_eq!(mesh3d.embedded_mesh_count, 1);
}

#[test]
fn test_v73_gateway_evaluation_and_proposals() {
    let mut gateway = SovereignPdfcraftArtcraftMissingComponentsV73PrGateway::new();
    assert!(gateway.evaluate_v73_pdfcraft_parity());

    let proposals = gateway.collect_all_proposals();
    assert_eq!(proposals.len(), 10);

    for p in &proposals {
        assert!(p.pr_id >= 7321 && p.pr_id <= 7330);
        assert!(!p.pr_title.is_empty());
        assert_eq!(p.original_suite, "ArtCraft / PdfCraft Studio");
        assert_eq!(p.status, "Ready for Merge");
        assert_eq!(p.components_added.len(), 3);
    }
}

#[test]
fn test_v73_suite_verification() {
    let mut suite = PdfcraftArtcraftMissingComponentsV73PrSuite::new();
    assert_eq!(suite.total_pr_proposals(), 10);
    assert!(suite.verify_complete_v73_parity());
}
