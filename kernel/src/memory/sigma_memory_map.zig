// sigma_memory_map.zig — Sigma Memory Map: Page Frame Allocator (Zig)
// Language: Zig (freestanding, no stdlib)
// OOP: FrameAllocator "interface" via vtable; BitmapAllocator (concrete)
// Specification: .kiro/specs/sigmaos-roadmap/design.md (Memory subsystem)

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 1. Constants
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

const PAGE_SIZE:   usize = 4096;
const MAX_FRAMES:  usize = 1024 * 1024;  // 4 GiB addressable @ 4 KiB pages
const BITMAP_WORDS: usize = MAX_FRAMES / 64;  // 64 frames per u64 word

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 2. PhysAddr / VirtAddr newtype wrappers
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

pub const PhysAddr = struct {
    inner: u64,

    pub fn fromFrame(frame: usize) PhysAddr {
        return PhysAddr{ .inner = @as(u64, @intCast(frame)) * PAGE_SIZE };
    }

    pub fn toFrame(self: PhysAddr) usize {
        return @as(usize, @intCast(self.inner / PAGE_SIZE));
    }

    pub fn asU64(self: PhysAddr) u64 { return self.inner; }
};

pub const VirtAddr = struct {
    inner: u64,

    pub fn fromRaw(v: u64) VirtAddr { return VirtAddr{ .inner = v }; }
    pub fn asU64(self: VirtAddr) u64 { return self.inner; }
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 3. MemoryRegion — describes a usable RAM region
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

pub const MemoryRegionKind = enum(u8) {
    Usable    = 0,
    Reserved  = 1,
    AcpiReclaimable = 2,
    AcpiNvs   = 3,
    BadMemory = 4,
};

pub const MemoryRegion = struct {
    base:  PhysAddr,
    len:   u64,
    kind:  MemoryRegionKind,
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 4. FrameAllocator vtable (abstract interface)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

pub const AllocError = error{ OutOfMemory, InvalidFrame, AlreadyFree };

pub const FrameAllocatorVtable = struct {
    allocFn:  *const fn (ctx: *anyopaque) AllocError!PhysAddr,
    freeFn:   *const fn (ctx: *anyopaque, addr: PhysAddr) AllocError!void,
    statsFn:  *const fn (ctx: *anyopaque) FrameStats,
};

pub const FrameStats = struct {
    total_frames: usize,
    free_frames:  usize,
    used_frames:  usize,
};

pub const FrameAllocator = struct {
    ctx:    *anyopaque,
    vtable: *const FrameAllocatorVtable,

    pub fn alloc(self: FrameAllocator) AllocError!PhysAddr {
        return self.vtable.allocFn(self.ctx);
    }
    pub fn free(self: FrameAllocator, addr: PhysAddr) AllocError!void {
        return self.vtable.freeFn(self.ctx, addr);
    }
    pub fn stats(self: FrameAllocator) FrameStats {
        return self.vtable.statsFn(self.ctx);
    }
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 5. BitmapAllocator — concrete frame allocator using bitmap
//        64 frames per u64 word; O(n/64) alloc, O(1) free
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

pub const BitmapAllocator = struct {
    bitmap:       [BITMAP_WORDS]u64,
    total_frames: usize,
    free_frames:  usize,
    base:         PhysAddr,

    pub fn init(base: PhysAddr, total_frames: usize) BitmapAllocator {
        var self = BitmapAllocator{
            .bitmap       = [_]u64{0xFFFFFFFFFFFFFFFF} ** BITMAP_WORDS,
            .total_frames = total_frames,
            .free_frames  = total_frames,
            .base         = base,
        };
        // Mark frames beyond total_frames as used
        const valid_words = total_frames / 64;
        const remainder   = total_frames % 64;
        var i: usize = valid_words;
        while (i < BITMAP_WORDS) : (i += 1) {
            self.bitmap[i] = 0;  // All used (out of range)
        }
        if (remainder > 0 and valid_words < BITMAP_WORDS) {
            self.bitmap[valid_words] = (@as(u64, 1) << @intCast(remainder)) -% 1;
        }
        return self;
    }

    /// Mark a physical address range as reserved (used by firmware).
    pub fn markReserved(self: *BitmapAllocator, addr: PhysAddr, pages: usize) void {
        const start_frame = addr.toFrame();
        var i: usize = 0;
        while (i < pages) : (i += 1) {
            const frame = start_frame + i;
            if (frame >= self.total_frames) break;
            const word  = frame / 64;
            const bit   = @as(u6, @intCast(frame % 64));
            const was_set = (self.bitmap[word] >> bit) & 1;
            self.bitmap[word] &= ~(@as(u64, 1) << bit);
            if (was_set != 0) self.free_frames -%= 1;
        }
    }

    fn allocFrame(ctx: *anyopaque) AllocError!PhysAddr {
        const self: *BitmapAllocator = @ptrCast(@alignCast(ctx));
        if (self.free_frames == 0) return AllocError.OutOfMemory;
        var word: usize = 0;
        while (word < BITMAP_WORDS) : (word += 1) {
            if (self.bitmap[word] != 0) {
                const bit = @ctz(self.bitmap[word]);
                self.bitmap[word] &= ~(@as(u64, 1) << @intCast(bit));
                self.free_frames -= 1;
                const frame_idx = word * 64 + bit;
                return PhysAddr.fromFrame(frame_idx);
            }
        }
        return AllocError.OutOfMemory;
    }

    fn freeFrame(ctx: *anyopaque, addr: PhysAddr) AllocError!void {
        const self: *BitmapAllocator = @ptrCast(@alignCast(ctx));
        const frame = addr.toFrame();
        if (frame >= self.total_frames) return AllocError.InvalidFrame;
        const word = frame / 64;
        const bit  = @as(u6, @intCast(frame % 64));
        if ((self.bitmap[word] >> bit) & 1 != 0) return AllocError.AlreadyFree;
        self.bitmap[word] |= (@as(u64, 1) << bit);
        self.free_frames += 1;
    }

    fn getStats(ctx: *anyopaque) FrameStats {
        const self: *BitmapAllocator = @ptrCast(@alignCast(ctx));
        return FrameStats{
            .total_frames = self.total_frames,
            .free_frames  = self.free_frames,
            .used_frames  = self.total_frames - self.free_frames,
        };
    }

    pub const vtable: FrameAllocatorVtable = .{
        .allocFn  = allocFrame,
        .freeFn   = freeFrame,
        .statsFn  = getStats,
    };

    pub fn asAllocator(self: *BitmapAllocator) FrameAllocator {
        return FrameAllocator{
            .ctx    = @ptrCast(self),
            .vtable = &vtable,
        };
    }
};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 6. Page Table Abstraction (4-level x86-64 paging)
//        Entries stored as u64; flags as packed struct
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

pub const PageFlags = packed struct(u64) {
    present:      bool  = false,
    writable:     bool  = false,
    user_access:  bool  = false,
    write_through: bool = false,
    no_cache:     bool  = false,
    accessed:     bool  = false,
    dirty:        bool  = false,
    huge_page:    bool  = false,
    global:       bool  = false,
    _avl:         u3    = 0,
    addr_bits:    u40   = 0,
    _reserved:    u11   = 0,
    no_execute:   bool  = false,
};

/// Extract physical address from page table entry
pub fn pteToPhys(pte: u64) PhysAddr {
    return PhysAddr{ .inner = pte & 0x000F_FFFF_FFFF_F000 };
}

/// Build a page table entry from flags + physical address
pub fn buildPte(addr: PhysAddr, flags: PageFlags) u64 {
    var f = @as(u64, @bitCast(flags));
    f &= 0xFFF; // Keep low 12 bits (flags)
    return (addr.inner & 0x000F_FFFF_FFFF_F000) | f;
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//  § 7. Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

test "BitmapAllocator: alloc and free" {
    const std = @import("std");
    var balloc = BitmapAllocator.init(PhysAddr{ .inner = 0x100000 }, 256);
    const fa = balloc.asAllocator();

    const s0 = fa.stats();
    try std.testing.expect(s0.free_frames == 256);

    const p1 = try fa.alloc();
    try std.testing.expect(p1.asU64() != 0);
    try std.testing.expect(fa.stats().free_frames == 255);

    try fa.free(p1);
    try std.testing.expect(fa.stats().free_frames == 256);
}

test "BitmapAllocator: mark reserved reduces free count" {
    const std = @import("std");
    var balloc = BitmapAllocator.init(PhysAddr{ .inner = 0 }, 128);
    balloc.markReserved(PhysAddr{ .inner = 0 }, 10);
    try std.testing.expect(balloc.free_frames == 118);
}

test "PageFlags: buildPte and pteToPhys roundtrip" {
    const std = @import("std");
    const addr = PhysAddr{ .inner = 0x0000_8000_0000_0000 };
    const flags = PageFlags{ .present = true, .writable = true };
    const pte = buildPte(addr, flags);
    const recovered = pteToPhys(pte);
    try std.testing.expect(recovered.asU64() == addr.asU64());
}
