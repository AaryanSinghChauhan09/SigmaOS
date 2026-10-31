// Integration Test Suite for Sovereign Virtualization & Container Advancements (v88 PR Suite)

extern crate alloc;

#[path = "../src/virtualization/sovereign_virtualization_and_containers_v88_pr.rs"]
mod sovereign_virtualization_and_containers_v88_pr;

use sovereign_virtualization_and_containers_v88_pr::*;

#[test]
fn test_integration_virtualization_and_containers_v88_suite() {
    let gateway = SovereignVirtualizationAndContainersParityV88Gateway::new();
    let proposals = gateway.collect_all_proposals();

    assert_eq!(proposals.len(), 10);
    assert_eq!(proposals[0].pr_id, 8801);
    assert_eq!(proposals[9].pr_id, 8810);

    let summary = gateway.generate_pr_markdown_summary();
    assert!(summary.contains("Sovereign Virtualization & Container Ecosystem Parity Suite (v88)"));
    assert!(summary.contains("KVM/QEMU Dirty Ring"));
    assert!(summary.contains("gVisor Sentry"));
}

#[test]
fn test_integration_sub_engines_v88() {
    let mut gateway = SovereignVirtualizationAndContainersParityV88Gateway::new();

    // 1. KVM/QEMU
    let vm = gateway.kvm_qemu.launch_kvm_vm("vm_integration", 4, 4096, ConfidentialModeV88::IntelTdx);
    assert_eq!(vm.confidential_mode, ConfidentialModeV88::IntelTdx);

    // 2. Libvirt Domain
    let dom = gateway.libvirt.parse_domain_xml("<domain type='kvm'><name>dom_integration</name></domain>");
    assert_eq!(dom.domain_name, "dom_integration");

    // 3. Xen Hypervisor
    let guest = gateway.xen.create_domu_guest(10, "xen_domu", XenGuestModeV88::Pv, 512);
    assert_eq!(guest.domid, 10);

    // 4. VirtualBox / bhyve / OpenBSD vmd
    let bhyve = gateway.vbox_bhyve_vmm.spawn_bhyve_vm("bhyve1", 2, 1024);
    assert!(bhyve.capsicum_restricted);

    // 5. OCI / Podman
    let podman = gateway.oci.run_podman_rootless("pod1", "debian:12", 256);
    assert!(podman.rootless);

    // 6. LXC / LXD / systemd-nspawn
    let incus = gateway.system_containers.create_incus_instance("incus1", "ovn-net-main");
    assert_eq!(incus.engine_type, "incus");

    // 7. FreeBSD Jails & Illumos Zones
    let jail = gateway.freebsd_illumos.create_vnet_jail("jail_v88", "/jails/v88", "epair0a");
    assert_eq!(jail.vnet_interface, "epair0a");

    // 8. K8s / K3s / Rancher Harvester
    let pod = gateway.k8s_rancher.deploy_pod("kube-system", "coredns", "containerd");
    assert_eq!(pod.pod_name, "coredns");

    // 9. Firecracker / Cloud-Hypervisor
    let mvm = gateway.microvms.spawn_firecracker_microvm("fvm1", 1, 64);
    assert_eq!(mvm.boot_time_ms, 2);

    // 10. gVisor / Kata / Apple Hv
    let kata = gateway.secure_sandboxes.create_kata_sandbox("kata1", 256);
    assert_eq!(kata.technology, "kata");
}
