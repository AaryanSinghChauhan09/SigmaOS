#![no_std]
#![no_main]

/// OOP-based Graphics Compositor for SigmaOS
/// Implements graphics composition using OOP principles with traits and structs
/// No dependency on external graphics frameworks

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Position
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
        point.x >= self.position.x &&
        point.x < self.position.x + self.size.width as i32 &&
        point.y >= self.position.y &&
        point.y < self.position.y + self.size.height as i32
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.position.x < other.position.x + other.size.width as i32 &&
        self.position.x + self.size.width as i32 > other.position.x &&
        self.position.y < other.position.y + other.size.height as i32 &&
        self.position.y + self.size.height as i32 > other.position.y
    }
}

/// Color (RGBA)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
        ((self.a as u32) << 24) |
        ((self.r as u32) << 16) |
        ((self.g as u32) << 8) |
        (self.b as u32)
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
#[repr(C)]
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
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    RGB24 = 0,
    RGBA32 = 1,
    BGR24 = 2,
    BGRA32 = 3,
}

/// Surface capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

/// Bitmap surface (OOP: Concrete surface class)
#[repr(C)]
pub struct BitmapSurface {
    pub id: usize,
    pub data: Option<NonNull<u32>>,
    pub size: Size,
    pub stride: u32,
    pub capability: SurfaceCapability,
    pub locked: AtomicBool,
}

impl BitmapSurface {
    pub fn new(id: usize, width: u32, height: u32, capability: SurfaceCapability) -> Self {
        let data = unsafe {
            let size = (width * height) as usize;
            let ptr = alloc(size * mem::size_of::<u32>()) as *mut u32;
            if ptr.is_null() {
                None
            } else {
                Some(NonNull::new_unchecked(ptr))
            }
        };

        BitmapSurface {
            id,
            data,
            size: Size::new(width, height),
            stride: width * 4,
            capability,
            locked: AtomicBool::new(false),
        }
    }

    pub unsafe fn lock(&mut self) -> Result<(), GraphicsError> {
        if !self.capability.can_lock {
            return Err(GraphicsError::PermissionDenied);
        }

        if self.locked.load(Ordering::SeqCst) {
            return Err(GraphicsError::AlreadyLocked);
        }

        self.locked.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub unsafe fn unlock(&mut self) {
        self.locked.store(false, Ordering::SeqCst);
    }
}

impl Surface for BitmapSurface {
    fn size(&self) -> Size {
        self.size
    }

    fn data(&self) -> &[u32] {
        unsafe {
            if let Some(data) = self.data {
                let size = (self.size.width * self.size.height) as usize;
                core::slice::from_raw_parts(data.as_ptr(), size)
            } else {
                &[]
            }
        }
    }

    fn data_mut(&mut self) -> &mut [u32] {
        unsafe {
            if let Some(data) = self.data {
                let size = (self.size.width * self.size.height) as usize;
                core::slice::from_raw_parts_mut(data.as_ptr(), size)
            } else {
                &mut []
            }
        }
    }

    fn clear(&mut self, color: Color) {
        let color_value = color.to_u32();
        for pixel in self.data_mut() {
            *pixel = color_value;
        }
    }

    fn fill_rect(&mut self, rect: Rectangle, color: Color) {
        let color_value = color.to_u32();
        let data = self.data_mut();
        let stride = self.stride as usize / 4;

        for y in rect.position.y.max(0) as usize..(rect.position.y + rect.size.height as i32).min(self.size.height as i32) as usize {
            for x in rect.position.x.max(0) as usize..(rect.position.x + rect.size.width as i32).min(self.size.width as i32) as usize {
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

impl Drop for BitmapSurface {
    fn drop(&mut self) {
        unsafe {
            if let Some(data) = self.data {
                free(data.as_ptr() as *mut u8);
            }
        }
    }
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
}

/// Window info
#[repr(C)]
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

/// Simple window (OOP: Concrete window class)
#[repr(C)]
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
        let surface = BitmapSurface::new(id, rect.size.width, rect.size.height, SurfaceCapability::full());

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
        // In a real implementation, this would recreate the surface
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
    /// Compose frame
    fn compose(&mut self, output: &mut dyn Surface) -> Result<(), GraphicsError>;
    /// Get compositor statistics
    fn stats(&self) -> CompositorStats;
}

/// Graphics error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

/// Simple compositor (OOP: Concrete compositor class)
pub struct SimpleCompositor {
    windows: Vec<Option<Box<dyn Window>>>,
    window_order: Vec<usize>,
    stats: CompositorStats,
    capability: CompositorCapability,
}

/// Compositor capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

impl SimpleCompositor {
    pub fn new(capability: CompositorCapability) -> Self {
        SimpleCompositor {
            windows: Vec::new(),
            window_order: Vec::new(),
            stats: CompositorStats::new(),
            capability,
        }
    }
}

impl Compositor for SimpleCompositor {
    fn add_window(&mut self, window: Box<dyn Window>) -> Result<usize, GraphicsError> {
        if !self.capability.can_add_windows {
            return Err(GraphicsError::PermissionDenied);
        }

        let id = window.id();
        self.windows.push(Some(window));
        self.window_order.push(id);
        self.stats.total_windows += 1;
        Ok(id)
    }

    fn remove_window(&mut self, id: usize) -> Result<(), GraphicsError> {
        if !self.capability.can_remove_windows {
            return Err(GraphicsError::PermissionDenied);
        }

        let mut index = None;
        for (i, window_option) in self.windows.iter().enumerate() {
            if let Some(ref window) = *window_option {
                if window.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.windows[i] = None;
            self.window_order.retain(|&x| x != id);
            self.stats.total_windows -= 1;
            Ok(())
        } else {
            Err(GraphicsError::WindowNotFound)
        }
    }

    fn get_window(&mut self, id: usize) -> Option<&mut Box<dyn Window>> {
        for window_option in &mut self.windows {
            if let Some(ref mut window) = *window_option {
                if window.id() == id {
                    return Some(window);
                }
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

        // Clear output
        output.clear(Color::rgb(0, 0, 0));

        // Compose windows in order (back to front)
        for &window_id in &self.window_order {
            if let Some(ref mut window) = self.windows[window_id] {
                if let Some(surface) = window.surface() {
                    let window_rect = window.rect();
                    let output_data = output.data_mut();
                    let window_data = surface.data();
                    
                    let output_stride = output.info().stride as usize / 4;
                    let window_stride = surface.info().stride as usize / 4;

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
        }

        Ok(())
    }

    fn stats(&self) -> CompositorStats {
        let mut stats = self.stats.clone();
        stats.visible_windows = 0;

        for window_option in &self.windows {
            if let Some(ref window) = *window_option {
                if window.info().visible {
                    stats.visible_windows += 1;
                }
            }
        }

        stats
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            core::ptr::copy(self.data.add(index + 1), self.data.add(index), self.len - index - 1);
            self.len -= 1;
            item
        }
    }

    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut write = 0;
        for read in 0..self.len {
            unsafe {
                let item = &*self.data.add(read);
                if f(item) {
                    if write != read {
                        let item_copy = core::ptr::read(self.data.add(read));
                        core::ptr::write(self.data.add(write), item_copy);
                    }
                    write += 1;
                }
            }
        }
        self.len = write;
    }

    fn insert(&mut self, index: usize, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if index < self.len {
                core::ptr::copy(self.data.add(index), self.data.add(index + 1), self.len - index);
            }

            core::ptr::write(self.data.add(index), item);
            self.len += 1;
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            data: self.data,
            len: self.len,
            index: 0,
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            data: self.data,
            len: self.len,
            index: 0,
        }
    }

    fn position<F>(&self, mut f: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool,
    {
        for i in 0..self.len {
            unsafe {
                let item = &*self.data.add(i);
                if f(item) {
                    return Some(i);
                }
            }
        }
        None
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

struct Iter<T> {
    data: *const T,
    len: usize,
    index: usize,
}

impl<'a, T> Iterator for Iter<T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            unsafe {
                let item = &*self.data.add(self.index);
                self.index += 1;
                Some(item)
            }
        } else {
            None
        }
    }
}

struct IterMut<T> {
    data: *mut T,
    len: usize,
    index: usize,
}

impl<'a, T> Iterator for IterMut<T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            unsafe {
                let item = &mut *self.data.add(self.index);
                self.index += 1;
                Some(item)
            }
        } else {
            None
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
