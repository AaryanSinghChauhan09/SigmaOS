// SigmaOS High-Performance Zero-Copy IOMMU DMA Allocator
// Written in Zig for hardware-level memory mapping and scatter-gather rings.
// Inspired by Linux dma-iommu.c and FreeBSD vmm_iommu.c.

const std = @import("std");

pub const DmaDirection = enum {
    Bidirectional,
    ToDevice,
    FromDevice,
    None,
};

pub const ScatterGatherSegment = struct {
    physical_addr: u64,
    iova_addr: u64,
    length: usize,
    direction: DmaDirection,
};

pub const FastIommuAllocator = struct {
    pool_base_iova: u64,
    pool_size: usize,
    allocated_bytes: usize,
    segment_count: usize,
    segments: [256]ScatterGatherSegment,

    pub fn init(base: u64, size: usize) FastIommuAllocator {
        return FastIommuAllocator{
            .pool_base_iova = base,
            .pool_size = size,
            .allocated_bytes = 0,
            .segment_count = 0,
            .segments = undefined,
        };
    }

    pub fn allocate_sg(self: *FastIommuAllocator, phys: u64, len: usize, dir: DmaDirection) ?u64 {
        if (self.allocated_bytes + len > self.pool_size or self.segment_count >= 256) {
            return null;
        }

        const iova = self.pool_base_iova + self.allocated_bytes;
        self.segments[self.segment_count] = ScatterGatherSegment{
            .physical_addr = phys,
            .iova_addr = iova,
            .length = len,
            .direction = dir,
        };

        self.allocated_bytes += len;
        self.segment_count += 1;
        return iova;
    }

    pub fn translate(self: *const FastIommuAllocator, iova: u64) ?u64 {
        var i: usize = 0;
        while (i < self.segment_count) : (i += 1) {
            const seg = self.segments[i];
            if (iova >= seg.iova_addr and iova < seg.iova_addr + seg.length) {
                return seg.physical_addr + (iova - seg.iova_addr);
            }
        }
        return null;
    }
};
