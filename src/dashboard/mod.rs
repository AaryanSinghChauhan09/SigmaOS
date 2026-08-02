// SigmaOS Dashboard Module
pub mod control_center;
pub mod monitor;
pub mod process;
pub mod accessibility_gamification;

pub use control_center::{
    ControlCenterError, ControlPanel, ControlPanelImpl, DashboardWidget as ControlWidget,
    DisplayPanel, MetricData as ControlMetric, MetricType as ControlMetricType, NetworkPanel,
    PanelState, QuickActionType, QuickSetting, SoundPanel, UnifiedControlCenter,
    WidgetType as ControlWidgetType,
};
pub use monitor::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use accessibility_gamification::{
    ColorFilter, AccessibilityOverlay, Trophy, GamifiedProductivityTracker,
};
pub use process::{
    ProcessAction, ProcessError, ProcessFilter, ProcessInfo, ProcessManager,
    ProcessMonitorStrategy, ProcessPriority, ProcessState, SystemProcessMonitor,
};
