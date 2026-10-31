// Comprehensive Unit Tests for Dispatcher Status, Invariants, and Plan Verification

use std::fs;

#[test]
fn test_dispatcher_documentation_plan_verification() {
    let docs_plan = fs::read_to_string("docs/DISPATCHER_PLAN.md")
        .expect("docs/DISPATCHER_PLAN.md must exist");
    let wiki_plan = fs::read_to_string("wiki/DISPATCHER_PLAN.md")
        .expect("wiki/DISPATCHER_PLAN.md must exist");

    assert!(docs_plan.contains("SigmaOS Dispatcher Master Architecture & Support Matrix Plan"));
    assert!(docs_plan.contains("Invariants and scope"));
    assert!(docs_plan.contains("Single scheduling decision path"));
    assert!(docs_plan.contains("Context integrity"));
    assert!(docs_plan.contains("Correct state ordering"));
    assert!(docs_plan.contains("Interrupt and preemption safety"));
    assert!(docs_plan.contains("Kernel/user return is a security boundary"));
    assert!(docs_plan.contains("No blocking work in hard-interrupt context"));
    assert!(docs_plan.contains("No lost wakeups or double execution"));
    assert!(docs_plan.contains("Feature claims follow test tier"));
    assert!(docs_plan.contains("Proposed PR sequence"));
    assert!(docs_plan.contains("Periodic maintenance PR cadence"));
    assert!(docs_plan.contains("Component-specific rules"));
    assert!(docs_plan.contains("PR template and merge gates"));
    assert!(docs_plan.contains("Completion criteria"));

    assert_eq!(
        docs_plan, wiki_plan,
        "wiki/DISPATCHER_PLAN.md must mirror docs/DISPATCHER_PLAN.md exactly"
    );
}

#[test]
fn test_dispatcher_plan_invariants_content() {
    let content = fs::read_to_string("docs/DISPATCHER_PLAN.md")
        .expect("Failed to read docs/DISPATCHER_PLAN.md");

    let required_invariants = [
        "Single scheduling decision path",
        "Context integrity",
        "Correct state ordering",
        "Interrupt and preemption safety",
        "Kernel/user return is a security boundary",
        "No blocking work in hard-interrupt context",
        "No lost wakeups or double execution",
        "Feature claims follow test tier",
    ];

    for invariant in &required_invariants {
        assert!(
            content.contains(invariant),
            "Missing invariant in plan: {}",
            invariant
        );
    }
}

#[test]
fn test_dispatcher_plan_pr_sequence() {
    let content = fs::read_to_string("docs/DISPATCHER_PLAN.md")
        .expect("Failed to read docs/DISPATCHER_PLAN.md");

    for pr_num in 1..=16 {
        let pr_marker = format!("| {} |", pr_num);
        assert!(
            content.contains(&pr_marker),
            "Missing PR marker: {}",
            pr_marker
        );
    }

    assert!(
        content.contains("`docs(dispatch): define scheduler-dispatcher-IRQ boundaries`"),
        "Missing PR 1 title"
    );
    assert!(
        content.contains("`feat(dispatch): connect scheduler selection to x86_64 context switch`"),
        "Missing PR 4 title"
    );
    assert!(
        content.contains("`feat(dispatch): add run-queue locking and SMP switch protocol`"),
        "Missing PR 9 title"
    );
    assert!(
        content.contains("`docs(dispatch): publish support matrix and recovery guide`"),
        "Missing PR 16 title"
    );
    assert!(
        content.contains("Prioritize PRs 1–8"),
        "Missing priority recommendation for PRs 1-8"
    );
}

#[test]
fn test_dispatcher_plan_cadence_and_rules() {
    let content = fs::read_to_string("docs/DISPATCHER_PLAN.md")
        .expect("Failed to read docs/DISPATCHER_PLAN.md");

    let cadences = [
        "Every PR",
        "Weekly",
        "Monthly",
        "Quarterly",
        "Each release candidate",
        "After an incident",
    ];

    for cadence in &cadences {
        assert!(
            content.contains(cadence),
            "Missing maintenance cadence: {}",
            cadence
        );
    }

    let completion_criteria = ["Prototype:", "Integrated:", "Supported:"];

    for criteria in &completion_criteria {
        assert!(
            content.contains(criteria),
            "Missing completion criteria: {}",
            criteria
        );
    }
}

#[test]
fn test_feature_status_toml_dispatcher_registration() {
    let content = fs::read_to_string("FEATURE_STATUS.toml")
        .expect("Failed to read FEATURE_STATUS.toml");

    assert!(
        content.contains("[kernel.dispatcher]"),
        "FEATURE_STATUS.toml must contain [kernel.dispatcher]"
    );
    assert!(
        content.contains("docs/DISPATCHER_PLAN.md"),
        "FEATURE_STATUS.toml must reference docs/DISPATCHER_PLAN.md"
    );
    assert!(
        content.contains("wiki/DISPATCHER_PLAN.md"),
        "FEATURE_STATUS.toml must reference wiki/DISPATCHER_PLAN.md"
    );
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
