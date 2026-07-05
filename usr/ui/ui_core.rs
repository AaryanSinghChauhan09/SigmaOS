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

    /// Arrange children horizontally. Since we are no_std / no alloc,
    /// we expect layout dimensions to be updated directly on the child components.
    pub fn arrange_layout(&mut self, widths: &[SigmaU32]) {
        let mut current_x = self.bounds.x;
        for i in 0..self.child_count {
            let width = if i < widths.len() { widths[i] } else { 50 };
            // In a full registry system, children bounds are set via lookup:
            // registry.get_mut(self.child_ids[i]).set_bounds(Rect { x: current_x, y: self.bounds.y, w: width, h: self.bounds.h });
            current_x += width + self.spacing;
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
        // Draw a simulated solid border color in the frame buffer
        let start_y = self.bounds.y as usize;
        let end_y = (self.bounds.y + self.bounds.h) as usize;
        let start_x = self.bounds.x as usize;
        let end_x = (self.bounds.x + self.bounds.w) as usize;

        // Visual feedback based on state
        let color: u8 = if self.is_pressed { 0x55 } else if self.is_hovered { 0xAA } else { 0xFF };

        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = y * stride + x;
                if index < buffer.len() {
                    buffer[index] = color;
                }
            }
        }
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
                    self.is_pressed = !self.is_pressed;
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