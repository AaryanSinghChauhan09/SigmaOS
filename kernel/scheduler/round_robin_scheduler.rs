/// SigmaOS: Round-Robin Scheduler Implementation
/// Phase G Blocker #1: Round-robin scheduler
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.


#[allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Task Control Block (TCB) ───────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskState {
    pub running: SigmaBool,
    pub ready: SigmaBool,
    pub blocked: SigmaBool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskPriority {
    pub level: SigmaU8,
    pub static_prio: SigmaU8,
    pub dynamic_prio: SigmaU8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskContext {
    pub rip: SigmaU64,      // Instruction pointer
    pub rsp: SigmaU64,      // Stack pointer
    pub rbp: SigmaU64,      // Base pointer
    pub rax: SigmaU64,      // General purpose registers
    pub rbx: SigmaU64,
    pub rcx: SigmaU64,
    pub rdx: SigmaU64,
    pub rsi: SigmaU64,
    pub rdi: SigmaU64,
    pub r8:  SigmaU64,
    pub r9:  SigmaU64,
    pub r10: SigmaU64,
    pub r11: SigmaU64,
    pub r12: SigmaU64,
    pub r13: SigmaU64,
    pub r14: SigmaU64,
    pub r15: SigmaU64,
    pub rflags: SigmaU64,    // CPU flags
    pub cr3: SigmaU64,       // Page table base
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub tid: SigmaU64,           // Task ID
    pub state: TaskState,
    pub priority: TaskPriority,
    pub context: TaskContext,
    pub time_slice: SigmaU64,    // Remaining time slice
    pub total_runtime: SigmaU64, // Total runtime
    pub last_run: SigmaU64,      // Last run timestamp
    pub next: Option<SigmaU64>,  // Next task in queue
    pub prev: Option<SigmaU64>,  // Previous task in queue
}

// ─── Round-Robin Scheduler ───────────────────────────────────────────────

pub const MAX_TASKS: usize = 256;
pub const DEFAULT_TIME_SLICE: SigmaU64 = 10; // 10ms default quantum

pub struct RoundRobinScheduler {
    initialized: SigmaBool,
    task_queue: [Option<SigmaU64>; MAX_TASKS],
    task_table: [Option<TaskControlBlock>; MAX_TASKS],
    current_task: Option<SigmaU64>,
    queue_head: Option<SigmaU64>,
    queue_tail: Option<SigmaU64>,
    task_count: SigmaUsize,
    total_switches: SigmaU64,
    time_slice: SigmaU64,
}

impl RoundRobinScheduler {
    pub const fn new() -> Self {
        Self {
            initialized: false,
            task_queue: [None; MAX_TASKS],
            task_table: [None; MAX_TASKS],
            current_task: None,
            queue_head: None,
            queue_tail: None,
            task_count: 0,
            total_switches: 0,
            time_slice: DEFAULT_TIME_SLICE,
        }
    }

    /// Initialize the round-robin scheduler
    pub unsafe fn init(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Err("Scheduler already initialized");
        }

        // Clear task table
        for i in 0..MAX_TASKS {
            self.task_table[i] = None;
            self.task_queue[i] = None;
        }

        self.current_task = None;
        self.queue_head = None;
        self.queue_tail = None;
        self.task_count = 0;
        self.total_switches = 0;
        self.time_slice = DEFAULT_TIME_SLICE;
        self.initialized = true;

        Ok(())
    }

    /// Add a task to the scheduler
    pub unsafe fn add_task(&mut self, tcb: TaskControlBlock) -> Result<SigmaU64, &'static str> {
        if !self.initialized {
            return Err("Scheduler not initialized");
        }

        if self.task_count >= MAX_TASKS {
            return Err("Maximum tasks reached");
        }

        // Find free slot
        let tid = match self.find_free_slot() {
            Some(id) => id,
            None => return Err("No free task slots"),
        };

        // Store task
        self.task_table[tid] = Some(tcb);
        self.task_queue[tid] = Some(tid as SigmaU64);

        // Add to queue
        self.enqueue_task(tid as SigmaU64);

        self.task_count += 1;

        Ok(tid as SigmaU64)
    }

    /// Remove a task from the scheduler
    pub unsafe fn remove_task(&mut self, tid: SigmaU64) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Scheduler not initialized");
        }

        let tid_usize = tid as usize;
        if tid_usize >= MAX_TASKS {
            return Err("Invalid task ID");
        }

        if self.task_table[tid_usize].is_none() {
            return Err("Task not found");
        }

        // Remove from queue
        self.dequeue_task(tid);

        // Clear task slot
        self.task_table[tid_usize] = None;
        self.task_queue[tid_usize] = None;

        self.task_count -= 1;

        Ok(())
    }

    /// Schedule tick - called by timer interrupt
    pub unsafe fn tick(&mut self) -> Option<SigmaU64> {
        if !self.initialized {
            return None;
        }

        if let Some(current_tid) = self.current_task {
            let tid_usize = current_tid as usize;
            
            if let Some(ref mut tcb) = self.task_table[tid_usize] {
                // Decrement time slice
                if tcb.time_slice > 0 {
                    tcb.time_slice -= 1;
                }

                // If time slice expired, schedule next task
                if tcb.time_slice == 0 {
                    tcb.time_slice = self.time_slice;
                    return self.schedule_next();
                }
            }
        }

        None
    }

    /// Yield current task voluntarily
    pub unsafe fn yield_task(&mut self) -> Option<SigmaU64> {
        if !self.initialized {
            return None;
        }

        self.schedule_next()
    }

    /// Get current running task
    pub unsafe fn get_current_task(&mut self) -> Option<SigmaU64> {
        self.current_task
    }

    /// Schedule next task (round-robin)
    unsafe fn schedule_next(&mut self) -> Option<SigmaU64> {
        if self.task_count == 0 {
            return None;
        }

        // Save current task context
        if let Some(current_tid) = self.current_task {
            let tid_usize = current_tid as usize;
            if let Some(ref mut tcb) = self.task_table[tid_usize] {
                tcb.state.ready = true;
                tcb.state.running = false;
            }
            
            // Move current task to end of queue
            self.dequeue_task(current_tid);
            self.enqueue_task(current_tid);
        }

        // Get next task from queue
        let next_tid = self.dequeue_head();
        
        let timestamp = self.get_timestamp();
        if let Some(tid) = next_tid {
            let tid_usize = tid as usize;
            if let Some(ref mut tcb) = self.task_table[tid_usize] {
                tcb.state.running = true;
                tcb.state.ready = false;
                tcb.time_slice = self.time_slice;
                tcb.last_run = timestamp;
            }

            self.current_task = Some(tid);
            self.total_switches += 1;
        }

        next_tid
    }

    /// Enqueue task at tail of queue
    unsafe fn enqueue_task(&mut self, tid: SigmaU64) {
        if let Some(tail) = self.queue_tail {
            let tail_usize = tail as usize;
            if let Some(ref mut tcb) = self.task_table[tail_usize] {
                tcb.next = Some(tid);
            }
        } else {
            // Queue was empty
            self.queue_head = Some(tid);
        }

        let tid_usize = tid as usize;
        if let Some(ref mut tcb) = self.task_table[tid_usize] {
            tcb.prev = self.queue_tail;
            tcb.next = None;
        }

        self.queue_tail = Some(tid);
    }

    /// Dequeue task from queue
    unsafe fn dequeue_task(&mut self, tid: SigmaU64) {
        let tid_usize = tid as usize;
        if let Some(tcb) = self.task_table[tid_usize] {
            if let Some(prev) = tcb.prev {
                let prev_usize = prev as usize;
                if prev_usize < MAX_TASKS {
                    if let Some(ref mut prev_tcb) = self.task_table[prev_usize] {
                        prev_tcb.next = tcb.next;
                    }
                }
            } else {
                // Task was head
                self.queue_head = tcb.next;
            }

            if let Some(next) = tcb.next {
                let next_usize = next as usize;
                if next_usize < MAX_TASKS {
                    if let Some(ref mut next_tcb) = self.task_table[next_usize] {
                        next_tcb.prev = tcb.prev;
                    }
                }
            } else {
                // Task was tail
                self.queue_tail = tcb.prev;
            }
        }
    }

    /// Dequeue head of queue
    unsafe fn dequeue_head(&mut self) -> Option<SigmaU64> {
        let head = self.queue_head?;
        self.dequeue_task(head);
        Some(head)
    }

    /// Find free task slot
    fn find_free_slot(&self) -> Option<usize> {
        for i in 0..MAX_TASKS {
            if self.task_table[i].is_none() {
                return Some(i);
            }
        }
        None
    }

    /// Get current timestamp using RDTSC
    fn get_timestamp(&self) -> SigmaU64 {
        unsafe {
            let mut low: u32;
            let mut high: u32;
            core::arch::asm!(
                "rdtsc",
                "mov edx, eax",
                "mov eax, 0",
                out("eax") low,
                out("edx") high,
                options(nomem, nostack)
            );
            ((high as SigmaU64) << 32) | (low as SigmaU64)
        }
    }

    /// Get task count
    pub unsafe fn get_task_count(&mut self) -> SigmaUsize {
        self.task_count
    }

    /// Get total context switches
    pub unsafe fn get_total_switches(&mut self) -> SigmaU64 {
        self.total_switches
    }

    /// Set time slice quantum
    pub unsafe fn set_time_slice(&mut self, quantum: SigmaU64) {
        if quantum > 0 {
            self.time_slice = quantum;
        }
    }

    /// Get time slice quantum
    pub unsafe fn get_time_slice(&mut self) -> SigmaU64 {
        self.time_slice
    }

    /// Print scheduler statistics (returns formatted string buffer)
    pub unsafe fn print_stats(&mut self, buf: &mut [u8]) -> usize {
        // Simple number-to-string conversion
        let mut written = 0;
        
        // Write "Tasks: "
        let prefix = b"Tasks: ";
        if written + prefix.len() < buf.len() {
            buf[written..written+prefix.len()].copy_from_slice(prefix);
            written += prefix.len();
        }
        
        // Write task count
        written += self.write_number(self.task_count as u64, &mut buf[written..]);
        
        // Write " Switches: "
        let prefix2 = b" Switches: ";
        if written + prefix2.len() < buf.len() {
            buf[written..written+prefix2.len()].copy_from_slice(prefix2);
            written += prefix2.len();
        }
        
        // Write total switches
        written += self.write_number(self.total_switches, &mut buf[written..]);
        
        // Write " Quantum: "
        let prefix3 = b" Quantum: ";
        if written + prefix3.len() < buf.len() {
            buf[written..written+prefix3.len()].copy_from_slice(prefix3);
            written += prefix3.len();
        }
        
        // Write time slice
        written += self.write_number(self.time_slice, &mut buf[written..]);
        
        written
    }
    
    /// Helper: Write number to buffer
    fn write_number(&self, mut num: SigmaU64, buf: &mut [u8]) -> usize {
        if num == 0 {
            if buf.len() > 0 {
                buf[0] = b'0';
                return 1;
            }
            return 0;
        }
        
        let mut digits = [0u8; 20];
        let mut len = 0;
        
        while num > 0 && len < 20 {
            digits[len] = (num % 10) as u8 + b'0';
            num /= 10;
            len += 1;
        }
        
        // Reverse and copy
        let mut written = 0;
        for i in (0..len).rev() {
            if written < buf.len() {
                buf[written] = digits[i];
                written += 1;
            }
        }
        
        written
    }
}

// ─── Global Scheduler Instance ─────────────────────────────────────────────

static mut SCHEDULER: RoundRobinScheduler = RoundRobinScheduler::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_sched_init() -> SigmaI32 {
    match SCHEDULER.init() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sched_add_task(tcb: *const TaskControlBlock) -> SigmaI32 {
    if tcb.is_null() {
        return -1;
    }
    
    match SCHEDULER.add_task(*tcb) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sched_remove_task(tid: SigmaU64) -> SigmaI32 {
    match SCHEDULER.remove_task(tid) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sched_tick() -> SigmaU64 {
    match SCHEDULER.tick() {
        Some(tid) => tid,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sched_yield() -> SigmaU64 {
    match SCHEDULER.yield_task() {
        Some(tid) => tid,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sched_get_current() -> SigmaU64 {
    match SCHEDULER.get_current_task() {
        Some(tid) => tid,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sched_get_count() -> SigmaUsize {
    SCHEDULER.get_task_count()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sched_get_switches() -> SigmaU64 {
    SCHEDULER.get_total_switches()
}
