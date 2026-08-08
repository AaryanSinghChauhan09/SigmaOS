#![no_std]

use core::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct TaskId(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: TaskId,
    pub vruntime: u64,
    pub priority: u32,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.vruntime == other.vruntime
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.vruntime.cmp(&other.vruntime)
    }
}

pub struct CfsScheduler {
    tasks: [Option<Task>; 64],
    task_count: usize,
    current_time: u64,
}

impl CfsScheduler {
    pub const fn new() -> Self {
        CfsScheduler {
            tasks: [None; 64],
            task_count: 0,
            current_time: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        if self.task_count < 64 {
            self.tasks[self.task_count] = Some(task);
            self.task_count += 1;
            self.sort_tasks();
        }
    }

    pub fn pick_next_task(&mut self) -> Option<Task> {
        if self.task_count > 0 {
            let task = self.tasks[0].take();
            self.tasks[0] = self.tasks[self.task_count - 1];
            self.tasks[self.task_count - 1] = None;
            self.task_count -= 1;
            self.sort_tasks();
            task
        } else {
            None
        }
    }

    pub fn tick(&mut self) {
        self.current_time += 1;
    }

    fn sort_tasks(&mut self) {
        // Simple insertion sort for now since we don't have BTreeMap in no_std
        for i in 1..self.task_count {
            let mut j = i;
            while j > 0 && self.tasks[j - 1].unwrap().vruntime > self.tasks[j].unwrap().vruntime {
                self.tasks.swap(j - 1, j);
                j -= 1;
            }
        }
    }
}
