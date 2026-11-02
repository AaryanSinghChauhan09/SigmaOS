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
    assert!(summary.contains("KVM/QEMU Dirty Ring"));
    assert!(summary.contains("gVisor Sentry"));
    assert!(summary.contains("DILITHIUM5_SIG_V90_9001_OK"));
    assert!(summary.contains("DILITHIUM5_SIG_V90_9010_OK"));
}

#[test]
fn test_integration_sub_engines_v90() {
    let mut gateway = SovereignVirtualizationAndContainersParityV90Gateway::new();

    // 1. KVM/QEMU
    let vm = gateway.kvm_qemu.launch_kvm_vm("vm_integration", 4, 4096, ConfidentialModeV90::IntelTdx);
    assert_eq!(vm.confidential_mode, ConfidentialModeV90::IntelTdx);
    let attestation = gateway.kvm_qemu.generate_confidential_attestation_report("vm_integration").unwrap();
    assert!(attestation.contains("INTEL_TDX_QUOTE"));

    // 2. Libvirt Domain
    let dom = gateway.libvirt.parse_domain_xml("<domain type='kvm'><name>dom_integration</name></domain>");
    assert_eq!(dom.domain_name, "dom_integration");
    assert!(gateway.libvirt.transition_domain_state("dom_integration", LibvirtDomainStateV90::Suspended).is_ok());

    // 3. Xen Hypervisor
    let guest = gateway.xen.create_domu_guest(10, "xen_domu", XenGuestModeV90::Pv, 512);
    assert_eq!(guest.domid, 10);
    assert!(gateway.xen.write_xenstore_key(10, "ip", "10.0.0.10").is_ok());

    // 4. VirtualBox / bhyve / OpenBSD vmd
    let bhyve = gateway.vbox_bhyve_vmm.spawn_bhyve_vm("bhyve1", 2, 1024);
    assert!(bhyve.capsicum_restricted);
    assert!(bhyve.uefi_boot);

    // 5. OCI / Podman
    let podman = gateway.oci.run_podman_rootless("pod1", "debian:12", 256);
    assert!(podman.rootless);
    let sec_eval = gateway.oci.validate_seccomp_profile("pod1").unwrap();
    assert!(sec_eval.contains("SECCOMP_DEFAULT_STRICT_PASSED"));

    // 6. LXC / LXD / systemd-nspawn
    let incus = gateway.system_containers.create_incus_instance("incus1", "ovn-net-main");
    assert_eq!(incus.engine_type, "incus");
    assert!(incus.idmapped_mounts);

    // 7. FreeBSD Jails & Illumos Zones
    let jail = gateway.freebsd_illumos.create_vnet_jail("jail_v90", "/jails/v90", "epair0a");
    assert_eq!(jail.vnet_interface, "epair0a");

    // 8. K8s / K3s / Rancher Harvester
    let pod = gateway.k8s_rancher.deploy_pod("kube-system", "coredns", "containerd");
    assert_eq!(pod.pod_name, "coredns");
    assert!(gateway.k8s_rancher.verify_pod_ebpf_policy("kube-system", "coredns"));

    // 9. Firecracker / Cloud-Hypervisor
    let mvm = gateway.microvms.spawn_firecracker_microvm("fvm1", 1, 64);
    assert_eq!(mvm.boot_time_ms, 2);
    assert!(mvm.jailer_isolated);

    // 10. gVisor / Kata / Apple Hv
    let kata = gateway.secure_sandboxes.create_kata_sandbox("kata1", 256);
    assert_eq!(kata.technology, "kata");
}
