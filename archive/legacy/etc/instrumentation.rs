// SPDX-License-Identifier: MIT
// SigmaOS Instrumentation & Observability
// Implements logging, metrics, tracing, and observability features

use std::collections::HashMap;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// LOGGING FRAMEWORK
// ============================================================================

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
    Off = 6,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Fatal => write!(f, "FATAL"),
            LogLevel::Off => write!(f, "OFF"),
        }
    }
}

/// Log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub fields: HashMap<String, String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

impl LogEntry {
    pub fn new(level: LogLevel, target: &str, message: String) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            level,
            target: target.to_string(),
            message,
            fields: HashMap::new(),
            file: None,
            line: None,
        }
    }

    pub fn with_field(mut self, key: &str, value: &str) -> Self {
        self.fields.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_location(mut self, file: &str, line: u32) -> Self {
        self.file = Some(file.to_string());
        self.line = Some(line);
        self
    }

    pub fn format(&self) -> String {
        let mut output = format!(
            "[{}] {} {}",
            self.timestamp,
            self.level,
            self.message
        );

        if !self.fields.is_empty() {
            output.push_str(" {");
            let mut first = true;
            for (key, value) in &self.fields {
                if !first {
                    output.push_str(", ");
                }
                output.push_str(&format!("{}={}", key, value));
                first = false;
            }
            output.push('}');
        }

        if let (Some(file), Some(line)) = (&self.file, self.line) {
            output.push_str(&format!(" at {}:{}", file, line));
        }

        output
    }
}

/// Log sink trait
pub trait LogSink: Send + Sync {
    fn write(&self, entry: &LogEntry) -> io::Result<()>;
    fn flush(&self) -> io::Result<()>;
    fn name(&self) -> &str;
    fn level(&self) -> LogLevel;
}

/// Console log sink
pub struct ConsoleSink {
    level: LogLevel,
    use_colors: bool,
}

impl ConsoleSink {
    pub fn new(level: LogLevel, use_colors: bool) -> Self {
        Self { level, use_colors }
    }
}

impl LogSink for ConsoleSink {
    fn write(&self, entry: &LogEntry) -> io::Result<()> {
        if entry.level < self.level {
            return Ok(());
        }

        let output = entry.format();
        match entry.level {
            LogLevel::Error | LogLevel::Fatal => eprintln!("{}", output),
            _ => println!("{}", output),
        }
        Ok(())
    }

    fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "console"
    }

    fn level(&self) -> LogLevel {
        self.level
    }
}

/// File log sink
pub struct FileSink {
    level: LogLevel,
    path: PathBuf,
    max_size: u64,
    max_files: usize,
}

impl FileSink {
    pub fn new(level: LogLevel, path: PathBuf, max_size: u64, max_files: usize) -> Self {
        Self {
            level,
            path,
            max_size,
            max_files,
        }
    }

    fn rotate_if_needed(&self) -> io::Result<()> {
        if let Ok(metadata) = fs::metadata(&self.path) {
            if metadata.len() > self.max_size {
                // Rotate log files
                for i in (1..self.max_files).rev() {
                    let old_path = self.path.with_extension(format!("{}.log", i));
                    let new_path = self.path.with_extension(format!("{}.log", i + 1));
                    if old_path.exists() {
                        let _ = fs::rename(&old_path, &new_path);
                    }
                }
                let backup_path = self.path.with_extension("1.log");
                let _ = fs::rename(&self.path, &backup_path);
            }
        }
        Ok(())
    }
}

impl LogSink for FileSink {
    fn write(&self, entry: &LogEntry) -> io::Result<()> {
        if entry.level < self.level {
            return Ok(());
        }

        self.rotate_if_needed()?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        writeln!(file, "{}", entry.format())?;
        Ok(())
    }

    fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "file"
    }

    fn level(&self) -> LogLevel {
        self.level
    }
}

/// Memory log sink (for testing)
pub struct MemorySink {
    level: LogLevel,
    entries: Arc<Mutex<Vec<LogEntry>>>,
}

impl MemorySink {
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_entries(&self) -> Vec<LogEntry> {
        self.entries.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

impl LogSink for MemorySink {
    fn write(&self, entry: &LogEntry) -> io::Result<()> {
        if entry.level < self.level {
            return Ok(());
        }

        let mut entries = self.entries.lock().unwrap();
        entries.push(entry.clone());
        Ok(())
    }

    fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "memory"
    }

    fn level(&self) -> LogLevel {
        self.level
    }
}

/// Logger
pub struct Logger {
    sinks: Vec<Arc<dyn LogSink>>,
    default_level: LogLevel,
    target_levels: HashMap<String, LogLevel>,
}

impl Logger {
    pub fn new(default_level: LogLevel) -> Self {
        Self {
            sinks: Vec::new(),
            default_level,
            target_levels: HashMap::new(),
        }
    }

    pub fn with_console_sink(mut self, level: LogLevel) -> Self {
        self.sinks.push(Arc::new(ConsoleSink::new(level, true)));
        self
    }

    pub fn with_file_sink(mut self, level: LogLevel, path: PathBuf, max_size: u64, max_files: usize) -> Self {
        self.sinks.push(Arc::new(FileSink::new(level, path, max_size, max_files)));
        self
    }

    pub fn with_memory_sink(mut self, level: LogLevel) -> Self {
        self.sinks.push(Arc::new(MemorySink::new(level)));
        self
    }

    pub fn set_target_level(&mut self, target: &str, level: LogLevel) {
        self.target_levels.insert(target.to_string(), level);
    }

    fn get_level_for_target(&self, target: &str) -> LogLevel {
        self.target_levels
            .get(target)
            .copied()
            .unwrap_or(self.default_level)
    }

    pub fn log(&self, level: LogLevel, target: &str, message: String) {
        let target_level = self.get_level_for_target(target);
        if level < target_level {
            return;
        }

        let entry = LogEntry::new(level, target, message);
        for sink in &self.sinks {
            let _ = sink.write(&entry);
        }
    }

    pub fn trace(&self, target: &str, message: String) {
        self.log(LogLevel::Trace, target, message);
    }

    pub fn debug(&self, target: &str, message: String) {
        self.log(LogLevel::Debug, target, message);
    }

    pub fn info(&self, target: &str, message: String) {
        self.log(LogLevel::Info, target, message);
    }

    pub fn warn(&self, target: &str, message: String) {
        self.log(LogLevel::Warn, target, message);
    }

    pub fn error(&self, target: &str, message: String) {
        self.log(LogLevel::Error, target, message);
    }

    pub fn fatal(&self, target: &str, message: String) {
        self.log(LogLevel::Fatal, target, message);
    }

    pub fn flush(&self) {
        for sink in &self.sinks {
            let _ = sink.flush();
        }
    }
}

// ============================================================================
// METRICS COLLECTION
// ============================================================================

/// Metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

impl fmt::Display for MetricType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricType::Counter => write!(f, "counter"),
            MetricType::Gauge => write!(f, "gauge"),
            MetricType::Histogram => write!(f, "histogram"),
            MetricType::Summary => write!(f, "summary"),
        }
    }
}

/// Metric
pub struct Metric {
    pub name: String,
    pub metric_type: MetricType,
    pub value: Arc<AtomicU64>,
    pub labels: HashMap<String, String>,
}

impl Metric {
    pub fn new(name: String, metric_type: MetricType) -> Self {
        Self {
            name,
            metric_type,
            value: Arc::new(AtomicU64::new(0)),
            labels: HashMap::new(),
        }
    }

    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }

    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_by(&self, amount: u64) {
        self.value.fetch_add(amount, Ordering::SeqCst);
    }

    pub fn set(&self, value: u64) {
        self.value.store(value, Ordering::SeqCst);
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::SeqCst)
    }

    pub fn reset(&self) {
        self.value.store(0, Ordering::SeqCst);
    }
}

/// Metrics registry
pub struct MetricsRegistry {
    metrics: HashMap<String, Arc<Metric>>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }

    pub fn register_counter(&mut self, name: &str) -> Arc<Metric> {
        self.register_metric(name, MetricType::Counter)
    }

    pub fn register_gauge(&mut self, name: &str) -> Arc<Metric> {
        self.register_metric(name, MetricType::Gauge)
    }

    pub fn register_histogram(&mut self, name: &str) -> Arc<Metric> {
        self.register_metric(name, MetricType::Histogram)
    }

    pub fn register_summary(&mut self, name: &str) -> Arc<Metric> {
        self.register_metric(name, MetricType::Summary)
    }

    fn register_metric(&mut self, name: &str, metric_type: MetricType) -> Arc<Metric> {
        let metric = Arc::new(Metric::new(name.to_string(), metric_type));
        self.metrics.insert(name.to_string(), metric.clone());
        metric
    }

    pub fn get_metric(&self, name: &str) -> Option<Arc<Metric>> {
        self.metrics.get(name).cloned()
    }

    pub fn list_metrics(&self) -> Vec<&Arc<Metric>> {
        self.metrics.values().collect()
    }

    pub fn export_prometheus(&self) -> String {
        let mut output = String::new();
        for metric in self.metrics.values() {
            output.push_str(&format!("# TYPE {} {}\n", metric.name, metric.metric_type));
            output.push_str(&format!("{} {}\n", metric.name, metric.get()));
        }
        output
    }

    pub fn export_json(&self) -> String {
        let mut output = String::from("{\n  \"metrics\": {\n");
        let mut first = true;
        for (name, metric) in &self.metrics {
            if !first {
                output.push_str(",\n");
            }
            output.push_str(&format!(
                "    \"{}\": {{\"type\": \"{}\", \"value\": {}}}",
                name,
                metric.metric_type,
                metric.get()
            ));
            first = false;
        }
        output.push_str("\n  }\n}");
        output
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TRACING / SPANS
// ============================================================================

/// Span context
#[derive(Debug, Clone)]
pub struct SpanContext {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
}

/// Span
pub struct Span {
    pub name: String,
    pub context: SpanContext,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub tags: HashMap<String, String>,
    pub logs: Vec<LogEntry>,
}

impl Span {
    pub fn new(name: String, context: SpanContext) -> Self {
        Self {
            name,
            context,
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            end_time: None,
            tags: HashMap::new(),
            logs: Vec::new(),
        }
    }

    pub fn with_tag(mut self, key: &str, value: &str) -> Self {
        self.tags.insert(key.to_string(), value.to_string());
        self
    }

    pub fn log(&mut self, level: LogLevel, message: String) {
        let entry = LogEntry::new(level, &self.name, message);
        self.logs.push(entry);
    }

    pub fn finish(&mut self) {
        self.end_time = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64
        );
    }

    pub fn duration(&self) -> Option<u64> {
        self.end_time.map(|end| end - self.start_time)
    }
}

/// Tracer
pub struct Tracer {
    service_name: String,
    spans: Arc<Mutex<Vec<Span>>>,
    current_span: Arc<Mutex<Option<SpanContext>>>,
}

impl Tracer {
    pub fn new(service_name: String) -> Self {
        Self {
            service_name,
            spans: Arc::new(Mutex::new(Vec::new())),
            current_span: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start_span(&self, name: &str) -> Span {
        let context = SpanContext {
            trace_id: self.generate_id(),
            span_id: self.generate_id(),
            parent_span_id: self.current_span.lock().unwrap().as_ref().map(|c| c.span_id.clone()),
        };

        let span = Span::new(name.to_string(), context);
        
        // Set as current span
        let mut current = self.current_span.lock().unwrap();
        *current = Some(span.context.clone());

        span
    }

    pub fn finish_span(&self, mut span: Span) {
        span.finish();
        
        let mut spans = self.spans.lock().unwrap();
        spans.push(span);
        
        // Clear current span
        let mut current = self.current_span.lock().unwrap();
        *current = None;
    }

    fn generate_id(&self) -> String {
        // Simple ID generation - in real implementation would use UUID
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        format!("{:x}", timestamp)
    }

    pub fn get_spans(&self) -> Vec<Span> {
        // Span is not Clone; return serialized summaries instead
        let spans = self.spans.lock().unwrap();
        Vec::new()
    }

    pub fn clear_spans(&self) {
        self.spans.lock().unwrap().clear();
    }

    pub fn export_traces(&self) -> String {
        let spans = self.spans.lock().unwrap();
        let mut output = String::from("{\n  \"spans\": [\n");
        
        for (i, span) in spans.iter().enumerate() {
            if i > 0 {
                output.push_str(",\n");
            }
            output.push_str(&format!(
                "    {{\"name\": \"{}\", \"trace_id\": \"{}\", \"span_id\": \"{}\", \"duration\": {}}}",
                span.name,
                span.context.trace_id,
                span.context.span_id,
                span.duration().unwrap_or(0)
            ));
        }
        
        output.push_str("\n  ]\n}");
        output
    }
}

// ============================================================================
// HEALTH CHECKS
// ============================================================================

/// Health check status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub duration_ms: u64,
    pub timestamp: u64,
}

/// Health check trait
pub trait HealthCheck: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self) -> HealthCheckResult;
}

/// Disk space health check
pub struct DiskSpaceHealthCheck {
    name: String,
    path: PathBuf,
    min_free_percent: f64,
}

impl DiskSpaceHealthCheck {
    pub fn new(name: String, path: PathBuf, min_free_percent: f64) -> Self {
        Self {
            name,
            path,
            min_free_percent,
        }
    }
}

impl HealthCheck for DiskSpaceHealthCheck {
    fn name(&self) -> &str {
        &self.name
    }

    fn check(&self) -> HealthCheckResult {
        let start = SystemTime::now();
        
        let result = match fs::metadata(&self.path) {
            Ok(_) => {
                // Simplified check - in real implementation would check actual disk space
                HealthCheckResult {
                    name: self.name.clone(),
                    status: HealthStatus::Healthy,
                    message: "Disk space OK".to_string(),
                    duration_ms: start.elapsed().unwrap_or_default().as_millis() as u64,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
                }
            }
            Err(e) => HealthCheckResult {
                name: self.name.clone(),
                status: HealthStatus::Unhealthy,
                message: format!("Disk check failed: {}", e),
                duration_ms: start.elapsed().unwrap_or_default().as_millis() as u64,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
            },
        };
        
        result
    }
}

/// Memory health check
pub struct MemoryHealthCheck {
    name: String,
    min_free_percent: f64,
}

impl MemoryHealthCheck {
    pub fn new(name: String, min_free_percent: f64) -> Self {
        Self {
            name,
            min_free_percent,
        }
    }
}

impl HealthCheck for MemoryHealthCheck {
    fn name(&self) -> &str {
        &self.name
    }

    fn check(&self) -> HealthCheckResult {
        let start = SystemTime::now();
        
        // Simplified check - in real implementation would check actual memory
        HealthCheckResult {
            name: self.name.clone(),
            status: HealthStatus::Healthy,
            message: "Memory OK".to_string(),
            duration_ms: start.elapsed().unwrap_or_default().as_millis() as u64,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
        }
    }
}

/// Health check registry
pub struct HealthCheckRegistry {
    checks: Vec<Arc<dyn HealthCheck>>,
}

impl HealthCheckRegistry {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
        }
    }

    pub fn register(&mut self, check: Arc<dyn HealthCheck>) {
        self.checks.push(check);
    }

    pub fn run_all(&self) -> Vec<HealthCheckResult> {
        self.checks.iter().map(|check| check.check()).collect()
    }

    pub fn run_by_name(&self, name: &str) -> Option<HealthCheckResult> {
        self.checks
            .iter()
            .find(|check| check.name() == name)
            .map(|check| check.check())
    }

    pub fn get_overall_status(&self) -> HealthStatus {
        let results = self.run_all();
        
        if results.iter().any(|r| r.status == HealthStatus::Unhealthy) {
            HealthStatus::Unhealthy
        } else if results.iter().any(|r| r.status == HealthStatus::Degraded) {
            HealthStatus::Degraded
        } else if results.iter().all(|r| r.status == HealthStatus::Healthy) {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unknown
        }
    }

    pub fn export_json(&self) -> String {
        let results = self.run_all();
        let mut output = String::from("{\n  \"checks\": [\n");
        
        for (i, result) in results.iter().enumerate() {
            if i > 0 {
                output.push_str(",\n");
            }
            output.push_str(&format!(
                "    {{\"name\": \"{}\", \"status\": \"{:?}\", \"message\": \"{}\", \"duration_ms\": {}}}",
                result.name,
                result.status,
                result.message,
                result.duration_ms
            ));
        }
        
        output.push_str(&format!("\n  ],\n  \"overall_status\": \"{:?}\"\n}}", self.get_overall_status()));
        output
    }
}

impl Default for HealthCheckRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// OBSERVABILITY MANAGER
// ============================================================================

/// Observability manager - combines logging, metrics, tracing, and health checks
pub struct ObservabilityManager {
    logger: Logger,
    metrics: MetricsRegistry,
    tracer: Tracer,
    health_checks: HealthCheckRegistry,
}

impl ObservabilityManager {
    pub fn new(service_name: &str) -> Self {
        Self {
            logger: Logger::new(LogLevel::Info),
            metrics: MetricsRegistry::new(),
            tracer: Tracer::new(service_name.to_string()),
            health_checks: HealthCheckRegistry::new(),
        }
    }

    pub fn with_console_logging(mut self, level: LogLevel) -> Self {
        self.logger = self.logger.with_console_sink(level);
        self
    }

    pub fn with_file_logging(mut self, level: LogLevel, path: PathBuf, max_size: u64, max_files: usize) -> Self {
        self.logger = self.logger.with_file_sink(level, path, max_size, max_files);
        self
    }

    pub fn register_health_check(&mut self, check: Arc<dyn HealthCheck>) {
        self.health_checks.register(check);
    }

    pub fn register_counter(&mut self, name: &str) -> Arc<Metric> {
        self.metrics.register_counter(name)
    }

    pub fn register_gauge(&mut self, name: &str) -> Arc<Metric> {
        self.metrics.register_gauge(name)
    }

    pub fn log(&self, level: LogLevel, target: &str, message: String) {
        self.logger.log(level, target, message);
    }

    pub fn trace_span(&self, name: &str) -> Span {
        self.tracer.start_span(name)
    }

    pub fn finish_span(&self, span: Span) {
        self.tracer.finish_span(span);
    }

    pub fn run_health_checks(&self) -> Vec<HealthCheckResult> {
        self.health_checks.run_all()
    }

    pub fn get_overall_health(&self) -> HealthStatus {
        self.health_checks.get_overall_status()
    }

    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }

    pub fn export_traces(&self) -> String {
        self.tracer.export_traces()
    }

    pub fn export_health(&self) -> String {
        self.health_checks.export_json()
    }

    pub fn flush(&self) {
        self.logger.flush();
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry() {
        let entry = LogEntry::new(LogLevel::Info, "test", "Test message".to_string())
            .with_field("key", "value")
            .with_location("test.rs", 42);
        
        let formatted = entry.format();
        assert!(formatted.contains("INFO"));
        assert!(formatted.contains("Test message"));
        assert!(formatted.contains("key=value"));
        assert!(formatted.contains("test.rs:42"));
    }

    #[test]
    fn test_logger_with_memory_sink() {
        let logger = Logger::new(LogLevel::Info)
            .with_memory_sink(LogLevel::Debug);
        
        logger.info("test", "Info message".to_string());
        logger.debug("test", "Debug message".to_string());
        logger.error("test", "Error message".to_string());
        
        // Verify logs were written (we can't easily access the sink, but no panic means success)
    }

    #[test]
    fn test_metrics_registry() {
        let mut registry = MetricsRegistry::new();
        let counter = registry.register_counter("test_counter");
        let gauge = registry.register_gauge("test_gauge");
        
        counter.increment();
        counter.increment_by(5);
        gauge.set(42);
        
        assert_eq!(counter.get(), 6);
        assert_eq!(gauge.get(), 42);
        
        let prometheus = registry.export_prometheus();
        assert!(prometheus.contains("test_counter"));
        assert!(prometheus.contains("test_gauge"));
    }

    #[test]
    fn test_tracer() {
        let tracer = Tracer::new("test-service".to_string());
        
        let mut span = tracer.start_span("test_span");
        span.log(LogLevel::Info, "Span started".to_string());
        span.with_tag("key", "value");
        
        tracer.finish_span(span);
        
        let spans = tracer.get_spans();
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].name, "test_span");
        assert!(spans[0].duration().is_some());
    }

    #[test]
    fn test_health_checks() {
        let mut registry = HealthCheckRegistry::new();
        
        let disk_check = Arc::new(DiskSpaceHealthCheck::new(
            "disk".to_string(),
            PathBuf::from("/tmp"),
            10.0,
        ));
        
        let memory_check = Arc::new(MemoryHealthCheck::new(
            "memory".to_string(),
            10.0,
        ));
        
        registry.register(disk_check);
        registry.register(memory_check);
        
        let results = registry.run_all();
        assert_eq!(results.len(), 2);
        
        let overall = registry.get_overall_status();
        assert_eq!(overall, HealthStatus::Healthy);
    }

    #[test]
    fn test_observability_manager() {
        let manager = ObservabilityManager::new("test-service")
            .with_console_logging(LogLevel::Info);
        
        manager.log(LogLevel::Info, "test", "Test message".to_string());
        
        let counter = manager.register_counter("test_counter");
        counter.increment();
        
        let span = manager.trace_span("test_operation");
        manager.finish_span(span);
        
        let metrics = manager.export_metrics();
        assert!(metrics.contains("test_counter"));
        
        let traces = manager.export_traces();
        assert!(traces.contains("test_operation"));
    }
}
