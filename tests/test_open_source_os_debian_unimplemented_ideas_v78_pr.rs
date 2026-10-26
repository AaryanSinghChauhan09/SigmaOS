// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Open Source OS & Debian GitHub Unimplemented Ideas V78 PR Engine

#[path = "../src/distro/debian_github_repo_missing_components_v78_pr.rs"]
mod debian_github_repo_missing_components_v78_pr;

#[path = "../src/distro/open_source_os_missing_components_v78_pr.rs"]
mod open_source_os_missing_components_v78_pr;

#[path = "../src/distro/open_source_os_unimplemented_ideas_v78_pr.rs"]
mod open_source_os_unimplemented_ideas_v78_pr;

#[path = "../src/distro/open_source_os_debian_unimplemented_ideas_v78_pr.rs"]
mod open_source_os_debian_unimplemented_ideas_v78_pr;

use debian_github_repo_missing_components_v78_pr::*;
use open_source_os_debian_unimplemented_ideas_v78_pr::*;
use open_source_os_unimplemented_ideas_v78_pr::*;
use std::collections::BTreeSet;

#[test]
fn test_debian_dpkg_triggers_deferred_engine_v78() {
    let mut engine = DebianDpkgTriggersOrderingEngineV78Pr::new();
    assert!(engine.activate_trigger("/etc/alternatives"));
    let processed = engine.process_deferred_queue();
    assert_eq!(processed.len(), 5);
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7810);
    assert_eq!(spec.component_name, "dpkg-triggers");
}

#[test]
fn test_debian_apt_pqc_dilithium_engine_v78() {
    let engine = DebianSecureAptPqcDilithiumInReleaseEngineV78Pr::new();
    assert!(engine.verify_inrelease_signature(
        "stable-v78",
        b"Package: sigma-core",
        "DILITHIUM5-SIG-DEBIAN-APT-PQC-V78"
    ));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7811);
}

#[test]
fn test_debian_debconf_database_state_engine_v78() {
    let mut engine = DebianDebconfStateDatabaseEngineV78Pr::new();
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
    assert_eq!(spec.pr_id, 7812);
}

#[test]
fn test_debian_lintian_static_analysis_engine_v78() {
    let engine = DebianLintianStaticPolicyCheckerEngineV78Pr::new();
    let issues = engine.analyze_package_spec(false, false);
    assert_eq!(issues.len(), 3);
    assert!(issues.contains(&"E: missing-copyright-file".to_string()));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7813);
}

#[test]
fn test_debian_sbuild_cleanroom_builder_engine_v78() {
    let engine = DebianBuilddCleanroomSbuildChrootEngineV78Pr::new();
    assert!(engine.build_in_cleanroom("sbuild-chroot", "sigmaos_1.0.dsc"));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7814);
}

#[test]
fn test_debian_debsums_integrity_auditor_engine_v78() {
    let engine = DebianDebsumsIntegrityTamperDetectorEngineV78Pr::new();
    assert!(engine.audit_file(
        "/usr/bin/dpkg",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    ));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7815);
}

#[test]
fn test_debian_popcon_telemetry_analyzer_engine_v78() {
    let engine = DebianPopconUsageTelemetryEngineV78Pr::new();
    assert!(engine.compile_telemetry_packet().contains("vendor=SigmaOS-V78"));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7816);
}

#[test]
fn test_debian_piuparts_package_sanity_engine_v78() {
    let engine = DebianPiupartsLifecycleSanityEngineV78Pr::new();
    let mut files = BTreeSet::new();
    files.insert("/etc/debian_version".to_string());
    files.insert("/tmp/orphan_leftover.tmp".to_string());
    let orphans = engine.verify_purge_cleanliness(&files);
    assert_eq!(orphans, vec!["/tmp/orphan_leftover.tmp".to_string()]);
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7817);
}

#[test]
fn test_debian_subiquity_autoinstall_engine_v78() {
    let engine = DebianSubiquityInstallerAutoinstallEngineV78Pr::new();
    assert_eq!(
        engine.parse_autoinstall_hostname(),
        Some("sigma-node-v78".to_string())
    );
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7818);
}

#[test]
fn test_debian_multiarch_apt_resolver_engine_v78() {
    let engine = DebianMultiarchAptResolverEngineV78Pr::new();
    assert!(engine.resolve_coinstallation("libc6", "riscv64"));
    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 7819);
}

#[test]
fn test_unified_open_source_os_debian_v78_suite() {
    let suite = OpenSourceOsDebianUnimplementedIdeasV78PrSuite::new();
    let proposals = suite.generate_unified_pr_proposals();
    assert!(!proposals.is_empty());
    assert!(proposals[0].pr_id.starts_with("PR-SIGMA-V78-"));
    let parity = suite.evaluate_distro_parity_score();
    assert_eq!(parity, 100);
}
