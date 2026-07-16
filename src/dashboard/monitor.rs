// SigmaOS Unified Dashboard System
// Publisher-grade dashboards for system monitoring and productivity

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// System metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    CPU,
    Memory,
    Disk,
    Network,
    Temperature,
    Power,
}

/// System metric data point
#[derive(Debug, Clone)]
pub struct MetricData {
    pub metric_type: MetricType,
    pub value: f64,
    pub unit: String,
    pub timestamp: Instant,
}

/// Dashboard widget type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetType {
    LineChart,
    BarChart,
    Gauge,
    Text,
    Progress,
}

/// Dashboard widget
#[derive(Debug, Clone)]
pub struct DashboardWidget {
    pub id: String,
    pub widget_type: WidgetType,
    pub title: String,
    pub data: Vec<MetricData>,
    pub position: (u32, u32),
    pub size: (u32, u32),
}

impl DashboardWidget {
    pub fn new(id: String, widget_type: WidgetType, title: String) -> Self {
        Self {
            id,
            widget_type,
            title,
            data: Vec::new(),
            position: (0, 0),
            size: (200, 150),
        }
    }

    pub fn with_position(mut self, x: u32, y: u32) -> Self {
        self.position = (x, y);
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    pub fn add_data_point(&mut self, data: MetricData) {
        self.data.push(data);
        // Keep only last 100 data points
        if self.data.len() > 100 {
            self.data.remove(0);
        }
    }

    pub fn get_latest_value(&self) -> Option<f64> {
        self.data.last().map(|d| d.value)
    }

    pub fn get_average(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.data.iter().map(|d| d.value).sum();
        sum / self.data.len() as f64
    }
}

/// Unified dashboard
pub struct UnifiedDashboard {
    pub widgets: HashMap<String, DashboardWidget>,
    pub layout: String,
    pub theme: String,
    pub refresh_interval: Duration,
}

impl UnifiedDashboard {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            layout: "grid".to_string(),
            theme: "dark".to_string(),
            refresh_interval: Duration::from_secs(1),
        }
    }

    pub fn with_layout(mut self, layout: String) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_theme(mut self, theme: String) -> Self {
        self.theme = theme;
        self
    }

    pub fn add_widget(&mut self, widget: DashboardWidget) {
        self.widgets.insert(widget.id.clone(), widget);
    }

    pub fn remove_widget(&mut self, id: &str) {
        self.widgets.remove(id);
    }

    pub fn get_widget(&self, id: &str) -> Option<&DashboardWidget> {
        self.widgets.get(id)
    }

    pub fn update_widget(&mut self, id: &str, data: MetricData) {
        if let Some(widget) = self.widgets.get_mut(id) {
            widget.add_data_point(data);
        }
    }

    pub fn get_system_summary(&self) -> HashMap<String, f64> {
        let mut summary = HashMap::new();

        for (id, widget) in &self.widgets {
            if let Some(value) = widget.get_latest_value() {
                summary.insert(id.clone(), value);
            }
        }

        summary
    }

    pub fn set_refresh_interval(&mut self, interval: Duration) {
        self.refresh_interval = interval;
    }
}

impl Default for UnifiedDashboard {
    fn default() -> Self {
        Self::new()
    }
}

/// System monitor
pub struct SystemMonitor {
    pub dashboard: UnifiedDashboard,
    pub running: bool,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut dashboard = UnifiedDashboard::new();

        // Add default widgets
        let cpu_widget = DashboardWidget::new(
            "cpu".to_string(),
            WidgetType::LineChart,
            "CPU Usage".to_string(),
        )
        .with_position(0, 0)
        .with_size(400, 200);

        let memory_widget = DashboardWidget::new(
            "memory".to_string(),
            WidgetType::Gauge,
            "Memory Usage".to_string(),
        )
        .with_position(400, 0)
        .with_size(200, 200);

        let disk_widget = DashboardWidget::new(
            "disk".to_string(),
            WidgetType::Progress,
            "Disk Usage".to_string(),
        )
        .with_position(0, 200)
        .with_size(300, 100);

        dashboard.add_widget(cpu_widget);
        dashboard.add_widget(memory_widget);
        dashboard.add_widget(disk_widget);

        Self {
            dashboard,
            running: false,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn update_metrics(&mut self) {
        if !self.running {
            return;
        }

        // Simulate metric updates
        let cpu_data = MetricData {
            metric_type: MetricType::CPU,
            value: 45.0 + (rand::random::<f64>() * 20.0),
            unit: "%".to_string(),
            timestamp: Instant::now(),
        };

        let memory_data = MetricData {
            metric_type: MetricType::Memory,
            value: 60.0 + (rand::random::<f64>() * 15.0),
            unit: "%".to_string(),
            timestamp: Instant::now(),
        };

        let disk_data = MetricData {
            metric_type: MetricType::Disk,
            value: 75.0,
            unit: "%".to_string(),
            timestamp: Instant::now(),
        };

        self.dashboard.update_widget("cpu", cpu_data);
        self.dashboard.update_widget("memory", memory_data);
        self.dashboard.update_widget("disk", disk_data);
    }

    pub fn get_dashboard(&self) -> &UnifiedDashboard {
        &self.dashboard
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = UnifiedDashboard::new();
        assert_eq!(dashboard.layout, "grid");
        assert_eq!(dashboard.theme, "dark");
    }

    #[test]
    fn test_widget_creation() {
        let widget = DashboardWidget::new(
            "test".to_string(),
            WidgetType::LineChart,
            "Test".to_string(),
        );
        assert_eq!(widget.title, "Test");
    }

    #[test]
    fn test_add_data_point() {
        let mut widget = DashboardWidget::new(
            "test".to_string(),
            WidgetType::LineChart,
            "Test".to_string(),
        );
        let data = MetricData {
            metric_type: MetricType::CPU,
            value: 50.0,
            unit: "%".to_string(),
            timestamp: Instant::now(),
        };
        widget.add_data_point(data.clone());
        assert_eq!(widget.data.len(), 1);
    }

    #[test]
    fn test_average_calculation() {
        let mut widget = DashboardWidget::new(
            "test".to_string(),
            WidgetType::LineChart,
            "Test".to_string(),
        );
        widget.add_data_point(MetricData {
            metric_type: MetricType::CPU,
            value: 40.0,
            unit: "%".to_string(),
            timestamp: Instant::now(),
        });
        widget.add_data_point(MetricData {
            metric_type: MetricType::CPU,
            value: 60.0,
            unit: "%".to_string(),
            timestamp: Instant::now(),
        });
        assert_eq!(widget.get_average(), 50.0);
    }

    #[test]
    fn test_system_monitor() {
        let monitor = SystemMonitor::new();
        assert!(!monitor.running);
        assert_eq!(monitor.dashboard.widgets.len(), 3);
    }
}
