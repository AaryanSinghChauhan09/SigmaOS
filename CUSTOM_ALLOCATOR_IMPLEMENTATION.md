# Custom Allocator Implementation Guide for SigmaOS

## Overview

SigmaOS implements custom memory allocators to reduce dependency on standard library allocators and provide better control over memory management in the kernel space. This guide explains the design and implementation of custom allocators.

## Design Principles

1.  **No Std Dependency**: All allocators work without the standard library
2.  **Deterministic Behavior**: Predictable allocation/deallocation timing
3.  **Fragmentation Control**: Strategies to minimize memory fragmentation
4.  **Security**: Protection against allocator-based attacks
5.  **Performance**: Optimized for kernel workloads

## Allocator Types

### 1. Bump Allocator

Simple, fast allocator for short-lived allocations:

```rust
pub struct BumpAllocator {
    start: usize,
    current: usize,
    end: usize,
}

impl BumpAllocator {
    pub const fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            current: start,
            end: start + size,
        }
    }
    
    pub fn allocate(&mut self, size: usize, align: usize) -> Result<usize, AllocError> {
        let aligned_current = align_up(self.current, align);
        let new_current = aligned_current + size;
        
        if new_current > self.end {
            return Err(AllocError::OutOfMemory);
        }
        
        self.current = new_current;
        Ok(aligned_current)
    }
    
    pub fn reset(&mut self) {
        self.current = self.start;
    }
}

const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
```

### 2. Slab Allocator

Efficient for fixed-size allocations:

```rust
pub struct SlabAllocator<T> {
    free_list: Vec<*mut T>,
    objects: Vec<T>,
}

impl<T> SlabAllocator<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            free_list: Vec::with_capacity(capacity),
            objects: Vec::with_capacity(capacity),
        }
    }
    
    pub fn allocate(&mut self) -> Result<*mut T, AllocError> {
        if let Some(ptr) = self.free_list.pop() {
            return Ok(ptr);
        }
        
        if self.objects.len() >= self.objects.capacity() {
            return Err(AllocError::OutOfMemory);
        }
        
        self.objects.push(T::default());
        Ok(self.objects.last_mut().unwrap() as *mut T)
    }
    
    pub fn deallocate(&mut self, ptr: *mut T) {
        self.free_list.push(ptr);
    }
}
```

### 3. Buddy Allocator

For managing larger memory blocks:

```rust
pub struct BuddyAllocator {
    memory_base: usize,
    memory_size: usize,
    max_order: usize,
    free_lists: Vec<Vec<usize>>,
}

impl BuddyAllocator {
    pub fn new(base: usize, size: usize) -> Self {
        let max_order = size.trailing_zeros() as usize;
        let mut free_lists = vec![Vec::new(); max_order + 1];
        
        // Initialize with entire memory as one free block
        free_lists[max_order].push(base);
        
        Self {
            memory_base: base,
            memory_size: size,
            max_order,
            free_lists,
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> Result<usize, AllocError> {
        let order = self.size_to_order(size);
        
        // Find smallest free block that can satisfy request
        for current_order in order..=self.max_order {
            if let Some(block) = self.free_lists[current_order].pop() {
                // Split blocks until we reach desired order
                for split_order in (order..current_order).rev() {
                    let buddy = block + (1 << split_order);
                    self.free_lists[split_order].push(buddy);
                }
                return Ok(block);
            }
        }
        
        Err(AllocError::OutOfMemory)
    }
    
    pub fn deallocate(&mut self, ptr: usize, size: usize) {
        let mut order = self.size_to_order(size);
        let mut block = ptr;
        
        // Try to merge with buddy
        while order < self.max_order {
            let buddy = block ^ (1 << order);
            
            if let Some(pos) = self.free_lists[order].iter().position(|&b| b == buddy) {
                self.free_lists[order].remove(pos);
                block = block.min(buddy);
                order += 1;
            } else {
                break;
            }
        }
        
        self.free_lists[order].push(block);
    }
    
    fn size_to_order(&self, size: usize) -> usize {
        let aligned_size = size.next_power_of_two();
        aligned_size.trailing_zeros() as usize
    }
}
```

## Memory Pool Allocator

For specialized memory pools:

```rust
pub struct MemoryPool {
    pools: Vec<MemoryPoolLevel>,
}

pub struct MemoryPoolLevel {
    block_size: usize,
    free_blocks: Vec<usize>,
    total_blocks: usize,
}

impl MemoryPool {
    pub fn new() -> Self {
        Self {
            pools: vec![
                MemoryPoolLevel::new(16, 1024),
                MemoryPoolLevel::new(32, 1024),
                MemoryPoolLevel::new(64, 512),
                MemoryPoolLevel::new(128, 512),
                MemoryPoolLevel::new(256, 256),
                MemoryPoolLevel::new(512, 256),
                MemoryPoolLevel::new(1024, 128),
                MemoryPoolLevel::new(2048, 64),
            ],
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> Result<usize, AllocError> {
        for pool in &mut self.pools {
            if pool.block_size >= size {
                return pool.allocate();
            }
        }
        Err(AllocError::InvalidSize)
    }
    
    pub fn deallocate(&mut self, ptr: usize, size: usize) {
        for pool in &mut self.pools {
            if pool.block_size >= size {
                pool.deallocate(ptr);
                return;
            }
        }
    }
}

impl MemoryPoolLevel {
    fn new(block_size: usize, count: usize) -> Self {
        let mut free_blocks = Vec::with_capacity(count);
        // Initialize with free blocks
        for i in 0..count {
            free_blocks.push(i * block_size);
        }
        
        Self {
            block_size,
            free_blocks,
            total_blocks: count,
        }
    }
    
    fn allocate(&mut self) -> Result<usize, AllocError> {
        self.free_blocks.pop().ok_or(AllocError::OutOfMemory)
    }
    
    fn deallocate(&mut self, ptr: usize) {
        self.free_blocks.push(ptr);
    }
}
```

## Kernel-Specific Allocators

### Physical Memory Allocator

```rust
pub struct PhysicalMemoryAllocator {
    page_bitmap: Vec<u64>,
    total_pages: usize,
    free_pages: usize,
}

impl PhysicalMemoryAllocator {
    pub fn new(base: usize, size: usize) -> Self {
        let total_pages = size / PAGE_SIZE;
        let bitmap_size = (total_pages + 63) / 64;
        
        Self {
            page_bitmap: vec![0; bitmap_size],
            total_pages,
            free_pages: total_pages,
        }
    }
    
    pub fn allocate_page(&mut self) -> Result<usize, AllocError> {
        for (idx, &bitmap) in self.page_bitmap.iter().enumerate() {
            if bitmap != !0 {
                // Find first free bit
                let bit_offset = (!bitmap).trailing_zeros() as usize;
                let page_idx = idx * 64 + bit_offset;
                
                if page_idx < self.total_pages {
                    self.page_bitmap[idx] |= 1 << bit_offset;
                    self.free_pages -= 1;
                    return Ok(page_idx * PAGE_SIZE);
                }
            }
        }
        Err(AllocError::OutOfMemory)
    }
    
    pub fn free_page(&mut self, addr: usize) {
        let page_idx = addr / PAGE_SIZE;
        let idx = page_idx / 64;
        let bit_offset = page_idx % 64;
        
        self.page_bitmap[idx] &= !(1 << bit_offset);
        self.free_pages += 1;
    }
}
```

### Virtual Memory Allocator

```rust
pub struct VirtualMemoryAllocator {
    regions: Vec<VmRegion>,
    next_vaddr: usize,
}

pub struct VmRegion {
    start: usize,
    size: usize,
    is_free: bool,
}

impl VirtualMemoryAllocator {
    pub fn new() -> Self {
        Self {
            regions: vec![
                VmRegion {
                    start: KERNEL_VADDR_START,
                    size: KERNEL_VADDR_SIZE,
                    is_free: true,
                }
            ],
            next_vaddr: KERNEL_VADDR_START,
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> Result<usize, AllocError> {
        let aligned_size = align_up(size, PAGE_SIZE);
        
        // First-fit allocation
        for region in &mut self.regions {
            if region.is_free && region.size >= aligned_size {
                region.is_free = false;
                
                // Split region if there's remaining space
                if region.size > aligned_size {
                    let remaining = region.size - aligned_size;
                    region.size = aligned_size;
                    
                    self.regions.push(VmRegion {
                        start: region.start + aligned_size,
                        size: remaining,
                        is_free: true,
                    });
                }
                
                return Ok(region.start);
            }
        }
        
        Err(AllocError::OutOfMemory)
    }
    
    pub fn free(&mut self, addr: usize) {
        for region in &mut self.regions {
            if region.start == addr && !region.is_free {
                region.is_free = true;
                self.coalesce_regions();
                return;
            }
        }
    }
    
    fn coalesce_regions(&mut self) {
        self.regions.sort_by_key(|r| r.start);
        
        let mut i = 0;
        while i < self.regions.len() - 1 {
            if self.regions[i].is_free && self.regions[i + 1].is_free {
                if self.regions[i].start + self.regions[i].size == self.regions[i + 1].start {
                    self.regions[i].size += self.regions[i + 1].size;
                    self.regions.remove(i + 1);
                    continue;
                }
            }
            i += 1;
        }
    }
}
```

## Security Features

### Guard Pages

```rust
pub struct SecureAllocator {
    inner: Box<dyn Allocator>,
    guard_page_size: usize,
}

impl SecureAllocator {
    pub fn new(inner: Box<dyn Allocator>, guard_size: usize) -> Self {
        Self {
            inner,
            guard_page_size: guard_size,
        }
    }
    
    pub fn allocate_secure(&mut self, size: usize) -> Result<usize, AllocError> {
        let total_size = size + 2 * self.guard_page_size;
        let base = self.inner.allocate(total_size)?;
        
        // Place guard pages before and after allocation
        self.set_guard_page(base)?;
        self.set_guard_page(base + self.guard_page_size + size)?;
        
        Ok(base + self.guard_page_size)
    }
    
    fn set_guard_page(&self, addr: usize) -> Result<(), AllocError> {
        // Mark page as non-accessible
        Ok(())
    }
}
```

### Randomization

```rust
pub struct RandomizedAllocator {
    inner: Box<dyn Allocator>,
    rng: XorShiftRng,
}

impl RandomizedAllocator {
    pub fn new(inner: Box<dyn Allocator>) -> Self {
        Self {
            inner,
            rng: XorShiftRng::new(),
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> Result<usize, AllocError> {
        let base = self.inner.allocate(size)?;
        let offset = self.rng.next() % 16; // Random offset up to 16 bytes
        Ok(base + offset)
    }
}
```

## Integration with Collections

### Custom Vec Implementation

```rust
pub struct SigmaVec<T> {
    ptr: *mut T,
    capacity: usize,
    len: usize,
    allocator: &'static mut dyn Allocator,
}

impl<T> SigmaVec<T> {
    pub fn new(allocator: &'static mut dyn Allocator) -> Self {
        Self {
            ptr: ptr::null_mut(),
            capacity: 0,
            len: 0,
            allocator,
        }
    }
    
    pub fn push(&mut self, item: T) {
        if self.len == self.capacity {
            self.grow();
        }
        
        unsafe {
            ptr::write(self.ptr.add(self.len), item);
        }
        self.len += 1;
    }
    
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_size = new_capacity * core::mem::size_of::<T>();
        
        let new_ptr = self.allocator.allocate(new_size).unwrap() as *mut T;
        
        unsafe {
            if self.capacity > 0 {
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
                self.allocator.deallocate(self.ptr as usize, self.capacity * core::mem::size_of::<T>());
            }
        }
        
        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}
```

## Performance Optimization

### Cache-Friendly Allocation

```rust
pub struct CacheFriendlyAllocator {
    slab_allocators: [SlabAllocator<u8>; 8], // For sizes 2^0 to 2^7
    fallback: Box<dyn Allocator>,
}

impl CacheFriendlyAllocator {
    pub fn allocate(&mut self, size: usize) -> Result<usize, AllocError> {
        if size <= 128 {
            let index = (size - 1).next_power_of_two().trailing_zeros() as usize;
            return self.slab_allocators[index].allocate().map(|p| p as usize);
        }
        self.fallback.allocate(size)
    }
}
```

## Testing and Validation

### Allocator Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bump_allocator() {
        let mut allocator = BumpAllocator::new(0x1000, 0x1000);
        
        let ptr1 = allocator.allocate(16, 8).unwrap();
        let ptr2 = allocator.allocate(32, 8).unwrap();
        
        assert!(ptr2 > ptr1);
        assert_eq!(allocator.allocate(0x2000, 8), Err(AllocError::OutOfMemory));
    }
    
    #[test]
    fn test_slab_allocator() {
        let mut allocator: SlabAllocator<u64> = SlabAllocator::new(10);
        
        let ptr1 = allocator.allocate().unwrap();
        let ptr2 = allocator.allocate().unwrap();
        
        assert_ne!(ptr1, ptr2);
        
        allocator.deallocate(ptr1);
        let ptr3 = allocator.allocate().unwrap();
        
        assert_eq!(ptr1, ptr3);
    }
}
```

## Best Practices

1.  **Choose the Right Allocator**: Use bump allocators for temporary allocations, slab allocators for fixed-size objects
2.  **Minimize Fragmentation**: Prefer slab allocators for frequent small allocations
3.  **Monitor Usage**: Track allocation patterns to optimize allocator choice
4.  **Error Handling**: Always handle allocation failures gracefully
5.  **Memory Safety**: Ensure proper cleanup even in error paths

## Troubleshooting

### Out of Memory Errors

```rust
// Enable detailed allocation tracking
let mut allocator = TrackingAllocator::new(inner_allocator);

// Check allocation statistics
let stats = allocator.get_statistics();
println!("Allocations: {}", stats.total_allocations);
println!("Deallocations: {}", stats.total_deallocations);
println!("Current usage: {}", stats.current_usage);
```

### Fragmentation Issues

```rust
// Use memory pool for better fragmentation control
let mut pool = MemoryPool::new();

// Periodically defragment
pool.defragment();
```

## References

*   [The Slab Allocator](http://www.usenix.org/publications/library/proceedings/newor94/full_papers/bonwick.pdf)
*   [Buddy System Memory Allocation](https://en.wikipedia.org/wiki/Buddy_memory_allocation)
*   [jemalloc Design](http://www.canonware.com/jemalloc/design.html)
*   [Linux Kernel Memory Management](https://www.kernel.org/doc/html/latest/mm/index.html)
