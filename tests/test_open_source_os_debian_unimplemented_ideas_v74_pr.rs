// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Open Source OS & Debian GitHub Unimplemented Ideas V74 PR Engine

#[path = "../src/distro/debian_github_repo_missing_components_v74_pr.rs"]
mod debian_github_repo_missing_components_v74_pr;

#[path = "../src/distro/open_source_os_missing_components_v74_pr.rs"]
mod open_source_os_missing_components_v74_pr;

#[path = "../src/distro/open_source_os_debian_unimplemented_ideas_v74_pr.rs"]
mod open_source_os_debian_unimplemented_ideas_v74_pr;

use debian_github_repo_missing_components_v74_pr::*;
use open_source_os_debian_unimplemented_ideas_v74_pr::*;
use std::collections::BTreeSet;

#[test]
fn test_debian_dpkg_triggers_deferred_engine_v74() {
    let mut engine = DebianDpkgTriggersDeferredExecutionV74PrEngine::new();
    assert!(engine.activate_trigger("/etc/alternatives"));
    let processed = engine.process_deferred_queue();
    assert_eq!(processed.len(), 4);
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7410);
    assert_eq!(spec.component_name, "dpkg-triggers");
}

#[test]
fn test_debian_apt_pqc_dilithium_engine_v74() {
    let engine = DebianSecureAptPqcDilithiumInReleaseV74PrEngine::new();
    assert!(engine.verify_inrelease_signature(
        "stable-v74",
        b"Package: sigma-core",
        "DILITHIUM5-SIG-DEBIAN-APT-PQC-V74"
    ));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7411);
}

#[test]
fn test_debian_debconf_database_state_engine_v74() {
    let mut engine = DebianDebconfDatabaseStateV74PrEngine::new();
    assert_eq!(
        engine.get_answer("tzdata/zones"),
        Some(&"UTC".to_string())
    );
    engine.set_answer("tzdata/zones", "EST");
    assert_eq!(
        engine.get_answer("tzdata/zones"),
        Some(&"EST".to_string())
    );
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7412);
}

#[test]
fn test_debian_lintian_static_analysis_engine_v74() {
    let engine = DebianLintianStaticAnalysisV74PrEngine::new();
    let issues = engine.analyze_package_spec(false, false);
    assert_eq!(issues.len(), 2);
    assert!(issues.contains(&"E: missing-copyright-file".to_string()));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7413);
}

#[test]
fn test_debian_sbuild_cleanroom_builder_engine_v74() {
    let engine = DebianBuilddCleanroomSbuildV74PrEngine::new();
    assert!(engine.build_in_cleanroom("sbuild-chroot", "sigmaos_1.0.dsc"));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7414);
}

#[test]
fn test_debian_debsums_integrity_auditor_engine_v74() {
    let engine = DebianDebsumsIntegrityAuditorV74PrEngine::new();
    assert!(engine.audit_file(
        "/usr/bin/dpkg",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    ));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7415);
}

#[test]
fn test_debian_popcon_telemetry_analyzer_engine_v74() {
    let engine = DebianPopconTelemetryAnalyzerV74PrEngine::new();
    assert!(engine.compile_telemetry_packet().contains("vendor=SigmaOS"));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7416);
}

#[test]
fn test_debian_piuparts_package_sanity_engine_v74() {
    let engine = DebianPiupartsPackageSanitySuiteV74PrEngine::new();
    let mut files = BTreeSet::new();
    files.insert("/etc/debian_version".to_string());
    files.insert("/tmp/orphan_leftover.tmp".to_string());
    let orphans = engine.verify_purge_cleanliness(&files);
    assert_eq!(orphans, vec!["/tmp/orphan_leftover.tmp".to_string()]);
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7417);
}

#[test]
fn test_debian_subiquity_autoinstall_engine_v74() {
    let engine = DebianSubiquityInstallerAutoinstallV74PrEngine::new();
    assert_eq!(
        engine.parse_autoinstall_hostname(),
        Some("sigma-node-v74".to_string())
    );
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7418);
}

#[test]
fn test_debian_multiarch_apt_resolver_engine_v74() {
    let engine = DebianMultiarchAptResolverV74PrEngine::new();
    assert!(engine.resolve_coinstallation("libc6", "amd64"));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7419);
}

#[test]
fn test_unified_open_source_os_debian_v74_suite() {
    let suite = OpenSourceOsDebianUnimplementedIdeasV74PrSuite::new();
    let proposals = suite.generate_unified_pr_proposals();
    assert!(!proposals.is_empty());
    assert!(proposals[0].pr_id.starts_with("PR-OS-V74-"));
    let parity = suite.evaluate_distro_parity_score();
    assert!(parity >= 95);
}
