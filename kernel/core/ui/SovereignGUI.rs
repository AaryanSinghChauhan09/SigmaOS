// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/ui/SovereignGUI.rs — GUI Engine
//
// Implements the core GUI engine for SigmaOS desktop environment.
// Provides framebuffer management, drawing primitives, and window compositing.
// Inspired by: Wayland, X11, Windows DWM
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum screen width.
const MAX_WIDTH: SigmaUsize = 3840;
/// Maximum screen height.
const MAX_HEIGHT: SigmaUsize = 2160;
/// Maximum number of windows.
const MAX_WINDOWS: SigmaUsize = 64;
/// Window title length.
const WINDOW_TITLE_LEN: SigmaUsize = 64;

// ── Color ───────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: SigmaU8,
    pub g: SigmaU8,
    pub b: SigmaU8,
    pub a: SigmaU8,
}

impl Color {
    pub const fn rgb(r: SigmaU8, g: SigmaU8, b: SigmaU8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

// ── Rect ───────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rect {
    pub x: SigmaI32,
    pub y: SigmaI32,
    pub width: SigmaU32,
    pub height: SigmaU32,
}

// ── Window ─────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Window {
    pub window_id: SigmaU32,
    pub title: [SigmaU8; WINDOW_TITLE_LEN],
    pub rect: Rect,
    pub visible: SigmaBool,
    pub focused: SigmaBool,
    pub _pad: [SigmaU8; 7],
}

// ── Framebuffer ─────────────────────────────────────────────────────────────
pub struct Framebuffer {
    /// Pixel data (RGBA).
    pixels: [Color; MAX_WIDTH * MAX_HEIGHT],
    /// Width.
    width: SigmaU32,
    /// Height.
    height: SigmaU32,
    /// Dirty flag.
    dirty: SigmaBool,
}

impl Framebuffer {
    pub const fn new() -> Self {
        Self {
            pixels: [Color::rgb(0, 0, 0); MAX_WIDTH * MAX_HEIGHT],
            width: 0,
            height: 0,
            dirty: false,
        }
    }

    pub fn set_resolution(&mut self, width: SigmaU32, height: SigmaU32) -> SigmaI32 {
        if width as SigmaUsize > MAX_WIDTH || height as SigmaUsize > MAX_HEIGHT {
            return -1;
        }
        self.width = width;
        self.height = height;
        0
    }

    pub fn draw_pixel(&mut self, x: SigmaI32, y: SigmaI32, color: Color) -> SigmaI32 {
        if x < 0 || y < 0 || x as SigmaUsize >= self.width as SigmaUsize || y as SigmaUsize >= self.height as SigmaUsize {
            return -1;
        }
        let idx = (y as SigmaUsize * self.width as SigmaUsize + x as SigmaUsize);
        if idx < MAX_WIDTH * MAX_HEIGHT {
            self.pixels[idx] = color;
            self.dirty = true;
        }
        0
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        for y in rect.y..(rect.y + rect.height as SigmaI32) {
            for x in rect.x..(rect.x + rect.width as SigmaI32) {
                self.draw_pixel(x, y, color);
            }
        }
    }

    pub fn clear(&mut self, color: Color) {
        for i in 0..(self.width as SigmaUsize * self.height as SigmaUsize) {
            if i < MAX_WIDTH * MAX_HEIGHT {
                self.pixels[i] = color;
            }
        }
        self.dirty = true;
    }
}

// ── GUI Engine ─────────────────────────────────────────────────────────────
pub struct GUIEngine {
    /// Framebuffer.
    fb: Framebuffer,
    /// Windows.
    windows: [Window; MAX_WINDOWS],
    /// Window count.
    window_count: SigmaUsize,
    /// Next window ID.
    next_window_id: AtomicU32,
    /// Initialized flag.
    initialized: SigmaBool,
}

impl GUIEngine {
    pub const fn new() -> Self {
        Self {
            fb: Framebuffer::new(),
            windows: [Window {
                window_id: 0,
                title: [0u8; WINDOW_TITLE_LEN],
                rect: Rect { x: 0, y: 0, width: 0, height: 0 },
                visible: false,
                focused: false,
                _pad: [0u8; 7],
            }; MAX_WINDOWS],
            window_count: 0,
            next_window_id: AtomicU32::new(1),
            initialized: false,
        }
    }

    pub fn init(&mut self) -> Self {
        self.initialized = true;
        self.fb.set_resolution(1920, 1080);
        *self
    }

    pub fn create_window(&mut self, title: &[SigmaU8], rect: Rect) -> SigmaU32 {
        if self.window_count >= MAX_WINDOWS {
            return 0;
        }
        let id = self.next_window_id.fetch_add(1, Ordering::SeqCst);
        let idx = self.window_count;
        self.windows[idx].window_id = id;
        self.windows[idx].rect = rect;
        self.windows[idx].visible = true;
        let len = title.len().min(WINDOW_TITLE_LEN - 1);
        let mut i = 0;
        while i < len {
            self.windows[idx].title[i] = title[i];
            i += 1;
        }
        self.windows[idx].title[len] = 0;
        self.window_count += 1;
        id
    }

    pub fn destroy_window(&mut self, window_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.window_count {
            if self.windows[i].window_id == window_id {
                self.windows[i] = Window {
                    window_id: 0,
                    title: [0u8; WINDOW_TITLE_LEN],
                    rect: Rect { x: 0, y: 0, width: 0, height: 0 },
                    visible: false,
                    focused: false,
                    _pad: [0u8; 7],
                };
                self.window_count -= 1;
                return 0;
            }
        }
        -1
    }

    pub fn draw_window(&mut self, window_id: SigmaU32) {
        for i in 0..self.window_count {
            if self.windows[i].window_id == window_id && self.windows[i].visible {
                let rect = self.windows[i].rect;
                self.fb.fill_rect(rect, Color::rgb(50, 50, 50));
                // Draw window border
                let border = Rect {
                    x: rect.x, y: rect.y,
                    width: rect.width, height: 2,
                };
                self.fb.fill_rect(border, Color::rgb(100, 100, 100));
            }
        }
    }

    pub fn compose(&mut self) {
        self.fb.clear(Color::rgb(20, 20, 30));
        for i in 0..self.window_count {
            if self.windows[i].visible {
                self.draw_window(self.windows[i].window_id);
            }
        }
    }

    pub fn flush(&mut self) {
        // In production: copy framebuffer to GPU/display
        self.fb.dirty = false;
    }
}

static mut G_GUI: Option<GUIEngine> = None;

#[no_mangle]
pub unsafe extern "C" fn gui_init() {
    G_GUI = Some(GUIEngine::new().init());
}

#[no_mangle]
pub unsafe extern "C" fn gui_create_window(
    title: *const SigmaU8,
    title_len: SigmaUsize,
    x: SigmaI32,
    y: SigmaI32,
    width: SigmaU32,
    height: SigmaU32,
) -> SigmaU32 {
    if let Some(ref mut gui) = G_GUI {
        let t = core::slice::from_raw_parts(title, title_len.min(WINDOW_TITLE_LEN));
        let rect = Rect { x, y, width, height };
        gui.create_window(t, rect)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn gui_destroy_window(window_id: SigmaU32) -> SigmaI32 {
    if let Some(ref mut gui) = G_GUI {
        gui.destroy_window(window_id)
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn gui_compose() {
    if let Some(ref mut gui) = G_GUI {
        gui.compose();
    }
}

#[no_mangle]
pub unsafe extern "C" fn gui_flush() {
    if let Some(ref mut gui) = G_GUI {
        gui.flush();
    }
}

