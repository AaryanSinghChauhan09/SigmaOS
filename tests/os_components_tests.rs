// SigmaOS Comprehensive OS Components Integration & Unit Test Suite
// Verifies sovereign subsystem capabilities, compatibility layers, drivers, security, and tools.

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

use community_toolkit::{
    CommunityHandbookCatalog, HybridFirewallTemplateStore, ReproduciblePackageRecipeManager,
    SecurityProfileTemplateStore, VirtualizationBlueprintStore,
};
use statutory_compliance::{
    BreachSeverity, DisputeAuditRollbackEngine, PenaltyBreachNotifier, StatutoryAuthority,
    StatutoryGovernanceLayer,
};
use system_user::{ShadowEntry, SudoPolicyEngine, SudoersRule, UserError, UserManager as TestUserManager};

use access_control::{
    AclEntry, AclTag, CpuPrivilegeEnforcer, ExecutionRingMode, FileAttributeAccessControl,
    Nfs4Ace, Nfs4AceType, Nfs4Acl, PosixAcl, file_attribute_flags, nfs4_flags, nfs4_mask,
};
use alpc::{AlpcFacility, AlpcManager, AlpcMessage, alpc_flags};
use bitmap_pmm::{
    BitmapPhysicalMemoryManager, SelfReferentialPagingEngine as SelfRefPagingEngine, SyscallTableRouter,
};
use low_level_memory::{
    CopyOnWriteForkEngine, FastSyscallDispatcher, MinimalPosixSyscallMatrix, RecursivePageTableEngine,
    SlabObjectType, TrapRegisterFrame, TwoTierMemoryAllocator, posix_syscall_nr,
};
use task_scheduler::{
    Priority, PriorityScheduler, Scheduler, Task, TaskCapability, TaskState, TaskWorkloadType,
};

use pipes::Pipe;
use unveil::{UnveilManager, UnveilPermission};
use geom::{GeomProvider, GeomTopology, BioRequest};
use audio_editor::{MultiTrackSession, AudioTrack, SpectralNoiseSuppressionEffect, AudioEffect};
use video_editor::{VideoTimeline, VideoTrack, VideoClip, ExportProfile, ExportFormat};
use chimera_linux::{DinitServiceManager, DinitService, BsdUserlandCompat, ApkPackageStore, ApkPackageMetadata};
use debian_compat::{DebianAlternativesSystem, AptRepositorySync, DebianChannel};
use cachy_os::{BoreSchedulerGovernor, AnanicyManager, SchedPolicy};
use endeavour_os::{ReflectorMirrorManager, PacmanMirror, YayParuHelper, AurPackageSpec};
use fedora_compat::{DnfPackageResolver, SeLinuxEngine, SeLinuxContext};
use sigmatools::*;

use sigma_fs_extended::{Blake3BlockDeduplicationEngine, PfsType, PseudoFilesystemNamespace};

use segmentation_paging::{
    AddressBindingMode, CpuPrivilegeMode as SegCpuPrivilegeMode, GlobalDescriptorTable,
    MultiLevelPagingEngine, ProtectionLevel as SegProtectionLevel, ProtectionViolationType,
    RandomizedAddressSpace, SegmentDescriptor, SegmentType, SegmentedAddress,
};

use process_activity_manager::{
    ActivityState, ProcessActivityManager, RegisterSnapshot as ProcRegisterSnapshot,
    ResourceUsageMetrics,
};

#[test]
fn test_segmentation_paging_and_aslr() {
    let mut gdt = GlobalDescriptorTable::new();
    let code_desc = SegmentDescriptor::code_segment(
        0x00000000,
        0xFFFFFFFF,
        SegProtectionLevel::KernelRing0,
    );
    let selector = gdt.insert_descriptor(code_desc);
    assert_eq!(selector.index, 1);

    let seg_addr = SegmentedAddress {
        selector,
        offset: 0x00001000,
    };
    let linear = gdt.translate_address(seg_addr, SegCpuPrivilegeMode::KernelRing0).unwrap();
    assert_eq!(linear, 0x00001000);

    let mut paging = MultiLevelPagingEngine::new();
    paging.map_page(0x00007FFF00000000, 0x0000000100000000, false, true, false).unwrap();

    let pte = paging.walk_page_table(0x00007FFF00000000).unwrap();
    assert_eq!(pte.get_physical_address(), 0x0000000100000000);

    assert_eq!(
        paging.verify_execution_access(0x00007FFF00000000, false, true, true),
        Err(ProtectionViolationType::SmepViolation)
    );

    let aslr = RandomizedAddressSpace::new(0x12345678);
    let base = aslr.generate_random_base(AddressBindingMode::DynamicRunTime, 0x0001_0000_0000);
    assert!(base >= 0x0001_0000_0000);
}

#[test]
fn test_regex_unveil_and_glob_matching() {
    let mut unveil_mgr = UnveilManager::new();
    unveil_mgr.unveil("/var/log/*.log", "r").unwrap();
    assert!(unveil_mgr.validate_path("/var/log/syslog.log", UnveilPermission::Read).is_ok());
    assert!(unveil_mgr.validate_path("/var/log/syslog.txt", UnveilPermission::Read).is_err());
}

#[test]
fn test_hammer2_pfs_namespaces_and_blake3_dedup() {
    let mut pfs = PseudoFilesystemNamespace::new("root_master", PfsType::Master);
    pfs.file_map.insert("/etc/hostname".to_string(), "blake3-hash1".to_string());

    let snap = PseudoFilesystemNamespace::snapshot("root_snap_1", "root_master", pfs.file_map.clone());
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
    let mut pam = ProcessActivityManager::new();
    pam.register_process(500, "chrome", "/usr/bin/chrome").unwrap();
    pam.register_thread(500, 501, "render_main").unwrap();

    pam.set_foreground_process(500).unwrap();
    let active_procs = pam.get_active_processes();
    assert_eq!(active_procs.len(), 1);
    assert_eq!(active_procs[0].state, ActivityState::Interactive);

    let ctx = ProcRegisterSnapshot {
        rip: 0x00007FFF00002000,
        rsp: 0x00007FFFFFFFD000,
        rax: 1,
        ..Default::default()
    };
    pam.save_thread_context(500, 501, ctx).unwrap();

    let loaded_ctx = pam.get_thread_context(500, 501).unwrap();
    assert_eq!(loaded_ctx.rip, 0x00007FFF00002000);
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

    assert!(mgr.validate_path("/usr/bin/cargo", UnveilPermission::Read).is_ok());
    assert!(mgr.validate_path("/usr/bin/cargo", UnveilPermission::Execute).is_ok());
    assert!(mgr.validate_path("/usr/bin/cargo", UnveilPermission::Write).is_err());

    assert!(mgr.validate_path("/etc/nginx/nginx.conf", UnveilPermission::Read).is_ok());
    assert!(mgr.validate_path("/etc/nginx/nginx.conf", UnveilPermission::Write).is_err());
}

#[test]
fn test_geom_storage_topology_and_geli() {
    let mut geom = GeomTopology::new();
    let disk = GeomProvider::new("ada0", 8192, 512);
    geom.register_provider(disk);

    assert!(geom.create_partition("ada0", "ada0p1", 0, 4096, "freebsd-ufs").is_ok());
    assert!(geom.create_eli("ada0p1", "ada0p1.eli", "sovereign_pass").is_ok());

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

    let mut dsp_buf = [0.02, 0.80, -0.01];
    let noise_suppress = SpectralNoiseSuppressionEffect::new(0.05, 12.0);
    noise_suppress.apply(&mut dsp_buf);
    assert!(dsp_buf[0].abs() < 0.01);
    assert!(dsp_buf[1] > 0.70);
}

#[test]
fn test_video_editor_sigmacut_engine() {
    let mut timeline = VideoTimeline::new(1920, 1080);
    let mut track = VideoTrack::new(1);

    let clip = VideoClip::new("intro.mp4", 0, 60);
    track.add_clip(clip);
    timeline.add_track(track);

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
    let mut alts = DebianAlternativesSystem::new("editor".to_string());
    alts.register_alternative("/usr/bin/editor".to_string(), "/usr/bin/vim".to_string(), 50);
    alts.register_alternative("/usr/bin/editor".to_string(), "/usr/bin/nano".to_string(), 100);

    assert_eq!(alts.get_active_target().unwrap(), "/usr/bin/nano");

    let mut repo = AptRepositorySync::new(DebianChannel::Stable, "http://deb.debian.org/debian".to_string());
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

    let selinux = SeLinuxEngine::new(true);
    let httpd_sub = SeLinuxContext::new("system_u", "system_r", "httpd_t", "s0");
    let html_obj = SeLinuxContext::new("system_u", "object_r", "httpd_sys_content_t", "s0");
    assert!(selinux.authorize_access(&httpd_sub, &html_obj, "file", "read").is_ok());
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
    posix_acl.add_entry(AclEntry::new(AclTag::User(1001), 5)); // User 1001 gets r-x (5)

    assert!(posix_acl.get_mask().is_some());
    assert!(posix_acl.evaluate_access(1001, 1001, &[], 1000, 1000, 5)); // Allowed r-x
    assert!(!posix_acl.evaluate_access(1001, 1001, &[], 1000, 1000, 2)); // Denied write (2)
    assert!(!posix_acl.evaluate_access(1002, 1002, &[], 1000, 1000, 4)); // Other denied

    let child_posix = posix_acl.inherit_default_acl(false);
    assert_eq!(child_posix.get_mask(), Some(4)); // Execute bit stripped for file child

    // NFSv4 / FreeBSD Rich ACL verification
    let mut nfsv4_acl = Nfs4Acl::new();
    nfsv4_acl.add_ace(Nfs4Ace::new(Nfs4AceType::AccessDenied, 0, nfs4_mask::DELETE, 1002));
    nfsv4_acl.add_ace(Nfs4Ace::new(
        Nfs4AceType::AccessAllowed,
        nfs4_flags::FILE_INHERIT | nfs4_flags::DIRECTORY_INHERIT,
        nfs4_mask::READ_DATA | nfs4_mask::WRITE_DATA | nfs4_mask::DELETE,
        65534, // Everyone
    ));

    assert!(nfsv4_acl.evaluate_access(1002, 1002, nfs4_mask::READ_DATA));
    assert!(!nfsv4_acl.evaluate_access(1002, 1002, nfs4_mask::DELETE));
    assert!(nfsv4_acl.evaluate_access(1003, 1003, nfs4_mask::DELETE));

    let child_nfsv4 = nfsv4_acl.inherit_for_child(true);
    assert_eq!(child_nfsv4.aces.len(), 1);
}

#[test]
fn test_alpc_local_procedure_calls() {
    let mut mgr = AlpcManager::new();
    mgr.register_facility_server(AlpcFacility::SecurityAuth, "auth_server");

    let server = mgr.get_facility_server_mut(AlpcFacility::SecurityAuth).unwrap();
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
    assert_eq!((reply.header.flags & alpc_flags::REPLY_MESSAGE), alpc_flags::REPLY_MESSAGE);
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
        Task::new(3, Priority::Realtime, 5, TaskCapability::full())
            .with_workload(TaskWorkloadType::RealTimePeriodic {
                period_ms: 10,
                exec_time_ms: 2,
            }),
    );

    sched.add_task(task_cpu).unwrap();
    sched.add_task(task_io).unwrap();
    sched.add_task(task_rt).unwrap();

    let scheduled_id = sched.schedule().unwrap();
    assert_eq!(scheduled_id, 3); // Realtime periodic task scheduled first

    let stats = sched.stats();
    assert_eq!(stats.total_tasks, 3);
    assert_eq!(stats.running_tasks, 1);
    assert_eq!(stats.ready_tasks, 2);
}

#[test]
fn test_file_attributes_and_cpu_ring_privileges() {
    let imm = FileAttributeAccessControl::new(file_attribute_flags::IMMUTABLE);
    assert!(!imm.can_modify(false, true)); // Overwrite denied for root
    assert!(!imm.can_modify(true, true)); // Append denied for root
    assert!(!imm.can_unlink()); // Unlink denied

    let app = FileAttributeAccessControl::new(file_attribute_flags::APPEND_ONLY);
    assert!(app.can_modify(true, false)); // Append allowed for normal user
    assert!(!app.can_modify(false, false)); // Overwrite denied for normal user
    assert!(!app.can_unlink()); // Unlink denied

    let nounlink = FileAttributeAccessControl::new(file_attribute_flags::NO_UNLINK);
    assert!(nounlink.can_modify(false, false)); // Overwrite allowed
    assert!(!nounlink.can_unlink()); // Unlink denied

    let dump_file = FileAttributeAccessControl::new(file_attribute_flags::NO_DUMP);
    assert!(!dump_file.can_dump());

    let ring0 = CpuPrivilegeEnforcer::new(ExecutionRingMode::Ring0Supervisor);
    assert!(ring0.can_execute_privileged_instruction());

    let ring3 = CpuPrivilegeEnforcer::new(ExecutionRingMode::Ring3User);
    assert!(!ring3.can_execute_privileged_instruction());
}

#[test]
fn test_two_tier_memory_and_fast_syscalls() {
    let mut allocator = TwoTierMemoryAllocator::new(0x1000_0000, 64);

    let pcb_obj = allocator.alloc_slab_object(SlabObjectType::ProcessControlBlock).unwrap();
    let fd_obj = allocator.alloc_slab_object(SlabObjectType::FileDescriptor).unwrap();
    let inode_obj = allocator.alloc_slab_object(SlabObjectType::InodeStruct).unwrap();

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
    let mut manager = TestUserManager::new("/tmp/test_etc_shadow_sudo");
    manager.initialize().unwrap();

    let user = manager.create_user("charlie", "Charlie Sysadmin").unwrap();
    assert_eq!(user.username, "charlie");

    manager.set_password("charlie", "P@ssword2026").unwrap();
    assert!(manager.verify_password("charlie", "P@ssword2026"));

    manager.usermod("charlie", Some("/bin/bash"), Some("/home/charlie"), None, None).unwrap();
    assert_eq!(manager.get_user("charlie").unwrap().shell, "/bin/bash");

    manager.add_user_to_group("charlie", "wheel").unwrap();
    let groups = manager.get_user_groups("charlie");
    assert!(groups.contains(&"wheel".to_string()));

    let sudo_res = manager.sudo_engine.evaluate_sudo_privilege("charlie", &groups, "/usr/bin/apt");
    assert!(sudo_res.is_ok());
}

#[test]
fn test_statutory_compliance_overlay_and_community_toolkit() {
    let gov = StatutoryGovernanceLayer::new();
    let authorities = gov.evaluate_applicability(25, 18000.0);
    assert!(authorities.contains(&StatutoryAuthority::EpfoSocialSecurity));

    let mut notifier = PenaltyBreachNotifier::new();
    let alert_id = notifier.issue_breach_alert(
        StatutoryAuthority::EpfoSocialSecurity,
        BreachSeverity::MajorNonCompliance,
        2500.0,
        "Delay in ECR remittance",
        1700000000,
    );
    assert_eq!(alert_id, 1001);
    assert_eq!(notifier.get_total_penalty(), 2500.0);

    let mut rollback = DisputeAuditRollbackEngine::new();
    rollback.create_dispute_checkpoint(100, "Form GSTR-3B", "hash:state100", 1700000000);
    assert_eq!(rollback.resolve_dispute_and_rollback(100).unwrap(), "hash:state100");

    let handbook = CommunityHandbookCatalog::new();
    assert!(handbook.get_article("sigma-handbook-01").is_some());

    let mut recipes = ReproduciblePackageRecipeManager::new();
    recipes.register_recipe("nginx", "1.24.0", "sha256:112233", &["pcre"]);
    assert!(recipes.recipes.contains_key("nginx"));

    let sec = SecurityProfileTemplateStore::new();
    assert!(sec.profiles.contains_key("hardened-webserver"));

    let fw = HybridFirewallTemplateStore::new();
    assert!(fw.templates.contains_key("default-mesh-shield"));

    let virt = VirtualizationBlueprintStore::new();
    assert!(virt.blueprints.contains_key("micro-vm-node"));
}
