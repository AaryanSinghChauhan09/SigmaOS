// SigmaOS Dashboard Module
pub mod accessibility_gamification;
pub mod control_center;
pub mod monitor;
pub mod process;
pub mod privacy;
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
pub use privacy::{
    PrivacyDashboard, TelemetryRule, PrivacyBadgerTrackerShield, PrivacyPreset,
    TelemetryCategory, TrackerAction,
};
pub use statutory_compliance::{
    ComplianceRuleStatus, DisputeAuditRollbackEngine, PenaltyBreachNotifier,
    StatutoryBreachAlert, StatutoryFramework, StatutoryGovernanceLayer, StatutoryGovernanceRule,
};
