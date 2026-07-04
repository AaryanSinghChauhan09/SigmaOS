// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_compositor.rs — Zenith Compositor (Smithay-pattern)
// Language: Rust (std) — OOP via Compositor struct + Window/Surface traits
// Pattern: Scene-graph based compositor, Wayland-inspired protocol

use std::collections::BTreeMap;

// ── Geometry ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default)]
pub struct Rect { pub x: i32, pub y: i32, pub w: u32, pub h: u32 }
#[derive(Clone, Copy, Debug, Default)]
pub struct Point { pub x: i32, pub y: i32 }
#[derive(Clone, Copy, Debug, Default)]
pub struct Size { pub w: u32, pub h: u32 }

impl Rect {
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x && p.x < self.x + self.w as i32
            && p.y >= self.y && p.y < self.y + self.h as i32
    }
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.x + other.w as i32 && self.x + self.w as i32 > other.x
            && self.y < other.y + other.h as i32 && self.y + self.h as i32 > other.y
    }
}

// ── Pixel Format ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat { BGRA8888, RGBA8888, RGB888 }

// ── Surface (compositing unit) ────────────────────────────────────────────────

pub type SurfaceId = u32;

#[derive(Clone, Debug)]
pub struct Surface {
    pub id:       SurfaceId,
    pub geometry: Rect,
    pub z_order:  i32,
    pub opacity:  f32,          // 0.0–1.0
    pub visible:  bool,
    pub title:    String,
    pub buf:      Vec<u8>,      // BGRA pixels
    pub buf_w:    u32,
    pub buf_h:    u32,
    pub damaged:  Vec<Rect>,
    pub focused:  bool,
    pub maximised: bool,
    pub minimised: bool,
    pub decorations: bool,
}

impl Surface {
    pub fn new(id: SurfaceId, x: i32, y: i32, w: u32, h: u32, title: &str) -> Self {
        let buf_sz = (w * h * 4) as usize;
        Self {
            id, geometry: Rect { x, y, w, h }, z_order: id as i32,
            opacity: 1.0, visible: true, title: title.to_owned(),
            buf: vec![0u8; buf_sz], buf_w: w, buf_h: h,
            damaged: vec![Rect { x: 0, y: 0, w, h }],
            focused: false, maximised: false, minimised: false, decorations: true,
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, b: u8, g: u8, r: u8, a: u8) {
        if x >= self.buf_w || y >= self.buf_h { return; }
        let off = ((y * self.buf_w + x) * 4) as usize;
        if off + 3 < self.buf.len() {
            self.buf[off] = b; self.buf[off+1] = g;
            self.buf[off+2] = r; self.buf[off+3] = a;
        }
    }

    pub fn fill(&mut self, b: u8, g: u8, r: u8, a: u8) {
        for chunk in self.buf.chunks_exact_mut(4) {
            chunk[0] = b; chunk[1] = g; chunk[2] = r; chunk[3] = a;
        }
        self.damaged.push(Rect { x: 0, y: 0, w: self.buf_w, h: self.buf_h });
    }

    pub fn damage(&mut self, r: Rect) { self.damaged.push(r); }
    pub fn clear_damage(&mut self) { self.damaged.clear(); }
    pub fn is_dirty(&self) -> bool { !self.damaged.is_empty() }
}

// ── Output (display) ──────────────────────────────────────────────────────────

pub struct Output {
    pub id:     u32,
    pub size:   Size,
    pub fb:     Vec<u8>,  // backing framebuffer (BGRA)
    pub scale:  f32,
}

impl Output {
    pub fn new(id: u32, w: u32, h: u32) -> Self {
        Self { id, size: Size { w, h }, fb: vec![0u8; (w * h * 4) as usize], scale: 1.0 }
    }

    /// Blit a surface into the output framebuffer with alpha compositing
    pub fn composite_surface(&mut self, surface: &Surface) {
        if !surface.visible || surface.minimised { return; }
        let src_x = surface.geometry.x.max(0) as u32;
        let src_y = surface.geometry.y.max(0) as u32;
        let dst_w = surface.geometry.w.min(self.size.w.saturating_sub(src_x));
        let dst_h = surface.geometry.h.min(self.size.h.saturating_sub(src_y));

        for row in 0..dst_h {
            for col in 0..dst_w {
                let sb = ((row * surface.buf_w + col) * 4) as usize;
                if sb + 3 >= surface.buf.len() { break; }
                let alpha = (surface.buf[sb+3] as f32 / 255.0) * surface.opacity;
                let db = (((src_y + row) * self.size.w + (src_x + col)) * 4) as usize;
                if db + 3 >= self.fb.len() { break; }
                // Alpha blend: dst = src * alpha + dst * (1 - alpha)
                for c in 0..3 {
                    let src_c = surface.buf[sb+c] as f32;
                    let dst_c = self.fb[db+c] as f32;
                    self.fb[db+c] = (src_c * alpha + dst_c * (1.0 - alpha)) as u8;
                }
                self.fb[db+3] = 255;
            }
        }
    }
}

// ── Compositor ────────────────────────────────────────────────────────────────

pub struct Compositor {
    surfaces:    BTreeMap<SurfaceId, Surface>,
    output:      Output,
    next_id:     SurfaceId,
    focused:     Option<SurfaceId>,
    pointer:     Point,
    cursor_size: u32,
}

impl Compositor {
    pub fn new(output_w: u32, output_h: u32) -> Self {
        Self {
            surfaces: BTreeMap::new(),
            output:   Output::new(0, output_w, output_h),
            next_id:  1,
            focused:  None,
            pointer:  Point { x: (output_w / 2) as i32, y: (output_h / 2) as i32 },
            cursor_size: 12,
        }
    }

    pub fn create_surface(&mut self, x: i32, y: i32, w: u32, h: u32, title: &str) -> SurfaceId {
        let id = self.next_id; self.next_id += 1;
        self.surfaces.insert(id, Surface::new(id, x, y, w, h, title));
        if self.focused.is_none() { self.set_focus(Some(id)); }
        id
    }

    pub fn destroy_surface(&mut self, id: SurfaceId) {
        self.surfaces.remove(&id);
        if self.focused == Some(id) {
            self.focused = self.surfaces.keys().last().copied();
        }
    }

    pub fn get_surface_mut(&mut self, id: SurfaceId) -> Option<&mut Surface> {
        self.surfaces.get_mut(&id)
    }

    pub fn set_focus(&mut self, id: Option<SurfaceId>) {
        if let Some(old) = self.focused {
            if let Some(s) = self.surfaces.get_mut(&old) { s.focused = false; }
        }
        self.focused = id;
        if let Some(new) = id {
            if let Some(s) = self.surfaces.get_mut(&new) { s.focused = true; }
        }
    }

    pub fn move_surface(&mut self, id: SurfaceId, dx: i32, dy: i32) {
        if let Some(s) = self.surfaces.get_mut(&id) {
            s.geometry.x += dx; s.geometry.y += dy;
            s.damage(Rect { x: 0, y: 0, w: s.geometry.w, h: s.geometry.h });
        }
    }

    pub fn raise_surface(&mut self, id: SurfaceId) {
        let max_z = self.surfaces.values().map(|s| s.z_order).max().unwrap_or(0);
        if let Some(s) = self.surfaces.get_mut(&id) { s.z_order = max_z + 1; }
    }

    pub fn pointer_move(&mut self, dx: i32, dy: i32) {
        self.pointer.x = (self.pointer.x + dx)
            .clamp(0, self.output.size.w as i32 - 1);
        self.pointer.y = (self.pointer.y + dy)
            .clamp(0, self.output.size.h as i32 - 1);
    }

    pub fn pointer_click(&mut self, _button: u8) {
        // Find topmost surface under pointer
        let p = self.pointer;
        let top = self.surfaces.values()
            .filter(|s| s.visible && !s.minimised && s.geometry.contains(p))
            .max_by_key(|s| s.z_order)
            .map(|s| s.id);
        if top.is_some() {
            self.raise_surface(top.unwrap());
            self.set_focus(top);
        }
    }

    /// Composite all visible surfaces + cursor into output framebuffer
    pub fn render(&mut self) {
        // Clear background with dark colour
        let bg = 0x07u8;
        for chunk in self.output.fb.chunks_exact_mut(4) {
            chunk[0] = bg; chunk[1] = bg + 2; chunk[2] = bg + 3; chunk[3] = 255;
        }

        // Sort surfaces by z-order, back-to-front
        let mut order: Vec<SurfaceId> = self.surfaces.keys().copied().collect();
        order.sort_by_key(|id| self.surfaces[id].z_order);

        for id in order {
            let surface = &self.surfaces[&id];
            // Move surface out of borrow temporarily
            let s_clone = surface.clone();
            self.output.composite_surface(&s_clone);
        }

        // Draw cursor
        self.draw_cursor();

        // Clear damage on all surfaces
        for s in self.surfaces.values_mut() { s.clear_damage(); }
    }

    fn draw_cursor(&mut self) {
        let cx = self.pointer.x;
        let cy = self.pointer.y;
        let sz = self.cursor_size as i32;
        for dy in 0..sz {
            for dx in 0..sz {
                let x = cx + dx;
                let y = cy + dy;
                if x < 0 || y < 0 || x >= self.output.size.w as i32
                   || y >= self.output.size.h as i32 { continue; }
                // Simple arrow shape: fill triangle-ish area
                if dx + dy < sz {
                    let off = ((y as u32 * self.output.size.w + x as u32) * 4) as usize;
                    if off + 3 < self.output.fb.len() {
                        self.output.fb[off]   = 255;
                        self.output.fb[off+1] = 255;
                        self.output.fb[off+2] = 255;
                        self.output.fb[off+3] = 255;
                    }
                }
            }
        }
    }

    pub fn framebuffer(&self) -> &[u8] { &self.output.fb }
    pub fn output_size(&self) -> Size  { self.output.size }
    pub fn focused_surface(&self) -> Option<SurfaceId> { self.focused }
    pub fn pointer_pos(&self) -> Point { self.pointer }
    pub fn surface_count(&self) -> usize { self.surfaces.len() }
}
