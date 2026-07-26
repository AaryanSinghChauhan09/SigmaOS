// SigmaOS Dashboard Module
pub mod control_center;
pub mod monitor;
pub mod process;

pub use monitor::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
