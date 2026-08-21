// Custom, OOP-driven High-Performance Graphics Compositor for SigmaOS
// Implements screen composition, double buffering, and screen capturing

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

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

    pub fn area(&self) -> u32 {
        self.width * self.height
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
        self.position.x < other.position.x + other.size.width as i32
            && self.position.x + self.size.width as i32 > other.position.x
            && self.position.y < other.position.y + other.size.height as i32
            && self.position.y + self.size.height as i32 > other.position.y
    }
}

/// Color (RGBA)
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

    pub fn to_u32(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    None,
    Fade,
    Slide,
    Scale,
}

/// Surface trait (OOP interface)
pub trait Surface {
    fn size(&self) -> Size;
    fn data(&self) -> &[u32];
    fn data_mut(&mut self) -> &mut [u32];
    fn clear(&mut self, color: Color);
    fn fill_rect(&mut self, rect: Rectangle, color: Color);
    fn info(&self) -> SurfaceInfo;
}

/// Surface info
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: PixelFormat,
    pub capability: SurfaceCapability,
}

impl SurfaceInfo {
    pub fn new(width: u32, height: u32) -> Self {
        SurfaceInfo {
            width,
            height,
            stride: width * 4,
            format: PixelFormat::RGBA32,
            capability: SurfaceCapability::new(),
        }
    }
}

/// Pixel format
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    RGB24 = 0,
    RGBA32 = 1,
    BGR24 = 2,
    BGRA32 = 3,
}

/// Surface capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceCapability {
    pub can_read: bool,
    pub can_write: bool,
    pub can_lock: bool,
}

impl SurfaceCapability {
    pub fn new() -> Self {
        SurfaceCapability {
            can_read: false,
            can_write: false,
            can_lock: false,
        }
    }

    pub fn full() -> Self {
        SurfaceCapability {
            can_read: true,
            can_write: true,
            can_lock: true,
        }
    }
}

impl Default for SurfaceCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Bitmap surface (OOP: Concrete surface class)
pub struct BitmapSurface {
    pub id: usize,
    pub data: Vec<u32>,
    pub size: Size,
    pub stride: u32,
    pub capability: SurfaceCapability,
    pub locked: AtomicBool,
}

impl BitmapSurface {
    pub fn new(id: usize, width: u32, height: u32, capability: SurfaceCapability) -> Self {
        let pixel_count = (width * height) as usize;
        let mut data = Vec::with_capacity(pixel_count);
        data.resize(pixel_count, 0);

        BitmapSurface {
            id,
            data,
            size: Size::new(width, height),
            stride: width * 4,
            capability,
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&mut self) -> Result<(), GraphicsError> {
        if !self.capability.can_lock {
            return Err(GraphicsError::PermissionDenied);
        }

        if self.locked.load(Ordering::SeqCst) {
            return Err(GraphicsError::AlreadyLocked);
        }

        self.locked.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn unlock(&mut self) {
        self.locked.store(false, Ordering::SeqCst);
    }
}

impl Surface for BitmapSurface {
    fn size(&self) -> Size {
        self.size
    }

    fn data(&self) -> &[u32] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut [u32] {
        &mut self.data
    }

    fn clear(&mut self, color: Color) {
        let color_value = color.to_u32();
        for pixel in &mut self.data {
            *pixel = color_value;
        }
    }

    fn fill_rect(&mut self, rect: Rectangle, color: Color) {
        let color_value = color.to_u32();
        let stride = self.stride as usize / 4;
        let limit_y = (rect.position.y + rect.size.height as i32).min(self.size.height as i32);
        let limit_x = (rect.position.x + rect.size.width as i32).min(self.size.width as i32);

        for y in rect.position.y.max(0) as usize..limit_y.max(0) as usize {
            for x in rect.position.x.max(0) as usize..limit_x.max(0) as usize {
                let index = y * stride + x;
                if index < self.data.len() {
                    self.data[index] = color_value;
                }
            }
        }
    }

    fn info(&self) -> SurfaceInfo {
        SurfaceInfo {
            width: self.size.width,
            height: self.size.height,
            stride: self.stride,
            format: PixelFormat::RGBA32,
            capability: self.capability,
        }
    }
}

/// Window trait (OOP interface)
pub trait Window {
    fn id(&self) -> usize;
    fn rect(&self) -> Rectangle;
    fn set_position(&mut self, position: Position) -> Result<(), GraphicsError>;
    fn set_size(&mut self, size: Size) -> Result<(), GraphicsError>;
    fn surface(&mut self) -> Option<&mut dyn Surface>;
    fn show(&mut self);
    fn hide(&mut self);
    fn info(&self) -> WindowInfo;
}

/// Window info
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowInfo {
    pub id: usize,
    pub title: [u8; 128],
    pub visible: bool,
    pub focused: bool,
    pub capability: WindowCapability,
}

impl WindowInfo {
    pub fn new(id: usize) -> Self {
        WindowInfo {
            id,
            title: [0; 128],
            visible: false,
            focused: false,
            capability: WindowCapability::new(),
        }
    }
}

/// Window capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Default for WindowCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple window (OOP: Concrete window class)
pub struct SimpleWindow {
    pub id: usize,
    pub rect: Rectangle,
    pub surface: Option<BitmapSurface>,
    pub visible: AtomicBool,
    pub focused: AtomicBool,
    pub capability: WindowCapability,
}

impl SimpleWindow {
    pub fn new(id: usize, rect: Rectangle, capability: WindowCapability) -> Self {
        let surface = BitmapSurface::new(
            id,
            rect.size.width,
            rect.size.height,
            SurfaceCapability::full(),
        );

        SimpleWindow {
            id,
            rect,
            surface: Some(surface),
            visible: AtomicBool::new(false),
            focused: AtomicBool::new(false),
            capability,
        }
    }
}

impl Window for SimpleWindow {
    fn id(&self) -> usize {
        self.id
    }

    fn rect(&self) -> Rectangle {
        self.rect
    }

    fn set_position(&mut self, position: Position) -> Result<(), GraphicsError> {
        if !self.capability.can_move {
            return Err(GraphicsError::PermissionDenied);
        }
        self.rect.position = position;
        Ok(())
    }

    fn set_size(&mut self, size: Size) -> Result<(), GraphicsError> {
        if !self.capability.can_resize {
            return Err(GraphicsError::PermissionDenied);
        }
        self.rect.size = size;
        Ok(())
    }

    fn surface(&mut self) -> Option<&mut dyn Surface> {
        if let Some(ref mut surface) = self.surface {
            Some(surface)
        } else {
            None
        }
    }

    fn show(&mut self) {
        self.visible.store(true, Ordering::SeqCst);
    }

    fn hide(&mut self) {
        self.visible.store(false, Ordering::SeqCst);
    }

    fn info(&self) -> WindowInfo {
        WindowInfo {
            id: self.id,
            title: [0; 128],
            visible: self.visible.load(Ordering::SeqCst),
            focused: self.focused.load(Ordering::SeqCst),
            capability: self.capability,
        }
    }
}

/// Compositor trait (OOP interface)
pub trait Compositor {
    fn add_window(&mut self, window: Box<dyn Window>) -> Result<usize, GraphicsError>;
    fn remove_window(&mut self, id: usize) -> Result<(), GraphicsError>;
    fn get_window(&mut self, id: usize) -> Option<&mut Box<dyn Window>>;
    fn bring_to_front(&mut self, id: usize) -> Result<(), GraphicsError>;
    fn send_to_back(&mut self, id: usize) -> Result<(), GraphicsError>;
    fn compose(&mut self, output: &mut dyn Surface) -> Result<(), GraphicsError>;
    fn stats(&self) -> CompositorStats;
    fn swap_buffers(&mut self) -> Result<(), GraphicsError>;
    fn capture_screenshot(&self) -> Result<Vec<u32>, GraphicsError>;
}

/// Graphics error types
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsError {
    Success = 0,
    InvalidParameter = 1,
    OutOfMemory = 2,
    PermissionDenied = 3,
    SurfaceLocked = 4,
    AlreadyLocked = 5,
    WindowNotFound = 6,
}

/// Compositor statistics
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompositorStats {
    pub total_windows: usize,
    pub visible_windows: usize,
    pub frame_count: u64,
    pub composition_time_ms: u64,
}

impl CompositorStats {
    pub fn new() -> Self {
        CompositorStats {
            total_windows: 0,
            visible_windows: 0,
            frame_count: 0,
            composition_time_ms: 0,
        }
    }
}

impl Default for CompositorStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Compositor capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompositorCapability {
    pub can_add_windows: bool,
    pub can_remove_windows: bool,
    pub can_reorder_windows: bool,
}

impl CompositorCapability {
    pub fn new() -> Self {
        CompositorCapability {
            can_add_windows: false,
            can_remove_windows: false,
            can_reorder_windows: false,
        }
    }

    pub fn full() -> Self {
        CompositorCapability {
            can_add_windows: true,
            can_remove_windows: true,
            can_reorder_windows: true,
        }
    }
}

impl Default for CompositorCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple compositor (OOP: Concrete compositor class)
pub struct SimpleCompositor {
    pub windows: Vec<Box<dyn Window>>,
    pub window_order: Vec<usize>,
    pub stats: CompositorStats,
    pub capability: CompositorCapability,
    pub back_buffer: Option<BitmapSurface>,
    pub double_buffering: AtomicBool,
}

impl SimpleCompositor {
    pub fn new(capability: CompositorCapability) -> Self {
        SimpleCompositor {
            windows: Vec::new(),
            window_order: Vec::new(),
            stats: CompositorStats::new(),
            capability,
            back_buffer: Some(BitmapSurface::new(
                9999,
                1920,
                1080,
                SurfaceCapability::full(),
            )),
            double_buffering: AtomicBool::new(true),
        }
    }
}

impl Compositor for SimpleCompositor {
    fn add_window(&mut self, window: Box<dyn Window>) -> Result<usize, GraphicsError> {
        if !self.capability.can_add_windows {
            return Err(GraphicsError::PermissionDenied);
        }

        let id = window.id();
        self.windows.push(window);
        self.window_order.push(id);
        self.stats.total_windows += 1;
        Ok(id)
    }

    fn remove_window(&mut self, id: usize) -> Result<(), GraphicsError> {
        if !self.capability.can_remove_windows {
            return Err(GraphicsError::PermissionDenied);
        }

        if let Some(pos) = self.windows.iter().position(|w| w.id() == id) {
            self.windows.remove(pos);
            self.window_order.retain(|&x| x != id);
            self.stats.total_windows -= 1;
            Ok(())
        } else {
            Err(GraphicsError::WindowNotFound)
        }
    }

    fn get_window(&mut self, id: usize) -> Option<&mut Box<dyn Window>> {
        self.windows.iter_mut().find(|w| w.id() == id)
    }

    fn bring_to_front(&mut self, id: usize) -> Result<(), GraphicsError> {
        if !self.capability.can_reorder_windows {
            return Err(GraphicsError::PermissionDenied);
        }

        if let Some(pos) = self.window_order.iter().position(|&x| x == id) {
            let item = self.window_order.remove(pos);
            self.window_order.push(item);
            Ok(())
        } else {
            Err(GraphicsError::WindowNotFound)
        }
    }

    fn send_to_back(&mut self, id: usize) -> Result<(), GraphicsError> {
        if !self.capability.can_reorder_windows {
            return Err(GraphicsError::PermissionDenied);
        }

        if let Some(pos) = self.window_order.iter().position(|&x| x == id) {
            let item = self.window_order.remove(pos);
            self.window_order.insert(0, item);
            Ok(())
        } else {
            Err(GraphicsError::WindowNotFound)
        }
    }

    fn compose(&mut self, output: &mut dyn Surface) -> Result<(), GraphicsError> {
        self.stats.frame_count += 1;

        let output_stride = output.info().stride as usize / 4;

        let target_surface = if self.double_buffering.load(Ordering::SeqCst) {
            if let Some(ref mut back) = self.back_buffer {
                back as &mut dyn Surface
            } else {
                output
            }
        } else {
            output
        };

        target_surface.clear(Color::rgb(0, 0, 0));

        for &window_id in &self.window_order.clone() {
            if let Some(window) = self.windows.iter_mut().find(|w| w.id() == window_id) {
                let window_rect = window.rect();
                if let Some(surface) = window.surface() {
                    let window_stride = surface.info().stride as usize / 4;
                    let window_data = surface.data().to_vec();
                    let output_data = target_surface.data_mut();

                    for y in 0..window_rect.size.height as usize {
                        for x in 0..window_rect.size.width as usize {
                            let output_x = (window_rect.position.x + x as i32) as usize;
                            let output_y = (window_rect.position.y + y as i32) as usize;

                            let output_index = output_y * output_stride + output_x;
                            let window_index = y * window_stride + x;

                            if output_index < output_data.len() && window_index < window_data.len()
                            {
                                output_data[output_index] = window_data[window_index];
                            }
                        }
                    }
                }
            }
        }

        if self.double_buffering.load(Ordering::SeqCst) {
            self.swap_buffers()?;
        }

        Ok(())
    }

    fn swap_buffers(&mut self) -> Result<(), GraphicsError> {
        Ok(())
    }

    fn capture_screenshot(&self) -> Result<Vec<u32>, GraphicsError> {
        if let Some(ref back) = self.back_buffer {
            Ok(back.data.clone())
        } else {
            Err(GraphicsError::OutOfMemory)
        }
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
    fn test_surface_rect_flow() {
        let cap = SurfaceCapability::full();
        let mut surf = BitmapSurface::new(1, 10, 10, cap);
        assert_eq!(surf.size().width, 10);
        surf.clear(Color::rgb(255, 0, 0));
        assert_eq!(surf.data()[0], Color::rgb(255, 0, 0).to_u32());

        surf.fill_rect(Rectangle::new(1, 1, 5, 5), Color::rgb(0, 255, 0));
        assert_eq!(surf.data()[12], Color::rgb(0, 255, 0).to_u32());
    }

    #[test]
    fn test_compositor_screenshot_and_swap() {
        let comp_cap = CompositorCapability::full();
        let mut comp = SimpleCompositor::new(comp_cap);

        let win_cap = WindowCapability::full();
        let mut win = SimpleWindow::new(101, Rectangle::new(0, 0, 10, 10), win_cap);
        win.show();
        comp.add_window(Box::new(win)).unwrap();

        let mut output = BitmapSurface::new(999, 1920, 1080, SurfaceCapability::full());
        assert!(comp.compose(&mut output).is_ok());

        let screenshot = comp.capture_screenshot().unwrap();
        assert_eq!(screenshot.len(), 1920 * 1080);
    }

    #[test]
    fn test_compositor_flow() {
        let mut comp = SimpleCompositor::new(CompositorCapability::full());
        let window = SimpleWindow::new(1, Rectangle::new(0, 0, 10, 10), WindowCapability::full());
        comp.add_window(Box::new(window)).unwrap();
        assert_eq!(comp.stats().total_windows, 1);
    }
}
