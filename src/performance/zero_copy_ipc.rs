//! Lock-Free Zero-Copy IPC Queue Subsystem
//! Implements high-speed page-passing circular ring buffers with atomic fences for sub-microsecond latency.

pub use crate::kernel::performance::{
    CpuInstructionExtension, IPCError, IpcError, ProcessProfile, SchedInstruction, SchedOpcode,
    SimdOptimizer, SovereignSimdOptimizer, UdfSchedVm, VmPerformanceMetrics, ZeroCopyMetrics,
    ZeroCopyQueue,
};

pub const QUEUE_SIZE: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_ipc_flow() {
        let mut queue: ZeroCopyQueue<[u8; 4], 4> = ZeroCopyQueue::new();

        let page_payload = [0xDE, 0xAD, 0xBE, 0xEF];
        queue.enqueue(page_payload).unwrap();

        let retrieved_payload = queue.dequeue().unwrap();
        assert_eq!(retrieved_payload, page_payload);
    }

    #[test]
    fn test_zero_copy_queue_bounds() {
        let mut queue: ZeroCopyQueue<u32, 2> = ZeroCopyQueue::new();
        queue.enqueue(10).unwrap();
        queue.enqueue(20).unwrap();

        // Queue is now full
        assert_eq!(queue.enqueue(30), Err(IpcError::QueueFull));

        assert_eq!(queue.dequeue().unwrap(), 10);
        assert_eq!(queue.dequeue().unwrap(), 20);

        // Queue is now empty
        assert_eq!(queue.dequeue(), Err(IpcError::QueueEmpty));
    }

    #[test]
    fn test_udf_sched_vm_execution() {
        let program = vec![
            SchedInstruction {
                opcode: SchedOpcode::LoadPriority,
                arg1: 0,
                arg2: 0,
            },
            SchedInstruction {
                opcode: SchedOpcode::MulConst,
                arg1: 0,
                arg2: 3,
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
            priority_level: 4,
            runtime_ms: 10,
        };

        let result = vm.evaluate_priority(&process).unwrap();
        assert_eq!(result, 12);
    }
}
