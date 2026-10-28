// SPDX-License-Identifier: MIT
// SigmaOS Process Manager Architecture & Plan Verification Test Suite

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::Path;

#[test]
fn test_process_manager_documentation_plan_verification() {
    let docs_path = "docs/PROCESS_MANAGER_PLAN.md";
    let wiki_path = "wiki/PROCESS_MANAGER_PLAN.md";

    assert!(
        Path::new(docs_path).exists(),
        "docs/PROCESS_MANAGER_PLAN.md must exist"
    );
    assert!(
        Path::new(wiki_path).exists(),
        "wiki/PROCESS_MANAGER_PLAN.md must exist"
    );

    let docs_content = fs::read_to_string(docs_path).expect("Read docs plan");
    let wiki_content = fs::read_to_string(wiki_path).expect("Read wiki plan");

    assert_eq!(
        docs_content, wiki_content,
        "wiki/PROCESS_MANAGER_PLAN.md must mirror docs/PROCESS_MANAGER_PLAN.md exactly"
    );

    // Verify key sections and contracts exist in documentation
    assert!(docs_content.contains("SigmaOS Process Manager Master Architecture & Support Matrix Plan"));
    assert!(docs_content.contains("Design and Safety Rules"));
    assert!(docs_content.contains("Separate kernel lifecycle from service policy"));
    assert!(docs_content.contains("Reap every child"));
    assert!(docs_content.contains("Avoid PID-reuse races in management"));
    assert!(docs_content.contains("Bound service recovery"));
    assert!(docs_content.contains("Keep service dependencies declarative and cycle-checked"));
    assert!(docs_content.contains("Proposed PR Sequence"));
    assert!(docs_content.contains("Recommended First Milestone"));
    assert!(docs_content.contains("Periodic Maintenance PR Cadence"));
    assert!(docs_content.contains("Component-Specific Rules"));
    assert!(docs_content.contains("PR Template and Merge Gates"));
    assert!(docs_content.contains("Completion Gates"));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ProcessState {
    Created,
    Starting,
    Running,
    Blocked,
    Stopping,
    Exited,
    Reaped,
    Failed,
}

#[derive(Debug, Clone)]
struct ProcessControlBlock {
    pid: u64,
    ppid: u64,
    state: ProcessState,
    exit_code: Option<i32>,
    is_orphan: bool,
}

impl ProcessControlBlock {
    fn new(pid: u64, ppid: u64) -> Self {
        Self {
            pid,
            ppid,
            state: ProcessState::Created,
            exit_code: None,
            is_orphan: false,
        }
    }

    fn transition_to(&mut self, target: ProcessState) -> Result<(), &'static str> {
        use ProcessState::*;
        let valid = match (self.state, target) {
            (Created, Starting) => true,
            (Starting, Running) => true,
            (Starting, Failed) => true,
            (Running, Blocked) => true,
            (Running, Stopping) => true,
            (Running, Exited) => true,
            (Blocked, Running) => true,
            (Blocked, Exited) => true,
            (Stopping, Exited) => true,
            (Stopping, Failed) => true,
            (Exited, Reaped) => true,
            _ => false,
        };

        if valid {
            self.state = target;
            Ok(())
        } else {
            Err("Invalid process state transition")
        }
    }
}

#[test]
fn test_process_state_transitions() {
    let mut pcb = ProcessControlBlock::new(100, 1);
    assert_eq!(pcb.state, ProcessState::Created);

    assert!(pcb.transition_to(ProcessState::Starting).is_ok());
    assert!(pcb.transition_to(ProcessState::Running).is_ok());

    // Invalid transition directly from Running to Reaped
    assert!(pcb.transition_to(ProcessState::Reaped).is_err());

    assert!(pcb.transition_to(ProcessState::Blocked).is_ok());
    assert!(pcb.transition_to(ProcessState::Running).is_ok());
    assert!(pcb.transition_to(ProcessState::Stopping).is_ok());
    assert!(pcb.transition_to(ProcessState::Exited).is_ok());

    pcb.exit_code = Some(0);
    assert!(pcb.transition_to(ProcessState::Reaped).is_ok());
    assert_eq!(pcb.state, ProcessState::Reaped);

    // Cannot transition out of Reaped
    assert!(pcb.transition_to(ProcessState::Running).is_err());
}

#[test]
fn test_child_reaping_and_orphan_adoption() {
    struct ProcessManager {
        processes: HashMap<u64, ProcessControlBlock>,
        next_pid: u64,
    }

    impl ProcessManager {
        fn new() -> Self {
            let mut mgr = Self {
                processes: HashMap::new(),
                next_pid: 1,
            };
            // PID 1 is init
            let init = ProcessControlBlock {
                pid: 1,
                ppid: 0,
                state: ProcessState::Running,
                exit_code: None,
                is_orphan: false,
            };
            mgr.processes.insert(1, init);
            mgr.next_pid = 2;
            mgr
        }

        fn spawn(&mut self, ppid: u64) -> u64 {
            let pid = self.next_pid;
            self.next_pid += 1;
            let mut pcb = ProcessControlBlock::new(pid, ppid);
            pcb.transition_to(ProcessState::Starting).unwrap();
            pcb.transition_to(ProcessState::Running).unwrap();
            self.processes.insert(pid, pcb);
            pid
        }

        fn exit_process(&mut self, pid: u64, exit_code: i32) {
            if let Some(pcb) = self.processes.get_mut(&pid) {
                pcb.transition_to(ProcessState::Exited).unwrap();
                pcb.exit_code = Some(exit_code);
            }

            // Adopt children if parent exits
            let children: Vec<u64> = self
                .processes
                .values()
                .filter(|p| p.ppid == pid)
                .map(|p| p.pid)
                .collect();

            for child_pid in children {
                if let Some(child) = self.processes.get_mut(&child_pid) {
                    child.ppid = 1; // Reparented to PID 1
                    child.is_orphan = true;
                }
            }
        }

        fn waitpid(&mut self, ppid: u64, target_pid: u64) -> Result<i32, &'static str> {
            let pcb = self
                .processes
                .get(&target_pid)
                .ok_or("No such process")?;

            if pcb.ppid != ppid {
                return Err("Not parent of target process");
            }

            if pcb.state != ProcessState::Exited {
                return Err("Process still running");
            }

            let exit_code = pcb.exit_code.expect("Exit code must be set");

            // Reap process
            if let Some(pcb_mut) = self.processes.get_mut(&target_pid) {
                pcb_mut.transition_to(ProcessState::Reaped).unwrap();
            }

            Ok(exit_code)
        }
    }

    let mut pm = ProcessManager::new();
    let parent = pm.spawn(1);
    let child1 = pm.spawn(parent);
    let child2 = pm.spawn(parent);

    // Normal child exit and waitpid
    pm.exit_process(child1, 42);
    let code = pm.waitpid(parent, child1).expect("Waitpid succeeds");
    assert_eq!(code, 42);

    // Second waitpid fails (already reaped)
    assert!(pm.waitpid(parent, child1).is_err());

    // Parent exits before child2, orphan adoption to PID 1
    pm.exit_process(parent, 0);
    assert_eq!(pm.processes.get(&child2).unwrap().ppid, 1);
    assert!(pm.processes.get(&child2).unwrap().is_orphan);

    // PID 1 can now reap child2 when it exits
    pm.exit_process(child2, 100);
    let orphan_code = pm.waitpid(1, child2).expect("PID 1 reaps orphan");
    assert_eq!(orphan_code, 100);
}

#[test]
fn test_service_dependency_graph_cycle_detection() {
    struct ServiceGraph {
        dependencies: HashMap<String, Vec<String>>,
    }

    impl ServiceGraph {
        fn new() -> Self {
            Self {
                dependencies: HashMap::new(),
            }
        }

        fn add_service(&mut self, name: &str, deps: &[&str]) {
            let dep_vec = deps.iter().map(|s| s.to_string()).collect();
            self.dependencies.insert(name.to_string(), dep_vec);
        }

        fn validate_no_cycles(&self) -> Result<Vec<String>, &'static str> {
            let mut in_degree: HashMap<String, usize> = HashMap::new();
            let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

            for node in self.dependencies.keys() {
                in_degree.entry(node.clone()).or_insert(0);
                adj_list.entry(node.clone()).or_default();
            }

            for (node, deps) in &self.dependencies {
                for dep in deps {
                    if !self.dependencies.contains_key(dep) {
                        return Err("Missing required dependency");
                    }
                    adj_list.entry(dep.clone()).or_default().push(node.clone());
                    *in_degree.entry(node.clone()).or_insert(0) += 1;
                }
            }

            let mut queue: VecDeque<String> = in_degree
                .iter()
                .filter(|(_, &deg)| deg == 0)
                .map(|(node, _)| node.clone())
                .collect();

            let mut order = Vec::new();

            while let Some(node) = queue.pop_front() {
                order.push(node.clone());
                if let Some(neighbors) = adj_list.get(&node) {
                    for neighbor in neighbors {
                        let deg = in_degree.get_mut(neighbor).unwrap();
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }

            if order.len() == self.dependencies.len() {
                Ok(order)
            } else {
                Err("Dependency cycle detected")
            }
        }
    }

    // Valid DAG
    let mut valid_graph = ServiceGraph::new();
    valid_graph.add_service("vfs", &[]);
    valid_graph.add_service("network", &["vfs"]);
    valid_graph.add_service("httpd", &["network", "vfs"]);

    let order = valid_graph.validate_no_cycles().expect("DAG should validate");
    assert_eq!(order.first().unwrap(), "vfs");

    // Cyclic DAG
    let mut cyclic_graph = ServiceGraph::new();
    cyclic_graph.add_service("A", &["B"]);
    cyclic_graph.add_service("B", &["C"]);
    cyclic_graph.add_service("C", &["A"]);

    assert_eq!(
        cyclic_graph.validate_no_cycles(),
        Err("Dependency cycle detected")
    );

    // Missing dependency
    let mut missing_graph = ServiceGraph::new();
    missing_graph.add_service("A", &["nonexistent"]);

    assert_eq!(
        missing_graph.validate_no_cycles(),
        Err("Missing required dependency")
    );
}

#[test]
fn test_bounded_service_restart_policy() {
    struct ServiceSupervisor {
        restart_count: usize,
        max_restarts: usize,
        state: ProcessState,
    }

    impl ServiceSupervisor {
        fn new(max_restarts: usize) -> Self {
            Self {
                restart_count: 0,
                max_restarts,
                state: ProcessState::Running,
            }
        }

        fn handle_crash(&mut self) -> bool {
            if self.restart_count < self.max_restarts {
                self.restart_count += 1;
                self.state = ProcessState::Starting;
                // Simulating successful restart
                self.state = ProcessState::Running;
                true
            } else {
                self.state = ProcessState::Failed;
                false
            }
        }
    }

    let mut supervisor = ServiceSupervisor::new(3);
    assert!(supervisor.handle_crash()); // Restart 1
    assert_eq!(supervisor.restart_count, 1);
    assert_eq!(supervisor.state, ProcessState::Running);

    assert!(supervisor.handle_crash()); // Restart 2
    assert!(supervisor.handle_crash()); // Restart 3
    assert_eq!(supervisor.restart_count, 3);

    // 4th crash exceeds max_restarts limit
    assert!(!supervisor.handle_crash());
    assert_eq!(supervisor.state, ProcessState::Failed);
}
