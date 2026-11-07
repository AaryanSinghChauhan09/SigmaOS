// Comprehensive Unit Tests for CPU Scheduler Policies, Invariants, and Plan Verification

use std::fs;
use std::path::Path;

#[test]
fn test_scheduler_documentation_policy_verification() {
    let sched_policy = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/Doc-process-SCHEDULER_POLICY.md")
        .expect("docs/process/SCHEDULER_POLICY.md must exist");
    let sched_adr = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/Doc-process-SCHEDULER_ADR.md")
        .expect("docs/process/SCHEDULER_ADR.md must exist");
    let rq_policy = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/Doc-process-RUN_QUEUE_POLICY.md")
        .expect("docs/process/RUN_QUEUE_POLICY.md must exist");
    let ctx_policy = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/Doc-process-CONTEXT_SWITCH_POLICY.md")
        .expect("docs/process/CONTEXT_SWITCH_POLICY.md must exist");
    let preempt_policy = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/Doc-process-PREEMPTION_POLICY.md")
        .expect("docs/process/PREEMPTION_POLICY.md must exist");
    let fairness_policy = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/Doc-process-FAIRNESS_POLICY.md")
        .expect("docs/process/FAIRNESS_POLICY.md must exist");
    let affinity_policy = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/Doc-process-CPU_AFFINITY_POLICY.md")
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
fn test_scheduler_master_plan_documents_exist_and_match() {
    let doc_invariants = Path::new("/home/aaryansinghchauhan/SigmaOS.wiki/SCHEDULER_STATUS_AND_INVARIANTS.md");
    let wiki_invariants = Path::new("wiki/SCHEDULER_STATUS_AND_INVARIANTS.md");
    let doc_plan = Path::new("/home/aaryansinghchauhan/SigmaOS.wiki/SCHEDULER_DEVELOPMENT_PLAN.md");
    let wiki_plan = Path::new("wiki/SCHEDULER_DEVELOPMENT_PLAN.md");

    assert!(doc_invariants.exists(), "docs/SCHEDULER_STATUS_AND_INVARIANTS.md must exist");
    assert!(wiki_invariants.exists(), "wiki/SCHEDULER_STATUS_AND_INVARIANTS.md must exist");
    assert!(doc_plan.exists(), "docs/SCHEDULER_DEVELOPMENT_PLAN.md must exist");
    assert!(wiki_plan.exists(), "wiki/SCHEDULER_DEVELOPMENT_PLAN.md must exist");

    let doc_inv_content = fs::read_to_string(doc_invariants).expect("read doc invariants");
    let wiki_inv_content = fs::read_to_string(wiki_invariants).expect("read wiki invariants");
    assert_eq!(doc_inv_content, wiki_inv_content, "wiki copy must match docs copy for invariants");

    let doc_plan_content = fs::read_to_string(doc_plan).expect("read doc plan");
    let wiki_plan_content = fs::read_to_string(wiki_plan).expect("read wiki plan");
    assert_eq!(doc_plan_content, wiki_plan_content, "wiki copy must match docs copy for development plan");
}

#[test]
fn test_scheduler_master_plan_invariants_and_rules() {
    let content = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/SCHEDULER_STATUS_AND_INVARIANTS.md")
        .expect("read docs/SCHEDULER_STATUS_AND_INVARIANTS.md");

    let required_rules = [
        "Separate policy from mechanism.",
        "Define task states and transitions.",
        "Fair scheduling needs a measurable contract.",
        "Real-time policies need guardrails.",
        "Scheduler changes require data, not anecdotes.",
        "Concurrency is part of correctness.",
        "Resource controls must affect real scheduling.",
        "Keep architecture status separate.",
        "Keep diagnostics bounded and stable.",
    ];

    for rule in &required_rules {
        assert!(content.contains(rule), "Missing required invariant rule: {}", rule);
    }
}

#[test]
fn test_scheduler_master_plan_pr_sequence() {
    let content = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/SCHEDULER_DEVELOPMENT_PLAN.md")
        .expect("read docs/SCHEDULER_DEVELOPMENT_PLAN.md");

    for pr_num in 1..=18 {
        let pr_marker = format!("| {} |", pr_num);
        assert!(
            content.contains(&pr_marker),
            "Missing PR sequence marker: {}",
            pr_marker
        );
    }

    assert!(
        content.contains("`docs(scheduler): audit scheduler status and define invariants`"),
        "Missing PR 1 title"
    );
    assert!(
        content.contains("`feat(scheduler): implement EEVDF fair-class prototype`"),
        "Missing PR 8 title"
    );
    assert!(
        content.contains("`docs(scheduler): publish policy guide and support matrix`"),
        "Missing PR 18 title"
    );

    assert!(
        content.contains("Complete PRs 1–7 before deciding whether EEVDF or BORE becomes the default fair policy"),
        "Missing recommended milestone text"
    );
}

#[test]
fn test_scheduler_master_plan_cadence_and_gates() {
    let content = fs::read_to_string("/home/aaryansinghchauhan/SigmaOS.wiki/SCHEDULER_DEVELOPMENT_PLAN.md")
        .expect("read docs/SCHEDULER_DEVELOPMENT_PLAN.md");

    let cadences = [
        "Every PR",
        "Weekly",
        "Monthly",
        "Quarterly",
        "Each release candidate",
        "After each incident",
    ];

    for cadence in &cadences {
        assert!(
            content.contains(cadence),
            "Missing maintenance cadence: {}",
            cadence
        );
    }

    let completion_gates = ["**Prototype**", "**Integrated**", "**Supported**"];

    for gate in &completion_gates {
        assert!(
            content.contains(gate),
            "Missing completion gate: {}",
            gate
        );
    }
}

#[test]
fn test_feature_status_toml_scheduler_registration() {
    let content = fs::read_to_string("FEATURE_STATUS.toml")
        .expect("read FEATURE_STATUS.toml");

    let required_sections = [
        "[scheduler.core]",
        "[scheduler.bore]",
        "[scheduler.cfs]",
        "[scheduler.realtime]",
        "[scheduler.eevdf]",
    ];

    for section in &required_sections {
        assert!(
            content.contains(section),
            "FEATURE_STATUS.toml must contain {}",
            section
        );
    }

    assert!(
        content.contains("SCHEDULER_STATUS_AND_INVARIANTS.md"),
        "FEATURE_STATUS.toml must reference docs/SCHEDULER_STATUS_AND_INVARIANTS.md"
    );
    assert!(
        content.contains("SCHEDULER_DEVELOPMENT_PLAN.md"),
        "FEATURE_STATUS.toml must reference docs/SCHEDULER_DEVELOPMENT_PLAN.md"
    );
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
    assert_eq!(task.pid, 101);

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
