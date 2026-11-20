//! SigmaOS APT daemon.
//!
//! Background service handling package updates, install queues, and
//! transaction state via D-Bus.

#![warn(missing_docs)]

/// State of a package transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    /// Transaction is queued.
    Queued,
    /// Transaction is downloading.
    Downloading,
    /// Transaction is installing.
    Installing,
    /// Transaction finished successfully.
    Finished,
    /// Transaction failed.
    Failed,
}

/// A package transaction tracked by the daemon.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Identifier for the transaction.
    pub id: String,
    /// Packages affected by the transaction.
    pub packages: Vec<String>,
    /// Current state.
    pub state: TransactionState,
}

/// Trait for the APT daemon backend.
pub trait AptDaemon {
    /// Queue a new install transaction.
    fn install(&mut self, packages: &[String]) -> Result<Transaction, DaemonError>;
    /// Queue an update transaction.
    fn update(&mut self, packages: &[String]) -> Result<Transaction, DaemonError>;
    /// Get the state of a transaction.
    fn state(&self, id: &str) -> Result<TransactionState, DaemonError>;
}

/// Errors returned by the APT daemon.
#[derive(Debug)]
pub enum DaemonError {
    /// A package name was not found.
    PackageNotFound(String),
    /// The transaction failed.
    TransactionFailed(String),
}

impl std::fmt::Display for DaemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DaemonError::PackageNotFound(name) => write!(f, "package not found: {name}"),
            DaemonError::TransactionFailed(msg) => write!(f, "transaction failed: {msg}"),
        }
    }
}

impl std::error::Error for DaemonError {}
