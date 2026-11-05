// Integration Test Suite for Sovereign Virtualization & Container Advancements (v90 PR Suite)

extern crate alloc;

#[path = "../src/virtualization/sovereign_virtualization_and_containers_v90_pr.rs"]
mod sovereign_virtualization_and_containers_v90_pr;

use sovereign_virtualization_and_containers_v90_pr::*;

#[test]
fn test_integration_virtualization_and_containers_v90_suite() {
    let gateway = SovereignVirtualizationAndContainersParityV90Gateway::new();
    let proposals = gateway.collect_all_proposals();

    assert_eq!(proposals.len(), 10);
    assert_eq!(proposals[0].pr_id, 9001);
    assert_eq!(proposals[9].pr_id, 9010);

    let summary = gateway.generate_pr_markdown_summary();
    assert!(summary.contains("Sovereign Virtualization & Container Ecosystem Parity Suite (v90)"));
    assert!(summary.contains("KVM/QEMU Dirty Ring v2"));
    assert!(summary.contains("gVisor Sentry"));
    assert!(summary.contains("DILITHIUM5_SIG_V90_9001_OK"));
    assert!(summary.contains("DILITHIUM5_SIG_V90_9010_OK"));
}

#[test]
fn test_integration_sub_engines_v90() {
    let mut gateway = SovereignVirtualizationAndContainersParityV90Gateway::new();

    // 1. KVM/QEMU
    let vm = gateway.kvm_qemu.launch_kvm_vm(
        "vm_integration_v90",
        8,
        16384,
        ConfidentialModeV90::ArmRealmCca,
    );
    assert_eq!(vm.confidential_mode, ConfidentialModeV90::ArmRealmCca);
    assert_eq!(vm.dirty_ring_size, 16384);

    // 2. Libvirt Domain
    let dom = gateway
        .libvirt
        .parse_domain_xml("<domain type='kvm'><name>dom_integration_v90</name></domain>");
    assert_eq!(dom.domain_name, "dom_integration_v90");
    assert!(dom.snapshots_supported);

    // 3. Xen Hypervisor
    let guest = gateway
        .xen
        .create_domu_guest(15, "xen_domu_v90", XenGuestModeV90::Pvh, 2048);
    assert_eq!(guest.domid, 15);
    assert_eq!(guest.argo_ring_size, 524288);

    // 4. VirtualBox / bhyve / OpenBSD vmd
    let bhyve = gateway.vbox_bhyve_vmm.spawn_bhyve_vm("bhyve_v90", 4, 2048);
    assert!(bhyve.capsicum_restricted);

    // 5. OCI / Podman
    let podman = gateway.oci.run_podman_rootless("pod_v90", "debian:13", 512);
    assert!(podman.rootless);

    // 6. LXC / LXD / systemd-nspawn
    let incus = gateway
        .system_containers
        .create_incus_instance("incus_v90", "ovn-net-cluster");
    assert_eq!(incus.engine_type, "incus");

    // 7. FreeBSD Jails & Illumos Zones
    let jail = gateway
        .freebsd_illumos
        .create_vnet_jail("jail_v90", "/jails/v90", "epair0a");
    assert_eq!(jail.vnet_interface, "epair0a");

    // 8. K8s / K3s / Rancher Harvester
    let pod = gateway
        .k8s_rancher
        .deploy_pod("production", "api-gateway", "containerd");
    assert_eq!(pod.pod_name, "api-gateway");

    // 9. Firecracker / Cloud-Hypervisor
    let mvm = gateway
        .microvms
        .spawn_firecracker_microvm("fvm_v90", 2, 128);
    assert_eq!(mvm.boot_time_ms, 1);

    // 10. gVisor / Kata / Apple Hv
    let kata = gateway
        .secure_sandboxes
        .create_kata_sandbox("kata_v90", 512);
    assert_eq!(kata.technology, "kata");
    let apple = gateway
        .secure_sandboxes
        .launch_apple_hv_guest("apple_v90", 2048, true);
    assert!(apple.rosetta2_enabled);
    assert_eq!(apple.hv_vcpu_count, 8);
}
