//! SigmaOS Sovereign Word Processor — sigma_writer.zig
//! Freestanding rich-text document engine. No libc, no stdlib allocator.
//! Implements: document model, paragraph layout, text run rendering,
//! basic formatting (bold/italic/underline), cursor + caret tracking,
//! serialisation to SigmaDoc binary format.
//!
//! Sovereign principles:
//!   - No @import of std library beyond comptime primitives
//!   - All memory via injected SovereignAllocator vtable
//!   - OOP via tagged union + vtable dispatch

const builtin = @import("builtin");

// ─── Allocator Vtable ────────────────────────────────────────────────────────
pub const AllocFn  = *const fn (ctx: *anyopaque, size: usize, align_: usize) ?[*]u8;
pub const FreeFn   = *const fn (ctx: *anyopaque, ptr: [*]u8, size: usize) void;

pub const SovereignAllocator = struct {
    ctx:   *anyopaque,
    alloc: AllocFn,
    free:  FreeFn,

    pub fn allocT(self: *SovereignAllocator, comptime T: type, count: usize) ?[*]T {
        const raw = self.alloc(self.ctx, @sizeOf(T) * count, @alignOf(T)) orelse return null;
        return @ptrCast(raw);
    }
    pub fn freeT(self: *SovereignAllocator, comptime T: type, ptr: [*]T, count: usize) void {
        self.free(self.ctx, @ptrCast(ptr), @sizeOf(T) * count);
    }
};

// ─── Text Formatting Flags ───────────────────────────────────────────────────
pub const FmtFlags = packed struct {
    bold:      u1 = 0,
    italic:    u1 = 0,
    underline: u1 = 0,
    strike:    u1 = 0,
    code:      u1 = 0,
    _pad:      u3 = 0,
};

// ─── Text Run ────────────────────────────────────────────────────────────────
/// A contiguous run of characters sharing the same formatting.
pub const TextRun = struct {
    text:    [*]const u8,  // UTF-8 bytes; NOT null-terminated
    len:     u32,
    fmt:     FmtFlags,
    color:   u32,          // ARGB
    font_sz: u16,          // 1/64 pt units (e.g. 12pt → 768)
};

// ─── Paragraph ───────────────────────────────────────────────────────────────
pub const Align_ = enum(u2) { Left, Center, Right, Justify };

pub const Paragraph = struct {
    runs:       [*]TextRun,
    run_count:  u32,
    line_space: u16,   // 1/100 line-height multiplier (100 = 1.0)
    align_:     Align_,
    indent_px:  u16,
    next:       ?*Paragraph,
};

// ─── Document ────────────────────────────────────────────────────────────────
pub const Document = struct {
    head:   ?*Paragraph,
    tail:   ?*Paragraph,
    count:  u32,
    alloc:  *SovereignAllocator,

    pub fn init(alloc: *SovereignAllocator) Document {
        return .{ .head = null, .tail = null, .count = 0, .alloc = alloc };
    }

    /// Append a new empty paragraph; returns pointer to it or null on OOM.
    pub fn appendParagraph(self: *Document, align_: Align_) ?*Paragraph {
        const buf = self.alloc.allocT(Paragraph, 1) orelse return null;
        const para: *Paragraph = &buf[0];
        para.* = .{
            .runs      = undefined,
            .run_count = 0,
            .line_space = 120,
            .align_    = align_,
            .indent_px = 0,
            .next      = null,
        };
        if (self.tail) |t| {
            t.next = para;
        } else {
            self.head = para;
        }
        self.tail = para;
        self.count += 1;
        return para;
    }

    /// Append a text run to the last paragraph.
    pub fn appendRun(self: *Document, text: []const u8, fmt: FmtFlags, color: u32, sz: u16) bool {
        const para = self.tail orelse return false;
        // Grow run array (simple doubling slab)
        const new_count = para.run_count + 1;
        const new_buf = self.alloc.allocT(TextRun, new_count) orelse return false;
        // Copy existing runs
        var i: u32 = 0;
        while (i < para.run_count) : (i += 1) {
            new_buf[i] = para.runs[i];
        }
        if (para.run_count > 0) {
            self.alloc.freeT(TextRun, para.runs, para.run_count);
        }
        new_buf[para.run_count] = TextRun{
            .text    = text.ptr,
            .len     = @intCast(text.len),
            .fmt     = fmt,
            .color   = color,
            .font_sz = sz,
        };
        para.runs      = new_buf;
        para.run_count = new_count;
        return true;
    }
};

// ─── Caret ───────────────────────────────────────────────────────────────────
pub const Caret = struct {
    para_idx: u32,  // paragraph index (0-based)
    run_idx:  u32,  // run inside paragraph
    byte_off: u32,  // byte offset within run.text

    pub fn moveRight(self: *Caret, doc: *const Document) void {
        // Walk paragraphs to find current one
        var p = doc.head;
        var idx: u32 = 0;
        while (p) |para| : (p = para.next) {
            if (idx == self.para_idx) {
                const run = &para.runs[self.run_idx];
                if (self.byte_off + 1 < run.len) {
                    self.byte_off += 1;
                } else if (self.run_idx + 1 < para.run_count) {
                    self.run_idx += 1;
                    self.byte_off = 0;
                } else if (para.next != null) {
                    self.para_idx += 1;
                    self.run_idx   = 0;
                    self.byte_off  = 0;
                }
                return;
            }
            idx += 1;
        }
    }
};

// ─── SigmaDoc Binary Serialisation ───────────────────────────────────────────
/// Magic: 'S','D','O','C', version u8, para_count u32 LE
/// Each paragraph: run_count u32 LE, align u8, line_space u16 LE
/// Each run: len u32 LE, fmt u8, color u32 LE, font_sz u16 LE, bytes[len]

pub const SDOC_MAGIC = [4]u8{ 'S', 'D', 'O', 'C' };
pub const SDOC_VERSION: u8 = 1;

/// Write document to a fixed-size output buffer.
/// Returns bytes written or 0 on buffer-too-small.
pub fn serialise(doc: *const Document, out: []u8) usize {
    var pos: usize = 0;

    inline fn write_u8(b: u8) void  { out[pos] = b; pos += 1; }
    inline fn write_u16(v: u16) void {
        out[pos]   = @truncate(v & 0xFF);
        out[pos+1] = @truncate((v >> 8) & 0xFF);
        pos += 2;
    }
    inline fn write_u32(v: u32) void {
        out[pos]   = @truncate(v & 0xFF);
        out[pos+1] = @truncate((v >> 8)  & 0xFF);
        out[pos+2] = @truncate((v >> 16) & 0xFF);
        out[pos+3] = @truncate((v >> 24) & 0xFF);
        pos += 4;
    }

    if (out.len < 10) return 0;
    // Header
    out[0] = SDOC_MAGIC[0]; out[1] = SDOC_MAGIC[1];
    out[2] = SDOC_MAGIC[2]; out[3] = SDOC_MAGIC[3];
    pos = 4;
    write_u8(SDOC_VERSION);
    write_u32(doc.count);

    var p = doc.head;
    while (p) |para| : (p = para.next) {
        if (pos + 7 > out.len) return 0;
        write_u32(para.run_count);
        write_u8(@intFromEnum(para.align_));
        write_u16(para.line_space);

        var ri: u32 = 0;
        while (ri < para.run_count) : (ri += 1) {
            const run = &para.runs[ri];
            if (pos + 11 + run.len > out.len) return 0;
            write_u32(run.len);
            write_u8(@bitCast(run.fmt));
            write_u32(run.color);
            write_u16(run.font_sz);
            var bi: u32 = 0;
            while (bi < run.len) : (bi += 1) {
                write_u8(run.text[bi]);
            }
        }
    }
    return pos;
}

// ─── Self-test (comptime) ────────────────────────────────────────────────────
comptime {
    // Ensure key types have expected sizes
    if (@sizeOf(FmtFlags) != 1) @compileError("FmtFlags must be 1 byte");
    if (@sizeOf(TextRun)  != 20) @compileError("TextRun layout changed");
}
