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
