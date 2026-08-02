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

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::kernel::sched::task::{ProcessState, SchedPolicy, Task, PID_MAX_LIMIT};
use crate::kernel::vfs::inode::FsError;

pub struct RunQueue {
    pub cfs_rq: CfsRunQueue,
    pub rt_rq: RtRunQueue,
    pub dl_rq: DeadlineRunQueue,
    pub stop_rq: StopRunQueue,
    pub idle_rq: IdleRunQueue,
    pub nr_running: AtomicU32,
    pub cpu_id: u32,
    pub load: u64,
    pub avg_load: u64,
}

pub struct CfsRunQueue {
    pub vruntime: u64,
    pub min_vruntime: u64,
    pub nr_running: u32,
    pub rb_leftmost: Option<*const SchedEntity>,
    pub deadlines: Vec<SchedEntity>,
}

pub struct RtRunQueue {
    pub active: [Vec<SchedEntity>; 140],
    pub highest_prio: i32,
    pub nr_running: u32,
}

pub struct DeadlineRunQueue {
    pub earliest_deadline: u64,
    pub nr_running: u32,
    pub deadlines: Vec<SchedEntity>,
}

pub struct StopRunQueue {
    pub pending: bool,
    pub nr_running: u32,
}

pub struct IdleRunQueue {
    pub nr_running: u32,
}

pub struct SchedEntity {
    pub pid: u64,
    pub vruntime: u64,
    pub deadline: u64,
    pub runtime: u64,
    pub priority: i32,
    pub policy: SchedPolicy,
    pub cpu: u32,
    pub on_rq: bool,
}

pub trait SchedClass: Send + Sync {
    fn enqueue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError>;
    fn dequeue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError>;
    fn yield_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError>;
    fn check_preempt_curr(&self, rq: &mut RunQueue, task: &Task) -> bool;
    fn pick_next_task(&self, rq: &mut RunQueue) -> Option<u64>;
    fn put_prev_task(&self, rq: &mut RunQueue, task: &mut Task);
    fn set_curr_task(&self, rq: &mut RunQueue, task: &mut Task);
    fn task_tick(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError>;
    fn task_fork(&self, rq: &mut RunQueue, child: &mut Task, parent: &Task) -> Result<(), FsError>;
    fn task_dead(&self, rq: &mut RunQueue, task: &mut Task);
    fn prio_changed(&self, rq: &mut RunQueue, task: &mut Task);
}

pub struct StopSchedClass;
pub struct DeadlineSchedClass;
pub struct RealtimeSchedClass;
pub struct FairSchedClass;
pub struct IdleSchedClass;

impl SchedClass for StopSchedClass {
    fn enqueue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        rq.stop_rq.nr_running += 1;
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        rq.stop_rq.nr_running -= 1;
        Ok(())
    }
    fn yield_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        true
    }
    fn pick_next_task(&self, rq: &mut RunQueue) -> Option<u64> {
        None
    }
    fn put_prev_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn set_curr_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn task_tick(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn task_fork(
        &self,
        _rq: &mut RunQueue,
        _child: &mut Task,
        _parent: &Task,
    ) -> Result<(), FsError> {
        Ok(())
    }
    fn task_dead(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn prio_changed(&self, _rq: &mut RunQueue, _task: &mut Task) {}
}

impl SchedClass for FairSchedClass {
    fn enqueue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        rq.cfs_rq.nr_running += 1;
        rq.nr_running.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        rq.cfs_rq.nr_running -= 1;
        rq.nr_running.fetch_sub(1, Ordering::SeqCst);
        Ok(())
    }
    fn yield_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        self.dequeue_task(rq, task)?;
        self.enqueue_task(rq, task)
    }
    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        false
    }
    fn pick_next_task(&self, rq: &mut RunQueue) -> Option<u64> {
        None
    }
    fn put_prev_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn set_curr_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn task_tick(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn task_fork(
        &self,
        _rq: &mut RunQueue,
        _child: &mut Task,
        _parent: &Task,
    ) -> Result<(), FsError> {
        Ok(())
    }
    fn task_dead(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn prio_changed(&self, _rq: &mut RunQueue, _task: &mut Task) {}
}

impl SchedClass for DeadlineSchedClass {
    fn enqueue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn yield_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        false
    }
    fn pick_next_task(&self, rq: &mut RunQueue) -> Option<u64> {
        None
    }
    fn put_prev_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn set_curr_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn task_tick(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn task_fork(
        &self,
        _rq: &mut RunQueue,
        _child: &mut Task,
        _parent: &Task,
    ) -> Result<(), FsError> {
        Ok(())
    }
    fn task_dead(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn prio_changed(&self, _rq: &mut RunQueue, _task: &mut Task) {}
}

impl SchedClass for RealtimeSchedClass {
    fn enqueue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn yield_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        false
    }
    fn pick_next_task(&self, rq: &mut RunQueue) -> Option<u64> {
        None
    }
    fn put_prev_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn set_curr_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn task_tick(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn task_fork(
        &self,
        _rq: &mut RunQueue,
        _child: &mut Task,
        _parent: &Task,
    ) -> Result<(), FsError> {
        Ok(())
    }
    fn task_dead(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn prio_changed(&self, _rq: &mut RunQueue, _task: &mut Task) {}
}

impl SchedClass for IdleSchedClass {
    fn enqueue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn yield_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        false
    }
    fn pick_next_task(&self, rq: &mut RunQueue) -> Option<u64> {
        None
    }
    fn put_prev_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn set_curr_task(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn task_tick(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn task_fork(
        &self,
        _rq: &mut RunQueue,
        _child: &mut Task,
        _parent: &Task,
    ) -> Result<(), FsError> {
        Ok(())
    }
    fn task_dead(&self, _rq: &mut RunQueue, _task: &mut Task) {}
    fn prio_changed(&self, _rq: &mut RunQueue, _task: &mut Task) {}
}

pub struct Scheduler {
    pub runqueues: Vec<RunQueue>,
    pub current: Vec<u64>,
    pub sched_class: Vec<Box<dyn SchedClass>>,
}

impl Scheduler {
    pub fn new(num_cpus: u32) -> Self {
        Scheduler {
            runqueues: (0..num_cpus)
                .map(|i| RunQueue {
                    cfs_rq: CfsRunQueue {
                        vruntime: 0,
                        min_vruntime: 0,
                        nr_running: 0,
                        rb_leftmost: None,
                        deadlines: Vec::new(),
                    },
                    rt_rq: RtRunQueue {
                        active: [Vec::new(); 140],
                        highest_prio: 140,
                        nr_running: 0,
                    },
                    dl_rq: DeadlineRunQueue {
                        earliest_deadline: u64::MAX,
                        nr_running: 0,
                        deadlines: Vec::new(),
                    },
                    stop_rq: StopRunQueue {
                        pending: false,
                        nr_running: 0,
                    },
                    idle_rq: IdleRunQueue { nr_running: 0 },
                    nr_running: AtomicU32::new(0),
                    cpu_id: i,
                    load: 0,
                    avg_load: 0,
                })
                .collect(),
            current: vec![0; num_cpus as usize],
            sched_class: Vec::new(),
        }
    }

    pub fn register_class(&mut self, sched_class: Box<dyn SchedClass>) {
        self.sched_class.push(sched_class);
    }

    pub fn fork(&self, parent: &Task) -> Task {
        let mut child = Task::new(0, &parent.name);
        child.parent_pid = parent.pid;
        child.real_parent = parent.pid;
        child.tgid = parent.tgid;
        child.cred = parent.cred.clone();
        child.policy = parent.policy;
        child.static_prio = parent.static_prio;
        child.normal_prio = parent.normal_prio;
        child
    }

    pub fn schedule(&mut self) -> Option<u64> {
        None
    }
}
