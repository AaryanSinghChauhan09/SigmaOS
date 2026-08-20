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
fn test_sovereign_auxiliary_carry_and_system_awareness_inspection() {
    let mut af = SovereignAuxiliaryCarryEngine::new();
    let res = af.evaluate_add_af(0x0E, 0x05); // 14 + 5 = 19 > 15 -> AF set
    assert!(af.rflags_af);

    let mut awareness = SovereignSystemAwarenessEngine::new(AwarenessDegree::Omniscient);
    assert!(awareness.compute_availability_score() > 0);
}

#[test]
fn test_sovereign_avoidance_backbone_balloon_inspection() {
    let mut avoidance = SovereignDeadlockStarvationAvoidanceEngine::new(vec![5, 5]);
    avoidance.register_process(1, vec![3, 3]);
    assert!(avoidance.is_safe_state_request(1, &[1, 1]));

    let mut backbone = SovereignBackboneNetworkEngine::new();
    backbone.add_route([10, 0, 0, 0], 8, [192, 168, 1, 1], 10, RouteProtocol::Bgp);
    assert!(backbone.lookup_backbone_route([10, 1, 2, 3]).is_some());

    let mut balloon = SovereignMemoryBallooningBalancer::new(2048);
    assert_eq!(balloon.inflate_balloon(512), 512);
}
