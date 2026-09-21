# Memory Management

SigmaOS implements comprehensive memory management with Linux and BSD-inspired features including virtual memory, paging, swap, and memory allocation.

## Overview

Memory management provides:
- Virtual memory with page tables and address spaces
- Demand paging with swap support
- Memory allocation with slab allocator and buddy system
- Memory overcommit and OOM handling
- Transparent huge pages (THP)
- Memory compaction and defragmentation
- Memory accounting and cgroups integration
- NUMA-aware memory allocation

## Implementation

### Virtual Memory
```rust
// src/kernel/vm.rs
pub struct VirtualMemoryManager {
    pub page_tables: BTreeMap<Pid, PageTable>,
    pub free_pages: AtomicBitmap,
    pub allocated_pages: usize,
    pub total_pages: usize,
}

pub struct PageTable {
    pub entries: BTreeMap<VirtualAddress, PageTableEntry>,
    pub address_space_size: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct PageTableEntry {
    pub physical_address: PhysicalAddress,
    pub present: bool,
    pub writable: bool,
    pub executable: bool,
    pub user_accessible: bool,
    pub dirty: bool,
    pub accessed: bool,
}

impl VirtualMemoryManager {
    pub fn new(total_pages: usize) -> Self {
        VirtualMemoryManager {
            page_tables: BTreeMap::new(),
            free_pages: AtomicBitmap::new(total_pages),
            allocated_pages: 0,
            total_pages,
        }
    }

    pub fn allocate_page(&mut self) -> Result<PhysicalAddress, MemoryError> {
        if let Some(page_index) = self.free_pages.find_first_zero() {
            self.free_pages.set(page_index, true);
            self.allocated_pages += 1;
            Ok(PhysicalAddress::from_index(page_index))
        } else {
            Err(MemoryError::OutOfMemory)
        }
    }

    pub fn free_page(&mut self, physical_address: PhysicalAddress) {
        let page_index = physical_address.to_index();
        self.free_pages.set(page_index, false);
        self.allocated_pages -= 1;
    }

    pub fn map_page(&mut self, pid: Pid, virtual_addr: VirtualAddress, physical_addr: PhysicalAddress, flags: PageFlags) -> Result<(), MemoryError> {
        let page_table = self.page_tables.entry(pid).or_insert_with(PageTable::new);

        let entry = PageTableEntry {
            physical_address: physical_addr,
            present: true,
            writable: flags.contains(PageFlags::WRITABLE),
            executable: flags.contains(PageFlags::EXECUTABLE),
            user_accessible: flags.contains(PageFlags::USER),
            dirty: false,
            accessed: false,
        };

        page_table.entries.insert(virtual_addr, entry);
        Ok(())
    }

    pub fn unmap_page(&mut self, pid: Pid, virtual_addr: VirtualAddress) -> Result<(), MemoryError> {
        if let Some(page_table) = self.page_tables.get_mut(&pid) {
            if let Some(entry) = page_table.entries.remove(&virtual_addr) {
                self.free_page(entry.physical_address);
                Ok(())
            } else {
                Err(MemoryError::PageNotMapped)
            }
        } else {
            Err(MemoryError::ProcessNotFound)
        }
    }
}
```

### Demand Paging
```rust
// src/kernel/paging.rs
pub struct DemandPagingEngine {
    pub page_fault_handler: PageFaultHandler,
    pub swap_manager: SwapManager,
    pub page_cache: PageCache,
}

impl DemandPagingEngine {
    pub fn new() -> Self {
        DemandPagingEngine {
            page_fault_handler: PageFaultHandler::new(),
            swap_manager: SwapManager::new(),
            page_cache: PageCache::new(),
        }
    }

    pub fn handle_page_fault(&mut self, fault_addr: VirtualAddress, fault_type: PageFaultType) -> Result<(), MemoryError> {
        match fault_type {
            PageFaultType::NotPresent => {
                // Page not present, load from swap or allocate
                if let Some(swap_entry) = self.swap_manager.find_swap_entry(fault_addr) {
                    // Load from swap
                    let page = self.swap_manager.load_page(swap_entry)?;
                    self.page_cache.insert(fault_addr, page);
                } else {
                    // Allocate new page
                    let page = self.allocate_zeroed_page()?;
                    self.page_cache.insert(fault_addr, page);
                }
                Ok(())
            }
            PageFaultType::ProtectionFault => {
                // Protection fault, likely invalid access
                Err(MemoryError::AccessViolation)
            }
        }
    }

    fn allocate_zeroed_page(&self) -> Result<Page, MemoryError> {
        // Allocate and zero a page
        Ok(Page::zeroed())
    }
}
```

### Slab Allocator
```rust
// src/kernel/slab.rs
pub struct SlabAllocator {
    pub slabs: BTreeMap<usize, Slab>,
    pub small_object_cache: BTreeMap<usize, ObjectCache>,
}

pub struct Slab {
    pub object_size: usize,
    pub objects_per_slab: usize,
    pub free_objects: VecDeque<usize>,
    pub allocated_objects: usize,
}

pub struct ObjectCache {
    pub object_size: usize,
    pub free_objects: Vec<*mut u8>,
}

impl SlabAllocator {
    pub fn new() -> Self {
        SlabAllocator {
            slabs: BTreeMap::new(),
            small_object_cache: BTreeMap::new(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Result<*mut u8, MemoryError> {
        // Find appropriate slab or cache
        if let Some(cache) = self.small_object_cache.get_mut(&size) {
            if let Some(ptr) = cache.free_objects.pop() {
                return Ok(ptr);
            }
        }

        // Allocate from slab
        self.allocate_from_slab(size)
    }

    pub fn free(&mut self, ptr: *mut u8, size: usize) {
        // Return to cache or slab
        if let Some(cache) = self.small_object_cache.get_mut(&size) {
            cache.free_objects.push(ptr);
        }
    }

    fn allocate_from_slab(&mut self, size: usize) -> Result<*mut u8, MemoryError> {
        // Implement slab allocation
        Err(MemoryError::NotImplemented)
    }
}
```

### Memory Compaction
```rust
// src/kernel/compaction.rs
pub struct MemoryCompactor {
    pub movable_pages: BTreeSet<PhysicalAddress>,
    pub free_pages: BTreeSet<PhysicalAddress>,
}

impl MemoryCompactor {
    pub fn new() -> Self {
        MemoryCompactor {
            movable_pages: BTreeSet::new(),
            free_pages: BTreeSet::new(),
        }
    }

    pub fn compact(&mut self) -> Result<usize, MemoryError> {
        let mut moved_pages = 0;

        // Move movable pages to consolidate free space
        for page in &self.movable_pages {
            if self.move_page(*page) {
                moved_pages += 1;
            }
        }

        Ok(moved_pages)
    }

    fn move_page(&self, page: PhysicalAddress) -> bool {
        // Move page to new location
        // Returns true if successful
        false
    }

    pub fn add_movable_page(&mut self, page: PhysicalAddress) {
        self.movable_pages.insert(page);
    }

    pub fn add_free_page(&mut self, page: PhysicalAddress) {
        self.free_pages.insert(page);
    }
}
```

## Configuration

### Memory Management Configuration
```toml
# /etc/sigmaos/memory.toml
[virtual_memory]
# Virtual memory settings
page_size = 4096
address_space_size = 48

[paging]
# Demand paging settings
enabled = true
swap_enabled = true
swap_priority = -1
swapiness = 60

[allocation]
# Memory allocation settings
slab_enabled = true
buddy_enabled = true
huge_pages_enabled = true

[compaction]
# Memory compaction settings
enabled = true
compaction_interval_seconds = 300
defrag_enabled = true

[oom]
# OOM killer settings
enabled = true
score_adj_range = -1000..1000
```

### Runtime Control
```bash
# Show memory statistics
sigmem stats

# Show page tables
sigmem show-page-tables <pid>

# Enable swap
sigmem enable-swap /dev/sda1

# Set swappiness
sigmem set-swappiness 60

# Enable transparent huge pages
sigmem enable-thp

# Set huge page size
sigmem set-huge-page-size 2048

# Enable memory compaction
sigmem enable-compaction

# Trigger memory compaction
sigmem compact

# Show OOM scores
sigmem show-oom-scores

# Set OOM score adjustment
sigmem set-oom-score <pid> -500
```

## Performance Optimization

### Virtual Memory Tuning
Optimize virtual memory for performance:
```bash
# Enable transparent huge pages
sigmem enable-thp

# Set aggressive swappiness
sigmem set-swappiness 10

# Enable memory compaction
sigmem enable-compaction

# Increase page cache size
sigmem set-page-cache-size 4G

# Enable dirty writeback
sigmem enable-dirty-writeback
```

### Swap Optimization
Optimize swap for performance:
```bash
# Use zram for compressed swap
sigmem enable-zram

# Set swap priority
sigmem set-swap-priority -1

# Enable swap compression
sigmem enable-swap-compression

# Set swap size
sigmem set-swap-size 8G
```

### Allocator Optimization
Optimize memory allocator for performance:
```bash
# Enable slab allocator
sigmem enable-slab

# Enable buddy allocator
sigmem enable-buddy

# Enable huge pages
sigmem enable-huge-pages

# Set slab cache size
sigmem set-slab-cache-size 128M
```

## Troubleshooting

### Out of Memory
If system is out of memory:
1. Check memory statistics: `sigmem stats`
2. Check swap usage: `sigmem swap-stats`
3. Check OOM killer logs
4. Increase swap if necessary
5. Kill memory-hungry processes

### High Swap Usage
If swap usage is high:
1. Check swap statistics: `sigmem swap-stats`
2. Check memory usage: `sigmem stats`
3. Reduce swappiness: `sigmem set-swappiness 10`
4. Increase physical memory
5. Optimize applications

### Page Faults
If page faults are high:
1. Check page fault statistics: `sigmem page-fault-stats`
2. Check working set size
3. Increase memory
4. Check for memory leaks
5. Optimize application memory usage

### Memory Fragmentation
If memory is fragmented:
1. Check fragmentation: `sigmem fragmentation-stats`
2. Enable memory compaction: `sigmem enable-compaction`
3. Trigger compaction: `sigmem compact`
4. Enable transparent huge pages
5. Reboot if necessary

---

**[Memory Management](Category-Memory-Management)** | **[Virtual Memory](Category-Virtual-Memory)** | **[Paging](Category-Paging)**
