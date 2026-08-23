// SigmaOS Sovereign System Inspection & Open Source Parity Test Suite
// Inspects and verifies advanced subsystem mechanisms inspired by Linux, FreeBSD, OpenBSD, NetBSD, QEMU/KVM:
// - FreeBSD Capsicum & Jails VNET
// - OpenBSD Pledge/Unveil Sandboxing
// - Linux Btrfs subvolumes & incremental send/receive
// - PCIe ECAM configuration space & BAR decoding
// - PackageSnapshotRollbackEngine pre/post transaction rollbacks
// - QEMU/KVM Qcow2 image overlays, vCPU context & VirtIO virtqueue ring buffers

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

use pci_scanner::*;
use transaction::*;
use virt::*;
use btrfs::*;
use securelevels::*;
use jails::*;

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
    assert_eq!(dev.bars[0].bar_type, BarType::Memory64 { prefetchable: true });

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
    assert!(rollback_engine.rollback_to_snapshot(&mut current_pkgs, snap_id).is_ok());
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
