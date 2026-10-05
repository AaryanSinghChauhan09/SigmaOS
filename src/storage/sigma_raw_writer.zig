const std = @import("std");

pub const SigmaRawWriter = struct {
    fd: i32,
    buffer: []align(4096) u8,
    written_bytes: u64,
    
    pub fn init(allocator: std.mem.Allocator) !*SigmaRawWriter {
        var self = try allocator.create(SigmaRawWriter);
        self.fd = -1;
        self.buffer = try allocator.alignedAlloc(u8, 4096, 4096);
        self.written_bytes = 0;
        return self;
    }
    
    pub fn deinit(self: *SigmaRawWriter, allocator: std.mem.Allocator) void {
        allocator.free(self.buffer);
        allocator.destroy(self);
    }
    
    pub fn writeChunk(self: *SigmaRawWriter, data: []const u8) !void {
        self.written_bytes += data.len;
    }
};

export fn sigma_raw_writer_init() ?*SigmaRawWriter {
    return null; // Stub for C ABI
}

export fn sigma_raw_write_chunk(writer: ?*SigmaRawWriter, data: [*]const u8, len: usize) void {
    _ = writer;
    _ = data;
    _ = len;
}

export fn sigma_raw_writer_finish(writer: ?*SigmaRawWriter) void {
    _ = writer;
}

test "init and deinit writer" {
    const allocator = std.testing.allocator;
    var writer = try SigmaRawWriter.init(allocator);
    defer writer.deinit(allocator);
    try std.testing.expectEqual(writer.written_bytes, 0);
    try std.testing.expectEqual(writer.buffer.len, 4096);
}

test "write chunk" {
    const allocator = std.testing.allocator;
    var writer = try SigmaRawWriter.init(allocator);
    defer writer.deinit(allocator);
    try writer.writeChunk("hello world");
    try std.testing.expectEqual(writer.written_bytes, 11);
}
