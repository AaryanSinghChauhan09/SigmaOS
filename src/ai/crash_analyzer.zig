// src/ai/crash_analyzer.zig
// Low-level crash dump analyzer (Zig for performance-critical parsing)
// Replaces Python gdb/lldb dependencies in Omarchy

const std = @import("std");

/// Crash signal types
pub const CrashSignal = enum(i32) {
    SIGILL = 4,   // Illegal instruction
    SIGTRAP = 5,  // Trace trap
    SIGABRT = 6,  // Abort
    SIGBUS = 7,   // Bus error
    SIGFPE = 8,   // Floating point exception
    SIGSEGV = 11, // Segmentation fault
    SIGSTKFLT = 16, // Stack fault
    
    pub fn toString(self: CrashSignal) []const u8 {
        return switch (self) {
            .SIGILL => "Illegal Instruction",
            .SIGTRAP => "Trace Trap",
            .SIGABRT => "Abort",
            .SIGBUS => "Bus Error",
            .SIGFPE => "Floating Point Exception",
            .SIGSEGV => "Segmentation Fault",
            .SIGSTKFLT => "Stack Fault",
        };
    }
};

/// CPU register state (x86_64)
pub const RegisterState = struct {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    rsp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
    rflags: u64,
    
    pub fn format(
        self: RegisterState,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;
        try writer.print(
            \\RAX: 0x{x:0>16} RBX: 0x{x:0>16}
            \\RCX: 0x{x:0>16} RDX: 0x{x:0>16}
            \\RSI: 0x{x:0>16} RDI: 0x{x:0>16}
            \\RBP: 0x{x:0>16} RSP: 0x{x:0>16}
            \\R8:  0x{x:0>16} R9:  0x{x:0>16}
            \\R10: 0x{x:0>16} R11: 0x{x:0>16}
            \\R12: 0x{x:0>16} R13: 0x{x:0>16}
            \\R14: 0x{x:0>16} R15: 0x{x:0>16}
            \\RIP: 0x{x:0>16} RFLAGS: 0x{x:0>16}
        , .{
            self.rax, self.rbx, self.rcx, self.rdx,
            self.rsi, self.rdi, self.rbp, self.rsp,
            self.r8, self.r9, self.r10, self.r11,
            self.r12, self.r13, self.r14, self.r15,
            self.rip, self.rflags,
        });
    }
};

/// Memory mapping entry
pub const MemoryMapping = struct {
    start_addr: u64,
    end_addr: u64,
    permissions: u8, // rwx bits
    offset: u64,
    device: [2]u32,
    inode: u64,
    path: [256]u8,
    path_len: usize,
    
    pub fn contains(self: MemoryMapping, addr: u64) bool {
        return addr >= self.start_addr and addr < self.end_addr;
    }
    
    pub fn isExecutable(self: MemoryMapping) bool {
        return (self.permissions & 0x04) != 0;
    }
    
    pub fn isWritable(self: MemoryMapping) bool {
        return (self.permissions & 0x02) != 0;
    }
};

/// Stack frame in backtrace
pub const StackFrame = struct {
    address: u64,
    function_name: [128]u8,
    function_name_len: usize,
    file_name: [256]u8,
    file_name_len: usize,
    line_number: u32,
    offset: u64,
};

/// Complete crash dump structure
pub const CrashDumpInfo = struct {
    process_id: u32,
    thread_id: u32,
    signal: CrashSignal,
    fault_address: u64,
    registers: RegisterState,
    backtrace: []StackFrame,
    memory_maps: []MemoryMapping,
    timestamp_ns: u64,
    
    allocator: std.mem.Allocator,
    
    pub fn deinit(self: *CrashDumpInfo) void {
        self.allocator.free(self.backtrace);
        self.allocator.free(self.memory_maps);
    }
};

/// Analysis result
pub const AnalysisResult = struct {
    crash_type: CrashType,
    likely_cause: []const u8,
    suggestions: [][]const u8,
    confidence: f32,
    
    allocator: std.mem.Allocator,
    
    pub fn deinit(self: *AnalysisResult) void {
        for (self.suggestions) |suggestion| {
            self.allocator.free(suggestion);
        }
        self.allocator.free(self.suggestions);
    }
};

/// Crash type classification
pub const CrashType = enum {
    NullPointerDereference,
    BufferOverflow,
    StackOverflow,
    UseAfterFree,
    DoubleFree,
    UnalignedAccess,
    DivisionByZero,
    IllegalInstruction,
    Unknown,
    
    pub fn toString(self: CrashType) []const u8 {
        return switch (self) {
            .NullPointerDereference => "Null Pointer Dereference",
            .BufferOverflow => "Buffer Overflow",
            .StackOverflow => "Stack Overflow",
            .UseAfterFree => "Use After Free",
            .DoubleFree => "Double Free",
            .UnalignedAccess => "Unaligned Memory Access",
            .DivisionByZero => "Division by Zero",
            .IllegalInstruction => "Illegal Instruction",
            .Unknown => "Unknown",
        };
    }
};

/// Main crash analyzer
pub const CrashAnalyzer = struct {
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) CrashAnalyzer {
        return .{ .allocator = allocator };
    }
    
    /// Analyze crash dump and produce report
    pub fn analyze(self: *CrashAnalyzer, dump: *const CrashDumpInfo) !AnalysisResult {
        var result = AnalysisResult{
            .crash_type = .Unknown,
            .likely_cause = "",
            .suggestions = &[_][]const u8{},
            .confidence = 0.0,
            .allocator = self.allocator,
        };
        
        // Classify crash type based on signal and context
        result.crash_type = self.classifyCrash(dump);
        
        // Analyze fault address and registers
        if (dump.signal == .SIGSEGV or dump.signal == .SIGBUS) {
            if (dump.fault_address == 0 or dump.fault_address < 4096) {
                result.crash_type = .NullPointerDereference;
                result.confidence = 0.95;
            } else if (self.isStackAddress(dump.fault_address, dump)) {
                result.crash_type = .StackOverflow;
                result.confidence = 0.90;
            }
        } else if (dump.signal == .SIGFPE) {
            result.crash_type = .DivisionByZero;
            result.confidence = 0.85;
        } else if (dump.signal == .SIGILL) {
            result.crash_type = .IllegalInstruction;
            result.confidence = 0.85;
        }
        
        // Generate suggestions based on crash type
        result.suggestions = try self.generateSuggestions(result.crash_type);
        
        return result;
    }
    
    fn classifyCrash(self: *CrashAnalyzer, dump: *const CrashDumpInfo) CrashType {
        _ = self;
        
        return switch (dump.signal) {
            .SIGSEGV => .NullPointerDereference,
            .SIGFPE => .DivisionByZero,
            .SIGILL => .IllegalInstruction,
            .SIGBUS => .UnalignedAccess,
            .SIGABRT => .Unknown, // Could be assertion, double-free, etc.
            else => .Unknown,
        };
    }
    
    fn isStackAddress(self: *CrashAnalyzer, addr: u64, dump: *const CrashDumpInfo) bool {
        _ = self;
        
        // Check if address is near stack pointer
        const stack_delta = if (addr > dump.registers.rsp)
            addr - dump.registers.rsp
        else
            dump.registers.rsp - addr;
        
        // Within 8MB of stack pointer (typical stack size)
        return stack_delta < 8 * 1024 * 1024;
    }
    
    fn generateSuggestions(self: *CrashAnalyzer, crash_type: CrashType) ![][]const u8 {
        var suggestions = std.ArrayList([]const u8).init(self.allocator);
        errdefer suggestions.deinit();
        
        switch (crash_type) {
            .NullPointerDereference => {
                try suggestions.append(try self.allocator.dupe(u8, "Check for uninitialized pointers"));
                try suggestions.append(try self.allocator.dupe(u8, "Verify Option<T> unwrap safety"));
                try suggestions.append(try self.allocator.dupe(u8, "Review pointer arithmetic"));
            },
            .BufferOverflow => {
                try suggestions.append(try self.allocator.dupe(u8, "Add bounds checking on array access"));
                try suggestions.append(try self.allocator.dupe(u8, "Use safe slice operations"));
                try suggestions.append(try self.allocator.dupe(u8, "Validate input sizes before copy"));
            },
            .StackOverflow => {
                try suggestions.append(try self.allocator.dupe(u8, "Reduce local variable size"));
                try suggestions.append(try self.allocator.dupe(u8, "Check for infinite recursion"));
                try suggestions.append(try self.allocator.dupe(u8, "Allocate large buffers on heap"));
            },
            .UseAfterFree => {
                try suggestions.append(try self.allocator.dupe(u8, "Review ownership and lifetimes"));
                try suggestions.append(try self.allocator.dupe(u8, "Check for dangling references"));
                try suggestions.append(try self.allocator.dupe(u8, "Verify drop order"));
            },
            .DoubleFree => {
                try suggestions.append(try self.allocator.dupe(u8, "Ensure single ownership"));
                try suggestions.append(try self.allocator.dupe(u8, "Use RAII patterns"));
                try suggestions.append(try self.allocator.dupe(u8, "Avoid manual free() calls"));
            },
            .DivisionByZero => {
                try suggestions.append(try self.allocator.dupe(u8, "Add division by zero check"));
                try suggestions.append(try self.allocator.dupe(u8, "Validate divisor before operation"));
                try suggestions.append(try self.allocator.dupe(u8, "Use checked arithmetic"));
            },
            .IllegalInstruction => {
                try suggestions.append(try self.allocator.dupe(u8, "Check CPU feature flags"));
                try suggestions.append(try self.allocator.dupe(u8, "Verify instruction set compatibility"));
                try suggestions.append(try self.allocator.dupe(u8, "Review inline assembly"));
            },
            else => {
                try suggestions.append(try self.allocator.dupe(u8, "Enable AddressSanitizer for debugging"));
                try suggestions.append(try self.allocator.dupe(u8, "Review recent code changes"));
                try suggestions.append(try self.allocator.dupe(u8, "Check system logs for context"));
            },
        }
        
        return suggestions.toOwnedSlice();
    }
    
    /// Parse /proc/pid/maps format
    pub fn parseMemoryMaps(
        self: *CrashAnalyzer,
        maps_content: []const u8,
    ) ![]MemoryMapping {
        var mappings = std.ArrayList(MemoryMapping).init(self.allocator);
        errdefer mappings.deinit();
        
        var lines = std.mem.split(u8, maps_content, "\n");
        while (lines.next()) |line| {
            if (line.len == 0) continue;
            
            var mapping: MemoryMapping = undefined;
            
            // Parse: start-end perms offset dev inode pathname
            var tokens = std.mem.tokenize(u8, line, " \t");
            
            // Address range
            if (tokens.next()) |addr_range| {
                var addr_parts = std.mem.split(u8, addr_range, "-");
                if (addr_parts.next()) |start_str| {
                    mapping.start_addr = try std.fmt.parseInt(u64, start_str, 16);
                }
                if (addr_parts.next()) |end_str| {
                    mapping.end_addr = try std.fmt.parseInt(u64, end_str, 16);
                }
            }
            
            // Permissions
            if (tokens.next()) |perms| {
                mapping.permissions = 0;
                if (perms.len >= 3) {
                    if (perms[0] == 'r') mapping.permissions |= 0x01;
                    if (perms[1] == 'w') mapping.permissions |= 0x02;
                    if (perms[2] == 'x') mapping.permissions |= 0x04;
                }
            }
            
            // Offset
            if (tokens.next()) |offset_str| {
                mapping.offset = try std.fmt.parseInt(u64, offset_str, 16);
            }
            
            // Device (skip for now)
            _ = tokens.next();
            
            // Inode
            if (tokens.next()) |inode_str| {
                mapping.inode = try std.fmt.parseInt(u64, inode_str, 10);
            }
            
            // Pathname
            mapping.path_len = 0;
            if (tokens.next()) |pathname| {
                const copy_len = @min(pathname.len, mapping.path.len - 1);
                @memcpy(mapping.path[0..copy_len], pathname[0..copy_len]);
                mapping.path_len = copy_len;
            }
            
            try mappings.append(mapping);
        }
        
        return mappings.toOwnedSlice();
    }
    
    /// Format analysis result as human-readable text
    pub fn formatResult(
        self: *CrashAnalyzer,
        result: *const AnalysisResult,
        writer: anytype,
    ) !void {
        _ = self;
        
        try writer.print("Crash Type: {s}\n", .{result.crash_type.toString()});
        try writer.print("Confidence: {d:.1}%\n\n", .{result.confidence * 100});
        
        try writer.writeAll("Suggestions:\n");
        for (result.suggestions, 0..) |suggestion, i| {
            try writer.print("  {}. {s}\n", .{ i + 1, suggestion });
        }
    }
};

// Export C-compatible API for Rust FFI
export fn crash_analyzer_create() ?*CrashAnalyzer {
    const allocator = std.heap.page_allocator;
    const analyzer = allocator.create(CrashAnalyzer) catch return null;
    analyzer.* = CrashAnalyzer.init(allocator);
    return analyzer;
}

export fn crash_analyzer_destroy(analyzer: ?*CrashAnalyzer) void {
    if (analyzer) |a| {
        const allocator = a.allocator;
        allocator.destroy(a);
    }
}

// Tests
test "crash analyzer initialization" {
    var analyzer = CrashAnalyzer.init(std.testing.allocator);
    _ = analyzer;
}

test "null pointer detection" {
    var analyzer = CrashAnalyzer.init(std.testing.allocator);
    
    var backtrace = [_]StackFrame{};
    var memory_maps = [_]MemoryMapping{};
    
    const dump = CrashDumpInfo{
        .process_id = 1234,
        .thread_id = 1234,
        .signal = .SIGSEGV,
        .fault_address = 0, // NULL
        .registers = std.mem.zeroes(RegisterState),
        .backtrace = &backtrace,
        .memory_maps = &memory_maps,
        .timestamp_ns = 0,
        .allocator = std.testing.allocator,
    };
    
    var result = try analyzer.analyze(&dump);
    defer result.deinit();
    
    try std.testing.expectEqual(CrashType.NullPointerDereference, result.crash_type);
    try std.testing.expect(result.confidence > 0.9);
}

test "parse memory maps" {
    var analyzer = CrashAnalyzer.init(std.testing.allocator);
    
    const maps_content =
        \\400000-401000 r-xp 00000000 08:01 123 /bin/test
        \\600000-601000 rw-p 00000000 08:01 123 /bin/test
    ;
    
    const mappings = try analyzer.parseMemoryMaps(maps_content);
    defer std.testing.allocator.free(mappings);
    
    try std.testing.expectEqual(@as(usize, 2), mappings.len);
    try std.testing.expect(mappings[0].isExecutable());
    try std.testing.expect(!mappings[0].isWritable());
    try std.testing.expect(mappings[1].isWritable());
}
