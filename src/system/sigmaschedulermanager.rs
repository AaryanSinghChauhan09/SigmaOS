use alloc::string::String;

#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
}

pub struct SigmaSchedulerManager;
pub type SigmaScheduleManager = SigmaSchedulerManager;
