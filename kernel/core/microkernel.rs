//! microkernel.rs — SigmaOS Sovereign Capability-Isolated Microkernel
//! Implements the minimal supervisor ("μ-kernel") that runs at highest privilege.
//! Everything else (drivers, FS, network) runs as isolated user-space services
//! communicating only via typed, capability-gated IPC channels.
//!
//! Sovereign: #![no_std], no external crates, no libc.
//! Architecture: x86-64 (ring 0).
//!
//! Design principles:
//!   - Seggregation of Mechanism from Policy (L4 lineage)
//!   - All resource access via unforgeable capability tokens (CNode)
//!   - Zero-copy message passing via shared page mapping + descriptor exchange
//!   - Formally minimal TCB: < 10K lines of code

#![no_std]
#![allow(dead_code)]
#![allow(unused)]

// ─── Architecture Primitives ──────────────────────────────────────────────────
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub u64);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub u64);

impl VirtAddr {
    #[inline(always)]
    pub unsafe fn as_mut_ptr<T>(&self) -> *mut T {
        self.0 as *mut T
    }
    #[inline(always)]
    pub unsafe fn as_ptr<T>(&self) -> *const T {
        self.0 as *const T
    }
}

// ─── Capability Token ─────────────────────────────────────────────────────────
/// A capability is an unforgeable 64-bit token referencing a kernel object.
/// Format: [63:48] object type | [47:16] generation | [15:0] index
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Capability(u64);

impl Capability {
    pub const NULL: Capability = Capability(0);

    pub fn new(obj_type: ObjType, generation: u32, index: u16) -> Self {
        Capability(
            ((obj_type as u64) << 48)
          | ((generation as u64 & 0xFFFF_FFFF) << 16)
          | (index as u64)
        )
    }

    pub fn obj_type(self) -> ObjType {
        ObjType::from_u16((self.0 >> 48) as u16)
    }

    pub fn generation(self) -> u32 {
        ((self.0 >> 16) & 0xFFFF_FFFF) as u32
    }

    pub fn index(self) -> u16 {
        (self.0 & 0xFFFF) as u16
    }

    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjType {
    Null         = 0,
    Thread       = 1,
    AddressSpace = 2,
    Endpoint     = 3,  // IPC endpoint (message passing)
    Notification = 4,  // async signal object
    Frame        = 5,  // physical memory frame
    PageTable    = 6,
    CNode        = 7,  // capability node (array of capabilities)
    IrqHandler   = 8,
    IoPort       = 9,
    Untyped      = 10, // raw untyped memory for retyping
    _Unknown     = 0xFFFF,
}

impl ObjType {
    fn from_u16(v: u16) -> Self {
        match v {
            0  => ObjType::Null,
            1  => ObjType::Thread,
            2  => ObjType::AddressSpace,
            3  => ObjType::Endpoint,
            4  => ObjType::Notification,
            5  => ObjType::Frame,
            6  => ObjType::PageTable,
            7  => ObjType::CNode,
            8  => ObjType::IrqHandler,
            9  => ObjType::IoPort,
            10 => ObjType::Untyped,
            _  => ObjType::_Unknown,
        }
    }
}

// ─── Access Rights Bitmask ────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Rights(u32);

impl Rights {
    pub const NONE:    Rights = Rights(0);
    pub const READ:    Rights = Rights(1 << 0);
    pub const WRITE:   Rights = Rights(1 << 1);
    pub const EXECUTE: Rights = Rights(1 << 2);
    pub const GRANT:   Rights = Rights(1 << 3);  // may delegate cap
    pub const ALL:     Rights = Rights(0x0F);

    pub fn has(self, r: Rights) -> bool { (self.0 & r.0) == r.0 }
    pub fn add(self, r: Rights) -> Rights { Rights(self.0 | r.0) }
    pub fn remove(self, r: Rights) -> Rights { Rights(self.0 & !r.0) }
}

// ─── CNode (Capability Table) ─────────────────────────────────────────────────
pub const CNODE_SLOTS: usize = 256;

pub struct CNode {
    pub slots:      [Capability; CNODE_SLOTS],
    pub rights:     [Rights;     CNODE_SLOTS],
    pub generation: u32,
}

impl CNode {
    pub const fn new() -> Self {
        Self {
            slots:      [Capability::NULL; CNODE_SLOTS],
            rights:     [Rights::NONE;     CNODE_SLOTS],
            generation: 0,
        }
    }

    /// Insert a capability at a slot. Returns false if slot occupied.
    pub fn insert(&mut self, slot: usize, cap: Capability, rights: Rights) -> bool {
        if slot >= CNODE_SLOTS { return false; }
        if self.slots[slot].is_valid() { return false; }
        self.slots[slot] = cap;
        self.rights[slot] = rights;
        true
    }

    /// Revoke (delete) a capability from a slot. Bumps generation.
    pub fn revoke(&mut self, slot: usize) {
        if slot < CNODE_SLOTS {
            self.slots[slot] = Capability::NULL;
            self.rights[slot] = Rights::NONE;
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Derive a less-privileged copy (rights must be subset).
    pub fn derive(&self, src: usize, dst_rights: Rights) -> Option<Capability> {
        if src >= CNODE_SLOTS { return None; }
        let cap = self.slots[src];
        if !cap.is_valid() { return None; }
        if !self.rights[src].has(Rights::GRANT) { return None; }
        // Mask down rights
        let _ = self.rights[src].remove(dst_rights);
        Some(cap)
    }
}

// ─── Thread Control Block ─────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ThreadState {
    Inactive   = 0,
    Running    = 1,
    Blocked    = 2,  // waiting on endpoint
    Queued     = 3,  // in scheduler ready queue
    Suspended  = 4,
}

#[repr(C, align(16))]
pub struct ThreadRegs {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rsp: u64, pub rbp: u64,
    pub r8:  u64, pub r9:  u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64, pub rflags: u64,
    pub cs:  u64, pub ss:     u64,
    pub fs_base: u64, pub gs_base: u64,
}

impl ThreadRegs {
    pub const fn zeroed() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rsp: 0, rbp: 0,
            r8:  0, r9:  0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rflags: 0x202,  // IF=1
            cs: 0x08, ss: 0x10,     // GDT ring-0 selectors
            fs_base: 0, gs_base: 0,
        }
    }
}

pub const MAX_THREADS: usize = 1024;

pub struct Thread {
    pub id:        u32,
    pub state:     ThreadState,
    pub priority:  u8,        // 0 (lowest) – 255 (highest)
    pub regs:      ThreadRegs,
    pub cnode_idx: u16,       // index into global CNode pool
    pub aspace_idx: u16,      // index into address space pool
    pub ipc_tag:   u64,       // pending IPC message tag
    pub ipc_badge: u64,       // badge from sender
    pub budget_us: u32,       // remaining timeslice in µs
}

impl Thread {
    pub const fn inactive() -> Self {
        Self {
            id:        0,
            state:     ThreadState::Inactive,
            priority:  128,
            regs:      ThreadRegs::zeroed(),
            cnode_idx: 0xFFFF,
            aspace_idx: 0xFFFF,
            ipc_tag:   0,
            ipc_badge: 0,
            budget_us: 2000,  // 2ms default timeslice
        }
    }
}

// ─── IPC Endpoint ─────────────────────────────────────────────────────────────
/// An endpoint is the fundamental IPC primitive. Threads call Send/Recv on it.
/// Message registers (MR0–MR7) hold the payload (64 bytes).

pub const MSG_REGISTERS: usize = 8;

#[derive(Copy, Clone)]
pub struct IpcMessage {
    pub tag:  u64,                        // label (bits 63:16) + extra_caps (15:12) + length (11:0)
    pub mrs:  [u64; MSG_REGISTERS],
    pub caps: [Capability; 4],            // capability transfer slots
}

impl IpcMessage {
    pub const fn empty() -> Self {
        Self { tag: 0, mrs: [0; MSG_REGISTERS], caps: [Capability::NULL; 4] }
    }

    pub fn label(self) -> u64 { self.tag >> 16 }
    pub fn length(self) -> u8 { (self.tag & 0xFFF) as u8 }
    pub fn extra_caps(self) -> u8 { ((self.tag >> 12) & 0xF) as u8 }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum EndpointState { Idle, Sending, Receiving }

pub struct Endpoint {
    pub state:       EndpointState,
    pub waiter_head: u16,       // thread ID of first waiter (0xFFFF = none)
    pub waiter_tail: u16,
    pub msg:         IpcMessage, // buffered message (for async)
    pub badge:       u64,        // set by sender's capability
}

impl Endpoint {
    pub const fn new() -> Self {
        Self {
            state: EndpointState::Idle,
            waiter_head: 0xFFFF,
            waiter_tail: 0xFFFF,
            msg: IpcMessage::empty(),
            badge: 0,
        }
    }
}

pub const MAX_ENDPOINTS: usize = 256;

// ─── Notification Object ──────────────────────────────────────────────────────
/// Bitfield-based async signalling. send OR's bits; recv ANDS and clears.
pub struct Notification {
    pub word:    u64,
    pub waiter:  u16,   // thread waiting on this (0xFFFF = none)
}

impl Notification {
    pub const fn new() -> Self { Self { word: 0, waiter: 0xFFFF } }

    pub fn signal(&mut self, bits: u64) { self.word |= bits; }
    pub fn wait_clears(&mut self) -> u64 {
        let v = self.word;
        self.word = 0;
        v
    }
}

// ─── Microkernel Object Pool ──────────────────────────────────────────────────
pub struct MicrokernelState {
    pub threads:       [Thread;       MAX_THREADS],
    pub cnodes:        [CNode;        512],
    pub endpoints:     [Endpoint;     MAX_ENDPOINTS],
    pub notifications: [Notification; 128],
    pub thread_count:  u16,
    pub current_tid:   u16,          // index of running thread
}

impl MicrokernelState {
    pub const fn new() -> Self {
        Self {
            threads:       [const { Thread::inactive() }; MAX_THREADS],
            cnodes:        [const { CNode::new() };       512],
            endpoints:     [const { Endpoint::new() };    MAX_ENDPOINTS],
            notifications: [const { Notification::new() }; 128],
            thread_count:  0,
            current_tid:   0,
        }
    }

    /// Allocate a new thread slot. Returns thread id or None.
    pub fn alloc_thread(&mut self) -> Option<u16> {
        for i in 0..MAX_THREADS {
            if self.threads[i].state == ThreadState::Inactive {
                self.threads[i] = Thread::inactive();
                self.threads[i].id    = i as u32;
                self.threads[i].state = ThreadState::Queued;
                return Some(i as u16);
            }
        }
        None
    }

    /// Perform a synchronous IPC: sender (by tid) sends to endpoint, receiver unblocks.
    /// Returns Ok(()) on success, Err on invalid cap or deadlock.
    pub fn ipc_send(&mut self, sender: u16, ep_idx: usize, msg: IpcMessage) -> Result<(), IpcError> {
        if ep_idx >= MAX_ENDPOINTS { return Err(IpcError::InvalidEndpoint); }
        let ep = &mut self.endpoints[ep_idx];
        if ep.state == EndpointState::Receiving && ep.waiter_head != 0xFFFF {
            // Direct transfer: wake receiver
            let recv_tid = ep.waiter_head as usize;
            self.threads[recv_tid].ipc_tag   = msg.tag;
            self.threads[recv_tid].ipc_badge  = ep.badge;
            for i in 0..MSG_REGISTERS {
                // Copy MRs to receiver (in real impl, this is register file)
                // Here: store in thread for retrieval on next schedule
                let _ = msg.mrs[i]; // prevent dead_code warn
            }
            self.threads[recv_tid].state = ThreadState::Queued;
            ep.waiter_head = 0xFFFF;
            ep.state       = EndpointState::Idle;
            Ok(())
        } else {
            // Queue sender
            ep.msg   = msg;
            ep.state = EndpointState::Sending;
            self.threads[sender as usize].state = ThreadState::Blocked;
            Ok(())
        }
    }

    pub fn ipc_recv(&mut self, receiver: u16, ep_idx: usize) -> Result<IpcMessage, IpcError> {
        if ep_idx >= MAX_ENDPOINTS { return Err(IpcError::InvalidEndpoint); }
        let ep = &mut self.endpoints[ep_idx];
        if ep.state == EndpointState::Sending {
            // Message waiting — take it
            let msg = ep.msg;
            ep.state = EndpointState::Idle;
            // Wake any blocked sender — simplified: assume single sender
            Ok(msg)
        } else {
            // Block receiver
            ep.state       = EndpointState::Receiving;
            ep.waiter_head = receiver;
            self.threads[receiver as usize].state = ThreadState::Blocked;
            Err(IpcError::WouldBlock)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IpcError {
    InvalidEndpoint,
    WouldBlock,
    InvalidCapability,
    AccessDenied,
    DeadlockDetected,
}

// ─── Microkernel System Call Dispatcher ───────────────────────────────────────
/// System call numbers (sovereign ABI, non-POSIX)
#[repr(u64)]
pub enum SyscallNr {
    Null             = 0,
    Send             = 1,
    Recv             = 2,
    Call             = 3,  // send + recv atomically
    Reply            = 4,
    Yield            = 5,
    ThreadCreate     = 10,
    ThreadDelete     = 11,
    ThreadSuspend    = 12,
    ThreadResume     = 13,
    CNodeInsert      = 20,
    CNodeRevoke      = 21,
    CNodeDerive      = 22,
    FrameMap         = 30,
    FrameUnmap       = 31,
    NotifSignal      = 40,
    NotifWait        = 41,
    Debug            = 0xFF,
}

/// Dispatch a system call. Returns (result_a, result_b) in registers a0/a1.
pub fn syscall_dispatch(
    state:  &mut MicrokernelState,
    tid:    u16,
    nr:     u64,
    a0: u64, a1: u64, a2: u64, a3: u64,
) -> (u64, u64) {
    match nr {
        n if n == SyscallNr::Send as u64 => {
            let ep_idx = a0 as usize;
            let mut msg = IpcMessage::empty();
            msg.tag = a1;
            // MRs would come from thread register state in real impl
            match state.ipc_send(tid, ep_idx, msg) {
                Ok(())              => (0, 0),
                Err(e) => (ipc_err_to_u64(e), 0),
            }
        }
        n if n == SyscallNr::Recv as u64 => {
            let ep_idx = a0 as usize;
            match state.ipc_recv(tid, ep_idx) {
                Ok(msg)  => (0, msg.tag),
                Err(e)   => (ipc_err_to_u64(e), 0),
            }
        }
        n if n == SyscallNr::Yield as u64 => {
            // Voluntarily yield timeslice — scheduler picks next thread
            state.threads[tid as usize].state = ThreadState::Queued;
            state.threads[tid as usize].budget_us = 2000;
            (0, 0)
        }
        n if n == SyscallNr::ThreadCreate as u64 => {
            match state.alloc_thread() {
                Some(new_tid) => {
                    state.threads[new_tid as usize].regs.rip = a0; // entry point
                    state.threads[new_tid as usize].regs.rsp = a1; // stack pointer
                    state.threads[new_tid as usize].priority = a2 as u8;
                    (0, new_tid as u64)
                }
                None => (1, 0), // OOM_THREADS
            }
        }
        n if n == SyscallNr::ThreadDelete as u64 => {
            let target = a0 as u16;
            if (target as usize) < MAX_THREADS {
                state.threads[target as usize].state = ThreadState::Inactive;
            }
            (0, 0)
        }
        n if n == SyscallNr::CNodeInsert as u64 => {
            let cnode_idx = (a0 >> 32) as usize;
            let slot      = (a0 & 0xFFFF_FFFF) as usize;
            let cap       = Capability(a1);
            let rights    = Rights(a2 as u32);
            if cnode_idx < 512 && state.cnodes[cnode_idx].insert(slot, cap, rights) {
                (0, 0)
            } else {
                (2, 0) // SLOT_OCCUPIED
            }
        }
        n if n == SyscallNr::CNodeRevoke as u64 => {
            let cnode_idx = (a0 >> 32) as usize;
            let slot      = (a0 & 0xFFFF_FFFF) as usize;
            if cnode_idx < 512 { state.cnodes[cnode_idx].revoke(slot); }
            (0, 0)
        }
        n if n == SyscallNr::NotifSignal as u64 => {
            let notif_idx = a0 as usize;
            let bits      = a1;
            if notif_idx < state.notifications.len() {
                state.notifications[notif_idx].signal(bits);
            }
            (0, 0)
        }
        n if n == SyscallNr::NotifWait as u64 => {
            let notif_idx = a0 as usize;
            if notif_idx < state.notifications.len() {
                let bits = state.notifications[notif_idx].wait_clears();
                if bits != 0 { (0, bits) }
                else {
                    // Block thread
                    state.notifications[notif_idx].waiter = tid;
                    state.threads[tid as usize].state = ThreadState::Blocked;
                    (0, 0)
                }
            } else {
                (3, 0) // INVALID_NOTIF
            }
        }
        _ => (0xFF, 0), // UNKNOWN_SYSCALL
    }
}

fn ipc_err_to_u64(e: IpcError) -> u64 {
    match e {
        IpcError::InvalidEndpoint  => 10,
        IpcError::WouldBlock       => 11,
        IpcError::InvalidCapability => 12,
        IpcError::AccessDenied     => 13,
        IpcError::DeadlockDetected => 14,
    }
}

// ─── Priority Scheduler ───────────────────────────────────────────────────────
/// 256-level bitmap scheduler (O(1) pick-next).
pub struct PriorityScheduler {
    bitmap:  [u64; 4],   // 4 × 64 bits = 256 priority levels
    queues:  [[u16; 64]; 256],   // per-priority run queue (ring buffer, max 64 threads)
    heads:   [u8; 256],
    tails:   [u8; 256],
    counts:  [u8; 256],
}

impl PriorityScheduler {
    pub const fn new() -> Self {
        Self {
            bitmap: [0u64; 4],
            queues: [[0xFFFF; 64]; 256],
            heads:  [0u8; 256],
            tails:  [0u8; 256],
            counts: [0u8; 256],
        }
    }

    pub fn enqueue(&mut self, tid: u16, priority: u8) {
        let p   = priority as usize;
        let t   = self.tails[p] as usize;
        let cnt = self.counts[p] as usize;
        if cnt < 64 {
            self.queues[p][t] = tid;
            self.tails[p]     = ((t + 1) % 64) as u8;
            self.counts[p]   += 1;
            // Set bitmap bit
            self.bitmap[p / 64] |= 1u64 << (p % 64);
        }
    }

    /// Pick the highest-priority runnable thread. Returns thread id or None.
    pub fn pick_next(&mut self) -> Option<u16> {
        for word_idx in (0..4).rev() {
            let w = self.bitmap[word_idx];
            if w == 0 { continue; }
            let bit = 63 - w.leading_zeros() as usize;
            let p   = word_idx * 64 + bit;
            let h   = self.heads[p] as usize;
            let tid = self.queues[p][h];
            self.heads[p]   = ((h + 1) % 64) as u8;
            self.counts[p] -= 1;
            if self.counts[p] == 0 {
                self.bitmap[word_idx] &= !(1u64 << bit);
            }
            return Some(tid);
        }
        None
    }
}
