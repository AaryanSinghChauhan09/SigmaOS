//! System Observability (Prometheus/Grafana/Elasticsearch Inspiration)
//! Metrics collection, log aggregation, distributed tracing, and dashboards

#![no_std]

extern crate alloc;

use crate::klib::{Vec, String};
use alloc::vec::Vec;
use alloc::string::String;

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Metric
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub metric_type: MetricType,
    pub value: f64,
    pub labels: Vec<(String, String)>,
    pub timestamp: u64,
}

impl Metric {
    pub fn new(name: &str, metric_type: MetricType, value: f64) -> Self {
        Self {
            name: name.to_string(),
            metric_type,
            value,
            labels: Vec::new(),
            timestamp: 0,
        }
    }

    pub fn add_label(&mut self, key: &str, value: &str) {
        self.labels.push((key.to_string(), value.to_string()));
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }
}

/// Metrics collector
pub struct MetricsCollector {
    pub metrics: Vec<Metric>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
        }
    }

    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
    }

    pub fn get_metric(&self, name: &str) -> Option<&Metric> {
        self.metrics.iter().find(|m| m.name == name)
    }

    pub fn list_metrics(&self) -> Vec<&Metric> {
        self.metrics.iter().collect()
    }

    pub fn get_metric_value(&self, name: &str) -> Option<f64> {
        self.get_metric(name).map(|m| m.value)
    }
}

/// Log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub service: String,
    pub message: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl LogEntry {
    pub fn new(level: LogLevel, service: &str, message: &str) -> Self {
        Self {
            timestamp: 0,
            level,
            service: service.to_string(),
            message: message.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, key: &str, value: &str) {
        self.fields.push((key.to_string(), value.to_string()));
    }
}

/// Log aggregator
pub struct LogAggregator {
    pub logs: Vec<LogEntry>,
}

impl LogAggregator {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
        }
    }

    pub fn add_log(&mut self, log: LogEntry) {
        self.logs.push(log);
    }

    pub fn query_logs(&self, service: &str, level: LogLevel) -> Vec<&LogEntry> {
        self.logs.iter()
            .filter(|l| l.service == service && l.level == level)
            .collect()
    }

    pub fn get_logs_by_service(&self, service: &str) -> Vec<&LogEntry> {
        self.logs.iter().filter(|l| l.service == service).collect()
    }
}

/// Trace span
#[derive(Debug, Clone)]
pub struct TraceSpan {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub operation_name: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub tags: Vec<(String, String)>,
}

impl TraceSpan {
    pub fn new(trace_id: &str, span_id: &str, operation_name: &str) -> Self {
        Self {
            trace_id: trace_id.to_string(),
            span_id: span_id.to_string(),
            parent_span_id: None,
            operation_name: operation_name.to_string(),
            start_time: 0,
            end_time: None,
            tags: Vec::new(),
        }
    }

    pub fn set_parent(&mut self, parent_span_id: &str) {
        self.parent_span_id = Some(parent_span_id.to_string());
    }

    pub fn add_tag(&mut self, key: &str, value: &str) {
        self.tags.push((key.to_string(), value.to_string()));
    }

    pub fn finish(&mut self) {
        self.end_time = Some(0); // In production, would use actual time
    }

    pub fn duration(&self) -> u64 {
        if let Some(end) = self.end_time {
            end - self.start_time
        } else {
            0
        }
    }
}

/// Tracing system
pub struct TracingSystem {
    pub spans: Vec<TraceSpan>,
}

impl TracingSystem {
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
        }
    }

    pub fn create_span(&mut self, operation_name: &str) -> String {
        let trace_id = Self::generate_id();
        let span_id = Self::generate_id();
        let span = TraceSpan::new(&trace_id, &span_id, operation_name);
        self.spans.push(span);
        span_id
    }

    fn generate_id() -> String {
        // Generate trace/span ID (OpenTelemetry inspiration)
        "trace_abcdef1234567890".to_string()
    }

    pub fn get_span(&mut self, span_id: &str) -> Option<&mut TraceSpan> {
        self.spans.iter_mut().find(|s| s.span_id == span_id)
    }

    pub fn finish_span(&mut self, span_id: &str) -> Result<(), ObservabilityError> {
        if let Some(span) = self.get_span(span_id) {
            span.finish();
            Ok(())
        } else {
            Err(ObservabilityError::SpanNotFound)
        }
    }

    pub fn get_trace(&self, trace_id: &str) -> Vec<&TraceSpan> {
        self.spans.iter().filter(|s| s.trace_id == trace_id).collect()
    }
}

/// Dashboard
#[derive(Debug, Clone)]
pub struct Dashboard {
    pub name: String,
    pub panels: Vec<Panel>,
}

#[derive(Debug, Clone)]
pub struct Panel {
    pub id: String,
    pub title: String,
    pub panel_type: PanelType,
    pub queries: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelType {
    Graph,
    Gauge,
    Table,
    Stat,
}

impl Dashboard {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            panels: Vec::new(),
        }
    }

    pub fn add_panel(&mut self, panel: Panel) {
        self.panels.push(panel);
    }

    pub fn get_panel(&mut self, id: &str) -> Option<&mut Panel> {
        self.panels.iter_mut().find(|p| p.id == id)
    }
}

impl Panel {
    pub fn new(id: &str, title: &str, panel_type: PanelType) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            panel_type,
            queries: Vec::new(),
        }
    }

    pub fn add_query(&mut self, query: &str) {
        self.queries.push(query.to_string());
    }
}

/// SigmaObservability - Observability Platform
pub struct SigmaObservability {
    pub metrics_collector: MetricsCollector,
    pub log_aggregator: LogAggregator,
    pub tracing_system: TracingSystem,
    pub dashboards: Vec<Dashboard>,
}

impl SigmaObservability {
    pub fn new() -> Self {
        Self {
            metrics_collector: MetricsCollector::new(),
            log_aggregator: LogAggregator::new(),
            tracing_system: TracingSystem::new(),
            dashboards: Vec::new(),
        }
    }

    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics_collector.add_metric(metric);
    }

    pub fn add_log(&mut self, log: LogEntry) {
        self.log_aggregator.add_log(log);
    }

    pub fn create_span(&mut self, operation_name: &str) -> String {
        self.tracing_system.create_span(operation_name)
    }

    pub fn add_dashboard(&mut self, dashboard: Dashboard) {
        self.dashboards.push(dashboard);
    }

    pub fn get_observability_stats(&self) -> ObservabilityStats {
        ObservabilityStats {
            total_metrics: self.metrics_collector.metrics.len(),
            total_logs: self.log_aggregator.logs.len(),
            total_spans: self.tracing_system.spans.len(),
            total_dashboards: self.dashboards.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObservabilityStats {
    pub total_metrics: usize,
    pub total_logs: usize,
    pub total_spans: usize,
    pub total_dashboards: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservabilityError {
    MetricNotFound,
    LogNotFound,
    SpanNotFound,
    DashboardNotFound,
    QueryFailed,
}

impl Default for SigmaObservability {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_creation() {
        let metric = Metric::new("test_metric", MetricType::Counter, 100.0);
        assert_eq!(metric.name, "test_metric");
        assert_eq!(metric.value, 100.0);
    }

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();
        let metric = Metric::new("test", MetricType::Counter, 1.0);
        collector.add_metric(metric);
        assert_eq!(collector.list_metrics().len(), 1);
    }

    #[test]
    fn test_log_entry() {
        let log = LogEntry::new(LogLevel::Info, "test-service", "test message");
        assert_eq!(log.service, "test-service");
    }

    #[test]
    fn test_trace_span() {
        let span = TraceSpan::new("trace-1", "span-1", "test-operation");
        assert_eq!(span.operation_name, "test-operation");
    }

    #[test]
    fn test_tracing_system() {
        let mut tracing = TracingSystem::new();
        let span_id = tracing.create_span("test-operation");
        assert!(!span_id.is_empty());
    }

    #[test]
    fn test_dashboard() {
        let mut dashboard = Dashboard::new("test-dashboard");
        let panel = Panel::new("panel-1", "Test Panel", PanelType::Graph);
        dashboard.add_panel(panel);
        assert_eq!(dashboard.panels.len(), 1);
    }

    #[test]
    fn test_sigmaobservability() {
        let mut obs = SigmaObservability::new();
        let metric = Metric::new("test", MetricType::Counter, 1.0);
        obs.add_metric(metric);
        let stats = obs.get_observability_stats();
        assert_eq!(stats.total_metrics, 1);
    }
}