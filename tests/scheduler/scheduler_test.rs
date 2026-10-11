#[derive(Debug, Clone)]
pub struct Task {
    pub pid: u32,
    pub priority: i32,
    pub vruntime: u64,
}

pub struct CfsScheduler {
    tasks: Vec<Task>,
}

impl CfsScheduler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn pick_next_task(&mut self) -> Option<Task> {
        if self.tasks.is_empty() {
            return None;
        }
        self.tasks.sort_by_key(|t| t.vruntime);
        Some(self.tasks[0].clone())
    }

    pub fn update_vruntime(&mut self, pid: u32, delta_exec: u64) {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.pid == pid) {
            t.vruntime += delta_exec;
        }
    }
}

#[test]
fn test_scheduler_fairness_and_preemption() {
    let mut sched = CfsScheduler::new();

    sched.add_task(Task { pid: 1, priority: 0, vruntime: 100 });
    sched.add_task(Task { pid: 2, priority: 0, vruntime: 50 });

    let next = sched.pick_next_task().expect("should pick lowest vruntime task");
    assert_eq!(next.pid, 2);

    sched.update_vruntime(2, 100);
    let next_after_update = sched.pick_next_task().unwrap();
    assert_eq!(next_after_update.pid, 1);
}
