//! SigmaOS: Hardened Kernel Memory Allocator
//! Built in Zig — freestanding, zero stdlib, zero external components.
//! Employs security features: Block Headers, Heap Canary validation, Double-Free detection.

const SigmaU8 = u8;
const SigmaU16 = u16;
const SigmaU32 = u32;
const SigmaU64 = u64;
const SigmaUsize = usize;
const SigmaBool = bool;

const HEAP_CANARY: SigmaU64 = 0xDEADC0DEBEEFCAFE;

pub const AllocHeader = extern struct {
    canary: SigmaU64,
    size: SigmaUsize,
    is_free: SigmaBool,
    magic: SigmaU32,
};

pub const HardenedAllocator = struct {
    heap_start: SigmaU64,
    heap_end: SigmaU64,
    initialized: SigmaBool = false,

    const Self = @This();

    pub fn init(self: *Self, start: SigmaU64, end: SigmaU64) void {
        self.heap_start = start;
        self.heap_end = end;
        self.initialized = true;

        // Structure the heap with a single large free block
        const first_block = @intToPtr(*mut AllocHeader, start);
        first_block.* = .{
            .canary = HEAP_CANARY,
            .size = (end - start) - @sizeOf(AllocHeader),
            .is_free = true,
            .magic = 0x516D_A051, // "SIGMAOS1"
        };
    }

    pub fn allocate(self: *Self, size: SigmaUsize) ?*mut anyopaque {
        if (!self.initialized) return null;

        // Align size to 16 bytes
        const aligned_size = (size + 15) & ~@as(SigmaUsize, 15);
        var current_addr = self.heap_start;

        while (current_addr < self.heap_end) {
            const header = @intToPtr(*mut AllocHeader, current_addr);

            // Security check: Verify header canary to catch heap smashing
            if (header.canary != HEAP_CANARY or header.magic != 0x516D_A051) {
                // Panic: Heap corruption detected
                return null;
            }

            if (header.is_free and header.size >= aligned_size) {
                // Can we split this block?
                const min_split_size = aligned_size + @sizeOf(AllocHeader) + 16;
                if (header.size >= min_split_size) {
                    const new_block_addr = current_addr + @sizeOf(AllocHeader) + aligned_size;
                    const next_block = @intToPtr(*mut AllocHeader, new_block_addr);
                    next_block.* = .{
                        .canary = HEAP_CANARY,
                        .size = header.size - aligned_size - @sizeOf(AllocHeader),
                        .is_free = true,
                        .magic = 0x516D_A051,
                    };
                    header.size = aligned_size;
                }

                header.is_free = false;
                // Return pointer to payload (just after the header)
                return @intToPtr(*mut anyopaque, current_addr + @sizeOf(AllocHeader));
            }

            current_addr += @sizeOf(AllocHeader) + header.size;
        }

        return null; // Out of memory
    }

    pub fn free(self: *Self, ptr: ?*anyopaque) void {
        if (ptr == null or !self.initialized) return;

        const payload_addr = @ptrToInt(ptr);
        const header_addr = payload_addr - @sizeOf(AllocHeader);
        const header = @intToPtr(*mut AllocHeader, header_addr);

        // Security check: Validate header integrity before freeing
        if (header.canary != HEAP_CANARY or header.magic != 0x516D_A051) {
            // Panic: double free or heap corruption detected
            return;
        }

        if (header.is_free) {
            // Double-free warning/panic
            return;
        }

        header.is_free = true;
        
        // Coalescing: simple sweep over the heap to merge adjacent free blocks
        self.coalesce();
    }

    fn coalesce(self: *Self) void {
        var current_addr = self.heap_start;
        while (current_addr < self.heap_end) {
            const current = @intToPtr(*mut AllocHeader, current_addr);
            if (current.canary != HEAP_CANARY) break;

            const next_addr = current_addr + @sizeOf(AllocHeader) + current.size;
            if (next_addr >= self.heap_end) break;

            const next = @intToPtr(*mut AllocHeader, next_addr);
            if (next.canary != HEAP_CANARY) break;

            if (current.is_free and next.is_free) {
                current.size += @sizeOf(AllocHeader) + next.size;
                // Repeat with the same address to merge further if possible
                continue;
            }
            current_addr = next_addr;
        }
    }
};

var global_allocator = HardenedAllocator{
    .heap_start = 0,
    .heap_end = 0,
};

export fn sigma_allocator_init(start: SigmaU64, end: SigmaU64) void {
    global_allocator.init(start, end);
}

export fn sigma_malloc(size: SigmaUsize) ?*anyopaque {
    return global_allocator.allocate(size);
}

export fn sigma_free(ptr: ?*anyopaque) void {
    global_allocator.free(ptr);
}
