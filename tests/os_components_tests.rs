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

#[path = "../src/access/control.rs"]
mod access_control;

#[path = "../src/tools/sigmatools.rs"]
mod sigmatools;

use access_control::{
    AclEntry, AclTag, Nfs4Ace, Nfs4AceType, Nfs4Acl, PosixAcl, nfs4_flags, nfs4_mask,
};
use alpc::{AlpcFacility, AlpcManager, AlpcMessage, alpc_flags};
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

    let mut repo = AptRepositorySync::new(DebianChannel::Stable, "https://deb.debian.org/debian".to_string());
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
