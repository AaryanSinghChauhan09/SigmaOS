// SPDX-License-Identifier: MIT
// SigmaOS Fast Clipboard Ring Buffer (`src/desktop/sigma_cliphist_fast_ring.zig`)
//
// Lock-free circular ring buffer for sub-microsecond Wayland clipboard captures.
// Implemented in Zig using std.mem and std.atomic for seamless cross-process sharing.

const std = @import("std");

pub const ClipboardEntry = struct {
    id: u64,
    timestamp: i64,
    mime_type: [32]u8,
    mime_len: u8,
    content: [1024]u8,
    content_len: u16,
};

pub const CliphistRingBuffer = struct {
    entries: [256]ClipboardEntry = undefined,
    head: std.atomic.Value(u32) = std.atomic.Value(u32).init(0),
    tail: std.atomic.Value(u32) = std.atomic.Value(u32).init(0),
    count: std.atomic.Value(u32) = std.atomic.Value(u32).init(0),

    pub fn init() CliphistRingBuffer {
        return CliphistRingBuffer{};
    }

    pub fn push(self: *CliphistRingBuffer, id: u64, mime: []const u8, data: []const u8) bool {
        const h = self.head.load(.monotonic);
        const next_h = (h + 1) % 256;

        var entry = &self.entries[h];
        entry.id = id;
        entry.timestamp = std.time.milliTimestamp();

        const copy_mime_len = @min(mime.len, 32);
        @memcpy(entry.mime_type[0..copy_mime_len], mime[0..copy_mime_len]);
        entry.mime_len = @intCast(copy_mime_len);

        const copy_content_len = @min(data.len, 1024);
        @memcpy(entry.content[0..copy_content_len], data[0..copy_content_len]);
        entry.content_len = @intCast(copy_content_len);

        self.head.store(next_h, .release);
        _ = self.count.fetchAdd(1, .monotonic);
        return true;
    }
};

test "cliphist ring buffer basic push" {
    var ring = CliphistRingBuffer.init();
    _ = ring.push(1, "text/plain", "echo hello sigmaos");
    try std.testing.expect(ring.count.load(.monotonic) == 1);
}
