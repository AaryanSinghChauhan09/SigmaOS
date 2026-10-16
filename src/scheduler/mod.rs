// SigmaOS Scheduler Subsystem
// Bounded multi-level priority queue scheduler, eBPF-based scheduling & CPU affinity manager

pub mod affinity;
pub mod core;
pub mod cpuidle;
pub mod distro_schedulers;
pub mod ebpf_scheduler;
pub mod eevdf;
pub mod energy_aware;
pub mod numa_balancer;
pub mod numa_scheduler;
pub mod preempt_rt;
pub mod process;
pub mod run_queue;
pub mod sched_deadline;
pub mod scheduler;
pub mod schedutil;
pub mod sovereign;

pub use core::{
    ProcessState, SchedulerCore, SchedulerError, SchedulerProcessControlBlock,
    DEFAULT_STARVATION_THRESHOLD_TICKS, MAX_PROCESS_TABLE_SIZE,
};
pub use run_queue::{
    BoundedRunQueue, RunQueueError, TaskRunRef, MAX_RUNQUEUE_CAPACITY, NUM_PRIORITY_LEVELS,
};

pub use cpuidle::{CState, CpuIdleGovernor};
pub use numa_balancer::{NumaLoadBalancer, NumaNodeStats, NumaTask};
pub use preempt_rt::{PreemptGuard, RtMutex, RtWaiter};
pub use sched_deadline::{DeadlineScheduler, DeadlineTask};
pub use schedutil::{CpuFreqPolicy, FreqGovernorMode, SchedutilGovernor};

pub use distro_schedulers::*;
pub use ebpf_scheduler::{
    BpfMapDescriptor, BpfMapType, BpfProgram, SchedError, SchedExtScheduler, ScheduleDecision,
    SchedulerConfig, SchedulerStats, SchedulingPolicy, UserSpaceScheduler,
};

pub use affinity::{CpuAffinityMask, NumaDomainTopology, ProcessCpuAssigner};

// SchedExt: Linux 6.11+ Extensible Scheduler Framework
pub mod schedext;
pub use schedext::{
    SchedExtCpuRq, SchedExtDomain, SchedExtFlags, SchedExtLavd, SchedExtManager, SchedExtRusty,
    SchedExtStats, SchedExtTaskCtx,
};
