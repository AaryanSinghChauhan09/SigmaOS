//! SigmaOS DMA Manager Subsystem

use super::dma_queue::{DmaDescriptor, DmaDirection, ScatterGatherElement};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Typed errors for DMA operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmaError {
    BufferAlreadyMapped(u64),
    BufferNotMapped(u64),
    QueueFull,
    InvalidScatterGatherList,
    CoherenceFailure,
    DoubleDmaPrevented(u64),
}

/// The DMA Manager governs physical buffer mapping, cache coherence, ring descriptors, and double-DMA protection.
pub struct DmaManager {
    max_queue_depth: usize,
    next_descriptor_id: AtomicU64,
    mapped_buffers: BTreeMap<u64, DmaDescriptor>,
    completed_transfers: Vec<u64>,
}

impl DmaManager {
    pub fn new(max_queue_depth: usize) -> Self {
        Self {
            max_queue_depth,
            next_descriptor_id: AtomicU64::new(1),
            mapped_buffers: BTreeMap::new(),
            completed_transfers: Vec::new(),
        }
    }

    /// Map a buffer for DMA transfer.
    /// Checks for scatter-gather validity and double-DMA mapping.
    pub fn map_buffer(
        &mut self,
        buffer_id: u64,
        sg_list: Vec<ScatterGatherElement>,
        direction: DmaDirection,
    ) -> Result<u64, DmaError> {
        if sg_list.is_empty() {
            return Err(DmaError::InvalidScatterGatherList);
        }

        if self.mapped_buffers.len() >= self.max_queue_depth {
            return Err(DmaError::QueueFull);
        }

        if self
            .mapped_buffers
            .values()
            .any(|desc| desc.buffer_id == buffer_id && desc.in_flight)
        {
            return Err(DmaError::DoubleDmaPrevented(buffer_id));
        }

        let desc_id = self.next_descriptor_id.fetch_add(1, Ordering::SeqCst);
        let mut desc = DmaDescriptor::new(desc_id, buffer_id, sg_list, direction);

        // Coherence flush prior to transfer if outbound
        if direction == DmaDirection::ToDevice || direction == DmaDirection::Bidirectional {
            self.flush_cpu_cache(&desc);
        }

        desc.in_flight = true;
        self.mapped_buffers.insert(desc_id, desc);

        Ok(desc_id)
    }

    /// Complete a DMA transfer and invalidate CPU cache for inbound data.
    pub fn complete_dma(&mut self, desc_id: u64) -> Result<usize, DmaError> {
        if let Some(mut desc) = self.mapped_buffers.remove(&desc_id) {
            if desc.direction == DmaDirection::FromDevice
                || desc.direction == DmaDirection::Bidirectional
            {
                self.invalidate_cpu_cache(&desc);
            }
            desc.in_flight = false;
            let bytes = desc.total_bytes;
            self.completed_transfers.push(desc_id);
            Ok(bytes)
        } else {
            Err(DmaError::BufferNotMapped(desc_id))
        }
    }

    fn flush_cpu_cache(&self, _desc: &DmaDescriptor) {
        // Architecture-specific dcache flush
    }

    fn invalidate_cpu_cache(&self, _desc: &DmaDescriptor) {
        // Architecture-specific dcache invalidate
    }

    pub fn pending_count(&self) -> usize {
        self.mapped_buffers.len()
    }
}
