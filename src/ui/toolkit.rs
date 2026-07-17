#![no_std]
#![no_main]

/// OOP-based Native UI Toolkit for SigmaOS
/// Implements UI toolkit using OOP principles with traits and structs
/// No dependency on external UI frameworks
/// Based on Roadmap Item 44: Native toolkit

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Widget ID
pub type WidgetID = usize;

/// Widget type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WidgetType {
    Button = 0,
    Label = 1,
    TextBox = 2,
    Checkbox = 3,
    ComboBox = 4,
    Slider = 5,
    Panel = 6,
}

/// Widget state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WidgetState {
    Normal = 0,
    Hovered = 1,
    Pressed = 2,
    Disabled = 3,
    Hidden = 4,
}

/// Widget trait (OOP interface)
pub trait Widget {
    /// Get widget ID
    fn id(&self) -> WidgetID;
    /// Get widget type
    fn widget_type(&self) -> WidgetType;
    /// Get widget label
    fn label(&self) -> &[u8];
    /// Set widget label
    fn set_label(&mut self, label: &[u8]);
    /// Get widget state
    fn state(&self) -> WidgetState;
    /// Set widget state
    fn set_state(&mut self, state: WidgetState);
    /// Render widget
    fn render(&self) -> Result<(), UIError>;
    /// Get widget info
    fn info(&self) -> WidgetInfo;
}

/// UI error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum UIError {
    Success = 0,
    RenderFailed = 1,
    InvalidState = 2,
    PermissionDenied = 3,
}

/// Widget info
#[repr(C)]
pub struct WidgetInfo {
    pub id: WidgetID,
    pub widget_type: WidgetType,
    pub label: [u8; 128],
    pub state: WidgetState,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub capability: WidgetCapability,
}

impl WidgetInfo {
    pub fn new(id: WidgetID, widget_type: WidgetType) -> Self {
        WidgetInfo {
            id,
            widget_type,
            label: [0; 128],
            state: WidgetState::Normal,
            x: 0,
            y: 0,
            width: 100,
            height: 30,
            capability: WidgetCapability::new(),
        }
    }
}

/// Widget capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WidgetCapability {
    pub can_interact: bool,
    pub can_modify: bool,
    pub can_hide: bool,
}

impl WidgetCapability {
    pub fn new() -> Self {
        WidgetCapability {
            can_interact: false,
            can_modify: false,
            can_hide: false,
        }
    }

    pub fn full() -> Self {
        WidgetCapability {
            can_interact: true,
            can_modify: true,
            can_hide: true,
        }
    }
}

/// Simple widget (OOP: Concrete widget class)
#[repr(C)]
pub struct SimpleWidget {
    pub id: WidgetID,
    pub widget_type: WidgetType,
    pub label: [u8; 128],
    pub state: AtomicUsize, // WidgetState as usize
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub capability: WidgetCapability,
}

impl SimpleWidget {
    pub fn new(id: WidgetID, widget_type: WidgetType, label: &[u8], capability: WidgetCapability) -> Self {
        let mut label_array = [0u8; 128];
        let label_len = label.len().min(127);

        unsafe {
            core::ptr::copy_nonoverlapping(label.as_ptr(), label_array.as_mut_ptr(), label_len);
        }

        SimpleWidget {
            id,
            widget_type,
            label: label_array,
            state: AtomicUsize::new(WidgetState::Normal as usize),
            x: 0,
            y: 0,
            width: 100,
            height: 30,
            capability,
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_state(&self) -> WidgetState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }
}

impl Widget for SimpleWidget {
    fn id(&self) -> WidgetID {
        self.id
    }

    fn widget_type(&self) -> WidgetType {
        self.widget_type
    }

    fn label(&self) -> &[u8] {
        let len = self.label.iter().position(|&b| b == 0).unwrap_or(128);
        &self.label[..len]
    }

    fn set_label(&mut self, label: &[u8]) {
        let len = label.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(label.as_ptr(), self.label.as_mut_ptr(), len);
        }
    }

    fn state(&self) -> WidgetState {
        self.get_state()
    }

    fn set_state(&mut self, state: WidgetState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    fn render(&self) -> Result<(), UIError> {
        if self.get_state() == WidgetState::Hidden {
            return Ok(());
        }
        // In a real implementation, this would render to a surface
        Ok(())
    }

    fn info(&self) -> WidgetInfo {
        WidgetInfo {
            id: self.id,
            widget_type: self.widget_type,
            label: self.label,
            state: self.get_state(),
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            capability: self.capability,
        }
    }
}

/// UI layout trait (OOP interface)
pub trait UILayout {
    /// Add widget
    fn add_widget(&mut self, widget: Box<dyn Widget>) -> Result<WidgetID, UIError>;
    /// Remove widget
    fn remove_widget(&mut self, id: WidgetID) -> Result<(), UIError>;
    /// Get widget
    fn get_widget(&self, id: WidgetID) -> Option<&dyn Widget>;
    /// Render all widgets
    fn render(&self) -> Result<(), UIError>;
    /// Get layout statistics
    fn stats(&self) -> LayoutStats;
}

/// Layout statistics
#[repr(C)]
pub struct LayoutStats {
    pub total_widgets: usize,
    pub visible_widgets: usize,
    pub by_type: [usize; 7],
}

impl LayoutStats {
    pub fn new() -> Self {
        LayoutStats {
            total_widgets: 0,
            visible_widgets: 0,
            by_type: [0; 7],
        }
    }
}

/// Simple UI layout (OOP: Concrete layout class)
pub struct SimpleUILayout {
    widgets: Vec<Option<Box<dyn Widget>>>,
    next_id: AtomicUsize,
    stats: LayoutStats,
    capability: LayoutCapability,
}

/// Layout capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LayoutCapability {
    pub can_add: bool,
    pub can_remove: bool,
    pub can_render: bool,
}

impl LayoutCapability {
    pub fn new() -> Self {
        LayoutCapability {
            can_add: false,
            can_remove: false,
            can_render: false,
        }
    }

    pub fn full() -> Self {
        LayoutCapability {
            can_add: true,
            can_remove: true,
            can_render: true,
        }
    }
}

impl SimpleUILayout {
    pub fn new(capability: LayoutCapability) -> Self {
        SimpleUILayout {
            widgets: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: LayoutStats::new(),
            capability,
        }
    }
}

impl UILayout for SimpleUILayout {
    fn add_widget(&mut self, widget: Box<dyn Widget>) -> Result<WidgetID, UIError> {
        if !self.capability.can_add {
            return Err(UIError::PermissionDenied);
        }

        let id = widget.id();
        let widget_type = widget.widget_type();
        self.widgets.push(Some(widget));
        self.stats.total_widgets += 1;
        self.stats.visible_widgets += 1;
        self.stats.by_type[widget_type as usize] += 1;
        Ok(id)
    }

    fn remove_widget(&mut self, id: WidgetID) -> Result<(), UIError> {
        if !self.capability.can_remove {
            return Err(UIError::PermissionDenied);
        }

        let mut index = None;
        let mut widget_type = WidgetType::Button;

        for (i, widget_option) in self.widgets.iter().enumerate() {
            if let Some(ref widget) = *widget_option {
                if widget.id() == id {
                    index = Some(i);
                    widget_type = widget.widget_type();
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.widgets[i] = None;
            self.stats.total_widgets -= 1;
            self.stats.visible_widgets -= 1;
            self.stats.by_type[widget_type as usize] -= 1;
            Ok(())
        } else {
            Err(UIError::InvalidState)
        }
    }

    fn get_widget(&self, id: WidgetID) -> Option<&dyn Widget> {
        for widget_option in &self.widgets {
            if let Some(ref widget) = *widget_option {
                if widget.id() == id {
                    return Some(widget.as_ref());
                }
            }
        }
        None
    }

    fn render(&self) -> Result<(), UIError> {
        if !self.capability.can_render {
            return Err(UIError::PermissionDenied);
        }

        for widget_option in &self.widgets {
            if let Some(ref widget) = *widget_option {
                let _ = widget.render();
            }
        }
        Ok(())
    }

    fn stats(&self) -> LayoutStats {
        self.stats
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

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
