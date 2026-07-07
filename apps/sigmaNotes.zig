// SigmaOS sigma_notes — Minimal text notes app in Zig
// Replaces apps/sigmaNotes.js (JavaScript).
// Reads/writes plaintext files via POSIX syscalls only — zero runtime.

const std = @import("std");
const os  = std.os;
const io  = std.io;
const fs  = std.fs;
const mem = std.mem;

const NOTES_DIR  = "./sigma_notes";
const MAX_NOTES  = 64;
const NAME_LEN   = 128;
const BODY_LEN   = 4096;

// ── Note Storage ──────────────────────────────────────────────────────────

const NoteEntry = struct {
    name: [NAME_LEN]u8,
    len:  usize,
};

// ── Commands ──────────────────────────────────────────────────────────────

fn ensureNotesDir() void {
    fs.cwd().makeDir(NOTES_DIR) catch {};
}

fn cmdNew(name: []const u8, body: []const u8) !void {
    ensureNotesDir();
    var path_buf: [256]u8 = undefined;
    const path = try std.fmt.bufPrint(&path_buf, "{s}/{s}.txt", .{NOTES_DIR, name});
    const file = try fs.cwd().createFile(path, .{});
    defer file.close();
    try file.writeAll(body);
    const stdout = io.getStdOut().writer();
    try stdout.print("[sigmaNotes] Note '{s}' saved.\n", .{name});
}

fn cmdRead(name: []const u8) !void {
    var path_buf: [256]u8 = undefined;
    const path = try std.fmt.bufPrint(&path_buf, "{s}/{s}.txt", .{NOTES_DIR, name});
    const file = fs.cwd().openFile(path, .{}) catch {
        const stdout = io.getStdOut().writer();
        try stdout.print("[sigmaNotes] Note '{s}' not found.\n", .{name});
        return;
    };
    defer file.close();
    var buf: [BODY_LEN]u8 = undefined;
    const n   = try file.readAll(&buf);
    const stdout = io.getStdOut().writer();
    try stdout.writeAll(buf[0..n]);
    try stdout.writeByte('\n');
}

fn cmdList() !void {
    ensureNotesDir();
    const stdout = io.getStdOut().writer();
    try stdout.writeAll("[sigmaNotes] Notes:\n");
    var dir = fs.cwd().openIterableDir(NOTES_DIR, .{}) catch {
        try stdout.writeAll("  (no notes directory)\n");
        return;
    };
    defer dir.close();
    var it = dir.iterate();
    while (try it.next()) |entry| {
        try stdout.print("  - {s}\n", .{entry.name});
    }
}

// ── Main ──────────────────────────────────────────────────────────────────

pub fn main() !void {
    const stdout = io.getStdOut().writer();
    const args   = std.process.argsAlloc(std.heap.page_allocator) catch &[_][]u8{};

    if (args.len < 2) {
        try stdout.writeAll("SigmaOS sigmaNotes v0.1\n");
        try stdout.writeAll("Usage: sigmaNotes <new|read|list> [name] [body]\n");
        return;
    }

    const cmd = args[1];
    if (mem.eql(u8, cmd, "new") and args.len >= 4) {
        try cmdNew(args[2], args[3]);
    } else if (mem.eql(u8, cmd, "read") and args.len >= 3) {
        try cmdRead(args[2]);
    } else if (mem.eql(u8, cmd, "list")) {
        try cmdList();
    } else {
        try stdout.print("[sigmaNotes] Unknown command: {s}\n", .{cmd});
    }
}
