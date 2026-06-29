// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: x86_64 Paging (Zig, no libc, no stdlib)
//! Replaces: arch/x86_64/paging.c
//! =========================================================================

const PAGE_SIZE: usize = 4096;
const PAGE_PRESENT: u64 = 1 << 0;
const PAGE_WRITABLE: u64 = 1 << 1;
const PAGE_USER: u64 = 1 << 2;
const PAGE_HUGE: u64 = 1 << 7;

/// Represents a single 64-bit page table entry
pub const PageEntry = struct {
    raw: u64,

    pub fn new(phys_addr: u64, flags: u64) PageEntry {
        return PageEntry{ .raw = (phys_addr & 0x000F_FFFF_FFFF_F000) | flags };
    }

    pub fn set_present(self: *PageEntry) void {
        self.raw |= PAGE_PRESENT;
    }

    pub fn set_writable(self: *PageEntry) void {
        self.raw |= PAGE_WRITABLE;
    }

    pub fn physical_address(self: PageEntry) u64 {
        return self.raw & 0x000F_FFFF_FFFF_F000;
    }

    pub fn is_present(self: PageEntry) bool {
        return (self.raw & PAGE_PRESENT) != 0;
    }
};

/// A 512-entry page table (4KB, one page)
pub const PageTable = struct {
    entries: [512]PageEntry,

    pub fn new() PageTable {
        return PageTable{ .entries = [_]PageEntry{PageEntry{ .raw = 0 }} ** 512 };
    }

    /// Map a virtual page index to a physical address with given flags
    pub fn map(self: *PageTable, index: usize, phys_addr: u64, flags: u64) void {
        if (index < 512) {
            self.entries[index] = PageEntry.new(phys_addr, flags | PAGE_PRESENT);
        }
    }

    /// Unmap a page entry
    pub fn unmap(self: *PageTable, index: usize) void {
        if (index < 512) {
            self.entries[index].raw = 0;
        }
    }
};

/// Sovereign Frame Allocator — manages physical page frames
pub const FrameAllocator = struct {
    base: u64,
    total_frames: usize,
    next_free: usize,

    pub fn new(base: u64, total_frames: usize) FrameAllocator {
        return FrameAllocator{
            .base = base,
            .total_frames = total_frames,
            .next_free = 0,
        };
    }

    /// Allocate the next free physical frame
    pub fn allocate(self: *FrameAllocator) ?u64 {
        if (self.next_free >= self.total_frames) {
            return null;
        }
        const frame = self.base + @as(u64, self.next_free) * PAGE_SIZE;
        self.next_free += 1;
        return frame;
    }

    pub fn frames_used(self: *const FrameAllocator) usize {
        return self.next_free;
    }

    pub fn frames_free(self: *const FrameAllocator) usize {
        return self.total_frames - self.next_free;
    }
};
