pub mod profiler;
pub mod mglru;
pub mod io_uring;
pub mod io_scheduler;
pub mod network_bbr;
pub mod eevdf;
pub mod zero_copy_ipc;

pub use profiler::{Profiler, SimpleProfiler, Profile, SimpleProfile, ProfileType, ProfilerError, CallGraph, SimpleCallGraph};
pub use mglru::{MultiGenLRU, PageInfo, PageState, MAX_GENERATIONS, MAX_PAGES_TRACKED};
pub use io_uring::{IoUring, SubmissionQueueEntry, CompletionQueueEntry, IoOpcode, SQ_RING_SIZE, CQ_RING_SIZE};
pub use io_scheduler::{AdaptiveIOScheduler, DeviceType, IOSchedulerPolicy, IORequest};
pub use network_bbr::{BbrEngine, BbrState};
pub use eevdf::{EevdfScheduler, EevdfTask, MAX_SCHED_TASKS};
pub use zero_copy_ipc::{ZeroCopyQueue, IPCError, QUEUE_SIZE};
||||||| 43be3a7e8
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
||||||| 984d1301f
pub mod mglru;
pub mod io_uring;
pub mod io_scheduler;
pub mod network_bbr;
pub mod eevdf;
pub mod zero_copy_ipc;

pub use profiler::{Profiler, SimpleProfiler, Profile, SimpleProfile, ProfileType, ProfilerError, CallGraph, SimpleCallGraph};
pub use mglru::{MultiGenLRU, PageInfo, PageState, MAX_GENERATIONS, MAX_PAGES_TRACKED};
pub use io_uring::{IoUring, SubmissionQueueEntry, CompletionQueueEntry, IoOpcode, SQ_RING_SIZE, CQ_RING_SIZE};
pub use io_scheduler::{AdaptiveIOScheduler, DeviceType, IOSchedulerPolicy, IORequest};
pub use network_bbr::{BbrEngine, BbrState};
pub use eevdf::{EevdfScheduler, EevdfTask, MAX_SCHED_TASKS};
pub use zero_copy_ipc::{ZeroCopyQueue, IPCError, QUEUE_SIZE};
