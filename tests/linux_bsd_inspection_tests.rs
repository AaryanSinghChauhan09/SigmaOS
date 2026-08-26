// SigmaOS Linux & BSD Parity Inspection Unit Test Suite
// Verifies working mechanisms inspired by Linux and BSD distributions:
// - FreeBSD Jails & sysctl MIB
// - NetBSD Rump Kernel hypercall routing
// - OpenBSD sysctl MIB
// - KVM/QEMU vCPU execution loop & VirtIO device rings
// - OpenBSD Pledge & Unveil sandboxing
// - Gentoo Portage USE-flag dependency solver
// - CachyOS BORE interactive scheduler
// - Linux XDP & Netmap fast packet engine
// - Landlock VFS & OpenBSD Pledge access controller
// - ULE & EEVDF/BORE interactive hybrid scheduler
// - HAMMER2 PFS & Btrfs CoW storage engine
// - Linux memory compaction & 2MB superpages allocator
// - Alpine Linux APK Package Index parser & signature verifier
// - DragonFly BSD HAMMER2 PFS snapshot & replication engine
// - NixOS Declarative System Configuration & generation rollback switcher

#[path = "../src/compatibility/bsd.rs"]
mod bsd;

#[path = "../src/virtualization/kvm_vcpu.rs"]
mod kvm_vcpu;

#[path = "../src/security/unveil.rs"]
mod unveil;

#[path = "../src/compatibility/gap_closure.rs"]
mod gap_closure;

#[path = "../src/unimplemented_features.rs"]
mod unimplemented_features;

#[path = "../src/virtualization/vm_manager.rs"]
mod vm_manager;

#[path = "../src/scheduler/eevdf.rs"]
mod eevdf;

#[path = "../src/memory/tlb_associative.rs"]
mod tlb_associative;

#[path = "../src/desktop/zenith_advanced_features.rs"]
mod zenith_advanced;

#[path = "../src/kernel/linux_bsd_innovations.rs"]
mod linux_bsd_innovations;

#[path = "../src/boot/firmware.rs"]
mod firmware;

use bsd::*;
use gap_closure::{ZorinAppearanceSwitcher, ZorinLayoutPreset};
use unimplemented_features::{
    AlpineApkPackageIndex, ApkPackageEntry, DragonFlyHammer2FsSnapshot,
    NixOsDeclarativeConfigEngine,
};
use kvm_vcpu::{KvmExitCode, KvmVcpu, VirtioDeviceBackend, VirtioDeviceType, RAX_HLT_SIGNAL};
use unveil::{UnveilManager, UnveilPermission};

#[test]
fn test_freebsd_jail_manager_inspection() {
    let mut mgr = FreeBsdJailManager::new();
    let jail_id = mgr
        .create_jail("secure_web_jail", "192.168.1.100", "/vfs/jails/web")
        .unwrap();
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
    assert!(unveil
        .validate_path("/var/log/syslog", UnveilPermission::Read)
        .is_ok());
    assert!(unveil
        .validate_path("/var/log/syslog", UnveilPermission::Write)
        .is_err());
}

#[test]
fn test_zorin_gap_closure_inspection() {
    let mut zorin = ZorinAppearanceSwitcher::new();
    zorin.switch_layout_preset(ZorinLayoutPreset::MacOsLike);
    assert_eq!(zorin.panel_height_pixels, 64);
}

#[test]
fn test_vm_manager_kvm_qemu_inspection() {
    use vm_manager::{
        HypervisorBackend, KvmExitReason, KvmHypervisor, OsType, VirtioBlockDeviceConfig,
        VirtioNetDeviceConfig, VmConfig, VmState,
    };

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

    kvm.attach_virtio_blk(
        &vm_id,
        VirtioBlockDeviceConfig {
            image_path: "/var/lib/images/rootfs.qcow2".to_string(),
            read_only: false,
            direct_io: true,
            queue_size: 256,
            block_size: 512,
        },
    )
    .unwrap();

    kvm.attach_virtio_net(
        &vm_id,
        VirtioNetDeviceConfig {
            mac_address: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
            tap_interface: "tap0".to_string(),
            queues: 2,
            offload_tso: true,
            offload_csum: true,
        },
    )
    .unwrap();

    kvm.start_vm(&vm_id).unwrap();
    assert_eq!(kvm.get_vm_state(&vm_id).unwrap(), VmState::Running);

    let exit = kvm.run_vcpu(&vm_id, 0).unwrap();
    assert_eq!(exit, KvmExitReason::Hlt);

    kvm.stop_vm(&vm_id).unwrap();
    assert_eq!(kvm.get_vm_state(&vm_id).unwrap(), VmState::Stopped);
}

#[test]
fn test_kernel_classic_algorithms_inspection() {
    use eevdf::{ComputeUnit, EevdfScheduler, Task};
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
    use zenith_advanced::{
        AppletCategory, DesktopApplet, DesktopAppletEngine, ZenithThemePreset,
        ZenithThemePresetManager,
    };

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


#[path = "../src/network/protocols.rs"]
mod protocols;

#[path = "../src/security/hardening.rs"]
mod hardening;

#[test]
fn test_sovereign_linux_bsd_kernel_innovations_inspection() {
    use linux_bsd_innovations::{
        KernelFastPacketEngine, FastPacketFrame, XdpAction,
        KernelAccessController, LandlockAccessRight, PLEDGE_STDIO, PLEDGE_RPATH, PLEDGE_EXEC,
        InteractiveHybridScheduler, HybridTask,
        CowStorageEngine, MemoryCompactionSuperpagesAllocator,
    };

    // 1. Fast packet ring engine
    let mut packet_engine = KernelFastPacketEngine::new(10);
    packet_engine.enqueue_rx(FastPacketFrame {
        id: 100,
        payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
        rx_timestamp_ns: 1_000_000,
        ingress_ifindex: 1,
    }).unwrap();
    let processed = packet_engine.process_xdp_filter(|frame| {
        if frame.id == 100 {
            XdpAction::Pass
        } else {
            XdpAction::Drop
        }
    });
    assert_eq!(processed, 1);
    assert_eq!(packet_engine.pass_count, 1);

    // 2. Access Controller (Landlock & Pledge)
    let mut access_ctrl = KernelAccessController::new();
    access_ctrl.add_path_rule("/usr/bin", vec![LandlockAccessRight::Read, LandlockAccessRight::Execute]);
    access_ctrl.restrict_pledge(PLEDGE_STDIO | PLEDGE_RPATH);
    assert!(access_ctrl.check_path_access("/usr/bin/cargo", LandlockAccessRight::Execute).is_ok());
    assert!(access_ctrl.check_pledge(PLEDGE_STDIO).is_ok());
    assert!(access_ctrl.check_pledge(PLEDGE_EXEC).is_err());

    // 3. Interactive Hybrid Scheduler
    let mut hybrid_sched = InteractiveHybridScheduler::new();
    let mut t1 = HybridTask::new(1, 10);
    t1.sleep_time_ms = 80;
    t1.cpu_time_ms = 20;
    hybrid_sched.add_task(t1);
    let sched_pid = hybrid_sched.schedule_next().unwrap();
    assert_eq!(sched_pid, 1);

    // 4. CoW Storage Engine
    let mut cow = CowStorageEngine::new();
    let blk_id = cow.write_block(b"Kernel Data Block");
    assert!(cow.verify_block_integrity(blk_id).unwrap());
    let snap_id = cow.create_pfs_snapshot("PFS_SNAP_01", blk_id);
    assert_eq!(snap_id, 1);

    // 5. Memory Compaction Superpages Allocator
    let mut mem_alloc = MemoryCompactionSuperpagesAllocator::new(1024);
    assert_eq!(mem_alloc.compact_free_frames(), 1024);
    let pfn = mem_alloc.allocate_2mb_superpage().unwrap();
    assert_eq!(pfn, 0);
}

#[test]
fn test_alpine_apk_package_index_inspection() {
    use unimplemented_features::{AlpineApkPackageIndex, ApkPackageEntry};

    let mut apk_index = AlpineApkPackageIndex::new();
    let key = [0x99; 32];
    assert!(apk_index.verify_index_signature(&key));

    apk_index.add_package(ApkPackageEntry {
        name: "openssl".to_string(),
        version: "3.1.0".to_string(),
        arch: "x86_64".to_string(),
        sha256_hash: [0xAB; 32],
        dependencies: vec!["musl".to_string()],
    });

    let pkg = apk_index.find_package("openssl").unwrap();
    assert_eq!(pkg.version, "3.1.0");
    assert_eq!(apk_index.resolve_dependencies("openssl"), vec!["musl"]);
}

#[test]
fn test_dragonfly_hammer2_snapshot_inspection() {
    use unimplemented_features::DragonFlyHammer2FsSnapshot;

    let mut hammer2 = DragonFlyHammer2FsSnapshot::new();
    hammer2.register_cluster_node(1, "192.168.1.50");

    let snap_id = hammer2.create_pfs_snapshot("ROOT_PFS", 0x1234567887654321, 1680000000);
    assert_eq!(snap_id, 1);
    assert!(hammer2.replicate_snapshot_to_node(snap_id, 1).is_ok());

    let rolled_back_merkle = hammer2.rollback_pfs("ROOT_PFS", snap_id).unwrap();
    assert_eq!(rolled_back_merkle, 0x1234567887654321);
}

#[test]
fn test_nixos_declarative_config_engine_inspection() {
    use unimplemented_features::NixOsDeclarativeConfigEngine;

    let mut nix_engine = NixOsDeclarativeConfigEngine::new();
    let gen1 = nix_engine.build_generation(0xDEADBEEF, 1680000000, 150, "quiet splash");
    assert_eq!(gen1, 1);

    let gen2 = nix_engine.build_generation(0xCAFEBABE, 1680001000, 155, "debug");
    assert_eq!(gen2, 2);
    assert_eq!(nix_engine.active_generation, 2);

    let rolled_back = nix_engine.rollback_generation().unwrap();
    assert_eq!(rolled_back.gen_number, 1);
    assert_eq!(nix_engine.active_generation, 1);
}

#[test]
fn test_linux_bsd_firmware_innovations_inspection() {
    use firmware::{
        EfiVariableStore, CpuMicrocodePatchEngine, MicrocodeVendor,
        FirmwareCapsuleUpdateManager, CapsuleUpdateStatus, SmbiosFirmwareParser,
        IommuFirmwareEngine, IommuArchitecture, EFI_GLOBAL_VARIABLE_GUID,
    };

    // 1. UEFI NVRAM Variable Management (Linux efivarfs & FreeBSD efivar(8))
    let mut efivars = EfiVariableStore::new();
    assert!(efivars.get_variable("BootOrder", EFI_GLOBAL_VARIABLE_GUID).is_some());
    efivars.set_variable("FastBoot", EFI_GLOBAL_VARIABLE_GUID, 7, &[0x01]);
    assert_eq!(efivars.get_variable("FastBoot", EFI_GLOBAL_VARIABLE_GUID).unwrap().data, &[0x01]);
    let manifest = efivars.export_efivarfs_manifest();
    assert!(manifest.contains("BootOrder"));

    // 2. CPU Microcode Patch Engine (Intel/AMD ucode & FreeBSD cpuctl(4))
    let mut ucode_engine = CpuMicrocodePatchEngine::new(2);
    let mut raw_intel = vec![0u8; 48];
    raw_intel[0..4].copy_from_slice(&1u32.to_le_bytes());
    raw_intel[4..8].copy_from_slice(&0x000000B0u32.to_le_bytes());
    raw_intel[32..36].copy_from_slice(&2048u32.to_le_bytes());

    let intel_hdr = ucode_engine.parse_intel_header(&raw_intel).unwrap();
    assert_eq!(intel_hdr.vendor, MicrocodeVendor::Intel);
    assert!(ucode_engine.apply_microcode_update(0, intel_hdr));
    assert_eq!(ucode_engine.get_core_patch_level(0), Some(0x000000B0));

    // 3. FWUPD / UEFI ESRT Capsule Manager
    let mut capsule_mgr = FirmwareCapsuleUpdateManager::new();
    let mut capsule_payload = vec![0u8; 32];
    capsule_payload[0..11].copy_from_slice(b"CAPSULE_SIG");
    capsule_payload[12..16].copy_from_slice(&0x02010000u32.to_le_bytes());

    let sys_guid = "3b61b360-1e5b-4227-b50a-8d184713e2f5";
    assert!(capsule_mgr.stage_capsule_payload(sys_guid, &capsule_payload).is_ok());
    assert_eq!(capsule_mgr.current_status, CapsuleUpdateStatus::Staged);
    assert!(capsule_mgr.process_reboot_capsules());
    assert_eq!(capsule_mgr.current_status, CapsuleUpdateStatus::UpdateSuccess);

    // 4. SMBIOS / DMI Firmware Parser
    let mut smbios = SmbiosFirmwareParser::new();
    assert!(smbios.parse_smbios_entry_point(b"_SM_123456789012"));
    assert_eq!(smbios.bios_info.unwrap().vendor, "SigmaOS Sovereign Core UEFI");

    // 5. IOMMU ACPI DMAR / IVRS Controller
    let mut iommu = IommuFirmwareEngine::new();
    let mut dmar_header = vec![0u8; 40];
    dmar_header[0..4].copy_from_slice(b"DMAR");
    assert!(iommu.parse_acpi_dmar(&dmar_header));
    assert_eq!(iommu.architecture, IommuArchitecture::IntelVtD);
    assert!(iommu.is_preboot_dma_protected);
}

#[test]
fn test_bgp_routing_table_manager_inspection() {
    use protocols::{BgpRoutingTableManager, BgpRoutePrefix};

    let mut bgp_mgr = BgpRoutingTableManager::new(65001, [10, 0, 0, 1], true);
    bgp_mgr.advertise_prefix([192, 168, 1, 0], 24, [10, 0, 0, 1], 100);
    assert_eq!(bgp_mgr.routes.len(), 1);

    let incoming = BgpRoutePrefix {
        prefix_ip: [192, 168, 1, 0],
        prefix_len: 24,
        next_hop: [10, 0, 0, 2],
        as_path: vec![65002],
        local_pref: 200,
        is_reflected: false,
    };

    assert!(bgp_mgr.process_incoming_route(incoming, true));
    assert_eq!(bgp_mgr.routes.len(), 2);
    assert!(bgp_mgr.routes[1].is_reflected);

    let best = bgp_mgr.best_path_selection([192, 168, 1, 0], 24).unwrap();
    assert_eq!(best.local_pref, 200);
}

#[test]
fn test_pam_authentication_policy_engine_inspection() {
    use hardening::{PamAuthenticationPolicyEngine, PamControlFlag, PamModuleType};

    let mut pam = PamAuthenticationPolicyEngine::new(true);
    pam.add_rule(PamModuleType::Auth, PamControlFlag::Required, "pam_unix", true);
    pam.add_rule(PamModuleType::Auth, PamControlFlag::Required, "pam_tpm2", true);

    assert!(pam.authenticate_pam_stack(PamModuleType::Auth, true).unwrap());
    assert!(pam.authenticate_pam_stack(PamModuleType::Auth, false).is_err());
}

#[test]
fn test_gentoo_use_flag_engine_inspection() {
    use unimplemented_features::GentooUseFlagEngine;

    let mut gentoo = GentooUseFlagEngine::new();
    gentoo.set_use_flag("+ssl");
    gentoo.set_use_flag("x264");
    gentoo.set_use_flag("-wayland");

    assert!(gentoo.is_flag_enabled("ssl"));
    assert!(gentoo.is_flag_enabled("x264"));
    assert!(!gentoo.is_flag_enabled("wayland"));

    gentoo.set_use_flag("wayland");
    assert!(gentoo.resolve_conflicts(("wayland", "x264")).is_err());
}
