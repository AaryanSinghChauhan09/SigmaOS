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

#[path = "../src/virtualization/vm_manager.rs"]
mod vm_manager;

#[path = "../src/scheduler/eevdf.rs"]
mod eevdf;

#[path = "../src/memory/tlb_associative.rs"]
mod tlb_associative;

#[path = "../src/desktop/zenith_advanced_features.rs"]
mod zenith_advanced;

#[path = "../src/distro/wiki_ideas_implementation.rs"]
mod wiki_ideas_implementation;

#[path = "../src/process/advanced_process_control.rs"]
mod advanced_process_control;

#[path = "../src/kernel/linux_bsd_innovations.rs"]
mod linux_bsd_innovations;

#[path = "../src/filesystem/bsd_linux_innovations.rs"]
mod fs_bsd_linux_innovations;

#[path = "../src/process/sovereign_process_engine.rs"]
mod sovereign_process_engine;

#[path = "../src/sigpkg/sovereign_sigpkg.rs"]
mod sovereign_sigpkg;

#[path = "../src/init/systemd_init.rs"]
mod systemd_init;

#[path = "../src/boot/firmware.rs"]
mod firmware;

#[path = "../src/distro/wiki_ideas_implementation.rs"]
mod wiki_ideas_implementation;

#[path = "../src/process/advanced_process_control.rs"]
mod advanced_process_control;

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

use bsd::*;
use gap_closure::{ZorinAppearanceSwitcher, ZorinLayoutPreset};
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
        KvmExitReason, KvmHypervisor, OsType, VirtioBlockDeviceConfig, VirtioNetDeviceConfig,
        VmConfig, VmState, HypervisorBackend,
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

    kvm.attach_virtio_blk(&vm_id, VirtioBlockDeviceConfig {
        image_path: "/var/lib/images/rootfs.qcow2".to_string(),
        read_only: false,
        direct_io: true,
        queue_size: 256,
        block_size: 512,
    }).unwrap();

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
fn test_wiki_distro_innovations_inspection() {
    use wiki_ideas_implementation::{
        NixDeclarativeSystemState, ArchRecipeSandboxCompiler, SnapperTransactionGuard,
        SigmaZeroCopySpliceEngine, EbpfSyscallPolicyVerifier, FreeBsdCapsicumDescriptorDelegate,
        PolicyAction, CAP_READ, CAP_SEEK, SystemdUnitType,
        SovereignSystemdParityEngine, SovereignHybridSchedulerInnovations,
    };

    // 1. NixOS Declarative System State
    let mut nix = NixDeclarativeSystemState::new();
    let config = "[packages]\n- curl\n[services]\n- networkd";
    let gen = nix.parse_and_apply_config(config, 100).unwrap();
    assert_eq!(gen.id, 2);
    assert_eq!(nix.rollback().unwrap().id, 1);

    // 2. Arch Plaintext Recipe Sandbox Compiler
    let recipe_text = "pkgname=curl\npkgver=8.2.1\ndepends=(zlib openssl)";
    let recipe = ArchRecipeSandboxCompiler::parse_recipe(recipe_text).unwrap();
    assert_eq!(recipe.pkgname, "curl");
    let compiler = ArchRecipeSandboxCompiler::new();
    let artifact = compiler.compile_in_sandbox(&recipe, "/tmp/sandbox").unwrap();
    assert!(!artifact.is_empty());

    // 3. openSUSE Snapper Pre/Post Transaction Guard
    let mut snapper = SnapperTransactionGuard::new();
    let pre_id = snapper.create_pre_snapshot("Pre update", 1000);
    let _post_id = snapper.create_post_snapshot(pre_id, "Post update", 1005).unwrap();
    assert_eq!(snapper.snapshots.len(), 2);

    // 4. Zero-Copy Splice Pipeline
    let splice = SigmaZeroCopySpliceEngine::new();
    assert_eq!(splice.splice(1, 2, 4096).unwrap(), 4096);

    // 5. eBPF Syscall Policy Verifier
    let mut verifier = EbpfSyscallPolicyVerifier::new();
    verifier.block_syscall(59); // execve
    assert_eq!(verifier.evaluate_syscall(59), PolicyAction::Deny);

    // 6. FreeBSD Capsicum Descriptor Delegation
    let cap = FreeBsdCapsicumDescriptorDelegate::grant_capability(3, CAP_READ | CAP_SEEK);
    assert!(FreeBsdCapsicumDescriptorDelegate::validate_access(&cap, CAP_READ));

    // 7. Systemd Parity Engine
    let mut systemd = SovereignSystemdParityEngine::new();
    systemd.register_unit("test.service", SystemdUnitType::Service, &[]);
    assert_eq!(systemd.start_unit("test.service"), Ok(()));
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
    // 1. Process VM read/write
    let mut vm = ProcessVmReadWriteEngine::new();
    vm.register_process_memory(42, 0x1000, vec![1, 2, 3, 4]);
    assert_eq!(vm.process_vm_readv(42, 0x1000, 2).unwrap(), vec![1, 2]);

    // 2. Job control & daemonize
    let mut job = JobControlLifecycleEngine::new();
    job.spawn_job(42, 42, 42, true, "test_cmd");
    job.daemonize(42).unwrap();
    assert_eq!(job.jobs.get(&42).unwrap().state, JobState::Background);

    // 3. Process waiter & rusage
    let mut waiter = ProcessWaiterAndRusageCollector::new();
    waiter.record_rusage(42, BsdRusage { ru_utime_ms: 50, ..Default::default() });
    assert_eq!(waiter.get_rusage(42).unwrap().ru_utime_ms, 50);

    // 4. Cancellation & orphan reparenting
    let mut cancel = ProcessCancellationAndTerminationManager::new();
    cancel.register_process(10, 5, CancellationType::Deferred);
    cancel.reparent_orphans(5);
    assert_eq!(cancel.process_parents.get(&10), Some(&1));

    // 5. Advanced IPC Hub
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

#[path = "../src/network/protocols.rs"]
mod protocols;

#[path = "../src/security/hardening.rs"]
mod hardening;

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

    // 2. CPU Microcode Patch Engine
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

#[test]
fn test_gentoo_portage_mask_engine_inspection() {
    use unimplemented_features::GentooUseFlagEngine;
    let mut use_engine = GentooUseFlagEngine::new();
    use_engine.set_use_flag("+qt5");
    use_engine.set_use_flag("-wayland");
    assert!(use_engine.is_flag_enabled("qt5"));
    assert!(!use_engine.is_flag_enabled("wayland"));

    use unimplemented_features::GentooPortageMaskEngine;
    let mut portage = GentooPortageMaskEngine::new("amd64");
    portage.register_ebuild("sys-kernel/gentoo-sources", "6.6", &["~amd64"], false);
    portage.register_ebuild("app-admin/sudo", "0", &["amd64"], false);
    assert!(portage.evaluate_installability("app-admin/sudo", "0", false).unwrap());
    assert!(portage.evaluate_installability("sys-kernel/gentoo-sources", "6.6", false).is_err());
    assert!(portage.evaluate_installability("sys-kernel/gentoo-sources", "6.6", true).unwrap());
    portage.add_hard_mask("app-admin/sudo");
    assert!(portage.evaluate_installability("app-admin/sudo", "0", true).is_err());

    use linux_bsd_parity::GentooPortageUseFlagsEngine;
    let mut use_flags_engine = GentooPortageUseFlagsEngine::new();
    use_flags_engine.set_global_use_flags(&["+ssl", "+x265"]);
    use_flags_engine.register_package("media-video/ffmpeg", &["ssl", "x265", "gtk"]);
    let resolved = use_flags_engine.resolve_package_flags("media-video/ffmpeg").unwrap();
    assert_eq!(resolved.len(), 2);
    assert!(resolved.contains(&"ssl".to_string()));
    assert!(resolved.contains(&"x265".to_string()));
}

#[test]
fn test_xbps_package_manager_inspection() {
    use linux_bsd_parity::{XbpsPackage, XbpsPackageManager};
    let mut xbps = XbpsPackageManager::new();
    xbps.register_repository_package(XbpsPackage {
        name: "glibc".to_string(),
        version: "2.38".to_string(),
        revision: 1,
        run_depends: vec![],
        sha256_hash: [0x11; 32],
        is_signed: true,
    });
    xbps.register_repository_package(XbpsPackage {
        name: "bash".to_string(),
        version: "5.2.21".to_string(),
        revision: 1,
        run_depends: vec!["glibc".to_string()],
        sha256_hash: [0x22; 32],
        is_signed: true,
    });
    assert!(xbps.verify_signature("bash"));
    let deps = xbps.resolve_dependencies("bash").unwrap();
    assert_eq!(deps, vec!["glibc".to_string(), "bash".to_string()]);
    let count = xbps.install_package_atomic("bash").unwrap();
    assert_eq!(count, 2);
    assert_eq!(xbps.installed_packages.len(), 2);
}

#[test]
fn test_linux_devlink_driver_inspection() {
    use linux_bsd_parity::{DevlinkPortFlavor, LinuxDevlinkDriver};
    let mut devlink = LinuxDevlinkDriver::new();
    devlink.register_port("pci", "0000:01:00.0", 1, DevlinkPortFlavor::Physical);
    assert!(devlink.split_port(1, 4).is_ok());
    assert_eq!(devlink.ports[0].split_count, 4);
    let flashed = devlink.flash_device_firmware("pci", "0000:01:00.0", b"FIRMWARE_IMAGE_BLOB").unwrap();
    assert_eq!(flashed, 19);
}

#[test]
fn test_systemd_unit_dependency_engine_inspection() {
    use linux_bsd_parity::{SystemdUnit, SystemdUnitDependencyEngine};
    let mut engine = SystemdUnitDependencyEngine::new();
    engine.add_unit(SystemdUnit {
        name: "network.target".to_string(),
        requires: vec![],
        after: vec![],
    });
    engine.add_unit(SystemdUnit {
        name: "sshd.service".to_string(),
        requires: vec!["network.target".to_string()],
        after: vec!["network.target".to_string()],
    });
    assert!(!engine.detect_circular_dependencies());
    let seq = engine.compute_startup_sequence().unwrap();
    assert_eq!(seq, vec!["network.target".to_string(), "sshd.service".to_string()]);
}

#[test]
fn test_alpine_apk_v3_and_triggers_inspection() {
    use unimplemented_features::{AlpineApkPackageIndex, ApkPackageEntry, ApkTriggerScript};
    let mut apk = AlpineApkPackageIndex::new();
    apk.add_package(ApkPackageEntry {
        name: "musl".to_string(),
        version: "1.2.4".to_string(),
        arch: "x86_64".to_string(),
        sha256_hash: [0x77; 32],
        dependencies: vec![],
    });
    apk.add_trigger(ApkTriggerScript {
        trigger_path: "/lib/modules".to_string(),
        command: "depmod -a".to_string(),
    });
    assert_eq!(apk.run_package_triggers(), 1);
    assert!(apk.verify_apk_v3_checksum("musl", &[0x77; 32]));
    assert!(apk.resolve_musl_abi_compat("1.2.4"));
}

#[test]
fn test_dragonfly_hammer2_pfs_cluster_delta_inspection() {
    use unimplemented_features::DragonFlyHammer2FsSnapshot;
    let mut hammer2 = DragonFlyHammer2FsSnapshot::new();
    hammer2.register_cluster_node(5, "192.168.1.105");
    let snap_id = hammer2.create_pfs_snapshot("VAR_PFS", 0x12345678, 1700000000);
    let delta_hash = hammer2.sync_cluster_delta(snap_id, 5).unwrap();
    assert_ne!(delta_hash, 0);
    assert!(hammer2.verify_cluster_merkle_roots("VAR_PFS"));
}

#[test]
fn test_sysctl_parameter_registry_inspection() {
    use sysctl::{SysctlRegistry, SysctlValue};
    let mut registry = SysctlRegistry::new();
    assert_eq!(registry.get("kern.ostype"), Some(&SysctlValue::String("SigmaOS".to_string())));
    assert!(registry.set("vm.swappiness", SysctlValue::Int(15)).is_ok());
    assert_eq!(registry.get("vm.swappiness"), Some(&SysctlValue::Int(15)));
    assert!(registry.set("vm.swappiness", SysctlValue::Int(-1)).is_err());
}

#[test]
fn test_pam_authentication_stack_inspection() {
    use root_improvement::{PamEngine, PamGroup, PamRule, PamControlFlag, PamUnixModule, PamResult, SudoDoasElevator};
    let mut engine = PamEngine::new();
    let db = vec![("admin".to_string(), "hash_secret".to_string())];
    let unix_mod = std::sync::Arc::new(PamUnixModule::new(db));
    engine.add_rule(PamGroup::Auth, PamRule {
        control_flag: PamControlFlag::Required,
        module: unix_mod,
    });
    assert_eq!(engine.execute_group(PamGroup::Auth, "admin", "hash_secret"), PamResult::Success);
    assert_eq!(engine.execute_group(PamGroup::Auth, "admin", "wrong_hash"), PamResult::AuthError);
    let mut elevator = SudoDoasElevator::new();
    elevator.password_database.push(("admin".to_string(), "pass123".to_string()));
    assert_eq!(elevator.elevate_via_doas("admin", "pass123", 1000).unwrap(), 0);
    assert!(elevator.verify_active_sudo_session(0, 2000));
}

#[test]
fn test_multi_arch_abi_and_syscall_bridge_inspection() {
    use abi_extended::{Arm64AapcsFrame, Riscv64AbiFrame, SystemVAbiFrame};
    use distro_bridge::{LinuxBsdAbiBridge, BinaryAbiFormat};
    let sysv = SystemVAbiFrame::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(sysv.arg_registers[0], 1);
    let arm64 = Arm64AapcsFrame::new(&[10, 20, 30, 40, 50, 60, 70, 80]);
    assert_eq!(arm64.arg_registers[7], 80);
    let riscv = Riscv64AbiFrame::new(&[100, 200, 300, 400, 500, 600, 700, 800]);
    assert_eq!(riscv.arg_registers[0], 100);
    let mut linux_bridge = LinuxBsdAbiBridge::new(BinaryAbiFormat::LinuxElf64);
    assert_eq!(linux_bridge.dispatch_syscall(9).unwrap(), 0x7FFF0000); // SYS_mmap
    let mut openbsd_bridge = LinuxBsdAbiBridge::new(BinaryAbiFormat::OpenBsdElf64);
    assert_eq!(openbsd_bridge.dispatch_syscall(20).unwrap(), 1000); // SYS_getpid

    use linux_bsd_parity::GentooPortageUseFlagsEngine;
    let mut portage = GentooPortageUseFlagsEngine::new();
    portage.set_global_use_flags(&["+ssl", "-gtk"]);
    portage.register_package("curl", &["ssl", "gtk"]);
    let flags = portage.resolve_package_flags("curl").unwrap();
    assert!(flags.contains(&"ssl".to_string()));
    assert!(!flags.contains(&"gtk".to_string()));
}

#[test]
fn test_xbps_package_manager_inspection() {
    use linux_bsd_parity::{XbpsPackage, XbpsPackageManager};

    let mut xbps = XbpsPackageManager::new();
    xbps.register_repository_package(XbpsPackage {
        name: "glibc".to_string(),
        version: "2.38".to_string(),
        revision: 1,
        run_depends: vec![],
        sha256_hash: [0x11; 32],
        is_signed: true,
    });
    xbps.register_repository_package(XbpsPackage {
        name: "bash".to_string(),
        version: "5.2.21".to_string(),
        revision: 1,
        run_depends: vec!["glibc".to_string()],
        sha256_hash: [0x22; 32],
        is_signed: true,
    });

    assert!(xbps.verify_signature("bash"));
    let deps = xbps.resolve_dependencies("bash").unwrap();
    assert_eq!(deps, vec!["glibc".to_string(), "bash".to_string()]);

    let count = xbps.install_package_atomic("bash").unwrap();
    assert_eq!(count, 2);
    assert_eq!(xbps.installed_packages.len(), 2);
}

#[test]
fn test_linux_devlink_driver_inspection() {
    use linux_bsd_parity::{DevlinkPortFlavor, LinuxDevlinkDriver};

    let mut devlink = LinuxDevlinkDriver::new();
    devlink.register_port("pci", "0000:01:00.0", 1, DevlinkPortFlavor::Physical);

    assert!(devlink.split_port(1, 4).is_ok());
    assert_eq!(devlink.ports[0].split_count, 4);

    let flashed = devlink.flash_device_firmware("pci", "0000:01:00.0", b"FIRMWARE_IMAGE_BLOB").unwrap();
    assert_eq!(flashed, 19);
}

#[test]
fn test_systemd_unit_dependency_engine_inspection() {
    use linux_bsd_parity::{SystemdUnit, SystemdUnitDependencyEngine};

    let mut engine = SystemdUnitDependencyEngine::new();
    engine.add_unit(SystemdUnit {
        name: "network.target".to_string(),
        requires: vec![],
        after: vec![],
    });
    engine.add_unit(SystemdUnit {
        name: "sshd.service".to_string(),
        requires: vec!["network.target".to_string()],
        after: vec!["network.target".to_string()],
    });

    assert!(!engine.detect_circular_dependencies());
    let seq = engine.compute_startup_sequence().unwrap();
    assert_eq!(seq, vec!["network.target".to_string(), "sshd.service".to_string()]);
}

#[test]
fn test_alpine_apk_v3_and_triggers_inspection() {
    use unimplemented_features::{AlpineApkPackageIndex, ApkPackageEntry, ApkTriggerScript};

    let mut apk = AlpineApkPackageIndex::new();
    apk.add_package(ApkPackageEntry {
        name: "musl".to_string(),
        version: "1.2.4".to_string(),
        arch: "x86_64".to_string(),
        sha256_hash: [0x77; 32],
        dependencies: vec![],
    });

    apk.add_trigger(ApkTriggerScript {
        trigger_path: "/lib/modules".to_string(),
        command: "depmod -a".to_string(),
    });

    assert_eq!(apk.run_package_triggers(), 1);
    assert!(apk.verify_apk_v3_checksum("musl", &[0x77; 32]));
    assert!(apk.resolve_musl_abi_compat("1.2.4"));
}

#[test]
fn test_dragonfly_hammer2_pfs_cluster_delta_inspection() {
    use unimplemented_features::DragonFlyHammer2FsSnapshot;

    let mut hammer2 = DragonFlyHammer2FsSnapshot::new();
    hammer2.register_cluster_node(5, "192.168.1.105");
    let snap_id = hammer2.create_pfs_snapshot("VAR_PFS", 0x12345678, 1700000000);

    let delta_hash = hammer2.sync_cluster_delta(snap_id, 5).unwrap();
    assert_ne!(delta_hash, 0);
    assert!(hammer2.verify_cluster_merkle_roots("VAR_PFS"));
}

#[test]
fn test_kernel_parity_innovations_geom_devlink_unveil_vnet_inspection() {
    use linux_bsd_innovations::{
        FreeBsdGeomTopology, LinuxDevlinkHealthMonitor, OpenBsdUnveilEngine, FreeBsdVnetManager,
    };

    // 1. FreeBSD GEOM Topology
    let mut geom = FreeBsdGeomTopology::new();
    geom.register_class("DISK");
    assert!(geom.add_provider("DISK", "ada0", 1_000_000_000, 512).is_ok());
    let provider = geom.find_provider("ada0").unwrap();
    assert_eq!(provider.mediasize_bytes, 1_000_000_000);
    assert_eq!(provider.sectorsize, 512);

    // 2. Linux Devlink Health Monitor
    let mut devlink_health = LinuxDevlinkHealthMonitor::new();
    devlink_health.register_reporter("pci/0000:00:1f.3/fw");
    assert_eq!(devlink_health.get_state("pci/0000:00:1f.3/fw").unwrap(), "healthy");
    assert!(devlink_health.report_error("pci/0000:00:1f.3/fw").is_ok());
    assert_eq!(devlink_health.get_state("pci/0000:00:1f.3/fw").unwrap(), "error");
    assert!(devlink_health.recover("pci/0000:00:1f.3/fw").is_ok());
    assert_eq!(devlink_health.get_state("pci/0000:00:1f.3/fw").unwrap(), "recovered");

    // 3. OpenBSD Unveil Engine
    let mut unveil_engine = OpenBsdUnveilEngine::new();
    assert!(unveil_engine.unveil("/usr/lib", "r").is_ok());
    assert!(unveil_engine.unveil("/tmp", "rwc").is_ok());
    unveil_engine.lock();
    assert!(unveil_engine.unveil("/etc", "r").is_err()); // Locked engine rejects unveiling
    assert!(unveil_engine.check_path("/usr/lib/libc.so", 'r').is_ok());
    assert!(unveil_engine.check_path("/usr/lib/libc.so", 'w').is_err());
    assert!(unveil_engine.check_path("/tmp/test.txt", 'w').is_ok());

    // 4. FreeBSD VNET Virtualized Network Stack
    let mut vnet_mgr = FreeBsdVnetManager::new();
    assert!(vnet_mgr.create_vnet(10).is_ok());
    assert!(vnet_mgr.assign_interface(10, "epair0b").is_ok());
    assert!(vnet_mgr.assign_ip(10, "10.0.0.10/24").is_ok());
    let vnet_stack = vnet_mgr.get_vnet(10).unwrap();
    assert_eq!(vnet_stack.interfaces, vec!["epair0b".to_string()]);
    assert_eq!(vnet_stack.ip_addresses, vec!["10.0.0.10/24".to_string()]);
}

#[test]
fn test_sovereign_fhs_hierarchy_engine_inspection() {
    use fs_bsd_linux_innovations::SovereignFhsHierarchyEngine;

    let fhs = SovereignFhsHierarchyEngine::new();
    assert_eq!(fhs.resolve_fhs_path("/bin/bash"), "/usr/bin/bash");
    assert_eq!(fhs.resolve_fhs_path("/sbin/ip"), "/usr/sbin/ip");
    assert_eq!(fhs.resolve_fhs_path("/lib/modules"), "/usr/lib/modules");
    assert_eq!(fhs.resolve_fhs_path("/var/run/dbus/system_bus_socket"), "/run/dbus/system_bus_socket");
    assert_eq!(fhs.resolve_fhs_path("/usr/local/bin/python"), "/usr/local/bin/python");
}

#[test]
fn test_sovereign_pid_allocator_and_process_tree_inspection() {
    use sovereign_process_engine::{SovereignPidAllocator, SovereignProcessManager};

    // 1. PID Allocator & Bitmap Recycling
    let mut pid_alloc = SovereignPidAllocator::new(1005);
    let p1 = pid_alloc.alloc_pid(1).unwrap();
    let p2 = pid_alloc.alloc_pid(1).unwrap();
    assert_eq!(p1, 1000);
    assert_eq!(p2, 1001);

    pid_alloc.free_pid(p1);
    let recycled_p = pid_alloc.alloc_pid(1).unwrap();
    assert_eq!(recycled_p, 1000); // Recycled PID reused first!

    // 2. Process Tree Hierarchy & FreeBSD pdfork handle
    let mut pm = SovereignProcessManager::new();
    let parent_pid = pm.spawn_process("parent_init", 0);
    let child_pid = pm.spawn_process("child_worker", parent_pid);

    let parent_node = pm.process_tree.get(&parent_pid).unwrap();
    assert_eq!(parent_node.children_pids, vec![child_pid]);

    let child_node = pm.process_tree.get(&child_pid).unwrap();
    assert_eq!(child_node.ppid, parent_pid);
    assert_eq!(child_node.pgid, parent_node.pgid); // Process group inherited
    assert!(child_node.pdfork_fd.is_some());
}

#[test]
fn test_sovereign_mirror_fallback_engine_inspection() {
    use sovereign_sigpkg::{CentralRepositoryManager, SovereignMirrorFallbackEngine};

    let mut repo_mgr = CentralRepositoryManager::new();
    repo_mgr.add_mirror("https://slow-mirror.sigmaos.org", "US-West", 150);
    repo_mgr.add_mirror("https://fast-mirror.sigmaos.org", "US-East", 20);

    let mut fallback_engine = SovereignMirrorFallbackEngine::new(repo_mgr, 3);
    let mock_payload = vec![0x10, 0x20, 0x30];
    let mut expected_checksum = [0u8; 32];
    for (i, &b) in mock_payload.iter().enumerate() {
        expected_checksum[i % 32] ^= b;
    }

    let res = fallback_engine
        .fetch_package_with_failover("openssl", &expected_checksum, |url: &str, _pkg: &str| {
            if url.contains("slow") {
                Err("Connection timeout")
            } else {
                Ok(vec![0x10, 0x20, 0x30])
            }
        })
        .unwrap();

    assert_eq!(res.mirror_url, "https://fast-mirror.sigmaos.org");
    assert_eq!(res.payload, vec![0x10, 0x20, 0x30]);
}

#[test]
fn test_systemd_unit_parser_watchdog_slice_inspection() {
    use systemd_init::{SystemdCgroupSliceGovernor, SystemdServiceWatchdog, SystemdUnitFileParser};

    // 1. INI Unit File Parsing
    let unit_content = r#"
[Unit]
Description=Sovereign Web Server Service

[Service]
ExecStart=/usr/bin/nginx -g 'daemon off;'
Restart=always
WatchdogSec=10s
Slice=system.slice

[Install]
WantedBy=multi-user.target
"#;

    let parsed = SystemdUnitFileParser::parse_unit_file(unit_content);
    assert_eq!(parsed.unit_description, "Sovereign Web Server Service");
    assert_eq!(parsed.exec_start, "/usr/bin/nginx -g 'daemon off;'");
    assert_eq!(parsed.restart_policy, "always");
    assert_eq!(parsed.watchdog_sec, 10);
    assert_eq!(parsed.slice, "system.slice");
    assert_eq!(parsed.wanted_by, "multi-user.target");

    // 2. Watchdog Heartbeat Monitoring
    let mut watchdog = SystemdServiceWatchdog::new("nginx", 10);
    watchdog.ping_watchdog(100);
    assert!(watchdog.check_health(105)); // Elapsed 5s <= 10s (healthy)
    assert!(!watchdog.check_health(115)); // Elapsed 15s > 10s (unhealthy / timeout)

    // 3. Cgroup v2 Slice Governor Quotas
    let mut slice_gov = SystemdCgroupSliceGovernor::new("system.slice", 100, 1024 * 1024);
    assert!(slice_gov.allocate_slice_memory(512 * 1024).is_ok());
    assert!(slice_gov.allocate_slice_memory(600 * 1024).is_err()); // Quota exceeded
}
