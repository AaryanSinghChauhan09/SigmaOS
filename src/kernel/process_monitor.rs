// Process Monitoring for SigmaOS
// Process monitoring per Wiki 04-Kernel.md
// Provides process listing, tree, and details without conflicting with existing types

use std::string::{String, ToString};
use std::vec::Vec;

/// Process state for monitoring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitoredProcessState {
    Running,
    Sleeping,
    Waiting,
    Stopped,
    Zombie,
    Dead,
}

impl MonitoredProcessState {
    pub fn as_str(&self) -> &str {
        match self {
            MonitoredProcessState::Running => "R",
            MonitoredProcessState::Sleeping => "S",
            MonitoredProcessState::Waiting => "D",
            MonitoredProcessState::Stopped => "T",
            MonitoredProcessState::Zombie => "Z",
            MonitoredProcessState::Dead => "X",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "R" => MonitoredProcessState::Running,
            "S" => MonitoredProcessState::Sleeping,
            "D" => MonitoredProcessState::Waiting,
            "T" => MonitoredProcessState::Stopped,
            "Z" => MonitoredProcessState::Zombie,
            "X" => MonitoredProcessState::Dead,
            _ => MonitoredProcessState::Sleeping,
        }
    }
}

/// Process entry for monitoring
#[derive(Debug, Clone)]
pub struct ProcessEntry {
    pub pid: u32,
    pub ppid: u32,
    pub uid: u32,
    pub gid: u32,
    pub state: MonitoredProcessState,
    pub name: String,
    pub command: String,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub memory_kb: u64,
    pub runtime: u64,
    pub priority: i32,
    pub nice: i32,
    pub threads: u32,
}

impl ProcessEntry {
    pub fn new(pid: u32, name: String) -> Self {
        ProcessEntry {
            pid,
            ppid: 0,
            uid: 0,
            gid: 0,
            state: MonitoredProcessState::Running,
            name,
            command: String::new(),
            cpu_percent: 0.0,
            memory_percent: 0.0,
            memory_kb: 0,
            runtime: 0,
            priority: 0,
            nice: 0,
            threads: 1,
        }
    }

    pub fn get_summary(&self) -> String {
        format!(
            "{} {} {} {} {:.1}% {:.1}% {}K {} {} {}",
            self.pid,
            self.ppid,
            self.state.as_str(),
            self.name,
            self.cpu_percent,
            self.memory_percent,
            self.memory_kb,
            self.priority,
            self.nice,
            self.threads
        )
    }
}

/// Process tree node
#[derive(Debug, Clone)]
pub struct ProcessTreeNode {
    pub process: ProcessEntry,
    pub children: Vec<ProcessTreeNode>,
}

impl ProcessTreeNode {
    pub fn new(process: ProcessEntry) -> Self {
        ProcessTreeNode {
            process,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: ProcessTreeNode) {
        self.children.push(child);
    }

    pub fn print_tree(&self, indent: usize) -> String {
        let mut output = String::new();
        let prefix = "  ".repeat(indent);
        output.push_str(&format!(
            "{}[{}] {} ({})\n",
            prefix,
            self.process.pid,
            self.process.name,
            self.process.state.as_str()
        ));
        output.push_str(&format!("{}[{}] {} ({})\n", prefix, self.process.pid, self.process.name, self.process.state.as_str()));

        for child in &self.children {
            output.push_str(&child.print_tree(indent + 1));
        }

        output
    }
}

/// Process filter
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessFilter {
    All,
    Running,
    Sleeping,
    Stopped,
    Zombie,
    ByUser(u32),
    ByName(String),
}

impl ProcessFilter {
    pub fn matches(&self, process: &ProcessEntry) -> bool {
        match self {
            ProcessFilter::All => true,
            ProcessFilter::Running => process.state == MonitoredProcessState::Running,
            ProcessFilter::Sleeping => process.state == MonitoredProcessState::Sleeping,
            ProcessFilter::Stopped => process.state == MonitoredProcessState::Stopped,
            ProcessFilter::Zombie => process.state == MonitoredProcessState::Zombie,
            ProcessFilter::ByUser(uid) => process.uid == *uid,
            ProcessFilter::ByName(name) => process.name.contains(name.as_str()),
        }
    }
}

/// Process sort field
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessSortField {
    Pid,
    Name,
    Cpu,
    Memory,
    Runtime,
    Priority,
}

impl ProcessSortField {
    pub fn compare(&self, a: &ProcessEntry, b: &ProcessEntry) -> std::cmp::Ordering {
        match self {
            ProcessSortField::Pid => a.pid.cmp(&b.pid),
            ProcessSortField::Name => a.name.cmp(&b.name),
            ProcessSortField::Cpu => a.cpu_percent.partial_cmp(&b.cpu_percent).unwrap_or(std::cmp::Ordering::Equal).reverse(),
            ProcessSortField::Memory => a.memory_percent.partial_cmp(&b.memory_percent).unwrap_or(std::cmp::Ordering::Equal).reverse(),
            ProcessSortField::Runtime => a.runtime.cmp(&b.runtime).reverse(),
            ProcessSortField::Priority => a.priority.cmp(&b.priority).reverse(),
        }
    }
}

/// Process monitor
#[derive(Debug, Clone)]
pub struct ProcessMonitor {
    pub processes: Vec<ProcessEntry>,
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        ProcessMonitor {
            processes: Vec::new(),
        }
    }
}

impl ProcessMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_process(&mut self, process: ProcessEntry) {
        self.processes.push(process);
    }

    pub fn get_process(&self, pid: u32) -> Option<&ProcessEntry> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    pub fn get_process_mut(&mut self, pid: u32) -> Option<&mut ProcessEntry> {
        self.processes.iter_mut().find(|p| p.pid == pid)
    }

    pub fn list_all(&self) -> Vec<ProcessEntry> {
        self.processes.clone()
    }

    pub fn list_filtered(&self, filter: ProcessFilter) -> Vec<ProcessEntry> {
        self.processes.iter()
            .filter(|p| filter.matches(p))
            .cloned()
            .collect()
    }

    pub fn list_sorted(&self, sort_field: ProcessSortField) -> Vec<ProcessEntry> {
        let mut processes = self.processes.clone();
        processes.sort_by(|a, b| sort_field.compare(a, b));
        processes
    }

    pub fn list_filtered_sorted(&self, filter: ProcessFilter, sort_field: ProcessSortField) -> Vec<ProcessEntry> {
        let mut processes = self.list_filtered(filter);
        processes.sort_by(|a, b| sort_field.compare(a, b));
        processes
    }

    pub fn build_tree(&self) -> Vec<ProcessTreeNode> {
        let mut nodes: Vec<ProcessTreeNode> = Vec::new();
        let mut by_pid: std::collections::HashMap<u32, ProcessTreeNode> =
            std::collections::HashMap::new();
        let mut by_pid: std::collections::HashMap<u32, ProcessTreeNode> = std::collections::HashMap::new();

        for process in &self.processes {
            by_pid.insert(process.pid, ProcessTreeNode::new(process.clone()));
        }

        let mut root_nodes: Vec<ProcessTreeNode> = Vec::new();

        for process in &self.processes {
            if let Some(mut node) = by_pid.remove(&process.pid) {
                if process.ppid == 0 || !by_pid.contains_key(&process.ppid) {
                    root_nodes.push(node);
                } else if let Some(parent) = by_pid.get_mut(&process.ppid) {
                    parent.add_child(node);
                }
            }
        }

        root_nodes
    }

    pub fn print_tree(&self) -> String {
        let tree = self.build_tree();
        let mut output = String::from("Process Tree:\n");

        for node in &tree {
            output.push_str(&node.print_tree(0));
        }

        output
    }

    pub fn show_process(&self, pid: u32) -> Option<String> {
        if let Some(process) = self.get_process(pid) {
            let mut details = format!("Process Details for PID {}:\n", pid);
            details.push_str(&format!("  Name: {}\n", process.name));
            details.push_str(&format!("  Command: {}\n", process.command));
            details.push_str(&format!("  State: {}\n", process.state.as_str()));
            details.push_str(&format!("  PPID: {}\n", process.ppid));
            details.push_str(&format!("  UID: {}\n", process.uid));
            details.push_str(&format!("  GID: {}\n", process.gid));
            details.push_str(&format!("  CPU: {:.1}%\n", process.cpu_percent));
            details.push_str(&format!("  Memory: {:.1}% ({} KB)\n", process.memory_percent, process.memory_kb));
            details.push_str(&format!("  Runtime: {} seconds\n", process.runtime));
            details.push_str(&format!("  Priority: {}\n", process.priority));
            details.push_str(&format!("  Nice: {}\n", process.nice));
            details.push_str(&format!("  Threads: {}\n", process.threads));
            Some(details)
        } else {
            None
        }
    }

    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Process Statistics:\n");
        stats.push_str(&format!("Total processes: {}\n", self.processes.len()));

        let running = self
            .processes
            .iter()
            .filter(|p| p.state == MonitoredProcessState::Running)
            .count();
        let sleeping = self
            .processes
            .iter()
            .filter(|p| p.state == MonitoredProcessState::Sleeping)
            .count();
        let stopped = self
            .processes
            .iter()
            .filter(|p| p.state == MonitoredProcessState::Stopped)
            .count();
        let zombie = self
            .processes
            .iter()
            .filter(|p| p.state == MonitoredProcessState::Zombie)
            .count();
        let running = self.processes.iter().filter(|p| p.state == MonitoredProcessState::Running).count();
        let sleeping = self.processes.iter().filter(|p| p.state == MonitoredProcessState::Sleeping).count();
        let stopped = self.processes.iter().filter(|p| p.state == MonitoredProcessState::Stopped).count();
        let zombie = self.processes.iter().filter(|p| p.state == MonitoredProcessState::Zombie).count();

        stats.push_str(&format!("Running: {}\n", running));
        stats.push_str(&format!("Sleeping: {}\n", sleeping));
        stats.push_str(&format!("Stopped: {}\n", stopped));
        stats.push_str(&format!("Zombie: {}\n", zombie));

        let total_cpu: f32 = self.processes.iter().map(|p| p.cpu_percent).sum();
        let total_memory: u64 = self.processes.iter().map(|p| p.memory_kb).sum();

        stats.push_str(&format!("Total CPU usage: {:.1}%\n", total_cpu));
        stats.push_str(&format!("Total memory: {} KB\n", total_memory));

        stats
    }

    pub fn kill_process(&mut self, pid: u32) -> bool {
        if let Some(pos) = self.processes.iter().position(|p| p.pid == pid) {
            self.processes.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn set_priority(&mut self, pid: u32, priority: i32) -> bool {
        if let Some(process) = self.get_process_mut(pid) {
            process.priority = priority;
            true
        } else {
            false
        }
    }

    pub fn set_nice(&mut self, pid: u32, nice: i32) -> bool {
        if let Some(process) = self.get_process_mut(pid) {
            process.nice = nice;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_state_as_str() {
        assert_eq!(MonitoredProcessState::Running.as_str(), "R");
        assert_eq!(MonitoredProcessState::Sleeping.as_str(), "S");
        assert_eq!(MonitoredProcessState::Zombie.as_str(), "Z");
    }

    #[test]
    fn test_process_state_from_str() {
        assert_eq!(MonitoredProcessState::from_str("R"), MonitoredProcessState::Running);
        assert_eq!(MonitoredProcessState::from_str("S"), MonitoredProcessState::Sleeping);
        assert_eq!(MonitoredProcessState::from_str("Z"), MonitoredProcessState::Zombie);
    }

    #[test]
    fn test_process_entry_creation() {
        let process = ProcessEntry::new(123, String::from("test"));
        assert_eq!(process.pid, 123);
        assert_eq!(process.name, "test");
        assert_eq!(process.state, MonitoredProcessState::Running);
    }

    #[test]
    fn test_process_entry_get_summary() {
        let mut process = ProcessEntry::new(123, String::from("test"));
        process.ppid = 1;
        process.cpu_percent = 5.5;
        process.memory_percent = 2.3;
        process.memory_kb = 1024;

        let summary = process.get_summary();
        assert!(summary.contains("123"));
        assert!(summary.contains("test"));
        assert!(summary.contains("5.5%"));
    }

    #[test]
    fn test_process_tree_node_creation() {
        let process = ProcessEntry::new(123, String::from("test"));
        let node = ProcessTreeNode::new(process);
        assert_eq!(node.process.pid, 123);
        assert_eq!(node.children.len(), 0);
    }

    #[test]
    fn test_process_tree_node_add_child() {
        let parent = ProcessEntry::new(1, String::from("parent"));
        let child = ProcessEntry::new(2, String::from("child"));

        let mut node = ProcessTreeNode::new(parent);
        node.add_child(ProcessTreeNode::new(child));

        assert_eq!(node.children.len(), 1);
    }

    #[test]
    fn test_process_filter_all() {
        let process = ProcessEntry::new(123, String::from("test"));
        assert!(ProcessFilter::All.matches(&process));
    }

    #[test]
    fn test_process_filter_running() {
        let mut process = ProcessEntry::new(123, String::from("test"));
        process.state = MonitoredProcessState::Running;
        assert!(ProcessFilter::Running.matches(&process));

        process.state = MonitoredProcessState::Sleeping;
        assert!(!ProcessFilter::Running.matches(&process));
    }

    #[test]
    fn test_process_filter_by_user() {
        let mut process = ProcessEntry::new(123, String::from("test"));
        process.uid = 1000;
        assert!(ProcessFilter::ByUser(1000).matches(&process));
        assert!(!ProcessFilter::ByUser(0).matches(&process));
    }

    #[test]
    fn test_process_monitor_creation() {
        let monitor = ProcessMonitor::new();
        assert_eq!(monitor.processes.len(), 0);
    }

    #[test]
    fn test_process_monitor_add_process() {
        let mut monitor = ProcessMonitor::new();
        monitor.add_process(ProcessEntry::new(123, String::from("test")));
        assert_eq!(monitor.processes.len(), 1);
    }

    #[test]
    fn test_process_monitor_get_process() {
        let mut monitor = ProcessMonitor::new();
        monitor.add_process(ProcessEntry::new(123, String::from("test")));

        let process = monitor.get_process(123);
        assert!(process.is_some());
        assert_eq!(process.unwrap().pid, 123);
    }

    #[test]
    fn test_process_monitor_list_all() {
        let mut monitor = ProcessMonitor::new();
        monitor.add_process(ProcessEntry::new(123, String::from("test")));
        monitor.add_process(ProcessEntry::new(124, String::from("test2")));

        let all = monitor.list_all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_process_monitor_list_filtered() {
        let mut monitor = ProcessMonitor::new();
        let mut process1 = ProcessEntry::new(123, String::from("test"));
        process1.state = MonitoredProcessState::Running;
        monitor.add_process(process1);

        let mut process2 = ProcessEntry::new(124, String::from("test2"));
        process2.state = MonitoredProcessState::Sleeping;
        monitor.add_process(process2);

        let running = monitor.list_filtered(ProcessFilter::Running);
        assert_eq!(running.len(), 1);
    }

    #[test]
    fn test_process_monitor_list_sorted() {
        let mut monitor = ProcessMonitor::new();
        let mut process1 = ProcessEntry::new(123, String::from("test"));
        process1.cpu_percent = 10.0;
        monitor.add_process(process1);

        let mut process2 = ProcessEntry::new(124, String::from("test2"));
        process2.cpu_percent = 5.0;
        monitor.add_process(process2);

        let sorted = monitor.list_sorted(ProcessSortField::Cpu);
        assert_eq!(sorted[0].pid, 123); // Higher CPU first
    }

    #[test]
    fn test_process_monitor_show_process() {
        let mut monitor = ProcessMonitor::new();
        let mut process = ProcessEntry::new(123, String::from("test"));
        process.cpu_percent = 5.5;
        process.memory_kb = 1024;
        monitor.add_process(process);

        let details = monitor.show_process(123);
        assert!(details.is_some());
        assert!(details.unwrap().contains("123"));
    }

    #[test]
    fn test_process_monitor_get_statistics() {
        let mut monitor = ProcessMonitor::new();
        let mut process1 = ProcessEntry::new(123, String::from("test"));
        process1.state = MonitoredProcessState::Running;
        process1.cpu_percent = 5.0;
        monitor.add_process(process1);

        let mut process2 = ProcessEntry::new(124, String::from("test2"));
        process2.state = MonitoredProcessState::Sleeping;
        process2.cpu_percent = 3.0;
        monitor.add_process(process2);

        let stats = monitor.get_statistics();
        assert!(stats.contains("Total processes: 2"));
        assert!(stats.contains("Running: 1"));
        assert!(stats.contains("Sleeping: 1"));
    }

    #[test]
    fn test_process_monitor_kill_process() {
        let mut monitor = ProcessMonitor::new();
        monitor.add_process(ProcessEntry::new(123, String::from("test")));

        assert!(monitor.kill_process(123));
        assert_eq!(monitor.processes.len(), 0);
    }

    #[test]
    fn test_process_monitor_set_priority() {
        let mut monitor = ProcessMonitor::new();
        monitor.add_process(ProcessEntry::new(123, String::from("test")));

        assert!(monitor.set_priority(123, 10));
        assert_eq!(monitor.get_process(123).unwrap().priority, 10);
    }

    #[test]
    fn test_process_monitor_set_nice() {
        let mut monitor = ProcessMonitor::new();
        monitor.add_process(ProcessEntry::new(123, String::from("test")));

        assert!(monitor.set_nice(123, 5));
        assert_eq!(monitor.get_process(123).unwrap().nice, 5);
    }
}
