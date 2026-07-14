//! Custom Memory Allocator for SigmaOS
//! Implements memory allocation without relying on std
//! Uses buddy system algorithm for efficient memory management

const BLOCK_SIZE: usize = 4096;
const MAX_ORDER: usize = 10;

/// Memory block structure
pub const MemoryBlock = struct {
    size: usize,
    used: bool,
    next: ?*MemoryBlock,
    prev: ?*MemoryBlock,
};

/// Buddy system memory allocator
pub const BuddyAllocator = struct {
    free_lists: [?*MemoryBlock][MAX_ORDER + 1],
    total_memory: usize,
    used_memory: usize,
    heap_start: [*]u8,
    heap_size: usize,
};

pub fn initAllocator(allocator: *BuddyAllocator, heap_start: [*]u8, heap_size: usize) void {
    allocator.free_lists = [_]null;
    allocator.total_memory = heap_size;
    allocator.used_memory = 0;
    allocator.heap_start = heap_start;
    allocator.heap_size = heap_size;

    // Initialize free lists
    var remaining = heap_size;
    var current = heap_start;

    while (remaining >= BLOCK_SIZE) {
        var order = calculateOrder(remaining);
        var block_size = BLOCK_SIZE * (1 << order);

        var block = @ptrCast(*MemoryBlock, current);
        block.size = block_size;
        block.used = false;
        block.next = null;
        block.prev = null;

        addToFreeList(allocator, block, order);

        current = current + block_size;
        remaining -= block_size;
    }
}

fn calculateOrder(size: usize) usize {
    var order: usize = 0;
    var block_size = BLOCK_SIZE;
    while (block_size < size and order < MAX_ORDER) {
        block_size *= 2;
        order += 1;
    }
    return order;
}

fn addToFreeList(allocator: *BuddyAllocator, block: *MemoryBlock, order: usize) void {
    block.next = allocator.free_lists[order];
    if (allocator.free_lists[order]) |head| {
        head.prev = block;
    }
    allocator.free_lists[order] = block;
}

fn removeFromFreeList(allocator: *BuddyAllocator, block: *MemoryBlock, order: usize) void {
    const prev = block.prev;
    const next = block.next;

    if (prev) |prev_block| {
        prev_block.next = next;
    } else {
        allocator.free_lists[order] = next;
    }

    if (next) |next_block| {
        next_block.prev = prev;
    }

    block.prev = null;
    block.next = null;
}

pub fn allocate(allocator: *BuddyAllocator, size: usize) ?[*]u8 {
    const aligned_size = (size + @sizeOf(MemoryBlock) + 15) & ~@as(usize, 15);
    const order = calculateOrder(aligned_size);

    // Find a free block of sufficient size
    var current_order = order;
    var block: ?*MemoryBlock = null;

    while (current_order <= MAX_ORDER) {
        if (allocator.free_lists[current_order]) |free_block| {
            block = free_block;
            break;
        }
        current_order += 1;
    }

    if (block == null) {
        return null;
    }

    removeFromFreeList(allocator, block, current_order);

    // Split block if necessary
    while (current_order > order) {
        current_order -= 1;
        const split_size = BLOCK_SIZE * (1 << current_order);
        const buddy = @ptrCast(*MemoryBlock, @intToPtr([*]u8, block) + split_size);

        buddy.size = split_size;
        buddy.used = false;
        buddy.next = null;
        buddy.prev = null;

        addToFreeList(allocator, buddy, current_order);

        block.size = split_size;
    }

    block.used = true;
    allocator.used_memory += block.size;

    return @intToPtr([*]u8, block) + @sizeOf(MemoryBlock);
}

pub fn deallocate(allocator: *BuddyAllocator, ptr: [*]u8) void {
    const block = @ptrCast(*MemoryBlock, ptr - @sizeOf(MemoryBlock));

    if (!block.used) {
        return; // Already freed
    }

    block.used = false;
    allocator.used_memory -= block.size;

    const size = block.size;
    const order = calculateOrder(size);

    // Try to merge with buddy
    var current_block = block;
    var current_order = order;

    while (current_order < MAX_ORDER) {
        const buddy = getBuddy(current_block, current_order);

        if (buddy.used or buddy.size != current_block.size) {
            break;
        }

        // Remove buddy from free list
        removeFromFreeList(allocator, buddy, current_order);

        // Merge blocks
        if (current_block < buddy) {
            current_block.size *= 2;
        } else {
            buddy.size *= 2;
            current_block = buddy;
        }

        current_order += 1;
    }

    addToFreeList(allocator, current_block, current_order);
}

fn getBuddy(block: *MemoryBlock, order: usize) *MemoryBlock {
    const block_addr = @intFromPtr(block);
    const size = BLOCK_SIZE * (1 << order);
    const buddy_addr = block_addr ^ size;
    return @ptrFromInt(MemoryBlock, buddy_addr);
}

pub fn getStats(allocator: *BuddyAllocator) MemoryStats {
    return MemoryStats{
        .total = allocator.total_memory,
        .used = allocator.used_memory,
        .free = allocator.total_memory - allocator.used_memory,
    };
}

pub const MemoryStats = struct {
    total: usize,
    used: usize,
    free: usize,
};

// External allocator functions (would be provided by kernel)
extern fn kernelAlloc(size: usize) [*]u8;
extern fn kernelFree(ptr: [*]u8);
