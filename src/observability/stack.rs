extern crate alloc;

use core::mem;
/// OOP-based Observability Stack for SigmaOS
/// Implements observability using OOP principles with traits and structs
/// No dependency on external observability frameworks
/// Based on Roadmap Item 90: Observability stack
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};

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
            .store(Self::get_current_time() as usize, Ordering::SeqCst);
    }

    fn stop(&mut self) {
        if !self.capability.can_stop {
            return;
        }
        self.end_time
            .store(Self::get_current_time() as usize, Ordering::SeqCst);
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
#[derive(Debug, Clone, Copy)]
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

/// SigmaTrace dynamic span profiling trait
pub trait SigmaTrace {
    fn trace_event(
        &mut self,
        event_name: &[u8],
        payload_size: usize,
    ) -> Result<(), ObservabilityError>;
    fn get_total_payload_traced(&self) -> usize;
}

/// Simple implementation of SigmaTrace
pub struct SimpleSigmaTrace {
    pub total_payload: AtomicUsize,
    pub event_count: AtomicUsize,
}

impl SimpleSigmaTrace {
    pub fn new() -> Self {
        SimpleSigmaTrace {
            total_payload: AtomicUsize::new(0),
            event_count: AtomicUsize::new(0),
        }
    }
}

impl SigmaTrace for SimpleSigmaTrace {
    fn trace_event(
        &mut self,
        _event_name: &[u8],
        payload_size: usize,
    ) -> Result<(), ObservabilityError> {
        self.total_payload.fetch_add(payload_size, Ordering::SeqCst);
        self.event_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn get_total_payload_traced(&self) -> usize {
        self.total_payload.load(Ordering::SeqCst)
    }
}

/// SigmaMetrics telemetry metrics export trait
pub trait SigmaMetrics {
    fn export_metrics_prometheus(&self) -> Result<f64, ObservabilityError>;
    fn increment_export_count(&mut self);
}

/// Simple implementation of SigmaMetrics
pub struct SimpleSigmaMetrics {
    pub export_count: AtomicUsize,
    pub last_exported_value: AtomicUsize,
}

impl SimpleSigmaMetrics {
    pub fn new() -> Self {
        SimpleSigmaMetrics {
            export_count: AtomicUsize::new(0),
            last_exported_value: AtomicUsize::new(0),
        }
    }
}

impl SigmaMetrics for SimpleSigmaMetrics {
    fn export_metrics_prometheus(&self) -> Result<f64, ObservabilityError> {
        let val = self.last_exported_value.load(Ordering::SeqCst) as f64;
        Ok(val)
    }

    fn increment_export_count(&mut self) {
        self.export_count.fetch_add(1, Ordering::SeqCst);
    }
}

/// SigmaDebug crash dump & debugger observability integration trait
pub trait SigmaDebug {
    fn capture_core_dump(&mut self, error_code: u32) -> Result<(), ObservabilityError>;
    fn load_symbols(&mut self, symbol_count: usize) -> Result<(), ObservabilityError>;
    fn get_last_error(&self) -> u32;
}

/// Simple implementation of SigmaDebug
pub struct SimpleSigmaDebug {
    pub last_error: AtomicUsize,
    pub symbols_loaded: AtomicUsize,
}

impl SimpleSigmaDebug {
    pub fn new() -> Self {
        SimpleSigmaDebug {
            last_error: AtomicUsize::new(0),
            symbols_loaded: AtomicUsize::new(0),
        }
    }
}

impl SigmaDebug for SimpleSigmaDebug {
    fn capture_core_dump(&mut self, error_code: u32) -> Result<(), ObservabilityError> {
        self.last_error.store(error_code as usize, Ordering::SeqCst);
        Ok(())
    }

    fn load_symbols(&mut self, symbol_count: usize) -> Result<(), ObservabilityError> {
        self.symbols_loaded.store(symbol_count, Ordering::SeqCst);
        Ok(())
    }

    fn get_last_error(&self) -> u32 {
        self.last_error.load(Ordering::SeqCst) as u32
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
    pub trace: SimpleSigmaTrace,
    pub metrics_exporter: SimpleSigmaMetrics,
    pub debug_helper: SimpleSigmaDebug,
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
            trace: SimpleSigmaTrace::new(),
            metrics_exporter: SimpleSigmaMetrics::new(),
            debug_helper: SimpleSigmaDebug::new(),
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
            if let Some(ref metric) = metric_option {
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
        for metric_option in &*self.metrics {
            if let Some(ref metric) = metric_option {
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

        for span_option in &mut *self.spans {
            if let Some(ref mut span) = span_option {
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
        for span_option in &*self.spans {
            if let Some(ref span) = span_option {
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

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmatrace_dynamic_span_profiling() {
        let mut trace = SimpleSigmaTrace::new();
        assert_eq!(trace.get_total_payload_traced(), 0);
        assert!(trace.trace_event(b"syscall_read", 256).is_ok());
        assert_eq!(trace.get_total_payload_traced(), 256);
        assert_eq!(trace.event_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_sigmametrics_telemetry() {
        let mut metrics = SimpleSigmaMetrics::new();
        metrics.last_exported_value.store(42, Ordering::SeqCst);
        let val = metrics.export_metrics_prometheus().unwrap();
        assert_eq!(val, 42.0);
        metrics.increment_export_count();
        assert_eq!(metrics.export_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_sigmadebug_dump_and_symbols() {
        let mut dbg = SimpleSigmaDebug::new();
        assert_eq!(dbg.get_last_error(), 0);
        assert!(dbg.capture_core_dump(0xDEADBEEF).is_ok());
        assert_eq!(dbg.get_last_error(), 0xDEADBEEF);
        assert!(dbg.load_symbols(1500).is_ok());
        assert_eq!(dbg.symbols_loaded.load(Ordering::SeqCst), 1500);
    }
}
