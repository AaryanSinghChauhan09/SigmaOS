// SigmaOS Competitor OS Innovations & Kernel Architecture Suite
// Clean-room Rust implementations inspired by Linux (zswap, NUMA, taskset, EDF, POSIX mq) and microkernel design principles.
// Zero-dependency, #![no_std] compliant native Rust implementation.

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// =========================================================================
// 1. ZSWAP TRANSPARENT PAGE COMPRESSION ENGINE (INFLIGHT MEMORY POOL)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompressedPagePoolEntry {
    pub page_id: u64,
    pub original_size_bytes: usize,
    pub compressed_size_bytes: usize,
    pub compressed_data: Vec<u8>,
}

pub struct ZswapTransparentPageCompressionEngine {
    pub compressed_pool: BTreeMap<u64, CompressedPagePoolEntry>,
    pub max_pool_bytes: usize,
    pub current_pool_bytes: usize,
}

impl ZswapTransparentPageCompressionEngine {
    pub fn new(max_pool_bytes: usize) -> Self {
        Self {
            compressed_pool: BTreeMap::new(),
            max_pool_bytes,
            current_pool_bytes: 0,
        }
    }

    /// Compress page using Run-Length Encoding / LZ-style compression simulation
    pub fn compress_and_store_page(&mut self, page_id: u64, page_data: &[u8]) -> Result<usize, &'static str> {
        let original_size = page_data.len();
        if original_size == 0 {
            return Err("Cannot compress empty page");
        }

        // Escaped run-length encoding (RLE) with marker 0xFE
        let mut compressed = Vec::new();
        let mut i = 0;
        while i < page_data.len() {
            let byte = page_data[i];
            let mut run = 1;
            while i + run < page_data.len() && page_data[i + run] == byte && run < 255 {
                run += 1;
            }
            if byte == 0xFE {
                compressed.push(0xFE);
                compressed.push(0xFE);
                compressed.push(run as u8);
                i += run;
            } else if run > 3 {
                compressed.push(0xFE); // Marker byte
                compressed.push(byte);
                compressed.push(run as u8);
                i += run;
            } else {
                compressed.push(byte);
                i += 1;
            }
        }

        let compressed_size = compressed.len();
        if self.current_pool_bytes + compressed_size > self.max_pool_bytes {
            return Err("Zswap pool capacity exceeded; page rejected to disk swap");
        }

        self.current_pool_bytes += compressed_size;
        self.compressed_pool.insert(
            page_id,
            CompressedPagePoolEntry {
                page_id,
                original_size_bytes: original_size,
                compressed_size_bytes: compressed_size,
                compressed_data: compressed,
            },
        );

        Ok(compressed_size)
    }

    /// Decompress stored page back to original buffer
    pub fn decompress_page(&mut self, page_id: u64) -> Result<Vec<u8>, &'static str> {
        let entry = self
            .compressed_pool
            .remove(&page_id)
            .ok_or("Page ID not found in zswap pool")?;

        self.current_pool_bytes -= entry.compressed_size_bytes;

        let mut decompressed = Vec::with_capacity(entry.original_size_bytes);
        let data = &entry.compressed_data;
        let mut i = 0;

        while i < data.len() {
            if data[i] == 0xFE && i + 2 < data.len() {
                let byte = data[i + 1];
                let count = data[i + 2] as usize;
                decompressed.extend(core::iter::repeat(byte).take(count));
                i += 3;
            } else {
                decompressed.push(data[i]);
                i += 1;
            }
        }

        Ok(decompressed)
    }
}

// =========================================================================
// 2. NUMA MULTI-SOCKET MEMORY ALLOCATOR & NODE AFFINITY
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumaNodeMemoryInfo {
    pub node_id: u32,
    pub total_memory_mb: usize,
    pub allocated_memory_mb: usize,
    pub latency_cost: u32,
}

pub struct NumaMultiSocketMemoryEngine {
    pub numa_nodes: BTreeMap<u32, NumaNodeMemoryInfo>,
}

impl NumaMultiSocketMemoryEngine {
    pub fn new() -> Self {
        Self {
            numa_nodes: BTreeMap::new(),
        }
    }

    pub fn register_numa_node(&mut self, node_id: u32, total_mb: usize, latency_cost: u32) {
        self.numa_nodes.insert(
            node_id,
            NumaNodeMemoryInfo {
                node_id,
                total_memory_mb: total_mb,
                allocated_memory_mb: 0,
                latency_cost,
            },
        );
    }

    /// Allocates memory on the preferred local NUMA node or falls back to lowest-latency node
    pub fn allocate_numa_memory(&mut self, preferred_node: u32, size_mb: usize) -> Result<u32, &'static str> {
        if let Some(node) = self.numa_nodes.get_mut(&preferred_node) {
            if node.allocated_memory_mb + size_mb <= node.total_memory_mb {
                node.allocated_memory_mb += size_mb;
                return Ok(preferred_node);
            }
        }

        // Fallback: find node with lowest latency and available capacity
        let mut best_fallback: Option<u32> = None;
        let mut lowest_latency = u32::MAX;

        for (node_id, node) in self.numa_nodes.iter() {
            if node.allocated_memory_mb + size_mb <= node.total_memory_mb {
                if node.latency_cost < lowest_latency {
                    lowest_latency = node.latency_cost;
                    best_fallback = Some(*node_id);
                }
            }
        }

        if let Some(fallback_id) = best_fallback {
            if let Some(node) = self.numa_nodes.get_mut(&fallback_id) {
                node.allocated_memory_mb += size_mb;
                return Ok(fallback_id);
            }
        }

        Err("Out of memory across all NUMA nodes")
    }
}

impl Default for NumaMultiSocketMemoryEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. TASKSET CPU PINNING & CORE AFFINITY SCHEDULER
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessCpuAffinity {
    pub pid: u32,
    pub cpu_affinity_mask: u64, // Bitmask of pinned CPU cores (e.g. 0b0011 -> CPU 0 and 1)
    pub assigned_cpu: u32,
}

pub struct TasksetCpuPinningSchedulerEngine {
    pub available_cpus_count: u32,
    pub process_affinities: BTreeMap<u32, ProcessCpuAffinity>,
}

impl TasksetCpuPinningSchedulerEngine {
    pub fn new(cpu_count: u32) -> Self {
        Self {
            available_cpus_count: cpu_count,
            process_affinities: BTreeMap::new(),
        }
    }

    pub fn set_process_affinity(&mut self, pid: u32, affinity_mask: u64) -> bool {
        if affinity_mask == 0 {
            return false;
        }

        // Find first available CPU core matching the mask within system CPU count
        let mut assigned = None;
        for cpu in 0..self.available_cpus_count {
            if (affinity_mask & (1u64 << cpu)) != 0 {
                assigned = Some(cpu);
                break;
            }
        }

        let assigned_cpu = match assigned {
            Some(cpu) => cpu,
            None => return false, // No CPU core within available_cpus_count matches the mask
        };

        self.process_affinities.insert(
            pid,
            ProcessCpuAffinity {
                pid,
                cpu_affinity_mask: affinity_mask,
                assigned_cpu,
            },
        );

        true
    }

    pub fn get_assigned_cpu(&self, pid: u32) -> Option<u32> {
        self.process_affinities.get(&pid).map(|p| p.assigned_cpu)
    }
}

// =========================================================================
// 4. EDF (EARLIEST DEADLINE FIRST) REALTIME SCHEDULER
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdfRealtimeTask {
    pub task_id: u32,
    pub deadline_timestamp: u64,
    pub execution_cost_ms: u32,
    pub priority: u32,
}

pub struct EdfDeadlineInheritanceSchedulerEngine {
    pub ready_queue: Vec<EdfRealtimeTask>,
}

impl EdfDeadlineInheritanceSchedulerEngine {
    pub fn new() -> Self {
        Self {
            ready_queue: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task_id: u32, deadline: u64, cost_ms: u32, priority: u32) {
        self.ready_queue.push(EdfRealtimeTask {
            task_id,
            deadline_timestamp: deadline,
            execution_cost_ms: cost_ms,
            priority,
        });

        // Sort by earliest deadline first (EDF scheduling)
        self.ready_queue
            .sort_by(|a, b| a.deadline_timestamp.cmp(&b.deadline_timestamp));
    }

    pub fn pick_next_task(&mut self) -> Option<EdfRealtimeTask> {
        if self.ready_queue.is_empty() {
            None
        } else {
            Some(self.ready_queue.remove(0))
        }
    }
}

impl Default for EdfDeadlineInheritanceSchedulerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. POSIX MESSAGE QUEUE ENGINE (mq_open, mq_send, mq_receive)
// =========================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PosixMessage {
    pub priority: u32,
    pub payload: Vec<u8>,
}

pub struct PosixMsgQueueEngine {
    pub queue_name: String,
    pub max_messages: usize,
    pub messages: Vec<PosixMessage>,
}

impl PosixMsgQueueEngine {
    pub fn new(queue_name: &str, max_messages: usize) -> Self {
        Self {
            queue_name: queue_name.to_string(),
            max_messages,
            messages: Vec::new(),
        }
    }

    pub fn mq_send(&mut self, payload: &[u8], priority: u32) -> Result<(), &'static str> {
        if self.messages.len() >= self.max_messages {
            return Err("POSIX message queue full");
        }

        self.messages.push(PosixMessage {
            priority,
            payload: payload.to_vec(),
        });

        // Priority queue ordering (highest priority message first)
        self.messages.sort_by(|a, b| b.priority.cmp(&a.priority));
        Ok(())
    }

    pub fn mq_receive(&mut self) -> Option<PosixMessage> {
        if self.messages.is_empty() {
            None
        } else {
            Some(self.messages.remove(0))
        }
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zswap_compression_engine() {
        let mut zswap = ZswapTransparentPageCompressionEngine::new(1024 * 1024);
        let page_data = vec![0x00; 4096];

        let compressed_len = zswap.compress_and_store_page(101, &page_data).unwrap();
        assert!(compressed_len < page_data.len());

        let decompressed = zswap.decompress_page(101).unwrap();
        assert_eq!(decompressed, page_data);
    }

    #[test]
    fn test_numa_memory_allocator() {
        let mut numa = NumaMultiSocketMemoryEngine::new();
        numa.register_numa_node(0, 1024, 10);
        numa.register_numa_node(1, 2048, 20);

        let allocated_node = numa.allocate_numa_memory(0, 512).unwrap();
        assert_eq!(allocated_node, 0);

        let fallback_node = numa.allocate_numa_memory(0, 800).unwrap(); // Node 0 overflow -> fallback to Node 1
        assert_eq!(fallback_node, 1);
    }

    #[test]
    fn test_taskset_cpu_affinity() {
        let mut taskset = TasksetCpuPinningSchedulerEngine::new(8);
        assert!(taskset.set_process_affinity(1001, 0b0000_1100)); // Pin to CPU 2 and 3
        assert_eq!(taskset.get_assigned_cpu(1001), Some(2));
    }

    #[test]
    fn test_edf_scheduler() {
        let mut edf = EdfDeadlineInheritanceSchedulerEngine::new();
        edf.add_task(1, 1000, 10, 1);
        edf.add_task(2, 500, 15, 2); // Earlier deadline

        let next = edf.pick_next_task().unwrap();
        assert_eq!(next.task_id, 2);
    }

    #[test]
    fn test_posix_message_queue() {
        let mut mq = PosixMsgQueueEngine::new("/test_queue", 10);
        mq.mq_send(b"Low priority", 1).unwrap();
        mq.mq_send(b"High priority", 10).unwrap();

        let msg = mq.mq_receive().unwrap();
        assert_eq!(msg.priority, 10);
        assert_eq!(msg.payload, b"High priority");
    }
}
