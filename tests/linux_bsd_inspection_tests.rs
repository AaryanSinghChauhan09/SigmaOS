// SigmaOS Linux & BSD Parity Inspection Unit Test Suite
// Verifies working mechanisms inspired by Linux and BSD distributions:
// - FreeBSD Jails & sysctl MIB
// - NetBSD Rump Kernel hypercall routing
// - OpenBSD sysctl MIB
// - Linux LSB / fstab parsing

#[path = "../src/compatibility/bsd.rs"]
mod bsd;

#[path = "../src/distro/linux_bsd_inspirations.rs"]
mod distro_inspirations;

use bsd::*;
use distro_inspirations::*;

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
fn test_sovereign_ostree_and_io_uring_inspection() {
    let mut ostree = SovereignOstreeEngine::new();
    let idx = ostree.stage_commit("commit_hash_123", "v1.0.0-release", "kernel-6.8", 0xABCDEF);
    assert_eq!(idx, 0);

    let mut io_uring = SovereignIoUring::new(16);
    let sqe = SubmissionQueueEntry {
        opcode: IoUringOpcode::Nop,
        fd: 0,
        offset: 0,
        data: vec![42],
        user_data: 42,
    };
    assert!(io_uring.submit_entry(sqe).is_ok());
    let processed = io_uring.submit_and_wait();
    assert_eq!(processed, 1);
}

#[test]
fn test_openbsd_pledge_unveil_sentinel_inspection() {
    let mut sentinel = OpenBsdPledgeUnveilSentinel::new();
    assert!(sentinel.pledge_process(101, &["stdio", "rpath"]).is_ok());
    assert!(sentinel.unveil_process(101, "/usr/share", "r").is_ok());

    assert!(sentinel.audit_syscall(101, 100, "rpath", Some("/usr/share/doc")));
    assert!(!sentinel.audit_syscall(101, 101, "wpath", Some("/usr/share/doc")));
    assert_eq!(sentinel.audit_log.len(), 1);
}

#[test]
fn test_ebpf_verification_and_interpreter_inspection() {
    let mut engine = SovereignEbpfEngine::new(64);
    let instrs = vec![
        EbpfInstruction {
            opcode: EbpfOpcode::Add,
            dst: 1,
            src: 0,
            offset: 0,
            imm: 15,
            use_imm: true,
        },
        EbpfInstruction {
            opcode: EbpfOpcode::Add,
            dst: 0,
            src: 1,
            offset: 0,
            imm: 0,
            use_imm: false,
        },
        EbpfInstruction {
            opcode: EbpfOpcode::Exit,
            dst: 0,
            src: 0,
            offset: 0,
            imm: 0,
            use_imm: false,
        },
    ];

    assert_eq!(engine.execute(&instrs).unwrap(), 15);
}

#[test]
fn test_nix_store_gc_and_dedup_inspection() {
    let mut store = NixStyleStore::new("/sigma/store".to_string());
    let path1 = store.register_path(b"core-lib", Vec::new());
    let path2 = store.register_path(b"cli-app", vec![path1.clone()]);
    let path3 = store.register_path(b"unused-dep", Vec::new());

    store.add_gc_root(path2.clone());
    let collected = store.garbage_collect();

    assert!(collected.contains(&path3));
    assert!(!collected.contains(&path1));
    assert!(!collected.contains(&path2));
}

#[test]
fn test_gentoo_use_flags_and_conflicts_inspection() {
    let mut pm = GentooUseFlagsManager::new();
    pm.set_global_flags(&["ssl", "nls", "wayland"]);
    pm.set_package_override("gui-libs/gtk", &["-x11", "opengl"]);

    assert!(pm.is_flag_enabled("gui-libs/gtk", "wayland"));
    assert!(!pm.is_flag_enabled("gui-libs/gtk", "x11"));
    assert!(pm.is_flag_enabled("gui-libs/gtk", "opengl"));

    let reqs = vec!["opengl", "!x11"];
    assert!(pm.verify_requirements("gui-libs/gtk", &reqs).is_ok());
}

#[test]
fn test_sovereign_landlock_and_runit_inspection() {
    let mut landlock = SovereignLandlockLsm::new();
    assert!(landlock.add_rule("/etc/sigma/config", LandlockAccess::ReadOnly).is_ok());
    landlock.restrict_self();
    assert!(landlock.check_access("/etc/sigma/config", LandlockAccess::ReadOnly));
    assert!(!landlock.check_access("/etc/sigma/config", LandlockAccess::ReadWrite));

    let mut supervisor = SovereignRunitSupervisor::new(RunitRunlevel::Boot);
    supervisor.register_service("network-daemon", RunitRunlevel::Boot, &[], 3);
    assert_eq!(supervisor.services.len(), 1);
    assert_eq!(supervisor.services[0].name, "network-daemon");
}
