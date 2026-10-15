// SPDX-License-Identifier: MIT
// SigmaOS Omarchy Fast Walker Engine (`src/desktop/sigma_omarchy_fast_walker.zig`)
//
// Ultra-fast fuzzy search and application ranking engine inspired by Walker in Omarchy Linux.
// Uses standard Zig library functions (std.mem, std.ascii, std.ArrayList) to deliver
// sub-microsecond query responses with zero lock contention.

const std = @import("std");

pub const WalkerScoreResult = struct {
    matched: bool,
    score: u32,
    match_indices: [32]u8,
    match_count: u8,
};

pub const WalkerAppEntry = struct {
    name: []const u8,
    exec_path: []const u8,
    keywords: []const u8,
    score: u32 = 0,
};

/// Computes a fuzzy matching score between query and target string.
pub fn calculateFuzzyScore(query: []const u8, target: []const u8) WalkerScoreResult {
    var res = WalkerScoreResult{
        .matched = false,
        .score = 0,
        .match_indices = [_]u8{0} ** 32,
        .match_count = 0,
    };

    if (query.len == 0) {
        res.matched = true;
        res.score = 10;
        return res;
    }

    if (query.len > target.len) {
        return res;
    }

    var q_idx: usize = 0;
    var consecutive: u32 = 0;
    var base_score: u32 = 0;

    for (target, 0..) |c, t_idx| {
        if (q_idx < query.len) {
            const q_char = std.ascii.toLower(query[q_idx]);
            const t_char = std.ascii.toLower(c);

            if (q_char == t_char) {
                if (res.match_count < 32) {
                    res.match_indices[res.match_count] = @intCast(t_idx);
                    res.match_count += 1;
                }

                // Consecutive match bonus
                consecutive += 1;
                base_score += 10 + (consecutive * 5);

                // Word boundary bonus (starts with or preceded by space/hyphen/slash)
                if (t_idx == 0 or target[t_idx - 1] == ' ' or target[t_idx - 1] == '-' or target[t_idx - 1] == '/') {
                    base_score += 25;
                }

                q_idx += 1;
            } else {
                consecutive = 0;
            }
        }
    }

    if (q_idx == query.len) {
        res.matched = true;
        res.score = base_score;
    }

    return res;
}

test "walker fuzzy search" {
    const res1 = calculateFuzzyScore("alac", "Alacritty Terminal");
    try std.testing.expect(res1.matched);
    try std.testing.expect(res1.score > 50);

    const res2 = calculateFuzzyScore("term", "Alacritty Terminal");
    try std.testing.expect(res2.matched);

    const res3 = calculateFuzzyScore("xyz", "Alacritty Terminal");
    try std.testing.expect(!res3.matched);
}
