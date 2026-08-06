// SigmaOS Performance Module
pub mod profiler;
pub mod smart_optimizer;

pub use profiler::{
    CallGraph, Profile, ProfileType, Profiler, ProfilerError, SimpleCallGraph, SimpleProfile,
    SimpleProfiler,
};

pub use smart_optimizer::{
    CpuPriorityOptimizer, GlarySmartRule, IoPriorityOptimizer, IoTaskPriority,
    PerformanceProfileRule, RamDefragmenter, SmartPerformanceProfile, SmartResourceOptimizer,
    GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER,
};
