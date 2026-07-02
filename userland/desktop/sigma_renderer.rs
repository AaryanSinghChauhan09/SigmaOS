// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_renderer.rs — Software Renderer (2D, GPU-ready)
// Language: Rust (std) — OOP via Renderer + DrawCmd

use crate::userland::desktop::sigma_theme::Color;
use crate::userland::desktop::sigma_compositor::Rect;

// ── Draw Commands ─────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub enum DrawCmd {
    FillRect   { rect: Rect, color: Color, radius: u32 },
    StrokeRect { rect: Rect, color: Color, width: u32, radius: u32 },
    FillCircle { cx: i32, cy: i32, r: u32, color: Color },
    Line       { x1:i32,y1:i32, x2:i32,y2:i32, color:Color, width:u32 },
    GlassPanel { rect: Rect, bg: Color, border: Color, blur: u32, radius: u32 },
    Image      { rect: Rect, data: Vec<u8>, img_w: u32, img_h: u32 },
    Text       { x:i32, y:i32, text:String, color:Color, size:u32 },
    Clip       { rect: Rect },
    UnClip,
}

// ── Framebuffer ────────────────────────────────────────────────────────────────
pub struct Framebuffer { pub buf: Vec<u8>, pub w: u32, pub h: u32 }

impl Framebuffer {
    pub fn new(w: u32, h: u32) -> Self {
        Self { buf: vec![0u8; (w*h*4) as usize], w, h }
    }
    #[inline]
    pub fn put(&mut self, x: u32, y: u32, c: Color) {
        if x >= self.w || y >= self.h { return; }
        let o = ((y*self.w+x)*4) as usize;
        if o+3 >= self.buf.len() { return; }
        let a = c.a as f32 / 255.0;
        self.buf[o]   = (c.b as f32 * a + self.buf[o]   as f32 * (1.0-a)) as u8;
        self.buf[o+1] = (c.g as f32 * a + self.buf[o+1] as f32 * (1.0-a)) as u8;
        self.buf[o+2] = (c.r as f32 * a + self.buf[o+2] as f32 * (1.0-a)) as u8;
        self.buf[o+3] = 255;
    }
    pub fn clear(&mut self, c: Color) {
        for ch in self.buf.chunks_exact_mut(4) {
            ch[0]=c.b; ch[1]=c.g; ch[2]=c.r; ch[3]=c.a;
        }
    }
}

// ── Renderer ──────────────────────────────────────────────────────────────────
pub struct Renderer { pub fb: Framebuffer, clip: Option<Rect> }

impl Renderer {
    pub fn new(w: u32, h: u32) -> Self { Self { fb: Framebuffer::new(w,h), clip: None } }

    pub fn execute(&mut self, cmd: &DrawCmd) {
        match cmd {
            DrawCmd::FillRect { rect, color, radius }   => self.fill_rect(*rect, *color, *radius),
            DrawCmd::StrokeRect{rect,color,width,radius} => self.stroke_rect(*rect,*color,*width,*radius),
            DrawCmd::FillCircle{cx,cy,r,color}           => self.fill_circle(*cx,*cy,*r,*color),
            DrawCmd::Line{x1,y1,x2,y2,color,width}      => self.line(*x1,*y1,*x2,*y2,*color,*width),
            DrawCmd::GlassPanel{rect,bg,border,blur,radius} => self.glass_panel(*rect,*bg,*border,*blur,*radius),
            DrawCmd::Image{rect,data,img_w,img_h}        => self.blit_image(*rect,data,*img_w,*img_h),
            DrawCmd::Clip{rect}  => self.clip = Some(*rect),
            DrawCmd::UnClip      => self.clip = None,
            DrawCmd::Text{..}    => {} // Text rendering requires glyph atlas — stub
        }
    }

    pub fn exec_list(&mut self, cmds: &[DrawCmd]) { for c in cmds { self.execute(c); } }

    fn clipped(&self, x: i32, y: i32) -> bool {
        if let Some(r) = self.clip {
            x < r.x || x >= r.x+r.w as i32 || y < r.y || y >= r.y+r.h as i32
        } else { false }
    }

    fn fill_rect(&mut self, r: Rect, c: Color, radius: u32) {
        for y in r.y.max(0) as u32 .. (r.y+r.h as i32).min(self.fb.h as i32) as u32 {
            for x in r.x.max(0) as u32 .. (r.x+r.w as i32).min(self.fb.w as i32) as u32 {
                if self.clipped(x as i32, y as i32) { continue; }
                if radius > 0 {
                    let dx = (x as i32 - r.x).min(r.w as i32 - 1 - (x as i32 - r.x));
                    let dy = (y as i32 - r.y).min(r.h as i32 - 1 - (y as i32 - r.y));
                    if (dx as u32) < radius && (dy as u32) < radius {
                        let ox = radius as i32 - dx - 1;
                        let oy = radius as i32 - dy - 1;
                        if ox*ox + oy*oy > (radius as i32 * radius as i32) { continue; }
                    }
                }
                self.fb.put(x, y, c);
            }
        }
    }

    fn stroke_rect(&mut self, r: Rect, c: Color, w: u32, _radius: u32) {
        for t in 0..w {
            let t = t as i32;
            self.hline(r.x, r.y+t, r.w, c);
            self.hline(r.x, r.y+r.h as i32-1-t, r.w, c);
            self.vline(r.x+t, r.y, r.h, c);
            self.vline(r.x+r.w as i32-1-t, r.y, r.h, c);
        }
    }

    fn fill_circle(&mut self, cx: i32, cy: i32, rad: u32, c: Color) {
        let r = rad as i32;
        for dy in -r..=r { for dx in -r..=r {
            if dx*dx + dy*dy <= r*r {
                let px = cx+dx; let py = cy+dy;
                if !self.clipped(px,py) { self.fb.put(px as u32, py as u32, c); }
            }
        }}
    }

    fn line(&mut self, x1:i32,y1:i32,x2:i32,y2:i32, c:Color, _w:u32) {
        // Bresenham
        let (mut x, mut y) = (x1, y1);
        let (dx, dy) = ((x2-x1).abs(), (y2-y1).abs());
        let sx = if x1<x2 {1} else {-1};
        let sy = if y1<y2 {1} else {-1};
        let mut err = dx - dy;
        loop {
            if !self.clipped(x,y) { self.fb.put(x as u32, y as u32, c); }
            if x==x2 && y==y2 { break; }
            let e2 = 2*err;
            if e2 > -dy { err -= dy; x += sx; }
            if e2 <  dx { err += dx; y += sy; }
        }
    }

    fn glass_panel(&mut self, r: Rect, bg: Color, border: Color, blur: u32, radius: u32) {
        // Background fill with semi-transparency
        self.fill_rect(r, bg, radius);
        // Simple blur approximation: mix with surrounding pixels
        if blur > 0 {
            // Box blur — average 3×3 neighbourhood
            for y in r.y.max(1) as u32 .. (r.y+r.h as i32-1).min(self.fb.h as i32-1) as u32 {
                for x in r.x.max(1) as u32 .. (r.x+r.w as i32-1).min(self.fb.w as i32-1) as u32 {
                    let mut rb=0u32; let mut gb=0u32; let mut bb=0u32;
                    for ky in 0u32..3 { for kx in 0u32..3 {
                        let px = (x + kx).saturating_sub(1); let py = (y+ky).saturating_sub(1);
                        let o = ((py*self.fb.w+px)*4) as usize;
                        if o+3 < self.fb.buf.len() {
                            bb += self.fb.buf[o] as u32;
                            gb += self.fb.buf[o+1] as u32;
                            rb += self.fb.buf[o+2] as u32;
                        }
                    }}
                    let o = ((y*self.fb.w+x)*4) as usize;
                    if o+3 < self.fb.buf.len() {
                        self.fb.buf[o]  = (bb/9) as u8;
                        self.fb.buf[o+1]= (gb/9) as u8;
                        self.fb.buf[o+2]= (rb/9) as u8;
                    }
                }
            }
        }
        self.stroke_rect(r, border, 1, radius);
    }

    fn blit_image(&mut self, r: Rect, data: &[u8], img_w: u32, img_h: u32) {
        for dy in 0..r.h { for dx in 0..r.w {
            let sx = (dx as u64 * img_w as u64 / r.w as u64) as u32;
            let sy = (dy as u64 * img_h as u64 / r.h as u64) as u32;
            let src = ((sy*img_w+sx)*4) as usize;
            if src+3 < data.len() {
                let c = Color::rgba(data[src+2], data[src+1], data[src], data[src+3]);
                self.fb.put((r.x as u32+dx).min(self.fb.w-1), (r.y as u32+dy).min(self.fb.h-1), c);
            }
        }}
    }

    fn hline(&mut self, x:i32,y:i32,len:u32,c:Color) {
        let py = y.clamp(0,self.fb.h as i32-1) as u32;
        for px in (x.max(0) as u32)..(x+len as i32).min(self.fb.w as i32) as u32 {
            self.fb.put(px,py,c);
        }
    }
    fn vline(&mut self, x:i32,y:i32,len:u32,c:Color) {
        let px = x.clamp(0,self.fb.w as i32-1) as u32;
        for py in (y.max(0) as u32)..(y+len as i32).min(self.fb.h as i32) as u32 {
            self.fb.put(px,py,c);
        }
    }

    pub fn framebuffer(&self) -> &[u8] { &self.fb.buf }
    pub fn size(&self) -> (u32,u32) { (self.fb.w, self.fb.h) }
}
