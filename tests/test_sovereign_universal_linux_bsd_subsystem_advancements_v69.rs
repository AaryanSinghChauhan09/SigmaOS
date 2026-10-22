// Integration test suite for SigmaOS Universal Linux & BSD Subsystem Advancements Engine (V69)

#[path = "../src/distro/sovereign_universal_linux_bsd_subsystem_advancements_v69_pr.rs"]
mod sovereign_universal_linux_bsd_subsystem_advancements_v69_pr;

use sovereign_universal_linux_bsd_subsystem_advancements_v69_pr::sovereign_linux_bsd_all_subsystems_harmony::{
    LinuxBsdDistroInspiration, SigmaOsSubsystem,
};
use sovereign_universal_linux_bsd_subsystem_advancements_v69_pr::{
    SovereignUniversalLinuxBsdSubsystemAdvancementsEngineV69,
    SovereignUniversalLinuxBsdSubsystemAdvancementsPrGatewayV69,
};

#[test]
fn test_v69_full_system_harmony_and_audit() {
    let engine = SovereignUniversalLinuxBsdSubsystemAdvancementsEngineV69::new();
    assert_eq!(engine.evaluate_harmony_index(), 100);

    let (audit_map, score) = engine.audit_all_subsystems();
    assert_eq!(audit_map.len(), 44);
    assert_eq!(score, 100);
    for (subsystem, compliant) in audit_map {
        assert!(compliant, "Subsystem {} must be compliant", subsystem);
    }
}

#[test]
fn test_v69_cross_distro_policy_and_payload_translation() {
    let mut engine = SovereignUniversalLinuxBsdSubsystemAdvancementsEngineV69::new();

    // Verify paradigm switching across FreeBSD, OpenBSD, NixOS, Arch, Debian, Fedora, Whonix, Qubes
    assert!(engine
        .switch_active_distro_paradigm(LinuxBsdDistroInspiration::FreeBSD)
        .is_ok());
    assert_eq!(
        engine.current_active_distro,
        LinuxBsdDistroInspiration::FreeBSD
    );

    assert!(engine
        .switch_active_distro_paradigm(LinuxBsdDistroInspiration::OpenBSD)
        .is_ok());
    assert!(engine
        .switch_active_distro_paradigm(LinuxBsdDistroInspiration::NixOS)
        .is_ok());

    // Policy translation
    let policy = engine.translate_policy(
        SigmaOsSubsystem::KernelScheduling,
        SigmaOsSubsystem::SecuritySandboxing,
    );
    assert!(policy.is_some());
    let pol = policy.unwrap();
    assert!(pol.enforce_strict_isolation);

    // Payload transpilation
    let arch_to_deb = engine.transpile_payload(
        LinuxBsdDistroInspiration::ArchLinux,
        LinuxBsdDistroInspiration::DebianGNU,
        "pacman_hook:icon_update:pkg.tar.zst",
    );
    assert_eq!(arch_to_deb, "dpkg_trigger:icon_update:deb");

    let freebsd_to_arch = engine.transpile_payload(
        LinuxBsdDistroInspiration::FreeBSD,
        LinuxBsdDistroInspiration::ArchLinux,
        "jail_vnet:net0",
    );
    assert_eq!(freebsd_to_arch, "ebpf_xdp_namespace:net0");
}

#[test]
fn test_v69_cross_subsystem_workflow_execution() {
    let mut engine = SovereignUniversalLinuxBsdSubsystemAdvancementsEngineV69::new();

    let event_id = engine
        .execute_cross_subsystem_workflow(
            SigmaOsSubsystem::StorageDrive,
            SigmaOsSubsystem::CryptographySecurity,
            "ENCRYPT_VOLUME",
            "volume_id=nvme0n1;cipher=dilithium_kyber",
        )
        .expect("Workflow execution must succeed");

    assert!(event_id > 0);
}

#[test]
fn test_v69_pr_gateway_report_and_specs() {
    let gateway = SovereignUniversalLinuxBsdSubsystemAdvancementsPrGatewayV69::new();
    assert_eq!(gateway.proposals.len(), 44);
    assert!(gateway.verify_pr_proposals_coverage());

    let report = gateway.generate_summary_report();
    assert!(report.contains("Total PR Proposals: 44"));
    assert!(report.contains("Harmony Index: 100%"));
    assert!(report.contains("Subsystem Coverage: 44/44"));
    assert!(report.contains("Distro Paradigms: 38/38"));
    assert!(report.contains("VERIFIED PASS"));

    // Check specific PR proposals
    let sched_pr = gateway
        .proposals
        .iter()
        .find(|p| p.subsystem == SigmaOsSubsystem::KernelScheduling)
        .expect("Kernel scheduling PR must exist");
    assert_eq!(sched_pr.pr_id, "PR-HARMONY-V69-SUB-001");
    assert_eq!(sched_pr.supported_distro_count, 38);

    let firewall_pr = gateway
        .proposals
        .iter()
        .find(|p| p.subsystem == SigmaOsSubsystem::NetworkSecurityFirewall)
        .expect("Firewall PR must exist");
    assert_eq!(firewall_pr.pr_id, "PR-HARMONY-V69-SUB-044");
}
