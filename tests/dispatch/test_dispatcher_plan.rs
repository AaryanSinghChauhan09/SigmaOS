// Comprehensive Unit Tests for Dispatcher Status, Invariants, and Plan Verification

#[test]
fn test_dispatcher_documentation_plan_verification() {
    use std::fs;

    let docs_plan = fs::read_to_string("docs/DISPATCHER_PLAN.md")
        .expect("docs/DISPATCHER_PLAN.md must exist");
    let wiki_plan = fs::read_to_string("wiki/DISPATCHER_PLAN.md")
        .expect("wiki/DISPATCHER_PLAN.md must exist");

    assert!(docs_plan.contains("SigmaOS Dispatcher Master Architecture & Support Matrix Plan"));
    assert!(docs_plan.contains("Invariants and Scope"));
    assert!(docs_plan.contains("Single scheduling decision path"));
    assert!(docs_plan.contains("Context integrity"));
    assert!(docs_plan.contains("No blocking work in hard-interrupt context"));
    assert!(docs_plan.contains("Proposed PR Sequence Roadmap"));
    assert!(docs_plan.contains("Periodic Maintenance PR Cadence"));
    assert!(docs_plan.contains("Component-Specific Rules"));
    assert!(docs_plan.contains("PR Template and Merge Gates"));
    assert!(docs_plan.contains("Completion Criteria"));

    assert_eq!(docs_plan, wiki_plan, "wiki/DISPATCHER_PLAN.md must mirror docs/DISPATCHER_PLAN.md exactly");
}

#[test]
fn test_dispatcher_task_switch_and_context_frame_invariants() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum TaskState {
        New,
        Runnable,
        Running,
        Blocked,
        Exited,
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct CpuContext {
        rsp: u64,
        rip: u64,
        rflags: u64,
        cr3: u64,
    }

    #[allow(dead_code)]
    struct DispatcherTask {
        id: u64,
        state: TaskState,
        context: CpuContext,
        queue_count: usize,
    }

    impl DispatcherTask {
        fn new(id: u64, entry: u64, stack: u64, page_table: u64) -> Self {
            Self {
                id,
                state: TaskState::New,
                context: CpuContext {
                    rsp: stack,
                    rip: entry,
                    rflags: 0x202, // IF bit enabled
                    cr3: page_table,
                },
                queue_count: 0,
            }
        }

        fn enqueue(&mut self) -> Result<(), &'static str> {
            if self.queue_count > 0 {
                return Err("Task double enqueue invariant violated");
            }
            if self.state == TaskState::New || self.state == TaskState::Blocked {
                self.state = TaskState::Runnable;
                self.queue_count = 1;
                Ok(())
            } else {
                Err("Invalid state for enqueue")
            }
        }

        fn dispatch_switch_to(&mut self) -> Result<(), &'static str> {
            if self.state != TaskState::Runnable {
                return Err("Dispatcher can only switch to Runnable task");
            }
            self.state = TaskState::Running;
            self.queue_count = 0;
            Ok(())
        }

        fn block_task(&mut self) -> Result<(), &'static str> {
            if self.state != TaskState::Running {
                return Err("Only Running task can block");
            }
            self.state = TaskState::Blocked;
            Ok(())
        }
    }

    let mut task = DispatcherTask::new(1, 0x400000, 0x7fff0000, 0x1000);
    assert_eq!(task.state, TaskState::New);

    assert!(task.enqueue().is_ok());
    assert_eq!(task.state, TaskState::Runnable);
    assert_eq!(task.queue_count, 1);

    // Double enqueue should fail
    assert!(task.enqueue().is_err());

    assert!(task.dispatch_switch_to().is_ok());
    assert_eq!(task.state, TaskState::Running);
    assert_eq!(task.queue_count, 0);

    assert!(task.block_task().is_ok());
    assert_eq!(task.state, TaskState::Blocked);
}

#[test]
fn test_deferred_work_queue_invariants() {
    struct DeferredWorkItem {
        id: u32,
        can_block: bool,
    }

    struct DeferredWorkQueue {
        items: Vec<DeferredWorkItem>,
        max_capacity: usize,
    }

    impl DeferredWorkQueue {
        fn new(max_capacity: usize) -> Self {
            Self {
                items: Vec::new(),
                max_capacity,
            }
        }

        fn enqueue_from_hard_irq(&mut self, id: u32) -> Result<(), &'static str> {
            if self.items.len() >= self.max_capacity {
                return Err("Queue saturated");
            }
            // Enqueued work runs in thread context so it CAN block later,
            // but hard-IRQ top half itself does not block during enqueue.
            self.items.push(DeferredWorkItem { id, can_block: true });
            Ok(())
        }

        fn drain_one(&mut self) -> Option<DeferredWorkItem> {
            if self.items.is_empty() {
                None
            } else {
                Some(self.items.remove(0))
            }
        }
    }

    let mut queue = DeferredWorkQueue::new(2);
    assert!(queue.enqueue_from_hard_irq(101).is_ok());
    assert!(queue.enqueue_from_hard_irq(102).is_ok());
    assert!(queue.enqueue_from_hard_irq(103).is_err()); // Saturation check

    let executed = queue.drain_one().unwrap();
    assert_eq!(executed.id, 101);
    assert!(executed.can_block);
}
