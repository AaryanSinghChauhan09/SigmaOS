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

// SigmaOS Scheduler Module
// EEVDF scheduler, S-INIT supervisor, and process scheduling

pub mod eevdf;
pub mod process;
pub mod scheduler;
pub mod sovereign;
pub mod numa_scheduler;

pub use eevdf::{
    ComputeUnit, EevdfScheduler, SInitSupervisor, Service, ServiceState, Task, TaskState,
};
pub use process::{Process, ProcessCapability, ProcessPriority, ProcessScheduler, ProcessState, SimpleProcess, SimpleProcessScheduler, SchedulerCapability, SchedulerStats, SchedulerError as ProcSchedulerError};
pub use scheduler::{Scheduler, SchedulerError};
pub use sovereign::{Priority, SimpleThread, Thread, ThreadID, ThreadState};
