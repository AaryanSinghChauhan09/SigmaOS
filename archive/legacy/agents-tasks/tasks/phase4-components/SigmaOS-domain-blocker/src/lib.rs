//! SigmaOS network domain blocker.
//!
//! Applies DNS-based and hosts-file filtering to block ads, trackers, and
//! malicious domains system-wide.

#![warn(missing_docs)]

/// A blocking rule.
#[derive(Debug, Clone)]
pub struct BlockRule {
    /// Domain to block.
    pub domain: String,
    /// Optional comment about why the domain is blocked.
    pub reason: Option<String>,
}

/// A blocklist subscription source.
#[derive(Debug, Clone)]
pub struct Blocklist {
    /// Name of the list.
    pub name: String,
    /// URL to fetch updates from.
    pub url: String,
    /// Rules contained in the list.
    pub rules: Vec<BlockRule>,
}

/// Trait for the domain blocker backend.
pub trait DomainBlocker {
    /// Add a manual block rule.
    fn block(&mut self, rule: BlockRule) -> Result<(), BlockerError>;
    /// Remove a block rule by domain.
    fn unblock(&mut self, domain: &str) -> Result<(), BlockerError>;
    /// Subscribe to a blocklist.
    fn subscribe(&mut self, list: Blocklist) -> Result<(), BlockerError>;
}

/// Errors returned by the domain blocker.
#[derive(Debug)]
pub enum BlockerError {
    /// The blocklist URL could not be fetched.
    FetchFailed(String),
    /// The rule was not found.
    RuleNotFound,
}

impl std::fmt::Display for BlockerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockerError::FetchFailed(msg) => write!(f, "fetch failed: {msg}"),
            BlockerError::RuleNotFound => write!(f, "rule not found"),
        }
    }
}

impl std::error::Error for BlockerError {}
