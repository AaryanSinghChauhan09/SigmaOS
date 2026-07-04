
//! SigmaOS Slab Allocator
//! A simple slab memory allocator for kernel use.

const std = @import("std");

/// Slab allocator configuration.
pub const SlabConfig = struct {
    object_size: usize,
    slab_size: usize = 4096, // 4KB per slab
};

/// A single slab in the allocator.
const Slab = struct {
    next: ?*Slab = null,
    free_list: ?*anyopaque = null,
    in_use: usize = 0,
};

/// The slab allocator itself.
pub const SlabAllocator = struct {
    config: SlabConfig,
    slabs: ?*Slab = null,
    empty_slabs: ?*Slab = null,
    allocator: std.mem.Allocator,

    /// Creates a new SlabAllocator.
    pub fn init(allocator: std.mem.Allocator, config: SlabConfig) SlabAllocator {
        return SlabAllocator{
            .config = config,
            .allocator = allocator,
        };
    }

    /// Allocates a new object from the slab allocator.
    pub fn alloc(self: *SlabAllocator) !*anyopaque {
        // Try to find a slab with free space
        var current_slab = self.slabs;
        while (current_slab) |slab| {
            if (slab.free_list != null) {
                const obj = slab.free_list;
                slab.free_list = @ptrCast(*anyopaque, @as([*]u8, @ptrCast(obj))[0..self.config.object_size].ptr);
                slab.in_use += 1;
                return obj;
            }
            current_slab = slab.next;
        }

        // No slab found, allocate a new one
        const new_slab = try self.allocator.create(Slab);
        new_slab.* = .{
            .next = self.slabs,
            .free_list = try self.allocator.allocSentinel(u8, self.config.slab_size, undefined),
            .in_use = 0,
        };

        // Initialize free list
        var free_ptr = @as([*]u8, @ptrCast(new_slab.free_list));
        var i: usize = 0;
        while (i < self.config.slab_size - self.config.object_size) {
            const next_ptr = free_ptr + self.config.object_size;
            @as(*?*anyopaque, @ptrCast(free_ptr)).* = @ptrCast(*anyopaque, next_ptr);
            free_ptr = next_ptr;
            i += self.config.object_size;
        }
        @as(*?*anyopaque, @ptrCast(free_ptr)).* = null;

        self.slabs = new_slab;
        return self.alloc(); // Try allocation again with new slab
    }

    /// Deallocates an object back to the slab allocator.
    pub fn dealloc(self: *SlabAllocator, ptr: *anyopaque) void {
        // Find the slab this object belongs to
        var current_slab = self.slabs;
        var prev_slab: ?*Slab = null;
        while (current_slab) |slab| {
            const slab_start = @intFromPtr(slab.free_list);
            const slab_end = slab_start + self.config.slab_size;
            const obj_addr = @intFromPtr(ptr);

            if (obj_addr >= slab_start and obj_addr < slab_end) {
                // Add back to free list
                @as(*?*anyopaque, @ptrCast(ptr)).* = slab.free_list;
                slab.free_list = ptr;
                slab.in_use -= 1;

                // If slab is empty, move to empty_slabs list
                if (slab.in_use == 0) {
                    if (prev_slab) |prev| {
                        prev.next = slab.next;
                    } else {
                        self.slabs = slab.next;
                    }
                    slab.next = self.empty_slabs;
                    self.empty_slabs = slab;
                }
                return;
            }

            prev_slab = current_slab;
            current_slab = slab.next;
        }
    }
};
