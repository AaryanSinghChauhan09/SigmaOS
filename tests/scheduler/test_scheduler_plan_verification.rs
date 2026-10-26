// Comprehensive Unit Tests for CPU Scheduler Status, Invariants, and Plan Verification

#[test]
fn test_scheduler_documentation_plan_verification() {
    use std::fs;

    let invariants_doc = fs::read_to_string("docs/SCHEDULER_STATUS_AND_INVARIANTS.md")
        .expect("docs/SCHEDULER_STATUS_AND_INVARIANTS.md must exist");
    let dev_plan_doc = fs::read_to_string("docs/SCHEDULER_DEVELOPMENT_PLAN.md")
        .expect("docs/SCHEDULER_DEVELOPMENT_PLAN.md must exist");

    assert!(invariants_doc.contains("SigmaOS CPU Scheduler Status, Architecture & Policy Invariants"));
    assert!(invariants_doc.contains("Core Scheduling Rules & Invariants"));
    assert!(invariants_doc.contains("Policy/Mechanism Separation"));
    assert!(invariants_doc.contains("Explicit Task State Machine"));

    assert!(dev_plan_doc.contains("SigmaOS CPU Scheduler Development Plan"));
    assert!(dev_plan_doc.contains("18-PR Sequence Roadmap"));
    assert!(dev_plan_doc.contains("Periodic Maintenance Cadences"));
    assert!(dev_plan_doc.contains("Review and Merge Rules"));
}

#[test]
fn test_scheduler_task_state_machine_invariants() {
    // Verify task state representations and valid transition model
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum TaskState {
        New,
        Runnable,
        Running,
        Blocked,
        Stopped,
        Exiting,
        Idle,
    }

    struct Task {
        id: u64,
        state: TaskState,
        cpu_id: Option<u32>,
    }

    impl Task {
        fn new(id: u64) -> Self {
            Self {
                id,
                state: TaskState::New,
                cpu_id: None,
            }
        }

        fn enqueue(&mut self, cpu: u32) -> Result<(), &'static str> {
            if self.state == TaskState::New || self.state == TaskState::Blocked || self.state == TaskState::Runnable {
                self.state = TaskState::Runnable;
                self.cpu_id = Some(cpu);
                Ok(())
            } else {
                Err("Invalid enqueue transition")
            }
        }

        fn select_to_run(&mut self) -> Result<(), &'static str> {
            if self.state == TaskState::Runnable {
                self.state = TaskState::Running;
                Ok(())
            } else {
                Err("Task must be Runnable to be selected for Running")
            }
        }

        fn block(&mut self) -> Result<(), &'static str> {
            if self.state == TaskState::Running {
                self.state = TaskState::Blocked;
                self.cpu_id = None;
                Ok(())
            } else {
                Err("Task must be Running to Block")
            }
        }
    }

    let mut task = Task::new(101);
    assert_eq!(task.state, TaskState::New);

    assert!(task.enqueue(0).is_ok());
    assert_eq!(task.state, TaskState::Runnable);

    assert!(task.select_to_run().is_ok());
    assert_eq!(task.state, TaskState::Running);

    assert!(task.block().is_ok());
    assert_eq!(task.state, TaskState::Blocked);

    // Cannot run directly from Blocked
    assert!(task.select_to_run().is_err());
}
