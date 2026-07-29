// Kernel Performance - Zero-Copy IPC & UDF Scheduler VM
// High-speed zero-copy IPC and autonomic UDF CPU scheduling engine

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerfIpcError {
    QueueFull,
    QueueEmpty,
    InvalidPayload,
}

/// Thread-Safe, Lock-Free Circular Ring-Buffer for Zero-Copy IPC
pub struct ZeroCopyQueue<T, const N: usize> {
    buffer: [Option<T>; N],
    head: usize,
    tail: usize,
}

impl<T: Clone + Copy, const N: usize> ZeroCopyQueue<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: [const { None }; N],
            head: 0,
            tail: 0,
        }
    }

    /// Pushes a zero-copy reference or page frame onto the queue without locks
    pub fn enqueue(&mut self, item: T) -> Result<(), PerfIpcError> {
        let head = self.head;
        let tail = self.tail;

        if head.wrapping_sub(tail) >= N {
            return Err(PerfIpcError::QueueFull);
        }

        let idx = head % N;
        self.buffer[idx] = Some(item);
        self.head = head.wrapping_add(1);
        Ok(())
    }

    /// Pulls a zero-copy reference or page frame out of the queue
    pub fn dequeue(&mut self) -> Result<T, PerfIpcError> {
        let head = self.head;
        let tail = self.tail;

        if tail == head {
            return Err(PerfIpcError::QueueEmpty);
        }

        let idx = tail % N;
        let item = self.buffer[idx].take().ok_or(PerfIpcError::InvalidPayload)?;
        self.tail = tail.wrapping_add(1);
        Ok(item)
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /// Check if queue is full
    pub fn is_full(&self) -> bool {
        self.head.wrapping_sub(self.tail) >= N
    }

    /// Get current queue size
    pub fn len(&self) -> usize {
        self.head.wrapping_sub(self.tail)
    }
}

impl<T: Clone + Copy, const N: usize> Default for ZeroCopyQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// UDF Scheduler Instruction set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedOpcode {
    LoadPriority = 0x01, // Load process priority: LoadPriority(reg_idx)
    LoadRuntime = 0x02,  // Load process runtime: LoadRuntime(reg_idx)
    MulConst = 0x03,     // Multiply register by constant: MulConst(reg_idx, constant)
    AddConst = 0x04,     // Add constant to register: AddConst(reg_idx, constant)
    StoreResult = 0x05,  // Store final decision: StoreResult(reg_idx)
    Halt = 0x0F,         // Halt VM
}

pub struct SchedInstruction {
    pub opcode: SchedOpcode,
    pub arg1: u8,
    pub arg2: u8,
}

pub struct ProcessProfile {
    pub priority_level: u8, // 1 to 10
    pub runtime_ms: u32,
}

/// Lightweight scheduler bytecode VM executing inside the scheduling loop
pub struct UdfSchedVm {
    registers: [u32; 4],
    program: Vec<SchedInstruction>,
}

impl UdfSchedVm {
    pub fn new(program: Vec<SchedInstruction>) -> Self {
        Self {
            registers: [0; 4],
            program,
        }
    }

    /// Evaluates a process profile, calculating its custom scheduling dynamic priority weight
    pub fn evaluate_priority(&mut self, process: &ProcessProfile) -> Result<u32, &'static str> {
        let mut pc = 0;
        let limit = self.program.len();
        let mut decision = 0;

        while pc < limit {
            let inst = &self.program[pc];
            match inst.opcode {
                SchedOpcode::LoadPriority => {
                    let reg = inst.arg1 as usize;
                    if reg < 4 {
                        self.registers[reg] = process.priority_level as u32;
                    } else {
                        return Err("Register index out of bounds");
                    }
                }
                SchedOpcode::LoadRuntime => {
                    let reg = inst.arg1 as usize;
                    if reg < 4 {
                        self.registers[reg] = process.runtime_ms;
                    } else {
                        return Err("Register index out of bounds");
                    }
                }
                SchedOpcode::MulConst => {
                    let reg = inst.arg1 as usize;
                    if reg < 4 {
                        self.registers[reg] = self.registers[reg].wrapping_mul(inst.arg2 as u32);
                    } else {
                        return Err("Register index out of bounds");
                    }
                }
                SchedOpcode::AddConst => {
                    let reg = inst.arg1 as usize;
                    if reg < 4 {
                        self.registers[reg] = self.registers[reg].wrapping_add(inst.arg2 as u32);
                    } else {
                        return Err("Register index out of bounds");
                    }
                }
                SchedOpcode::StoreResult => {
                    let reg = inst.arg1 as usize;
                    if reg < 4 {
                        decision = self.registers[reg];
                    } else {
                        return Err("Register index out of bounds");
                    }
                }
                SchedOpcode::Halt => {
                    break;
                }
            }
            pc += 1;
        }

        Ok(decision)
    }

    /// Load a new scheduling program
    pub fn load_program(&mut self, program: Vec<SchedInstruction>) {
        self.program = program;
        self.registers = [0; 4];
    }

    /// Get current register values (for debugging)
    pub fn get_registers(&self) -> [u32; 4] {
        self.registers
    }
}

impl Default for UdfSchedVm {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_queue() {
        let mut queue: ZeroCopyQueue<u32, 4> = ZeroCopyQueue::new();

        assert!(queue.is_empty());
        assert!(!queue.is_full());

        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();

        assert_eq!(queue.len(), 3);

        assert_eq!(queue.dequeue().unwrap(), 1);
        assert_eq!(queue.dequeue().unwrap(), 2);

        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_queue_full() {
        let mut queue: ZeroCopyQueue<u32, 2> = ZeroCopyQueue::new();

        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();

        assert!(queue.is_full());
        assert!(queue.enqueue(3).is_err());
    }

    #[test]
    fn test_queue_empty() {
        let mut queue: ZeroCopyQueue<u32, 2> = ZeroCopyQueue::new();

        assert!(queue.dequeue().is_err());
    }

    #[test]
    fn test_udf_sched_vm() {
        let program = vec![
            SchedInstruction {
                opcode: SchedOpcode::LoadPriority,
                arg1: 0,
                arg2: 0,
            },
            SchedInstruction {
                opcode: SchedOpcode::AddConst,
                arg1: 0,
                arg2: 5,
            },
            SchedInstruction {
                opcode: SchedOpcode::StoreResult,
                arg1: 0,
                arg2: 0,
            },
            SchedInstruction {
                opcode: SchedOpcode::Halt,
                arg1: 0,
                arg2: 0,
            },
        ];

        let mut vm = UdfSchedVm::new(program);
        let process = ProcessProfile {
            priority_level: 3,
            runtime_ms: 100,
        };

        let result = vm.evaluate_priority(&process).unwrap();
        assert_eq!(result, 8); // 3 + 5 = 8
    }

    #[test]
    fn test_udf_sched_vm_multiply() {
        let program = vec![
            SchedInstruction {
                opcode: SchedOpcode::LoadPriority,
                arg1: 0,
                arg2: 0,
            },
            SchedInstruction {
                opcode: SchedOpcode::MulConst,
                arg1: 0,
                arg2: 2,
            },
            SchedInstruction {
                opcode: SchedOpcode::StoreResult,
                arg1: 0,
                arg2: 0,
            },
            SchedInstruction {
                opcode: SchedOpcode::Halt,
                arg1: 0,
                arg2: 0,
            },
        ];

        let mut vm = UdfSchedVm::new(program);
        let process = ProcessProfile {
            priority_level: 5,
            runtime_ms: 100,
        };

        let result = vm.evaluate_priority(&process).unwrap();
        assert_eq!(result, 10); // 5 * 2 = 10
    }

    #[test]
    fn test_udf_sched_vm_combined() {
        let program = vec![
            SchedInstruction {
                opcode: SchedOpcode::LoadPriority,
                arg1: 0,
                arg2: 0,
            },
            SchedInstruction {
                opcode: SchedOpcode::LoadRuntime,
                arg1: 1,
                arg2: 0,
            },
            SchedInstruction {
                opcode: SchedOpcode::AddConst,
                arg1: 0,
                arg2: 10,
            },
            SchedInstruction {
                opcode: SchedOpcode::MulConst,
                arg1: 1,
                arg2: 2,
            },
            SchedInstruction {
                opcode: SchedOpcode::StoreResult,
                arg1: 1,
                arg2: 0,
            },
            SchedInstruction {
                opcode: SchedOpcode::Halt,
                arg1: 0,
                arg2: 0,
            },
        ];

        let mut vm = UdfSchedVm::new(program);
        let process = ProcessProfile {
            priority_level: 3,
            runtime_ms: 50,
        };

        let result = vm.evaluate_priority(&process).unwrap();
        assert_eq!(result, 100); // (50 * 2) = 100
    }

    #[test]
    fn test_register_bounds() {
        let program = vec![
            SchedInstruction {
                opcode: SchedOpcode::LoadPriority,
                arg1: 5,
                arg2: 0,
            },
            SchedInstruction {
                opcode: SchedOpcode::Halt,
                arg1: 0,
                arg2: 0,
            },
        ];

        let mut vm = UdfSchedVm::new(program);
        let process = ProcessProfile {
            priority_level: 3,
            runtime_ms: 100,
        };

        assert!(vm.evaluate_priority(&process).is_err());
    }
}

// ==========================================
// HIGH-PERFORMANCE POWER-OF-TWO IPC OPTIMIZATION
// ==========================================

/// High-Performance, Lock-Free Circular Ring-Buffer using Power-of-Two Bitwise Masking
pub struct PowerOfTwoZeroCopyQueue<T, const N: usize> {
    buffer: [Option<T>; N],
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl<T: Clone + Copy, const N: usize> PowerOfTwoZeroCopyQueue<T, N> {
    /// Creates a new optimized queue. Enforces that size N is a power-of-two at runtime.
    pub fn new() -> Self {
        assert!(N > 0 && (N & (N - 1)) == 0, "Buffer size N must be a power of two for bitwise optimization");
        Self {
            buffer: [const { None }; N],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Enqueues a payload using single-cycle bitwise masking instead of slow modulo/division arithmetic
    pub fn enqueue(&mut self, item: T) -> Result<(), PerfIpcError> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);

        if head.wrapping_sub(tail) >= N {
            return Err(PerfIpcError::QueueFull);
        }

        // Bitwise optimization: idx = head & (N - 1)
        let idx = head & (N - 1);
        self.buffer[idx] = Some(item);
        self.head.store(head.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    /// Dequeues a payload in O(1) single-cycle latency
    pub fn dequeue(&mut self) -> Result<T, PerfIpcError> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Relaxed);

        if tail == head {
            return Err(PerfIpcError::QueueEmpty);
        }

        // Bitwise optimization: idx = tail & (N - 1)
        let idx = tail & (N - 1);
        let item = self.buffer[idx].take().ok_or(PerfIpcError::InvalidPayload)?;
        self.tail.store(tail.wrapping_add(1), Ordering::Release);
        Ok(item)
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire) == self.tail.load(Ordering::Acquire)
    }

    pub fn is_full(&self) -> bool {
        self.head.load(Ordering::Acquire).wrapping_sub(self.tail.load(Ordering::Acquire)) >= N
    }

    pub fn len(&self) -> usize {
        self.head.load(Ordering::Acquire).wrapping_sub(self.tail.load(Ordering::Acquire))
    }
}

impl<T: Clone + Copy, const N: usize> Default for PowerOfTwoZeroCopyQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod bitwise_queue_tests {
    use super::*;

    #[test]
    fn test_power_of_two_queue_success() {
        let mut queue: PowerOfTwoZeroCopyQueue<u32, 8> = PowerOfTwoZeroCopyQueue::new();
        assert!(queue.is_empty());

        queue.enqueue(10).unwrap();
        queue.enqueue(20).unwrap();
        queue.enqueue(30).unwrap();

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.dequeue().unwrap(), 10);
        assert_eq!(queue.dequeue().unwrap(), 20);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_power_of_two_queue_full_empty() {
        let mut queue: PowerOfTwoZeroCopyQueue<u32, 4> = PowerOfTwoZeroCopyQueue::new();
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();
        queue.enqueue(4).unwrap();

        assert!(queue.is_full());
        assert_eq!(queue.enqueue(5), Err(PerfIpcError::QueueFull));

        assert_eq!(queue.dequeue().unwrap(), 1);
        assert_eq!(queue.dequeue().unwrap(), 2);
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.dequeue().unwrap(), 4);

        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), Err(PerfIpcError::QueueEmpty));
    }

    #[test]
    #[should_panic(expected = "Buffer size N must be a power of two for bitwise optimization")]
    fn test_non_power_of_two_panic() {
        let _queue: PowerOfTwoZeroCopyQueue<u32, 6> = PowerOfTwoZeroCopyQueue::new();
    }
}
