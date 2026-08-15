#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

pub mod scheduler;
pub mod task;
pub mod sigma_mlfq;
pub mod sigma_transformer_sched;
pub mod sigma_thermal_sched;
pub mod gaming_performance;

pub use gaming_performance::{
    DragonFlySmpQueueManager, LwktMessage, PowerGovernor, SovereignGameMode, UksmPageDeduplicator,
};
pub use scheduler::{
    DeadlineSchedClass, FairSchedClass, IdleSchedClass, RealtimeSchedClass, RunQueue, SchedClass,
    Scheduler, StopSchedClass,
};
pub use task::{Cred, ProcessState, SchedPolicy, Task, INIT_PID, PID_MAX_LIMIT};
pub use sigma_mlfq::{MlfqSchedClass, MlfqScheduler};
pub use sigma_transformer_sched::{TransformerSchedClass, TransformerScheduler};
pub use sigma_thermal_sched::{SchedulingDecision, ThermalSchedClass, ThermalScheduler};
