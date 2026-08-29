// SigmaOS Native UI and Mathematical Visualisation Subsystem Mod

pub mod math_plotter;
pub mod toolkit;
pub mod control_center;

pub use math_plotter::{PlotFunction, SovereignMathPlotter};
pub use toolkit::{
    LayoutCapability, LayoutStats, SimpleUILayout, SimpleWidget, UIError, UILayout, Widget,
    WidgetCapability, WidgetID, WidgetInfo, WidgetState, WidgetType,
};
pub use control_center::{
    UnifiedControlCenter, SwitchboardPlug, ControlCenterCategory, SystemSettingItem,
};
