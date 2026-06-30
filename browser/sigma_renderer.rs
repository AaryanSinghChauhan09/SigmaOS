//! sigma_renderer.rs — SigmaOS Sovereign Browser Framebuffer Renderer
//! Paints render boxes into a linear ARGB32 framebuffer.
//! No std, no external crates. Sovereign rasterisation.
//!
//! Implements:
//!   - Fill rectangle (ARGB blending via Porter-Duff SRC_OVER)
//!   - Bitmap font rasteriser (4×6 pixel cells, ASCII 0x20–0x7E)
//!   - Border drawing (outline)
//!   - Render tree walk → framebuffer paint

#![no_std]
#![allow(dead_code)]

use crate::sigma_layout::{LayoutEngine, RenderBox, Display};

// ─── Framebuffer ──────────────────────────────────────────────────────────────
pub struct Framebuffer<'fb> {
    pub pixels: &'fb mut [u32],  // ARGB32 packed
    pub width:  u32,
    pub height: u32,
    pub stride: u32,             // pixels per row (may differ from width)
}

impl<'fb> Framebuffer<'fb> {
    pub fn new(pixels: &'fb mut [u32], w: u32, h: u32) -> Self {
        Self { pixels, width: w, height: h, stride: w }
    }

    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 { return; }
        let (ux, uy) = (x as u32, y as u32);
        if ux >= self.width || uy >= self.height { return; }
        let idx = uy * self.stride + ux;
        self.pixels[idx as usize] = alpha_over(color, self.pixels[idx as usize]);
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: u32) {
        for dy in 0..h as i32 {
            for dx in 0..w as i32 {
                self.set_pixel(x + dx, y + dy, color);
            }
        }
    }

    pub fn draw_border(&mut self, x: i32, y: i32, w: u32, h: u32, thickness: u32, color: u32) {
        let t = thickness as i32;
        // Top
        self.fill_rect(x, y, w, thickness, color);
        // Bottom
        self.fill_rect(x, y + h as i32 - t, w, thickness, color);
        // Left
        self.fill_rect(x, y, thickness, h, color);
        // Right
        self.fill_rect(x + w as i32 - t, y, thickness, h, color);
    }
}

// ─── Porter-Duff SRC_OVER alpha blending ─────────────────────────────────────
#[inline]
fn alpha_over(src: u32, dst: u32) -> u32 {
    let sa = (src >> 24) & 0xFF;
    if sa == 0xFF { return src; }
    if sa == 0x00 { return dst; }
    let inv_a = 255 - sa;
    let sr = (src >> 16) & 0xFF;
    let sg = (src >>  8) & 0xFF;
    let sb =  src        & 0xFF;
    let dr = (dst >> 16) & 0xFF;
    let dg = (dst >>  8) & 0xFF;
    let db =  dst        & 0xFF;
    let r = (sr * sa + dr * inv_a) / 255;
    let g = (sg * sa + dg * inv_a) / 255;
    let b = (sb * sa + db * inv_a) / 255;
    0xFF_00_00_00 | (r << 16) | (g << 8) | b
}

// ─── 4×6 Bitmap Font ─────────────────────────────────────────────────────────
/// Each glyph: 6 rows × 4 bits wide, packed into 3 bytes (24 bits = 4×6).
/// Glyphs for ASCII 0x20 (space) to 0x5F (underscore).
/// This is a minimal 4×6 pixel monochrome font sufficient for debug/UI.
const GLYPH_W: u32 = 4;
const GLYPH_H: u32 = 6;
const FONT_FIRST: u8 = 0x20;
const FONT_LAST:  u8 = 0x7E;

/// Font bitmap: 95 glyphs × 3 bytes each.
/// Each glyph stored as 6 nybbles (top row first), 4 bits per row.
/// Bit 3 = leftmost pixel.
static FONT_BITMAP: &[u8] = &[
    // ' ' 0x20
    0x00, 0x00, 0x00,
    // '!' 0x21
    0x44, 0x44, 0x04,
    // '"' 0x22
    0xAA, 0x00, 0x00,
    // '#' 0x23
    0xAE, 0xEA, 0x00,
    // '$' 0x24
    0x4E, 0x46, 0xE4,
    // '%' 0x25
    0x92, 0x44, 0x90,
    // '&' 0x26
    0x44, 0xA4, 0xA0,
    // ''' 0x27
    0x44, 0x00, 0x00,
    // '(' 0x28
    0x24, 0x44, 0x20,
    // ')' 0x29
    0x84, 0x44, 0x80,
    // '*' 0x2A
    0x00, 0xAE, 0xA0,
    // '+' 0x2B
    0x00, 0x4E, 0x40,
    // ',' 0x2C
    0x00, 0x00, 0x48,
    // '-' 0x2D
    0x00, 0x0E, 0x00,
    // '.' 0x2E
    0x00, 0x00, 0x04,
    // '/' 0x2F
    0x02, 0x44, 0x80,
    // '0' 0x30
    0x6A, 0xAA, 0x60,
    // '1' 0x31
    0x44, 0x44, 0xE0,
    // '2' 0x32
    0xE2, 0x4E, 0xE0,
    // '3' 0x33
    0xE2, 0x62, 0xE0,
    // '4' 0x34
    0xAA, 0xEA, 0x20,
    // '5' 0x35
    0xE8, 0xE2, 0xE0,
    // '6' 0x36
    0x68, 0xEA, 0x60,
    // '7' 0x37
    0xE2, 0x22, 0x20,
    // '8' 0x38
    0x6A, 0x6A, 0x60,
    // '9' 0x39
    0x6A, 0xE2, 0x60,
    // ':' 0x3A
    0x00, 0x40, 0x40,
    // ';' 0x3B
    0x00, 0x40, 0x48,
    // '<' 0x3C
    0x24, 0x84, 0x20,
    // '=' 0x3D
    0x00, 0xE0, 0xE0,
    // '>' 0x3E
    0x84, 0x24, 0x80,
    // '?' 0x3F
    0xE2, 0x40, 0x40,
    // '@' 0x40
    0x6A, 0xEA, 0x60,
    // 'A' 0x41
    0x6A, 0xEA, 0xA0,
    // 'B' 0x42
    0xCA, 0xCA, 0xC0,
    // 'C' 0x43
    0x6A, 0x88, 0x60,
    // 'D' 0x44
    0xCA, 0xAA, 0xC0,
    // 'E' 0x45
    0xE8, 0xC8, 0xE0,
    // 'F' 0x46
    0xE8, 0xC8, 0x80,
    // 'G' 0x47
    0x68, 0xAA, 0x60,
    // 'H' 0x48
    0xAA, 0xEA, 0xA0,
    // 'I' 0x49
    0xE4, 0x44, 0xE0,
    // 'J' 0x4A
    0xE2, 0x2A, 0x40,
    // 'K' 0x4B
    0xAC, 0xCA, 0xA0,
    // 'L' 0x4C
    0x88, 0x88, 0xE0,
    // 'M' 0x4D
    0xAE, 0xAA, 0xA0,
    // 'N' 0x4E
    0xAE, 0xAA, 0xA0,
    // 'O' 0x4F
    0x6A, 0xAA, 0x60,
    // 'P' 0x50
    0xCA, 0xCC, 0x80,
    // 'Q' 0x51
    0x6A, 0xAE, 0x60,
    // 'R' 0x52
    0xCA, 0xCA, 0xA0,
    // 'S' 0x53
    0x68, 0x62, 0xC0,
    // 'T' 0x54
    0xE4, 0x44, 0x40,
    // 'U' 0x55
    0xAA, 0xAA, 0x60,
    // 'V' 0x56
    0xAA, 0xAA, 0x40,
    // 'W' 0x57
    0xAA, 0xAE, 0xA0,
    // 'X' 0x58
    0xAA, 0x4A, 0xA0,
    // 'Y' 0x59
    0xAA, 0x44, 0x40,
    // 'Z' 0x5A
    0xE2, 0x48, 0xE0,
    // '[' 0x5B
    0x64, 0x44, 0x60,
    // '\' 0x5C
    0x80, 0x44, 0x20,
    // ']' 0x5D
    0xC4, 0x44, 0xC0,
    // '^' 0x5E
    0x4A, 0x00, 0x00,
    // '_' 0x5F
    0x00, 0x00, 0xE0,
    // '`' 0x60
    0x80, 0x00, 0x00,
    // 'a' 0x61
    0x00, 0x6A, 0x60,
    // 'b' 0x62
    0x80, 0xCA, 0xC0,
    // 'c' 0x63
    0x00, 0x6A, 0x60, // simplified — same as 'a'
    // 'd' 0x64
    0x20, 0xEA, 0x60,
    // 'e' 0x65
    0x00, 0xEE, 0x60,
    // 'f' 0x66
    0x24, 0xC4, 0x40,
    // 'g' 0x67
    0x00, 0x6A, 0x62,
    // 'h' 0x68
    0x80, 0xCA, 0xA0,
    // 'i' 0x69
    0x40, 0x44, 0x40,
    // 'j' 0x6A
    0x20, 0x22, 0x24,
    // 'k' 0x6B
    0x80, 0xAC, 0xA0,
    // 'l' 0x6C
    0x44, 0x44, 0x40,
    // 'm' 0x6D
    0x00, 0xEA, 0xA0,
    // 'n' 0x6E
    0x00, 0xCA, 0xA0,
    // 'o' 0x6F
    0x00, 0x6A, 0x60,
    // 'p' 0x70
    0x00, 0xCA, 0xC8,
    // 'q' 0x71
    0x00, 0x6A, 0x62,
    // 'r' 0x72
    0x00, 0x6C, 0x80,
    // 's' 0x73
    0x00, 0x6E, 0x60,
    // 't' 0x74
    0x40, 0xE4, 0x40,
    // 'u' 0x75
    0x00, 0xAA, 0x60,
    // 'v' 0x76
    0x00, 0xAA, 0x40,
    // 'w' 0x77
    0x00, 0xAA, 0xE0,
    // 'x' 0x78
    0x00, 0xA4, 0xA0,
    // 'y' 0x79
    0x00, 0xAA, 0x62,
    // 'z' 0x7A
    0x00, 0xE4, 0xE0,
    // '{' 0x7B
    0x24, 0x84, 0x20,
    // '|' 0x7C
    0x44, 0x44, 0x40,
    // '}' 0x7D
    0x84, 0x24, 0x80,
    // '~' 0x7E
    0x5A, 0x00, 0x00,
];

/// Draw a single glyph at pixel position (px, py) with scale factor.
pub fn draw_glyph(fb: &mut Framebuffer<'_>, c: u8, px: i32, py: i32, color: u32, scale: u32) {
    if c < FONT_FIRST || c > FONT_LAST { return; }
    let idx = (c - FONT_FIRST) as usize;
    let base = idx * 3;
    if base + 2 >= FONT_BITMAP.len() { return; }
    // Extract 24 bits = 4×6
    let bits: u32 = ((FONT_BITMAP[base] as u32) << 16)
                  | ((FONT_BITMAP[base + 1] as u32) << 8)
                  |  (FONT_BITMAP[base + 2] as u32);
    for row in 0..GLYPH_H {
        for col in 0..GLYPH_W {
            let bit_idx = (GLYPH_H - 1 - row) * GLYPH_W + (GLYPH_W - 1 - col);
            if (bits >> bit_idx) & 1 == 1 {
                let x = px + (col * scale) as i32;
                let y = py + (row * scale) as i32;
                fb.fill_rect(x, y, scale, scale, color);
            }
        }
    }
}

/// Draw ASCII text.
pub fn draw_text(fb: &mut Framebuffer<'_>, text: &[u8], x: i32, y: i32, color: u32, scale: u32) {
    let mut cx = x;
    let char_w = (GLYPH_W + 1) * scale;
    for &b in text {
        draw_glyph(fb, b, cx, y, color, scale);
        cx += char_w as i32;
        if cx >= fb.width as i32 { break; }
    }
}

// ─── Render Pass ──────────────────────────────────────────────────────────────
pub struct Renderer;

impl Renderer {
    /// Paint entire render tree onto the framebuffer.
    pub fn paint<'fb>(
        fb:     &mut Framebuffer<'fb>,
        engine: &LayoutEngine,
        dom:    &crate::sigma_browser::Dom,
        root:   u16,
    ) {
        // Clear to dark background
        fb.fill_rect(0, 0, fb.width, fb.height, 0xFF_0D_0D_1A);
        Self::paint_box(fb, engine, dom, root);
    }

    fn paint_box(
        fb:     &mut Framebuffer<'_>,
        engine: &LayoutEngine,
        dom:    &crate::sigma_browser::Dom,
        idx:    u16,
    ) {
        if idx == 0xFFFF { return; }
        let rb = &engine.boxes[idx as usize];
        let d = &rb.dims;
        let s = &rb.style;

        if s.display == Display::None { return; }

        // Background
        if s.bg_color != 0 {
            fb.fill_rect(d.x, d.y, d.width, d.height, s.bg_color);
        }

        // Border
        if s.border_w > 0 {
            fb.draw_border(d.x, d.y, d.width, d.height, s.border_w as u32, s.border_color);
        }

        // Text content from Text nodes that are children of this element
        let node = &dom.nodes[rb.dom_node as usize];
        let mut child_dom = node.first_child;
        let mut text_y = d.y + s.padding.top + 2;
        while child_dom != 0xFFFF {
            let cn = &dom.nodes[child_dom as usize];
            if cn.node_type == crate::sigma_browser::NodeType::Text && cn.text_len > 0 {
                let text = dom.slab.get(cn.text_off, cn.text_len);
                // Scale: font_size / 64 / 6 (height of glyph row), clamp 1–4
                let scale = ((s.font_size / 64) / 6).clamp(1, 4) as u32;
                draw_text(fb, text, d.x + s.padding.left + 2, text_y, s.color, scale);
                text_y += ((GLYPH_H + 2) * scale) as i32;
            }
            child_dom = cn.next_sibling;
        }

        // Recurse into child render boxes
        Self::paint_box(fb, engine, dom, rb.first_child);
        // Siblings
        Self::paint_box(fb, engine, dom, rb.next_sib);
    }
}
