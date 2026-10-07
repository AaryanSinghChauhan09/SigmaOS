// SigmaOS Low-Level DMA Controller & PRDT Allocator
// Fast, zero-allocation memory alignment & Physical Region Descriptor generator for AHCI/NVMe
const std = @import("std");

pub const DmaBufferError = error{
    MisalignedBuffer,
    BufferTooLarge,
    PrdtOverflow,
    InvalidPageBoundary,
};

pub const PrdtEntryPacked = packed struct {
    data_base_address: u64,
    reserved0: u32,
    byte_count_and_ioc: u32,
};

pub const DmaEngine = struct {
    page_size: usize = 4096,
    max_prdt_entries: usize = 32,

    pub fn validate_alignment(addr: usize, alignment: usize) bool {
        return (addr & (alignment - 1)) == 0;
    }

    pub fn build_prdt(
        self: *const DmaEngine,
        buffer_phys_addr: u64,
        total_bytes: usize,
        out_prdt: []PrdtEntryPacked,
    ) DmaBufferError!usize {
        if (!validate_alignment(buffer_phys_addr, 4)) {
            return DmaBufferError.MisalignedBuffer;
        }

        var remaining = total_bytes;
        var cur_addr = buffer_phys_addr;
        var entry_count: usize = 0;

        while (remaining > 0) {
            if (entry_count >= out_prdt.len or entry_count >= self.max_prdt_entries) {
                return DmaBufferError.PrdtOverflow;
            }

            const chunk_size = @min(remaining, 4 * 1024 * 1024); // 4MB maximum per PRDT entry
            const is_last = (chunk_size == remaining);
            const ioc_bit: u32 = if (is_last) (1 << 31) else 0;
            const byte_count_field = (@as(u32, @intCast(chunk_size - 1)) & 0x003F_FFFF) | ioc_bit;

            out_prdt[entry_count] = PrdtEntryPacked{
                .data_base_address = cur_addr,
                .reserved0 = 0,
                .byte_count_and_ioc = byte_count_field,
            };

            cur_addr += chunk_size;
            remaining -= chunk_size;
            entry_count += 1;
        }

        return entry_count;
    }
};

pub fn main() !void {
    const engine = DmaEngine{};
    var entries: [16]PrdtEntryPacked = undefined;
    const count = try engine.build_prdt(0x10000000, 65536, &entries);
    std.debug.print("Generated {} PRDT entries for 64KB transfer\n", .{count});
}
