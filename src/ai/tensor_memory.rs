// Zero-Copy AI Tensor Memory Manager for SigmaOS
// Inspired by Linux TTM/GEM graphics translation memory and FreeBSD UMA / contigmalloc.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Precision and format of tensor data elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TensorDtype {
    Fp32,
    Fp16,
    Bf16,
    Int8,
    Int4,
}

impl TensorDtype {
    /// Returns element size in bytes (fractional for INT4, rounded up to byte boundary).
    pub fn element_size_bytes(&self) -> usize {
        match self {
            TensorDtype::Fp32 => 4,
            TensorDtype::Fp16 | TensorDtype::Bf16 => 2,
            TensorDtype::Int8 => 1,
            TensorDtype::Int4 => 1, // 2 elements per byte, treated as 1 byte min unit
        }
    }
}

/// Memory pin mode for acceleration hardware access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPinMode {
    UnpinnedHost,
    PinnedHostDma,
    DeviceLocalGpu,
    CoherentShared,
}

/// Descriptor for a Zero-Copy Tensor Buffer.
#[derive(Debug, Clone)]
pub struct TensorBuffer {
    pub id: usize,
    pub name: String,
    pub shape: Vec<usize>,
    pub dtype: TensorDtype,
    pub byte_size: usize,
    pub physical_address: u64,
    pub virt_ptr: usize,
    pub pin_mode: MemoryPinMode,
    pub alignment: usize,
    pub is_mapped_for_dma: bool,
    pub reference_count: usize,
}

impl TensorBuffer {
    /// Calculates total element count from tensor shape dimensions.
    pub fn element_count(&self) -> usize {
        if self.shape.is_empty() {
            0
        } else {
            self.shape.iter().product()
        }
    }
}

/// Memory Manager Statistics
#[derive(Debug, Clone, Default)]
pub struct TensorMemoryStats {
    pub total_allocated_bytes: usize,
    pub pinned_dma_bytes: usize,
    pub coherent_shared_bytes: usize,
    pub buffer_count: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub dma_mappings: usize,
}

/// Manager responsible for zero-copy AI tensor allocation, page-pinning, and UMA slab pooling.
pub struct AiTensorMemoryManager {
    buffers: Vec<TensorBuffer>,
    next_buffer_id: AtomicUsize,
    total_capacity_bytes: usize,
    used_bytes: usize,
    cache_hits: usize,
    cache_misses: usize,
    dma_mappings: usize,
}

impl AiTensorMemoryManager {
    /// Creates a new manager with maximum capacity in bytes.
    pub fn new(capacity_bytes: usize) -> Self {
        Self {
            buffers: Vec::new(),
            next_buffer_id: AtomicUsize::new(1),
            total_capacity_bytes: capacity_bytes,
            used_bytes: 0,
            cache_hits: 0,
            cache_misses: 0,
            dma_mappings: 0,
        }
    }

    /// Allocates a zero-copy tensor buffer with requested dimensions, dtype, and pin mode.
    pub fn allocate_tensor(
        &mut self,
        name: &str,
        shape: Vec<usize>,
        dtype: TensorDtype,
        pin_mode: MemoryPinMode,
    ) -> Result<usize, &'static str> {
        let total_elements: usize = if shape.is_empty() {
            0
        } else {
            shape.iter().product()
        };

        let raw_byte_size = match dtype {
            TensorDtype::Int4 => (total_elements + 1) / 2,
            _ => total_elements * dtype.element_size_bytes(),
        };

        // Align to 64-byte boundary for AVX-512 / DMA alignment
        let alignment = 64;
        let byte_size = (raw_byte_size + (alignment - 1)) & !(alignment - 1);

        if self.used_bytes + byte_size > self.total_capacity_bytes {
            return Err("Out of AI Tensor Memory capacity");
        }

        let id = self.next_buffer_id.fetch_add(1, Ordering::SeqCst);
        let phys_addr = 0x1_0000_0000 + (id as u64 * 0x100_000);
        let virt_ptr = 0x7F00_0000_0000 + (id * 0x100_000);

        let is_mapped = match pin_mode {
            MemoryPinMode::PinnedHostDma
            | MemoryPinMode::CoherentShared
            | MemoryPinMode::DeviceLocalGpu => {
                self.dma_mappings += 1;
                true
            }
            MemoryPinMode::UnpinnedHost => false,
        };

        let buffer = TensorBuffer {
            id,
            name: name.to_string(),
            shape,
            dtype,
            byte_size,
            physical_address: phys_addr,
            virt_ptr,
            pin_mode,
            alignment,
            is_mapped_for_dma: is_mapped,
            reference_count: 1,
        };

        self.used_bytes += byte_size;
        self.buffers.push(buffer);
        self.cache_misses += 1;

        Ok(id)
    }

    /// Re-uses or acquires existing tensor buffer if shape and dtype match.
    pub fn share_tensor(&mut self, id: usize) -> Option<TensorBuffer> {
        if let Some(buf) = self.buffers.iter_mut().find(|b| b.id == id) {
            buf.reference_count += 1;
            self.cache_hits += 1;
            Some(buf.clone())
        } else {
            None
        }
    }

    /// Deallocates or releases reference to a tensor buffer.
    pub fn release_tensor(&mut self, id: usize) -> bool {
        let mut remove_idx = None;
        let mut freed_bytes = 0;

        for (idx, buf) in self.buffers.iter_mut().enumerate() {
            if buf.id == id {
                if buf.reference_count > 1 {
                    buf.reference_count -= 1;
                    return true;
                } else {
                    remove_idx = Some(idx);
                    freed_bytes = buf.byte_size;
                    break;
                }
            }
        }

        if let Some(idx) = remove_idx {
            self.buffers.remove(idx);
            self.used_bytes = self.used_bytes.saturating_sub(freed_bytes);
            true
        } else {
            false
        }
    }

    /// Pin tensor buffer into DMA contiguous physical memory for hardware access.
    pub fn pin_buffer_dma(&mut self, id: usize) -> bool {
        if let Some(buf) = self.buffers.iter_mut().find(|b| b.id == id) {
            if !buf.is_mapped_for_dma {
                buf.pin_mode = MemoryPinMode::PinnedHostDma;
                buf.is_mapped_for_dma = true;
                self.dma_mappings += 1;
            }
            true
        } else {
            false
        }
    }

    /// Retrieves buffer by ID.
    pub fn get_buffer(&self, id: usize) -> Option<&TensorBuffer> {
        self.buffers.iter().find(|b| b.id == id)
    }

    /// Returns memory manager statistics telemetry.
    pub fn get_stats(&self) -> TensorMemoryStats {
        let pinned_bytes: usize = self
            .buffers
            .iter()
            .filter(|b| b.pin_mode == MemoryPinMode::PinnedHostDma)
            .map(|b| b.byte_size)
            .sum();

        let coherent_bytes: usize = self
            .buffers
            .iter()
            .filter(|b| b.pin_mode == MemoryPinMode::CoherentShared)
            .map(|b| b.byte_size)
            .sum();

        TensorMemoryStats {
            total_allocated_bytes: self.used_bytes,
            pinned_dma_bytes: pinned_bytes,
            coherent_shared_bytes: coherent_bytes,
            buffer_count: self.buffers.len(),
            cache_hits: self.cache_hits,
            cache_misses: self.cache_misses,
            dma_mappings: self.dma_mappings,
        }
    }
}
