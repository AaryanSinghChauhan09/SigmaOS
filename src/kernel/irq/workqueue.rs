/// SigmaOS Workqueue async deferred execution engine
/// Runs deferred kernel tasks in thread context
use crate::klib::VecDeque;

pub struct Work {
    pub name: String,
    pub func: fn(),
}

pub struct Workqueue {
    pub name: String,
    queue: VecDeque<Work>,
}

impl Workqueue {
    pub fn new(name: &str) -> Self {
        Workqueue {
            name: name.to_string(),
            queue: VecDeque::new(),
        }
    }

    pub fn queue_work(&mut self, work: Work) {
        self.queue.push_back(work);
    }

    pub fn process_work(&mut self) -> usize {
        let mut count = 0;
        while let Some(work) = self.queue.pop_front() {
            (work.func)();
            count += 1;
        }
        count
    }
}
