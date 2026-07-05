/// SigmaOS: usr/ui/ui_core.rs
/// Object-Oriented UI Framework (No Alloc).
/// Provides the base Widget trait and Layout containers.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaUsize = usize;
type SigmaBool  = bool;

pub const MAX_CHILDREN: SigmaUsize = 16;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EventType {
    MouseClick { x: SigmaU32, y: SigmaU32 },
    MouseHover { x: SigmaU32, y: SigmaU32 },
    KeyPress { keycode: SigmaU32 },
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub x: SigmaU32,
    pub y: SigmaU32,
    pub w: SigmaU32,
    pub h: SigmaU32,
}

// ─── OOP Trait Definition ─────────────────────────────────────────────────────

pub trait Widget {
    fn draw(&self, buffer: &mut [u8], stride: usize);
    fn handle_event(&mut self, event: EventType) -> SigmaBool;
    fn set_bounds(&mut self, rect: Rect);
    fn get_bounds(&self) -> Rect;
}

// ─── Basic Containers ─────────────────────────────────────────────────────────

#[derive(Copy, Clone)]
pub struct HBox {
    pub bounds: Rect,
    // For no_std, we use indices into a global widget registry,
    // or fixed arrays if the specific widgets are known.
    // For this demonstration, we store fixed IDs of registered children.
    pub child_ids: [SigmaU32; MAX_CHILDREN],
    pub child_count: SigmaUsize,
    pub spacing: SigmaU32,
}

impl HBox {
    pub const fn new(spacing: SigmaU32) -> Self {
        HBox {
            bounds: Rect { x: 0, y: 0, w: 0, h: 0 },
            child_ids: [0; MAX_CHILDREN],
            child_count: 0,
            spacing,
        }
    }
    
    pub fn add_child(&mut self, id: SigmaU32) -> SigmaBool {
        if self.child_count < MAX_CHILDREN {
            self.child_ids[self.child_count] = id;
            self.child_count += 1;
            true
        } else {
            false
        }
    }
}

// The global widget registry would hold implementations of Widget, allowing
// HBox to query sizes and calculate layouts dynamically.

// ─── Button Widget ────────────────────────────────────────────────────────────

#[derive(Copy, Clone)]
pub struct Button {
    pub bounds: Rect,
    pub label: [u8; 32],
    pub label_len: SigmaUsize,
    pub is_hovered: SigmaBool,
    pub is_pressed: SigmaBool,
}

impl Widget for Button {
    fn draw(&self, buffer: &mut [u8], stride: usize) {
        // Draw rectangle based on bounds and hover/press state
    }
    
    fn handle_event(&mut self, event: EventType) -> SigmaBool {
        match event {
            EventType::MouseHover { x, y } => {
                let inside = x >= self.bounds.x && x <= self.bounds.x + self.bounds.w &&
                             y >= self.bounds.y && y <= self.bounds.y + self.bounds.h;
                if inside != self.is_hovered {
                    self.is_hovered = inside;
                    return true; // Needs redraw
                }
            },
            EventType::MouseClick { x, y } => {
                let inside = x >= self.bounds.x && x <= self.bounds.x + self.bounds.w &&
                             y >= self.bounds.y && y <= self.bounds.y + self.bounds.h;
                if inside {
                    self.is_pressed = true;
                    // Trigger callback via IPC
                    return true;
                }
            },
            _ => {}
        }
        false
    }
    
    fn set_bounds(&mut self, rect: Rect) { self.bounds = rect; }
    fn get_bounds(&self) -> Rect { self.bounds }
}