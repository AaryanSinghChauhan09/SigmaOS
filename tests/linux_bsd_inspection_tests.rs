// SigmaOS Linux & BSD Parity Inspection Unit Test Suite
// Verifies working mechanisms inspired by Linux and BSD distributions:
// - FreeBSD Jails & sysctl MIB
// - NetBSD Rump Kernel hypercall routing
// - OpenBSD sysctl MIB
// - KVM/QEMU vCPU execution loop & VirtIO device rings
// - OpenBSD Pledge & Unveil sandboxing
// - Gentoo Portage USE-flag dependency solver
// - CachyOS BORE interactive scheduler

#[path = "../src/compatibility/bsd.rs"]
mod bsd;

#[path = "../src/virtualization/kvm_vcpu.rs"]
mod kvm_vcpu;

#[path = "../src/security/unveil.rs"]
mod unveil;

#[path = "../src/compatibility/gap_closure.rs"]
mod gap_closure;

#[path = "../src/virtualization/vm_manager.rs"]
mod vm_manager;

#[path = "../src/scheduler/eevdf.rs"]
mod eevdf;

#[path = "../src/memory/tlb_associative.rs"]
mod tlb_associative;

#[path = "../src/virtualization/vm_manager.rs"]
mod vm_manager;

#[path = "../src/desktop/zenith_advanced_features.rs"]
mod zenith_advanced;

#[path = "../src/scheduler/eevdf.rs"]
mod eevdf;

#[path = "../src/memory/tlb_associative.rs"]
mod tlb_associative;

use bsd::*;
use gap_closure::{ZorinAppearanceSwitcher, ZorinLayoutPreset};
use kvm_vcpu::{KvmExitCode, KvmVcpu, VirtioDeviceBackend, VirtioDeviceType, RAX_HLT_SIGNAL};
use unveil::{UnveilManager, UnveilPermission};

#[test]
fn test_freebsd_jail_manager_inspection() {
    let mut mgr = FreeBsdJailManager::new();
    let jail_id = mgr.create_jail("secure_web_jail", "192.168.1.100", "/vfs/jails/web").unwrap();
    assert_eq!(jail_id, 1);

    let jail_ref = mgr.jails.get(&jail_id).unwrap();
    assert_eq!(jail_ref.hostname, "secure_web_jail");
    assert_eq!(jail_ref.ip_address, "192.168.1.100");
}

#[test]
fn test_netbsd_rump_router_inspection() {
    let res = NetBsdRumpKernelRouter::dispatch_hypercall(RumpHypercall::Syscall, 100);
    assert_eq!(res, 101);

    let res_mem = NetBsdRumpKernelRouter::dispatch_hypercall(RumpHypercall::MemoryAlloc, 5000);
    assert_eq!(res_mem, 8192);
}

#[test]
fn test_openbsd_sysctl_mib_inspection() {
    let mut mib = OpenBsdSysctlKernelMib::new();
    assert!(mib.write_mib("kern.ostype", "SigmaOS-OpenBSD").is_ok());
    assert!(mib.write_mib("kern.securelevel", "1").is_ok());

    assert_eq!(mib.query_mib("kern.ostype").unwrap(), "SigmaOS-OpenBSD");
    assert_eq!(mib.query_mib("kern.securelevel").unwrap(), "1");
    assert!(!mib.is_raw_disk_write_allowed());
}

#[test]
fn test_kvm_qemu_vcpu_inspection() {
    let mut vcpu = KvmVcpu::new(0);
    vcpu.registers.rax = RAX_HLT_SIGNAL;
    let exit = vcpu.run_vcpu_step();
    assert_eq!(exit, KvmExitCode::ExitHlt);

    let mut virtio_net = VirtioDeviceBackend::new(VirtioDeviceType::Network);
    let processed = virtio_net.process_virtqueue_ring();
    assert_eq!(processed, 16);
}

#[test]
fn test_openbsd_unveil_inspection() {
    let mut unveil = UnveilManager::new();
    assert!(unveil.unveil("/var/log", "r").is_ok());
    assert!(unveil.validate_path("/var/log/syslog", UnveilPermission::Read).is_ok());
    assert!(unveil.validate_path("/var/log/syslog", UnveilPermission::Write).is_err());
}

#[test]
fn test_zorin_gap_closure_inspection() {
    let mut zorin = ZorinAppearanceSwitcher::new();
    zorin.switch_layout_preset(ZorinLayoutPreset::MacOsLike);
    assert_eq!(zorin.panel_height_pixels, 64);
}


#[test]
fn test_vm_manager_kvm_qemu_inspection() {
    use vm_manager::{KvmHypervisor, VmConfig, OsType, VmState, KvmExitReason, VirtioBlockDeviceConfig, VirtioNetDeviceConfig, HypervisorBackend};
    use std::path::PathBuf;

    let mut kvm = KvmHypervisor::new();
    assert_eq!(kvm.name(), "KVM/QEMU Hardware Virtualization");
    assert!(kvm.capabilities().irqchip_supported);

    let config = VmConfig {
        name: "KVM Inspection VM".to_string(),
        cpu_cores: 2,
        memory_mb: 4096,
        disk_size_gb: 40,
        network_enabled: true,
        gpu_passthrough: false,
        os_type: OsType::Linux,
        cpu_pinning_cores: vec![0, 1],
        hugepages_enabled: true,
        vfio_pci_passthrough_address: None,
        memory_balloon_mb: 1024,
        virtio_net_queues: 2,
        cpu_model: "host".to_string(),
        machine_type: "q35".to_string(),
        nested_virtualization: true,
        io_uring_enabled: true,
        kvm_dirty_ring_size: 2048,
    };

    let vm_id = kvm.create_vm(&config).unwrap();
    assert_eq!(kvm.get_vm_state(&vm_id).unwrap(), VmState::Stopped);

    kvm.attach_virtio_blk(&vm_id, VirtioBlockDeviceConfig {
        image_path: PathBuf::from("/var/lib/images/rootfs.qcow2"),
        read_only: false,
        direct_io: true,
        queue_size: 256,
        block_size: 512,
    }).unwrap();

    kvm.attach_virtio_net(&vm_id, VirtioNetDeviceConfig {
        mac_address: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
        tap_interface: "tap0".to_string(),
        queues: 2,
        offload_tso: true,
        offload_csum: true,
    }).unwrap();

    kvm.start_vm(&vm_id).unwrap();
    assert_eq!(kvm.get_vm_state(&vm_id).unwrap(), VmState::Running);

    let exit = kvm.run_vcpu(&vm_id, 0).unwrap();
    assert_eq!(exit, KvmExitReason::Hlt);

    kvm.stop_vm(&vm_id).unwrap();
    assert_eq!(kvm.get_vm_state(&vm_id).unwrap(), VmState::Stopped);
}


#[test]
fn test_kernel_classic_algorithms_inspection() {
    use eevdf::{EevdfScheduler, Task, ComputeUnit};
    use tlb_associative::{AssociativeTlbCache, TlbAssociativityMode, TlbPageFlags};

    let mut sched = EevdfScheduler::new();
    let mut task = Task::new(1, 100, 10);
    task.assign_compute_unit(ComputeUnit::CpuCore(0));
    sched.add_task(task);
    assert_eq!(sched.ready_count(), 1);

    let scheduled = sched.schedule();
    assert_eq!(scheduled, Some(1));

    let mut tlb = AssociativeTlbCache::new(TlbAssociativityMode::FullyAssociative, 16);
    tlb.insert_translation(0x10, 0x50, TlbPageFlags::rw_user(), 1);
    let translated = tlb.lookup_page_translation(0x10, 1, false, false);
    assert_eq!(translated, Ok(0x50));
    assert_eq!(tlb.get_hit_ratio_pct(), 100.0);
}


#[test]
fn test_kvm_qemu_vcpu_inspection() {
    use vm_manager::{KvmHypervisor, VmConfig, OsType, VmState, KvmExitReason, VirtioBlockDeviceConfig, VirtioNetDeviceConfig, HypervisorBackend};
    use std::path::PathBuf;

    let mut kvm = KvmHypervisor::new();
    assert_eq!(kvm.name(), "KVM/QEMU Hardware Virtualization");
    assert!(kvm.capabilities().irqchip_supported);

    let config = VmConfig {
        name: "KVM Inspection VM".to_string(),
        cpu_cores: 2,
        memory_mb: 4096,
        disk_size_gb: 40,
        network_enabled: true,
        gpu_passthrough: false,
        os_type: OsType::Linux,
        cpu_pinning_cores: vec![0, 1],
        hugepages_enabled: true,
        vfio_pci_passthrough_address: None,
        memory_balloon_mb: 1024,
        virtio_net_queues: 2,
        cpu_model: "host".to_string(),
        machine_type: "q35".to_string(),
        nested_virtualization: true,
        io_uring_enabled: true,
        kvm_dirty_ring_size: 2048,
    };

    let vm_id = kvm.create_vm(&config).unwrap();
    assert_eq!(kvm.get_vm_state(&vm_id).unwrap(), VmState::Stopped);

    kvm.attach_virtio_blk(&vm_id, VirtioBlockDeviceConfig {
        image_path: PathBuf::from("/var/lib/images/rootfs.qcow2"),
        read_only: false,
        direct_io: true,
        queue_size: 256,
        block_size: 512,
    }).unwrap();

    kvm.attach_virtio_net(&vm_id, VirtioNetDeviceConfig {
        mac_address: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
        tap_interface: "tap0".to_string(),
        queues: 2,
        offload_tso: true,
        offload_csum: true,
    }).unwrap();

    kvm.start_vm(&vm_id).unwrap();
    assert_eq!(kvm.get_vm_state(&vm_id).unwrap(), VmState::Running);

    let exit = kvm.run_vcpu(&vm_id, 0).unwrap();
    assert_eq!(exit, KvmExitReason::Hlt);

    kvm.stop_vm(&vm_id).unwrap();
    assert_eq!(kvm.get_vm_state(&vm_id).unwrap(), VmState::Stopped);
}


#[test]
fn test_kernel_classic_algorithms_inspection() {
    use eevdf::{EevdfScheduler, Task, ComputeUnit};
    use tlb_associative::{AssociativeTlbCache, TlbAssociativityMode, TlbPageFlags};

    let mut sched = EevdfScheduler::new();
    let mut task = Task::new(1, 100, 10);
    task.assign_compute_unit(ComputeUnit::CpuCore(0));
    sched.add_task(task);
    assert_eq!(sched.ready_count(), 1);

    let scheduled = sched.schedule();
    assert_eq!(scheduled, Some(1));

    let mut tlb = AssociativeTlbCache::new(TlbAssociativityMode::FullyAssociative, 16);
    tlb.insert_translation(0x10, 0x50, TlbPageFlags::rw_user(), 1);
    let translated = tlb.lookup_page_translation(0x10, 1, false, false);
    assert_eq!(translated, Ok(0x50));
    assert_eq!(tlb.get_hit_ratio_pct(), 100.0);
}


#[test]
fn test_zenith_desktop_applets_and_themes_inspection() {
    use zenith_advanced::{DesktopAppletEngine, DesktopApplet, AppletCategory, ZenithThemePresetManager, ZenithThemePreset};

    let mut applet_engine = DesktopAppletEngine::new();
    assert_eq!(applet_engine.get_active_applets().len(), 3);

    applet_engine.register_applet(DesktopApplet {
        id: "battery".to_string(),
        name: "Battery & Power".to_string(),
        category: AppletCategory::PowerBattery,
        enabled: true,
        position_index: 3,
    });
    assert_eq!(applet_engine.get_active_applets().len(), 4);

    let mut theme_mgr = ZenithThemePresetManager::new();
    assert_eq!(theme_mgr.current_preset, ZenithThemePreset::CinnamonModern);

    theme_mgr.apply_preset(ZenithThemePreset::PantheonGranite);
    assert_eq!(theme_mgr.current_preset, ZenithThemePreset::PantheonGranite);
    assert_eq!(theme_mgr.accent_color_hex, "#3852A4");
}
