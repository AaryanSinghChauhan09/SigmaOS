// SigmaOS Observability Module
pub mod stack;
pub mod cognitive_narrative;
pub mod profiler;

pub use profiler::{SigmaProfiler, TracepointType, PerformanceMetric};

pub use cognitive_narrative::{CognitiveOSNarrator, AdaptiveComplianceGater, SynestheticFeedbackEngine, GenerativeConfigParser, InterplanetaryDtnRoute, CollectiveSimulationNode};
pub use stack::{
    Metric, MetricCapability, MetricID, MetricInfo, MetricType, ObservabilityError,
    ObservabilityStack, ObservabilityStats, SigmaDebug, SigmaMetrics, SigmaTrace, SimpleMetric,
    SimpleObservabilityStack, SimpleSigmaDebug, SimpleSigmaMetrics, SimpleSigmaTrace, SimpleSpan,
    Span, SpanCapability, SpanInfo, StackCapability, TraceID,
};
