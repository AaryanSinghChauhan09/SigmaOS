// OOP-based Graphics Compositor for SigmaOS
// Implements graphics composition using OOP principles with traits and structs
// No dependency on external graphics frameworks
#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Position
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

/// Size
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }
}

/// Rectangle
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub position: Position,
    pub size: Size,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rectangle {
            position: Position::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub fn contains(&self, point: Position) -> bool {
        point.x >= self.position.x
            && point.x < self.position.x + self.size.width as i32
            && point.y >= self.position.y
            && point.y < self.position.y + self.size.height as i32
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        !(self.position.x + self.size.width as i32 <= other.position.x
            || other.position.x + other.size.width as i32 <= self.position.x
            || self.position.y + self.size.height as i32 <= other.position.y
            || other.position.y + other.size.height as i32 <= self.position.y)
    }
}

/// Color
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::new(r, g, b, 255)
    }
}

/// Surface info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SurfaceInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

impl SurfaceInfo {
    pub fn new(width: u32, height: u32, stride: u32, format: u32) -> Self {
        SurfaceInfo {
            width,
            height,
            stride,
            format,
        }
    }
}

/// Surface trait (OOP interface)
pub trait Surface {
    /// Get surface info
    fn info(&self) -> SurfaceInfo;
    /// Get surface data
    fn data(&self) -> &[u32];
    /// Get mutable surface data
    fn data_mut(&mut self) -> &mut [u32];
    /// Clear surface
    fn clear(&mut self, color: Color);
}

/// Simple surface (OOP: Concrete surface class)
pub struct SimpleSurface {
    info: SurfaceInfo,
    data: Vec<u32>,
}

impl SimpleSurface {
    pub fn new(width: u32, height: u32) -> Self {
        let stride = width;
        let data_size = (stride * height) as usize;
        SimpleSurface {
            info: SurfaceInfo::new(width, height, stride, 0),
            data: vec![0; data_size],
        }
    }
}

impl Surface for SimpleSurface {
    fn info(&self) -> SurfaceInfo {
        self.info
    }

    fn data(&self) -> &[u32] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut [u32] {
        &mut self.data
    }

    fn clear(&mut self, color: Color) {
        let color_value = ((color.a as u32) << 24)
            | ((color.r as u32) << 16)
            | ((color.g as u32) << 8)
            | (color.b as u32);
        for pixel in self.data.iter_mut() {
            *pixel = color_value;
        }
    }
}

/// Window capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WindowCapability {
    pub can_move: bool,
    pub can_resize: bool,
    pub can_close: bool,
    pub can_minimize: bool,
    pub can_maximize: bool,
}

impl WindowCapability {
    pub fn new() -> Self {
        WindowCapability {
            can_move: false,
            can_resize: false,
            can_close: false,
            can_minimize: false,
            can_maximize: false,
        }
    }

    pub fn full() -> Self {
        WindowCapability {
            can_move: true,
            can_resize: true,
            can_close: true,
            can_minimize: true,
            can_maximize: true,
        }
    }
}

/// Window info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WindowInfo {
    pub id: usize,
    pub title: [u8; 64],
    pub rect: Rectangle,
    pub visible: bool,
    pub focused: bool,
    pub capability: WindowCapability,
}

impl WindowInfo {
    pub fn new(id: usize, rect: Rectangle) -> Self {
        WindowInfo {
            id,
            title: [0; 64],
            rect,
            visible: false,
            focused: false,
            capability: WindowCapability::new(),
        }
    }
}

/// Window trait (OOP interface)
pub trait Window {
    /// Get window ID
    fn id(&self) -> usize;
    /// Get window info
    fn info(&self) -> WindowInfo;
    /// Get window surface
    fn surface(&self) -> Option<&dyn Surface>;
    /// Get mutable window surface
    fn surface_mut(&mut self) -> Option<&mut dyn Surface>;
    /// Set window position
    fn set_position(&mut self, position: Position) -> Result<(), CompositorError>;
    /// Set window size
    fn set_size(&mut self, size: Size) -> Result<(), CompositorError>;
    /// Show window
    fn show(&mut self);
    /// Hide window
    fn hide(&mut self);
    /// Close window
    fn close(&mut self) -> Result<(), CompositorError>;
}

/// Simple window (OOP: Concrete window class)
pub struct SimpleWindow {
    info: WindowInfo,
    surface: SimpleSurface,
}

impl SimpleWindow {
    pub fn new(id: usize, rect: Rectangle, capability: WindowCapability) -> Self {
        SimpleWindow {
            info: WindowInfo {
                id,
                title: [0; 64],
                rect,
                visible: false,
                focused: false,
                capability,
            },
            surface: SimpleSurface::new(rect.size.width, rect.size.height),
        }
    }
}

impl Window for SimpleWindow {
    fn id(&self) -> usize {
        self.info.id
    }

    fn info(&self) -> WindowInfo {
        self.info
    }

    fn surface(&self) -> Option<&dyn Surface> {
        Some(&self.surface)
    }

    fn surface_mut(&mut self) -> Option<&mut dyn Surface> {
        Some(&mut self.surface)
    }

    fn set_position(&mut self, position: Position) -> Result<(), CompositorError> {
        if !self.info.capability.can_move {
            return Err(CompositorError::PermissionDenied);
        }
        self.info.rect.position = position;
        Ok(())
    }

    fn set_size(&mut self, size: Size) -> Result<(), CompositorError> {
        if !self.info.capability.can_resize {
            return Err(CompositorError::PermissionDenied);
        }
        self.info.rect.size = size;
        self.surface = SimpleSurface::new(size.width, size.height);
        Ok(())
    }

    fn show(&mut self) {
        self.info.visible = true;
    }

    fn hide(&mut self) {
        self.info.visible = false;
    }

    fn close(&mut self) -> Result<(), CompositorError> {
        if !self.info.capability.can_close {
            return Err(CompositorError::PermissionDenied);
        }
        self.info.visible = false;
        Ok(())
    }
}

/// Compositor error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorError {
    Success = 0,
    InvalidParameter = 1,
    PermissionDenied = 2,
    NotFound = 3,
    AlreadyExists = 4,
    SurfaceLost = 5,
}

/// Compositor capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CompositorCapability {
    pub can_compose: bool,
    pub can_capture: bool,
    pub can_animate: bool,
}

impl CompositorCapability {
    pub fn new() -> Self {
        CompositorCapability {
            can_compose: false,
            can_capture: false,
            can_animate: false,
        }
    }

    pub fn full() -> Self {
        CompositorCapability {
            can_compose: true,
            can_capture: true,
            can_animate: true,
        }
    }
}

/// Compositor stats
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CompositorStats {
    pub total_windows: usize,
    pub visible_windows: usize,
    pub frame_count: usize,
    pub composition_time_us: u64,
}

impl CompositorStats {
    pub fn new() -> Self {
        CompositorStats {
            total_windows: 0,
            visible_windows: 0,
            frame_count: 0,
            composition_time_us: 0,
        }
    }
}

/// Compositor trait (OOP interface)
pub trait Compositor {
    /// Add window
    fn add_window(&mut self, window: Box<dyn Window>) -> Result<(), CompositorError>;
    /// Remove window
    fn remove_window(&mut self, id: usize) -> Result<(), CompositorError>;
    /// Get window
    fn get_window(&self, id: usize) -> Option<&dyn Window>;
    /// Get mutable window
    fn get_window_mut(&mut self, id: usize) -> Option<&mut dyn Window>;
    /// Compose frame
    fn compose(&mut self, output: &mut dyn Surface) -> Result<(), CompositorError>;
    /// Get stats
    fn stats(&self) -> CompositorStats;
}

/// Simple compositor (OOP: Concrete compositor class)
pub struct SimpleCompositor {
    windows: Vec<Box<dyn Window>>,
    next_id: AtomicUsize,
    stats: CompositorStats,
    capability: CompositorCapability,
    double_buffering: AtomicBool,
}

impl SimpleCompositor {
    pub fn new(capability: CompositorCapability) -> Self {
        SimpleCompositor {
            windows: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: CompositorStats::new(),
            capability,
            double_buffering: AtomicBool::new(false),
        }
    }

    pub fn set_double_buffering(&self, enabled: bool) {
        self.double_buffering.store(enabled, Ordering::SeqCst);
    }

    pub fn swap_buffers(&mut self) -> Result<(), CompositorError> {
        // In a real implementation, this would swap the front and back buffers
        Ok(())
    }
}

impl Compositor for SimpleCompositor {
    fn add_window(&mut self, window: Box<dyn Window>) -> Result<(), CompositorError> {
        if !self.capability.can_compose {
            return Err(CompositorError::PermissionDenied);
        }
        self.windows.push(window);
        self.stats.total_windows = self.windows.len();
        Ok(())
    }

    fn remove_window(&mut self, id: usize) -> Result<(), CompositorError> {
        if let Some(index) = self.windows.iter().position(|w| w.id() == id) {
            self.windows.remove(index);
            self.stats.total_windows = self.windows.len();
            Ok(())
        } else {
            Err(CompositorError::NotFound)
        }
    }

    fn get_window(&self, id: usize) -> Option<&dyn Window> {
        self.windows.iter().find(|w| w.id() == id).map(|w| w.as_ref())
    }

    fn get_window_mut(&mut self, id: usize) -> Option<&mut dyn Window> {
        self.windows
            .iter_mut()
            .find(|w| w.id() == id)
            .map(|w| w.as_mut())
    }

    fn compose(&mut self, output: &mut dyn Surface) -> Result<(), CompositorError> {
        if !self.capability.can_compose {
            return Err(CompositorError::PermissionDenied);
        }

        let output_info = output.info();
        let output_stride = output_info.stride as usize;
        let output_data = output.data_mut();

        // Clear output surface
        output.clear(Color::rgb(0, 0, 0));

        // Compose windows in order (back to front)
        for window in &self.windows {
            if !window.info().visible {
                continue;
            }

            if let Some(surface) = window.surface() {
                let window_info = window.info();
                let window_rect = window_info.rect;
                let window_stride = surface.info().stride as usize;
                let window_data = surface.data();

                // Copy window surface to output
                for y in 0..window_rect.size.height as usize {
                    for x in 0..window_rect.size.width as usize {
                        let output_x = (window_rect.position.x + x as i32) as usize;
                        let output_y = (window_rect.position.y + y as i32) as usize;

                        let output_index = output_y * output_stride + output_x;
                        let window_index = y * window_stride + x;

                        if output_index < output_data.len() && window_index < window_data.len() {
                            output_data[output_index] = window_data[window_index];
                        }
                    }
                }
            }
        }

        self.stats.frame_count += 1;

        if self.double_buffering.load(Ordering::SeqCst) {
            self.swap_buffers()?;
        }

        Ok(())
    }

    fn stats(&self) -> CompositorStats {
        let mut stats = self.stats;
        stats.visible_windows = self.windows.iter().filter(|w| w.info().visible).count();
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compositor_flow() {
        let mut comp = SimpleCompositor::new(CompositorCapability::full());
        let window = SimpleWindow::new(1, Rectangle::new(0, 0, 10, 10), WindowCapability::full());
        comp.add_window(Box::new(window)).unwrap();
        assert_eq!(comp.stats().total_windows, 1);
    }

    #[test]
    fn test_rectangle_operations() {
        let rect1 = Rectangle::new(0, 0, 100, 100);
        let rect2 = Rectangle::new(50, 50, 100, 100);
        
        assert!(rect1.intersects(&rect2));
        
        let point = Position::new(10, 10);
        assert!(rect1.contains(point));
        
        let point_outside = Position::new(150, 150);
        assert!(!rect1.contains(point_outside));
    }

    #[test]
    fn test_window_lifecycle() {
        let mut window = SimpleWindow::new(1, Rectangle::new(0, 0, 100, 100), WindowCapability::full());
        
        assert!(!window.info().visible);
        window.show();
        assert!(window.info().visible);
        window.hide();
        assert!(!window.info().visible);
        
        assert!(window.set_position(Position::new(50, 50)).is_ok());
        assert_eq!(window.info().rect.position, Position::new(50, 50));
    }
}
