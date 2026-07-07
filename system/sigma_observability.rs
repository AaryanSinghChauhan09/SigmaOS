// sigma_observability.rs — Unified Observability Stack
// eBPF-powered metrics, distributed tracing (OpenTelemetry), structured
// logging, and AI anomaly detection for SigmaOS system health.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

// ── Metrics Engine ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub metric_type: MetricType,
    pub value: f64,
    pub labels: Vec<(String, String)>,
    pub timestamp_ns: u64,
}

#[derive(Debug)]
pub struct MetricsRegistry {
    pub metrics: Vec<Metric>,
    pub scrape_interval_ms: u32,
}

impl MetricsRegistry {
    pub fn new(scrape_interval: u32) -> Self {
        MetricsRegistry {
            metrics: Vec::new(),
            scrape_interval_ms: scrape_interval,
        }
    }

    pub fn record_counter(&mut self, name: &str, value: f64, ts: u64) {
        self.metrics.push(Metric {
            name: String::from(name),
            metric_type: MetricType::Counter,
            value,
            labels: Vec::new(),
            timestamp_ns: ts,
        });
    }

    pub fn record_gauge(&mut self, name: &str, value: f64, ts: u64) {
        self.metrics.push(Metric {
            name: String::from(name),
            metric_type: MetricType::Gauge,
            value,
            labels: Vec::new(),
            timestamp_ns: ts,
        });
    }

    /// Export metrics in Prometheus exposition format
    pub fn export_prometheus(&self) -> String {
        let mut out = String::new();
        for m in &self.metrics {
            let type_str = match m.metric_type {
                MetricType::Counter => "counter",
                MetricType::Gauge => "gauge",
                MetricType::Histogram => "histogram",
                MetricType::Summary => "summary",
            };
            out.push_str(&alloc::format!(
                "# TYPE {} {}\n{} {}\n",
                m.name, type_str, m.name, m.value
            ));
        }
        out
    }
}

// ── Distributed Tracing (OpenTelemetry-compatible) ──────────────────────────

#[derive(Debug, Clone)]
pub struct Span {
    pub trace_id: u128,
    pub span_id: u64,
    pub parent_span_id: Option<u64>,
    pub operation: String,
    pub service: String,
    pub start_ns: u64,
    pub end_ns: u64,
    pub status: SpanStatus,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpanStatus {
    Ok,
    Error,
    Unset,
}

#[derive(Debug)]
pub struct Tracer {
    pub service_name: String,
    pub spans: Vec<Span>,
    pub next_span_id: u64,
}

impl Tracer {
    pub fn new(service: &str) -> Self {
        Tracer {
            service_name: String::from(service),
            spans: Vec::new(),
            next_span_id: 1,
        }
    }

    pub fn start_span(&mut self, op: &str, trace_id: u128, parent: Option<u64>, ts: u64) -> u64 {
        let id = self.next_span_id;
        self.next_span_id += 1;
        self.spans.push(Span {
            trace_id,
            span_id: id,
            parent_span_id: parent,
            operation: String::from(op),
            service: self.service_name.clone(),
            start_ns: ts,
            end_ns: 0,
            status: SpanStatus::Unset,
            attributes: Vec::new(),
        });
        id
    }

    pub fn end_span(&mut self, span_id: u64, ts: u64, status: SpanStatus) {
        if let Some(span) = self.spans.iter_mut().find(|s| s.span_id == span_id) {
            span.end_ns = ts;
            span.status = status;
        }
    }
}

// ── Structured Logging ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub service: String,
    pub timestamp_ns: u64,
    pub fields: Vec<(String, String)>,
    pub trace_id: Option<u128>,
    pub span_id: Option<u64>,
}

#[derive(Debug)]
pub struct Logger {
    pub service: String,
    pub min_level: LogLevel,
    pub entries: Vec<LogEntry>,
}

impl Logger {
    pub fn new(service: &str, min_level: LogLevel) -> Self {
        Logger {
            service: String::from(service),
            min_level,
            entries: Vec::new(),
        }
    }

    pub fn log(&mut self, level: LogLevel, msg: &str, ts: u64) {
        self.entries.push(LogEntry {
            level,
            message: String::from(msg),
            service: self.service.clone(),
            timestamp_ns: ts,
            fields: Vec::new(),
            trace_id: None,
            span_id: None,
        });
    }

    /// Export logs as JSON Lines format
    pub fn export_jsonl(&self) -> String {
        let mut out = String::new();
        for entry in &self.entries {
            out.push_str(&alloc::format!(
                "{{\"level\":\"{:?}\",\"msg\":\"{}\",\"svc\":\"{}\",\"ts\":{}}}\n",
                entry.level, entry.message, entry.service, entry.timestamp_ns
            ));
        }
        out
    }
}

// ── AI Anomaly Detection ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct AnomalyAlert {
    pub metric_name: String,
    pub expected_range: (f64, f64),
    pub actual_value: f64,
    pub severity: u8,
    pub timestamp_ns: u64,
}

/// Simple threshold-based anomaly detection (will integrate with sigma_ai_engine)
pub fn detect_anomalies(metrics: &[Metric], thresholds: &[(String, f64, f64)]) -> Vec<AnomalyAlert> {
    let mut alerts = Vec::new();
    for m in metrics {
        for (name, low, high) in thresholds {
            if m.name == *name && (m.value < *low || m.value > *high) {
                alerts.push(AnomalyAlert {
                    metric_name: m.name.clone(),
                    expected_range: (*low, *high),
                    actual_value: m.value,
                    severity: if m.value > *high * 2.0 { 5 } else { 3 },
                    timestamp_ns: m.timestamp_ns,
                });
            }
        }
    }
    alerts
}
