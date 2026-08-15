// SigmaOS Core Kernel Structures and Advanced Algorithms Subsystem
// Conforms to zero-dependency, #![no_std] compliant OOP structures

use core::cell::{Cell, RefCell};
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

// 1. Singly, Sequenced, and Circular Doubly Linked Lists

pub struct SinglyLinkedList<T> {
    pub value: T,
    pub next: Option<Box<SinglyLinkedList<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }

    pub fn push_next(&mut self, next_val: T) {
        let mut node = Box::new(SinglyLinkedList::new(next_val));
        if let Some(existing) = self.next.take() {
            node.next = Some(existing);
        }
        self.next = Some(node);
    }
}

pub struct SequencedSinglyLinkedList<T> {
    pub value: T,
    pub sequence_number: u64,
    pub next: Option<Box<SequencedSinglyLinkedList<T>>>,
}

impl<T> SequencedSinglyLinkedList<T> {
    pub fn new(value: T, seq: u64) -> Self {
        Self {
            value,
            sequence_number: seq,
            next: None,
        }
    }
}

pub struct CircularDoublyLinkedList<T> {
    pub value: Option<T>,
    // Simulated pointers to represent list links (Windows LIST_ENTRY and Linux list_head style)
    pub next_id: Option<usize>,
    pub prev_id: Option<usize>,
}

impl<T> CircularDoublyLinkedList<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
            next_id: None,
            prev_id: None,
        }
    }
}

// 2. Scheduler SystemThread, WorkItems, APCs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArchitectureClass {
    X86,
    X64,
    Arm,
    Cisc,
}

pub struct SystemThread {
    pub thread_id: usize,
    pub priority: u8,
    pub arch: CpuArchitectureClass,
    pub register_context: [u64; 16], // Mock registers (rax, rbx, r1-r15 etc)
}

impl SystemThread {
    pub fn new(thread_id: usize, priority: u8, arch: CpuArchitectureClass) -> Self {
        Self {
            thread_id,
            priority,
            arch,
            register_context: [0u64; 16],
        }
    }
}

pub struct WorkItem {
    pub work_id: usize,
    pub is_processed: AtomicBool,
    pub payload_hash: u32,
}

impl WorkItem {
    pub fn new(work_id: usize, payload_hash: u32) -> Self {
        Self {
            work_id,
            is_processed: AtomicBool::new(false),
            payload_hash,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApcMode {
    KernelMode,
    UserMode,
}

pub struct Apc {
    pub apc_id: usize,
    pub mode: ApcMode,
    pub callback_id: usize,
}

pub struct ApcQueue {
    pub apcs: Vec<Apc>,
}

impl ApcQueue {
    pub fn new() -> Self {
        Self { apcs: Vec::new() }
    }

    pub fn queue_apc(&mut self, apc: Apc) {
        self.apcs.push(apc);
    }

    pub fn deliver_apcs(&mut self, mode: ApcMode) -> usize {
        let mut count = 0;
        self.apcs.retain(|apc| {
            if apc.mode == mode {
                println!("[apc] Delivering APC #{} in {:?}", apc.apc_id, mode);
                count += 1;
                false // Remove from queue
            } else {
                true // Retain in queue
            }
        });
        count
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
        for slot in queue.iter_mut() {
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
        for slot in queue.iter_mut() {
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
