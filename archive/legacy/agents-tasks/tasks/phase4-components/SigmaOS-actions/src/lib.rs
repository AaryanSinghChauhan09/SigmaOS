//! SigmaOS system action dispatcher.
//!
//! Routes file and application actions to the appropriate handler,
//! including open-with menus and default app associations.

#![warn(missing_docs)]

/// A system action that can be dispatched.
#[derive(Debug, Clone)]
pub struct Action {
    /// Identifier for the action.
    pub id: String,
    /// Human-readable display name.
    pub label: String,
    /// Executable or command to run.
    pub exec: String,
}

/// Trait for objects that can register and dispatch actions.
pub trait ActionDispatcher {
    /// Register a new action.
    fn register(&mut self, action: Action);
    /// Dispatch an action by id with the given file targets.
    fn dispatch(&self, id: &str, targets: &[String]) -> Result<(), ActionError>;
}

/// Errors returned when dispatching actions.
#[derive(Debug)]
pub enum ActionError {
    /// The requested action id was not found.
    NotFound,
    /// The underlying command failed to spawn.
    SpawnFailed(String),
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionError::NotFound => write!(f, "action not found"),
            ActionError::SpawnFailed(msg) => write!(f, "spawn failed: {msg}"),
        }
    }
}

impl std::error::Error for ActionError {}
