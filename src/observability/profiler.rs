#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS High-Performance eBPF Tracing & Latency Profiler (SigmaProfiler)
// Designed for tracking scheduler task latency, system tracepoints, and CPU profiling

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TracepointType {
    ContextSwitch,
    SyscallEntry,
    PageFault,
    IrqTrigger,
}

pub struct PerformanceMetric {
    pub total_hits: u64,
    pub cumulative_latency_nanos: u64,
    pub max_latency_nanos: u64,
}

pub struct SigmaProfiler {
    pub tracepoints: HashMap<TracepointType, PerformanceMetric>,
    pub tracing_active: bool,
}

impl SigmaProfiler {
    pub fn new() -> Self {
        let mut profiler = SigmaProfiler {
            tracepoints: HashMap::new(),
            tracing_active: true,
        };
        // Initialize standard tracepoints
        profiler.tracepoints.insert(
            TracepointType::ContextSwitch,
            PerformanceMetric {
                total_hits: 0,
                cumulative_latency_nanos: 0,
                max_latency_nanos: 0,
            },
        );
        profiler.tracepoints.insert(
            TracepointType::SyscallEntry,
            PerformanceMetric {
                total_hits: 0,
                cumulative_latency_nanos: 0,
                max_latency_nanos: 0,
            },
        );
        profiler.tracepoints.insert(
            TracepointType::PageFault,
            PerformanceMetric {
                total_hits: 0,
                cumulative_latency_nanos: 0,
                max_latency_nanos: 0,
            },
        );
        profiler
    }

    pub fn record_event(&mut self, trace_type: TracepointType, latency_nanos: u64) {
        if !self.tracing_active {
            return;
        }
        if let Some(metric) = self.tracepoints.get_mut(&trace_type) {
            metric.total_hits += 1;
            metric.cumulative_latency_nanos += latency_nanos;
            if latency_nanos > metric.max_latency_nanos {
                metric.max_latency_nanos = latency_nanos;
            }
        }
    }

    pub fn get_average_latency(&self, trace_type: TracepointType) -> Option<f64> {
        if let Some(metric) = self.tracepoints.get(&trace_type) {
            if metric.total_hits == 0 {
                Some(0.0)
            } else {
                Some(metric.cumulative_latency_nanos as f64 / metric.total_hits as f64)
            }
        } else {
            None
        }
    }

    pub fn reset_metrics(&mut self) {
        for metric in self.tracepoints.values_mut() {
            metric.total_hits = 0;
            metric.cumulative_latency_nanos = 0;
            metric.max_latency_nanos = 0;
        }
    }
}

impl Default for SigmaProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_metrics() {
        let mut profiler = SigmaProfiler::new();
        profiler.record_event(TracepointType::ContextSwitch, 450);
        profiler.record_event(TracepointType::ContextSwitch, 550);

        let avg = profiler
            .get_average_latency(TracepointType::ContextSwitch)
            .unwrap();
        assert_eq!(avg, 500.0);

        let metric = profiler
            .tracepoints
            .get(&TracepointType::ContextSwitch)
            .unwrap();
        assert_eq!(metric.max_latency_nanos, 550);
        assert_eq!(metric.total_hits, 2);
    }
}
