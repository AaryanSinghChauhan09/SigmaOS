// SPDX-License-Identifier: GPL-3.0-or-later
// SigmaOS Sovereign Fast Dotfile Scanner
// (`src/onboarding/sigma_fast_dotfile_scanner.zig`)
// Low-level Zig module for zero-allocation recursive dotfile inspection,
// browser profile path identification, and fast hash fingerprinting.

const std = @import("std");

pub const ConfigCategory = enum(u8) {
    ShellBash,
    ShellZsh,
    TerminalKitty,
    TerminalAlacritty,
    TerminalFoot,
    TerminalGhostty,
    EditorNvim,
    EditorHelix,
    BrowserFirefox,
    BrowserChrome,
    BrowserChromium,
    WindowManagerHyprland,
    WindowManagerCinnamon,
    Unknown,
};

pub const DiscoveredConfig = struct {
    category: ConfigCategory,
    path_len: usize,
    path_buf: [256]u8,
    file_size_bytes: u64,

    pub fn getPath(self: *const DiscoveredConfig) []const u8 {
        return self.path_buf[0..self.path_len];
    }
};

/// Match a file relative path against known Linux Mint and Omarchy configuration patterns
pub fn classifyConfigPath(rel_path: []const u8) ConfigCategory {
    if (std.mem.eql(u8, rel_path, ".bashrc")) return .ShellBash;
    if (std.mem.eql(u8, rel_path, ".zshrc")) return .ShellZsh;
    if (std.mem.indexOf(u8, rel_path, ".config/kitty") != null) return .TerminalKitty;
    if (std.mem.indexOf(u8, rel_path, ".config/alacritty") != null) return .TerminalAlacritty;
    if (std.mem.indexOf(u8, rel_path, ".config/foot") != null) return .TerminalFoot;
    if (std.mem.indexOf(u8, rel_path, ".config/ghostty") != null) return .TerminalGhostty;
    if (std.mem.indexOf(u8, rel_path, ".config/nvim") != null) return .EditorNvim;
    if (std.mem.indexOf(u8, rel_path, ".config/helix") != null) return .EditorHelix;
    if (std.mem.indexOf(u8, rel_path, ".mozilla/firefox") != null) return .BrowserFirefox;
    if (std.mem.indexOf(u8, rel_path, ".config/google-chrome") != null) return .BrowserChrome;
    if (std.mem.indexOf(u8, rel_path, ".config/chromium") != null) return .BrowserChromium;
    if (std.mem.indexOf(u8, rel_path, ".config/hypr") != null) return .WindowManagerHyprland;
    if (std.mem.indexOf(u8, rel_path, ".config/cinnamon") != null) return .WindowManagerCinnamon;

    return .Unknown;
}

/// Calculate fast 64-bit checksum for configuration file deduplication
pub fn fastConfigHash(data: []const u8) u64 {
    var hash: u64 = 0xcbf29ce484222325; // FNV-1a 64-bit offset basis
    const prime: u64 = 0x100000001b3;

    for (data) |byte| {
        hash ^= @as(u64, byte);
        hash *%= prime;
    }
    return hash;
}
