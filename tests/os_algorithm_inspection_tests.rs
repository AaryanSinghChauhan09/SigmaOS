// SigmaOS Comprehensive OS Algorithm Inspection Integration & Unit Test Suite
// Verifies deep kernel algorithms: CachyOS BORE scheduler, Anticipatory I/O elevator,
// Demand paging page-fault resolution, VFS hard link & inode life-cycle,
// Packet ring-buffer analysis, and Kernel Security Hardening (KASLR, SMEP/SMAP, Syscall dispatch).

#[path = "../src/distro/linux_bsd_inspirations.rs"]
mod linux_bsd_inspirations;

#[path = "../src/kernel/sched/comprehensive_schedulers.rs"]
mod comprehensive_schedulers;

#[path = "../src/kernel/paging.rs"]
mod kernel_paging;

#[path = "../src/filesystem/vfs.rs"]
mod vfs;

#[path = "../src/network/analyzer.rs"]
mod network_analyzer;

#[path = "../src/security/kernel_hardening.rs"]
mod kernel_hardening;

use comprehensive_schedulers::{AnticipatoryIoScheduler, DiskIoRequest};
use kernel_hardening::{
    HardenedSyscallDispatcher, SmepSmapEnforcer, SovereignKaslrEngine, SyscallFilterRule,
};
use kernel_paging::{DemandPageType, DemandPageZone, DemandPagingSubsystem, PageFaultReason};
use linux_bsd_inspirations::CachyBoreScheduler;
use network_analyzer::{OperatingSystemType, PacketHeader, PacketRingBuffer, ProtocolType};
use vfs::{InodeType, Vfs};

#[test]
fn test_cachy_bore_scheduler_algorithm_inspection() {
    let mut bore = CachyBoreScheduler::new();
    bore.register_task(101, 0, 0); // Interactive task
    bore.register_task(102, 0, 0); // Batch task

    // Simulate task execution bursts
    bore.update_task_execution(101, 15, 200, 4); // Short burst, long sleep -> interactive
    bore.update_task_execution(102, 300, 10, 4); // Long burst, short sleep -> CPU bound

    let selected_interactive = bore.pick_next_task(4);
    assert_eq!(
        selected_interactive,
        Some(101),
        "CachyOS BORE algorithm must prioritize interactive tasks with higher score"
    );
}

#[test]
fn test_anticipatory_io_scheduler_algorithm_inspection() {
    let mut io_sched = AnticipatoryIoScheduler::new();

    // Enqueue requests across different cylinder positions
    io_sched.enqueue_request(DiskIoRequest::simple(1, 10, 100, true));
    io_sched.enqueue_request(DiskIoRequest::simple(2, 11, 105, true)); // Spatial locality
    io_sched.enqueue_request(DiskIoRequest::simple(3, 12, 500, false)); // Far away write

    let req1 = io_sched.dispatch_next().unwrap();
    assert_eq!(req1.pid, 1);

    let req2 = io_sched.dispatch_next().unwrap();
    assert_eq!(
        req2.pid, 2,
        "Anticipatory scheduler must exploit spatial locality for read operations"
    );
}

#[test]
fn test_demand_paging_subsystem_algorithm_inspection() {
    let mut paging = DemandPagingSubsystem::new(1024 * 1024);
    paging.map_demand_zone(DemandPageZone {
        start_vaddr: 0x00400000,
        page_count: 10,
        zone_type: DemandPageType::AnonymousZero,
        read_only: false,
    });

    let res = paging.handle_demand_fault(0x00400008, PageFaultReason::PageNotPresent);
    assert!(
        res.is_ok(),
        "Demand paging must allocate page on zero-fill page fault"
    );
    assert_eq!(paging.get_active_mapped_pages_count(), 1);
}

#[test]
fn test_vfs_inode_and_link_count_algorithm_inspection() {
    let mut vfs = Vfs::new();

    let root = vfs.get_root();
    let file_inode = vfs
        .create_file(root, "test_file.txt", InodeType::File)
        .expect("File creation failed");

    // Inspect initial link counts
    let inode_ref = vfs.get_inode(file_inode).expect("Inode lookup failed");
    assert_eq!(inode_ref.link_count, 1);
    assert_eq!(inode_ref.hard_links_count, 1);

    // Create hard link
    vfs.create_hard_link(file_inode, root, "test_file_link.txt")
        .expect("Hard link creation failed");
    let inode_ref2 = vfs.get_inode(file_inode).expect("Inode lookup failed");
    assert_eq!(inode_ref2.link_count, 2);
    assert_eq!(inode_ref2.hard_links_count, 2);

    // Unlink first reference
    vfs.delete_file(root, "test_file.txt")
        .expect("Delete file failed");
    let inode_ref3 = vfs
        .get_inode(file_inode)
        .expect("Inode must persist after unlinking one hard link");
    assert_eq!(inode_ref3.link_count, 1);
    assert_eq!(inode_ref3.hard_links_count, 1);

    // Unlink final reference
    vfs.delete_file(root, "test_file_link.txt")
        .expect("Delete hard link failed");
    assert!(
        vfs.get_inode(file_inode).is_none(),
        "Inode must be freed when hard link count reaches zero"
    );
}

#[test]
fn test_network_ring_buffer_and_fingerprinting_inspection() {
    let mut ring = PacketRingBuffer::new(4);
    let hdr = PacketHeader {
        src_ip: [192, 168, 1, 100],
        dst_ip: [192, 168, 1, 1],
        src_port: 8080,
        dst_port: 80,
        protocol: ProtocolType::Tcp,
        payload_len: 128,
        ttl: 64,
        window_size: 65535,
    };

    assert!(ring.push_packet(hdr.clone(), &[0x00; 128]).is_ok());
    assert_eq!(ring.count(), 1);

    let os = PacketRingBuffer::passive_os_fingerprint(&hdr);
    assert_eq!(os, OperatingSystemType::LinuxKernel);
}

#[test]
fn test_kernel_hardening_kaslr_smep_and_syscalls_inspection() {
    // 1. KASLR Virtual Address Slide Inspection
    let mut kaslr = SovereignKaslrEngine::new(0xFFFF800000000000);
    let slide = kaslr.calculate_randomized_slide(0x123456789ABCDEF0);
    assert!(
        slide > 0,
        "KASLR slide must be non-zero when entropy is applied"
    );
    assert_eq!(kaslr.get_kernel_text_base(), 0xFFFF800000000000 + slide);

    // 2. SMEP / SMAP Execution Control Inspection
    let mut smep_smap = SmepSmapEnforcer::new();
    smep_smap.enable_smep();
    smep_smap.enable_smap();

    assert!(smep_smap.is_smep_active());
    assert!(smep_smap.is_smap_active());

    // User-space execution trap inspection
    assert!(
        smep_smap
            .verify_instruction_fetch(0x00007FFF00001000, true)
            .is_err(),
        "SMEP must prevent kernel from executing user-space code"
    );

    // 3. Hardened Syscall Dispatcher Filtering
    let mut dispatcher = HardenedSyscallDispatcher::new();
    dispatcher.add_filter_rule(1, SyscallFilterRule::Allow);
    dispatcher.add_filter_rule(59, SyscallFilterRule::DenyWithEperm); // sys_execve

    assert_eq!(dispatcher.dispatch_syscall(1, &[0, 0, 0]), Ok(0));
    assert!(
        dispatcher.dispatch_syscall(59, &[0, 0, 0]).is_err(),
        "Dispatcher must block denied syscalls"
    );
}
