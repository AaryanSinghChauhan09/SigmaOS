#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Metrics manager for system metrics collection and reporting
/// Inspired by Linux monitoring tools (prometheus, collectd, telegraf)
#[derive(Debug, Clone)]
pub struct MetricsManager {
    metrics: Vec<Metric>,
    collectors: Vec<MetricCollector>,
}

/// A metric
#[derive(Debug, Clone)]
pub struct Metric {
    id: String,
    name: String,
    metric_type: MetricType,
    value: f64,
    labels: Vec<MetricLabel>,
    timestamp: u64,
}

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Metric label
#[derive(Debug, Clone)]
pub struct MetricLabel {
    key: String,
    value: String,
}

/// Metric collector
#[derive(Debug, Clone)]
pub struct MetricCollector {
    id: String,
    name: String,
    collector_type: CollectorType,
    state: CollectorState,
    interval_ms: u32,
}

/// Collector type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectorType {
    Cpu,
    Memory,
    Disk,
    Network,
    Process,
    Custom,
}

/// Collector state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectorState {
    Active,
    Paused,
    Disabled,
}

impl MetricsManager {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
            collectors: Vec::new(),
        }
    }

    /// Record a metric
    pub fn record_metric(&mut self, id: String, name: String, metric_type: MetricType, value: f64, labels: Vec<MetricLabel>, timestamp: u64) -> Result<(), String> {
        if let Some(metric) = self.metrics.iter_mut().find(|m| m.id == id) {
            metric.value = value;
            metric.timestamp = timestamp;
            metric.labels = labels;
        } else {
            let metric = Metric {
                id: id.clone(),
                name,
                metric_type,
                value,
                labels,
                timestamp,
            };
            self.metrics.push(metric);
        }
        Ok(())
    }

    /// Remove a metric
    pub fn remove_metric(&mut self, id: &str) -> Result<(), String> {
        let pos = self.metrics.iter().position(|m| m.id == id)
            .ok_or_else(|| format!("Metric with id {} not found", id))?;
        self.metrics.remove(pos);
        Ok(())
    }

    /// Add a collector
    pub fn add_collector(&mut self, id: String, name: String, collector_type: CollectorType, interval_ms: u32) -> Result<(), String> {
        if self.collectors.iter().any(|c| c.id == id) {
            return Err(format!("Collector with id {} already exists", id));
        }
        
        let collector = MetricCollector {
            id: id.clone(),
            name,
            collector_type,
            state: CollectorState::Active,
            interval_ms,
        };
        self.collectors.push(collector);
        Ok(())
    }

    /// Remove a collector
    pub fn remove_collector(&mut self, id: &str) -> Result<(), String> {
        let pos = self.collectors.iter().position(|c| c.id == id)
            .ok_or_else(|| format!("Collector with id {} not found", id))?;
        self.collectors.remove(pos);
        Ok(())
    }

    /// Pause a collector
    pub fn pause_collector(&mut self, id: &str) -> Result<(), String> {
        let collector = self.collectors.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Collector with id {} not found", id))?;
        collector.state = CollectorState::Paused;
        Ok(())
    }

    /// Resume a collector
    pub fn resume_collector(&mut self, id: &str) -> Result<(), String> {
        let collector = self.collectors.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Collector with id {} not found", id))?;
        collector.state = CollectorState::Active;
        Ok(())
    }

    /// Get all metrics
    pub fn get_metrics(&self) -> &[Metric] {
        &self.metrics
    }

    /// Get metrics by type
    pub fn get_metrics_by_type(&self, metric_type: MetricType) -> Vec<&Metric> {
        self.metrics.iter().filter(|m| m.metric_type == metric_type).collect()
    }

    /// Get metric by ID
    pub fn get_metric(&self, id: &str) -> Option<&Metric> {
        self.metrics.iter().find(|m| m.id == id)
    }

    /// Get all collectors
    pub fn get_collectors(&self) -> &[MetricCollector] {
        &self.collectors
    }

    /// Get active collectors
    pub fn get_active_collectors(&self) -> Vec<&MetricCollector> {
        self.collectors.iter().filter(|c| c.state == CollectorState::Active).collect()
    }
}

impl Default for MetricsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_metric() {
        let mut manager = MetricsManager::new();
        let result = manager.record_metric("metric1".to_string(), "cpu_usage".to_string(), MetricType::Gauge, 75.5, vec![], 1000);
        assert!(result.is_ok());
        assert_eq!(manager.get_metrics().len(), 1);
    }

    #[test]
    fn test_update_metric() {
        let mut manager = MetricsManager::new();
        manager.record_metric("metric1".to_string(), "cpu_usage".to_string(), MetricType::Gauge, 75.5, vec![], 1000).unwrap();
        
        manager.record_metric("metric1".to_string(), "cpu_usage".to_string(), MetricType::Gauge, 80.0, vec![], 2000).unwrap();
        assert_eq!(manager.get_metric("metric1").unwrap().value, 80.0);
    }

    #[test]
    fn test_add_collector() {
        let mut manager = MetricsManager::new();
        let result = manager.add_collector("col1".to_string(), "CPU Collector".to_string(), CollectorType::Cpu, 1000);
        assert!(result.is_ok());
        assert_eq!(manager.get_collectors().len(), 1);
    }

    #[test]
    fn test_pause_resume_collector() {
        let mut manager = MetricsManager::new();
        manager.add_collector("col1".to_string(), "CPU Collector".to_string(), CollectorType::Cpu, 1000).unwrap();
        
        manager.pause_collector("col1").unwrap();
        assert_eq!(manager.get_collector("col1").unwrap().state, CollectorState::Paused);
        
        manager.resume_collector("col1").unwrap();
        assert_eq!(manager.get_collector("col1").unwrap().state, CollectorState::Active);
    }

    #[test]
    fn test_get_metrics_by_type() {
        let mut manager = MetricsManager::new();
        manager.record_metric("metric1".to_string(), "cpu_usage".to_string(), MetricType::Gauge, 75.5, vec![], 1000).unwrap();
        manager.record_metric("metric2".to_string(), "requests_total".to_string(), MetricType::Counter, 1000.0, vec![], 1000).unwrap();
        
        let gauges = manager.get_metrics_by_type(MetricType::Gauge);
        assert_eq!(gauges.len(), 1);
        assert_eq!(gauges[0].id, "metric1");
    }
}
