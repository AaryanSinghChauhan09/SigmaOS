// SigmaOS Native UI and Mathematical Visualisation Subsystem Mod

pub mod control_center;
pub mod gtk_toolkit;
pub mod math_plotter;
pub mod toolkit;

pub use gtk_toolkit::{
    AdwActionRow, AdwColorScheme, AdwPreferencesGroup, AdwPreferencesPage, DockItem, GtkBox,
    GtkButton, GtkCssProvider, GtkCssRule, GtkHeaderBar, GtkOrientation, GtkSignalDispatcher,
    GtkWidget, SovereignDockBar, SovereignGtkToolkit, SovereignOverviewWorkspaceSwitcher,
    SovereignSystemStatusPanel, SystemTrayApplet,
};

pub use control_center::{
    ControlCenterCategory, DisplaySettingsPlug, NetworkSettingsPlug, SwitchboardPlug,
    UnifiedControlCenter,
};
pub use math_plotter::{PlotFunction, SovereignMathPlotter};
pub use toolkit::{
    LayoutCapability, LayoutStats, SimpleUILayout, SimpleWidget, UIError, UILayout, Widget,
    WidgetCapability, WidgetID, WidgetInfo, WidgetState, WidgetType,
};
