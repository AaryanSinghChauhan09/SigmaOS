// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Linux & BSD Parity Inspection Unit Tests

#[path = "../src/kernel/linux_bsd_innovations.rs"]
mod linux_bsd_innovations;
#[path = "../src/unimplemented_features.rs"]
mod unimplemented_features;
#[path = "../src/boot/firmware.rs"]
mod firmware;

#[path = "../src/kernel/sysctl.rs"]
mod sysctl;

#[path = "../src/security/root_improvement.rs"]
mod root_improvement;

#[path = "../src/compatibility/abi_extended.rs"]
mod abi_extended;

#[path = "../src/compatibility/distro_bridge.rs"]
mod distro_bridge;

#[path = "../src/network/protocols.rs"]
mod protocols;
#[path = "../src/security/hardening.rs"]
mod hardening;
#[path = "../src/distro/linux_bsd_parity.rs"]
mod linux_bsd_parity;
#[path = "../src/distro/ready_to_use.rs"]
mod ready_to_use;
#[path = "../src/compatibility/garuda_zen.rs"]
mod garuda_zen;
#[path = "../src/compatibility/distro_bridge.rs"]
mod distro_bridge;
#[path = "../src/virtualization/vm_manager.rs"]
mod vm_manager;
#[path = "../src/compatibility/zorin.rs"]
mod zorin;
#[path = "../src/process/advanced_process_control.rs"]
mod advanced_process_control;
#[path = "../src/kernel/sysctl.rs"]
mod sysctl;
#[path = "../src/security/root_improvement.rs"]
mod root_improvement;
#[path = "../src/compatibility/abi_extended.rs"]
mod abi_extended;
#[path = "../src/compatibility/freebsd_jails.rs"]
mod freebsd_jails;
#[path = "../src/compatibility/bsd.rs"]
mod bsd_compat;
#[path = "../src/distro/wiki_ideas_implementation.rs"]
mod wiki_ideas;
#[path = "../src/kernel/bore.rs"]
mod bore;

#[path = "../src/unimplemented_features.rs"]
mod unimplemented_features;

#[path = "../src/distro/linux_bsd_parity.rs"]
mod linux_bsd_parity;

#[path = "../src/unimplemented_features.rs"]
mod unimplemented_features;

#[path = "../src/kernel/linux_bsd_innovations.rs"]
mod linux_bsd_innovations;

#[path = "../src/boot/firmware.rs"]
mod firmware;

#[path = "../src/unimplemented_features.rs"]
mod unimplemented_features;

#[path = "../src/boot/firmware.rs"]
mod firmware;

#[path = "../src/network/protocols.rs"]
mod protocols;

#[path = "../src/security/hardening.rs"]
mod hardening;

#[path = "../src/kernel/linux_bsd_innovations.rs"]
mod linux_bsd_innovations;

use bsd::*;
use gap_closure::{ZorinAppearanceSwitcher, ZorinLayoutPreset};
use kvm_vcpu::{KvmExitCode, KvmVcpu, VirtioDeviceBackend, VirtioDeviceType, RAX_HLT_SIGNAL};
use unveil::{UnveilManager, UnveilPermission};

#[test]
fn test_freebsd_jail_manager_inspection() {
    let mut jail_mgr = FreeBsdJailManager::new();
    let jid = jail_mgr.create_jail("web_jail", "/jails/web", "192.168.1.100").unwrap();
    assert_eq!(jid, 1);
    assert!(jail_mgr.stop_jail(jid).is_ok());
}

#[test]
fn test_netbsd_rump_router_inspection() {
    assert_eq!(NetBsdRumpKernelRouter::dispatch_hypercall(RumpHypercall::Syscall, 0x100), 257);
}

#[test]
fn test_openbsd_sysctl_mib_inspection() {
    let sysctl = OpenBsdSysctlKernelMib::new();
    assert_eq!(sysctl.query_mib("kern.ostype").unwrap(), "OpenBSD");
}

#[test]
fn test_kvm_qemu_vcpu_inspection() {
    use vm_manager::KvmVirtualCpu;
    let vcpu = KvmVirtualCpu::new(0);
    assert_eq!(vcpu.vcpu_id, 0);
}

#[test]
fn test_zorin_gap_closure_inspection() {
    use zorin::ZorinWindowsAppSupport;
    let res = ZorinWindowsAppSupport::inspect_package_format("ms_office_installer.exe").unwrap();
    assert!(res.contains("libreoffice"));
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
        name: "ubuntu_guest".to_string(),
        cpu_cores: 2,
        memory_mb: 2048,
        disk_size_gb: 20,
        network_enabled: true,
        gpu_passthrough: false,
        os_type: OsType::Linux,
        cpu_pinning_cores: vec![],
        hugepages_enabled: false,
        vfio_pci_passthrough_address: None,
        memory_balloon_mb: 0,
        virtio_net_queues: 1,
        cpu_model: "host".to_string(),
        machine_type: "q35".to_string(),
        nested_virtualization: false,
        io_uring_enabled: true,
        kvm_dirty_ring_size: 1024,
    };
    let vmid = qemu.create_vm(&config).unwrap();
    assert!(qemu.start_vm(&vmid).is_ok());
    assert_eq!(qemu.get_vm_state(&vmid).unwrap(), VmState::Running);
    assert!(qemu.stop_vm(&vmid).is_ok());
}

#[test]
fn test_kernel_classic_algorithms_inspection() {
    use distro_bridge::{LinuxBsdAbiBridge, BinaryAbiFormat};
    use bore::{BoreScheduler, BoreTask};

    let mut bore_sched = BoreScheduler::new();
    let t1 = BoreTask::new(1, "batch-job");
    let t2 = BoreTask::new(2, "user-input");
    bore_sched.add_task(t1);
    bore_sched.add_task(t2);
    assert!(bore_sched.schedule().is_some());

    let mut abi_disp = LinuxBsdAbiBridge::new(BinaryAbiFormat::LinuxElf64);
    let sys_res = abi_disp.dispatch_syscall(9).unwrap();
    assert_eq!(sys_res, 0x7FFF0000);
}

#[test]
fn test_wiki_distro_innovations_inspection() {
    let cap = FreeBsdCapsicumDescriptorDelegate::grant_capability(5, CAP_READ | CAP_SEEK);
    assert!(FreeBsdCapsicumDescriptorDelegate::validate_access(&cap, CAP_READ));

    // 7. Systemd Parity Engine
    let mut systemd = SovereignSystemdParityEngine::new();
    systemd.register_unit("test.service", SystemdUnitType::Service, &[]);
    assert_eq!(systemd.start_unit("test.service"), Ok(SystemdUnitActiveState::Active));
    // 8. Real-Time Hybrid Scheduler
    let sched = SovereignHybridSchedulerInnovations::new();
    assert!(sched.verify_rt_lane_preemption_latency());
}

#[test]
fn test_advanced_process_control_inspection() {
    use advanced_process_control::{
        ProcessVmReadWriteEngine, JobControlLifecycleEngine, ProcessWaiterAndRusageCollector,
        ProcessCancellationAndTerminationManager, AdvancedIpcHub, JobState, CancellationType, BsdRusage,
    };
    let mut vm = ProcessVmReadWriteEngine::new();
    vm.register_process_memory(42, 0x1000, vec![1, 2, 3, 4]);
    assert_eq!(vm.process_vm_readv(42, 0x1000, 2).unwrap(), vec![1, 2]);

    let mut job = JobControlLifecycleEngine::new();
    job.spawn_job(42, 42, 42, true, "test_cmd");
    job.daemonize(42).unwrap();
    assert_eq!(job.jobs.get(&42).unwrap().state, JobState::Background);

    let mut waiter = ProcessWaiterAndRusageCollector::new();
    waiter.record_rusage(42, BsdRusage { ru_utime_ms: 50, ..Default::default() });
    assert_eq!(waiter.get_rusage(42).unwrap().ru_utime_ms, 50);

    let mut cancel = ProcessCancellationAndTerminationManager::new();
    cancel.register_process(10, 5, CancellationType::Deferred);
    cancel.reparent_orphans(5);
    assert_eq!(cancel.process_parents.get(&10), Some(&1));

    let mut ipc = AdvancedIpcHub::new();
    let efd = ipc.eventfd_create(10, false);
    assert_eq!(ipc.eventfd_read(efd).unwrap(), 10);
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


#[path = "../src/distro/linux_bsd_parity.rs"]
mod linux_bsd_parity;

#[test]
fn test_sovereign_linux_bsd_kernel_innovations_inspection() {
    use linux_bsd_innovations::{
        KernelFastPacketEngine, FastPacketFrame, XdpAction,
        KernelAccessController, LandlockAccessRight, PLEDGE_STDIO, PLEDGE_RPATH, PLEDGE_EXEC,
        InteractiveHybridScheduler, HybridTask,
        CowStorageEngine, MemoryCompactionSuperpagesAllocator,
    };
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

    let mut access_ctrl = KernelAccessController::new();
    access_ctrl.add_path_rule("/usr/bin", vec![LandlockAccessRight::Read, LandlockAccessRight::Execute]);
    access_ctrl.restrict_pledge(PLEDGE_STDIO | PLEDGE_RPATH);
    assert!(access_ctrl.check_path_access("/usr/bin/cargo", LandlockAccessRight::Execute).is_ok());
    assert!(access_ctrl.check_pledge(PLEDGE_STDIO).is_ok());
    assert!(access_ctrl.check_pledge(PLEDGE_EXEC).is_err());

    let mut hybrid_sched = InteractiveHybridScheduler::new();
    let mut t1 = HybridTask::new(1, 10);
    t1.sleep_time_ms = 80;
    t1.cpu_time_ms = 20;
    hybrid_sched.add_task(t1);
    let sched_pid = hybrid_sched.schedule_next().unwrap();
    assert_eq!(sched_pid, 1);

    let mut cow = CowStorageEngine::new();
    let blk_id = cow.write_block(b"Kernel Data Block");
    assert!(cow.verify_block_integrity(blk_id).unwrap());
    let snap_id = cow.create_pfs_snapshot("PFS_SNAP_01", blk_id);
    assert_eq!(snap_id, 1);

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

    let mut ucode_engine = CpuMicrocodePatchEngine::new(2);
    let mut raw_intel = vec![0u8; 48];
    raw_intel[0..4].copy_from_slice(&1u32.to_le_bytes());
    raw_intel[4..8].copy_from_slice(&0x000000B0u32.to_le_bytes());
    raw_intel[32..36].copy_from_slice(&2048u32.to_le_bytes());
    let intel_hdr = ucode_engine.parse_intel_header(&raw_intel).unwrap();
    assert_eq!(intel_hdr.vendor, MicrocodeVendor::Intel);
    assert!(ucode_engine.apply_microcode_update(0, intel_hdr));
    assert_eq!(ucode_engine.get_core_patch_level(0), Some(0x000000B0));

    let mut capsule_mgr = FirmwareCapsuleUpdateManager::new();
    let mut capsule_payload = vec![0u8; 32];
    capsule_payload[0..11].copy_from_slice(b"CAPSULE_SIG");
    capsule_payload[12..16].copy_from_slice(&0x02010000u32.to_le_bytes());
    let sys_guid = "3b61b360-1e5b-4227-b50a-8d184713e2f5";
    assert!(capsule_mgr.stage_capsule_payload(sys_guid, &capsule_payload).is_ok());
    assert_eq!(capsule_mgr.current_status, CapsuleUpdateStatus::Staged);
    assert!(capsule_mgr.process_reboot_capsules());
    assert_eq!(capsule_mgr.current_status, CapsuleUpdateStatus::UpdateSuccess);

    let mut smbios = SmbiosFirmwareParser::new();
    assert!(smbios.parse_smbios_entry_point(b"_SM_123456789012"));
    assert_eq!(smbios.bios_info.unwrap().vendor, "SigmaOS Sovereign Core UEFI");

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

#[test]
fn test_gentoo_portage_mask_engine_inspection() {
    use gap_closure::{TargetDistroFamily, SovereignDistroAbsorptionEngine};

    let mut engine = SovereignDistroAbsorptionEngine::new();
    engine.set_active_target(TargetDistroFamily::GentooPortage);
    let spec = engine.execute_distro_absorption("sys-kernel/gentoo-sources");
    assert!(spec.contains("S-PORTAGE Absorption"));
}
