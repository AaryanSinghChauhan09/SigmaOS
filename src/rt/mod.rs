//! Real-Time Capabilities (PREEMPT_RT Inspiration)
//! Real-time kernel, scheduling policies, and industrial support
extern crate alloc;



use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Scheduling policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingPolicy {
    Fifo,
    RoundRobin,
    Deadline,
    RealTime,
}

/// Real-time task
#[derive(Debug, Clone)]
pub struct RealTimeTask {
    pub id: String,
    pub name: String,
    pub priority: u32,
    pub deadline: u64,
    pub period: u64,
    pub execution_time: u64,
    pub state: TaskState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Completed,
    MissedDeadline,
}

impl RealTimeTask {
    pub fn new(name: &str, priority: u32, deadline: u64, period: u64) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            priority,
            deadline,
            period,
            execution_time: 0,
            state: TaskState::Ready,
        }
    }

    fn generate_id() -> String {
        "task_abcdef1234567890".to_string()
    }

    pub fn start(&mut self) {
        self.state = TaskState::Running;
    }

    pub fn complete(&mut self) {
        self.state = TaskState::Completed;
    }

    pub fn miss_deadline(&mut self) {
        self.state = TaskState::MissedDeadline;
    }
}

/// Latency monitor
#[derive(Debug, Clone)]
pub struct LatencyMonitor {
    pub measurements: Vec<LatencyMeasurement>,
}

#[derive(Debug, Clone)]
pub struct LatencyMeasurement {
    pub task_id: String,
    pub timestamp: u64,
    pub latency_ns: u64,
    pub deadline_missed: bool,
}

impl LatencyMonitor {
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }

    pub fn add_measurement(&mut self, measurement: LatencyMeasurement) {
        self.measurements.push(measurement);
    }

    pub fn get_max_latency(&self) -> u64 {
        self.measurements.iter().map(|m| m.latency_ns).max().unwrap_or(0)
    }

    pub fn get_avg_latency(&self) -> u64 {
        if self.measurements.is_empty() {
            return 0;
        }
        let sum: u64 = self.measurements.iter().map(|m| m.latency_ns).sum();
        sum / self.measurements.len() as u64
    }

    pub fn get_deadline_miss_rate(&self) -> f64 {
        if self.measurements.is_empty() {
            return 0.0;
        }
        let missed = self.measurements.iter().filter(|m| m.deadline_missed).count();
        (missed as f64 / self.measurements.len() as f64) * 100.0
    }
}

/// Timing analyzer
#[derive(Debug, Clone)]
pub struct TimingAnalyzer {
    pub worst_case_execution_times: Vec<(String, u64)>,
}

impl TimingAnalyzer {
    pub fn new() -> Self {
        Self {
            worst_case_execution_times: Vec::new(),
        }
    }

    pub fn add_wcet(&mut self, task_name: &str, wcet: u64) {
        self.worst_case_execution_times.push((task_name.to_string(), wcet));
    }

    pub fn get_wcet(&self, task_name: &str) -> Option<u64> {
        self.worst_case_execution_times
            .iter()
            .find(|(name, _)| name == task_name)
            .map(|(_, wcet)| *wcet)
    }

    pub fn analyze_schedulability(&self, tasks: &[RealTimeTask]) -> bool {
        // Rate monotonic analysis
        let total_utilization: f64 = tasks.iter()
            .map(|t| t.execution_time as f64 / t.period as f64)
            .sum();
        
        total_utilization <= 1.0
    }
}

/// SigmaRT - Real-Time System
pub struct SigmaRT {
    pub tasks: Vec<RealTimeTask>,
    pub scheduling_policy: SchedulingPolicy,
    pub latency_monitor: LatencyMonitor,
    pub timing_analyzer: TimingAnalyzer,
}

impl SigmaRT {
    pub fn new(scheduling_policy: SchedulingPolicy) -> Self {
        Self {
            tasks: Vec::new(),
            scheduling_policy,
            latency_monitor: LatencyMonitor::new(),
            timing_analyzer: TimingAnalyzer::new(),
        }
    }

    pub fn add_task(&mut self, task: RealTimeTask) {
        self.tasks.push(task);
    }

    pub fn get_task(&mut self, id: &str) -> Option<&mut RealTimeTask> {
        self.tasks.iter_mut().find(|t| t.id == id || t.name == id)
    }

    pub fn schedule(&mut self) -> Result<(), RTError> {
        // Schedule tasks based on policy
        match self.scheduling_policy {
            SchedulingPolicy::Fifo => self.schedule_fifo(),
            SchedulingPolicy::RoundRobin => self.schedule_rr(),
            SchedulingPolicy::Deadline => self.schedule_deadline(),
            SchedulingPolicy::RealTime => self.schedule_rt(),
        }
    }

    fn schedule_fifo(&mut self) -> Result<(), RTError> {
        for task in &mut self.tasks {
            if task.state == TaskState::Ready {
                task.start();
                task.complete();
            }
        }
        Ok(())
    }

    fn schedule_rr(&mut self) -> Result<(), RTError> {
        // Round-robin scheduling
        let mut index = 0;
        loop {
            if let Some(task) = self.tasks.get_mut(index) {
                if task.state == TaskState::Ready {
                    task.start();
                    task.complete();
                }
            }
            index = (index + 1) % self.tasks.len();
            if index == 0 {
                break;
            }
        }
        Ok(())
    }

    fn schedule_deadline(&mut self) -> Result<(), RTError> {
        // Deadline scheduling (earliest deadline first)
        let mut sorted_tasks: Vec<&mut RealTimeTask> = self.tasks.iter_mut().collect();
        sorted_tasks.sort_by_key(|t| t.deadline);
        
        for task in sorted_tasks {
            if task.state == TaskState::Ready {
                task.start();
                task.complete();
            }
        }
        Ok(())
    }

    fn schedule_rt(&mut self) -> Result<(), RTError> {
        // Real-time scheduling (priority-based)
        let mut sorted_tasks: Vec<&mut RealTimeTask> = self.tasks.iter_mut().collect();
        sorted_tasks.sort_by_key(|t| t.priority);
        
        for task in sorted_tasks {
            if task.state == TaskState::Ready {
                task.start();
                task.complete();
            }
        }
        Ok(())
    }

    pub fn monitor_latency(&mut self) {
        // Monitor task latency
        for task in &self.tasks {
            let measurement = LatencyMeasurement {
                task_id: task.id.clone(),
                timestamp: 0,
                latency_ns: task.execution_time,
                deadline_missed: task.state == TaskState::MissedDeadline,
            };
            self.latency_monitor.add_measurement(measurement);
        }
    }

    pub fn get_rt_stats(&self) -> RTStats {
        RTStats {
            total_tasks: self.tasks.len(),
            ready_tasks: self.tasks.iter().filter(|t| t.state == TaskState::Ready).count(),
            running_tasks: self.tasks.iter().filter(|t| t.state == TaskState::Running).count(),
            completed_tasks: self.tasks.iter().filter(|t| t.state == TaskState::Completed).count(),
            missed_deadlines: self.tasks.iter().filter(|t| t.state == TaskState::MissedDeadline).count(),
            max_latency: self.latency_monitor.get_max_latency(),
            avg_latency: self.latency_monitor.get_avg_latency(),
            deadline_miss_rate: self.latency_monitor.get_deadline_miss_rate(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RTStats {
    pub total_tasks: usize,
    pub ready_tasks: usize,
    pub running_tasks: usize,
    pub completed_tasks: usize,
    pub missed_deadlines: usize,
    pub max_latency: u64,
    pub avg_latency: u64,
    pub deadline_miss_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RTError {
    TaskNotFound,
    SchedulingFailed,
    DeadlineMissed,
    InsufficientResources,
}

impl Default for SigmaRT {
    fn default() -> Self {
        Self::new(SchedulingPolicy::RealTime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_time_task() {
        let task = RealTimeTask::new("test-task", 10, 1000, 5000);
        assert_eq!(task.name, "test-task");
        assert_eq!(task.priority, 10);
    }

    #[test]
    fn test_latency_monitor() {
        let mut monitor = LatencyMonitor::new();
        let measurement = LatencyMeasurement {
            task_id: "task-1".to_string(),
            timestamp: 0,
            latency_ns: 100,
            deadline_missed: false,
        };
        monitor.add_measurement(measurement);
        assert_eq!(monitor.get_max_latency(), 100);
    }

    #[test]
    fn test_timing_analyzer() {
        let mut analyzer = TimingAnalyzer::new();
        analyzer.add_wcet("task-1", 1000);
        assert_eq!(analyzer.get_wcet("task-1"), Some(1000));
    }

    #[test]
    fn test_sigmart() {
        let mut rt = SigmaRT::new(SchedulingPolicy::RealTime);
        let task = RealTimeTask::new("test-task", 10, 1000, 5000);
        rt.add_task(task);
        assert!(rt.schedule().is_ok());
    }
}