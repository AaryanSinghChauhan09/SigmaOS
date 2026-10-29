// SPDX-License-Identifier: MIT
// Open Source OS & Debian Linux Unimplemented Ideas V81 PR Integration Tests
// (`tests/test_open_source_os_debian_unimplemented_ideas_v81_pr.rs`)

#[path = "../src/distro/open_source_os_debian_unimplemented_ideas_v81_pr.rs"]
mod open_source_os_debian_unimplemented_ideas_v81_pr;

use open_source_os_debian_unimplemented_ideas_v81_pr::debian_github_repo_missing_components_v81_pr::*;
use open_source_os_debian_unimplemented_ideas_v81_pr::open_source_os_unimplemented_ideas_v81_pr_mod::*;
use open_source_os_debian_unimplemented_ideas_v81_pr::*;
use std::collections::BTreeSet;

#[test]
fn test_unified_open_source_os_debian_v81_suite() {
    let suite = OpenSourceOsDebianUnimplementedIdeasV81PrSuite::new();
    let proposals = suite.generate_unified_pr_proposals();

    assert_eq!(proposals.len(), 19);

    // Verify 9 OS proposals and 10 Debian proposals
    let os_proposals: Vec<_> = proposals.iter().filter(|p| p.pr_id.starts_with("PR-OS-V81-")).collect();
    let debian_proposals: Vec<_> = proposals.iter().filter(|p| p.pr_id.starts_with("PR-DEBIAN-V81-")).collect();

    assert_eq!(os_proposals.len(), 9);
    assert_eq!(debian_proposals.len(), 10);

    for prop in &proposals {
        assert!(!prop.pr_id.is_empty());
        assert!(!prop.title.is_empty());
        assert!(!prop.target_component.is_empty());
        assert!(!prop.status.is_empty());
        assert!(!prop.description.is_empty());
        assert!(!prop.git_diff_patch.is_empty());
        assert!(prop.pqc_signature.contains("DILITHIUM5"));
    }

    let parity_score = suite.evaluate_distro_parity_score();
    assert_eq!(parity_score, 100);
}

#[test]
fn test_open_source_os_v81_unimplemented_ideas_suite() {
    let suite = SovereignUnimplementedIdeasSuiteV81::new();
    let manifests = suite.collect_all_manifests();
    assert_eq!(manifests.len(), 9);

    let summary = suite.generate_suite_summary();
    assert_eq!(summary.get("total_proposals").unwrap(), "9");
    assert_eq!(summary.get("pqc_signature_algorithm").unwrap(), "Dilithium-5");
    assert_eq!(summary.get("interoperability_score").unwrap(), "100.0%");
}

#[test]
fn test_debian_dpkg_triggers_deferred_engine_v81() {
    let mut engine = DebianDpkgTriggersOrderingEngineV81Pr::new();
    assert!(engine.activate_trigger("/usr/share/doc"));
    assert!(!engine.activate_trigger("/usr/share/doc"));

    let processed = engine.process_deferred_queue();
    assert_eq!(processed.len(), 5);
    assert_eq!(engine.pending_triggers.len(), 0);

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8110);
    assert_eq!(spec.component_name, "dpkg-triggers");
    assert!(spec.pqc_signature.contains("DILITHIUM5"));
}

#[test]
fn test_debian_apt_pqc_dilithium_engine_v81() {
    let engine = DebianSecureAptPqcDilithiumInReleaseEngineV81Pr::new();
    let valid = engine.verify_inrelease_signature("stable", b"Package: test", "DILITHIUM5-SIG-VALID");
    assert!(valid);

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8111);
    assert_eq!(spec.component_name, "apt-pqc");
}

#[test]
fn test_debian_debconf_database_state_engine_v81() {
    let mut engine = DebianDebconfStateDatabaseEngineV81Pr::new();
    assert_eq!(engine.get_answer("tzdata/zones").unwrap(), "UTC");

    engine.set_answer("tzdata/zones", "EST");
    assert_eq!(engine.get_answer("tzdata/zones").unwrap(), "EST");

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8112);
    assert_eq!(spec.component_name, "debconf");
}

#[test]
fn test_debian_lintian_static_analysis_engine_v81() {
    let engine = DebianLintianStaticPolicyCheckerEngineV81Pr::new();
    let issues = engine.analyze_package_spec(false, false);
    assert!(issues.contains(&"E: missing-copyright-file".to_string()));
    assert!(issues.contains(&"W: no-changelog-entry".to_string()));

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8113);
    assert_eq!(spec.component_name, "lintian");
}

#[test]
fn test_debian_sbuild_cleanroom_builder_engine_v81() {
    let engine = DebianBuilddCleanroomSbuildChrootEngineV81Pr::new();
    assert!(engine.build_in_cleanroom("/chroot/sbuild-amd64", "package.dsc"));
    assert!(!engine.build_in_cleanroom("/chroot/normal", "package.tar.gz"));

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8114);
    assert_eq!(spec.component_name, "sbuild");
}

#[test]
fn test_debian_debsums_integrity_auditor_engine_v81() {
    let engine = DebianDebsumsIntegrityTamperDetectorEngineV81Pr::new();
    let hash = "a".repeat(64);
    assert!(engine.audit_file("/bin/bash", &hash));
    assert!(!engine.audit_file("/bin/bash", "short_hash"));

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8115);
    assert_eq!(spec.component_name, "debsums");
}

#[test]
fn test_debian_popcon_telemetry_analyzer_engine_v81() {
    let engine = DebianPopconUsageTelemetryEngineV81Pr::new();
    let packet = engine.compile_telemetry_packet();
    assert!(packet.contains("vendor=SigmaOS-V81"));

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8116);
    assert_eq!(spec.component_name, "popcon");
}

#[test]
fn test_debian_piuparts_package_sanity_engine_v81() {
    let engine = DebianPiupartsLifecycleSanityEngineV81Pr::new();
    let mut files = BTreeSet::new();
    files.insert("/etc/debian_version".to_string());
    files.insert("/var/log/orphan.log".to_string());

    let orphans = engine.verify_purge_cleanliness(&files);
    assert_eq!(orphans.len(), 1);
    assert_eq!(orphans[0], "/var/log/orphan.log");

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8117);
    assert_eq!(spec.component_name, "piuparts");
}

#[test]
fn test_debian_subiquity_autoinstall_engine_v81() {
    let engine = DebianSubiquityInstallerAutoinstallEngineV81Pr::new();
    let hostname = engine.parse_autoinstall_hostname();
    assert_eq!(hostname.unwrap(), "sigma-node-v81");

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8118);
    assert_eq!(spec.component_name, "subiquity");
}

#[test]
fn test_debian_multiarch_apt_resolver_engine_v81() {
    let engine = DebianMultiarchAptResolverEngineV81Pr::new();
    assert!(engine.resolve_coinstallation("libc6", "amd64"));
    assert!(engine.resolve_coinstallation("libc6", "i386"));
    assert!(!engine.resolve_coinstallation("libc6", "invalid_arch"));

    let spec = engine.generate_pr_spec();
    assert_eq!(spec.pr_id, 8119);
    assert_eq!(spec.component_name, "apt-multiarch");
}
