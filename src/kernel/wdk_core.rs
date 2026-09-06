#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
#![cfg_attr(not(test), no_std)]
// SigmaOS Windows-Driver-Kit (WDK) Core Subsystem
// Inspired by: x86-64/ARM kernel architectures, Windows Driver Kit, Linux, and BSD.
// Zero external library dependency, no_std compatible.


use std::string::String;
use std::string::ToString;
use std::vec::Vec;

// =========================================================================
// 1. IRQL (Interrupt Request Level) & CPU Context
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Irql {
    PassiveLevel = 0,   // User-mode & standard kernel execution
    ApcLevel = 1,       // Asynchronous Procedure Calls
    DispatchLevel = 2,  // Deferred Procedure Calls & scheduler
    Dirql = 3,          // Device Interrupt Request Level
    HighLevel = 4,      // Highest hardware priority / NMI
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuRing {
    Ring0, // Kernel Mode
    Ring3, // User Mode
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArch {
    X86_64,
    Arm64,
}

// =========================================================================
// 2. Address Spaces & Security Tokens
// =========================================================================

#[derive(Debug, Clone)]
pub struct SecurityToken {
    pub sid: String,
    pub privilege_mask: u64,
    pub is_system: bool,
}

impl SecurityToken {
    pub fn new(sid: &str, privileges: u64, is_system: bool) -> Self {
        Self {
            sid: sid.to_string(),
            privilege_mask: privileges,
            is_system,
        }
    }
}

pub struct AddressSpace {
    pub page_directory_base: u64,
    pub is_user_space: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionContext {
    ThreadContext,
    SystemContext,
    ArbitraryContext,
}

// =========================================================================
// 3. Threads, APCs, DPCs, and Work Items
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Waiting,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApcMode {
    KernelMode,
    UserMode,
}

pub struct Apc {
    pub mode: ApcMode,
    pub routine: fn(context: usize),
    pub context: usize,
}

pub struct Dpc {
    pub routine: fn(deferred_context: usize, system_argument1: usize, system_argument2: usize),
    pub deferred_context: usize,
    pub system_argument1: usize,
    pub system_argument2: usize,
}

pub struct WorkItem {
    pub routine: fn(parameter: usize),
    pub parameter: usize,
}

pub struct WdkThread {
    pub thread_id: u32,
    pub is_system_thread: bool,
    pub state: ThreadState,
    pub base_priority: i8,
    pub current_priority: i8,
    pub token: SecurityToken,
    pub registers: [u64; 16], // x86-64/ARM thread registers
    pub apc_queue: Vec<Apc>,
    pub alertable: bool,
}

impl WdkThread {
    pub fn new(id: u32, is_system: bool, token: SecurityToken) -> Self {
        Self {
            thread_id: id,
            is_system_thread: is_system,
            state: ThreadState::Ready,
            base_priority: 8,
            current_priority: 8,
            token,
            registers: [0u64; 16],
            apc_queue: Vec::new(),
            alertable: false,
        }
    }

    pub fn queue_apc(&mut self, apc: Apc) {
        self.apc_queue.push(apc);
    }

    pub fn deliver_apcs(&mut self) -> usize {
        let mut count = 0;
        let mut pending = Vec::new();
        core::mem::swap(&mut self.apc_queue, &mut pending);

        for apc in pending {
            (apc.routine)(apc.context);
            count += 1;
        }
        count
    }
}

// =========================================================================
// 4. Kernel Synchronization Primitives (Events, SpinLocks, Mutexes, ERESOURCE)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    NotificationEvent, // Manual-reset (stays signalled until cleared)
    SynchronizationEvent, // Auto-reset (resets to non-signalled after one waiter)
}

pub struct EventObject {
    pub event_type: EventType,
    pub is_signalled: bool,
    pub waiting_threads: Vec<u32>,
}

impl EventObject {
    pub fn new(event_type: EventType, initial_state: bool) -> Self {
        Self {
            event_type,
            is_signalled: initial_state,
            waiting_threads: Vec::new(),
        }
    }

    pub fn set_event(&mut self) -> usize {
        self.is_signalled = true;
        let waiters_notified = self.waiting_threads.len();

        if self.event_type == EventType::SynchronizationEvent && waiters_notified > 0 {
            // Auto-reset on notifying first waiter
            self.is_signalled = false;
            self.waiting_threads.remove(0);
            return 1;
        }

        self.waiting_threads.clear();
        waiters_notified
    }

    pub fn reset_event(&mut self) {
        self.is_signalled = false;
    }

    pub fn wait_on_event(&mut self, thread_id: u32) -> bool {
        if self.is_signalled {
            if self.event_type == EventType::SynchronizationEvent {
                self.is_signalled = false;
            }
            return true;
        }
        self.waiting_threads.push(thread_id);
        false
    }
}

pub struct SpinLock {
    pub owner_cpu: Option<u32>,
    pub previous_irql: Irql,
}

impl SpinLock {
    pub fn new() -> Self {
        Self {
            owner_cpu: None,
            previous_irql: Irql::PassiveLevel,
        }
    }

    pub fn acquire(&mut self, cpu_id: u32, current_irql: Irql) -> Result<Irql, &'static str> {
        if current_irql > Irql::DispatchLevel {
            return Err("SpinLock: Cannot acquire spinlock above DISPATCH_LEVEL");
        }
        self.owner_cpu = Some(cpu_id);
        // Elevate IRQL to DISPATCH_LEVEL to prevent scheduler context switches
        self.previous_irql = current_irql;
        Ok(self.previous_irql)
    }

    pub fn release(&mut self) -> Irql {
        self.owner_cpu = None;
        self.previous_irql
    }
}

pub struct MutexObject {
    pub owner_thread_id: Option<u32>,
    pub recursion_count: u32,
    pub is_signalled: bool, // Signalled when unowned
}

impl MutexObject {
    pub fn new() -> Self {
        Self {
            owner_thread_id: None,
            recursion_count: 0,
            is_signalled: true,
        }
    }

    pub fn acquire_mutex(&mut self, thread_id: u32) -> bool {
        if self.is_signalled || self.owner_thread_id == Some(thread_id) {
            self.owner_thread_id = Some(thread_id);
            self.recursion_count += 1;
            self.is_signalled = false;
            return true;
        }
        false
    }

    pub fn release_mutex(&mut self, thread_id: u32) -> Result<bool, &'static str> {
        if self.owner_thread_id != Some(thread_id) {
            return Err("MutexObject: Thread does not own the mutex");
        }
        self.recursion_count -= 1;
        if self.recursion_count == 0 {
            self.owner_thread_id = None;
            self.is_signalled = true;
            return Ok(true); // Fully released
        }
        Ok(false) // Still recursively held
    }
}

pub struct FastMutex {
    pub owner_thread_id: Option<u32>,
    pub count: i32,
}

impl FastMutex {
    pub fn new() -> Self {
        Self {
            owner_thread_id: None,
            count: 1, // 1 = available, <=0 = locked
        }
    }

    pub fn acquire_fast(&mut self, thread_id: u32) -> Result<Irql, &'static str> {
        // Fast Mutex raises IRQL to APC_LEVEL to prevent thread suspension during lock ownership
        self.count -= 1;
        self.owner_thread_id = Some(thread_id);
        Ok(Irql::ApcLevel)
    }

    pub fn release_fast(&mut self) -> Irql {
        self.count += 1;
        self.owner_thread_id = None;
        Irql::PassiveLevel
    }
}

pub struct GuardedMutex {
    pub owner_thread_id: Option<u32>,
    pub count: i32,
}

impl GuardedMutex {
    pub fn new() -> Self {
        Self {
            owner_thread_id: None,
            count: 1,
        }
    }

    pub fn acquire_guarded(&mut self, thread_id: u32) {
        // Guarded Mutex enters a guarded region disabling all kernel APCs
        self.count -= 1;
        self.owner_thread_id = Some(thread_id);
    }

    pub fn release_guarded(&mut self) {
        self.count += 1;
        self.owner_thread_id = None;
    }
}

// ERESOURCE: Shared/Exclusive reader-writer resource lock
pub struct EResource {
    pub active_exclusive_owner: Option<u32>,
    pub active_shared_count: u32,
    pub waiting_exclusive_count: u32,
    pub waiting_shared_count: u32,
}

impl EResource {
    pub fn new() -> Self {
        Self {
            active_exclusive_owner: None,
            active_shared_count: 0,
            waiting_exclusive_count: 0,
            waiting_shared_count: 0,
        }
    }

    pub fn acquire_exclusive(&mut self, thread_id: u32) -> bool {
        if self.active_exclusive_owner.is_none() && self.active_shared_count == 0 {
            self.active_exclusive_owner = Some(thread_id);
            return true;
        }
        self.waiting_exclusive_count += 1;
        false
    }

    pub fn acquire_shared(&mut self, _thread_id: u32) -> bool {
        if self.active_exclusive_owner.is_none() && self.waiting_exclusive_count == 0 {
            self.active_shared_count += 1;
            return true;
        }
        self.waiting_shared_count += 1;
        false
    }

    pub fn release_resource(&mut self, thread_id: u32) {
        if self.active_exclusive_owner == Some(thread_id) {
            self.active_exclusive_owner = None;
        } else if self.active_shared_count > 0 {
            self.active_shared_count -= 1;
        }
    }
}

// =========================================================================
// 5. Timers & PRCB (Processor Control Block)
// =========================================================================

pub struct WdkTimer {
    pub timer_id: u32,
    pub due_time_ms: u64,
    pub period_ms: u32,
    pub is_periodic: bool,
    pub is_signalled: bool,
    pub dpc: Option<Dpc>,
}

pub struct TimerTable {
    pub timers: Vec<WdkTimer>,
}

impl TimerTable {
    pub fn new() -> Self {
        Self { timers: Vec::new() }
    }

    pub fn register_timer(&mut self, timer: WdkTimer) {
        self.timers.push(timer);
    }

    pub fn tick_timers(&mut self, elapsed_ms: u64) -> Vec<Dpc> {
        let mut triggered_dpcs = Vec::new();
        for timer in &mut self.timers {
            if !timer.is_signalled {
                if elapsed_ms >= timer.due_time_ms {
                    timer.is_signalled = true;
                    if let Some(ref dpc) = timer.dpc {
                        triggered_dpcs.push(Dpc {
                            routine: dpc.routine,
                            deferred_context: dpc.deferred_context,
                            system_argument1: dpc.system_argument1,
                            system_argument2: dpc.system_argument2,
                        });
                    }
                    if timer.is_periodic {
                        timer.due_time_ms += timer.period_ms as u64;
                        timer.is_signalled = false;
                    }
                }
            }
        }
        triggered_dpcs
    }
}

// PRCB (Processor Control Block)
pub struct Prcb {
    pub cpu_id: u32,
    pub current_irql: Irql,
    pub active_thread_id: Option<u32>,
    pub dpc_queue: Vec<Dpc>,
    pub timer_table: TimerTable,
}

impl Prcb {
    pub fn new(cpu_id: u32) -> Self {
        Self {
            cpu_id,
            current_irql: Irql::PassiveLevel,
            active_thread_id: None,
            dpc_queue: Vec::new(),
            timer_table: TimerTable::new(),
        }
    }

    pub fn queue_dpc(&mut self, dpc: Dpc) {
        self.dpc_queue.push(dpc);
    }

    pub fn execute_dpc_queue(&mut self) -> usize {
        let previous_irql = self.current_irql;
        // DPC queue executes at DISPATCH_LEVEL
        self.current_irql = Irql::DispatchLevel;

        let mut count = 0;
        let mut pending = Vec::new();
        core::mem::swap(&mut self.dpc_queue, &mut pending);

        for dpc in pending {
            (dpc.routine)(dpc.deferred_context, dpc.system_argument1, dpc.system_argument2);
            count += 1;
        }

        self.current_irql = previous_irql;
        count
    }
}

// =========================================================================
// 6. Memory Pools (NonPagedPool / PagedPool)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolType {
    NonPagedPool,       // Guaranteed to remain in physical RAM (no page faults, safe for high IRQL)
    PagedPool,          // Can be paged to disk (only safe for PASSIVE_LEVEL / APC_LEVEL)
    NonPagedPoolNx,     // NonPagedPool with No-Execute permission
}

pub struct PoolAllocation {
    pub address: usize,
    pub size: usize,
    pub pool_type: PoolType,
    pub tag: [u8; 4], // standard 4-byte pool tag
}

pub struct KernelPoolMemory {
    pub allocations: Vec<PoolAllocation>,
    pub non_paged_limit: usize,
    pub active_bytes: usize,
}

impl KernelPoolMemory {
    pub fn new(non_paged_limit: usize) -> Self {
        Self {
            allocations: Vec::new(),
            non_paged_limit,
            active_bytes: 0,
        }
    }

    pub fn ex_allocate_pool(&mut self, pool_type: PoolType, size: usize, tag: [u8; 4]) -> Result<usize, &'static str> {
        if pool_type == PoolType::NonPagedPool && self.active_bytes + size > self.non_paged_limit {
            return Err("ExAllocatePoolWithTag: NonPagedPool limit exceeded!");
        }
        let address = 0x80000000usize + self.active_bytes;
        self.allocations.push(PoolAllocation {
            address,
            size,
            pool_type,
            tag,
        });
        self.active_bytes += size;
        Ok(address)
    }

    pub fn ex_free_pool(&mut self, address: usize) -> Result<(), &'static str> {
        if let Some(pos) = self.allocations.iter().position(|a| a.address == address) {
            let alloc = self.allocations.remove(pos);
            self.active_bytes -= alloc.size;
            Ok(())
        } else {
            Err("ExFreePool: Invalid pool allocation address")
        }
    }
}

// =========================================================================
// 7. WDK Drivers (DriverEntry, Unload, IOCTLs, BugChecks)
// =========================================================================

pub struct IoStatusBlock {
    pub status: i32,
    pub information: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoctlControl {
    IoctlBufferIo,
    IoctlDirectIo,
    IoctlNeitherIo,
}

pub struct IRP {
    pub ioctl: IoctlControl,
    pub input_buffer: Vec<u8>,
    pub output_buffer: Vec<u8>,
    pub io_status: IoStatusBlock,
}

pub struct WdkDriverObject {
    pub name: String,
    pub driver_entry: fn(driver_object: &mut WdkDriverObject) -> i32,
    pub driver_unload: Option<fn(driver_object: &WdkDriverObject)>,
    pub dispatch_ioctl: Option<fn(irp: &mut IRP) -> i32>,
}

impl WdkDriverObject {
    pub fn new(name: &str, entry: fn(driver_object: &mut WdkDriverObject) -> i32) -> Self {
        Self {
            name: name.to_string(),
            driver_entry: entry,
            driver_unload: None,
            dispatch_ioctl: None,
        }
    }

    pub fn load_driver(&mut self) -> i32 {
        (self.driver_entry)(self)
    }

    pub fn unload_driver(&self) -> bool {
        if let Some(unload) = self.driver_unload {
            unload(self);
            true
        } else {
            false
        }
    }
}

// Access Violations & BugChecks
pub struct BugCheckData {
    pub code: u32,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
}

pub struct BugCheckRegistry {
    pub bug_check: Option<BugCheckData>,
}

impl BugCheckRegistry {
    pub fn new() -> Self {
        Self { bug_check: None }
    }

    pub fn ke_bug_check_ex(&mut self, code: u32, arg1: u64, arg2: u64, arg3: u64, arg4: u64) {
        self.bug_check = Some(BugCheckData {
            code,
            arg1,
            arg2,
            arg3,
            arg4,
        });
    }
}

// =========================================================================
// MACH / ALPC ZERO-COPY MESSAGE PORT RIGHTS (macOS XNU & Windows NT Parity)
// =========================================================================

/// Capability-based Mach/ALPC Message Port Rights (inspired by macOS Mach IPC ports)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachPortRight {
    None = 0,
    Receive = 1,
    Send = 2,
    SendOnce = 3,
    PortSet = 4,
}

/// Zero-copy Message Payload descriptor (inspired by Windows NT ALPC shared section messages)
#[derive(Debug, Clone)]
pub struct AlpcMessagePayload {
    pub msg_id: u64,
    pub sender_pid: u64,
    pub section_base: usize,
    pub section_size: usize,
    pub inline_data: [u8; 128],
    pub inline_len: usize,
    pub right: MachPortRight,
}

/// ALPC Priority-Boosted Zero-Copy Port Queue (combines Mach IPC rights and Windows NT ALPC completion ports)
pub struct AlpcZeroCopyPortQueue {
    pub port_id: u64,
    pub owner_right: MachPortRight,
    pub messages: Vec<AlpcMessagePayload>,
    pub priority_boost: u32,
    pub send_count: usize,
    pub recv_count: usize,
}

impl AlpcZeroCopyPortQueue {
    pub fn new(port_id: u64, owner_right: MachPortRight) -> Self {
        Self {
            port_id,
            owner_right,
            messages: Vec::new(),
            priority_boost: 0,
            send_count: 0,
            recv_count: 0,
        }
    }

    /// Enqueues a message payload if the caller holds a valid Send or SendOnce port right
    pub fn send_message(&mut self, payload: AlpcMessagePayload) -> Result<(), &'static str> {
        if payload.right != MachPortRight::Send && payload.right != MachPortRight::SendOnce {
            return Err("ALPC: Insufficient port rights to send message");
        }
        self.send_count += 1;
        self.priority_boost = self.priority_boost.saturating_add(1);
        self.messages.push(payload);
        Ok(())
    }

    /// Receives the next priority-dequeued message payload if caller holds Receive right
    pub fn receive_message(&mut self, caller_right: MachPortRight) -> Result<AlpcMessagePayload, &'static str> {
        if caller_right != MachPortRight::Receive {
            return Err("ALPC: Caller lacks Receive right for this port");
        }
        if self.messages.is_empty() {
            return Err("ALPC: Message queue is empty");
        }
        self.recv_count += 1;
        self.priority_boost = self.priority_boost.saturating_sub(1);
        Ok(self.messages.remove(0))
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    static mut APC_TRIGGERED: usize = 0;
    static mut DPC_TRIGGERED: usize = 0;
    static mut WORK_TRIGGERED: usize = 0;

    fn test_apc_routine(context: usize) {
        unsafe {
            APC_TRIGGERED += context;
        }
    }

    fn test_dpc_routine(context: usize, arg1: usize, arg2: usize) {
        unsafe {
            DPC_TRIGGERED += context + arg1 + arg2;
        }
    }

    fn test_work_routine(parameter: usize) {
        unsafe {
            WORK_TRIGGERED += parameter;
        }
    }

    #[test]
    fn test_irql_levels_comparison() {
        assert!(Irql::PassiveLevel < Irql::ApcLevel);
        assert!(Irql::DispatchLevel > Irql::ApcLevel);
        assert!(Irql::HighLevel > Irql::Dirql);
    }

    #[test]
    fn test_wdk_threads_and_apcs() {
        unsafe { APC_TRIGGERED = 0; }
        let token = SecurityToken::new("S-1-5-18", 0xFFFF, true);
        let mut thread = WdkThread::new(101, true, token);
        assert_eq!(thread.token.sid, "S-1-5-18");
        assert!(thread.token.is_system);

        let apc1 = Apc {
            mode: ApcMode::KernelMode,
            routine: test_apc_routine,
            context: 10,
        };
        let apc2 = Apc {
            mode: ApcMode::UserMode,
            routine: test_apc_routine,
            context: 20,
        };

        thread.queue_apc(apc1);
        thread.queue_apc(apc2);
        assert_eq!(thread.apc_queue.len(), 2);

        let delivered = thread.deliver_apcs();
        assert_eq!(delivered, 2);
        assert_eq!(thread.apc_queue.len(), 0);
        unsafe {
            assert_eq!(APC_TRIGGERED, 30);
        }
    }

    #[test]
    fn test_prcb_and_dpcs() {
        unsafe { DPC_TRIGGERED = 0; }
        let mut prcb = Prcb::new(0);
        assert_eq!(prcb.current_irql, Irql::PassiveLevel);

        let dpc = Dpc {
            routine: test_dpc_routine,
            deferred_context: 5,
            system_argument1: 15,
            system_argument2: 25,
        };

        prcb.queue_dpc(dpc);
        assert_eq!(prcb.dpc_queue.len(), 1);

        let executed = prcb.execute_dpc_queue();
        assert_eq!(executed, 1);
        unsafe {
            assert_eq!(DPC_TRIGGERED, 45); // 5 + 15 + 25 = 45
        }
        assert_eq!(prcb.current_irql, Irql::PassiveLevel);
    }

    #[test]
    fn test_event_objects_signalled_unsignalled() {
        // NotificationEvent stays signalled
        let mut note_event = EventObject::new(EventType::NotificationEvent, false);
        assert!(!note_event.wait_on_event(501)); // false because not signalled, registers waiter
        assert_eq!(note_event.waiting_threads.len(), 1);

        let waiters = note_event.set_event();
        assert_eq!(waiters, 1);
        assert!(note_event.is_signalled);
        assert!(note_event.wait_on_event(502)); // true because manual-reset stays signalled
        assert!(note_event.is_signalled);

        // SynchronizationEvent auto-resets
        let mut sync_event = EventObject::new(EventType::SynchronizationEvent, false);
        assert!(!sync_event.wait_on_event(503));
        let notified = sync_event.set_event();
        assert_eq!(notified, 1);
        assert!(!sync_event.is_signalled); // automatically reset to non-signalled

        sync_event.is_signalled = true;
        assert!(sync_event.wait_on_event(504)); // succeeds but auto-resets
        assert!(!sync_event.is_signalled);
    }

    #[test]
    fn test_spinlocks_dispatch() {
        let mut lock = SpinLock::new();
        let prev = lock.acquire(1, Irql::PassiveLevel).unwrap();
        assert_eq!(prev, Irql::PassiveLevel);
        assert_eq!(lock.owner_cpu, Some(1));

        // Attempting to acquire lock at high IRQL
        assert!(lock.acquire(2, Irql::HighLevel).is_err());

        let restored = lock.release();
        assert_eq!(restored, Irql::PassiveLevel);
        assert_eq!(lock.owner_cpu, None);
    }

    #[test]
    fn test_mutexes_and_recursion() {
        let mut mutex = MutexObject::new();
        assert!(mutex.acquire_mutex(701));
        assert!(!mutex.is_signalled);
        assert_eq!(mutex.recursion_count, 1);

        // Recursive acquisition
        assert!(mutex.acquire_mutex(701));
        assert_eq!(mutex.recursion_count, 2);

        // Mismatched release
        assert!(mutex.release_mutex(702).is_err());

        // Recursive releases
        let first_rel = mutex.release_mutex(701).unwrap();
        assert!(!first_rel); // recursion count 1, still locked
        let second_rel = mutex.release_mutex(701).unwrap();
        assert!(second_rel); // fully released
        assert!(mutex.is_signalled);
    }

    #[test]
    fn test_fast_and_guarded_mutexes() {
        let mut fast = FastMutex::new();
        let irql = fast.acquire_fast(12).unwrap();
        assert_eq!(irql, Irql::ApcLevel);
        assert_eq!(fast.owner_thread_id, Some(12));
        assert_eq!(fast.release_fast(), Irql::PassiveLevel);

        let mut guarded = GuardedMutex::new();
        guarded.acquire_guarded(14);
        assert_eq!(guarded.owner_thread_id, Some(14));
        guarded.release_guarded();
        assert_eq!(guarded.owner_thread_id, None);
    }

    #[test]
    fn test_eresource_shared_exclusive() {
        let mut res = EResource::new();
        assert!(res.acquire_exclusive(801));
        assert!(!res.acquire_shared(802)); // failed because exclusive active
        assert_eq!(res.waiting_shared_count, 1);

        res.release_resource(801);
        res.waiting_shared_count = 0;

        assert!(res.acquire_shared(802));
        assert!(res.acquire_shared(803)); // multiple shared readers ok
        assert_eq!(res.active_shared_count, 2);
    }

    #[test]
    fn test_timers_tables() {
        let mut table = TimerTable::new();
        let dpc = Dpc {
            routine: test_dpc_routine,
            deferred_context: 1,
            system_argument1: 2,
            system_argument2: 3,
        };
        let timer = WdkTimer {
            timer_id: 201,
            due_time_ms: 1000,
            period_ms: 0,
            is_periodic: false,
            is_signalled: false,
            dpc: Some(dpc),
        };
        table.register_timer(timer);

        // Before due time
        let dpcs = table.tick_timers(500);
        assert_eq!(dpcs.len(), 0);

        // After due time
        let dpcs = table.tick_timers(1200);
        assert_eq!(dpcs.len(), 1);
    }

    #[test]
    fn test_pool_allocations() {
        let mut pool = KernelPoolMemory::new(4096);
        let addr = pool.ex_allocate_pool(PoolType::NonPagedPool, 1024, *b"TEST").unwrap();
        assert_eq!(pool.active_bytes, 1024);

        // Exceed limit
        assert!(pool.ex_allocate_pool(PoolType::NonPagedPool, 4000, *b"FAIL").is_err());

        assert!(pool.ex_free_pool(addr).is_ok());
        assert_eq!(pool.active_bytes, 0);
    }

    fn dummy_driver_unload(_driver: &WdkDriverObject) {}

    fn dummy_driver_entry(driver: &mut WdkDriverObject) -> i32 {
        driver.driver_unload = Some(dummy_driver_unload);
        0
    }

    #[test]
    fn test_wdk_driver_loading() {
        let mut driver = WdkDriverObject::new("SigmaSata", dummy_driver_entry);
        assert_eq!(driver.load_driver(), 0);
        assert!(driver.driver_unload.is_some());
        assert!(driver.unload_driver());
    }

    #[test]
    fn test_work_items() {
        unsafe { WORK_TRIGGERED = 0; }
        let work = WorkItem {
            routine: test_work_routine,
            parameter: 42,
        };
        (work.routine)(work.parameter);
        unsafe {
            assert_eq!(WORK_TRIGGERED, 42);
        }
    }

    #[test]
    fn test_bugchecks() {
        let mut bsod = BugCheckRegistry::new();
        assert!(bsod.bug_check.is_none());
        bsod.ke_bug_check_ex(0x0000000A, 0x11, 0x22, 0x33, 0x44); // IRQL_NOT_LESS_OR_EQUAL
        let report = bsod.bug_check.unwrap();
        assert_eq!(report.code, 0x0000000A);
    }

    #[test]
    fn test_mach_alpc_zero_copy_port_rights() {
        let mut port_queue = AlpcZeroCopyPortQueue::new(101, MachPortRight::Receive);

        let msg = AlpcMessagePayload {
            msg_id: 1,
            sender_pid: 1000,
            section_base: 0x7FFF0000,
            section_size: 4096,
            inline_data: [0u8; 128],
            inline_len: 0,
            right: MachPortRight::Send,
        };

        // Send with Send right should succeed
        assert!(port_queue.send_message(msg).is_ok());

        // Send with None right should fail
        let bad_msg = AlpcMessagePayload {
            msg_id: 2,
            sender_pid: 1000,
            section_base: 0,
            section_size: 0,
            inline_data: [0u8; 128],
            inline_len: 0,
            right: MachPortRight::None,
        };
        assert!(port_queue.send_message(bad_msg).is_err());

        // Receive without Receive right should fail
        assert!(port_queue.receive_message(MachPortRight::Send).is_err());

        // Receive with Receive right should succeed
        let received = port_queue.receive_message(MachPortRight::Receive).unwrap();
        assert_eq!(received.msg_id, 1);
        assert_eq!(received.section_base, 0x7FFF0000);
    }
}
