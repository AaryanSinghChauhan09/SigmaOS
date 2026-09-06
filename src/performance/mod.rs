#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
pub mod cachy_opt;
pub mod eevdf;
pub mod io_scheduler;
pub mod io_uring;
pub mod mglru;
pub mod network_bbr;
pub mod profiler;
pub mod smart_optimizer;
pub mod zero_copy_ipc;

pub use eevdf::{EevdfScheduler, EevdfTask, MAX_SCHED_TASKS};
pub use io_scheduler::{AdaptiveIOScheduler, DeviceType, IORequest, IOSchedulerPolicy};
pub use io_uring::{
    CompletionQueueEntry, IoOpcode, IoUring, SubmissionQueueEntry, CQ_RING_SIZE, SQ_RING_SIZE,
};
pub use mglru::{MultiGenLRU, PageInfo, PageState, MAX_GENERATIONS, MAX_PAGES_TRACKED};
pub use network_bbr::{BbrEngine, BbrState};
pub use profiler::{
    CallGraph, Profile, ProfileType, Profiler, ProfilerError, SimpleCallGraph, SimpleProfile,
    SimpleProfiler,
};
pub use zero_copy_ipc::{IPCError, ZeroCopyQueue, QUEUE_SIZE};
pub mod tuned;

pub use cachy_opt::{
    AnanicyCppDaemon, AnanicyRule, BoreScheduler, CachyKernelManager, IoSchedClass,
    PhysicalPageFrame, UltraKernelSamepageMerger, X86v3v4OptimizationDetector,
};
pub use smart_optimizer::{
    CpuPriorityOptimizer, GlarySmartRule, IoPriorityOptimizer, IoTaskPriority,
    PerformanceProfileRule, RamDefragmenter, SmartPerformanceProfile, SmartResourceOptimizer,
    GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER,
};

pub use tuned::{BootStageMetrics, PerformanceTuner, TuningProfileKind};
