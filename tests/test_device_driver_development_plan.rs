// tests/test_device_driver_development_plan.rs
// Standalone integration test for the 15-PR Device Driver Development and Maintenance Plan

#[path = "../src/driver/sovereign_device_driver_development_and_pr_suite.rs"]
mod sovereign_device_driver_development_and_pr_suite;

use sovereign_device_driver_development_and_pr_suite::*;

#[test]
fn test_driver_pr_tracker_all_15_prs() {
    let tracker = DriverPrTracker::new();
    assert_eq!(tracker.prs.len(), 15, "Expected exactly 15 defined PRs");

    for pr_num in 1..=15 {
        let pr = tracker.prs.get(&pr_num).expect("PR definition missing");
        assert_eq!(pr.pr_number, pr_num);
        assert!(!pr.title.is_empty(), "PR title cannot be empty");
        assert!(!pr.main_work.is_empty(), "PR main work cannot be empty");
        assert!(!pr.acceptance_criteria.is_empty(), "PR criteria cannot be empty");
    }

    assert_eq!(
        tracker.prs.get(&1).unwrap().status,
        DriverSupportStatus::Supported
    );
    assert_eq!(
        tracker.prs.get(&15).unwrap().status,
        DriverSupportStatus::Supported
    );
    assert_eq!(tracker.overall_status(), DriverSupportStatus::Supported);
}

#[test]
fn test_status_vocabulary_transitions() {
    let mut tracker = DriverPrTracker::new();

    for num in 1..=15 {
        tracker.set_pr_status(num, DriverSupportStatus::Proposed);
    }
    assert_eq!(tracker.overall_status(), DriverSupportStatus::Proposed);

    tracker.set_pr_status(1, DriverSupportStatus::Prototype);
    assert_eq!(tracker.overall_status(), DriverSupportStatus::Prototype);

    for num in 1..=8 {
        tracker.set_pr_status(num, DriverSupportStatus::Integrated);
    }
    assert_eq!(tracker.overall_status(), DriverSupportStatus::Integrated);

    for num in 1..=15 {
        tracker.set_pr_status(num, DriverSupportStatus::Supported);
    }
    assert_eq!(tracker.overall_status(), DriverSupportStatus::Supported);
}

#[test]
fn test_bus_device_lifecycle_and_rollback() {
    let mut bus = BusDeviceLifecycleGovernor::new();

    let dev_desc = DeviceDescriptor {
        device_key: "pci_0000_00_02_0".to_string(),
        bus_type: BusType::Pci,
        id: DeviceId {
            vendor_id: 0x1AF4,
            device_id: 0x1001,
            subvendor_id: 0,
            subdevice_id: 0,
            class_code: 1,
            subclass_code: 0,
        },
        resources: vec![DeviceResource {
            resource_id: 0,
            start_address: 0xFE000000,
            size: 0x1000,
            is_mmio: true,
            is_allocated: false,
        }],
        irq_line: 11,
        is_bound: false,
        bound_driver_name: None,
    };

    bus.register_device(dev_desc).unwrap();

    let drv_reg = DriverRegistration {
        driver_name: "virtio_blk_driver".to_string(),
        supported_bus: BusType::Pci,
        supported_devices: vec![DeviceId {
            vendor_id: 0x1AF4,
            device_id: 0x1001,
            subvendor_id: 0,
            subdevice_id: 0,
            class_code: 1,
            subclass_code: 0,
        }],
        is_active: true,
    };

    bus.register_driver(drv_reg).unwrap();

    let bound_drv = bus.match_and_probe("pci_0000_00_02_0").unwrap();
    assert_eq!(bound_drv, "virtio_blk_driver");
    assert!(bus.devices.get("pci_0000_00_02_0").unwrap().is_bound);

    bus.detach_device("pci_0000_00_02_0").unwrap();
    assert!(!bus.devices.get("pci_0000_00_02_0").unwrap().is_bound);

    // Test probe failure rollback logging
    assert!(bus.probe_with_injected_failure("pci_0000_00_02_0", 2).is_err());
    assert!(!bus.probe_failure_rollback_log.is_empty());
}

#[test]
fn test_platform_dma_api_governor() {
    let mut dma = PlatformDmaApiGovernor::new(32);
    let virt = CpuVirtualAddress(0xFFFF_8000_1000_0000);
    let phys = CpuPhysicalAddress(0x0000_0000_8000_0000);

    let handle = dma
        .map_dma_buffer(virt, phys, 4096, DmaDirection::Bidirectional)
        .expect("DMA mapping failed");

    assert_eq!(handle.dma_addr, DeviceDmaAddress(0x8000_0000));
    assert!(dma.sync_dma_for_device(handle.mapping_id).is_ok());
    assert!(dma.sync_dma_for_cpu(handle.mapping_id).is_ok());
    assert_eq!(dma.barrier_sync_count, 2);

    assert!(dma.unmap_dma_buffer(handle.mapping_id).is_ok());
}

#[test]
fn test_interrupt_and_deferred_work_contract() {
    let mut irq = InterruptAndDeferredWorkContract::new();

    irq.register_irq(10, "virtio_net", true).unwrap();
    irq.register_irq(10, "virtio_blk", true).unwrap();

    let count = irq.dispatch_irq(10).unwrap();
    assert_eq!(count, 2, "Both shared handlers should be dispatched");

    let task1 = irq.defer_work_from_irq("virtio_net", "rx_refill");
    let task2 = irq.defer_work_from_irq("virtio_blk", "io_completion");
    assert_eq!(task1, 1);
    assert_eq!(task2, 2);

    let executed = irq.process_deferred_work();
    assert_eq!(executed, 2);

    assert!(irq.unregister_irq(10, "virtio_net").is_ok());
}

#[test]
fn test_pci_enumeration_and_resource_governor() {
    let mut pci = PciEnumerationAndResourceGovernor::new();
    pci.scan_qemu_pci_bus();

    assert_eq!(pci.discovered_devices.len(), 4);
    assert!(pci.discovered_devices.iter().any(|d| d.device_id == 0x1001)); // VirtIO Blk
    assert!(pci.discovered_devices.iter().any(|d| d.device_id == 0x1000)); // VirtIO Net
    assert!(!pci.allocated_mmio_ranges.is_empty());
}

#[test]
fn test_virtio_integrations_and_master_suite() {
    let suite = SovereignDeviceDriverDevelopmentAndPrMasterSuite::new();

    assert!(suite.health_check());
    assert_eq!(suite.virtio_blk.state, VirtIoDeviceState::Active);
    assert_eq!(suite.virtio_net.state, VirtIoDeviceState::Active);

    let summary = suite.summary_report();
    assert!(summary.contains("Defined Driver PRs: 15"));
    assert!(summary.contains("Overall Support Status: Supported"));
    assert!(summary.contains("Discovered PCI Devices: 4"));
}
