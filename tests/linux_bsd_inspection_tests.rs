// SigmaOS Linux & BSD Parity Inspection Unit Test Suite
// Verifies working mechanisms inspired by Linux and BSD distributions:
// - FreeBSD Jails & sysctl MIB
// - NetBSD Rump Kernel hypercall routing
// - OpenBSD sysctl MIB
// - Linux eBPF VM & OSTree Engine & Declarative System Engine

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
fn test_sovereign_ebpf_and_ostree_inspection() {
    let mut engine = SovereignEbpfEngine::new(64);
    let instrs = vec![
        EbpfInstruction {
            opcode: EbpfOpcode::Add,
            dst: 0,
            src: 0,
            offset: 0,
            imm: 42,
            use_imm: true,
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
    let res = engine.execute(&instrs).unwrap();
    assert_eq!(res, 42);

    let mut ostree = SovereignOstreeEngine::new();
    let idx = ostree.stage_commit("commit1", "1.0.0", "vmlinuz-1.0", 0x1234);
    assert_eq!(idx, 0);
    assert_eq!(ostree.get_active_deployment().unwrap().version, "1.0.0");
}

#[test]
fn test_sovereign_declarative_engine_inspection() {
    let mut decl = SovereignDeclarativeSystemEngine::new();
    let gen1 = decl.build_generation("sigma-node", &["coreutils"], &["syslogd"]);
    assert_eq!(gen1, 1);
    assert!(decl.generations[0].active);
}
