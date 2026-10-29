//! Integration tests for Sovereign Tech Media Component Advancements V76 PR.

#[path = "../src/distro/tech_media_component_advancements_v76_pr.rs"]
mod tech_media_component_advancements_v76_pr;

use tech_media_component_advancements_v76_pr::*;

#[test]
fn test_v76_tech_media_integration_suite() {
    let master = SovereignTechMediaComponentAdvancementsV76MasterSuite::new();
    assert!(master.verify_all_components());

    let synthesis = SovereignTechMediaArticleInsightSynthesisEngineV76::new();
    assert_eq!(synthesis.insights.len(), 27);

    // Verify query resolution for specific domain patterns
    let itsfoss_insights = synthesis.query_insights_by_domain("itsfoss.com");
    assert_eq!(itsfoss_insights.len(), 1);
    assert_eq!(itsfoss_insights[0].publication, "ItsFOSS");

    let concat_domain = synthesis.query_insights_by_domain("techpowerup.techpwindowscentral.com");
    assert_eq!(concat_domain.len(), 1);

    let dw_domain = synthesis.query_insights_by_domain("distrowatch.com");
    assert_eq!(dw_domain.len(), 1);
    assert_eq!(dw_domain[0].publication, "DistroWatch");
}

#[test]
fn test_v76_individual_engines_functionality() {
    let terminal = ItsFossTerminalOptimizationEngineV76::new();
    assert!(terminal.verify_tooling());

    let linux9to5 = NineToFiveLinuxKernelRtEngineV76::new();
    assert!(linux9to5.is_lts_rt_active());

    let geeky = GeekyGadgetsSbcTelemetryEngineV76::new();
    assert!(geeky.is_riscv());

    let lfoundation = LinuxFoundationSbomLicenseAuditEngineV76::new();
    assert!(lfoundation.is_sbom_compliant());

    let phoronix = PhoronixAutomatedBenchmarkEngineV76::new();
    assert!(phoronix.verify_benchmark());
    assert_eq!(phoronix.calculate_geometric_mean(&[2.0, 8.0]), 4.0);
}
