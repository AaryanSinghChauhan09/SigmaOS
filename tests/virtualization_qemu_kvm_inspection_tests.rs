// SigmaOS Virtualization, QEMU & KVM Inspection Unit Test Suite
// Inspired by QEMU, KVM, Firecracker, and Cloud-Hypervisor test harnesses.

#[path = "../src/virtualization/vm_manager.rs"]
mod vm_manager;

use vm_manager::{
    kvm_ioctl, AmdViIommuManager, HypervisorBackend, IntelVtxBackend, KvmDirtyRing, KvmExitReason,
    KvmIoctlDispatcher, KvmVirtualCpu, OsType, QemuBackend, QemuMonitorEngine, VirtioVirtqueue,
    VmConfig, VmManager, VmState,
};

#[test]
fn test_kvm_vcpu_lifecycle_and_exit_reasons() {
    let mut vcpu = KvmVirtualCpu::new(1);
    vcpu.registers.rip = 0xFFFFFFFF81000000;
    vcpu.registers.rsp = 0xFFFF888000000000;

    assert_eq!(vcpu.vcpu_id, 1);
    assert_eq!(vcpu.registers.rip, 0xFFFFFFFF81000000);

    // Initial state exit reason should be Hlt
    let exit = vcpu.run_vcpu();
    assert_eq!(exit, KvmExitReason::Hlt);

    // Inject interrupt and verify execution exit
    vcpu.inject_interrupt(0x20); // IRQ 32
    let irq_exit = vcpu.run_vcpu();
    assert_eq!(irq_exit, KvmExitReason::Interrupt);

    // Test live migration snapshot save and restore
    vcpu.dirty_ring.mark_page_dirty(42);
    let snapshot = vcpu.save_migration_state();
    assert_eq!(snapshot.vcpu_id, 1);
    assert_eq!(snapshot.registers.rip, 0xFFFFFFFF81000000);
    assert_eq!(snapshot.dirty_pages_count, 1);

    let mut new_vcpu = KvmVirtualCpu::new(2);
    new_vcpu.restore_migration_state(snapshot);
    assert_eq!(new_vcpu.vcpu_id, 1);
    assert_eq!(new_vcpu.registers.rip, 0xFFFFFFFF81000000);
}

#[test]
fn test_kvm_dirty_ring_tracking() {
    let mut dirty_ring = KvmDirtyRing::new(4096);
    assert_eq!(dirty_ring.ring_size, 4096);

    // Verify page 0, 100, 4095
    assert!(!dirty_ring.is_page_dirty(0));
    assert!(!dirty_ring.is_page_dirty(100));

    dirty_ring.mark_page_dirty(0);
    dirty_ring.mark_page_dirty(100);

    assert!(dirty_ring.is_page_dirty(0));
    assert!(dirty_ring.is_page_dirty(100));
    assert!(!dirty_ring.is_page_dirty(101));

    dirty_ring.clear();
    assert!(!dirty_ring.is_page_dirty(0));
    assert!(!dirty_ring.is_page_dirty(100));
}

#[test]
fn test_virtio_split_virtqueue() {
    let mut vq = VirtioVirtqueue::new(128);
    assert_eq!(vq.queue_size, 128);

    // Submit buffer descriptors
    assert!(vq.submit_descriptor(0, 0x10000, 2048, 1).is_ok());
    assert!(vq.submit_descriptor(1, 0x20000, 4096, 0).is_ok());

    assert_eq!(vq.avail_idx, 2);
    assert_eq!(vq.descriptors[0].addr, 0x10000);
    assert_eq!(vq.descriptors[1].len, 4096);

    // Out of bounds descriptor ID check
    assert!(vq.submit_descriptor(128, 0x30000, 512, 0).is_err());

    vq.complete_descriptor();
    assert_eq!(vq.used_idx, 1);
}

#[test]
fn test_qemu_monitor_protocol_qmp() {
    let mut qmp = QemuMonitorEngine::new();
    qmp.subscribe_event("SHUTDOWN");
    assert_eq!(qmp.event_subscribers.len(), 1);

    let query_res = qmp
        .execute_qmp_command("{\"execute\": \"query-status\"}")
        .unwrap();
    assert!(query_res.contains("\"running\": true"));

    let balloon_res = qmp
        .execute_qmp_command("{\"execute\": \"balloon\", \"arguments\": {\"value\": 2048}}")
        .unwrap();
    assert!(balloon_res.contains("\"return\": {}"));

    assert_eq!(qmp.command_history.len(), 2);
}

#[test]
fn test_kvm_ioctl_dispatcher() {
    let mut kvm = KvmIoctlDispatcher::new();
    let version = kvm
        .dispatch_ioctl(kvm_ioctl::KVM_GET_API_VERSION, 0)
        .unwrap();
    assert_eq!(version, 12);

    let vm_fd = kvm.dispatch_ioctl(kvm_ioctl::KVM_CREATE_VM, 0).unwrap();
    assert_eq!(vm_fd, 0);

    let vcpu_fd = kvm.dispatch_ioctl(kvm_ioctl::KVM_CREATE_VCPU, 1).unwrap();
    assert_eq!(vcpu_fd, 1);
    assert_eq!(kvm.created_vcpus, vec![1]);

    let mem_res = kvm
        .dispatch_ioctl(kvm_ioctl::KVM_SET_USER_MEMORY_REGION, (1024 << 16) | 0)
        .unwrap();
    assert_eq!(mem_res, 0);
    assert_eq!(*kvm.user_memory_regions.get(&0).unwrap(), 1024);
}

#[test]
fn test_amd_vi_iommu_dma_protection() {
    let mut iommu = AmdViIommuManager::new();
    let dev_addr = "0000:01:00.0";

    assert!(!iommu.verify_dma_access(dev_addr));

    iommu.attach_device(dev_addr.to_string());
    assert!(iommu.verify_dma_access(dev_addr));

    iommu.translation_table_active = false;
    assert!(!iommu.verify_dma_access(dev_addr));
}

#[test]
fn test_intel_vtx_hypervisor_backend() {
    let mut vtx = IntelVtxBackend::new().with_hpet(true);
    let config = VmConfig {
        name: "SovereignGuest".to_string(),
        cpu_cores: 4,
        memory_mb: 8192,
        disk_size_gb: 40,
        network_enabled: true,
        gpu_passthrough: false,
        os_type: OsType::Linux,
        cpu_pinning_cores: vec![0, 1, 2, 3],
        hugepages_enabled: true,
        vfio_pci_passthrough_address: None,
        memory_balloon_mb: 4096,
        virtio_net_queues: 4,
        cpu_model: "host".to_string(),
        machine_type: "q35".to_string(),
        nested_virtualization: true,
        io_uring_enabled: true,
        kvm_dirty_ring_size: 4096,
    };

    let vm_id = vtx.create_vm(&config).unwrap();
    assert_eq!(vtx.get_vm_state(&vm_id).unwrap(), VmState::Stopped);

    vtx.start_vm(&vm_id).unwrap();
    assert_eq!(vtx.get_vm_state(&vm_id).unwrap(), VmState::Running);

    vtx.pause_vm(&vm_id).unwrap();
    assert_eq!(vtx.get_vm_state(&vm_id).unwrap(), VmState::Paused);

    vtx.resume_vm(&vm_id).unwrap();
    assert_eq!(vtx.get_vm_state(&vm_id).unwrap(), VmState::Running);

    vtx.stop_vm(&vm_id).unwrap();
    assert_eq!(vtx.get_vm_state(&vm_id).unwrap(), VmState::Stopped);
}

#[test]
fn test_full_vm_manager_lifecycle() {
    let mut manager = VmManager::new(Box::new(QemuBackend::new())).with_auto_start(true);
    assert!(manager.is_auto_start_enabled());

    let config = VmConfig {
        name: "ProductionMicroVM".to_string(),
        cpu_cores: 2,
        memory_mb: 2048,
        disk_size_gb: 20,
        network_enabled: true,
        gpu_passthrough: false,
        os_type: OsType::Linux,
        cpu_pinning_cores: vec![0, 1],
        hugepages_enabled: true,
        vfio_pci_passthrough_address: None,
        memory_balloon_mb: 1024,
        virtio_net_queues: 2,
        cpu_model: "host".to_string(),
        machine_type: "microvm".to_string(),
        nested_virtualization: false,
        io_uring_enabled: true,
        kvm_dirty_ring_size: 2048,
    };

    let vm_id = manager.create_vm(config).unwrap();
    manager.start_vm(&vm_id).unwrap();

    let running = manager.running_vms();
    assert_eq!(running.len(), 1);
    assert_eq!(running[0], vm_id);

    // Test dynamic tuning methods
    manager.set_memory_balloon(&vm_id, 1024).unwrap();
    manager.pin_cpu_cores(&vm_id, vec![2, 3]).unwrap();
    manager.set_virtio_queues(&vm_id, 4).unwrap();
    manager.set_hugepages(&vm_id, true).unwrap();

    let updated_config = manager.get_vm_config(&vm_id).unwrap();
    assert_eq!(updated_config.memory_balloon_mb, 1024);
    assert_eq!(updated_config.cpu_pinning_cores, vec![2, 3]);
    assert_eq!(updated_config.virtio_net_queues, 4);
    assert!(updated_config.hugepages_enabled);

    // Snapshot creation & deletion
    let snap_id = manager.create_snapshot(&vm_id, "backup_1").unwrap();
    assert_eq!(manager.snapshots().len(), 1);
    manager.delete_snapshot(&snap_id).unwrap();
    assert_eq!(manager.snapshots().len(), 0);
}
