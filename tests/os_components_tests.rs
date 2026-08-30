// SigmaOS Comprehensive OS Components Integration & Unit Test Suite
// Verifies sovereign subsystem capabilities, compatibility layers, drivers, security, and tools.

extern crate alloc;

#[path = "../src/klib/mod.rs"]
pub mod klib;

#[path = "../src/ipc/pipes.rs"]
mod pipes;
#[path = "../src/security/unveil.rs"]
mod unveil;
#[path = "../src/storage/geom.rs"]
mod geom;
#[path = "../src/audio/editor.rs"]
mod audio_editor;
#[path = "../src/graphics/video_editor.rs"]
mod video_editor;
#[path = "../src/compatibility/chimera_linux.rs"]
mod chimera_linux;
#[path = "../src/compatibility/debian.rs"]
mod debian_compat;
#[path = "../src/compatibility/bsd.rs"]
mod bsd;
#[path = "../src/distro/linux_bsd_inspirations.rs"]
mod distro_inspirations;
#[path = "../src/compatibility/cachy_os.rs"]
mod cachy_os;
#[path = "../src/distro/endeavour_os.rs"]
mod endeavour_os;
#[path = "../src/compatibility/fedora.rs"]
mod fedora_compat;
#[path = "../src/scheduler/scheduler.rs"]
mod task_scheduler;
#[path = "../src/ipc/alpc.rs"]
mod alpc;
#[path = "../src/memory/bitmap_pmm.rs"]
mod bitmap_pmm;
#[path = "../src/memory/low_level.rs"]
mod low_level_memory;
#[path = "../src/access/control.rs"]
mod access_control;
#[path = "../src/filesystem/ext4_ntfs_security.rs"]
mod ext4_ntfs_security;
#[path = "../src/dashboard/statutory_compliance.rs"]
mod statutory_compliance;
#[path = "../src/community/toolkit.rs"]
mod community_toolkit;
#[path = "../src/system/user.rs"]
mod system_user;
#[path = "../src/tools/sigmatools.rs"]
mod sigmatools;
#[path = "../src/memory/segmentation_paging.rs"]
mod segmentation_paging;
#[path = "../src/process/activity_manager.rs"]
mod process_activity_manager;
#[path = "../src/filesystem/sigma_fs.rs"]
mod sigma_fs_extended;
#[path = "../src/event/epoll.rs"]
mod epoll;
#[path = "../src/loader/elf/relocation.rs"]
mod elf_relocation;
#[path = "../src/sigpkg/mod.rs"]
mod sigpkg;
#[path = "../src/device/manager.rs"]
mod device_manager;
#[path = "../src/compatibility/antix.rs"]
mod antix_compat;

use pipes::Pipe;
use unveil::{UnveilManager, UnveilPermission};
use geom::{BioRequest, GeomProvider, GeomTopology};
use audio_editor::{AudioEffect, AudioTrack, MultiTrackSession, SpectralNoiseSuppressionEffect};
use video_editor::{ExportFormat, ExportProfile, VideoClip, VideoTimeline, VideoTrack};
use chimera_linux::{ApkPackageMetadata, ApkPackageStore, BsdUserlandCompat, DinitService, DinitServiceManager};
use debian_compat::{AptRepositorySync, DebianAlternativesSystem, DebianChannel};
use cachy_os::{AnanicyManager, BoreSchedulerGovernor, SchedPolicy};
use endeavour_os::{AurPackageSpec, PacmanMirror, ReflectorMirrorManager, YayParuHelper};
use fedora_compat::DnfPackageResolver;
use task_scheduler::{Priority, PriorityScheduler, Scheduler, Task, TaskCapability, TaskWorkloadType};
use alpc::{alpc_flags, AlpcFacility, AlpcManager, AlpcMessage};
use bitmap_pmm::{BitmapPhysicalMemoryManager, SelfReferentialPagingEngine as SelfRefPagingEngine, SyscallTableRouter};
use low_level_memory::{posix_syscall_nr, CopyOnWriteForkEngine, FastSyscallDispatcher, MinimalPosixSyscallMatrix, RecursivePageTableEngine, SlabObjectType, TrapRegisterFrame, TwoTierMemoryAllocator};
use access_control::{AclEntry, AclTag as ControlAclTag, CapBoundingSet, DacPermission, FilterPolicy, MacSecurityLabel, PosixAcl, SensitivityLevel, ZeroTrustAccessGate};
use statutory_compliance::{ComplianceRuleStatus, DisputeAuditRollbackEngine, PenaltyBreachNotifier, StatutoryFramework, StatutoryGovernanceLayer, StatutoryGovernanceRule};
use community_toolkit::{CommunityHandbookCatalog, HybridFirewallTemplateStore, ReproduciblePackageRecipeManager, SecurityProfileTemplateStore, VirtualizationBlueprintStore};
use system_user::UserManager;
use sigmatools::*;
use segmentation_paging::{AddressBindingMode, AslrEntropyConfig, CpuRing as SegCpuPrivilegeMode, ExecutableAddressBinding, RandomizedAddressSpace, SegmentDescriptor, SegmentSelector, SpaceProtectionFlags, SegmentationPagingEngine};
use process_activity_manager::{ActivityManager, ActivityState, RegisterSnapshot as ProcRegisterSnapshot};
use sigma_fs_extended::{Blake3BlockDeduplicationEngine, PfsType, PseudoFilesystemNamespace};
use epoll::{EpollEvent, EpollInstance, EpollOp, EPOLLET, EPOLLIN};
use elf_relocation::{ElfRelaEntry, ElfRelocator, ElfSymbol, R_X86_64_GLOB_DAT, R_X86_64_RELATIVE};

#[test]
fn test_segmentation_paging_and_aslr() {
    let code_desc = SegmentDescriptor::code_segment_ring0();
    assert_eq!(code_desc.dpl, SegCpuPrivilegeMode::Ring0Kernel);

    let selector = SegmentSelector::new(1, false, SegCpuPrivilegeMode::Ring0Kernel);
    assert_eq!(selector.index, 1);

    let engine = segmentation_paging::SegmentationPagingEngine::new(
        segmentation_paging::SpaceProtectionFlags::strict_hardening(),
    );
    let linear = engine
        .translate_logical_to_linear(selector, 0x1000, SegCpuPrivilegeMode::Ring0Kernel)
        .unwrap();
    assert_eq!(linear, 0x1000);

    let aslr = RandomizedAddressSpace::compute_aslr_layout(
        0x100000000,
        AslrEntropyConfig::linux_default(),
        0x12345678,
    );
    assert!(aslr.text_base >= 0x100000000);
}

#[test]
fn test_regex_unveil_and_glob_matching() {
    let mut unveil_mgr = UnveilManager::new();
    unveil_mgr.unveil("/var/log/*.log", "r").unwrap();
    assert!(unveil_mgr
        .validate_path("/var/log/syslog.log", UnveilPermission::Read)
        .is_ok());
    assert!(unveil_mgr
        .validate_path("/var/log/syslog.txt", UnveilPermission::Read)
        .is_err());
}

#[test]
fn test_hammer2_pfs_namespaces_and_blake3_dedup() {
    let mut pfs = PseudoFilesystemNamespace::new("root_master", PfsType::Master);
    pfs.file_map
        .insert("/etc/hostname".to_string(), "blake3-hash1".to_string());

    let snap =
        PseudoFilesystemNamespace::snapshot("root_snap_1", "root_master", pfs.file_map.clone());
    assert!(snap.is_read_only);
    assert_eq!(snap.parent_snapshot_id.unwrap(), "root_master");

    let mut dedup = Blake3BlockDeduplicationEngine::new();
    let hash1 = dedup.store_block(b"SOVEREIGN_SYSTEM_BLOCK_DATA");
    let hash2 = dedup.store_block(b"SOVEREIGN_SYSTEM_BLOCK_DATA");
    assert_eq!(hash1, hash2);
    assert_eq!(*dedup.ref_counts.get(&hash1).unwrap(), 2);

    assert!(!dedup.release_block(&hash1));
    assert!(dedup.release_block(&hash1));
    assert!(dedup.read_block(&hash1).is_none());
}

#[test]
fn test_process_activity_manager_and_registers() {
    let mut pam = ActivityManager::new();
    pam.register_process(500, 0, "chrome", 0);

    pam.set_foreground_process(500).unwrap();
    let active_proc = pam.get_process_activity(500).unwrap();
    assert_eq!(active_proc.state, ActivityState::Interactive);

    let ctx = ProcRegisterSnapshot {
        rip: 0x00007FFF00002000,
        rsp: 0x00007FFFFFFFD000,
        rax: 1,
        rbx: 0,
        rcx: 0,
        rdx: 0,
        rsi: 0,
        rdi: 0,
        rbp: 0,
        rflags: 0,
        cs: 0,
        ds: 0,
        ss: 0,
        es: 0,
        fs: 0,
        gs: 0,
    };
    pam.capture_register_snapshot(500, ctx).unwrap();

    let loaded_proc = pam.get_process_activity(500).unwrap();
    assert_eq!(
        loaded_proc.register_snapshot.unwrap().rip,
        0x00007FFF00002000
    );
}

#[test]
fn test_zero_copy_ipc_pipes() {
    let mut pipe1 = Pipe::new(true);
    let mut pipe2 = Pipe::new(true);

    pipe1.write(b"sigmaos zero copy payload");
    let spliced = pipe1.splice(&mut pipe2, 25);
    assert_eq!(spliced, 25);

    let mut read_buf = [0u8; 32];
    let n = pipe2.read(&mut read_buf);
    assert_eq!(n, 25);
    assert_eq!(&read_buf[..25], b"sigmaos zero copy payload");
}

#[test]
fn test_unveil_sandboxing_and_landlock() {
    let mut mgr = UnveilManager::new();
    mgr.unveil("/usr/bin", "rx").unwrap();
    mgr.unveil_at("/etc", "nginx", "r").unwrap();

    assert!(mgr
        .validate_path("/usr/bin/cargo", UnveilPermission::Read)
        .is_ok());
    assert!(mgr
        .validate_path("/usr/bin/cargo", UnveilPermission::Execute)
        .is_ok());
    assert!(mgr
        .validate_path("/usr/bin/cargo", UnveilPermission::Write)
        .is_err());

    assert!(mgr
        .validate_path("/etc/nginx/nginx.conf", UnveilPermission::Read)
        .is_ok());
    assert!(mgr
        .validate_path("/etc/nginx/nginx.conf", UnveilPermission::Write)
        .is_err());
}

#[test]
fn test_geom_storage_topology_and_geli() {
    let mut geom = GeomTopology::new();
    let disk = GeomProvider::new("ada0", 8192, 512);
    geom.register_provider(disk);

    assert!(geom
        .create_partition("ada0", "ada0p1", 0, 4096, "freebsd-ufs")
        .is_ok());
    assert!(geom
        .create_eli("ada0p1", "ada0p1.eli", "sovereign_pass")
        .is_ok());

    let mut write_bio = BioRequest::new_write(0, b"SOVEREIGN_STORAGE_BLOCK".to_vec());
    geom.dispatch_bio("ada0p1.eli", &mut write_bio);
    assert!(write_bio.completed);

    let mut read_bio = BioRequest::new_read(0, 23);
    geom.dispatch_bio("ada0p1.eli", &mut read_bio);
    assert!(read_bio.completed);
    assert_eq!(read_bio.data, b"SOVEREIGN_STORAGE_BLOCK".to_vec());
}

#[test]
fn test_audio_dsp_mixing_and_effects() {
    let mut session = MultiTrackSession::new(44100);

    let t1 = AudioTrack::new(1, "Vocals")
        .with_samples(&[0.6, 0.6, 0.6])
        .with_volume(1.0);
    let t2 = AudioTrack::new(2, "Guitars")
        .with_samples(&[0.2, -0.2, 0.2])
        .with_volume(1.0);

    session.add_track(t1);
    session.add_track(t2);

    let mix = session.mix_session();
    assert_eq!(mix.len(), 3);
    assert!((mix[0] - 0.8).abs() < 1e-5);

    let mut dsp_buf: [f32; 3] = [0.02, 0.80, -0.01];
    let noise_suppress = SpectralNoiseSuppressionEffect::new(0.05);
    noise_suppress.apply(&mut dsp_buf);
    assert!(
        if dsp_buf[0] < 0.0 {
            -dsp_buf[0]
        } else {
            dsp_buf[0]
        } < 0.01
    );
    assert!(dsp_buf[1] > 0.70);
}

#[test]
fn test_video_editor_sigmacut_engine() {
    let mut timeline = VideoTimeline::new(1920, 1080);
    let mut track = VideoTrack::new(1, "Main Track");

    let clip = VideoClip::new(0, "intro.mp4", 0, 60);
    track.add_clip(clip);
    timeline.add_video_track(track);

    assert_eq!(timeline.scrub_timeline_gpu(20), Ok(()));
    assert_eq!(timeline.playhead_frame, 20);

    let frame = timeline.render_frame(20, [0, 0, 0]);
    assert_eq!(frame[0], [120, 180, 240]);

    let profile = ExportProfile {
        format: ExportFormat::AV1,
        bitrate_kbps: 12000,
        hardware_accelerated: true,
        passes: 2,
    };
    let payload = timeline.export_video(profile).unwrap();
    assert!(payload.starts_with(b"AV1-COMPLIANT"));
}

#[test]
fn test_chimera_linux_parity() {
    let mut dinit = DinitServiceManager::new();
    let svc = DinitService::new(b"nginx");
    dinit.register_service(svc);
    assert!(dinit.start_service(b"nginx").is_ok());

    let compat = BsdUserlandCompat;
    let pids = compat.pgrep_filter_by_name(&[(b"nginx", 101)], b"ng");
    assert_eq!(pids, vec![101]);

    let mut store = ApkPackageStore::new();
    let pkg = ApkPackageMetadata::new(b"libkmod", b"31-r0", b"sha256sumhex");
    store.register_apk_installed(pkg);
    assert!(store.verify_installed_checksum(b"libkmod", b"sha256sumhex"));
}

#[test]
fn test_debian_compat_system() {
    let mut alts = debian_compat::DebianAlternativesSystem::new("editor".to_string());
    alts.register_alternative(
        "/usr/bin/editor".to_string(),
        "/usr/bin/vim".to_string(),
        50,
    );
    alts.register_alternative(
        "/usr/bin/editor".to_string(),
        "/usr/bin/nano".to_string(),
        100,
    );

    assert_eq!(alts.get_active_target().unwrap(), "/usr/bin/nano");

    let mut repo = debian_compat::AptRepositorySync::new(
        debian_compat::DebianChannel::Stable,
        "http://deb.debian.org/debian".to_string(),
    );
    repo.verify_release_keyring(&[0x99, 0x01]);
    assert!(repo.fetch_package_index().is_ok());
}

#[test]
fn test_cachy_os_performance_governor() {
    let bore = BoreSchedulerGovernor::new();
    let burstiness = bore.calculate_burstiness(1, 100);
    assert_eq!(bore.determine_nice_offset(burstiness), -5);

    let ananicy = AnanicyManager::new();
    let (nice, policy, io) = ananicy.lookup_and_tune_process("game_engine");
    assert_eq!(nice, -10);
    assert_eq!(policy, SchedPolicy::Fifo);
    assert_eq!(io, 1);

    let mut repo_selector = cachy_os::CachyosRepoMirrorSelector::new(3);
    repo_selector.add_mirror(cachy_os::CachyosMirror {
        url: "https://mirror.cachyos.org/v3".to_string(),
        arch_v_level: 3,
        ping_ms: 10,
        speed_kbps: 60000,
    });
    let mirror = repo_selector.select_fastest_mirror().unwrap();
    assert_eq!(mirror.arch_v_level, 3);
}

#[test]
fn test_endeavour_os_parity() {
    let mut reflector = ReflectorMirrorManager::new();
    reflector.add_mirror(PacmanMirror {
        country: "Germany".to_string(),
        url: "https://fast.archlinux.de".to_string(),
        latency_ms: 10,
        speed_kbps: 20000,
    });
    let ranked = reflector.rank_mirrors(Some("Germany"));
    assert_eq!(ranked[0].url, "https://fast.archlinux.de");

    let mut helper = YayParuHelper::new();
    helper.register_aur_package(AurPackageSpec {
        name: "yay-bin".to_string(),
        version: "12.0.0".to_string(),
        pkgbuild_url: "https://aur.archlinux.org/yay-bin.git".to_string(),
        votes: 500,
    });
    assert!(helper.build_and_install("yay-bin").is_ok());
}

#[test]
fn test_fedora_rpm_and_selinux() {
    let mut resolver = DnfPackageResolver::new();
    resolver.sync_repodata();
    resolver.register_rpm("kernel-core", vec![]);
    let order = resolver.resolve_and_install("kernel-core").unwrap();
    assert_eq!(order, vec!["kernel-core".to_string()]);

    let selinux = fedora_compat::SeLinuxEngine::new(true);
    let src = fedora_compat::SeLinuxContext::new("system_u", "system_r", "httpd_t", "s0");
    let tgt =
        fedora_compat::SeLinuxContext::new("system_u", "object_r", "httpd_sys_content_t", "s0");
    assert!(selinux.authorize_access(&src, &tgt, "file", "read").is_ok());
}

#[test]
fn test_epoll_event_loop_multiplexing() {
    let mut epoll = EpollInstance::new(1, 10);
    let ev1 = EpollEvent::new(EPOLLIN | EPOLLET, 4);
    assert!(epoll.ctl(EpollOp::CtlAdd, 4, Some(ev1)).is_ok());

    epoll.trigger_event(4, EPOLLIN);

    let mut ready = [EpollEvent::new(0, 0); 4];
    let n = epoll.wait(&mut ready);
    assert_eq!(n, 1);
    assert_eq!(ready[0].data.fd, 4);
    assert_eq!(ready[0].events & EPOLLIN, EPOLLIN);
}

#[test]
fn test_elf_dynamic_relocation_resolution() {
    let mut relocator = ElfRelocator::new(0x400000);
    let sym = ElfSymbol::new(b"sys_yield", 0x401050, 64);
    relocator.add_symbol(sym);

    let rel_entry = ElfRelaEntry::new(0x20, R_X86_64_RELATIVE, 0, 0x100);
    let resolved_rel = relocator.resolve_relocation(&rel_entry, None).unwrap();
    assert_eq!(resolved_rel, 0x400100);

    let glob_entry = ElfRelaEntry::new(0x28, R_X86_64_GLOB_DAT, 1, 0x10);
    let resolved_glob = relocator
        .resolve_relocation(&glob_entry, Some(b"sys_yield"))
        .unwrap();
    assert_eq!(resolved_glob, 0x401060);
}

#[test]
fn test_sigmatools_suite() {
    let mut etcher = SovereignDpkgEtcher::new("/dev/nvme0n1p1".to_string());
    assert!(etcher.flash_iso_image(&[0x7F, b'E', b'L', b'F']).is_ok());

    let calc = SovereignIPCalculator;
    let (net, bcast, hosts) = calc.calculate_subnet_details("10.0.0.50", 24).unwrap();
    assert_eq!(net, "10.0.0.0");
    assert_eq!(bcast, "10.0.0.255");
    assert_eq!(hosts, 254);

    let prettifier = SovereignJsonPrettifier;
    let pretty = prettifier.prettify_json("{\"kernel\":\"sigmaos\",\"version\":1}");
    assert!(pretty.contains("\n"));

    let gen = SovereignPasswordGenerator;
    let pass = gen.generate_secure_password(24, true);
    assert_eq!(pass.len(), 24);

    let rtc = AlmeidaCmosRtc::decode_cmos_values(0x00, 0x30, 0x14, 0x15, 0x08, 0x26, true);
    assert_eq!(rtc.format_timestamp(), "2026-08-15 14:30:00");
}

#[test]
fn test_posix_and_nfsv4_acls() {
    // POSIX 1003.1e ACL verification
    let mut posix_acl = PosixAcl::from_mode(1000, 1000, 0o700); // Owner rwx, Group ---, Other ---
    posix_acl.add_entry_direct(AclEntry::new(ControlAclTag::User(1001), 5)); // User 1001 gets r-x (5)

    assert!(posix_acl.evaluate_access(1001, 1001, &[], 1000, 1000, 5)); // Allowed r-x

    let child_posix = posix_acl.inherit_default_acl(false);
    assert_eq!(child_posix.entries.len(), posix_acl.entries.len());

    let mut gate = ZeroTrustAccessGate::new(FilterPolicy::Whitelist, 0xFFFF);
    let allowed_mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
    gate.mac_filter.add_mac(allowed_mac);
    gate.matrix
        .grant_right(1, 10, access_control::acm_rights::READ);
}

#[test]
fn test_alpc_local_procedure_calls() {
    let mut mgr = AlpcManager::new();
    mgr.register_facility_server(AlpcFacility::SecurityAuth, "auth_server");

    let server = mgr
        .get_facility_server_mut(AlpcFacility::SecurityAuth)
        .unwrap();
    server.register_procedure(301, |req| {
        let payload = req.get_payload();
        if payload == b"VERIFY_TOKEN_XYZ" {
            b"TOKEN_VALIDATED_OK".to_vec()
        } else {
            b"TOKEN_INVALID".to_vec()
        }
    });

    let req = AlpcMessage::new_inline(
        100,
        AlpcFacility::SecurityAuth,
        301,
        500,
        1000,
        b"VERIFY_TOKEN_XYZ".to_vec(),
    );

    let reply = mgr.request_reply(AlpcFacility::SecurityAuth, req).unwrap();
    assert_eq!(reply.get_payload(), b"TOKEN_VALIDATED_OK");
    assert_eq!(
        (reply.header.flags & alpc_flags::REPLY_MESSAGE),
        alpc_flags::REPLY_MESSAGE
    );
}

#[test]
fn test_task_states_and_workload_classifications() {
    let mut sched = PriorityScheduler::new();

    let task_cpu = Box::new(
        Task::new(1, Priority::High, 10, TaskCapability::full())
            .with_workload(TaskWorkloadType::CpuBound),
    );
    let task_io = Box::new(
        Task::new(2, Priority::Normal, 10, TaskCapability::full())
            .with_workload(TaskWorkloadType::IoBound),
    );
    let task_rt = Box::new(
        Task::new(3, Priority::Realtime, 5, TaskCapability::full()).with_workload(
            TaskWorkloadType::RealTimePeriodic {
                period_ms: 10,
                exec_time_ms: 2,
            },
        ),
    );

    sched.add_task(task_cpu).unwrap();
    sched.add_task(task_io).unwrap();
    sched.add_task(task_rt).unwrap();

    let scheduled_id = sched.schedule().unwrap();
    assert_eq!(scheduled_id, 3);

    let stats = sched.stats();
    assert_eq!(stats.total_tasks, 3);
    assert_eq!(stats.running_tasks, 1);
    assert_eq!(stats.ready_tasks, 2);
}

#[test]
fn test_file_attributes_and_cpu_ring_privileges() {
    let mut bounds = CapBoundingSet::new(0xFFFF_FFFF);
    assert!(bounds.is_capability_permitted(21));
    bounds.drop_capability(21);
    assert!(!bounds.is_capability_permitted(21));

    let dac = DacPermission::new(1000, 1000, 0o755);
    assert!(dac.evaluate_access(1000, 1000, access_control::dac_flags::READ));
    assert!(!dac.evaluate_access(1001, 1001, access_control::dac_flags::WRITE));

    let mac_sub = MacSecurityLabel::new(SensitivityLevel::Secret, 0x01);
    let mac_obj = MacSecurityLabel::new(SensitivityLevel::Confidential, 0x01);
    assert!(mac_sub.can_read(&mac_obj));
}

#[test]
fn test_two_tier_memory_and_fast_syscalls() {
    let mut allocator = TwoTierMemoryAllocator::new(0x1000_0000, 64);

    let pcb_obj = allocator
        .alloc_slab_object(SlabObjectType::ProcessControlBlock)
        .unwrap();
    let fd_obj = allocator
        .alloc_slab_object(SlabObjectType::FileDescriptor)
        .unwrap();
    let inode_obj = allocator
        .alloc_slab_object(SlabObjectType::InodeStruct)
        .unwrap();

    assert!(pcb_obj >= 0x1000_0000);
    assert!(fd_obj >= 0x1000_0000);
    assert!(inode_obj >= 0x1000_0000);

    allocator.free_slab_object(SlabObjectType::ProcessControlBlock, pcb_obj);

    let mut pt_engine = RecursivePageTableEngine::new(0x0008_0000);
    pt_engine.enable_self_referential_mapping();
    assert_ne!(pt_engine.calculate_pml4_virt_address(), 0);

    let mut cow_engine = CopyOnWriteForkEngine::new();
    cow_engine.fork_share_page(0x1000, 0x1000_0000);

    let mut dispatcher = FastSyscallDispatcher::new();
    dispatcher.configure_fast_syscall(0xFFFFFFFF80102000, 0x08, 0x1B);

    let syscall_matrix = MinimalPosixSyscallMatrix::new();
    let mut frame = TrapRegisterFrame::default();
    frame.rax = posix_syscall_nr::SYS_OPEN;

    let res_fd = dispatcher.dispatch_trap(&mut frame, &syscall_matrix);
    assert_eq!(res_fd, 3);
}

#[test]
fn test_bitmap_pmm_and_syscall_router() {
    let mut pmm = BitmapPhysicalMemoryManager::new(64 * 4096);
    pmm.free_region(0x20000, 16 * 4096);

    let frame_addr = pmm.alloc_block().unwrap();
    assert_eq!(frame_addr, 0x20000);

    let paging = SelfRefPagingEngine::new(0x30000);
    let mut pml4 = [0u64; 512];
    paging.vmm_init_self_reference(&mut pml4);
    assert_eq!(pml4[510], 0x30000 | 3);

    let mut router = SyscallTableRouter::new();
    router.register_handler(2, |a, b, _, _| (a * b) as i64);
    assert_eq!(router.syscall_handler(2, 6, 7, 0), 42);
}

#[test]
fn test_shadow_passwords_usermod_and_sudo_policy() {
    let mut manager = UserManager::new("/tmp/test_etc_shadow_sudo");
    manager.initialize().unwrap();

    let user = manager.create_user("charlie", "Charlie Sysadmin").unwrap();
    assert_eq!(user.username, "charlie");

    manager.set_password("charlie", "P@ssword2026").unwrap();
    assert!(manager.verify_password("charlie", "P@ssword2026"));

    manager
        .usermod(
            "charlie",
            Some("/bin/bash"),
            Some("/home/charlie"),
            None,
            None,
        )
        .unwrap();
    assert_eq!(manager.get_user("charlie").unwrap().shell, "/bin/bash");

    manager.add_user_to_group("charlie", "wheel").unwrap();
    let groups = manager.get_user_groups("charlie");
    assert!(groups.contains(&"wheel".to_string()));

    let sudo_res = manager
        .sudo_engine
        .evaluate_sudo_privilege("charlie", &groups, "/usr/bin/apt");
    assert!(sudo_res.is_ok());
}

#[test]
fn test_statutory_compliance_overlay_and_community_toolkit() {
    let gov = StatutoryGovernanceLayer::new();
    assert!(!gov.rules.is_empty());

    let mut notifier = PenaltyBreachNotifier::new();
    let rule = StatutoryGovernanceRule {
        rule_id: "EPFO-01".to_string(),
        framework: StatutoryFramework::IndianDpdpAct2023,
        description: "Delay in ECR remittance".to_string(),
        status: ComplianceRuleStatus::Breached,
        max_penalty_amount_usd: 2500,
    };
    notifier.notify_breach(&rule, "Delay in ECR remittance", 1700000000);
    assert_eq!(notifier.alerts.len(), 1);

    let mut rollback = DisputeAuditRollbackEngine::new();
    rollback.create_audit_checkpoint(100, "hash:state100");
    assert_eq!(
        rollback.rollback_dispute_checkpoint(100).unwrap(),
        "hash:state100"
    );

    let handbook = CommunityHandbookCatalog::new();
    assert!(!handbook.articles.is_empty());

    let recipes = ReproduciblePackageRecipeManager::new();
    assert!(!recipes.recipes.is_empty());

    let sec = SecurityProfileTemplateStore::new();
    assert!(sec.templates.contains_key("browser_sandboxed"));
}

#[test]
fn test_freebsd_jail_manager_inspection() {
    let mut mgr = bsd::FreeBsdJailManager::new();
    let jid = mgr
        .create_jail("web1.jail.local", "192.168.1.100", "/jails/web1")
        .unwrap();
    assert_eq!(jid, 1);
    assert!(mgr.check_network_allowed(jid, "192.168.1.100"));
    assert!(mgr.stop_jail(jid).is_ok());
    assert!(!mgr.check_network_allowed(jid, "192.168.1.100"));
}

#[test]
fn test_openbsd_sysctl_mib_inspection() {
    let mut mib = bsd::OpenBsdSysctlKernelMib::new();
    assert_eq!(mib.query_mib("kern.securelevel").unwrap(), "0");
    assert!(mib.is_raw_disk_write_allowed());

    assert!(mib.write_mib("kern.securelevel", "1").is_ok());
    assert!(!mib.is_raw_disk_write_allowed());
    assert!(mib.write_mib("kern.securelevel", "0").is_err());
}

#[test]
fn test_netbsd_rump_router_inspection() {
    let mut router = distro_inspirations::NetBsdRumpRouter::new();
    router.register_driver(distro_inspirations::RumpDriver {
        name: "pci_net".to_string(),
        context: distro_inspirations::DriverContext::KernelSpace,
        operations_handled: vec!["send_packet".to_string()],
    });
    let res = router.dispatch_hypercall("pci_net", "send_packet");
    assert!(res.is_ok());
}

#[test]
fn test_sovereign_landlock_and_runit_inspection() {
    let mut sandbox = distro_inspirations::SovereignLandlockLsm::new();
    assert!(sandbox
        .add_rule("/usr/share", distro_inspirations::LandlockAccess::ReadOnly)
        .is_ok());

    let mut runit = distro_inspirations::SovereignRunitSupervisor::new(
        distro_inspirations::RunitRunlevel::Default,
    );
    runit.register_service("nginx", distro_inspirations::RunitRunlevel::Default, &[], 3);
    assert_eq!(runit.tick_supervision(), 1);
    assert_eq!(
        runit.get_service_status("nginx").unwrap(),
        distro_inspirations::RunitServiceStatus::Running
    );
}

#[test]
fn test_sovereign_ostree_and_io_uring_inspection() {
    let mut ostree = distro_inspirations::SovereignOstreeEngine::new();
    let idx = ostree.stage_commit("commit-1.0.0", "1.0.0", "vmlinuz-6.8", 0x123456);
    assert_eq!(idx, 0);

    let mut io_ring = distro_inspirations::SovereignIoUring::new(64);
    assert!(io_ring
        .submit_entry(distro_inspirations::SubmissionQueueEntry {
            opcode: distro_inspirations::IoUringOpcode::Read,
            fd: 1,
            offset: 0,
            user_data: 100,
            data: vec![0u8; 512],
        })
        .is_ok());
    assert_eq!(io_ring.submit_and_wait(), 1);
}

#[test]
fn test_universal_server_image_adapter_flatpak_appimage() {
    use sigpkg::universal_adapter::{ServerImageFormat, UniversalServerImageAdapter};

    let adapter = UniversalServerImageAdapter::new();
    let flatpak_manifest = r#"
        name: org.kde.kdenlive
        version: 23.08.4
        distro: Flatpak
        cmd: kdenlive
    "#;

    let meta_flatpak = adapter
        .parse_server_image_manifest(ServerImageFormat::FlatpakBundleRef, flatpak_manifest)
        .unwrap();
    assert_eq!(meta_flatpak.name, "org.kde.kdenlive");
    assert_eq!(meta_flatpak.version, "23.08.4");
    assert_eq!(meta_flatpak.target_distro, "Flatpak");

    let appimage_manifest = r#"
        name: GIMP-AppImage
        version: 2.10.36
        distro: AppImage
        cmd: AppRun
    "#;

    let meta_appimage = adapter
        .parse_server_image_manifest(ServerImageFormat::AppImageSquashFs, appimage_manifest)
        .unwrap();
    assert_eq!(meta_appimage.name, "GIMP-AppImage");
    assert_eq!(meta_appimage.entry_cmd, Some("AppRun".to_string()));
}

#[test]
fn test_device_manager_and_simple_device() {
    use device_manager::{
        Device, DeviceClass, DeviceManager, PowerState, SimpleDevice, SimpleDeviceManager,
    };

    let dev = SimpleDevice::new(42, b"sovereign_nvme_drive", DeviceClass::Block);
    assert_eq!(dev.id(), 42);
    assert_eq!(dev.name(), b"sovereign_nvme_drive");
    assert_eq!(dev.device_class(), DeviceClass::Block);
    assert_eq!(dev.get_power_state(), PowerState::D0);

    let mut mgr = SimpleDeviceManager::new();
    let id = mgr.register_device(alloc::boxed::Box::new(dev)).unwrap();
    assert_eq!(id, 42);
    let registered_dev = mgr.get_device(42).unwrap();
    assert_eq!(registered_dev.name(), b"sovereign_nvme_drive");
}

#[test]
fn test_antix_linux_parity_and_lightweight_init() {
    use antix_compat::{
        AntiXControlCentre, AntiXInitSwitcher, AntiXInitSystem, AntiXPersistenceManager,
        AntiXPersistenceMode, AntixCliToolsSuite, AntixKernelUpdater, AntixPackageInstallerShim,
        CliTool, KernelVariant, LightweightApp,
    };

    let mut switcher = AntiXInitSwitcher::new(AntiXInitSystem::Runit);
    let process_id = switcher.dispatch_fast_init_process("runsvdir").unwrap();
    assert_eq!(process_id, 1);

    let mut pm = AntiXPersistenceManager::new(AntiXPersistenceMode::HomePersistence);
    assert!(pm.mount_overlay());
    pm.save_state_snapshot(1024);
    assert_eq!(pm.sync_ram_overlay_to_disk().unwrap(), 1024);

    let mut cc = AntiXControlCentre::new();
    let ram_msg = cc.apply_antix_64mb_ram_guard(64);
    assert!(ram_msg.contains("64MB RAM constraint detected"));

    let pkg_msg = AntixPackageInstallerShim::install_app(LightweightApp::DilloBrowser);
    assert!(pkg_msg.contains("Dillo"));

    let cli_msg = AntixCliToolsSuite::execute_cli_tool(CliTool::CliApti);
    assert!(cli_msg.contains("cli-apti"));

    let kernel_updater = AntixKernelUpdater::new();
    let k_msg = kernel_updater.switch_kernel_variant(KernelVariant::Kernel486NonPae);
    assert!(k_msg.contains("non-PAE"));
}
