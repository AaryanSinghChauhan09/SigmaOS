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

#[test]
fn test_gentoo_use_flags_and_arch_deps_inspection() {
    let mut use_mgr = GentooUseFlagsManager::new();
    use_mgr.set_global_flags(&["wayland", "vulkan"]);
    assert!(use_mgr.is_flag_enabled("zenith-desktop", "wayland"));
    assert!(use_mgr.is_flag_enabled("zenith-desktop", "vulkan"));
    assert!(!use_mgr.is_flag_enabled("zenith-desktop", "x11"));

    let mut resolver = ArchDependencyResolver::new();
    resolver.add_package(PackageNode {
        name: "zenith-desktop".to_string(),
        version: "1.0.0".to_string(),
        dependencies: vec!["wayland-protocols".to_string(), "pixman".to_string()],
        provides: vec![],
    });
    resolver.add_package(PackageNode {
        name: "wayland-protocols".to_string(),
        version: "1.32".to_string(),
        dependencies: vec![],
        provides: vec![],
    });
    resolver.add_package(PackageNode {
        name: "pixman".to_string(),
        version: "0.42.2".to_string(),
        dependencies: vec![],
        provides: vec![],
    });

    let order = resolver.resolve_dependencies("zenith-desktop").unwrap();
    assert_eq!(order.len(), 3);
    assert_eq!(order.last().unwrap(), &"zenith-desktop".to_string());
}

#[test]
fn test_hammer2_and_zfs_cow_snapshots_inspection() {
    let mut hammer2 = Hammer2MultiVersionEngine::new();
    hammer2.write_inode(100, "/etc/sovereign.conf", b"HASH_PAYLOAD_100");
    assert_eq!(hammer2.inodes.len(), 1);

    let snap_txg = hammer2.create_snapshot();
    assert_eq!(snap_txg, 1);

    let mut zfs = SovereignZfsPoolEngine::new("rpool", ZfsVdevType::Mirror);
    zfs.create_dataset("ds0");
    let res = zfs.write_block_cow("ds0", 1, b"BLOCK_DATA");
    assert!(res.is_ok());
}
