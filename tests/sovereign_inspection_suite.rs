// SigmaOS Sovereign System Inspection & Open Source Parity Test Suite
// Inspects and verifies advanced subsystem mechanisms inspired by Linux, FreeBSD, OpenBSD, NetBSD, QEMU/KVM:
// - FreeBSD Capsicum & Jails VNET
// - OpenBSD Pledge/Unveil Sandboxing
// - Linux Btrfs subvolumes & incremental send/receive
// - PCIe ECAM configuration space & BAR decoding
// - PackageSnapshotRollbackEngine pre/post transaction rollbacks
// - QEMU/KVM Qcow2 image overlays, vCPU context & VirtIO virtqueue ring buffers
// - Classic OS Algorithms (VirtIO Ballooning, Banker's Algorithm, Sleeping Barber, Ticket Spinlocks, Stack Canaries, Batch Queue)

#[path = "../src/kernel/pci_scanner.rs"]
mod pci_scanner;

#[path = "../src/sigpkg/transaction.rs"]
mod transaction;

#[path = "../src/virt/mod.rs"]
mod virt;

#[path = "../src/fs/btrfs.rs"]
mod btrfs;

#[path = "../src/security/securelevels.rs"]
mod securelevels;

#[path = "../src/security/jails.rs"]
mod jails;

#[path = "../src/kernel/classic_os.rs"]
mod classic_os;

use btrfs::*;
use classic_os::*;
use jails::*;
use pci_scanner::*;
use securelevels::*;
use transaction::*;
use virt::*;

#[test]
fn test_inspection_pcie_ecam_and_bar_decoder() {
    let ecam = PcieEcamManager::new(0xE000_0000);
    let dev_addr = ecam.calculate_function_offset(0, 1, 0);
    // Base 0xE000_0000 + Slot 1 (32KB offset 0x8000) = 0xE000_8000
    assert_eq!(dev_addr, 0xE000_0000 + 0x8000);

    let mut dev = PciDevice::new(0, 1, 0x8086, 0x10D3, 0x02);
    dev.decode_bar(0, 0xFE00_000C, Some(0x0000_0001), 65536);
    assert_eq!(dev.bars.len(), 1);
    assert_eq!(dev.bars[0].address, 0x0000_0001_FE00_0000);
    assert_eq!(
        dev.bars[0].bar_type,
        BarType::Memory64 { prefetchable: true }
    );

    dev.add_capability(0x11, 0x60); // MSI-X
    assert_eq!(dev.capabilities.len(), 1);
    assert_eq!(dev.capabilities[0].id, PciCapabilityId::MsiX);
}

#[test]
fn test_inspection_package_snapshot_rollback() {
    let mut rollback_engine = PackageSnapshotRollbackEngine::new();
    let mut current_pkgs = vec![
        ("glibc".to_string(), "2.38".to_string()),
        ("bash".to_string(), "5.2".to_string()),
    ];

    let snap_id = rollback_engine.create_pre_transaction_snapshot(
        "Before system upgrade",
        &[("glibc", "2.38"), ("bash", "5.2")],
    );
    assert_eq!(snap_id, 1);

    // Upgrade system
    current_pkgs[1].1 = "5.3".to_string();
    current_pkgs.push(("curl".to_string(), "8.2.1".to_string()));

    // Perform atomic rollback
    assert!(rollback_engine
        .rollback_to_snapshot(&mut current_pkgs, snap_id)
        .is_ok());
    assert_eq!(current_pkgs.len(), 2);
    assert_eq!(current_pkgs[0].0, "glibc");
    assert_eq!(current_pkgs[1].1, "5.2");
}

#[test]
fn test_inspection_qemu_kvm_virt_enhancements() {
    let mut vm = EnhancedVirtualMachine::new("sigma-kvm-guest", 4, 8192);
    assert_eq!(vm.vcpus.len(), 4);

    vm.attach_qcow2_overlay("base_disk.qcow2", "overlay_disk.qcow2");
    assert!(vm.qcow2_overlay.is_some());

    let overlay = vm.qcow2_overlay.as_mut().unwrap();
    overlay.allocate_cluster(0x2000);
    assert!(overlay.is_cluster_allocated(0x2000));

    vm.attach_vfio_device(1, "0000:02:00.0");
    assert_eq!(vm.vfio_devices.len(), 1);

    assert!(vm.start().is_ok());
    assert_eq!(vm.state, VMState::Running);
    assert_eq!(vm.vcpus[0].exit_reason, 3); // KVM_EXIT_HLT
}

#[test]
fn test_inspection_btrfs_subvolume_send_receive() {
    let mut fs = BtrfsFilesystem::new();
    let sub_id = fs.create_subvolume("home".to_string(), None).unwrap();
    assert!(sub_id >= 1);

    let stream = fs.send_subvolume(sub_id).unwrap();
    assert!(!stream.is_empty());

    let rec_id = fs.receive_subvolume(&stream).unwrap();
    assert!(rec_id >= 1);
}

#[test]
fn test_inspection_freebsd_securelevels() {
    let mgr = SovereignSecurelevelManager::new();
    assert_eq!(mgr.securelevel(), Securelevel::Permissive);
}

#[test]
fn test_inspection_freebsd_jails_and_vnet() {
    let mut mgr = JailManager::new();
    let jid = mgr.spawn_jail(
        "isolated_web_jail",
        "/vfs/jails/web",
        "web_jail.sigmaos.org",
        vec!["192.168.1.50".to_string()],
        JailCapabilities::secure_default(),
    );

    assert_eq!(jid, 1);
    let jail = mgr.active_jails.iter().find(|j| j.jid == jid).unwrap();
    assert_eq!(jail.name, "isolated_web_jail");
    assert!(!jail.capabilities.allow_mounting);
}

#[test]
fn test_inspection_classic_os_algorithms() {
    // 1. VirtIO Memory Ballooning
    let mut balloon = VirtioBalloonManager::new(10);
    balloon.set_target_pages(15);
    let inflated = balloon.inflate(&[100, 101, 102]);
    assert_eq!(inflated, 3);
    assert_eq!(balloon.current_pages(), 13);

    // 2. Banker's Algorithm (Deadlock Avoidance)
    let available = vec![3, 3, 2];
    let max_matrix = vec![vec![3, 3, 2], vec![3, 2, 2]];
    let allocation = vec![vec![0, 1, 0], vec![2, 0, 0]];
    let mut banker = BankersAlgorithm::new(2, 3, available, max_matrix, allocation);
    assert!(banker.is_safe_state());
    assert!(banker.request_resources(1, &[1, 0, 2]));

    // 3. Sleeping Barber Synchronization
    let mut barber = SleepingBarberQueue::new(2);
    assert!(barber.is_barber_sleeping());
    assert!(barber.customer_arrives(1)); // Served immediately
    assert!(!barber.is_barber_sleeping());
    assert!(barber.customer_arrives(2));
    assert_eq!(barber.service_next_customer(), Some(2));

    // 4. Ticket Spinlock with Exponential Backoff
    let spinlock = TicketSpinlock::new();
    let ticket = spinlock.lock();
    spinlock.unlock(ticket);

    // 5. Stack Canary Protection
    let protector = StackCanaryProtector::new(0x1234_5678_9ABC_DEF0);
    let canary = protector.generate_canary();
    assert!(protector.verify_canary(canary));
    assert!(!protector.verify_canary(canary ^ 0xFF));

    // 6. Multiprogrammed Batch Queue Processor
    let mut batch_queue = BatchSystemQueue::new(2);
    batch_queue.submit_job(BatchJob {
        job_id: 1,
        priority: 1,
        estimated_time_ms: 100,
    });
    batch_queue.submit_job(BatchJob {
        job_id: 2,
        priority: 2,
        estimated_time_ms: 200,
    });
    assert_eq!(batch_queue.running_count(), 2);
    assert!(batch_queue.complete_job(1));
}
