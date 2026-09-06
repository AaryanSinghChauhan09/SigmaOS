#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Scheduler Module
// eBPF-based scheduling system & CPU affinity manager

pub mod affinity;
pub mod distro_schedulers;
pub mod ebpf_scheduler;
pub mod eevdf;
pub mod energy_aware;
pub mod numa_scheduler;
pub mod process;
pub mod scheduler;
pub mod sovereign;

pub use distro_schedulers::*;
pub use ebpf_scheduler::{
    BpfMapDescriptor, BpfMapType, BpfProgram, SchedError, SchedExtScheduler, ScheduleDecision,
    SchedulerConfig, SchedulerStats, SchedulingPolicy, UserSpaceScheduler,
};

pub use affinity::{CpuAffinityMask, NumaDomainTopology, ProcessCpuAssigner};
