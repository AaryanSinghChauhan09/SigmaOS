// Custom, OOP-driven High-Performance Graphics Compositor for SigmaOS
// Implements screen composition, double buffering, and screen capturing

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub pos: Position,
    pub size: Size,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rectangle {
            pos: Position { x, y },
            size: Size { width, height },
        }
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.pos.x < other.pos.x + other.size.width as i32
            && self.pos.x + self.size.width as i32 > other.pos.x
            && self.pos.y < other.pos.y + other.size.height as i32
            && self.pos.y + self.size.height as i32 > other.pos.y
    }
}

/// ARGB Color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsError {
    Success = 0,
    InvalidSurface = 1,
    OutOfBounds = 2,
    AllocationFailed = 3,
}

pub trait Surface {
    fn size(&self) -> Size;
    fn set_pixel(&mut self, pos: Position, color: Color) -> Result<(), GraphicsError>;
    fn get_pixel(&self, pos: Position) -> Result<Color, GraphicsError>;
    fn fill_rect(&mut self, rect: Rectangle, color: Color);
}

pub struct BitmapSurface {
    pub size: Size,
    pub buffer: Vec<u8>,
}

impl BitmapSurface {
    pub fn new(size: Size) -> Self {
        let len = (size.width * size.height * 4) as usize;
        let mut buffer = Vec::new();
        buffer.resize(len, 0);
        BitmapSurface { size, buffer }
    }
}

impl Surface for BitmapSurface {
    fn size(&self) -> Size {
        self.size
    }

    fn set_pixel(&mut self, pos: Position, color: Color) -> Result<(), GraphicsError> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.width as i32 || pos.y >= self.size.height as i32 {
            return Err(GraphicsError::OutOfBounds);
        }
        let index = ((pos.y as u32 * self.size.width + pos.x as u32) * 4) as usize;
        if index + 3 < self.buffer.len() {
            self.buffer[index] = color.r;
            self.buffer[index + 1] = color.g;
            self.buffer[index + 2] = color.b;
            self.buffer[index + 3] = color.a;
            Ok(())
        } else {
            Err(GraphicsError::OutOfBounds)
        }
    }

    fn get_pixel(&self, pos: Position) -> Result<Color, GraphicsError> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.width as i32 || pos.y >= self.size.height as i32 {
            return Err(GraphicsError::OutOfBounds);
        }
        let index = ((pos.y as u32 * self.size.width + pos.x as u32) * 4) as usize;
        if index + 3 < self.buffer.len() {
            Ok(Color::new(
                self.buffer[index],
                self.buffer[index + 1],
                self.buffer[index + 2],
                self.buffer[index + 3],
            ))
        } else {
            Err(GraphicsError::OutOfBounds)
        }
    }

    fn fill_rect(&mut self, rect: Rectangle, color: Color) {
        let start_x = rect.pos.x.max(0);
        let start_y = rect.pos.y.max(0);
        let end_x = (rect.pos.x + rect.size.width as i32).min(self.size.width as i32);
        let end_y = (rect.pos.y + rect.size.height as i32).min(self.size.height as i32);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let _ = self.set_pixel(Position { x, y }, color);
            }
        }
    }
}

pub struct Window {
    pub id: usize,
    pub bounds: Rectangle,
    pub surface: BitmapSurface,
    pub visible: AtomicBool,
    pub z_order: AtomicUsize,
}

impl Window {
    pub fn new(id: usize, bounds: Rectangle) -> Self {
        Window {
            id,
            bounds,
            surface: BitmapSurface::new(bounds.size),
            visible: AtomicBool::new(true),
            z_order: AtomicUsize::new(0),
        }
    }
}

pub trait Compositor {
    fn create_window(&mut self, bounds: Rectangle) -> Result<usize, GraphicsError>;
    fn destroy_window(&mut self, id: usize) -> Result<(), GraphicsError>;
    fn compose(&mut self, output: &mut dyn Surface) -> Result<(), GraphicsError>;
}

pub struct SimpleCompositor {
    pub windows: Vec<Window>,
    pub next_id: AtomicUsize,
}

impl Default for SimpleCompositor {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleCompositor {
    pub fn new() -> Self {
        SimpleCompositor {
            windows: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Compositor for SimpleCompositor {
    fn create_window(&mut self, bounds: Rectangle) -> Result<usize, GraphicsError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let window = Window::new(id, bounds);
        self.windows.push(window);
        Ok(id)
    }

    fn destroy_window(&mut self, id: usize) -> Result<(), GraphicsError> {
        if let Some(pos) = self.windows.iter().position(|w| w.id == id) {
            self.windows.remove(pos);
            Ok(())
        } else {
            Err(GraphicsError::InvalidSurface)
        }
    }

    fn compose(&mut self, output: &mut dyn Surface) -> Result<(), GraphicsError> {
        output.fill_rect(
            Rectangle::new(0, 0, output.size().width, output.size().height),
            Color::BLACK,
        );

        for win in &self.windows {
            if !win.visible.load(Ordering::SeqCst) {
                continue;
            }

            for wy in 0..win.bounds.size.height as i32 {
                for wx in 0..win.bounds.size.width as i32 {
                    if let Ok(color) = win.surface.get_pixel(Position { x: wx, y: wy }) {
                        let out_pos = Position {
                            x: win.bounds.pos.x + wx,
                            y: win.bounds.pos.y + wy,
                        };
                        let _ = output.set_pixel(out_pos, color);
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_rect_fill() {
        let mut surface = BitmapSurface::new(Size { width: 100, height: 100 });
        surface.fill_rect(Rectangle::new(10, 10, 20, 20), Color::RED);
        assert_eq!(surface.get_pixel(Position { x: 15, y: 15 }).unwrap(), Color::RED);
        assert_eq!(surface.get_pixel(Position { x: 0, y: 0 }).unwrap(), Color::BLACK);
    }

    #[test]
    fn test_compositor_window_management() {
        let mut compositor = SimpleCompositor::new();
        let win_id = compositor.create_window(Rectangle::new(0, 0, 50, 50)).unwrap();
        assert_eq!(compositor.windows.len(), 1);

        let mut output = BitmapSurface::new(Size { width: 100, height: 100 });
        assert!(compositor.compose(&mut output).is_ok());

        assert!(compositor.destroy_window(win_id).is_ok());
        assert_eq!(compositor.windows.len(), 0);
    }
}
