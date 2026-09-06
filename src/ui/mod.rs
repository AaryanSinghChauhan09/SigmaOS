#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Native UI and Mathematical Visualisation Subsystem Mod

pub mod control_center;
pub mod gtk_toolkit;
pub mod math_plotter;
pub mod toolkit;
pub mod control_center;
pub mod folder_color;

pub use control_center::{
    ControlCenterCategory, DisplaySettingsPlug, NetworkSettingsPlug, SwitchboardPlug,
    UnifiedControlCenter,
};
pub use math_plotter::{PlotFunction, SovereignMathPlotter};
pub use toolkit::{
    GtkAccessibilityRole, GtkBox, GtkDisplayMetrics, GtkHeaderBar, GtkOrientation,
    GtkSignalDispatcher, GtkSignalEvent, GtkStyleContext, LayoutCapability, LayoutStats,
    SimpleUILayout, SimpleWidget, UIError, UILayout, Widget, WidgetCapability, WidgetID,
    WidgetInfo, WidgetState, WidgetType,
};
pub use control_center::{
    UnifiedControlCenter, SwitchboardPlug, ControlCenterCategory, SystemSettingItem,
};
pub use folder_color::{
    FolderColor, FolderColorSwitcherEngine, FolderCustomization, FolderEmblem,
};
