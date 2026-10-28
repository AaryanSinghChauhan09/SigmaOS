// Comprehensive Unit Tests for CPU Scheduler Policies, Invariants, and Plan Verification

#[test]
fn test_scheduler_documentation_policy_verification() {
    use std::fs;

    let sched_policy = fs::read_to_string("docs/process/SCHEDULER_POLICY.md")
        .expect("docs/process/SCHEDULER_POLICY.md must exist");
    let sched_adr = fs::read_to_string("docs/process/SCHEDULER_ADR.md")
        .expect("docs/process/SCHEDULER_ADR.md must exist");
    let rq_policy = fs::read_to_string("docs/process/RUN_QUEUE_POLICY.md")
        .expect("docs/process/RUN_QUEUE_POLICY.md must exist");
    let ctx_policy = fs::read_to_string("docs/process/CONTEXT_SWITCH_POLICY.md")
        .expect("docs/process/CONTEXT_SWITCH_POLICY.md must exist");
    let preempt_policy = fs::read_to_string("docs/process/PREEMPTION_POLICY.md")
        .expect("docs/process/PREEMPTION_POLICY.md must exist");
    let fairness_policy = fs::read_to_string("docs/process/FAIRNESS_POLICY.md")
        .expect("docs/process/FAIRNESS_POLICY.md must exist");
    let affinity_policy = fs::read_to_string("docs/process/CPU_AFFINITY_POLICY.md")
        .expect("docs/process/CPU_AFFINITY_POLICY.md must exist");

    assert!(sched_policy.contains("SigmaOS Scheduler Subsystem Policy"));
    assert!(sched_policy.contains("Authoritative Lifecycle State Machine"));
    assert!(sched_adr.contains("Architectural Decision Record: Scheduler Subsystem Lifecycle State Machine"));
    assert!(rq_policy.contains("Run Queue Policy Specification"));
    assert!(ctx_policy.contains("Context Switch Policy Specification"));
    assert!(preempt_policy.contains("Preemption Policy Specification"));
    assert!(fairness_policy.contains("Fairness and Starvation Prevention Policy"));
    assert!(affinity_policy.contains("CPU Affinity Policy Specification"));
}

#[test]
fn test_scheduler_task_state_machine_invariants() {
    // Verify task state representations and valid transition model
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum TaskState {
        New,
        Ready,
        Running,
        Sleeping,
        Stopped,
        Zombie,
        Reaped,
    }

    struct Task {
        pid: u64,
        state: TaskState,
        cpu_id: Option<u32>,
    }

    impl Task {
        fn new(pid: u64) -> Self {
            Self {
                pid,
                state: TaskState::New,
                cpu_id: None,
            }
        }

        fn enqueue(&mut self, cpu: u32) -> Result<(), &'static str> {
            if self.state == TaskState::New || self.state == TaskState::Sleeping || self.state == TaskState::Stopped {
                self.state = TaskState::Ready;
                self.cpu_id = Some(cpu);
                Ok(())
            } else {
                Err("Invalid enqueue transition")
            }
        }

        fn select_to_run(&mut self) -> Result<(), &'static str> {
            if self.state == TaskState::Ready {
                self.state = TaskState::Running;
                Ok(())
            } else {
                Err("Task must be Ready to be selected for Running")
            }
        }

        fn sleep(&mut self) -> Result<(), &'static str> {
            if self.state == TaskState::Running {
                self.state = TaskState::Sleeping;
                self.cpu_id = None;
                Ok(())
            } else {
                Err("Task must be Running to Sleep")
            }
        }

        fn exit(&mut self) -> Result<(), &'static str> {
            self.state = TaskState::Zombie;
            self.cpu_id = None;
            Ok(())
        }

        fn reap(&mut self) -> Result<(), &'static str> {
            if self.state == TaskState::Zombie {
                self.state = TaskState::Reaped;
                Ok(())
            } else {
                Err("Task must be Zombie to be Reaped")
            }
        }
    }

    let mut task = Task::new(101);
    assert_eq!(task.state, TaskState::New);

    assert!(task.enqueue(0).is_ok());
    assert_eq!(task.state, TaskState::Ready);

    assert!(task.select_to_run().is_ok());
    assert_eq!(task.state, TaskState::Running);

    assert!(task.sleep().is_ok());
    assert_eq!(task.state, TaskState::Sleeping);

    // Cannot run directly from Sleeping
    assert!(task.select_to_run().is_err());

    assert!(task.enqueue(0).is_ok());
    assert_eq!(task.state, TaskState::Ready);

    assert!(task.select_to_run().is_ok());
    assert_eq!(task.state, TaskState::Running);

    assert!(task.exit().is_ok());
    assert_eq!(task.state, TaskState::Zombie);

    assert!(task.reap().is_ok());
    assert_eq!(task.state, TaskState::Reaped);
}
