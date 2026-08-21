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
    CommunityHandbookCatalog, ReproduciblePackageRecipeManager,
    SecurityProfileTemplateStore,
};
use statutory_compliance::{
    ComplianceRuleStatus, DisputeAuditRollbackEngine, PenaltyBreachNotifier, StatutoryFramework,
    StatutoryGovernanceLayer, StatutoryGovernanceRule,
};
use system_user::{ShadowEntry, SudoPolicyEngine, SudoersRule, UserError, UserManager as TestUserManager};

use access_control::SimpleAccessController;
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
    AslrEntropyConfig, RandomizedAddressSpace, SegmentDescriptor,
};

use process_activity_manager::{
    ActivityState, RegisterSnapshot as ProcRegisterSnapshot,
};

#[test]

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
    let mut track = VideoTrack::new(1, "Track1");

    let clip = VideoClip::new(1, "intro.mp4", 0, 60);
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
}
