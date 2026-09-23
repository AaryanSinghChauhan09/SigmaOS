
use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// =========================================================================
// 1. SLUB / UMA SLAB OBJECT CACHE ALLOCATOR (Linux mm/slub.c & FreeBSD UMA parity)
// =========================================================================

/// Power-of-two slab size classes (16B, 32B, 64B, 128B, 256B, 512B, 1024B, 2048B)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SlabSizeClass {
    Size16 = 16,
    Size32 = 32,
    Size64 = 64,
    Size128 = 128,
    Size256 = 256,
    Size512 = 512,
    Size1024 = 1024,
    Size2048 = 2048,
}

impl SlabSizeClass {
    pub fn for_size(size: usize) -> Option<Self> {
        if size <= 16 {
            Some(SlabSizeClass::Size16)
        } else if size <= 32 {
            Some(SlabSizeClass::Size32)
        } else if size <= 64 {
            Some(SlabSizeClass::Size64)
        } else if size <= 128 {
            Some(SlabSizeClass::Size128)
        } else if size <= 256 {
            Some(SlabSizeClass::Size256)
        } else if size <= 512 {
            Some(SlabSizeClass::Size512)
        } else if size <= 1024 {
            Some(SlabSizeClass::Size1024)
        } else if size <= 2048 {
            Some(SlabSizeClass::Size2048)
        } else {
            None
        }
    }
}

/// SLUB / UMA fixed-size object cache allocator
pub struct SlabObjectCacheAllocator {
    pub object_size: usize,
    pub base_address: usize,
    pub capacity: usize,
    pub free_list: Vec<usize>,
    pub allocated_count: usize,
}

impl SlabObjectCacheAllocator {
    pub fn new(size_class: SlabSizeClass, base_address: usize, capacity: usize) -> Self {
        let object_size = size_class as usize;
        let mut free_list = Vec::with_capacity(capacity);
        for i in 0..capacity {
            free_list.push(base_address + (i * object_size));
        }

        Self {
            object_size,
            base_address,
            capacity,
            free_list,
            allocated_count: 0,
        }
    }

    /// Allocate a fixed-size slab object address
    pub fn alloc_object(&mut self) -> Option<usize> {
        if let Some(addr) = self.free_list.pop() {
            self.allocated_count += 1;
            Some(addr)
        } else {
            None
        }
    }

    /// Deallocate object address and return to slab free list
    pub fn free_object(&mut self, addr: usize) -> Result<(), &'static str> {
        if addr < self.base_address
            || addr >= self.base_address + (self.capacity * self.object_size)
        {
            return Err("SlabAllocator: Address out of slab cache bounds");
        }

        if (addr - self.base_address) % self.object_size != 0 {
            return Err("SlabAllocator: Unaligned object address for slab size class");
        }

        self.free_list.push(addr);
        self.allocated_count -= 1;
        Ok(())
    }
}

// =========================================================================
// 2. PCIE BAR & HARDWARE IO RESOURCE ALLOCATOR (Linux setup-res.c & FreeBSD sys/dev/pci parity)
// =========================================================================

/// PCIe MMIO BAR / IO Port Resource Window
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PcieResourceWindow {
    pub start_address: u64,
    pub size_bytes: u64,
    pub is_prefetchable: bool,
    pub is_64bit: bool,
    pub allocated_to_device: Option<String>,
}

/// PCIe MMIO BAR, IO Port, and MSI-X Vector Allocator
pub struct PcieResourceAllocator {
    pub mmio_32_start: u32,
    pub mmio_32_next: u32,
    pub mmio_64_start: u64,
    pub mmio_64_next: u64,
    pub io_port_next: u16,
    pub next_msix_vector: u8,
    pub windows: Vec<PcieResourceWindow>,
}

impl PcieResourceAllocator {
    pub fn new(mmio_32_start: u32, mmio_64_start: u64) -> Self {
        Self {
            mmio_32_start,
            mmio_32_next: mmio_32_start,
            mmio_64_start,
            mmio_64_next: mmio_64_start,
            io_port_next: 0x2000,
            next_msix_vector: 32, // IRQs 0-31 reserved for legacy system interrupts
            windows: Vec::new(),
        }
    }

    /// Allocate MMIO BAR address region for PCI device
    pub fn allocate_mmio_bar(
        &mut self,
        device_name: &str,
        size_bytes: u64,
        is_64bit: bool,
        is_prefetchable: bool,
    ) -> Result<u64, &'static str> {
        let align = size_bytes.max(16);
        let allocated_addr = if is_64bit {
            let aligned = (self.mmio_64_next + align - 1) & !(align - 1);
            self.mmio_64_next = aligned + size_bytes;
            aligned
        } else {
            let aligned = ((self.mmio_32_next as u64) + align - 1) & !(align - 1);
            if aligned + size_bytes > 0xFFFF_FFFF {
                return Err("PcieAllocator: 32-bit MMIO space exhausted");
            }
            self.mmio_32_next = (aligned + size_bytes) as u32;
            aligned
        };

        self.windows.push(PcieResourceWindow {
            start_address: allocated_addr,
            size_bytes,
            is_prefetchable,
            is_64bit,
            allocated_to_device: Some(device_name.to_string()),
        });

        Ok(allocated_addr)
    }

    /// Allocate MSI-X interrupt vector
    pub fn allocate_msix_vector(&mut self) -> Result<u8, &'static str> {
        if self.next_msix_vector >= 254 {
            return Err("PcieAllocator: MSI-X IRQ vector space exhausted");
        }
        let vec = self.next_msix_vector;
        self.next_msix_vector += 1;
        Ok(vec)
    }
}

// =========================================================================
// 3. LINUX CGROUPS V2 & FREEBSD RCTL CONTAINER RESOURCE GOVERNOR
// =========================================================================

/// Container / process resource quota limits inspired by Linux Cgroups v2 & FreeBSD RCTL
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub cpu_shares: u32,           // 1 - 1024 CPU weight
    pub ram_limit_bytes: u64,      // Hard memory ceiling
    pub ram_soft_limit_bytes: u64, // Soft memory warning threshold
    pub iops_read_limit: u32,      // Max read IOPS
    pub iops_write_limit: u32,     // Max write IOPS
}

/// Active resource utilization metrics
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub cpu_usage_percent: u32,
    pub ram_usage_bytes: u64,
    pub current_read_iops: u32,
    pub current_write_iops: u32,
}

/// Container Resource Governor enforcing quota limits and throttling
pub struct ContainerResourceGovernor {
    pub container_id: String,
    pub limits: ResourceLimits,
    pub usage: ResourceUsage,
}

impl ContainerResourceGovernor {
    pub fn new(container_id: &str, limits: ResourceLimits) -> Self {
        Self {
            container_id: container_id.to_string(),
            limits,
            usage: ResourceUsage {
                cpu_usage_percent: 0,
                ram_usage_bytes: 0,
                current_read_iops: 0,
                current_write_iops: 0,
            },
        }
    }

    /// Update container memory allocation and enforce Cgroup v2 limits
    pub fn request_memory(&mut self, bytes_to_add: u64) -> Result<(), &'static str> {
        if self.usage.ram_usage_bytes + bytes_to_add > self.limits.ram_limit_bytes {
            return Err("CgroupsGovernor: Memory allocation denied - container OOM limit reached");
        }

        self.usage.ram_usage_bytes += bytes_to_add;
        Ok(())
    }

    /// Check if container memory usage exceeds soft warning limit
    pub fn is_over_soft_limit(&self) -> bool {
        self.usage.ram_usage_bytes > self.limits.ram_soft_limit_bytes
    }
}

// =========================================================================
// 4. OPENBSD HARDENED ASLR GUARD PAGE ALLOCATOR
// =========================================================================

/// OpenBSD Hardened Malloc inspired Guard Page Allocator
pub struct HardenedGuardPageAllocator {
    pub base_address: usize,
    pub page_size: usize,
    pub current_offset: usize,
    pub guard_pages_count: usize,
}

impl HardenedGuardPageAllocator {
    pub fn new(base_address: usize, page_size: usize) -> Self {
        Self {
            base_address,
            page_size,
            current_offset: 0,
            guard_pages_count: 0,
        }
    }

    /// Allocate a memory region bounded by unmapped ASLR guard pages
    pub fn alloc_with_guard_page(&mut self, data_pages: usize) -> (usize, usize) {
        let guard_page_addr = self.base_address + self.current_offset;
        let data_addr = guard_page_addr + self.page_size;

        let total_span_pages = 1 + data_pages + 1; // [Guard Page] [Data Pages] [Guard Page]
        self.current_offset += total_span_pages * self.page_size;
        self.guard_pages_count += 2;

        (data_addr, data_pages * self.page_size)
    }
}

// =========================================================================
// 5. LINUX IO_URING & DMA ZERO-COPY RING BUFFER ALLOCATOR
// =========================================================================

/// DMA Zero-Copy Ring Buffer Descriptor (NVMe queues / NIC ring buffers)
#[derive(Debug, Clone)]
pub struct DmaRingBuffer {
    pub ring_id: u32,
    pub base_phys_addr: u64,
    pub buffer_size: usize,
    pub head: usize,
    pub tail: usize,
}

/// Zero-Copy DMA Ring Buffer Allocator
pub struct DmaRingBufferAllocator {
    pub next_ring_id: u32,
    pub phys_base_next: u64,
    pub rings: Vec<DmaRingBuffer>,
}

impl DmaRingBufferAllocator {
    pub fn new(phys_base_start: u64) -> Self {
        Self {
            next_ring_id: 1,
            phys_base_next: phys_base_start,
            rings: Vec::new(),
        }
    }

    /// Create a zero-copy DMA ring buffer for NVMe submission/completion queues or NIC descriptors
    pub fn create_dma_ring(&mut self, buffer_size: usize) -> Result<DmaRingBuffer, &'static str> {
        let ring_id = self.next_ring_id;
        self.next_ring_id += 1;

        let phys_addr = self.phys_base_next;
        self.phys_base_next += buffer_size as u64;

        let ring = DmaRingBuffer {
            ring_id,
            base_phys_addr: phys_addr,
            buffer_size,
            head: 0,
            tail: 0,
        };

        self.rings.push(ring.clone());
        Ok(ring)
    }
}

// =========================================================================
// 6. SIGMA RESOURCE ALLOCATOR HUB
// =========================================================================

/// Linux CMA (Contiguous Memory Allocator) & BSD Physical Contiguous Memory Engine
#[derive(Debug, Clone)]
pub struct ContiguousFrameAllocator {
    pub base_phys_addr: u64,
    pub total_frames: usize,
    pub frame_size_bytes: usize,
    pub frame_bitmap: Vec<bool>,
}

impl ContiguousFrameAllocator {
    pub fn new(base_phys_addr: u64, total_frames: usize, frame_size_bytes: usize) -> Self {
        Self {
            base_phys_addr,
            total_frames,
            frame_size_bytes,
            frame_bitmap: vec![false; total_frames],
        }
    }

    pub fn alloc_contiguous_frames(&mut self, count: usize) -> Result<u64, &'static str> {
        if count == 0 || count > self.total_frames {
            return Err("CMA: Invalid frame count requested");
        }

        let mut consecutive = 0;
        let mut start_idx = 0;

        for i in 0..self.total_frames {
            if !self.frame_bitmap[i] {
                if consecutive == 0 {
                    start_idx = i;
                }
                consecutive += 1;
                if consecutive == count {
                    for j in start_idx..start_idx + count {
                        self.frame_bitmap[j] = true;
                    }
                    return Ok(self.base_phys_addr + (start_idx * self.frame_size_bytes) as u64);
                }
            } else {
                consecutive = 0;
            }
        }

        Err("CMA: Contiguous frame allocation failed - space fragmented or exhausted")
    }

    pub fn free_contiguous_frames(&mut self, phys_addr: u64, count: usize) -> Result<(), &'static str> {
        if phys_addr < self.base_phys_addr {
            return Err("CMA: Address below base physical address");
        }

        let offset = (phys_addr - self.base_phys_addr) as usize;
        if offset % self.frame_size_bytes != 0 {
            return Err("CMA: Physical address unaligned to frame boundary");
        }

        let start_idx = offset / self.frame_size_bytes;
        if start_idx + count > self.total_frames {
            return Err("CMA: Frame range out of physical memory bounds");
        }

        for j in start_idx..start_idx + count {
            self.frame_bitmap[j] = false;
        }

        Ok(())
    }
}

/// Equal & Proportional Frame Allocator (OS Paging & Virtual Memory)
#[derive(Debug, Clone)]
pub struct ProcessFrameDemand {
    pub pid: u32,
    pub page_demand: usize,
}

#[derive(Debug, Clone)]
pub struct FrameProportionalAllocator {
    pub total_available_frames: usize,
}

impl FrameProportionalAllocator {
    pub fn new(total_frames: usize) -> Self {
        Self {
            total_available_frames: total_frames,
        }
    }

    /// Equal allocation: Divide total frames equally across active processes
    pub fn allocate_equal_frames(&self, process_count: usize) -> usize {
        if process_count == 0 {
            0
        } else {
            self.total_available_frames / process_count
        }
    }

    /// Proportional allocation: Allocate frames proportionally based on page demand ratio
    pub fn allocate_proportional_frames(&self, demands: &[ProcessFrameDemand]) -> Vec<(u32, usize)> {
        let total_demand: usize = demands.iter().map(|d| d.page_demand).sum();
        if total_demand == 0 {
            return demands.iter().map(|d| (d.pid, 0)).collect();
        }

        demands
            .iter()
            .map(|d| {
                let allocated = (d.page_demand * self.total_available_frames) / total_demand;
                (d.pid, allocated.max(1)) // Ensure minimum 1 frame per active process
            })
            .collect()
    }
}

/// Linked Block & Indexed Block Allocator (File Systems & Secondary Storage)
#[derive(Debug, Clone)]
pub struct LinkedBlockNode {
    pub block_id: u32,
    pub next_block_id: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct LinkedIndexedBlockAllocator {
    pub total_storage_blocks: usize,
    pub index_table: BTreeMap<u32, Vec<u32>>,
}

impl LinkedIndexedBlockAllocator {
    pub fn new(total_blocks: usize) -> Self {
        Self {
            total_storage_blocks: total_blocks,
            index_table: BTreeMap::new(),
        }
    }

    /// Allocate indexed block entry mapping logical file blocks to physical storage blocks
    pub fn allocate_file_index_block(&mut self, file_id: u32, block_list: Vec<u32>) -> Result<(), &'static str> {
        if block_list.iter().any(|&b| b as usize >= self.total_storage_blocks) {
            return Err("IndexedBlockAllocator: Storage block index out of bounds");
        }
        self.index_table.insert(file_id, block_list);
        Ok(())
    }

    /// Retrieve physical block for logical file offset via index table
    pub fn resolve_logical_block(&self, file_id: u32, logical_offset: usize) -> Option<u32> {
        self.index_table.get(&file_id).and_then(|blocks| blocks.get(logical_offset).copied())
    }
}

/// Free-Frame Accountant (Tracks Free Frames Before and After Allocations)
#[derive(Debug, Clone)]
pub struct FreeFrameAccountant {
    pub total_system_frames: usize,
    pub free_frames_before: usize,
    pub free_frames_after: usize,
    pub secondary_storage_swap_pages: usize,
}

impl FreeFrameAccountant {
    pub fn new(total_frames: usize, swap_pages: usize) -> Self {
        Self {
            total_system_frames: total_frames,
            free_frames_before: total_frames,
            free_frames_after: total_frames,
            secondary_storage_swap_pages: swap_pages,
        }
    }

    pub fn record_allocation_begin(&mut self, current_free: usize) {
        self.free_frames_before = current_free;
    }

    pub fn record_allocation_end(&mut self, current_free: usize) -> usize {
        self.free_frames_after = current_free;
        if self.free_frames_before >= self.free_frames_after {
            self.free_frames_before - self.free_frames_after
        } else {
            0
        }
    }

    pub fn page_out_to_secondary_storage(&mut self, pages_count: usize) -> Result<usize, &'static str> {
        if pages_count > self.secondary_storage_swap_pages {
            return Err("FreeFrameAccountant: Secondary storage swap space exhausted");
        }
        self.secondary_storage_swap_pages -= pages_count;
        Ok(self.secondary_storage_swap_pages)
    }
}

/// Central Resource Allocator Hub unifying kernel memory, PCIe, Cgroups, and DMA allocators
pub struct SigmaResourceAllocatorHub {
    pub pcie_allocator: PcieResourceAllocator,
    pub hardened_allocator: HardenedGuardPageAllocator,
    pub dma_allocator: DmaRingBufferAllocator,
    pub cma_allocator: ContiguousFrameAllocator,
    pub frame_accountant: FreeFrameAccountant,
}

impl SigmaResourceAllocatorHub {
    pub fn new() -> Self {
        Self {
            pcie_allocator: PcieResourceAllocator::new(0xE000_0000, 0x1000_0000_0000),
            hardened_allocator: HardenedGuardPageAllocator::new(0x7FFF_0000_0000, 4096),
            dma_allocator: DmaRingBufferAllocator::new(0x2000_0000),
            cma_allocator: ContiguousFrameAllocator::new(0x4000_0000, 1024, 4096),
            frame_accountant: FreeFrameAccountant::new(16384, 8192),
        }
    }
}

impl Default for SigmaResourceAllocatorHub {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slab_object_cache_allocator() {
        let mut slab = SlabObjectCacheAllocator::new(SlabSizeClass::Size64, 0x1000, 10);
        assert_eq!(slab.object_size, 64);
        assert_eq!(slab.free_list.len(), 10);

        let addr1 = slab.alloc_object().unwrap();
        assert_eq!(addr1, 0x1240); // 0x1000 + 9*64
        assert_eq!(slab.allocated_count, 1);

        assert!(slab.free_object(addr1).is_ok());
        assert_eq!(slab.allocated_count, 0);

        // Invalid unaligned address
        assert!(slab.free_object(0x1005).is_err());
    }

    #[test]
    fn test_pcie_resource_allocator() {
        let mut pcie = PcieResourceAllocator::new(0xE000_0000, 0x1000_0000_0000);

        let bar0 = pcie
            .allocate_mmio_bar("nvidia_gpu", 0x1000_0000, true, true)
            .unwrap();
        assert_eq!(bar0, 0x1000_0000_0000);

        let irq = pcie.allocate_msix_vector().unwrap();
        assert_eq!(irq, 32);
    }

    #[test]
    fn test_container_resource_governor() {
        let limits = ResourceLimits {
            cpu_shares: 512,
            ram_limit_bytes: 1024 * 1024 * 1024,
            ram_soft_limit_bytes: 512 * 1024 * 1024,
            iops_read_limit: 1000,
            iops_write_limit: 500,
        };

        let mut governor = ContainerResourceGovernor::new("container-web", limits);

        // Under limit
        assert!(governor.request_memory(256 * 1024 * 1024).is_ok());
        assert!(!governor.is_over_soft_limit());

        // Over soft warning limit
        assert!(governor.request_memory(384 * 1024 * 1024).is_ok());
        assert!(governor.is_over_soft_limit());

        // Exceed hard limit -> OOM error
        let oom = governor.request_memory(512 * 1024 * 1024);
        assert!(oom.is_err());
    }

    #[test]
    fn test_hardened_guard_page_allocator() {
        let mut hardened = HardenedGuardPageAllocator::new(0x7FFF_0000_0000, 4096);
        let (data_addr, size) = hardened.alloc_with_guard_page(4);

        assert_eq!(data_addr, 0x7FFF_0000_0000 + 4096); // offset by 1 guard page
        assert_eq!(size, 4 * 4096);
        assert_eq!(hardened.guard_pages_count, 2);
    }

    #[test]
    fn test_dma_ring_buffer_allocator() {
        let mut dma = DmaRingBufferAllocator::new(0x2000_0000);
        let ring = dma.create_dma_ring(65536).unwrap();

        assert_eq!(ring.ring_id, 1);
        assert_eq!(ring.base_phys_addr, 0x2000_0000);
        assert_eq!(ring.buffer_size, 65536);
    }

    #[test]
    fn test_contiguous_frame_allocator() {
        let mut cma = ContiguousFrameAllocator::new(0x4000_0000, 100, 4096);
        let addr = cma.alloc_contiguous_frames(4).unwrap();
        assert_eq!(addr, 0x4000_0000);

        assert!(cma.free_contiguous_frames(addr, 4).is_ok());
    }

    #[test]
    fn test_frame_proportional_allocator() {
        let prop = FrameProportionalAllocator::new(100);
        assert_eq!(prop.allocate_equal_frames(4), 25);

        let demands = vec![
            ProcessFrameDemand { pid: 101, page_demand: 10 },
            ProcessFrameDemand { pid: 102, page_demand: 30 },
        ];
        let allocs = prop.allocate_proportional_frames(&demands);
        assert_eq!(allocs[0], (101, 25));
        assert_eq!(allocs[1], (102, 75));
    }

    #[test]
    fn test_linked_indexed_block_allocator() {
        let mut idx_alloc = LinkedIndexedBlockAllocator::new(1000);
        assert!(idx_alloc.allocate_file_index_block(1, vec![10, 11, 12, 13]).is_ok());
        assert_eq!(idx_alloc.resolve_logical_block(1, 2), Some(12));
    }

    #[test]
    fn test_free_frame_accountant() {
        let mut acc = FreeFrameAccountant::new(1000, 500);
        acc.record_allocation_begin(1000);
        let allocated = acc.record_allocation_end(980);
        assert_eq!(allocated, 20);

        let swap_rem = acc.page_out_to_secondary_storage(100).unwrap();
        assert_eq!(swap_rem, 400);
    }
}
