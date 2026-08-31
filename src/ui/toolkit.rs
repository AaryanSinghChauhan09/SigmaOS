extern crate alloc;
// OOP-based Native UI Toolkit for SigmaOS
// Implements UI toolkit using OOP principles with traits and structs.


use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Widget ID
pub type WidgetID = usize;

/// Widget type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidgetCapability {
    pub can_interact: bool,
    pub can_modify: bool,
    pub can_hide: bool,
}

impl WidgetCapability {
    pub const fn new() -> Self {
        WidgetCapability {
            can_interact: false,
            can_modify: false,
            can_hide: false,
        }
    }

    pub const fn full() -> Self {
        WidgetCapability {
            can_interact: true,
            can_modify: true,
            can_hide: true,
        }
    }
}

impl Default for WidgetCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple widget (OOP: Concrete widget class)
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
    pub fn new(
        id: WidgetID,
        widget_type: WidgetType,
        label: &[u8],
        capability: WidgetCapability,
    ) -> Self {
        let mut label_array = [0u8; 128];
        let label_len = label.len().min(127);
        label_array[..label_len].copy_from_slice(&label[..label_len]);

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
        match self.state.load(Ordering::SeqCst) {
            0 => WidgetState::Normal,
            1 => WidgetState::Hovered,
            2 => WidgetState::Pressed,
            3 => WidgetState::Disabled,
            _ => WidgetState::Hidden,
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
        self.label = [0; 128];
        self.label[..len].copy_from_slice(&label[..len]);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutStats {
    pub total_widgets: usize,
    pub visible_widgets: usize,
    pub by_type: [usize; 7],
}

impl LayoutStats {
    pub const fn new() -> Self {
        LayoutStats {
            total_widgets: 0,
            visible_widgets: 0,
            by_type: [0; 7],
        }
    }
}

impl Default for LayoutStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple UI layout (OOP: Concrete layout class)
pub struct SimpleUILayout {
    pub widgets: Vec<Option<Box<dyn Widget>>>,
    pub next_id: AtomicUsize,
    pub stats: LayoutStats,
    pub capability: LayoutCapability,
}

/// Layout capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutCapability {
    pub can_add: bool,
    pub can_remove: bool,
    pub can_render: bool,
}

impl LayoutCapability {
    pub const fn new() -> Self {
        LayoutCapability {
            can_add: false,
            can_remove: false,
            can_render: false,
        }
    }

    pub const fn full() -> Self {
        LayoutCapability {
            can_add: true,
            can_remove: true,
            can_render: true,
        }
    }
}

impl Default for LayoutCapability {
    fn default() -> Self {
        Self::new()
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

use alloc::string::{String, ToString};
use alloc::format;

// --- GTK 4 / Libadwaita Abstractions ---
#[derive(Debug, Clone)]
pub struct GtkCssProvider {
    pub css_data: String,
    pub priority: u32, // GTK_STYLE_PROVIDER_PRIORITY_APPLICATION = 600
}

impl GtkCssProvider {
    pub fn new(css: &str) -> Self {
        Self {
            css_data: css.to_string(),
            priority: 600,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdwHeaderBar {
    pub title: String,
    pub show_start_title_buttons: bool,
    pub show_end_title_buttons: bool,
}

#[derive(Debug, Clone)]
pub struct AdwClamp {
    pub maximum_width: u32,
    pub tightening_threshold: u32,
}

#[derive(Debug, Clone)]
pub struct GtkBoxContainer {
    pub orientation_vertical: bool,
    pub spacing_px: u32,
    pub children_count: usize,
}

pub struct SovereignGtkToolkitEngine {
    pub css_providers: Vec<GtkCssProvider>,
    pub header_bars: Vec<AdwHeaderBar>,
    pub clamps: Vec<AdwClamp>,
    pub boxes: Vec<GtkBoxContainer>,
    pub is_libadwaita_active: bool,
}

impl SovereignGtkToolkitEngine {
    pub fn new() -> Self {
        Self {
            css_providers: Vec::new(),
            header_bars: Vec::new(),
            clamps: Vec::new(),
            boxes: Vec::new(),
            is_libadwaita_active: true,
        }
    }

    pub fn load_css_theme(&mut self, css_data: &str) {
        self.css_providers.push(GtkCssProvider::new(css_data));
    }

    pub fn add_adw_header_bar(&mut self, title: &str) -> usize {
        let bar = AdwHeaderBar {
            title: title.to_string(),
            show_start_title_buttons: true,
            show_end_title_buttons: true,
        };
        self.header_bars.push(bar);
        self.header_bars.len() - 1
    }

    pub fn add_adw_clamp(&mut self, max_width: u32) -> usize {
        let clamp = AdwClamp {
            maximum_width: max_width,
            tightening_threshold: (max_width as f32 * 0.8) as u32,
        };
        self.clamps.push(clamp);
        self.clamps.len() - 1
    }

    pub fn add_gtk_box(&mut self, vertical: bool, spacing: u32) -> usize {
        let box_container = GtkBoxContainer {
            orientation_vertical: vertical,
            spacing_px: spacing,
            children_count: 0,
        };
        self.boxes.push(box_container);
        self.boxes.len() - 1
    }
}

impl Default for SovereignGtkToolkitEngine {
    fn default() -> Self {
        Self::new()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_toolkit_rendering_and_layout() {
        let mut layout = SimpleUILayout::new(LayoutCapability::full());
        let button = SimpleWidget::new(
            101,
            WidgetType::Button,
            b"Confirm",
            WidgetCapability::full(),
        );

        layout.add_widget(Box::new(button)).unwrap();
        assert_eq!(layout.stats().total_widgets, 1);

        let widget_ref = layout.get_widget(101).unwrap();
        assert_eq!(widget_ref.label(), b"Confirm");
        assert_eq!(widget_ref.state(), WidgetState::Normal);
    }

    #[test]
    fn test_gtk_css_provider_and_libadwaita() {
        let mut gtk = SovereignGtkToolkitEngine::new();
        gtk.load_css_theme("window { background-color: #1e1e1e; }");
        assert_eq!(gtk.css_providers.len(), 1);
        assert_eq!(gtk.css_providers[0].priority, 600);

        let bar_idx = gtk.add_adw_header_bar("SigmaOS Sovereign Control Center");
        assert_eq!(bar_idx, 0);
        assert_eq!(gtk.header_bars[0].title, "SigmaOS Sovereign Control Center");

        let clamp_idx = gtk.add_adw_clamp(800);
        assert_eq!(clamp_idx, 0);
        assert_eq!(gtk.clamps[0].maximum_width, 800);

        let box_idx = gtk.add_gtk_box(true, 12);
        assert_eq!(box_idx, 0);
        assert!(gtk.boxes[0].orientation_vertical);
        assert_eq!(gtk.boxes[0].spacing_px, 12);
    }
}
