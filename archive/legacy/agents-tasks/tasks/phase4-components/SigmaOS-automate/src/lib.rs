//! SigmaOS task automation engine.
//!
//! Defines, schedules, and executes recurring system and application workflows.

#![warn(missing_docs)]

/// A single step within an automation task.
#[derive(Debug, Clone)]
pub struct Step {
    /// Identifier for the action to run.
    pub action_id: String,
    /// Arguments passed to the action.
    pub args: Vec<String>,
}

/// A complete automation task.
#[derive(Debug, Clone)]
pub struct Task {
    /// Name of the task.
    pub name: String,
    /// Cron-style schedule expression.
    pub schedule: String,
    /// Ordered steps to execute.
    pub steps: Vec<Step>,
}

/// Trait for the engine that runs tasks.
pub trait AutomationEngine {
    /// Register a task with the scheduler.
    fn register(&mut self, task: Task) -> Result<(), AutomateError>;
    /// Run a task immediately by name.
    fn run_now(&self, name: &str) -> Result<(), AutomateError>;
}

/// Errors returned by the automation engine.
#[derive(Debug)]
pub enum AutomateError {
    /// The schedule expression was invalid.
    InvalidSchedule,
    /// The task name was not registered.
    TaskNotFound,
}

impl std::fmt::Display for AutomateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutomateError::InvalidSchedule => write!(f, "invalid schedule"),
            AutomateError::TaskNotFound => write!(f, "task not found"),
        }
    }
}

impl std::error::Error for AutomateError {}
