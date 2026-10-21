use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetupStepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct SetupStep {
    pub name: String,
    pub status: SetupStepStatus,
}

pub struct SigmaSetupManager2;
