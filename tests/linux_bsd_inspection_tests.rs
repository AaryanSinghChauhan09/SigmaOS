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

#[path = "../src/desktop/zenith_advanced_features.rs"]
mod zenith_advanced;

#[path = "../src/distro/wiki_ideas_implementation.rs"]
mod wiki_ideas_implementation;

#[path = "../src/process/advanced_process_control.rs"]
mod advanced_process_control;

#[path = "../src/distro/missing_distro_innovations.rs"]
mod missing_distro_innovations;

#[path = "../src/process/sovereign_process_engine.rs"]
mod sovereign_process_engine;

#[path = "../src/shell/sovereign_shell_parity.rs"]
mod sovereign_shell_parity;

#[path = "../src/package/repository.rs"]
mod package_repository;

#[path = "../src/kernel/module_loader.rs"]
mod module_loader;

#[path = "../src/logging/unified.rs"]
mod unified;

#[path = "../src/filesystem/sovereign_link_engine.rs"]
mod sovereign_link_engine;

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
fn test_wiki_distro_innovations_inspection() {
    use wiki_ideas_implementation::{
        NixDeclarativeSystemState, ArchRecipeSandboxCompiler, SnapperTransactionGuard,
        SigmaZeroCopySpliceEngine, EbpfSyscallPolicyVerifier, FreeBsdCapsicumDescriptorDelegate,
        PolicyAction, CAP_READ, CAP_SEEK,
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
    let post_id = snapper.create_post_snapshot(pre_id, "Post update", 1005).unwrap();
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
    use wiki_ideas_implementation::{
        SystemdUnit, SystemdUnitType, SystemdUnitActiveState, SovereignSystemdParityEngine,
    };
    let mut systemd = SovereignSystemdParityEngine::new();
    systemd.register_unit(SystemdUnit {
        name: "test.service".to_string(),
        unit_type: SystemdUnitType::Service,
        active_state: SystemdUnitActiveState::Inactive,
        description: "Test".to_string(),
        exec_start: vec!["/bin/true".to_string()],
        dependencies: vec![],
        memory_limit_bytes: None,
        cpu_quota_pct: None,
    });
    assert_eq!(systemd.start_unit("test.service").unwrap(), SystemdUnitActiveState::Active);

    // 8. Real-Time Hybrid Scheduler
    use wiki_ideas_implementation::{
        RealtimeTask, SchedulerClass, SovereignHybridSchedulerInnovations,
    };
    let mut sched = SovereignHybridSchedulerInnovations::new();
    sched.add_task(RealtimeTask { pid: 1, class: SchedulerClass::RTLane, deadline_us: 50, wcet_us: 5, numa_node: 0 });
    assert_eq!(sched.select_next_rt_task().unwrap().pid, 1);

    // 9. Sovereign Process Engine (Process Spawning, I/O, Background Execution & IPC)
    use sovereign_process_engine::{SovereignProcessManager, SovereignProcessState};
    let mut sov_mgr = SovereignProcessManager::new();
    let s_pid = sov_mgr.sovereign_spawn("test_worker", 10);
    sov_mgr.sovereign_run_background(s_pid).unwrap();
    assert_eq!(sov_mgr.processes.get(&s_pid).unwrap().state, SovereignProcessState::BackgroundRunning);

    let ch_id = sov_mgr.create_ipc_channel(s_pid, 2);
    sov_mgr.sovereign_ipc_send(ch_id, b"data_pkt").unwrap();
    assert_eq!(sov_mgr.sovereign_ipc_receive(ch_id).unwrap(), b"data_pkt");

    // 10. Sovereign Shell Engine (Bash & Zsh Parity)
    use sovereign_shell_parity::{SovereignBashZshParityShell, RedirectionType};
    let mut shell = SovereignBashZshParityShell::new();
    shell.variables.insert("MY_VAR".to_string(), "hello_world".to_string());
    assert_eq!(shell.expand_variables("Value: $MY_VAR"), "Value: hello_world");

    let pipe = shell.parse_pipeline("cat /var/log/syslog | grep error > /tmp/err.log &");
    assert_eq!(pipe.len(), 2);
    assert_eq!(pipe[0].program, "cat");
    assert_eq!(pipe[1].program, "grep");
    assert!(pipe[1].run_in_background);
    assert_eq!(pipe[1].redirections, vec![RedirectionType::OutputTruncate("/tmp/err.log".to_string())]);

    // 11. Package Repository Innovations (Pinning, Mirror Ranking, Transaction Journal)
    use package_repository::{PackagePinEngine, PinPriority, MirrorSyncEngine, PackageTransactionJournal};
    let mut pin_eng = PackagePinEngine::new();
    pin_eng.add_pin_rule("sigmaos-kernel", "6.6.0", PinPriority::Hold);
    assert_eq!(pin_eng.get_pin_priority("sigmaos-kernel"), PinPriority::Hold);

    let mut mir_eng = MirrorSyncEngine::new();
    mir_eng.add_mirror("https://slow.repo.org", "US", 300);
    mir_eng.add_mirror("https://fast.repo.org", "US", 10);
    mir_eng.rank_mirrors();
    assert_eq!(mir_eng.get_fastest_mirror(), Some("https://fast.repo.org".to_string()));

    let mut journal = PackageTransactionJournal::new();
    let tx1 = journal.log_transaction("install", "bash", "5.2", 100);
    let tx2 = journal.log_transaction("install", "zsh", "5.9", 105);
    assert_eq!(journal.rollback_transaction(tx2).len(), 1);

    // 12. Sovereign Kernel Module Loader (insmod / rmmod / kldload / kldstat Parity)
    use module_loader::{SovereignKernelModuleManager, ModuleState};
    use std::collections::BTreeMap as TestBTreeMap;

    let mut kmod_mgr = SovereignKernelModuleManager::new();
    let mod_base = kmod_mgr
        .load_module("virtio_gpu", "1.0", "GPL", vec![], TestBTreeMap::new(), 16384)
        .unwrap();
    assert!(mod_base >= 0xFFFFFFFFC0000000);

    let ls_out = kmod_mgr.lsmod();
    assert_eq!(ls_out.len(), 1);
    assert!(ls_out[0].contains("virtio_gpu 16384 0"));

    kmod_mgr.set_module_parameter("virtio_gpu", "modeset", "1").unwrap();
    assert_eq!(kmod_mgr.loaded_modules.get("virtio_gpu").unwrap().parameters.get("modeset").map(|s| s.as_str()), Some("1"));

    kmod_mgr.unload_module("virtio_gpu").unwrap();
    assert_eq!(kmod_mgr.loaded_modules.len(), 0);

    // 13. System Log Innovations (Journald Compression, Rate Limiting, Audit Filtering, Structured Queries)
    use unified::{LogRateLimiter, JournaldCompressedBlock, AuditLogFilter, StructuredLogQueryEngine, UnifiedLogEntry, LogLevel, SyslogFacility};
    let mut limiter = LogRateLimiter::new(1, 100);
    assert!(limiter.allow_entry(100));
    assert!(!limiter.allow_entry(105)); // Suppressed by rate limiter

    let log_entry = UnifiedLogEntry::new(LogLevel::Error, b"NET", b"Link down", b"net.rs", 12).with_facility(SyslogFacility::Kernel).with_pid(100);
    let comp_block = JournaldCompressedBlock::compress_entries(&[log_entry.clone()]);
    assert_eq!(comp_block.uncompressed_entries_count, 1);

    let mut audit_filter = AuditLogFilter::new(LogLevel::Error);
    assert!(audit_filter.matches(&log_entry));

    let log_slice = [log_entry];
    let queried_logs = StructuredLogQueryEngine::query(&log_slice, "_PID", "100");
    assert_eq!(queried_logs.len(), 1);

    // 14. Sovereign Link Engine (Hard Links, Variant Symlinks, ELOOP Cycle Protection)
    use sovereign_link_engine::{SovereignLinkEngine, LinkType};
    let mut link_eng = SovereignLinkEngine::new();
    let ino = link_eng.create_file("/etc/hosts", b"127.0.0.1 localhost");
    link_eng.create_hard_link("/etc/hosts", "/etc/hosts.hard").unwrap();
    assert_eq!(link_eng.inodes.get(&ino).unwrap().hard_link_count, 2);

    link_eng.create_variant_symlink("/usr/lib/$ARCH/libm.so", "/lib/libm.so").unwrap();
    assert_eq!(link_eng.resolve_path("/lib/libm.so").unwrap(), "/usr/lib/x86_64/libm.so");

    // 15. Advanced Kernel Module Loader (Taint Flags, Device Matching, Module Signatures)
    use module_loader::{TaintFlag, DeviceBusType, ModuleSignature};
    let mut adv_kmod = SovereignKernelModuleManager::new();
    adv_kmod.add_taint(TaintFlag::GPLIncompatible);
    assert_eq!(adv_kmod.taint_flags, vec![TaintFlag::GPLIncompatible]);

    let usb_dev = DeviceBusType::Usb { vendor_id: 0x057E, product_id: 0x2009 };
    adv_kmod.register_device_alias(usb_dev.clone(), "hid_nintendo");
    assert_eq!(adv_kmod.auto_probe_module_for_device(&usb_dev), Some("hid_nintendo".to_string()));

    let mod_sig = ModuleSignature {
        algorithm: "Ed25519".to_string(),
        signature_bytes: vec![0xDE, 0xAD, 0xBE, 0xEF],
        key_id: "sec-key-1".to_string(),
    };
    assert!(adv_kmod.verify_signature(&mod_sig));

    // 14. Missing Distro Innovations (Clear Linux, Tails, Chimera, FreeBsd VNET, OpenBSD Unveil Auditor)
    use missing_distro_innovations::{
        ClearLinuxStatelessEngine, TailsAmnesicEngine, ChimeraDinitSupervisor, DinitServiceState,
        FreeBsdVnetStackEngine, OpenBsdUnveilAuditor,
    };
    let mut clear = ClearLinuxStatelessEngine::new();
    clear.set_vendor_default("/etc/issue", "SigmaOS Base");
    assert_eq!(clear.resolve_configuration("/etc/issue").unwrap(), "SigmaOS Base");

    let mut tails = TailsAmnesicEngine::new();
    tails.allocate_session_page(&[0x12, 0x34]);
    assert_eq!(tails.wipe_all_memory_on_shutdown(), 1);

    let mut dinit = ChimeraDinitSupervisor::new();
    dinit.register_service("syslogd", "/usr/sbin/syslogd", vec![]);
    assert_eq!(dinit.start_service("syslogd").unwrap(), DinitServiceState::Started);

    let mut vnet = FreeBsdVnetStackEngine::new();
    let v_stack = vnet.create_vnet_stack(10, "192.168.1.50");
    assert!(v_stack.loopback_up);

    let mut auditor = OpenBsdUnveilAuditor::new();
    auditor.log_violation(99, "/root/.ssh/id_rsa", "r", 500);
    assert_eq!(auditor.violations.len(), 1);
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
