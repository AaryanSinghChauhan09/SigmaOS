// src/ui/widget_api.rs
// Zero-dependency Widget API for SigmaOS
// Superior to Omarchy's GTK/Qt - native Rust, GPU-accelerated
//
// Features:
// - Declarative widget trees
// - Event-driven architecture
// - GPU rendering via Vulkan
// - Theme integration
// - Agent-powered interactions

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::fmt;

use crate::theming::{Color, Theme};

/// Widget ID for efficient lookups
pub type WidgetId = u64;

/// Widget tree node
pub struct Widget {
    pub id: WidgetId,
    pub kind: WidgetKind,
    pub layout: Layout,
    pub style: Style,
    pub state: WidgetState,
    pub children: Vec<Widget>,
    pub event_handlers: BTreeMap<EventType, EventHandler>,
}

/// Widget types
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetKind {
    Container,
    Text(String),
    Button(String),
    Input(String),
    Image(ImageData),
    Scroll,
    List,
    Grid,
    Stack,
    Custom(String),
}

/// Layout properties
#[derive(Debug, Clone)]
pub struct Layout {
    pub width: Dimension,
    pub height: Dimension,
    pub padding: Spacing,
    pub margin: Spacing,
    pub flex_direction: FlexDirection,
    pub justify_content: Justify,
    pub align_items: Align,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            width: Dimension::Auto,
            height: Dimension::Auto,
            padding: Spacing::zero(),
            margin: Spacing::zero(),
            flex_direction: FlexDirection::Row,
            justify_content: Justify::Start,
            align_items: Align::Start,
        }
    }
}

/// Dimension specification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
    Auto,
    Px(u32),
    Percent(f32),
    Fill,
}

/// Spacing (padding/margin)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spacing {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Spacing {
    pub const fn zero() -> Self {
        Self { top: 0, right: 0, bottom: 0, left: 0 }
    }
    
    pub const fn all(value: u32) -> Self {
        Self { top: value, right: value, bottom: value, left: value }
    }
    
    pub const fn symmetric(vertical: u32, horizontal: u32) -> Self {
        Self { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }
}

/// Flexbox direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

/// Justify content
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Justify {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Align items
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    Start,
    End,
    Center,
    Stretch,
}

/// Widget styling
#[derive(Debug, Clone)]
pub struct Style {
    pub background: Color,
    pub foreground: Color,
    pub border_color: Color,
    pub border_width: u32,
    pub border_radius: u32,
    pub font_size: u32,
    pub opacity: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: Color::rgba(0, 0, 0, 0),
            foreground: Color::rgb(255, 255, 255),
            border_color: Color::rgba(0, 0, 0, 0),
            border_width: 0,
            border_radius: 0,
            font_size: 14,
            opacity: 1.0,
        }
    }
}

/// Widget state
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WidgetState {
    pub visible: bool,
    pub enabled: bool,
    pub focused: bool,
    pub hovered: bool,
    pub pressed: bool,
}

impl Default for WidgetState {
    fn default() -> Self {
        Self {
            visible: true,
            enabled: true,
            focused: false,
            hovered: false,
            pressed: false,
        }
    }
}

/// Image data
#[derive(Debug, Clone, PartialEq)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

/// Event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventType {
    Click,
    DoubleClick,
    MouseDown,
    MouseUp,
    MouseMove,
    MouseEnter,
    MouseLeave,
    KeyDown,
    KeyUp,
    Focus,
    Blur,
    Input,
    Change,
    Submit,
    Scroll,
}

/// Event data
#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub target: WidgetId,
    pub position: Option<(i32, i32)>,
    pub key: Option<char>,
    pub modifiers: Modifiers,
}

/// Keyboard modifiers
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

/// Event handler function
pub type EventHandler = fn(&mut Widget, &Event);

/// Widget builder for ergonomic construction
pub struct WidgetBuilder {
    widget: Widget,
}

impl WidgetBuilder {
    pub fn new(kind: WidgetKind) -> Self {
        static mut NEXT_ID: WidgetId = 1;
        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };
        
        Self {
            widget: Widget {
                id,
                kind,
                layout: Layout::default(),
                style: Style::default(),
                state: WidgetState::default(),
                children: Vec::new(),
                event_handlers: BTreeMap::new(),
            },
        }
    }
    
    pub fn width(mut self, width: Dimension) -> Self {
        self.widget.layout.width = width;
        self
    }
    
    pub fn height(mut self, height: Dimension) -> Self {
        self.widget.layout.height = height;
        self
    }
    
    pub fn padding(mut self, padding: Spacing) -> Self {
        self.widget.layout.padding = padding;
        self
    }
    
    pub fn margin(mut self, margin: Spacing) -> Self {
        self.widget.layout.margin = margin;
        self
    }
    
    pub fn background(mut self, color: Color) -> Self {
        self.widget.style.background = color;
        self
    }
    
    pub fn foreground(mut self, color: Color) -> Self {
        self.widget.style.foreground = color;
        self
    }
    
    pub fn border(mut self, width: u32, color: Color) -> Self {
        self.widget.style.border_width = width;
        self.widget.style.border_color = color;
        self
    }
    
    pub fn border_radius(mut self, radius: u32) -> Self {
        self.widget.style.border_radius = radius;
        self
    }
    
    pub fn font_size(mut self, size: u32) -> Self {
        self.widget.style.font_size = size;
        self
    }
    
    pub fn on(mut self, event_type: EventType, handler: EventHandler) -> Self {
        self.widget.event_handlers.insert(event_type, handler);
        self
    }
    
    pub fn child(mut self, child: Widget) -> Self {
        self.widget.children.push(child);
        self
    }
    
    pub fn children(mut self, children: Vec<Widget>) -> Self {
        self.widget.children.extend(children);
        self
    }
    
    pub fn build(self) -> Widget {
        self.widget
    }
}

/// Widget API for building UIs
pub struct WidgetApi {
    root: Option<Widget>,
    theme: Theme,
    focused_widget: Option<WidgetId>,
}

impl WidgetApi {
    pub fn new(theme: Theme) -> Self {
        Self {
            root: None,
            theme,
            focused_widget: None,
        }
    }
    
    pub fn set_root(&mut self, widget: Widget) {
        self.root = Some(widget);
    }
    
    pub fn get_root(&self) -> Option<&Widget> {
        self.root.as_ref()
    }
    
    pub fn get_root_mut(&mut self) -> Option<&mut Widget> {
        self.root.as_mut()
    }
    
    pub fn find_widget(&self, id: WidgetId) -> Option<&Widget> {
        self.root.as_ref().and_then(|root| Self::find_widget_recursive(root, id))
    }
    
    fn find_widget_recursive(widget: &Widget, id: WidgetId) -> Option<&Widget> {
        if widget.id == id {
            return Some(widget);
        }
        
        for child in &widget.children {
            if let Some(found) = Self::find_widget_recursive(child, id) {
                return Some(found);
            }
        }
        
        None
    }
    
    pub fn find_widget_mut(&mut self, id: WidgetId) -> Option<&mut Widget> {
        self.root.as_mut().and_then(|root| Self::find_widget_mut_recursive(root, id))
    }
    
    fn find_widget_mut_recursive(widget: &mut Widget, id: WidgetId) -> Option<&mut Widget> {
        if widget.id == id {
            return Some(widget);
        }
        
        for child in &mut widget.children {
            if let Some(found) = Self::find_widget_mut_recursive(child, id) {
                return Some(found);
            }
        }
        
        None
    }
    
    pub fn dispatch_event(&mut self, event: Event) {
        if let Some(widget) = self.find_widget_mut(event.target) {
            if let Some(handler) = widget.event_handlers.get(&event.event_type) {
                handler(widget, &event);
            }
        }
    }
    
    pub fn set_focus(&mut self, id: WidgetId) {
        // Clear previous focus
        if let Some(old_id) = self.focused_widget {
            if let Some(widget) = self.find_widget_mut(old_id) {
                widget.state.focused = false;
            }
        }
        
        // Set new focus
        if let Some(widget) = self.find_widget_mut(id) {
            widget.state.focused = true;
            self.focused_widget = Some(id);
        }
    }
    
    pub fn apply_theme(&mut self, theme: Theme) {
        self.theme = theme;
        
        // Update all widgets with theme colors
        if let Some(root) = self.root.as_mut() {
            Self::apply_theme_recursive(root, &self.theme);
        }
    }
    
    fn apply_theme_recursive(widget: &mut Widget, theme: &Theme) {
        // Apply theme colors based on widget type
        match &widget.kind {
            WidgetKind::Container | WidgetKind::Scroll | WidgetKind::Stack => {
                widget.style.background = theme.colors.background;
            }
            WidgetKind::Text(_) => {
                widget.style.foreground = theme.colors.foreground;
            }
            WidgetKind::Button(_) => {
                widget.style.background = theme.colors.primary;
                widget.style.foreground = Color::rgb(255, 255, 255);
                widget.style.border_radius = theme.borders.radius_medium;
            }
            WidgetKind::Input(_) => {
                widget.style.background = theme.colors.surface;
                widget.style.foreground = theme.colors.foreground;
                widget.style.border_color = theme.colors.border;
                widget.style.border_width = theme.borders.width;
                widget.style.border_radius = theme.borders.radius_small;
            }
            _ => {}
        }
        
        // Apply to children
        for child in &mut widget.children {
            Self::apply_theme_recursive(child, theme);
        }
    }
}

/// Convenience functions for common widgets
pub mod widgets {
    use super::*;
    
    pub fn container() -> WidgetBuilder {
        WidgetBuilder::new(WidgetKind::Container)
    }
    
    pub fn text(content: &str) -> WidgetBuilder {
        WidgetBuilder::new(WidgetKind::Text(content.into()))
    }
    
    pub fn button(label: &str) -> WidgetBuilder {
        WidgetBuilder::new(WidgetKind::Button(label.into()))
            .padding(Spacing::symmetric(8, 16))
            .border_radius(4)
    }
    
    pub fn input(placeholder: &str) -> WidgetBuilder {
        WidgetBuilder::new(WidgetKind::Input(placeholder.into()))
            .padding(Spacing::all(8))
            .width(Dimension::Fill)
    }
    
    pub fn scroll() -> WidgetBuilder {
        WidgetBuilder::new(WidgetKind::Scroll)
            .width(Dimension::Fill)
            .height(Dimension::Fill)
    }
    
    pub fn list() -> WidgetBuilder {
        WidgetBuilder::new(WidgetKind::List)
    }
    
    pub fn grid() -> WidgetBuilder {
        WidgetBuilder::new(WidgetKind::Grid)
    }
    
    pub fn stack() -> WidgetBuilder {
        WidgetBuilder::new(WidgetKind::Stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::widgets::*;
    
    #[test]
    fn test_widget_builder() {
        let widget = container()
            .width(Dimension::Px(100))
            .height(Dimension::Px(100))
            .padding(Spacing::all(10))
            .background(Color::rgb(255, 0, 0))
            .build();
        
        assert_eq!(widget.layout.width, Dimension::Px(100));
        assert_eq!(widget.layout.height, Dimension::Px(100));
        assert_eq!(widget.style.background, Color::rgb(255, 0, 0));
    }
    
    #[test]
    fn test_widget_hierarchy() {
        let child1 = text("Hello").build();
        let child2 = button("Click").build();
        
        let parent = container()
            .child(child1)
            .child(child2)
            .build();
        
        assert_eq!(parent.children.len(), 2);
    }
    
    #[test]
    fn test_widget_api() {
        let theme = Theme {
            name: "Test".into(),
            colors: crate::theming::ColorScheme::dark_default(),
            typography: crate::theming::Typography::default(),
            spacing: crate::theming::Spacing::default(),
            borders: crate::theming::Borders::default(),
            shadows: crate::theming::Shadows::default(),
            animations: crate::theming::Animations::default(),
        };
        
        let mut api = WidgetApi::new(theme);
        
        let root = container()
            .child(text("Test").build())
            .build();
        
        let root_id = root.id;
        api.set_root(root);
        
        assert!(api.find_widget(root_id).is_some());
    }
}
