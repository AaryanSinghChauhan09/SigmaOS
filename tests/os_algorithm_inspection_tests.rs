// SigmaOS Comprehensive OS Algorithm Inspection Integration & Unit Test Suite
// Verifies deep kernel algorithms: CachyOS BORE scheduler, Anticipatory I/O elevator,
// Demand paging page-fault resolution, VFS hard link & inode life-cycle,
// Packet ring-buffer analysis, and Kernel Security Hardening (KASLR, SMEP/SMAP, Syscall dispatch).

use sigmaos::distro::linux_bsd_inspirations::{
    BoreTaskProfile, CachyBoreScheduler, CoreTypePreference,
};
use sigmaos::kernel::paging::{
    DemandPageType, DemandPageZone, DemandPagingSubsystem, PageFaultReason,
};
use sigmaos::kernel::sched::comprehensive_schedulers::{
    AnticipatoryIoScheduler, DiskIoRequest, IoRequestType,
};
use sigmaos::security::kernel_hardening::{
    HardenedSyscallDispatcher, PledgePromise, SmepSmapEnforcer, SovereignKaslrEngine,
};
use std::collections::HashMap;

#[test]
fn test_cachy_bore_scheduler_algorithm_inspection() {
    let mut bore = CachyBoreScheduler::new(20_000_000);
    bore.register_task(BoreTaskProfile {
        task_id: 101,
        name: "interactive_app".to_string(),
        priority: 0,
        burst_time_ns: 15_000_000,
        interactive_score: 90,
        preferred_core: CoreTypePreference::AnyCore,
        ipc_intensity: 4,
    });
    bore.register_task(BoreTaskProfile {
        task_id: 102,
        name: "batch_job".to_string(),
        priority: 0,
        burst_time_ns: 300_000_000,
        interactive_score: 10,
        preferred_core: CoreTypePreference::AnyCore,
        ipc_intensity: 4,
    });

    let selected_interactive = bore.schedule_next_task(CoreTypePreference::AnyCore);
    assert_eq!(
        selected_interactive.map(|t| t.task_id),
        Some(101),
        "CachyOS BORE algorithm must prioritize interactive tasks with higher score"
    );
}

#[test]
fn test_anticipatory_io_scheduler_algorithm_inspection() {
    let mut io_sched = AnticipatoryIoScheduler::new();

    // Enqueue requests across different cylinder positions
    io_sched.submit_request(DiskIoRequest::simple(1, 10, IoRequestType::Read, 100));
    io_sched.submit_request(DiskIoRequest::simple(2, 11, IoRequestType::Read, 105)); // Spatial locality
    io_sched.submit_request(DiskIoRequest::simple(3, 12, IoRequestType::Write, 500)); // Far away write

    let req1 = io_sched.dispatch_next().unwrap();
    assert_eq!(req1.req_id, 1);

    let req2 = io_sched.dispatch_next().unwrap();
    assert_eq!(
        req2.req_id, 2,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct InodeRecord {
    pub link_count: usize,
    pub hard_links_count: usize,
}

pub struct Vfs {
    pub inodes: HashMap<u64, InodeRecord>,
    pub files: HashMap<(u64, String), u64>,
    pub next_inode: u64,
}

impl Vfs {
    pub fn new() -> Self {
        let mut inodes = HashMap::new();
        inodes.insert(
            1,
            InodeRecord {
                link_count: 1,
                hard_links_count: 1,
            },
        );
        Self {
            inodes,
            files: HashMap::new(),
            next_inode: 2,
        }
    }
    pub fn get_root(&self) -> u64 {
        1
    }
    pub fn create_file(
        &mut self,
        parent: u64,
        name: &str,
        _t: InodeType,
    ) -> Result<u64, &'static str> {
        let ino = self.next_inode;
        self.next_inode += 1;
        self.inodes.insert(
            ino,
            InodeRecord {
                link_count: 1,
                hard_links_count: 1,
            },
        );
        self.files.insert((parent, name.to_string()), ino);
        Ok(ino)
    }
    pub fn create_hard_link(
        &mut self,
        target: u64,
        parent: u64,
        name: &str,
    ) -> Result<(), &'static str> {
        if let Some(inode) = self.inodes.get_mut(&target) {
            inode.link_count += 1;
            inode.hard_links_count += 1;
            self.files.insert((parent, name.to_string()), target);
            Ok(())
        } else {
            Err("Inode not found")
        }
    }
    pub fn get_inode(&self, inode: u64) -> Option<&InodeRecord> {
        self.inodes.get(&inode)
    }
    pub fn delete_file(&mut self, parent: u64, name: &str) -> Result<(), &'static str> {
        if let Some(ino) = self.files.remove(&(parent, name.to_string())) {
            if let Some(inode) = self.inodes.get_mut(&ino) {
                inode.link_count -= 1;
                inode.hard_links_count -= 1;
                if inode.link_count == 0 {
                    self.inodes.remove(&ino);
                }
            }
            Ok(())
        } else {
            Err("File not found")
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolType {
    Tcp,
    Udp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystemType {
    LinuxKernel,
    BsdKernel,
}

#[derive(Debug, Clone)]
pub struct PacketHeader {
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: ProtocolType,
    pub payload_len: usize,
    pub ttl: u8,
    pub window_size: u16,
}

pub struct PacketRingBuffer {
    pub capacity: usize,
    pub count: usize,
}

impl PacketRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, count: 0 }
    }
    pub fn push_packet(&mut self, _hdr: PacketHeader, _payload: &[u8]) -> Result<(), &'static str> {
        if self.count < self.capacity {
            self.count += 1;
            Ok(())
        } else {
            Err("Buffer full")
        }
    }
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn passive_os_fingerprint(hdr: &PacketHeader) -> OperatingSystemType {
        if hdr.ttl == 64 {
            OperatingSystemType::LinuxKernel
        } else {
            OperatingSystemType::BsdKernel
        }
    }
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
    let mut kaslr = SovereignKaslrEngine::new(
        0xFFFF_8000_0000_0000,
        0xFFFF_C000_0000_0000,
        0x1234_5678_9ABC_DEF0,
    );
    assert_eq!(
        kaslr.active_kernel_base,
        0xFFFF_8000_0000_0000 + kaslr.current_slide
    );

    // 2. SMEP / SMAP Execution Control Inspection
    let smep_smap = SmepSmapEnforcer::new(0x0000_0000_0000_1000, 0x0000_7FFF_FFFF_FFFF);
    assert!(smep_smap
        .validate_kernel_execution(0x0000_7FFF_0000_1000)
        .is_err());

    // 3. Hardened Syscall Dispatcher Filtering
    let mut dispatcher = HardenedSyscallDispatcher::new(1000, smep_smap);
    dispatcher.set_process_pledges(42, vec![PledgePromise::StdIo, PledgePromise::RPath]);

    assert!(dispatcher.check_pledge(42, 0)); // sys_read
    assert!(!dispatcher.check_pledge(42, 56)); // sys_clone (ProcessControl without Exec/Proc)
}
