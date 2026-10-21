use alloc::string::String;

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessPriority {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
}

pub struct SigmaProcessManager;
