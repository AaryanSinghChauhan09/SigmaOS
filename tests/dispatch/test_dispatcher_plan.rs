// Comprehensive Unit Tests for Dispatcher Status, Invariants, and Plan Verification

use std::fs;
use std::path::Path;

#[test]
fn test_dispatcher_documentation_plan_verification() {
    let docs_path = Path::new("docs/DISPATCHER_PLAN.md");
    let wiki_path = Path::new("wiki/DISPATCHER_PLAN.md");

    assert!(docs_path.exists(), "docs/DISPATCHER_PLAN.md must exist");
    assert!(wiki_path.exists(), "wiki/DISPATCHER_PLAN.md must exist");

    let docs_plan = fs::read_to_string(docs_path)
        .expect("docs/DISPATCHER_PLAN.md must be readable");
    let wiki_plan = fs::read_to_string(wiki_path)
        .expect("wiki/DISPATCHER_PLAN.md must be readable");

    // Exact mirror assertion
    assert_eq!(
        docs_plan, wiki_plan,
        "wiki/DISPATCHER_PLAN.md must mirror docs/DISPATCHER_PLAN.md exactly"
    );

    // Section headers & conceptual core
    assert!(docs_plan.contains("# SigmaOS Dispatcher Master Architecture & Support Matrix Plan"));
    assert!(docs_plan.contains("## Invariants and scope"));
    assert!(docs_plan.contains("## Proposed PR sequence"));
    assert!(docs_plan.contains("## Periodic maintenance PR cadence"));
    assert!(docs_plan.contains("## Component-specific rules"));
    assert!(docs_plan.contains("## PR template and merge gates"));
    assert!(docs_plan.contains("## Completion criteria"));

    // Definition & boundary checks
    assert!(docs_plan.contains("dispatcher"));
    assert!(docs_plan.contains("scheduler"));
    assert!(docs_plan.contains("interrupt controller"));
    assert!(docs_plan.contains("context save/restore"));
    assert!(docs_plan.contains("run-state transitions"));
    assert!(docs_plan.contains("deferred kernel work dispatch"));

    // Invariants
    assert!(docs_plan.contains("Single scheduling decision path"));
    assert!(docs_plan.contains("Context integrity"));
    assert!(docs_plan.contains("Correct state ordering"));
    assert!(docs_plan.contains("Interrupt and preemption safety"));
    assert!(docs_plan.contains("Kernel/user return is a security boundary"));
    assert!(docs_plan.contains("No blocking work in hard-interrupt context"));
    assert!(docs_plan.contains("No lost wakeups or double execution"));
    assert!(docs_plan.contains("Feature claims follow test tier"));

    // Prioritized sequence and first milestone
    assert!(docs_plan.contains("**Prioritize PRs 1–8** first"));
    assert!(docs_plan.contains("boot → idle → timer → scheduler chooses a task → dispatcher switches → task blocks or exits → another task runs"));

    // Completion criteria tiers
    assert!(docs_plan.contains("**Prototype:**"));
    assert!(docs_plan.contains("**Integrated:**"));
    assert!(docs_plan.contains("**Supported:**"));
}

#[test]
fn test_dispatcher_pr_sequence_roadmap() {
    let docs_plan = fs::read_to_string("docs/DISPATCHER_PLAN.md")
        .expect("docs/DISPATCHER_PLAN.md must exist");

    let expected_prs = [
        (1, "`docs(dispatch): define scheduler-dispatcher-IRQ boundaries`"),
        (2, "`test(dispatch): add context-frame layout and invariant tests`"),
        (3, "`refactor(dispatch): define architecture context-switch interface`"),
        (4, "`feat(dispatch): connect scheduler selection to x86_64 context switch`"),
        (5, "`feat(dispatch): add safe first-task and idle-task handoff`"),
        (6, "`feat(dispatch): connect timer interrupt return to reschedule path`"),
        (7, "`feat(dispatch): connect syscall and exception return-to-user hooks`"),
        (8, "`feat(dispatch): add wakeup and yield handoff semantics`"),
        (9, "`feat(dispatch): add run-queue locking and SMP switch protocol`"),
        (10, "`feat(dispatch): implement interrupt controller flow interface`"),
        (11, "`feat(dispatch): add bounded deferred-work executor`"),
        (12, "`feat(dispatch): define deferred-work priority and forward-progress rules`"),
        (13, "`feat(dispatch): add dispatcher tracing and counters`"),
        (14, "`test(dispatch): add timer, interrupt, and workqueue fault injection`"),
        (15, "`test(dispatch): add QEMU end-to-end and architecture matrix`"),
        (16, "`docs(dispatch): publish support matrix and recovery guide`"),
    ];

    for (pr_num, title) in &expected_prs {
        let row_marker = format!("| {} | {}", pr_num, title);
        assert!(
            docs_plan.contains(&row_marker),
            "docs/DISPATCHER_PLAN.md must contain PR {} with title {}",
            pr_num,
            title
        );
    }
}

#[test]
fn test_dispatcher_cadences_and_component_rules() {
    let docs_plan = fs::read_to_string("docs/DISPATCHER_PLAN.md")
        .expect("docs/DISPATCHER_PLAN.md must exist");

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
            docs_plan.contains(cadence),
            "docs/DISPATCHER_PLAN.md must contain maintenance cadence: {}",
            cadence
        );
    }

    let component_rules = [
        "Scheduler interface:",
        "Context switch:",
        "Interrupt entry/exit:",
        "Deferred work:",
        "Task lifecycle:",
        "CPU topology/SMP:",
        "Observability:",
        "Feature inventory:",
    ];

    for rule in &component_rules {
        assert!(
            docs_plan.contains(rule),
            "docs/DISPATCHER_PLAN.md must contain component rule: {}",
            rule
        );
    }

    // Merge gate rules
    assert!(docs_plan.contains("Require one maintainer approval for tests/docs and two qualified reviews"));
}

#[test]
fn test_feature_status_toml_dispatcher_registration() {
    let feature_status = fs::read_to_string("FEATURE_STATUS.toml")
        .expect("FEATURE_STATUS.toml must exist");

    assert!(
        feature_status.contains("[kernel.dispatcher]"),
        "FEATURE_STATUS.toml must contain [kernel.dispatcher]"
    );
    assert!(
        feature_status.contains("docs/DISPATCHER_PLAN.md"),
        "FEATURE_STATUS.toml must reference docs/DISPATCHER_PLAN.md"
    );
    assert!(
        feature_status.contains("wiki/DISPATCHER_PLAN.md"),
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
            self.items.push(DeferredWorkItem {
                id,
                can_block: true,
            });
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
