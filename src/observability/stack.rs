#![no_std]
#![no_main]

extern crate alloc;

use core::mem;
/// OOP-based Observability Stack for SigmaOS
/// Implements observability using OOP principles with traits and structs
/// No dependency on external observability frameworks
/// Based on Roadmap Item 90: Observability stack
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::vec::Vec;
use alloc::boxed::Box;

/// Metric ID
pub type MetricID = usize;

/// Metric type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MetricType {
    Counter = 0,
    Gauge = 1,
    Histogram = 2,
    Summary = 3,
}

/// Metric trait (OOP interface)
pub trait Metric {
    /// Get metric ID
    fn id(&self) -> MetricID;
    /// Get metric name
    fn name(&self) -> &[u8];
    /// Get metric type
    fn metric_type(&self) -> MetricType;
    /// Record value
    fn record(&mut self, value: f64);
    /// Get current value
    fn value(&self) -> f64;
    /// Get metric info
    fn info(&self) -> MetricInfo;
}

/// Metric info
#[repr(C)]
pub struct MetricInfo {
    pub id: MetricID,
    pub name: [u8; 64],
    pub metric_type: MetricType,
    pub value: f64,
    pub capability: MetricCapability,
}

impl MetricInfo {
    pub fn new(id: MetricID, metric_type: MetricType) -> Self {
        MetricInfo {
            id,
            name: [0; 64],
            metric_type,
            value: 0.0,
            capability: MetricCapability::new(),
        }
    }
}

/// Metric capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MetricCapability {
    pub can_record: bool,
    pub can_reset: bool,
}

impl MetricCapability {
    pub fn new() -> Self {
        MetricCapability {
            can_record: false,
            can_reset: false,
        }
    }

    pub fn full() -> Self {
        MetricCapability {
            can_record: true,
            can_reset: true,
        }
    }
}

/// Simple metric (OOP: Concrete metric class)
#[repr(C)]
pub struct SimpleMetric {
    pub id: MetricID,
    pub name: [u8; 64],
    pub metric_type: MetricType,
    pub value: AtomicUsize, // Store as usize for atomic operations
    pub capability: MetricCapability,
}

impl SimpleMetric {
    pub fn new(
        id: MetricID,
        name: &[u8],
        metric_type: MetricType,
        capability: MetricCapability,
    ) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleMetric {
            id,
            name: name_array,
            metric_type,
            value: AtomicUsize::new(0),
            capability,
        }
    }

    fn f64_to_usize(f: f64) -> usize {
        f as usize
    }

    fn usize_to_f64(u: usize) -> f64 {
        u as f64
    }
}

impl Metric for SimpleMetric {
    fn id(&self) -> MetricID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn metric_type(&self) -> MetricType {
        self.metric_type
    }

    fn record(&mut self, value: f64) {
        if !self.capability.can_record {
            return;
        }

        match self.metric_type {
            MetricType::Counter => {
                self.value
                    .fetch_add(Self::f64_to_usize(value), Ordering::SeqCst);
            }
            MetricType::Gauge => {
                self.value
                    .store(Self::f64_to_usize(value), Ordering::SeqCst);
            }
            MetricType::Histogram | MetricType::Summary => {
                self.value
                    .fetch_add(Self::f64_to_usize(value), Ordering::SeqCst);
            }
        }
    }

    fn value(&self) -> f64 {
        Self::usize_to_f64(self.value.load(Ordering::SeqCst))
    }

    fn info(&self) -> MetricInfo {
        MetricInfo {
            id: self.id,
            name: self.name,
            metric_type: self.metric_type,
            value: self.value(),
            capability: self.capability,
        }
    }
}

/// Trace ID
pub type TraceID = usize;

/// Span trait (OOP interface)
pub trait Span {
    /// Get span ID
    fn id(&self) -> TraceID;
    /// Get span name
    fn name(&self) -> &[u8];
    /// Start span
    fn start(&mut self);
    /// Stop span
    fn stop(&mut self);
    /// Get duration (nanoseconds)
    fn duration(&self) -> u64;
    /// Get span info
    fn info(&self) -> SpanInfo;
}

/// Span info
#[repr(C)]
pub struct SpanInfo {
    pub id: TraceID,
    pub name: [u8; 64],
    pub start_time: u64,
    pub end_time: u64,
    pub duration: u64,
    pub capability: SpanCapability,
}

impl SpanInfo {
    pub fn new(id: TraceID) -> Self {
        SpanInfo {
            id,
            name: [0; 64],
            start_time: 0,
            end_time: 0,
            duration: 0,
            capability: SpanCapability::new(),
        }
    }
}

/// Span capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpanCapability {
    pub can_start: bool,
    pub can_stop: bool,
}

impl SpanCapability {
    pub fn new() -> Self {
        SpanCapability {
            can_start: false,
            can_stop: false,
        }
    }

    pub fn full() -> Self {
        SpanCapability {
            can_start: true,
            can_stop: true,
        }
    }
}

/// Simple span (OOP: Concrete span class)
#[repr(C)]
pub struct SimpleSpan {
    pub id: TraceID,
    pub name: [u8; 64],
    pub start_time: AtomicUsize,
    pub end_time: AtomicUsize,
    pub capability: SpanCapability,
}

impl SimpleSpan {
    pub fn new(id: TraceID, name: &[u8], capability: SpanCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleSpan {
            id,
            name: name_array,
            start_time: AtomicUsize::new(0),
            end_time: AtomicUsize::new(0),
            capability,
        }
    }

    fn get_current_time() -> u64 {
        static mut COUNTER: u64 = 0;
        unsafe {
            COUNTER += 1_000_000;
            COUNTER
        }
    }
}

impl Span for SimpleSpan {
    fn id(&self) -> TraceID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn start(&mut self) {
        if !self.capability.can_start {
            return;
        }
        self.start_time
            .store(Self::get_current_time(), Ordering::SeqCst);
    }

    fn stop(&mut self) {
        if !self.capability.can_stop {
            return;
        }
        self.end_time
            .store(Self::get_current_time(), Ordering::SeqCst);
    }

    fn duration(&self) -> u64 {
        let start = self.start_time.load(Ordering::SeqCst) as u64;
        let end = self.end_time.load(Ordering::SeqCst) as u64;
        if end > start {
            end - start
        } else {
            0
        }
    }

    fn info(&self) -> SpanInfo {
        SpanInfo {
            id: self.id,
            name: self.name,
            start_time: self.start_time.load(Ordering::SeqCst) as u64,
            end_time: self.end_time.load(Ordering::SeqCst) as u64,
            duration: self.duration(),
            capability: self.capability,
        }
    }
}

/// Observability stack trait (OOP interface)
pub trait ObservabilityStack {
    /// Register metric
    fn register_metric(&mut self, metric: Box<dyn Metric>) -> Result<MetricID, ObservabilityError>;
    /// Unregister metric
    fn unregister_metric(&mut self, id: MetricID) -> Result<(), ObservabilityError>;
    /// Get metric
    fn get_metric(&self, id: MetricID) -> Option<&dyn Metric>;
    /// Create span
    fn create_span(&mut self, name: &[u8]) -> Result<TraceID, ObservabilityError>;
    /// Close span
    fn close_span(&mut self, id: TraceID) -> Result<(), ObservabilityError>;
    /// Get span
    fn get_span(&self, id: TraceID) -> Option<&dyn Span>;
    /// Get stack statistics
    fn stats(&self) -> ObservabilityStats;
}

/// Observability error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ObservabilityError {
    Success = 0,
    MetricNotFound = 1,
    SpanNotFound = 2,
    PermissionDenied = 3,
}

/// Observability statistics
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObservabilityStats {
    pub total_metrics: usize,
    pub total_spans: usize,
    pub active_spans: usize,
    pub metrics_by_type: [usize; 4],
}

impl ObservabilityStats {
    pub fn new() -> Self {
        ObservabilityStats {
            total_metrics: 0,
            total_spans: 0,
            active_spans: 0,
            metrics_by_type: [0; 4],
        }
    }
}

/// Simple observability stack (OOP: Concrete stack class)
pub struct SimpleObservabilityStack {
    metrics: Vec<Option<Box<dyn Metric>>>,
    spans: Vec<Option<Box<dyn Span>>>,
    next_metric_id: AtomicUsize,
    next_span_id: AtomicUsize,
    stats: ObservabilityStats,
    capability: StackCapability,
}

/// Stack capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StackCapability {
    pub can_register_metrics: bool,
    pub can_create_spans: bool,
}

impl StackCapability {
    pub fn new() -> Self {
        StackCapability {
            can_register_metrics: false,
            can_create_spans: false,
        }
    }

    pub fn full() -> Self {
        StackCapability {
            can_register_metrics: true,
            can_create_spans: true,
        }
    }
}

impl SimpleObservabilityStack {
    pub fn new(capability: StackCapability) -> Self {
        SimpleObservabilityStack {
            metrics: Vec::new(),
            spans: Vec::new(),
            next_metric_id: AtomicUsize::new(1),
            next_span_id: AtomicUsize::new(1),
            stats: ObservabilityStats::new(),
            capability,
        }
    }
}

impl ObservabilityStack for SimpleObservabilityStack {
    fn register_metric(&mut self, metric: Box<dyn Metric>) -> Result<MetricID, ObservabilityError> {
        if !self.capability.can_register_metrics {
            return Err(ObservabilityError::PermissionDenied);
        }

        let id = metric.id();
        let metric_type = metric.metric_type();
        self.metrics.push(Some(metric));
        self.stats.total_metrics += 1;
        self.stats.metrics_by_type[metric_type as usize] += 1;
        Ok(id)
    }

    fn unregister_metric(&mut self, id: MetricID) -> Result<(), ObservabilityError> {
        if !self.capability.can_register_metrics {
            return Err(ObservabilityError::PermissionDenied);
        }

        let mut index = None;
        let mut metric_type = MetricType::Counter;

        for (i, metric_option) in self.metrics.iter().enumerate() {
            if let Some(ref metric) = *metric_option {
                if metric.id() == id {
                    index = Some(i);
                    metric_type = metric.metric_type();
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.metrics[i] = None;
            self.stats.total_metrics -= 1;
            self.stats.metrics_by_type[metric_type as usize] -= 1;
            Ok(())
        } else {
            Err(ObservabilityError::MetricNotFound)
        }
    }

    fn get_metric(&self, id: MetricID) -> Option<&dyn Metric> {
        for metric_option in &self.metrics {
            if let Some(ref metric) = *metric_option {
                if metric.id() == id {
                    return Some(metric.as_ref());
                }
            }
        }
        None
    }

    fn create_span(&mut self, name: &[u8]) -> Result<TraceID, ObservabilityError> {
        if !self.capability.can_create_spans {
            return Err(ObservabilityError::PermissionDenied);
        }

        let id = self.next_span_id.fetch_add(1, Ordering::SeqCst);
        let span = SimpleSpan::new(id, name, SpanCapability::full());
        self.spans.push(Some(Box::new(span)));
        self.stats.total_spans += 1;
        self.stats.active_spans += 1;
        Ok(id)
    }

    fn close_span(&mut self, id: TraceID) -> Result<(), ObservabilityError> {
        if !self.capability.can_create_spans {
            return Err(ObservabilityError::PermissionDenied);
        }

        for span_option in &mut self.spans {
            if let Some(ref mut span) = *span_option {
                if span.id() == id {
                    span.stop();
                    self.stats.active_spans -= 1;
                    return Ok(());
                }
            }
        }
        Err(ObservabilityError::SpanNotFound)
    }

    fn get_span(&self, id: TraceID) -> Option<&dyn Span> {
        for span_option in &self.spans {
            if let Some(ref span) = *span_option {
                if span.id() == id {
                    return Some(span.as_ref());
                }
            }
        }
        None
    }

    fn stats(&self) -> ObservabilityStats {
        self.stats
    }
}

// Simple Vec implementation for no_std

pub trait SigmaDebug {
    fn debug_dump(&self) -> alloc::string::String;
}

pub trait SigmaMetrics {
    fn collect_metrics(&self) -> ObservabilityStats;
}

pub trait SigmaTrace {
    fn trace_event(&self, event: &str);
}

pub struct SimpleSigmaDebug;
impl SigmaDebug for SimpleSigmaDebug {
    fn debug_dump(&self) -> alloc::string::String {
        alloc::string::String::from("SimpleSigmaDebug dump")
    }
}

pub struct SimpleSigmaMetrics;
impl SigmaMetrics for SimpleSigmaMetrics {
    fn collect_metrics(&self) -> ObservabilityStats {
        ObservabilityStats::new()
    }
}

pub struct SimpleSigmaTrace;
impl SigmaTrace for SimpleSigmaTrace {
    fn trace_event(&self, _event: &str) {}
}
