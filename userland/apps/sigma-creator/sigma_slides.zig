//! sigma_slides.zig — SigmaOS Sovereign Presentation Engine
//! Freestanding slide layout & compositor. No libc, no std.
//! Implements: Slide model, elements (text box, shape, image ref),
//!             transition types, binary SigmaPresent (.spres) format.
//!
//! OOP pattern: Element is a tagged union dispatched via type field.

const builtin = @import("builtin");

// ─── Allocator (same vtable pattern as sigma_writer) ─────────────────────────
pub const AllocFn = *const fn (ctx: *anyopaque, size: usize, align_: usize) ?[*]u8;
pub const FreeFn  = *const fn (ctx: *anyopaque, ptr: [*]u8, size: usize) void;
pub const SovereignAllocator = struct {
    ctx: *anyopaque, alloc: AllocFn, free: FreeFn,
    pub fn allocT(self: *SovereignAllocator, comptime T: type, n: usize) ?[*]T {
        const raw = self.alloc(self.ctx, @sizeOf(T) * n, @alignOf(T)) orelse return null;
        return @ptrCast(raw);
    }
    pub fn freeT(self: *SovereignAllocator, comptime T: type, p: [*]T, n: usize) void {
        self.free(self.ctx, @ptrCast(p), @sizeOf(T) * n);
    }
};

// ─── Geometry ────────────────────────────────────────────────────────────────
pub const Rect = packed struct { x: i16, y: i16, w: u16, h: u16 };

// ─── Transition ──────────────────────────────────────────────────────────────
pub const Transition = enum(u8) {
    None, Fade, SlideLeft, SlideRight, ZoomIn, ZoomOut, Flip,
};

// ─── Element Types ────────────────────────────────────────────────────────────
pub const ElementKind = enum(u8) { TextBox, Shape, ImageRef };

pub const ShapeKind = enum(u8) { Rect, Ellipse, Triangle, Line };

pub const Element = union(ElementKind) {
    TextBox: TextBoxElem,
    Shape:   ShapeElem,
    ImageRef: ImageRefElem,
};

pub const TextBoxElem = struct {
    bounds:   Rect,
    text:     [*]const u8,
    text_len: u32,
    font_sz:  u16,         // 1/64 pt
    color:    u32,         // ARGB
    bg:       u32,         // ARGB (0 = transparent)
    bold:     bool,
    italic:   bool,
};

pub const ShapeElem = struct {
    bounds: Rect,
    kind:   ShapeKind,
    fill:   u32,    // ARGB
    stroke: u32,    // ARGB
    thick:  u8,     // stroke width px
};

pub const ImageRefElem = struct {
    bounds:  Rect,
    path:    [*]const u8,   // path in SigmaFS
    path_len: u16,
};

// ─── Slide ───────────────────────────────────────────────────────────────────
pub const MAX_ELEMENTS_PER_SLIDE: usize = 64;

pub const Slide = struct {
    bg_color:   u32 = 0xFF_1A_1A_2E,   // default dark blue
    transition: Transition = .None,
    dur_ms:     u16 = 500,              // transition duration
    elements:   [MAX_ELEMENTS_PER_SLIDE]Element = undefined,
    elem_count: u8 = 0,
    next:       ?*Slide = null,

    pub fn addElement(self: *Slide, elem: Element) bool {
        if (self.elem_count >= MAX_ELEMENTS_PER_SLIDE) return false;
        self.elements[self.elem_count] = elem;
        self.elem_count += 1;
        return true;
    }
};

// ─── Presentation ─────────────────────────────────────────────────────────────
pub const Presentation = struct {
    head:        ?*Slide = null,
    tail:        ?*Slide = null,
    slide_count: u16 = 0,
    alloc:       *SovereignAllocator,
    width_px:    u16 = 1920,
    height_px:   u16 = 1080,

    pub fn init(alloc: *SovereignAllocator, w: u16, h: u16) Presentation {
        return .{ .alloc = alloc, .width_px = w, .height_px = h };
    }

    pub fn appendSlide(self: *Presentation) ?*Slide {
        const buf = self.alloc.allocT(Slide, 1) orelse return null;
        const s: *Slide = &buf[0];
        s.* = Slide{ .next = null };
        if (self.tail) |t| t.next = s else self.head = s;
        self.tail = s;
        self.slide_count += 1;
        return s;
    }

    pub fn slideAt(self: *Presentation, idx: u16) ?*Slide {
        var cur = self.head;
        var i: u16 = 0;
        while (cur) |s| : (cur = s.next) {
            if (i == idx) return s;
            i += 1;
        }
        return null;
    }
};

// ─── SigmaPresent Binary Format (.spres) ─────────────────────────────────────
// Magic: 'S','P','R','S', version u8, w u16 LE, h u16 LE, slide_count u16 LE
// Per slide: bg u32 LE, transition u8, dur u16 LE, elem_count u8
// Per element: kind u8, then kind-specific payload

pub fn serialise(pres: *const Presentation, out: []u8) usize {
    var pos: usize = 0;

    const write_u8  = struct { fn f(o: []u8, p: *usize, v: u8) void  { o[p.*] = v; p.* += 1; } }.f;
    const write_u16 = struct { fn f(o: []u8, p: *usize, v: u16) void {
        o[p.*] = @truncate(v & 0xFF); o[p.*+1] = @truncate(v >> 8); p.* += 2;
    } }.f;
    const write_u32 = struct { fn f(o: []u8, p: *usize, v: u32) void {
        o[p.*]   = @truncate(v & 0xFF);
        o[p.*+1] = @truncate((v >> 8)  & 0xFF);
        o[p.*+2] = @truncate((v >> 16) & 0xFF);
        o[p.*+3] = @truncate(v >> 24);
        p.* += 4;
    } }.f;
    const write_i16 = struct { fn f(o: []u8, p: *usize, v: i16) void {
        write_u16(o, p, @bitCast(v));
    } }.f;

    if (out.len < 11) return 0;
    out[0]='S'; out[1]='P'; out[2]='R'; out[3]='S';
    pos = 4;
    write_u8(out, &pos, 1); // version
    write_u16(out, &pos, pres.width_px);
    write_u16(out, &pos, pres.height_px);
    write_u16(out, &pos, pres.slide_count);

    var cur = pres.head;
    while (cur) |s| : (cur = s.next) {
        if (pos + 8 > out.len) return 0;
        write_u32(out, &pos, s.bg_color);
        write_u8(out, &pos, @intFromEnum(s.transition));
        write_u16(out, &pos, s.dur_ms);
        write_u8(out, &pos, s.elem_count);

        var ei: u8 = 0;
        while (ei < s.elem_count) : (ei += 1) {
            const elem = &s.elements[ei];
            write_u8(out, &pos, @intFromEnum(elem.*));
            switch (elem.*) {
                .TextBox => |tb| {
                    if (pos + 20 + tb.text_len > out.len) return 0;
                    write_i16(out, &pos, tb.bounds.x);
                    write_i16(out, &pos, tb.bounds.y);
                    write_u16(out, &pos, tb.bounds.w);
                    write_u16(out, &pos, tb.bounds.h);
                    write_u32(out, &pos, tb.color);
                    write_u32(out, &pos, tb.bg);
                    write_u16(out, &pos, tb.font_sz);
                    write_u8(out, &pos, if (tb.bold) 1 else 0);
                    write_u8(out, &pos, if (tb.italic) 1 else 0);
                    write_u32(out, &pos, tb.text_len);
                    var ti: u32 = 0;
                    while (ti < tb.text_len) : (ti += 1) {
                        write_u8(out, &pos, tb.text[ti]);
                    }
                },
                .Shape => |sh| {
                    if (pos + 14 > out.len) return 0;
                    write_i16(out, &pos, sh.bounds.x);
                    write_i16(out, &pos, sh.bounds.y);
                    write_u16(out, &pos, sh.bounds.w);
                    write_u16(out, &pos, sh.bounds.h);
                    write_u8(out, &pos, @intFromEnum(sh.kind));
                    write_u32(out, &pos, sh.fill);
                    write_u32(out, &pos, sh.stroke);
                    write_u8(out, &pos, sh.thick);
                },
                .ImageRef => |ir| {
                    if (pos + 10 + ir.path_len > out.len) return 0;
                    write_i16(out, &pos, ir.bounds.x);
                    write_i16(out, &pos, ir.bounds.y);
                    write_u16(out, &pos, ir.bounds.w);
                    write_u16(out, &pos, ir.bounds.h);
                    write_u16(out, &pos, ir.path_len);
                    var pi: u16 = 0;
                    while (pi < ir.path_len) : (pi += 1) {
                        write_u8(out, &pos, ir.path[pi]);
                    }
                },
            }
        }
    }
    return pos;
}

// ─── Layout: compute text wrap positions (sovereign, no font metrics) ─────────
/// Returns number of line breaks needed given pixel width and average char width.
pub fn estimateLineBreaks(text_len: u32, box_w: u16, avg_char_w_px: u8) u16 {
    if (avg_char_w_px == 0 or box_w == 0) return 0;
    const chars_per_line: u32 = @as(u32, box_w) / @as(u32, avg_char_w_px);
    if (chars_per_line == 0) return @intCast(text_len);
    return @intCast(text_len / chars_per_line);
}
