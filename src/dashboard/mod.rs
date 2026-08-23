#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Dashboard Module
pub mod accessibility_gamification;
pub mod control_center;
pub mod monitor;
pub mod privacy;
pub mod process;
pub mod statutory_compliance;

pub use accessibility_gamification::{
    AccessibilityOverlay, ColorFilter, GamifiedProductivityTracker, Trophy,
};
pub use control_center::{
    ControlCenterError, ControlPanel, ControlPanelImpl, DashboardWidget as ControlWidget,
    DisplayPanel, MetricData as ControlMetric, MetricType as ControlMetricType, NetworkPanel,
    PanelState, QuickActionType, QuickSetting, SoundPanel, UnifiedControlCenter,
    WidgetType as ControlWidgetType,
};
pub use monitor::{
    DashboardWidget, MetricData, MetricType, SystemMonitor, UnifiedDashboard, WidgetType,
};
pub use process::{
    ProcessAction, ProcessError, ProcessFilter, ProcessInfo, ProcessManager,
    ProcessMonitorStrategy, ProcessPriority, ProcessState, SystemProcessMonitor,
};
