// SigmaOS Performance Module
pub mod profiler;
pub mod smart_optimizer;
pub mod cachy_opt;

pub use cachy_opt::{
    BoreScheduler, AnanicyRule, IoSchedClass, AnanicyCppDaemon, PhysicalPageFrame,
    UltraKernelSamepageMerger, X86v3v4OptimizationDetector, CachyKernelManager,
};

pub use profiler::{
    CallGraph, Profile, ProfileType, Profiler, ProfilerError, SimpleCallGraph, SimpleProfile,
    SimpleProfiler,
};

pub use smart_optimizer::{
    CpuPriorityOptimizer, GlarySmartRule, IoPriorityOptimizer, IoTaskPriority,
    PerformanceProfileRule, RamDefragmenter, SmartPerformanceProfile, SmartResourceOptimizer,
    GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER,
};
