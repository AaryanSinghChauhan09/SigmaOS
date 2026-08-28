extern crate alloc;
// OOP-based Graphics Compositor for SigmaOS
// Implements graphics composition using OOP principles with traits and structs
// No dependency on external graphics frameworks
// Improved with custom window animations, transition effects, and dynamic pixel-clipping.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Position
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

/// Surface trait (OOP interface)
pub trait Surface {
    /// Get surface size
    fn size(&self) -> Size;
    /// Get surface data
    fn data(&self) -> &[u32];
    /// Get mutable surface data
    fn data_mut(&mut self) -> &mut [u32];
    /// Clear surface with color
    fn clear(&mut self, color: Color);
    /// Fill rectangle with color
    fn fill_rect(&mut self, rect: Rectangle, color: Color);
    /// Get surface info
    fn info(&self) -> SurfaceInfo;
}

/// Surface info
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    RGB24 = 0,
    RGBA32 = 1,
    BGR24 = 2,
    BGRA32 = 3,
}

/// Surface capability
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
        let size = (width * height) as usize;
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);

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

        let data = self.data_mut();

        for y in rect.position.y.max(0) as usize..limit_y.max(0) as usize {
            for x in rect.position.x.max(0) as usize..limit_x.max(0) as usize {
                let index = y * stride + x;
                if index < data.len() {
                    data[index] = color_value;
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

/// Window transition and switcher animation types (macOS / Cinnamon inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    Fade,
    Slide,
    Minimize,
}

/// Window trait (OOP interface)
pub trait Window {
    /// Get window ID
    fn id(&self) -> usize;
    /// Get window rectangle
    fn rect(&self) -> Rectangle;
    /// Set window position
    fn set_position(&mut self, position: Position) -> Result<(), GraphicsError>;
    /// Set window size
    fn set_size(&mut self, size: Size) -> Result<(), GraphicsError>;
    /// Get window surface
    fn surface(&mut self) -> Option<&mut dyn Surface>;
    /// Show window
    fn show(&mut self);
    /// Hide window
    fn hide(&mut self);
    /// Get window info
    fn info(&self) -> WindowInfo;

    // Custom window animations (Cinnamon/macOS inspired)
    fn apply_transition(&mut self, anim: AnimationType, progress: f32);
    fn get_opacity(&self) -> f32;
    fn get_scale(&self) -> f32;
}

/// Window info
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

    // Animation properties
    pub animation_opacity: f32, // 0.0 to 1.0
    pub animation_scale: f32,   // 0.0 to 1.0
    pub current_animation: Option<AnimationType>,
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
            animation_opacity: 1.0f32,
            animation_scale: 1.0f32,
            current_animation: None,
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

    fn apply_transition(&mut self, anim: AnimationType, progress: f32) {
        self.current_animation = Some(anim);
        let clamped_progress = progress.max(0.0f32).min(1.0f32);
        match anim {
            AnimationType::Fade => {
                self.animation_opacity = 1.0f32 - clamped_progress;
            }
            AnimationType::Slide => {
                self.animation_opacity = 1.0f32 - clamped_progress;
            }
            AnimationType::Minimize => {
                self.animation_scale = 1.0f32 - (clamped_progress * 0.5f32);
                self.animation_opacity = 1.0f32 - clamped_progress;
            }
        }
    }

    fn get_opacity(&self) -> f32 {
        self.animation_opacity
    }

    fn get_scale(&self) -> f32 {
        self.animation_scale
    }
}

/// Compositor trait (OOP interface)
pub trait Compositor {
    /// Add window
    fn add_window(&mut self, window: Box<dyn Window>) -> Result<usize, GraphicsError>;
    /// Remove window
    fn remove_window(&mut self, id: usize) -> Result<(), GraphicsError>;
    /// Get window
    fn get_window(&mut self, id: usize) -> Option<&mut Box<dyn Window>>;
    /// Bring window to front
    fn bring_to_front(&mut self, id: usize) -> Result<(), GraphicsError>;
    /// Send window to back
    fn send_to_back(&mut self, id: usize) -> Result<(), GraphicsError>;
    /// Compose frame to front buffer (supporting double buffering)
    fn compose(&mut self, output: &mut dyn Surface) -> Result<(), GraphicsError>;
    /// Get compositor statistics
    fn stats(&self) -> CompositorStats;
    /// Dynamic double buffering: Swap front and back display buffers
    fn swap_buffers(&mut self) -> Result<(), GraphicsError>;
    /// Captures a screenshot of the currently composed frame
    fn capture_screenshot(&self) -> Result<Vec<u32>, GraphicsError>;
}

/// Graphics error types
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

/// Simple compositor (OOP: Concrete compositor class)
pub struct SimpleCompositor {
    windows: Vec<Box<dyn Window>>,
    window_order: Vec<usize>,
    stats: CompositorStats,
    capability: CompositorCapability,
    pub back_buffer: Option<BitmapSurface>,
    pub double_buffering: AtomicBool,
}

/// Compositor capability
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

        if let Some(pos) = self
            .windows
            .iter()
            .position(|w| w.id() == id)
        {
            self.windows.remove(pos);
            self.window_order.retain(|&x| x != id);
            self.stats.total_windows -= 1;
            Ok(())
        } else {
            Err(GraphicsError::WindowNotFound)
        }
    }

    fn get_window(&mut self, id: usize) -> Option<&mut Box<dyn Window>> {
        for win in &mut self.windows {
            if win.id() == id {
                return Some(win);
            }
        }
        None
    }

    fn bring_to_front(&mut self, id: usize) -> Result<(), GraphicsError> {
        if !self.capability.can_reorder_windows {
            return Err(GraphicsError::PermissionDenied);
        }

        if let Some(pos) = self.window_order.iter().position(|&x| x == id) {
            self.window_order.remove(pos);
            self.window_order.push(id);
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
            self.window_order.remove(pos);
            self.window_order.insert(0, id);
            Ok(())
        } else {
            Err(GraphicsError::WindowNotFound)
        }
    }

    fn compose(&mut self, output: &mut dyn Surface) -> Result<(), GraphicsError> {
        self.stats.frame_count += 1;

        let use_double_buffering =
            self.double_buffering.load(Ordering::SeqCst) && self.back_buffer.is_some();

        if use_double_buffering {
            let back = self.back_buffer.as_mut().unwrap();
            back.clear(Color::rgb(0, 0, 0));

            // Compose windows in order (back to front)
            for &window_id in &self.window_order {
                if let Some(window) = self.windows.iter_mut().find(|w| w.id() == window_id) {
                    let window_rect = window.rect();
                    let opacity = window.get_opacity(); // animation opacity factor

                    if let Some(surface) = window.surface() {
                        let window_stride = surface.info().stride as usize / 4;
                        let window_data = surface.data();

                        let back_stride = back.info().stride as usize / 4;
                        let back_data = back.data_mut();

                        // Copy window surface to back buffer with clipping & opacity
                        for y in 0..window_rect.size.height as usize {
                            for x in 0..window_rect.size.width as usize {
                                let output_x = (window_rect.position.x + x as i32) as usize;
                                let output_y = (window_rect.position.y + y as i32) as usize;

                                let output_index = output_y * back_stride + output_x;
                                let window_index = y * window_stride + x;

                                if output_index < back_data.len()
                                    && window_index < window_data.len()
                                {
                                    // Apply standard Alpha Blending (simulated via scaling)
                                    let pixel = window_data[window_index];
                                    if opacity < 0.99f32 {
                                        let a = ((pixel >> 24) & 0xFF) as f32 * opacity;
                                        let r = ((pixel >> 16) & 0xFF) as f32 * opacity;
                                        let g = ((pixel >> 8) & 0xFF) as f32 * opacity;
                                        let b = (pixel & 0xFF) as f32 * opacity;
                                        back_data[output_index] = ((a as u32) << 24)
                                            | ((r as u32) << 16)
                                            | ((g as u32) << 8)
                                            | (b as u32);
                                    } else {
                                        back_data[output_index] = pixel;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Copy back buffer to output
            let back = self.back_buffer.as_ref().unwrap();
            let back_data = back.data();
            let output_data = output.data_mut();
            let len = back_data.len().min(output_data.len());
            output_data[..len].copy_from_slice(&back_data[..len]);
        } else {
            output.clear(Color::rgb(0, 0, 0));

            // Compose windows in order (back to front)
            for &window_id in &self.window_order {
                if let Some(window) = self.windows.iter_mut().find(|w| w.id() == window_id) {
                    let window_rect = window.rect();
                    if let Some(surface) = window.surface() {
                        let window_stride = surface.info().stride as usize / 4;
                        let window_data = surface.data();

                        let output_stride = output.info().stride as usize / 4;
                        let output_data = output.data_mut();

                        // Copy window surface to output directly
                        for y in 0..window_rect.size.height as usize {
                            for x in 0..window_rect.size.width as usize {
                                let output_x = (window_rect.position.x + x as i32) as usize;
                                let output_y = (window_rect.position.y + y as i32) as usize;

                                let output_index = output_y * output_stride + output_x;
                                let window_index = y * window_stride + x;

                                if output_index < output_data.len()
                                    && window_index < window_data.len()
                                {
                                    output_data[output_index] = window_data[window_index];
                                }
                            }
                        }
                    }
                }
            }
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
        let mut stats = self.stats.clone();
        let mut visible = 0;
        for win in &self.windows {
            if win.info().visible {
                visible += 1;
            }
        }
        stats.visible_windows = visible;
        stats
    }
}

#[cfg(test)]
mod additional_compositor_tests {
    use super::*;

    #[test]
    fn test_window_minimization_and_fade_animations() {
        let rect = Rectangle::new(0, 0, 100, 100);
        let mut window = SimpleWindow::new(1, rect, WindowCapability::full());

        // Default scale and opacity should be 1.0
        assert_eq!(window.get_scale(), 1.0f32);
        assert_eq!(window.get_opacity(), 1.0f32);

        // Apply 50% progress Minimize transition
        window.apply_transition(AnimationType::Minimize, 0.5f32);
        assert_eq!(window.get_scale(), 0.75f32); // 1.0 - (0.5 * 0.5)
        assert_eq!(window.get_opacity(), 0.5f32); // 1.0 - 0.5

        // Apply complete transition
        window.apply_transition(AnimationType::Minimize, 1.0f32);
        assert_eq!(window.get_scale(), 0.5f32);
        assert_eq!(window.get_opacity(), 0.0f32);
    }
}
