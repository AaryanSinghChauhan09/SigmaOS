// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_widgets.rs — UI Widget Toolkit
// Language: Rust (std) — OOP via Widget trait + concrete widget types

use std::collections::BTreeMap;
use crate::userland::desktop::sigma_compositor::{Rect, Point};
use crate::userland::desktop::sigma_theme::{Color, ThemeEngine};

// ── Input Events ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum InputEvent {
    MouseMove(Point),
    MouseDown { pos: Point, button: u8 },
    MouseUp   { pos: Point, button: u8 },
    KeyDown   { scancode: u8, ascii: u8 },
    KeyUp     { scancode: u8 },
    Scroll    { delta: i32 },
    TextInput(String),
}

// ── Widget State ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct WidgetState { pub hovered: bool, pub pressed: bool, pub focused: bool, pub disabled: bool }

// ── Widget Trait (OOP) ────────────────────────────────────────────────────────

pub type WidgetId = u32;

pub trait Widget: Send {
    fn id(&self)         -> WidgetId;
    fn bounds(&self)     -> Rect;
    fn set_bounds(&mut self, r: Rect);
    fn visible(&self)    -> bool;
    fn set_visible(&mut self, v: bool);
    fn state(&self)      -> WidgetState;
    fn handle(&mut self, ev: &InputEvent) -> Option<WidgetEvent>;
    fn paint(&self, canvas: &mut Canvas, theme: &ThemeEngine);
}

// ── Widget Events ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum WidgetEvent {
    Clicked { id: WidgetId },
    Changed { id: WidgetId, value: String },
    Submitted { id: WidgetId, text: String },
    Focused { id: WidgetId },
    Blurred { id: WidgetId },
}

// ── Canvas (software renderer) ────────────────────────────────────────────────

pub struct Canvas {
    pub buf:    Vec<u8>,   // BGRA pixels
    pub width:  u32,
    pub height: u32,
}

impl Canvas {
    pub fn new(w: u32, h: u32) -> Self {
        Self { buf: vec![0u8; (w*h*4) as usize], width: w, height: h }
    }

    pub fn fill_rect(&mut self, r: Rect, c: Color) {
        for y in r.y.max(0) as u32 .. (r.y + r.h as i32).min(self.height as i32) as u32 {
            for x in r.x.max(0) as u32 .. (r.x + r.w as i32).min(self.width as i32) as u32 {
                let off = ((y * self.width + x) * 4) as usize;
                if off + 3 < self.buf.len() {
                    let a = c.a as f32 / 255.0;
                    let bi = self.buf[off]   as f32;
                    let gi = self.buf[off+1] as f32;
                    let ri = self.buf[off+2] as f32;
                    self.buf[off]   = (c.b as f32 * a + bi * (1.0-a)) as u8;
                    self.buf[off+1] = (c.g as f32 * a + gi * (1.0-a)) as u8;
                    self.buf[off+2] = (c.r as f32 * a + ri * (1.0-a)) as u8;
                    self.buf[off+3] = 255;
                }
            }
        }
    }

    pub fn draw_rect_border(&mut self, r: Rect, c: Color, thickness: u32) {
        for t in 0..thickness {
            let t = t as i32;
            let inner = Rect { x: r.x+t, y: r.y+t,
                               w: r.w.saturating_sub(2*t as u32),
                               h: r.h.saturating_sub(2*t as u32) };
            // Top, bottom, left, right lines
            self.hline(inner.x, inner.y, inner.w, c);
            self.hline(inner.x, inner.y + inner.h as i32 - 1, inner.w, c);
            self.vline(inner.x, inner.y, inner.h, c);
            self.vline(inner.x + inner.w as i32 - 1, inner.y, inner.h, c);
        }
    }

    fn hline(&mut self, x: i32, y: i32, len: u32, c: Color) {
        let py = y.clamp(0, self.height as i32 - 1) as u32;
        for px in (x.max(0) as u32)..(x + len as i32).min(self.width as i32) as u32 {
            let off = ((py * self.width + px) * 4) as usize;
            if off + 3 < self.buf.len() {
                self.buf[off]=c.b; self.buf[off+1]=c.g; self.buf[off+2]=c.r; self.buf[off+3]=c.a;
            }
        }
    }
    fn vline(&mut self, x: i32, y: i32, len: u32, c: Color) {
        let px = x.clamp(0, self.width as i32 - 1) as u32;
        for py in (y.max(0) as u32)..(y + len as i32).min(self.height as i32) as u32 {
            let off = ((py * self.width + px) * 4) as usize;
            if off + 3 < self.buf.len() {
                self.buf[off]=c.b; self.buf[off+1]=c.g; self.buf[off+2]=c.r; self.buf[off+3]=c.a;
            }
        }
    }
}

// ── Base Widget ───────────────────────────────────────────────────────────────

struct BaseWidget { id: WidgetId, bounds: Rect, visible: bool, state: WidgetState }
impl BaseWidget {
    fn new(id: WidgetId, bounds: Rect) -> Self {
        Self { id, bounds, visible: true, state: WidgetState::default() }
    }
    fn hit(&self, p: Point) -> bool { self.bounds.contains(p) }
}

// ── Button ────────────────────────────────────────────────────────────────────

pub struct Button {
    base:  BaseWidget,
    label: String,
    icon:  Option<String>,
}

impl Button {
    pub fn new(id: WidgetId, bounds: Rect, label: &str) -> Self {
        Self { base: BaseWidget::new(id, bounds), label: label.to_owned(), icon: None }
    }
    pub fn with_icon(mut self, icon: &str) -> Self { self.icon = Some(icon.to_owned()); self }
}

impl Widget for Button {
    fn id(&self)       -> WidgetId    { self.base.id }
    fn bounds(&self)   -> Rect        { self.base.bounds }
    fn set_bounds(&mut self, r: Rect) { self.base.bounds = r; }
    fn visible(&self)  -> bool        { self.base.visible }
    fn set_visible(&mut self, v:bool) { self.base.visible = v; }
    fn state(&self)    -> WidgetState { self.base.state }

    fn handle(&mut self, ev: &InputEvent) -> Option<WidgetEvent> {
        match ev {
            InputEvent::MouseMove(p) => {
                self.base.state.hovered = self.base.hit(*p); None
            }
            InputEvent::MouseDown { pos, button: 1 } if self.base.hit(*pos) => {
                self.base.state.pressed = true; None
            }
            InputEvent::MouseUp { pos, button: 1 } if self.base.state.pressed => {
                self.base.state.pressed = false;
                if self.base.hit(*pos) { Some(WidgetEvent::Clicked { id: self.base.id }) }
                else { None }
            }
            _ => None
        }
    }

    fn paint(&self, canvas: &mut Canvas, theme: &ThemeEngine) {
        if !self.base.visible { return; }
        let bg = if self.base.state.pressed      { theme.get_color("accent").with_alpha(230) }
                 else if self.base.state.hovered { theme.get_color("accent").with_alpha(180) }
                 else                            { theme.get_color("surface") };
        let border = if self.base.state.hovered || self.base.state.pressed {
            theme.get_color("accent")
        } else { theme.get_color("border") };
        canvas.fill_rect(self.base.bounds, bg);
        canvas.draw_rect_border(self.base.bounds, border, 1);
    }
}

// ── TextInput ─────────────────────────────────────────────────────────────────

pub struct TextInput {
    base:        BaseWidget,
    text:        String,
    placeholder: String,
    cursor:      usize,
    max_len:     usize,
    password:    bool,
}

impl TextInput {
    pub fn new(id: WidgetId, bounds: Rect, placeholder: &str) -> Self {
        Self {
            base: BaseWidget::new(id, bounds), text: String::new(),
            placeholder: placeholder.to_owned(), cursor: 0, max_len: 512, password: false,
        }
    }
    pub fn password(mut self) -> Self { self.password = true; self }
    pub fn text(&self) -> &str { &self.text }
    pub fn set_text(&mut self, t: &str) { self.text = t.to_owned(); self.cursor = self.text.len(); }
}

impl Widget for TextInput {
    fn id(&self)         -> WidgetId    { self.base.id }
    fn bounds(&self)     -> Rect        { self.base.bounds }
    fn set_bounds(&mut self, r: Rect)   { self.base.bounds = r; }
    fn visible(&self)    -> bool        { self.base.visible }
    fn set_visible(&mut self, v: bool)  { self.base.visible = v; }
    fn state(&self)      -> WidgetState { self.base.state }

    fn handle(&mut self, ev: &InputEvent) -> Option<WidgetEvent> {
        match ev {
            InputEvent::MouseDown { pos, .. } => {
                self.base.state.focused = self.base.hit(*pos);
                if self.base.state.focused { Some(WidgetEvent::Focused { id: self.base.id }) }
                else                       { Some(WidgetEvent::Blurred { id: self.base.id }) }
            }
            InputEvent::TextInput(s) if self.base.state.focused => {
                if self.text.len() < self.max_len {
                    self.text.insert_str(self.cursor, s);
                    self.cursor += s.len();
                    Some(WidgetEvent::Changed { id: self.base.id, value: self.text.clone() })
                } else { None }
            }
            InputEvent::KeyDown { scancode: 0x28, .. } if self.base.state.focused => {
                // Enter
                Some(WidgetEvent::Submitted { id: self.base.id, text: self.text.clone() })
            }
            InputEvent::KeyDown { scancode: 0x2A, .. } if self.base.state.focused => {
                // Backspace
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.text.remove(self.cursor);
                    Some(WidgetEvent::Changed { id: self.base.id, value: self.text.clone() })
                } else { None }
            }
            _ => None
        }
    }

    fn paint(&self, canvas: &mut Canvas, theme: &ThemeEngine) {
        if !self.base.visible { return; }
        let bg = theme.get_color("surface");
        let border = if self.base.state.focused { theme.get_color("accent") }
                     else { theme.get_color("border") };
        canvas.fill_rect(self.base.bounds, bg);
        canvas.draw_rect_border(self.base.bounds, border, if self.base.state.focused { 2 } else { 1 });
    }
}

// ── Widget Registry ───────────────────────────────────────────────────────────

pub struct WidgetRegistry {
    widgets: BTreeMap<WidgetId, Box<dyn Widget>>,
    next_id: WidgetId,
    canvas:  Canvas,
}

impl WidgetRegistry {
    pub fn new(w: u32, h: u32) -> Self {
        Self { widgets: BTreeMap::new(), next_id: 1, canvas: Canvas::new(w, h) }
    }

    pub fn alloc_id(&mut self) -> WidgetId { let id = self.next_id; self.next_id += 1; id }
    pub fn add(&mut self, w: Box<dyn Widget>) { let id = w.id(); self.widgets.insert(id, w); }
    pub fn remove(&mut self, id: WidgetId)    { self.widgets.remove(&id); }

    pub fn dispatch(&mut self, ev: &InputEvent) -> Vec<WidgetEvent> {
        let mut out = Vec::new();
        for w in self.widgets.values_mut() {
            if let Some(we) = w.handle(ev) { out.push(we); }
        }
        out
    }

    pub fn paint_all(&mut self, theme: &ThemeEngine) -> &[u8] {
        // Clear canvas
        self.canvas.buf.fill(0);
        let ids: Vec<WidgetId> = self.widgets.keys().copied().collect();
        for id in ids {
            if let Some(w) = self.widgets.get(&id) {
                let w_ref: &dyn Widget = w.as_ref();
                // Paint into canvas — we need unsafe reborrow here
                let canvas = &mut self.canvas;
                w_ref.paint(canvas, theme);
            }
        }
        &self.canvas.buf
    }

    pub fn get_mut(&mut self, id: WidgetId) -> Option<&mut Box<dyn Widget>> {
        self.widgets.get_mut(&id)
    }
    pub fn count(&self) -> usize { self.widgets.len() }
}
