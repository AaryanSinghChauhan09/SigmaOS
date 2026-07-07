// SigmaOS sigma_term — Minimal terminal emulator in Zig
// Replaces apps/sigmaTerm.js (JavaScript).
// Zig: comptime, zero hidden allocations, direct syscall access.
// Compiles to a native binary with no runtime dependency beyond musl libc.

const std = @import("std");
const os  = std.os;
const io  = std.io;
const mem = std.mem;

// ── Terminal Constants ────────────────────────────────────────────────────

const TERM_ROWS: u16 = 24;
const TERM_COLS: u16 = 80;
const HISTORY_SIZE: usize = 256;
const CMD_BUF_SIZE: usize = 512;

// ── History Ring Buffer ──────────────────────────────────────────────────

const HistoryRing = struct {
    buf:   [HISTORY_SIZE][CMD_BUF_SIZE]u8,
    lens:  [HISTORY_SIZE]usize,
    head:  usize,
    count: usize,

    pub fn init() HistoryRing {
        return .{
            .buf   = undefined,
            .lens  = [_]usize{0} ** HISTORY_SIZE,
            .head  = 0,
            .count = 0,
        };
    }

    pub fn push(self: *HistoryRing, cmd: []const u8) void {
        const idx  = self.head % HISTORY_SIZE;
        const copy = @min(cmd.len, CMD_BUF_SIZE - 1);
        @memcpy(self.buf[idx][0..copy], cmd[0..copy]);
        self.lens[idx] = copy;
        self.head  = (self.head + 1) % HISTORY_SIZE;
        if (self.count < HISTORY_SIZE) self.count += 1;
    }

    pub fn get(self: *const HistoryRing, back: usize) ?[]const u8 {
        if (back >= self.count) return null;
        const idx = (self.head + HISTORY_SIZE - 1 - back) % HISTORY_SIZE;
        return self.buf[idx][0..self.lens[idx]];
    }
};

// ── Built-in Commands ─────────────────────────────────────────────────────

fn handleBuiltin(cmd: []const u8, writer: anytype) !bool {
    const trimmed = mem.trim(u8, cmd, " \t\r\n");

    if (mem.eql(u8, trimmed, "clear")) {
        try writer.writeAll("\x1b[2J\x1b[H");
        return true;
    }
    if (mem.eql(u8, trimmed, "exit") or mem.eql(u8, trimmed, "quit")) {
        try writer.writeAll("Goodbye.\n");
        os.exit(0);
    }
    if (mem.eql(u8, trimmed, "version")) {
        try writer.writeAll("SigmaOS sigmaTerm v0.1 — Zig native terminal\n");
        return true;
    }
    if (mem.startsWith(u8, trimmed, "echo ")) {
        try writer.writeAll(trimmed[5..]);
        try writer.writeByte('\n');
        return true;
    }
    return false;
}

// ── Main REPL ─────────────────────────────────────────────────────────────

pub fn main() !void {
    const stdin  = io.getStdIn().reader();
    const stdout = io.getStdOut().writer();

    try stdout.writeAll("\x1b[1;36mSigmaOS sigmaTerm v0.1\x1b[0m\n");
    try stdout.writeAll("Type 'exit' to quit, 'version' for info.\n\n");

    var history = HistoryRing.init();
    var cmd_buf: [CMD_BUF_SIZE]u8 = undefined;

    while (true) {
        try stdout.writeAll("\x1b[32mσ\x1b[0m ");
        const maybe_line = try stdin.readUntilDelimiterOrEof(&cmd_buf, '\n');
        const line = maybe_line orelse break;

        if (line.len == 0) continue;
        history.push(line);

        const handled = try handleBuiltin(line, stdout);
        if (!handled) {
            try stdout.print("[sigmaTerm] Unknown command: {s}\n", .{line});
        }
    }
}
