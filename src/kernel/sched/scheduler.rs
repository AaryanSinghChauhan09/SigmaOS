// Completely Fair Scheduler (CFS), Real-time (RT), and Deadline CPU scheduler
// High-fidelity multi-class task scheduling inspired by standard Linux kernels

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
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

#[derive(Debug, Clone, Copy)]
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
    fn enqueue_task(&self, rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        rq.stop_rq.nr_running += 1;
        rq.nr_running.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        if rq.stop_rq.nr_running > 0 {
            rq.stop_rq.nr_running -= 1;
            rq.nr_running.fetch_sub(1, Ordering::SeqCst);
        }
        Ok(())
    }
    fn yield_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        true
    }
    fn pick_next_task(&self, rq: &mut RunQueue) -> Option<u64> {
        if rq.stop_rq.pending {
            Some(9999) // mock STOP task PID
        } else {
            None
        }
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
        let entity = SchedEntity {
            pid: task.pid,
            vruntime: task.utime,
            deadline: 0,
            runtime: task.stime,
            priority: task.priority,
            policy: task.policy,
            cpu: rq.cpu_id,
            on_rq: true,
        };
        rq.cfs_rq.deadlines.push(entity);
        rq.cfs_rq.nr_running += 1;
        rq.nr_running.fetch_add(1, Ordering::SeqCst);
        task.state = ProcessState::Runnable;
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        let initial_len = rq.cfs_rq.deadlines.len();
        rq.cfs_rq.deadlines.retain(|entity| entity.pid != task.pid);
        let removed = initial_len - rq.cfs_rq.deadlines.len();

        if removed > 0 {
            rq.cfs_rq.nr_running -= removed as u32;
            rq.nr_running.fetch_sub(removed as u32, Ordering::SeqCst);
        }
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
        // CFS selection: pick task with minimum virtual runtime (vruntime)
        if rq.cfs_rq.deadlines.is_empty() {
            return None;
        }
        let mut min_idx = 0;
        let mut min_vruntime = rq.cfs_rq.deadlines[0].vruntime;

        for (i, entity) in rq.cfs_rq.deadlines.iter().enumerate() {
            if entity.vruntime < min_vruntime {
                min_vruntime = entity.vruntime;
                min_idx = i;
            }
        }

        let min_entity = &rq.cfs_rq.deadlines[min_idx];
        rq.cfs_rq.min_vruntime = min_entity.vruntime;
        Some(min_entity.pid)
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
        // DL scheduling parameters
        let deadline = if task.static_prio > 0 { task.static_prio as u64 } else { 1000 };
        let entity = SchedEntity {
            pid: task.pid,
            vruntime: 0,
            deadline,
            runtime: task.stime,
            priority: task.priority,
            policy: task.policy,
            cpu: rq.cpu_id,
            on_rq: true,
        };
        rq.dl_rq.deadlines.push(entity);
        rq.dl_rq.nr_running += 1;
        rq.nr_running.fetch_add(1, Ordering::SeqCst);
        task.state = ProcessState::Runnable;
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        let initial_len = rq.dl_rq.deadlines.len();
        rq.dl_rq.deadlines.retain(|entity| entity.pid != task.pid);
        let removed = initial_len - rq.dl_rq.deadlines.len();

        if removed > 0 {
            rq.dl_rq.nr_running -= removed as u32;
            rq.nr_running.fetch_sub(removed as u32, Ordering::SeqCst);
        }
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
        // Earliest Deadline First (EDF) selection
        if rq.dl_rq.deadlines.is_empty() {
            return None;
        }
        let mut earliest_idx = 0;
        let mut min_deadline = rq.dl_rq.deadlines[0].deadline;

        for (i, entity) in rq.dl_rq.deadlines.iter().enumerate() {
            if entity.deadline < min_deadline {
                min_deadline = entity.deadline;
                earliest_idx = i;
            }
        }

        let earliest_entity = &rq.dl_rq.deadlines[earliest_idx];
        rq.dl_rq.earliest_deadline = earliest_entity.deadline;
        Some(earliest_entity.pid)
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
        let prio = task.priority.clamp(0, 139) as usize;
        let entity = SchedEntity {
            pid: task.pid,
            vruntime: 0,
            deadline: 0,
            runtime: task.stime,
            priority: task.priority,
            policy: task.policy,
            cpu: rq.cpu_id,
            on_rq: true,
        };
        rq.rt_rq.active[prio].push(entity);
        rq.rt_rq.nr_running += 1;
        rq.nr_running.fetch_add(1, Ordering::SeqCst);
        task.state = ProcessState::Runnable;
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, task: &mut Task) -> Result<(), FsError> {
        let prio = task.priority.clamp(0, 139) as usize;
        let initial_len = rq.rt_rq.active[prio].len();
        rq.rt_rq.active[prio].retain(|entity| entity.pid != task.pid);
        let removed = initial_len - rq.rt_rq.active[prio].len();

        if removed > 0 {
            rq.rt_rq.nr_running -= removed as u32;
            rq.nr_running.fetch_sub(removed as u32, Ordering::SeqCst);
        }
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
        // Priority Array scanning (0 is highest RT priority)
        for i in 0..140 {
            if !rq.rt_rq.active[i].is_empty() {
                return Some(rq.rt_rq.active[i][0].pid);
            }
        }
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
    fn enqueue_task(&self, rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        rq.idle_rq.nr_running += 1;
        rq.nr_running.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn dequeue_task(&self, rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        if rq.idle_rq.nr_running > 0 {
            rq.idle_rq.nr_running -= 1;
            rq.nr_running.fetch_sub(1, Ordering::SeqCst);
        }
        Ok(())
    }
    fn yield_task(&self, _rq: &mut RunQueue, _task: &mut Task) -> Result<(), FsError> {
        Ok(())
    }
    fn check_preempt_curr(&self, _rq: &mut RunQueue, _task: &Task) -> bool {
        false
    }
    fn pick_next_task(&self, _rq: &mut RunQueue) -> Option<u64> {
        Some(0) // IDLE task PID is always 0
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
        let mut s = Scheduler {
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
                        active: core::array::from_fn(|_| Vec::new()),
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
        };

        // Seed classes sequentially by scheduling priorities (Stop > DL > RT > Fair > Idle)
        s.register_class(Box::new(StopSchedClass));
        s.register_class(Box::new(DeadlineSchedClass));
        s.register_class(Box::new(RealtimeSchedClass));
        s.register_class(Box::new(FairSchedClass));
        s.register_class(Box::new(IdleSchedClass));

        s
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

    /// Perform unified core scheduling action across classes
    pub fn schedule(&mut self) -> Option<u64> {
        self.schedule_on_cpu(0)
    }

    /// Schedule next task on specific cpu_id
    pub fn schedule_on_cpu(&mut self, cpu_id: u32) -> Option<u64> {
        let cpu_idx = cpu_id as usize;
        if cpu_idx >= self.runqueues.len() {
            return None;
        }

        // Standard Linux scheduler loop querying classes sequentially by priority
        for class in &self.sched_class {
            if let Some(pid) = class.pick_next_task(&mut self.runqueues[cpu_idx]) {
                self.current[cpu_idx] = pid;
                return Some(pid);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfs_virtual_runtime_selection() {
        let mut rq = RunQueue {
            cfs_rq: CfsRunQueue {
                vruntime: 0,
                min_vruntime: 0,
                nr_running: 0,
                rb_leftmost: None,
                deadlines: Vec::new(),
            },
            rt_rq: RtRunQueue {
                active: core::array::from_fn(|_| Vec::new()),
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
            cpu_id: 0,
            load: 0,
            avg_load: 0,
        };

        let mut task_a = Task::new(100, "task_a");
        task_a.utime = 20; // higher vruntime

        let mut task_b = Task::new(200, "task_b");
        task_b.utime = 10; // lower vruntime

        let fair_class = FairSchedClass;
        fair_class.enqueue_task(&mut rq, &mut task_a).unwrap();
        fair_class.enqueue_task(&mut rq, &mut task_b).unwrap();

        // 1. Minimum vruntime should be picked first (task_b with 10)
        let next1 = fair_class.pick_next_task(&mut rq).unwrap();
        assert_eq!(next1, 200);

        // 2. Modify vruntime of task_b to be larger than task_a
        fair_class.dequeue_task(&mut rq, &mut task_b).unwrap();
        task_b.utime = 30;
        fair_class.enqueue_task(&mut rq, &mut task_b).unwrap();

        // Now task_a (with 20) is minimum
        let next2 = fair_class.pick_next_task(&mut rq).unwrap();
        assert_eq!(next2, 100);
    }

    #[test]
    fn test_rt_priority_array_scanning() {
        let mut rq = RunQueue {
            cfs_rq: CfsRunQueue {
                vruntime: 0,
                min_vruntime: 0,
                nr_running: 0,
                rb_leftmost: None,
                deadlines: Vec::new(),
            },
            rt_rq: RtRunQueue {
                active: core::array::from_fn(|_| Vec::new()),
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
            cpu_id: 0,
            load: 0,
            avg_load: 0,
        };

        let mut task_low = Task::new(300, "task_low");
        task_low.priority = 90; // lower priority

        let mut task_high = Task::new(400, "task_high");
        task_high.priority = 10; // higher priority (closer to 0)

        let rt_class = RealtimeSchedClass;
        rt_class.enqueue_task(&mut rq, &mut task_low).unwrap();
        rt_class.enqueue_task(&mut rq, &mut task_high).unwrap();

        // Pick task. Priority 10 should be selected first.
        let next = rt_class.pick_next_task(&mut rq).unwrap();
        assert_eq!(next, 400);
    }

    #[test]
    fn test_deadline_earliest_deadline_first() {
        let mut rq = RunQueue {
            cfs_rq: CfsRunQueue {
                vruntime: 0,
                min_vruntime: 0,
                nr_running: 0,
                rb_leftmost: None,
                deadlines: Vec::new(),
            },
            rt_rq: RtRunQueue {
                active: core::array::from_fn(|_| Vec::new()),
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
            cpu_id: 0,
            load: 0,
            avg_load: 0,
        };

        let mut task_late = Task::new(600, "task_late");
        task_late.static_prio = 2000; // deadline = 2000

        let mut task_early = Task::new(500, "task_early");
        task_early.static_prio = 500; // deadline = 500

        let dl_class = DeadlineSchedClass;
        dl_class.enqueue_task(&mut rq, &mut task_late).unwrap();
        dl_class.enqueue_task(&mut rq, &mut task_early).unwrap();

        // Earliest deadline should be selected first (task_early)
        let next = dl_class.pick_next_task(&mut rq).unwrap();
        assert_eq!(next, 500);
    }

    #[test]
    fn test_multiclass_scheduler_cascade() {
        let mut scheduler = Scheduler::new(1);

        let mut cfs_task = Task::new(100, "cfs_task");
        cfs_task.utime = 10;
        cfs_task.policy = SchedPolicy::Normal;

        let mut rt_task = Task::new(400, "rt_task");
        rt_task.priority = 10;
        rt_task.policy = SchedPolicy::RoundRobin;

        // Enqueue on their respective runqueues
        FairSchedClass.enqueue_task(&mut scheduler.runqueues[0], &mut cfs_task).unwrap();
        RealtimeSchedClass.enqueue_task(&mut scheduler.runqueues[0], &mut rt_task).unwrap();

        // Run unified schedule. Real-time class should take precedence over CFS.
        let scheduled_pid = scheduler.schedule().unwrap();
        assert_eq!(scheduled_pid, 400); // RT task picked first
    }
}
