#![cfg_attr(not(test), no_std)]
// SigmaOS Windows-Driver-Kit (WDK) Core Subsystem
// Inspired by: x86-64/ARM/CISC architectures, Windows Driver Kit (Executive, Kernel, I/O Manager), Linux Kernel, and BSD.
// Zero external library dependency, no_std compliant.

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
    X86,
    X86_64,
    Arm64,
    CiscSim,
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
// 3. Work Queue Items, Executive Worker Threads (ExpWorkerThread), and OOP Hierarchy
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkQueueType {
    CriticalWorkQueue = 0,
    DelayedWorkQueue = 1,
    HyperCriticalWorkQueue = 2,
}

pub type PworkerRoutine = fn(parameter: usize);

pub trait WorkItemBase {
    fn execute(&mut self);
    fn routine_ptr(&self) -> usize;
    fn parameter(&self) -> usize;
    fn is_non_paged_pool(&self) -> bool;
    fn is_caller_allocated(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct ExecutiveWorkItem {
    pub list_entry_flink: usize,
    pub worker_routine: PworkerRoutine,
    pub parameter: usize,
    pub queue_type: WorkQueueType,
    pub is_non_paged_pool: bool,
    pub is_caller_allocated: bool,
}

impl ExecutiveWorkItem {
    pub fn new(routine: PworkerRoutine, parameter: usize, queue_type: WorkQueueType) -> Self {
        Self {
            list_entry_flink: 0,
            worker_routine: routine,
            parameter,
            queue_type,
            is_non_paged_pool: false,
            is_caller_allocated: true,
        }
    }
}

impl WorkItemBase for ExecutiveWorkItem {
    fn execute(&mut self) {
        (self.worker_routine)(self.parameter);
    }

    fn routine_ptr(&self) -> usize {
        self.worker_routine as usize
    }

    fn parameter(&self) -> usize {
        self.parameter
    }

    fn is_non_paged_pool(&self) -> bool {
        self.is_non_paged_pool
    }

    fn is_caller_allocated(&self) -> bool {
        self.is_caller_allocated
    }
}

#[derive(Debug, Clone)]
pub struct IoWorkItem {
    pub base_item: ExecutiveWorkItem,
    pub device_object_ptr: usize,
    pub driver_routine: PworkerRoutine,
    pub context: usize,
}

impl IoWorkItem {
    pub fn new(device_object_ptr: usize, driver_routine: PworkerRoutine, context: usize) -> Self {
        Self {
            base_item: ExecutiveWorkItem {
                list_entry_flink: 0,
                worker_routine: driver_routine,
                parameter: context,
                queue_type: WorkQueueType::DelayedWorkQueue,
                is_non_paged_pool: true,
                is_caller_allocated: false,
            },
            device_object_ptr,
            driver_routine,
            context,
        }
    }
}

impl WorkItemBase for IoWorkItem {
    fn execute(&mut self) {
        (self.driver_routine)(self.context);
    }

    fn routine_ptr(&self) -> usize {
        self.driver_routine as usize
    }

    fn parameter(&self) -> usize {
        self.context
    }

    fn is_non_paged_pool(&self) -> bool {
        self.base_item.is_non_paged_pool
    }

    fn is_caller_allocated(&self) -> bool {
        self.base_item.is_caller_allocated
    }
}

#[derive(Debug, Clone)]
pub struct ExpWorkerThread {
    pub thread_id: u32,
    pub system_process_pid: u32, // PID 4 for System Process context
    pub queue_type: WorkQueueType,
    pub items_processed: usize,
}

impl ExpWorkerThread {
    pub fn new(id: u32, queue_type: WorkQueueType) -> Self {
        Self {
            thread_id: id,
            system_process_pid: 4, // System Process
            queue_type,
            items_processed: 0,
        }
    }
}

pub struct ExpWorkQueueSystem {
    pub hyper_critical_queue: Vec<ExecutiveWorkItem>,
    pub critical_queue: Vec<ExecutiveWorkItem>,
    pub delayed_queue: Vec<ExecutiveWorkItem>,
    pub worker_threads: Vec<ExpWorkerThread>,
}

impl ExpWorkQueueSystem {
    pub fn new() -> Self {
        Self {
            hyper_critical_queue: Vec::new(),
            critical_queue: Vec::new(),
            delayed_queue: Vec::new(),
            worker_threads: Vec::new(),
        }
    }

    pub fn ex_initialize_work_item(
        item: &mut ExecutiveWorkItem,
        routine: PworkerRoutine,
        parameter: usize,
        queue_type: WorkQueueType,
    ) {
        item.worker_routine = routine;
        item.parameter = parameter;
        item.queue_type = queue_type;
    }

    pub fn ex_queue_work_item(&mut self, item: ExecutiveWorkItem, queue_type: WorkQueueType) {
        match queue_type {
            WorkQueueType::HyperCriticalWorkQueue => self.hyper_critical_queue.push(item),
            WorkQueueType::CriticalWorkQueue => self.critical_queue.push(item),
            WorkQueueType::DelayedWorkQueue => self.delayed_queue.push(item),
        }
    }

    pub fn process_work_items(&mut self, worker_thread_id: u32) -> usize {
        let mut processed = 0;

        // Process in priority order: HyperCritical -> Critical -> Delayed
        while let Some(mut item) = self.hyper_critical_queue.pop() {
            item.execute();
            processed += 1;
        }

        while let Some(mut item) = self.critical_queue.pop() {
            item.execute();
            processed += 1;
        }

        while let Some(mut item) = self.delayed_queue.pop() {
            item.execute();
            processed += 1;
        }

        if let Some(thread) = self.worker_threads.iter_mut().find(|t| t.thread_id == worker_thread_id) {
            thread.items_processed += processed;
        }

        processed
    }
}

impl Default for ExpWorkQueueSystem {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. APCs (Asynchronous Procedure Calls), Alertable State, Thread Suspension & Process Shutdown
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Initialized,
    Ready,
    Running,
    Waiting,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorMode {
    KernelMode = 0,
    UserMode = 1,
}

pub type PkernelRoutine = fn(
    apc: &mut Apc,
    normal_routine: &mut Option<PnormalRoutine>,
    normal_context: &mut usize,
    system_argument1: &mut usize,
    system_argument2: &mut usize,
);

pub type PrundownRoutine = fn(apc: &Apc);

pub type PnormalRoutine = fn(normal_context: usize, system_argument1: usize, system_argument2: usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApcEnvironment {
    OriginalApcEnvironment = 0,
    AttachedApcEnvironment = 1,
    CurrentApcEnvironment = 2,
}

#[derive(Clone)]
pub struct Apc {
    pub target_thread_id: u32,
    pub processor_mode: ProcessorMode,
    pub kernel_routine: Option<PkernelRoutine>,
    pub rundown_routine: Option<PrundownRoutine>,
    pub normal_routine: Option<PnormalRoutine>,
    pub normal_context: usize,
    pub system_argument1: usize,
    pub system_argument2: usize,
    pub environment: ApcEnvironment,
    pub is_special_kernel_apc: bool,
    pub is_queued: bool,
}

impl Apc {
    pub fn new_kernel_apc(
        target_thread_id: u32,
        kernel_routine: PkernelRoutine,
        rundown_routine: Option<PrundownRoutine>,
        normal_routine: Option<PnormalRoutine>,
        normal_context: usize,
        sys_arg1: usize,
        sys_arg2: usize,
        is_special: bool,
    ) -> Self {
        Self {
            target_thread_id,
            processor_mode: ProcessorMode::KernelMode,
            kernel_routine: Some(kernel_routine),
            rundown_routine,
            normal_routine,
            normal_context,
            system_argument1: sys_arg1,
            system_argument2: sys_arg2,
            environment: ApcEnvironment::OriginalApcEnvironment,
            is_special_kernel_apc: is_special,
            is_queued: false,
        }
    }

    pub fn new_user_apc(
        target_thread_id: u32,
        kernel_routine: PkernelRoutine,
        rundown_routine: Option<PrundownRoutine>,
        normal_routine: PnormalRoutine,
        normal_context: usize,
        sys_arg1: usize,
        sys_arg2: usize,
    ) -> Self {
        Self {
            target_thread_id,
            processor_mode: ProcessorMode::UserMode,
            kernel_routine: Some(kernel_routine),
            rundown_routine,
            normal_routine: Some(normal_routine),
            normal_context,
            system_argument1: sys_arg1,
            system_argument2: sys_arg2,
            environment: ApcEnvironment::OriginalApcEnvironment,
            is_special_kernel_apc: false,
            is_queued: false,
        }
    }
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
    pub process_id: u32,
    pub is_system_thread: bool,
    pub state: ThreadState,
    pub base_priority: i8,
    pub current_priority: i8,
    pub token: SecurityToken,
    pub registers: [u64; 16], // x86-64/ARM/CISC thread registers
    pub apc_queue: Vec<Apc>,
    pub alertable: bool,
    pub is_suspended: bool,
    pub suspend_count: u32,
    pub active_processor_mode: ProcessorMode,
}

impl WdkThread {
    pub fn new(id: u32, process_id: u32, is_system: bool, token: SecurityToken) -> Self {
        Self {
            thread_id: id,
            process_id,
            is_system_thread: is_system,
            state: ThreadState::Ready,
            base_priority: 8,
            current_priority: 8,
            token,
            registers: [0u64; 16],
            apc_queue: Vec::new(),
            alertable: false,
            is_suspended: false,
            suspend_count: 0,
            active_processor_mode: ProcessorMode::KernelMode,
        }
    }

    pub fn queue_apc(&mut self, mut apc: Apc) {
        apc.is_queued = true;
        self.apc_queue.push(apc);
    }

    pub fn ke_delay_execution_thread(&mut self, alertable: bool, _timeout_ms: u64) -> bool {
        self.alertable = alertable;
        self.state = ThreadState::Waiting;

        // If alertable, deliver pending User/Kernel APCs immediately
        if alertable {
            let delivered = self.deliver_apcs();
            if delivered > 0 {
                self.state = ThreadState::Running;
                return true; // Interrupted by APC
            }
        }
        self.state = ThreadState::Running;
        false
    }

    pub fn suspend_thread(&mut self) -> u32 {
        self.suspend_count += 1;
        self.is_suspended = true;
        self.state = ThreadState::Suspended;
        self.suspend_count
    }

    pub fn resume_thread(&mut self) -> u32 {
        if self.suspend_count > 0 {
            self.suspend_count -= 1;
            if self.suspend_count == 0 {
                self.is_suspended = false;
                self.state = ThreadState::Ready;
            }
        }
        self.suspend_count
    }

    pub fn deliver_apcs(&mut self) -> usize {
        let mut delivered = 0;
        let mut pending = Vec::new();
        core::mem::swap(&mut self.apc_queue, &mut pending);

        let mut unhandled = Vec::new();

        for mut apc in pending {
            let can_deliver = match apc.processor_mode {
                ProcessorMode::KernelMode => {
                    apc.is_special_kernel_apc || self.active_processor_mode == ProcessorMode::KernelMode
                }
                ProcessorMode::UserMode => {
                    self.alertable && self.active_processor_mode == ProcessorMode::UserMode
                }
            };

            if can_deliver {
                let mut norm_routine = apc.normal_routine;
                let mut norm_ctx = apc.normal_context;
                let mut sys_arg1 = apc.system_argument1;
                let mut sys_arg2 = apc.system_argument2;

                // 1. Execute Kernel Routine if present
                if let Some(krn) = apc.kernel_routine {
                    krn(
                        &mut apc,
                        &mut norm_routine,
                        &mut norm_ctx,
                        &mut sys_arg1,
                        &mut sys_arg2,
                    );
                }

                // 2. Execute Normal Routine if kernel routine didn't clear it
                if let Some(norm) = norm_routine {
                    norm(norm_ctx, sys_arg1, sys_arg2);
                }

                delivered += 1;
            } else {
                unhandled.push(apc);
            }
        }

        // Re-queue undelivered APCs
        self.apc_queue.extend(unhandled);
        delivered
    }
}

pub struct WdkProcess {
    pub process_id: u32,
    pub name: String,
    pub threads: Vec<WdkThread>,
    pub is_shutting_down: bool,
}

impl WdkProcess {
    pub fn new(pid: u32, name: &str) -> Self {
        Self {
            process_id: pid,
            name: name.to_string(),
            threads: Vec::new(),
            is_shutting_down: false,
        }
    }

    pub fn process_shutdown(&mut self) {
        self.is_shutting_down = true;

        for thread in &mut self.threads {
            // Execute rundown routines on all pending APCs
            let mut pending = Vec::new();
            core::mem::swap(&mut thread.apc_queue, &mut pending);

            for apc in pending {
                if let Some(rundown) = apc.rundown_routine {
                    rundown(&apc);
                }
            }
            thread.state = ThreadState::Terminated;
        }
    }
}

// =========================================================================
// 5. Kernel Synchronization Primitives (Events, SpinLocks, Mutexes, ERESOURCE)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    NotificationEvent,    // Manual-reset (stays signalled until cleared)
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
// 6. Timers & KPRCB (Kernel Processor Control Block) with NUMA Nodes
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DpcImportance {
    LowImportance = 0,
    MediumImportance = 1,
    HighImportance = 2,
    HighImportanceNotified = 3,
}

// KPRCB (Kernel Processor Control Block)
pub struct Prcb {
    pub cpu_id: u32,
    pub numa_node_id: u32,
    pub arch: CpuArch,
    pub current_irql: Irql,
    pub active_thread_id: Option<u32>,
    pub idle_thread_id: Option<u32>,
    pub dpc_queue: Vec<(Dpc, DpcImportance)>,
    pub timer_table: TimerTable,
    pub dpc_count: u64,
    pub apc_pending: bool,
}

impl Prcb {
    pub fn new(cpu_id: u32, numa_node_id: u32, arch: CpuArch) -> Self {
        Self {
            cpu_id,
            numa_node_id,
            arch,
            current_irql: Irql::PassiveLevel,
            active_thread_id: None,
            idle_thread_id: None,
            dpc_queue: Vec::new(),
            timer_table: TimerTable::new(),
            dpc_count: 0,
            apc_pending: false,
        }
    }

    pub fn queue_dpc(&mut self, dpc: Dpc, importance: DpcImportance) {
        if importance == DpcImportance::HighImportance || importance == DpcImportance::HighImportanceNotified {
            self.dpc_queue.insert(0, (dpc, importance));
        } else {
            self.dpc_queue.push((dpc, importance));
        }
    }

    pub fn execute_dpc_queue(&mut self) -> usize {
        let previous_irql = self.current_irql;
        self.current_irql = Irql::DispatchLevel;

        let mut count = 0;
        let mut pending = Vec::new();
        core::mem::swap(&mut self.dpc_queue, &mut pending);

        for (dpc, _importance) in pending {
            (dpc.routine)(dpc.deferred_context, dpc.system_argument1, dpc.system_argument2);
            count += 1;
            self.dpc_count += 1;
        }

        self.current_irql = previous_irql;
        count
    }
}

// =========================================================================
// 7. Memory Pools (NonPagedPool / PagedPool / Caller-Allocated Buffers)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolType {
    NonPagedPool,       // Guaranteed in RAM (safe for high IRQL)
    PagedPool,          // Can be paged (PASSIVE_LEVEL / APC_LEVEL)
    NonPagedPoolNx,     // NonPagedPool with No-Execute permission
}

pub struct PoolAllocation {
    pub address: usize,
    pub size: usize,
    pub pool_type: PoolType,
    pub tag: [u8; 4], // standard 4-byte pool tag
    pub freed_in_normal_routine: bool,
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
            freed_in_normal_routine: false,
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
// 8. WDK Drivers & Async I/O Completion (io_complete_request)
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
    pub user_buffer_ptr: usize,
    pub io_status: IoStatusBlock,
    pub is_completed: bool,
    pub completion_event: Option<EventObject>,
}

impl IRP {
    pub fn new(ioctl: IoctlControl, input: Vec<u8>, user_buffer_ptr: usize) -> Self {
        Self {
            ioctl,
            input_buffer: input,
            output_buffer: Vec::new(),
            user_buffer_ptr,
            io_status: IoStatusBlock {
                status: 0,
                information: 0,
            },
            is_completed: false,
            completion_event: Some(EventObject::new(EventType::NotificationEvent, false)),
        }
    }
}

pub fn io_complete_request(
    irp: &mut IRP,
    _priority_boost: i8,
    thread: &mut WdkThread,
    _pool: &mut KernelPoolMemory,
) -> Result<(), &'static str> {
    irp.is_completed = true;

    if let Some(ref mut event) = irp.completion_event {
        event.set_event();
    }

    // Special Kernel APC for I/O completion
    fn io_completion_kernel_apc(
        _apc: &mut Apc,
        _norm_routine: &mut Option<PnormalRoutine>,
        _norm_ctx: &mut usize,
        _sys_arg1: &mut usize,
        _sys_arg2: &mut usize,
    ) {
        // Special kernel routine frees kernel resources, copies buffers to user space
    }

    let apc = Apc::new_kernel_apc(
        thread.thread_id,
        io_completion_kernel_apc,
        None,
        None,
        0,
        irp.io_status.information,
        irp.io_status.status as usize,
        true, // Special Kernel APC
    );

    thread.queue_apc(apc);
    Ok(())
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
// MACH / ALPC ZERO-COPY MESSAGE PORT RIGHTS
// =========================================================================

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachPortRight {
    None = 0,
    Receive = 1,
    Send = 2,
    SendOnce = 3,
    PortSet = 4,
}

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

    pub fn send_message(&mut self, payload: AlpcMessagePayload) -> Result<(), &'static str> {
        if payload.right != MachPortRight::Send && payload.right != MachPortRight::SendOnce {
            return Err("ALPC: Insufficient port rights to send message");
        }
        self.send_count += 1;
        self.priority_boost = self.priority_boost.saturating_add(1);
        self.messages.push(payload);
        Ok(())
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::{AtomicUsize, Ordering};

    static APC_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static DPC_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static WORK_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static RUNDOWN_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn test_normal_routine(context: usize, sys_arg1: usize, sys_arg2: usize) {
        APC_COUNTER.fetch_add(context + sys_arg1 + sys_arg2, Ordering::SeqCst);
    }

    fn test_dpc_routine(context: usize, arg1: usize, arg2: usize) {
        DPC_COUNTER.fetch_add(context + arg1 + arg2, Ordering::SeqCst);
    }

    fn test_work_routine(parameter: usize) {
        WORK_COUNTER.fetch_add(parameter, Ordering::SeqCst);
    }

    fn test_rundown_routine(_apc: &Apc) {
        RUNDOWN_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_irql_levels_comparison() {
        assert!(Irql::PassiveLevel < Irql::ApcLevel);
        assert!(Irql::DispatchLevel > Irql::ApcLevel);
        assert!(Irql::HighLevel > Irql::Dirql);
    }

    #[test]
    fn test_executive_work_queue_and_exp_worker_thread() {
        WORK_COUNTER.store(0, Ordering::SeqCst);
        let mut wq_sys = ExpWorkQueueSystem::new();
        let worker = ExpWorkerThread::new(1, WorkQueueType::CriticalWorkQueue);
        wq_sys.worker_threads.push(worker);

        let item1 = ExecutiveWorkItem::new(test_work_routine, 10, WorkQueueType::CriticalWorkQueue);
        let item2 = ExecutiveWorkItem::new(test_work_routine, 20, WorkQueueType::HyperCriticalWorkQueue);

        wq_sys.ex_queue_work_item(item1, WorkQueueType::CriticalWorkQueue);
        wq_sys.ex_queue_work_item(item2, WorkQueueType::HyperCriticalWorkQueue);

        let processed = wq_sys.process_work_items(1);
        assert_eq!(processed, 2);
        assert_eq!(WORK_COUNTER.load(Ordering::SeqCst), 30);
        assert_eq!(wq_sys.worker_threads[0].items_processed, 2);
    }

    #[test]
    fn test_oop_work_item_hierarchy() {
        let mut io_item = IoWorkItem::new(0xDEADBEEF, test_work_routine, 50);
        assert!(io_item.is_non_paged_pool());
        assert!(!io_item.is_caller_allocated());
        assert_eq!(io_item.parameter(), 50);

        WORK_COUNTER.store(0, Ordering::SeqCst);
        io_item.execute();
        assert_eq!(WORK_COUNTER.load(Ordering::SeqCst), 50);
    }

    #[test]
    fn test_apc_engine_alertable_wait_and_kernel_user_modes() {
        APC_COUNTER.store(0, Ordering::SeqCst);
        let token = SecurityToken::new("S-1-5-18", 0xFFFF, true);
        let mut thread = WdkThread::new(101, 4, true, token);

        let kernel_apc = Apc::new_kernel_apc(
            101,
            |_apc, norm, _ctx, _a1, _a2| {
                // Kernel routine lets normal routine execute
                assert!(norm.is_some());
            },
            Some(test_rundown_routine),
            Some(test_normal_routine),
            10,
            5,
            5,
            true, // Special Kernel APC
        );

        thread.queue_apc(kernel_apc);

        // Deliver Special Kernel APC even when in Kernel Mode
        let delivered = thread.deliver_apcs();
        assert_eq!(delivered, 1);
        assert_eq!(APC_COUNTER.load(Ordering::SeqCst), 20); // 10 + 5 + 5

        // User APC in Alertable Wait
        APC_COUNTER.store(0, Ordering::SeqCst);
        let user_apc = Apc::new_user_apc(
            101,
            |_apc, _norm, _ctx, _a1, _a2| {},
            Some(test_rundown_routine),
            test_normal_routine,
            100,
            0,
            0,
        );

        thread.queue_apc(user_apc);
        thread.active_processor_mode = ProcessorMode::UserMode;

        // Alertable wait delivers User APC
        let interrupted = thread.ke_delay_execution_thread(true, 100);
        assert!(interrupted);
        assert_eq!(APC_COUNTER.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn test_thread_suspension_and_process_shutdown() {
        RUNDOWN_COUNTER.store(0, Ordering::SeqCst);
        let token = SecurityToken::new("S-1-5-18", 0xFFFF, true);
        let mut thread = WdkThread::new(202, 10, false, token.clone());

        assert_eq!(thread.suspend_thread(), 1);
        assert_eq!(thread.state, ThreadState::Suspended);

        let apc = Apc::new_user_apc(
            202,
            |_apc, _norm, _ctx, _a1, _a2| {},
            Some(test_rundown_routine),
            test_normal_routine,
            1,
            2,
            3,
        );
        thread.queue_apc(apc);

        let mut process = WdkProcess::new(10, "test_app.exe");
        process.threads.push(thread);

        process.process_shutdown();
        assert!(process.is_shutting_down);
        assert_eq!(process.threads[0].state, ThreadState::Terminated);
        assert_eq!(RUNDOWN_COUNTER.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_numa_kprcb_and_dpcs_importance() {
        DPC_COUNTER.store(0, Ordering::SeqCst);
        let mut prcb = Prcb::new(0, 1, CpuArch::X86_64);
        assert_eq!(prcb.numa_node_id, 1);
        assert_eq!(prcb.arch, CpuArch::X86_64);

        let dpc1 = Dpc {
            routine: test_dpc_routine,
            deferred_context: 5,
            system_argument1: 10,
            system_argument2: 15,
        };

        let dpc2 = Dpc {
            routine: test_dpc_routine,
            deferred_context: 100,
            system_argument1: 0,
            system_argument2: 0,
        };

        prcb.queue_dpc(dpc1, DpcImportance::LowImportance);
        prcb.queue_dpc(dpc2, DpcImportance::HighImportance);

        let executed = prcb.execute_dpc_queue();
        assert_eq!(executed, 2);
        assert_eq!(DPC_COUNTER.load(Ordering::SeqCst), 130);
    }

    #[test]
    fn test_async_io_completion() {
        let token = SecurityToken::new("S-1-5-18", 0xFFFF, true);
        let mut thread = WdkThread::new(303, 4, true, token);
        let mut pool = KernelPoolMemory::new(4096);

        let mut irp = IRP::new(IoctlControl::IoctlBufferIo, vec![1, 2, 3], 0x7FFF0000);
        irp.io_status.status = 0;
        irp.io_status.information = 3;

        assert!(io_complete_request(&mut irp, 2, &mut thread, &mut pool).is_ok());
        assert!(irp.is_completed);
        assert_eq!(thread.apc_queue.len(), 1);
        assert!(thread.apc_queue[0].is_special_kernel_apc);
    }

    #[test]
    fn test_event_objects_signalled_unsignalled() {
        let mut note_event = EventObject::new(EventType::NotificationEvent, false);
        assert!(!note_event.wait_on_event(501));
        assert_eq!(note_event.waiting_threads.len(), 1);

        let waiters = note_event.set_event();
        assert_eq!(waiters, 1);
        assert!(note_event.is_signalled);
        assert!(note_event.wait_on_event(502));
        assert!(note_event.is_signalled);

        let mut sync_event = EventObject::new(EventType::SynchronizationEvent, false);
        assert!(!sync_event.wait_on_event(503));
        let notified = sync_event.set_event();
        assert_eq!(notified, 1);
        assert!(!sync_event.is_signalled);

        sync_event.is_signalled = true;
        assert!(sync_event.wait_on_event(504));
        assert!(!sync_event.is_signalled);
    }

    #[test]
    fn test_spinlocks_dispatch() {
        let mut lock = SpinLock::new();
        let prev = lock.acquire(1, Irql::PassiveLevel).unwrap();
        assert_eq!(prev, Irql::PassiveLevel);
        assert_eq!(lock.owner_cpu, Some(1));

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

        assert!(mutex.acquire_mutex(701));
        assert_eq!(mutex.recursion_count, 2);

        assert!(mutex.release_mutex(702).is_err());

        let first_rel = mutex.release_mutex(701).unwrap();
        assert!(!first_rel);
        let second_rel = mutex.release_mutex(701).unwrap();
        assert!(second_rel);
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
        assert!(!res.acquire_shared(802));
        assert_eq!(res.waiting_shared_count, 1);

        res.release_resource(801);
        res.waiting_shared_count = 0;

        assert!(res.acquire_shared(802));
        assert!(res.acquire_shared(803));
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

        let dpcs = table.tick_timers(500);
        assert_eq!(dpcs.len(), 0);

        let dpcs = table.tick_timers(1200);
        assert_eq!(dpcs.len(), 1);
    }

    #[test]
    fn test_pool_allocations() {
        let mut pool = KernelPoolMemory::new(4096);
        let addr = pool.ex_allocate_pool(PoolType::NonPagedPool, 1024, *b"TEST").unwrap();
        assert_eq!(pool.active_bytes, 1024);

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
    fn test_bugchecks() {
        let mut bsod = BugCheckRegistry::new();
        assert!(bsod.bug_check.is_none());
        bsod.ke_bug_check_ex(0x0000000A, 0x11, 0x22, 0x33, 0x44);
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

        assert!(port_queue.send_message(msg).is_ok());

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

        assert!(port_queue.receive_message(MachPortRight::Send).is_err());

        let received = port_queue.receive_message(MachPortRight::Receive).unwrap();
        assert_eq!(received.msg_id, 1);
        assert_eq!(received.section_base, 0x7FFF0000);
    }
}
