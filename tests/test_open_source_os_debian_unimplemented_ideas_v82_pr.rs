// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Open Source OS & Debian GitHub Unimplemented Ideas V82 PR Engine

#[path = "../src/distro/debian_github_repo_missing_components_v82_pr.rs"]
mod debian_github_repo_missing_components_v82_pr;

#[path = "../src/distro/open_source_os_unimplemented_ideas_v82_pr.rs"]
mod open_source_os_unimplemented_ideas_v82_pr;

#[path = "../src/distro/open_source_os_debian_unimplemented_ideas_v82_pr.rs"]
mod open_source_os_debian_unimplemented_ideas_v82_pr;

use debian_github_repo_missing_components_v82_pr::*;
use open_source_os_debian_unimplemented_ideas_v82_pr::*;
use open_source_os_unimplemented_ideas_v82_pr::*;
use std::collections::BTreeSet;

#[test]
fn test_debian_dpkg_triggers_deferred_engine_v82() {
    let mut engine = DebianDpkgTriggersOrderingEngineV82Pr::new();
    assert!(engine.activate_trigger("/etc/alternatives"));
    let processed = engine.process_deferred_queue();
    assert_eq!(processed.len(), 5);
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8210);
    assert_eq!(spec.component_name, "dpkg-triggers");
}

#[test]
fn test_debian_apt_pqc_dilithium_engine_v82() {
    let engine = DebianSecureAptPqcDilithiumInReleaseEngineV82Pr::new();
    assert!(engine.verify_inrelease_signature(
        "stable-v82",
        b"Package: sigma-core",
        "DILITHIUM5-SIG-DEBIAN-APT-PQC-V82"
    ));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8211);
}

#[test]
fn test_debian_debconf_database_state_engine_v82() {
    let mut engine = DebianDebconfStateDatabaseEngineV82Pr::new();
    assert_eq!(
        engine.get_answer("debconf/frontend"),
        Some(&"Noninteractive".to_string())
    );
    engine.set_answer("debconf/frontend", "Dialog");
    assert_eq!(
        engine.get_answer("debconf/frontend"),
        Some(&"Dialog".to_string())
    );
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8212);
}

#[test]
fn test_debian_lintian_static_analysis_engine_v82() {
    let engine = DebianLintianStaticPolicyCheckerEngineV82Pr::new();
    let issues = engine.analyze_package_spec(false, false);
    assert_eq!(issues.len(), 3);
    assert!(issues.contains(&"E: missing-copyright-file".to_string()));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8213);
}

#[test]
fn test_debian_sbuild_cleanroom_builder_engine_v82() {
    let engine = DebianBuilddCleanroomSbuildChrootEngineV82Pr::new();
    assert!(engine.build_in_cleanroom("sbuild-chroot", "sigmaos_1.0.dsc"));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8214);
}

#[test]
fn test_debian_debsums_integrity_auditor_engine_v82() {
    let engine = DebianDebsumsIntegrityTamperDetectorEngineV82Pr::new();
    assert!(engine.audit_file(
        "/usr/bin/dpkg",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    ));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8215);
}

#[test]
fn test_debian_popcon_telemetry_analyzer_engine_v82() {
    let engine = DebianPopconUsageTelemetryEngineV82Pr::new();
    assert!(engine
        .compile_telemetry_packet()
        .contains("vendor=SigmaOS-V82"));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8216);
}

#[test]
fn test_debian_piuparts_package_sanity_engine_v82() {
    let engine = DebianPiupartsLifecycleSanityEngineV82Pr::new();
    let mut files = BTreeSet::new();
    files.insert("/etc/debian_version".to_string());
    files.insert("/tmp/orphan_leftover.tmp".to_string());
    let orphans = engine.verify_purge_cleanliness(&files);
    assert_eq!(orphans, vec!["/tmp/orphan_leftover.tmp".to_string()]);
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8217);
}

#[test]
fn test_debian_subiquity_autoinstall_engine_v82() {
    let engine = DebianSubiquityInstallerAutoinstallEngineV82Pr::new();
    assert_eq!(
        engine.parse_autoinstall_hostname(),
        Some("sigma-node-v82".to_string())
    );
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8218);
}

#[test]
fn test_debian_multiarch_apt_resolver_engine_v82() {
    let engine = DebianMultiarchAptResolverEngineV82Pr::new();
    assert!(engine.resolve_coinstallation("libc6", "riscv64"));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8219);
}

#[test]
fn test_open_source_os_v82_unimplemented_ideas_suite() {
    let suite = OpenSourceOsUnimplementedIdeasV82PrSuite::new();
    let proposals = suite.collect_all_proposals();
    assert_eq!(proposals.len(), 9);
    assert_eq!(proposals[0].pr_id, "PR-SIGMA-V82-01-LINUX-633");

    let matrix = suite.evaluate_parity_matrix();
    assert_eq!(matrix.len(), 9);
    assert!(matrix.values().all(|v| *v));
}

#[test]
fn test_unified_open_source_os_debian_v82_suite() {
    let suite = OpenSourceOsDebianUnimplementedIdeasV82PrSuite::new();
    let proposals = suite.generate_unified_pr_proposals();
    assert_eq!(proposals.len(), 19);
    assert!(proposals[0].pr_id.starts_with("PR-OS-V82-"));
    let parity = suite.evaluate_distro_parity_score();
    assert_eq!(parity, 100);
}
