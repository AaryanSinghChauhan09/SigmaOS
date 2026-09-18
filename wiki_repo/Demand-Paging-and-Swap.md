# Demand Paging and Swap

SigmaOS implements demand paging with swap support to efficiently manage memory usage when physical RAM is exhausted. This feature is critical for systems with limited memory and for running memory-intensive applications.

## Overview

Demand paging allows the operating system to load pages of memory from disk only when they are needed, rather than loading entire programs into memory at once. When physical RAM is full, less frequently used pages can be swapped out to disk (swap space) to make room for more active pages.

## Architecture

### Page Fault Handling
When a process accesses a page that is not in physical memory:
1. Hardware triggers a page fault exception
2. Kernel handles the fault by locating the page in swap or loading from executable
3. Page is loaded into physical memory
4. Process resumes execution

### Swap Space
SigmaOS supports multiple swap backends:
- **Swap partitions**: Dedicated disk partitions for swap
- **Swap files**: Regular files used as swap space
- **Compressed swap (zswap)**: Compressed memory cache before disk swap

## Implementation

### Page Fault Handler
```rust
// src/kernel/paging.rs
pub fn handle_page_fault(fault_address: u64, error_code: u64) -> Result<(), PagingError> {
    let vma = find_vma(fault_address)?;
    
    if vma.is_swap_backed() {
        // Page is in swap space
        let swap_page = swap_manager.read_page(vma.swap_offset)?;
        map_page_to_physical(fault_address, swap_page)?;
    } else if vma.is_file_backed() {
        // Page is from executable file
        let file_page = file_manager.read_page(vma.file_offset)?;
        map_page_to_physical(fault_address, file_page)?;
    } else {
        // Page is anonymous and not yet allocated
        let zero_page = allocate_zero_page()?;
        map_page_to_physical(fault_address, zero_page)?;
    }
    
    Ok(())
}
```

### Swap Manager
```rust
// src/kernel/swap.rs
pub struct SwapManager {
    devices: Vec<SwapDevice>,
    active_swap: Option<usize>,
    swapin_count: AtomicU64,
    swapout_count: AtomicU64,
}

impl SwapManager {
    pub fn add_swap_partition(&mut self, device: String) -> Result<(), SwapError> {
        let swap_dev = SwapDevice::new_partition(device)?;
        self.devices.push(swap_dev);
        Ok(())
    }
    
    pub fn read_page(&self, offset: u64) -> Result<PhysicalPage, SwapError> {
        let active = self.active_swap.ok_or(SwapError::NoActiveSwap)?;
        let device = &self.devices[active];
        device.read_page(offset)
    }
    
    pub fn write_page(&self, page: PhysicalPage, offset: u64) -> Result<(), SwapError> {
        let active = self.active_swap.ok_or(SwapError::NoActiveSwap)?;
        let device = &self.devices[active];
        device.write_page(page, offset)
    }
}
```

### Memory Reclamation (LRU)
```rust
// src/kernel/lru.rs
pub struct LruReclaimer {
    active_list: Arc<Mutex<Vec<PageInfo>>>,
    inactive_list: Arc<Mutex<Vec<PageInfo>>>,
}

impl LruReclaimer {
    pub fn reclaim_pages(&self, target: usize) -> Result<Vec<PhysicalPage>, ReclaimError> {
        let mut inactive = self.inactive_list.lock().unwrap();
        let mut reclaimed = Vec::new();
        
        while reclaimed.len() < target && !inactive.is_empty() {
            let page = inactive.pop().unwrap();
            if page.can_reclaim() {
                if page.is_dirty() {
                    swap_manager.write_page(page.physical, page.swap_offset)?;
                }
                reclaimed.push(page.physical);
            }
        }
        
        Ok(reclaimed)
    }
}
```

## Configuration

### Swap Configuration
```toml
# /etc/sigmaos/swap.toml
[swap]
enabled = true
priority = 100

[[swap.devices]]
type = "partition"
path = "/dev/sda2"
priority = 100

[[swap.devices]]
type = "file"
path = "/swapfile"
size = "8G"
priority = 50

[zswap]
enabled = true
max_pool_percent = 25
```

### Swap Management Commands
```bash
# List active swap devices
sigswap list

# Add swap file
sigswap add /swapfile

# Remove swap device
sigswap remove /dev/sda2

# Show swap statistics
sigswap stats
```

## Performance Considerations

### Swap Thrashing
When the system is continuously swapping pages in and out, performance degrades significantly. SigmaOS detects thrashing using:
- Page fault rate monitoring
- Swap-in/out ratio tracking
- Automatic OOM killer activation when thrashing detected

### Swap Optimization
- **Pre-swapping**: Swap out pages before they're needed
- **Swappiness**: Control how aggressively the kernel swaps (0-100)
- **Swap locality**: Group related pages together on disk

## Troubleshooting

### Low Swap Performance
If swap performance is poor:
1. Check swap device: `sigswap list`
2. Verify swap priority configuration
3. Consider adding faster swap storage (SSD)
4. Reduce swappiness: `sysctl vm.swappiness=10`

### Out of Memory
If the system runs out of memory:
1. Check swap status: `sigswap stats`
2. Increase swap space
3. Enable zswap for compressed swap
4. Check for memory leaks: `sigtop`

---

**[Performance & Kernel](Category-Performance)** | **[Memory Management](Memory-Management)** | **[System Administration](System-Administration)**
