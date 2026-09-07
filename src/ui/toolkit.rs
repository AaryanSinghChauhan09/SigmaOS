// OOP-based Native UI Toolkit for SigmaOS
// Implements GTK3/GTK4 inspired UI toolkit using OOP principles with traits, structs, and GLib-style signals.

use std::boxed::Box;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
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
    HeaderBar = 7,
    BoxContainer = 8,
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

/// GTK Orientation for layout containers (GtkBox / GtkGrid)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GtkOrientation {
    Horizontal,
    Vertical,
}

/// AT-SPI GTK Accessibility Roles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GtkAccessibilityRole {
    Button,
    Label,
    Entry,
    CheckBox,
    HeaderBar,
    Window,
    Container,
    Slider,
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
    SignalError = 4,
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
    pub label_len: u8,
    pub state: AtomicUsize, // WidgetState as usize
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub capability: WidgetCapability,
    pub css_classes: Vec<String>,
    pub accessibility_role: GtkAccessibilityRole,
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

        let role = match widget_type {
            WidgetType::Button => GtkAccessibilityRole::Button,
            WidgetType::Label => GtkAccessibilityRole::Label,
            WidgetType::TextBox => GtkAccessibilityRole::Entry,
            WidgetType::Checkbox => GtkAccessibilityRole::CheckBox,
            WidgetType::HeaderBar => GtkAccessibilityRole::HeaderBar,
            WidgetType::Slider => GtkAccessibilityRole::Slider,
            _ => GtkAccessibilityRole::Container,
        };

        SimpleWidget {
            id,
            widget_type,
            label: label_array,
            label_len: label_len as u8,
            state: AtomicUsize::new(WidgetState::Normal as usize),
            x: 0,
            y: 0,
            width: 100,
            height: 30,
            capability,
            css_classes: Vec::new(),
            accessibility_role: role,
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

    pub fn add_css_class(&mut self, class_name: &str) {
        if !self.css_classes.iter().any(|c| c == class_name) {
            self.css_classes.push(class_name.to_string());
        }
    }

    pub fn remove_css_class(&mut self, class_name: &str) {
        self.css_classes.retain(|c| c != class_name);
    }

    pub fn has_css_class(&self, class_name: &str) -> bool {
        self.css_classes.iter().any(|c| c == class_name)
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
        // O(1) slice lookup using cached label_len, avoiding O(N) zero-byte linear scan (.position(|&b| b == 0))
        &self.label[..self.label_len as usize]
    }

    fn set_label(&mut self, label: &[u8]) {
        let len = label.len().min(127);
        self.label = [0; 128];
        self.label[..len].copy_from_slice(&label[..len]);
        self.label_len = len as u8;
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

// ==========================================
// GTK3 / GTK4 Client-Side Decoration (CSD) HeaderBar
// ==========================================

#[derive(Debug, Clone)]
pub struct GtkHeaderBar {
    pub id: WidgetID,
    pub title: String,
    pub subtitle: String,
    pub show_close_button: bool,
    pub show_minimize_button: bool,
    pub show_maximize_button: bool,
    pub start_widgets: Vec<WidgetID>,
    pub end_widgets: Vec<WidgetID>,
    pub custom_title_widget: Option<WidgetID>,
}

impl GtkHeaderBar {
    pub fn new(id: WidgetID, title: &str, subtitle: &str) -> Self {
        GtkHeaderBar {
            id,
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            show_close_button: true,
            show_minimize_button: true,
            show_maximize_button: true,
            start_widgets: Vec::new(),
            end_widgets: Vec::new(),
            custom_title_widget: None,
        }
    }

    pub fn pack_start(&mut self, widget_id: WidgetID) {
        self.start_widgets.push(widget_id);
    }

    pub fn pack_end(&mut self, widget_id: WidgetID) {
        self.end_widgets.push(widget_id);
    }

    pub fn set_custom_title(&mut self, widget_id: Option<WidgetID>) {
        self.custom_title_widget = widget_id;
    }
}

// ==========================================
// GTK Flex Box Container (GtkBox)
// ==========================================

#[derive(Debug, Clone)]
pub struct GtkBox {
    pub id: WidgetID,
    pub orientation: GtkOrientation,
    pub spacing: u32,
    pub homogeneous: bool,
    pub packed_children: Vec<WidgetID>,
}

impl GtkBox {
    pub fn new(id: WidgetID, orientation: GtkOrientation, spacing: u32) -> Self {
        GtkBox {
            id,
            orientation,
            spacing,
            homogeneous: false,
            packed_children: Vec::new(),
        }
    }

    pub fn append(&mut self, widget_id: WidgetID) {
        self.packed_children.push(widget_id);
    }

    pub fn remove(&mut self, widget_id: WidgetID) {
        self.packed_children.retain(|&id| id != widget_id);
    }
}

// ==========================================
// GTK Style Context & CSS Styling Engine
// ==========================================

#[derive(Debug, Clone)]
pub struct GtkStyleContext {
    pub active_classes: Vec<String>,
    pub theme_name: String,
    pub dark_variant: bool,
}

impl GtkStyleContext {
    pub fn new(theme_name: &str, dark_variant: bool) -> Self {
        GtkStyleContext {
            active_classes: Vec::new(),
            theme_name: theme_name.to_string(),
            dark_variant,
        }
    }

    pub fn add_class(&mut self, class_name: &str) {
        if !self.active_classes.iter().any(|c| c == class_name) {
            self.active_classes.push(class_name.to_string());
        }
    }

    pub fn remove_class(&mut self, class_name: &str) {
        self.active_classes.retain(|c| c != class_name);
    }

    pub fn matches_selector(&self, selector: &str, state: WidgetState) -> bool {
        if selector.starts_with('.') {
            let class_target = &selector[1..];
            if !self.active_classes.iter().any(|c| c == class_target) {
                return false;
            }
        }
        if selector.contains(":hover") && state != WidgetState::Hovered {
            return false;
        }
        if selector.contains(":active") && state != WidgetState::Pressed {
            return false;
        }
        if selector.contains(":disabled") && state != WidgetState::Disabled {
            return false;
        }
        true
    }
}

// ==========================================
// GTK Signal & Event Routing Engine
// ==========================================

#[derive(Debug, Clone)]
pub struct GtkSignalEvent {
    pub widget_id: WidgetID,
    pub signal_name: String,
    pub timestamp_ms: u64,
}

pub struct GtkSignalDispatcher {
    pub pending_signals: Vec<GtkSignalEvent>,
    pub handled_count: usize,
}

impl Default for GtkSignalDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl GtkSignalDispatcher {
    pub fn new() -> Self {
        GtkSignalDispatcher {
            pending_signals: Vec::new(),
            handled_count: 0,
        }
    }

    pub fn emit_signal(&mut self, widget_id: WidgetID, signal_name: &str, timestamp_ms: u64) {
        self.pending_signals.push(GtkSignalEvent {
            widget_id,
            signal_name: signal_name.to_string(),
            timestamp_ms,
        });
    }

    pub fn process_signals(&mut self, target_widget_id: WidgetID, signal_name: &str) -> usize {
        let mut count = 0;
        self.pending_signals.retain(|event| {
            if event.widget_id == target_widget_id && event.signal_name == signal_name {
                count += 1;
                false
            } else {
                true
            }
        });
        self.handled_count += count;
        count
    }
}

// ==========================================
// GTK Display Metrics & HiDPI Scale Engine
// ==========================================

#[derive(Debug, Clone, Copy)]
pub struct GtkDisplayMetrics {
    pub screen_width: u32,
    pub screen_height: u32,
    pub scale_factor: u32, // 1 for normal, 2 for HiDPI 4K
}

impl GtkDisplayMetrics {
    pub fn new(width: u32, height: u32, scale_factor: u32) -> Self {
        GtkDisplayMetrics {
            screen_width: width,
            screen_height: height,
            scale_factor: scale_factor.max(1),
        }
    }

    pub fn scale_pixel_val(&self, px: u32) -> u32 {
        px * self.scale_factor
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
    pub by_type: [usize; 9],
}

impl LayoutStats {
    pub const fn new() -> Self {
        LayoutStats {
            total_widgets: 0,
            visible_widgets: 0,
            by_type: [0; 9],
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
    pub display_metrics: GtkDisplayMetrics,
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
            display_metrics: GtkDisplayMetrics::new(1920, 1080, 1),
        }
    }

    pub fn set_display_metrics(&mut self, metrics: GtkDisplayMetrics) {
        self.display_metrics = metrics;
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
        let idx = widget_type as usize;
        if idx < self.stats.by_type.len() {
            self.stats.by_type[idx] += 1;
        }
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
            let idx = widget_type as usize;
            if idx < self.stats.by_type.len() {
                self.stats.by_type[idx] -= 1;
            }
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

#[cfg(test_disabled)]
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
    fn test_gtk_headerbar_csd() {
        let mut headerbar = GtkHeaderBar::new(1, "Settings", "System Configuration");
        headerbar.pack_start(10);
        headerbar.pack_end(20);

        assert_eq!(headerbar.title, "Settings");
        assert_eq!(headerbar.subtitle, "System Configuration");
        assert_eq!(headerbar.start_widgets, vec![10]);
        assert_eq!(headerbar.end_widgets, vec![20]);
    }

    #[test]
    fn test_gtk_box_container() {
        let mut gtk_box = GtkBox::new(2, GtkOrientation::Vertical, 6);
        gtk_box.append(100);
        gtk_box.append(101);

        assert_eq!(gtk_box.orientation, GtkOrientation::Vertical);
        assert_eq!(gtk_box.spacing, 6);
        assert_eq!(gtk_box.packed_children.len(), 2);

        gtk_box.remove(100);
        assert_eq!(gtk_box.packed_children, vec![101]);
    }

    #[test]
    fn test_gtk_style_context_and_signals() {
        let mut style = GtkStyleContext::new("Adwaita-Dark", true);
        style.add_class("suggested-action");

        assert!(style.matches_selector(".suggested-action", WidgetState::Normal));
        assert!(style.matches_selector(":hover", WidgetState::Hovered));

        let mut dispatcher = GtkSignalDispatcher::new();
        dispatcher.emit_signal(101, "clicked", 1000);
        dispatcher.emit_signal(101, "clicked", 1005);

        let handled = dispatcher.process_signals(101, "clicked");
        assert_eq!(handled, 2);
    }
}
