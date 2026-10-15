// SPDX-License-Identifier: MIT
// SigmaOS Fast Starship Prompt Generator (`src/desktop/sigma_starship_fast_prompt.zig`)
//
// Sub-millisecond contextual shell prompt generator in Zig.
// Uses standard Zig library functions (std.fmt, std.mem) to format
// directory paths, Git branch status, and prompt symbols with zero heap allocs.

const std = @import("std");

pub const PromptConfig = struct {
    directory: []const u8,
    git_branch: ?[]const u8,
    is_dirty: bool,
    status_code: u8,
    symbol: []const u8,
};

/// Formats a complete two-line contextual prompt into a caller-provided fixed buffer.
pub fn formatPrompt(cfg: PromptConfig, out_buf: []u8) !usize {
    var fbs = std.io.fixedBufferStream(out_buf);
    const writer = fbs.writer();

    // Line 1: Directory in cyan
    try writer.print("\x1b[36m{s}\x1b[0m", .{cfg.directory});

    // Optional Git branch in magenta
    if (cfg.git_branch) |branch| {
        const dirty_marker = if (cfg.is_dirty) "*" else "";
        try writer.print(" on \x1b[35mgit:({s}{s})\x1b[0m", .{ branch, dirty_marker });
    }

    // Line 2: Prompt symbol (green if success, red if non-zero exit)
    const color_code = if (cfg.status_code == 0) "\x1b[32m" else "\x1b[31m";
    try writer.print("\n{s}{s}\x1b[0m ", .{ color_code, cfg.symbol });

    return fbs.getWritten().len;
}

test "formatPrompt success" {
    var buffer: [256]u8 = undefined;
    const cfg = PromptConfig{
        .directory = "~/SigmaOS",
        .git_branch = "main",
        .is_dirty = false,
        .status_code = 0,
        .symbol = "❯",
    };

    const len = try formatPrompt(cfg, &buffer);
    try std.testing.expect(len > 0);
    try std.testing.expect(std.mem.indexOf(u8, buffer[0..len], "main") != null);
}
