use core::cell::{Cell, RefCell};
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

use std::boxed::Box;
use std::string::String;
use std::vec::Vec;

// 1. SINGLY LINKED LIST

pub struct SinglyListNode<T> {
    pub value: T,
    pub next: Option<Box<SinglyListNode<T>>>,
}

pub struct SinglyLinkedList<T> {
    pub head: Option<Box<SinglyListNode<T>>>,
    pub len: usize,
}

impl<T> SinglyLinkedList<T> {
    pub const fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(SinglyListNode {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// 2. SEQUENCED SINGLY LINKED LIST
// Each element has an epoch-sequence ID. Inspired by ARM / BSD epoch sequence tables.

pub struct SequencedSinglyListNode<T> {
    pub value: T,
    pub sequence: u64,
    pub next: Option<Box<SequencedSinglyListNode<T>>>,
}

pub struct SequencedSinglyLinkedList<T> {
    pub head: Option<Box<SequencedSinglyListNode<T>>>,
    pub next_sequence: u64,
    pub len: usize,
}

impl<T> SequencedSinglyLinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            next_sequence: 1,
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) -> u64 {
        let seq = self.next_sequence;
        self.next_sequence += 1;

        let new_node = Box::new(SequencedSinglyListNode {
            value,
            sequence: seq,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
        seq
    }

    pub fn pop_front(&mut self) -> Option<(T, u64)> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            (node.value, node.sequence)
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// 3. CIRCULAR DOUBLY LINKED LIST
// Inspired by Windows' sentinel LIST_ENTRY and Linux list_head.
// Emulates a circular doubly linked list with a sentinel head node.

pub struct CircularDoublyLinkedListNode<T> {
    pub value: T,
    pub next: Option<NonNull<CircularDoublyLinkedListNode<T>>>,
    pub prev: Option<NonNull<CircularDoublyLinkedListNode<T>>>,
}

pub struct CircularDoublyLinkedList<T> {
    pub head: Option<NonNull<CircularDoublyLinkedListNode<T>>>,
    pub tail: Option<NonNull<CircularDoublyLinkedListNode<T>>>,
    pub len: usize,
}

impl<T> CircularDoublyLinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_tail(&mut self, value: T) {
        let raw_node = Box::into_raw(Box::new(CircularDoublyLinkedListNode {
            value,
            next: None,
            prev: None,
        }));
        let mut non_null = unsafe { NonNull::new_unchecked(raw_node) };

        match self.tail {
            Some(mut old_tail) => {
                unsafe {
                    old_tail.as_mut().next = Some(non_null);
                    non_null.as_mut().prev = Some(old_tail);
                    // Circular linkage: tail.next points to head, head.prev points to tail
                    if let Some(mut head) = self.head {
                        non_null.as_mut().next = Some(head);
                        head.as_mut().prev = Some(non_null);
                    }
                }
                self.tail = Some(non_null);
            }
            None => {
                // First element
                unsafe {
                    non_null.as_mut().next = Some(non_null);
                    non_null.as_mut().prev = Some(non_null);
                }
                self.head = Some(non_null);
                self.tail = Some(non_null);
            }
        }
        self.len += 1;
    }

    pub fn pop_head(&mut self) -> Option<T> {
        let head_ptr = self.head?;
        self.len -= 1;

        if self.len == 0 {
            self.head = None;
            self.tail = None;
            let boxed_node = unsafe { Box::from_raw(head_ptr.as_ptr()) };
            Some(boxed_node.value)
        } else {
            unsafe {
                let mut next_node = head_ptr.as_ref().next.unwrap();
                let mut tail_node = self.tail.unwrap();

                tail_node.as_mut().next = Some(next_node);
                next_node.as_mut().prev = Some(tail_node);

                self.head = Some(next_node);

                let boxed_node = Box::from_raw(head_ptr.as_ptr());
                Some(boxed_node.value)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for CircularDoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_head().is_some() {}
    }
}

// 4. MULTI-ARCHITECTURE REGISTERS & THREAD STATES

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Waiting,
    Blocked,
    BlockedWaiting,
    BlockedSuspended,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApcMode {
    SpecialKernelMode,
    NormalKernelMode,
    KernelMode,
    UserMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IrqlLevel {
    PassiveLevel = 0,
    ApcLevel = 1,
    DispatchLevel = 2,
    DpcLevel = 3,
    HighLevel = 4,
}

#[derive(Debug, Clone)]
pub struct IrqlState {
    pub current_level: IrqlLevel,
}

impl IrqlState {
    pub fn new() -> Self {
        Self {
            current_level: IrqlLevel::PassiveLevel,
        }
    }

    pub fn raise_irql(&mut self, new_level: IrqlLevel) -> Result<IrqlLevel, &'static str> {
        if new_level < self.current_level {
            return Err("Cannot raise IRQL to lower level");
        }
        let old = self.current_level;
        self.current_level = new_level;
        Ok(old)
    }

    pub fn lower_irql(&mut self, new_level: IrqlLevel) -> Result<IrqlLevel, &'static str> {
        if new_level > self.current_level {
            return Err("Cannot lower IRQL to higher level");
        }
        let old = self.current_level;
        self.current_level = new_level;
        Ok(old)
    }
}

impl Default for IrqlState {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulated hardware CPU registers across various architectures (x86, x64, ARM, CISC)
#[derive(Debug, Clone, Default)]
pub struct CpuContext {
    // x86/x64 architecture context
    pub rip: u64,
    pub rsp: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rflags: u64,

    // ARM architecture context
    pub pc: u64,
    pub sp: u64,
    pub r0: u64,
    pub r1: u64,
    pub cpsr: u64,
}

// 5. ASYNCHRONOUS PROCEDURE CALLS (APC)
// Inspired by Windows KAPC and Linux signal delivery models.

pub struct Apc {
    pub apc_id: u64,
    pub target_tid: u64,
    pub mode: ApcMode,
    pub priority: u8,
    pub param: u64,
}

pub struct ApcQueue {
    pub apcs: SinglyLinkedList<Apc>,
}

impl ApcQueue {
    pub const fn new() -> Self {
        Self {
            apcs: SinglyLinkedList::new(),
        }
    }

    pub fn queue_apc(&mut self, apc: Apc) {
        // Enqueue APC sorted by priority (higher priority first)
        let priority = apc.priority;
        let mut apc_opt = Some(apc);
        let mut temp = SinglyLinkedList::new();
        let mut placed = false;

        while let Some(current) = self.apcs.pop_front() {
            if !placed && priority >= current.priority {
                temp.push_front(apc_opt.take().unwrap());
                placed = true;
                temp.push_front(current);
            } else {
                temp.push_front(current);
            }
        }

        // Restore list from temp
        let mut final_apcs = SinglyLinkedList::new();
        while let Some(item) = temp.pop_front() {
            final_apcs.push_front(item);
        }

        if !placed {
            if let Some(item) = apc_opt {
                final_apcs.push_front(item);
            }
        }

        self.apcs = final_apcs;
    }

    pub fn deliver_next(&mut self) -> Option<Apc> {
        self.apcs.pop_front()
    }

    pub fn len(&self) -> usize {
        self.apcs.len()
    }
}

// 6. SYSTEM THREAD
// Inspired by Windows ETHREAD, Linux task_struct, and BSD thread structures.

pub struct SystemThread {
    pub tid: u64,
    pub parent_pid: u64,
    pub state: ThreadState,
    pub context: CpuContext,
    pub core_affinity: usize,
    pub apc_queue: ApcQueue,
    pub kernel_stack_base: u64,
    pub user_mode_suspended: bool,
    pub user_suspend_count: u32,
    pub irql: IrqlState,
}

impl SystemThread {
    pub fn new(tid: u64, parent_pid: u64, core_affinity: usize) -> Self {
        Self {
            tid,
            parent_pid,
            state: ThreadState::Ready,
            context: CpuContext::default(),
            core_affinity,
            apc_queue: ApcQueue::new(),
            kernel_stack_base: 0xFFFF_8000_0000_0000 | (tid << 12),
            user_mode_suspended: false,
            user_suspend_count: 0,
            irql: IrqlState::new(),
        }
    }

    pub fn suspend_user_mode(&mut self) -> u32 {
        self.user_suspend_count += 1;
        self.user_mode_suspended = true;
        // Enqueue user-mode suspension APC
        self.queue_apc(Apc {
            apc_id: 0x5555,
            target_tid: self.tid,
            mode: ApcMode::UserMode,
            priority: 255,
            param: 0x1, // User-mode suspend flag
        });
        self.user_suspend_count
    }

    pub fn resume_user_mode(&mut self) -> u32 {
        if self.user_suspend_count > 0 {
            self.user_suspend_count -= 1;
            if self.user_suspend_count == 0 {
                self.user_mode_suspended = false;
            }
        }
        self.user_suspend_count
    }

    pub fn is_suspended(&self) -> bool {
        self.user_mode_suspended
    }

    pub fn queue_apc(&mut self, apc: Apc) {
        self.apc_queue.queue_apc(apc);
    }

    pub fn dispatch_pending_apcs(&mut self) -> usize {
        let mut delivered = 0;
        while let Some(apc) = self.apc_queue.deliver_next() {
            // Emulate execution: transition CPU context based on APC mode/parameters
            match apc.mode {
                ApcMode::SpecialKernelMode => {
                    self.context.rip = 0xFFFFFFFF_0000_0100; // Mock special kernel APC
                    self.context.rax = apc.param;
                }
                ApcMode::NormalKernelMode | ApcMode::KernelMode => {
                    self.context.rip = 0xFFFFFFFF_0000_1000; // Mock kernel APC routine
                    self.context.rax = apc.param;
                }
                ApcMode::UserMode => {
                    self.context.rip = 0x00007FFF_0000_2000; // Mock user APC routine
                    self.context.rax = apc.param;
                }
            }
            delivered += 1;
        }
        delivered
    }
}

// 7. WORK ITEMS
// Inspired by Windows WORK_QUEUE_ITEM and Linux work_struct/workqueue model.

pub struct WorkItem {
    pub work_id: u64,
    pub executed: bool,
    pub execution_flags: u32,
    pub payload_data: u64,
}

impl WorkItem {
    pub const fn new(work_id: u64, payload: u64) -> Self {
        Self {
            work_id,
            executed: false,
            execution_flags: 0,
            payload_data: payload,
        }
    }

    pub fn execute(&mut self) {
        self.executed = true;
        self.execution_flags |= 0x1; // Mark active/completed flags
    }
}

// 8. UNIT TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singly_linked_list_operations() {
        let mut list = SinglyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.push_front(10);
        list.push_front(20);
        list.push_front(30);

        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());

        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_sequenced_slist_reclaiming() {
        let mut list = SequencedSinglyLinkedList::new();
        assert!(list.is_empty());

        let seq1 = list.push_front(100);
        let seq2 = list.push_front(200);

        assert_eq!(seq1, 1);
        assert_eq!(seq2, 2);
        assert_eq!(list.len(), 2);

        let pop1 = list.pop_front();
        assert_eq!(pop1, Some((200, 2)));

        let pop2 = list.pop_front();
        assert_eq!(pop2, Some((100, 1)));

        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_circular_doubly_linked_list_sentinel() {
        let mut list = CircularDoublyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.push_tail(1000);
        list.push_tail(2000);
        list.push_tail(3000);

        assert_eq!(list.len(), 3);

        assert_eq!(list.pop_head(), Some(1000));
        assert_eq!(list.pop_head(), Some(2000));
        assert_eq!(list.pop_head(), Some(3000));
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn test_system_thread_context_and_states() {
        let mut thread = SystemThread::new(42, 10, 2);
        assert_eq!(thread.tid, 42);
        assert_eq!(thread.parent_pid, 10);
        assert_eq!(thread.core_affinity, 2);
        assert_eq!(thread.state, ThreadState::Ready);

        thread.state = ThreadState::Running;
        assert_eq!(thread.state, ThreadState::Running);

        // Hardware registers configuration simulation
        thread.context.rip = 0x8000;
        thread.context.rsp = 0x7FFF;
        thread.context.pc = 0x9000;
        thread.context.sp = 0x8FFF;

        assert_eq!(thread.context.rip, 0x8000);
        assert_eq!(thread.context.rsp, 0x7FFF);
        assert_eq!(thread.context.pc, 0x9000);
        assert_eq!(thread.context.sp, 0x8FFF);
    }

    #[test]
    fn test_deferred_work_items_execution() {
        let mut item = WorkItem::new(101, 0xABCD);
        assert_eq!(item.work_id, 101);
        assert_eq!(item.payload_data, 0xABCD);
        assert!(!item.executed);

        item.execute();
        assert!(item.executed);
        assert_eq!(item.execution_flags, 0x1);
    }

    #[test]
    fn test_user_mode_thread_suspension() {
        let mut thread = SystemThread::new(88, 1, 0);
        assert!(!thread.is_suspended());

        thread.suspend_user_mode();
        assert!(thread.is_suspended());
        assert_eq!(thread.user_suspend_count, 1);

        thread.resume_user_mode();
        assert!(!thread.is_suspended());
        assert_eq!(thread.user_suspend_count, 0);
    }

    #[test]
    fn test_apc_special_kernel_delivery() {
        let mut thread = SystemThread::new(99, 1, 0);
        let special_apc = Apc {
            apc_id: 1,
            target_tid: 99,
            mode: ApcMode::SpecialKernelMode,
            priority: 200,
            param: 0xDEADBEEF,
        };

        thread.queue_apc(special_apc);
        let delivered = thread.dispatch_pending_apcs();
        assert_eq!(delivered, 1);
        assert_eq!(thread.context.rip, 0xFFFFFFFF_0000_0100);
        assert_eq!(thread.context.rax, 0xDEADBEEF);
    }

    #[test]
    fn test_apc_queue_delivery_and_execution() {
        let mut thread = SystemThread::new(9, 1, 0);

        // Create APCs with different priorities
        let apc_low = Apc {
            apc_id: 1,
            target_tid: 9,
            mode: ApcMode::UserMode,
            priority: 5,
            param: 100,
        };

        let apc_high = Apc {
            apc_id: 2,
            target_tid: 9,
            mode: ApcMode::KernelMode,
            priority: 10,
            param: 200,
        };

        thread.queue_apc(apc_low);
        thread.queue_apc(apc_high);

        // High priority (10) should be delivered first, then low priority (5)
        assert_eq!(thread.apc_queue.len(), 2);

        // Dispatch APCs
        let executed = thread.dispatch_pending_apcs();
        assert_eq!(executed, 2);
        assert_eq!(thread.apc_queue.len(), 0);

        // The last dispatched APC was low (param: 100, UserMode)
        assert_eq!(thread.context.rip, 0x00007FFF_0000_2000);
        assert_eq!(thread.context.rax, 100);
    }
}
// 3. Next-Generation Advanced Algorithms (SovereignAlgorithms Blueprint)

const MAX_SCHEDULER_TASKS: usize = 16;
const MAX_LEDGER_BLOCKS: usize = 8;

/// Real-Time Task Descriptor for Earliest Deadline First (EDF)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdfTask {
    pub tid: u32,
    pub absolute_deadline: u64,   // Cycle deadline limit
    pub remaining_execution: u64, // Cycles required to complete
    pub is_active: bool,
}

/// Task Descriptor for Lottery Scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LotteryTask {
    pub tid: u32,
    pub tickets: u32, // Tickets allocated to this task (weight)
    pub is_active: bool,
}

/// Ledger Block securing system process events (Consensus Process Logging)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArchitectureClass {
    X86_64,
    AArch64,
    RiscV64,
}

pub struct AuditBlock {
    pub block_id: u32,
    pub timestamp: u64,
    pub event_type: u32, // 1: Spawn, 2: Terminate, 3: Privilege Escalate
    pub pid: u32,
    pub actor_hash: u32,
    pub prev_block_hash: u32,
    pub current_hash: u32, // Checksum linking block metadata + prev_block_hash
}

/// Linear Congruential Generator (LCG) for sub-nanosecond pseudo-random draws
pub struct LcgRandom {
    pub state: u64,
}

impl LcgRandom {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Generates next pseudo-random u32
    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state >> 32) as u32
    }
}

/// Global Shard Manager for Advanced Scheduling and Audit Ledgers
pub struct AdvancedAlgorithmsManager {
    pub edf_queue: RefCell<[Option<EdfTask>; MAX_SCHEDULER_TASKS]>,
    pub lottery_queue: RefCell<[Option<LotteryTask>; MAX_SCHEDULER_TASKS]>,
    pub audit_ledger: RefCell<[Option<AuditBlock>; MAX_LEDGER_BLOCKS]>,
    pub random_engine: RefCell<LcgRandom>,
    pub next_block_id: Cell<u32>,
}

impl AdvancedAlgorithmsManager {
    pub fn new(seed: u64) -> Self {
        const EMPTY_EDF: Option<EdfTask> = None;
        const EMPTY_LOTTERY: Option<LotteryTask> = None;
        const EMPTY_BLOCK: Option<AuditBlock> = None;

        Self {
            edf_queue: RefCell::new([EMPTY_EDF; MAX_SCHEDULER_TASKS]),
            lottery_queue: RefCell::new([EMPTY_LOTTERY; MAX_SCHEDULER_TASKS]),
            audit_ledger: RefCell::new([EMPTY_BLOCK; MAX_LEDGER_BLOCKS]),
            random_engine: RefCell::new(LcgRandom::new(seed)),
            next_block_id: Cell::new(1),
        }
    }

    /// Basic FNV-1a hash algorithm to create block hashes (Consensus Cryptography)
    pub fn calculate_block_hash(block: &AuditBlock) -> u32 {
        let mut hash: u32 = 2166136261;

        let fields = [
            block.block_id,
            (block.timestamp & 0xFFFFFFFF) as u32,
            (block.timestamp >> 32) as u32,
            block.event_type,
            block.pid,
            block.actor_hash,
            block.prev_block_hash,
        ];

        for &val in &fields {
            hash ^= val;
            hash = hash.wrapping_mul(16777619);
        }

        hash
    }

    // 1. EARLIEST DEADLINE FIRST REAL-TIME SCHEDULER
    pub fn add_edf_task(&self, task: EdfTask) -> Result<(), &'static str> {
        let mut queue = self.edf_queue.borrow_mut();
        for slot in (*queue).iter_mut() {
            if slot.is_none() {
                *slot = Some(task);
                return Ok(());
            }
        }
        Err("EDF Scheduler: Queue full")
    }

    /// Selects the next task with the closest absolute deadline (EDF Core Algorithm)
    pub fn schedule_edf(&self) -> Option<EdfTask> {
        let queue = self.edf_queue.borrow();
        let mut best_idx: Option<usize> = None;

        for (idx, slot) in queue.iter().enumerate() {
            if let Some(ref task) = slot {
                if task.is_active && task.remaining_execution > 0 {
                    match best_idx {
                        None => best_idx = Some(idx),
                        Some(best) => {
                            if task.absolute_deadline
                                < queue[best].as_ref().unwrap().absolute_deadline
                            {
                                best_idx = Some(idx);
                            }
                        }
                    }
                }
            }
        }

        best_idx.map(|idx| queue[idx].unwrap())
    }

    // 2. PROBABILISTIC LOTTERY SCHEDULER
    pub fn add_lottery_task(&self, task: LotteryTask) -> Result<(), &'static str> {
        let mut queue = self.lottery_queue.borrow_mut();
        for slot in (*queue).iter_mut() {
            if slot.is_none() {
                *slot = Some(task);
                return Ok(());
            }
        }
        Err("Lottery Scheduler: Queue full")
    }

    /// Draws a random winning ticket and returns the winning task (Lottery Core Algorithm)
    pub fn schedule_lottery(&self) -> Option<LotteryTask> {
        let queue = self.lottery_queue.borrow();

        // 1. Sum up total outstanding active tickets in queue
        let mut total_tickets = 0;
        for slot in queue.iter() {
            if let Some(ref task) = slot {
                if task.is_active {
                    total_tickets += task.tickets;
                }
            }
        }

        if total_tickets == 0 {
            return None;
        }

        // 2. Draw winning ticket
        let mut rng = self.random_engine.borrow_mut();
        let winning_ticket = (rng.next_u32() % total_tickets) + 1;

        // 3. Find the task that holds the winning ticket range
        let mut ticket_counter = 0;
        for slot in queue.iter() {
            if let Some(ref task) = slot {
                if task.is_active {
                    ticket_counter += task.tickets;
                    if ticket_counter >= winning_ticket {
                        return Some(*task);
                    }
                }
            }
        }

        None
    }

    // 3. CONSENSUS-BASED PROCESS AUDIT LEDGER

    /// Append a process lifecycle audit log into our secure chained cryptographic block ledger
    pub fn audit_process_event(
        &self,
        timestamp: u64,
        event_type: u32,
        pid: u32,
        actor_hash: u32,
    ) -> Result<(), &'static str> {
        let mut ledger = self.audit_ledger.borrow_mut();

        // Find previous block hash
        let block_id = self.next_block_id.get();
        let prev_hash = if block_id > 1 {
            let mut last_hash = 0;
            for slot in ledger.iter() {
                if let Some(ref block) = slot {
                    if block.block_id == block_id - 1 {
                        last_hash = block.current_hash;
                        break;
                    }
                }
            }
            last_hash
        } else {
            0 // Genesis block hash offset
        };

        let mut block = AuditBlock {
            block_id,
            timestamp,
            event_type,
            pid,
            actor_hash,
            prev_block_hash: prev_hash,
            current_hash: 0,
        };

        block.current_hash = Self::calculate_block_hash(&block);

        // Store block in circular ledger array
        let index = (block_id as usize - 1) % MAX_LEDGER_BLOCKS;
        ledger[index] = Some(block);

        self.next_block_id.set(block_id + 1);

        Ok(())
    }

    /// Validates the cryptographic chain of the ledger, blocking immediately on tamper detection (Self-Audit)
    pub fn verify_ledger_integrity(&self) -> bool {
        let ledger = self.audit_ledger.borrow();

        for i in 0..MAX_LEDGER_BLOCKS {
            if let Some(ref block) = ledger[i] {
                // Verify block hash
                let computed_hash = Self::calculate_block_hash(block);
                if block.current_hash != computed_hash {
                    println!(
                        "Audit Ledger: Corruption detected on Block {} - Signature hash mismatch!",
                        block.block_id
                    );
                    return false; // Chain tampered!
                }

                // Verify previous link (if previous block is still in buffer)
                if block.block_id > 1 {
                    let prev_index = (block.block_id as usize - 2) % MAX_LEDGER_BLOCKS;
                    if let Some(ref prev_block) = ledger[prev_index] {
                        if prev_block.block_id == block.block_id - 1
                            && block.prev_block_hash != prev_block.current_hash
                        {
                            println!(
                                "Audit Ledger: Chain broken between Block {} and {}!",
                                prev_block.block_id, block.block_id
                            );
                            return false; // Break detected!
                        }
                    }
                }
            }
        }

        true
    }
}
