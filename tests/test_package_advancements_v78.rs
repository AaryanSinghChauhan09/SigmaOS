// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Distro Package Advancements Suite V78
// (`tests/test_package_advancements_v78.rs`)

#[path = "../src/package/sovereign_distro_package_advancements_v78.rs"]
mod sovereign_distro_package_advancements_v78;

#[path = "../src/package/sovereign_distro_package_advancements_v78_pr.rs"]
mod sovereign_distro_package_advancements_v78_pr;

use sovereign_distro_package_advancements_v78::*;
use sovereign_distro_package_advancements_v78_pr::SovereignDistroPackagePrGatewayV78;

#[test]
fn test_package_advancements_v78_full_cycle() {
    // 1. Universal Package Format Inspector V78
    let mut inspector = UniversalPackageFormatInspectorV78::new();
    assert_eq!(inspector.total_formats_supported, 240);

    let manifest = inspector
        .inspect_package_payload("ripgrep", b"!<arch>\ndeb_payload", "ripgrep_13.0.0_amd64.deb")
        .expect("Failed to inspect DEB package payload");
    assert_eq!(manifest.name, "ripgrep");
    assert_eq!(manifest.detected_format, PackageFormat::Deb);
    assert_eq!(manifest.signature_kind, PackageSignatureKindV78::GpgOpenPgp);
    assert_eq!(manifest.provenance.as_ref().unwrap().slsa_level, 4);

    // 2. Polyglot OOP Design Patterns V78
    let mut pattern_engine = PolyglotPatternEngineV78::new();
    let cmd = Box::new(InstallPackageCommandV78 {
        package_id: "curl".to_string(),
    });
    let dispatch_res = pattern_engine
        .dispatch_command(cmd)
        .expect("Dispatch failed");
    assert!(dispatch_res.contains("curl"));
    let rollback_res = pattern_engine
        .rollback_last_command()
        .expect("Rollback failed");
    assert!(rollback_res.contains("curl"));

    // 3. UDF Scriptlet Execution Engine V78
    let mut scriptlet_engine = UdfScriptletEngineV78::new();
    let script_res = scriptlet_engine
        .run_scriptlet(ScriptletHookV78::PostInstall, "echo Hello SigmaOS")
        .expect("Scriptlet execution failed");
    assert!(script_res.contains("PostInstall"));

    let malicious_res = scriptlet_engine.run_scriptlet(ScriptletHookV78::PreRemove, "rm -rf /");
    assert!(malicious_res.is_err());

    // 4. SAT Dependency Solver & Vulnerability Auditor V78
    let solver = UniversalCrossDistroSatSolverV78::new();
    let raw_deps = vec!["libssl-dev".to_string(), "glibc".to_string(), "custom-tool".to_string()];
    let solved = solver.solve_and_remap(&raw_deps).expect("SAT solving failed");
    assert_eq!(
        solved,
        vec![
            "sovereign-openssl-dev".to_string(),
            "sovereign-libc".to_string(),
            "sovereign-custom-tool".to_string()
        ]
    );

    let vuln_deps = vec!["CVE-2024-9999-bad-ssl".to_string()];
    assert!(solver.solve_and_remap(&vuln_deps).is_err());

    // 5. Multi-Sandbox Security Governor V78
    let sandbox_gov = UniversalMultiSandboxGovernorV78::new();
    let sandbox_policy = sandbox_gov.enforce_sandbox_policy("ripgrep");
    assert!(sandbox_policy.contains("LinuxLandlockSeccomp"));

    // 6. CAS Delta Store Governor V78
    let mut cas_store = UniversalCasDeltaStoreGovernorV78::new();
    let res1 = cas_store.store_chunk_cas("chunk123", b"data1");
    assert!(res1.contains("Stored new CAS chunk"));
    let res2 = cas_store.store_chunk_cas("chunk123", b"data1");
    assert!(res2.contains("Deduplicated CAS chunk"));
    assert_eq!(cas_store.total_dedup_bytes, 5);

    // 7. Boot Environment Snapshot Governor V78
    let mut bootenv_gov = UniversalBootEnvSnapshotGovernorV78::new();
    let snap_res = bootenv_gov.create_pre_install_snapshot(SnapshotEngineKindV78::FreeBsdBectl, "v78-update");
    assert!(snap_res.contains("be-snap-FreeBsdBectl-v78-update"));

    // 8. Foreign PM CLI Router V78
    let router = UniversalPmCliRouterV78::new();
    let route_res = router
        .route_cli_command(&["apt", "install", "ripgrep"])
        .expect("CLI routing failed");
    assert!(route_res.contains("Translated APT command"));

    // 9. PR Gateway V78
    let mut gateway = SovereignDistroPackagePrGatewayV78::new();
    gateway.submit_proposal(
        7801,
        "Universal Package Advancements V78",
        PackageFormat::Deb,
        "ripgrep",
        "Full test cycle proposal V78",
    );
    assert_eq!(gateway.total_proposals(), 1);
    let proposal = gateway.get_proposal(7801).expect("Proposal not found");
    assert_eq!(proposal.verified_slsa_level, 4);

    // 10. Master Suite Coordinator V78
    let mut suite = SovereignDistroPackageAdvancementsSuiteV78::new();
    let pipeline_res = suite
        .run_full_suite_pipeline("htop", b"!<arch>\nhtop_data", "htop_1.0_amd64.deb")
        .expect("Master pipeline failed");
    assert!(pipeline_res.contains("Suite V78 Execution Succeeded"));
}
