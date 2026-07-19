// SigmaOS Dashboard Module
pub mod control_center;
pub mod monitor;
pub mod process;

pub use control_center::{
    ControlCenterError, ControlPanel, ControlPanelImpl, DashboardWidget as ControlWidget,
    DisplayPanel, MetricData as ControlMetric, MetricType as ControlMetricType, NetworkPanel,
    PanelState, QuickActionType, QuickSetting, SoundPanel, UnifiedControlCenter, WidgetType as ControlWidgetType,
};
pub use monitor::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use process::{
    ProcessAction, ProcessFilter, ProcessInfo, ProcessManager, ProcessMonitorStrategy,
    ProcessPriority, ProcessState, ProcessError, SystemProcessMonitor,
};
