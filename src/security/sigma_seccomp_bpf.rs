#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! # SigmaOS Seccomp-BPF Filter Engine
//!
//! A SigmaOS-native syscall filter engine inspired by Linux seccomp-BPF and
//! OpenBSD `pledge(2)`.  It evaluates an ordered list of [`SeccompRule`]s
//! against a syscall number and its arguments, returning a [`SeccompAction`].
//!
//! ## Usage
//! ```text
//! let mut engine = FilterEngine::new();
//! let filter = SeccompFilter::new()
//!     .allow(SYS_READ)
//!     .allow(SYS_WRITE)
//!     .default_action(SeccompAction::Kill);
//! engine.install_filter(filter);
//! let action = engine.evaluate(SYS_MMAP, &[0; 6]);
//! assert_eq!(action, SeccompAction::Kill);
//! ```
//!
//! ## Comparison with Linux seccomp
//! | Feature            | Linux seccomp-BPF | SigmaOS seccomp-BPF |
//! |--------------------|-------------------|---------------------|
//! | Rule language      | BPF bytecode      | Rust structs        |
//! | Arg comparators    | Yes               | Yes                 |
//! | Multi-filter stack | Yes               | Yes                 |
//! | Audit logging      | via auditd        | Built-in `Log`      |
//! | pledge parity      | No                | `PledgePolicy`      |

#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::vec::Vec;
use std::string::String;

// ── Syscall Numbers (representative subset) ──────────────────────────────────

/// Commonly filtered syscall numbers (x86-64 ABI).
pub mod syscalls {
    pub const SYS_READ: u64          = 0;
    pub const SYS_WRITE: u64         = 1;
    pub const SYS_OPEN: u64          = 2;
    pub const SYS_CLOSE: u64         = 3;
    pub const SYS_STAT: u64          = 4;
    pub const SYS_MMAP: u64          = 9;
    pub const SYS_MPROTECT: u64      = 10;
    pub const SYS_MUNMAP: u64        = 11;
    pub const SYS_BRK: u64           = 12;
    pub const SYS_IOCTL: u64         = 16;
    pub const SYS_SOCKET: u64        = 41;
    pub const SYS_CONNECT: u64       = 42;
    pub const SYS_ACCEPT: u64        = 43;
    pub const SYS_BIND: u64          = 49;
    pub const SYS_LISTEN: u64        = 50;
    pub const SYS_FORK: u64          = 57;
    pub const SYS_EXECVE: u64        = 59;
    pub const SYS_EXIT: u64          = 60;
    pub const SYS_KILL: u64          = 62;
    pub const SYS_PTRACE: u64        = 101;
    pub const SYS_PRCTL: u64         = 157;
}

// ── SeccompAction ─────────────────────────────────────────────────────────────

/// Action to take when a seccomp rule matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeccompAction {
    /// Allow the syscall to proceed.
    Allow,
    /// Kill the offending thread immediately (SIGSYS).
    Kill,
    /// Send SIGSYS to the thread (allows signal handler to run).
    Trap,
    /// Return the specified error code to the caller.
    Errno(i32),
    /// Log the syscall but allow it.
    Log,
    /// Notify a supervising tracer (ptrace).
    Trace,
}

impl SeccompAction {
    /// Return `true` if this action allows the syscall to execute.
    pub fn permits_execution(&self) -> bool {
        matches!(self, SeccompAction::Allow | SeccompAction::Log)
    }
}

// ── Arg Comparator ───────────────────────────────────────────────────────────

/// Index of a syscall argument (0–5).
pub type ArgIndex = usize;

/// A comparison operation on one syscall argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgComparator {
    /// `arg[index] == value`
    Equal { index: ArgIndex, value: u64 },
    /// `arg[index] != value`
    NotEqual { index: ArgIndex, value: u64 },
    /// `arg[index] < value`
    LessThan { index: ArgIndex, value: u64 },
    /// `arg[index] <= value`
    LessEqual { index: ArgIndex, value: u64 },
    /// `arg[index] > value`
    GreaterThan { index: ArgIndex, value: u64 },
    /// `arg[index] >= value`
    GreaterEqual { index: ArgIndex, value: u64 },
    /// `arg[index] & mask == mask` (all bits set)
    MaskedEqual { index: ArgIndex, mask: u64, value: u64 },
}

impl ArgComparator {
    /// Evaluate this comparator against the provided argument array.
    ///
    /// Returns `true` if the comparison holds.
    pub fn matches(&self, args: &[u64; 6]) -> bool {
        match *self {
            ArgComparator::Equal { index, value } => args[index] == value,
            ArgComparator::NotEqual { index, value } => args[index] != value,
            ArgComparator::LessThan { index, value } => args[index] < value,
            ArgComparator::LessEqual { index, value } => args[index] <= value,
            ArgComparator::GreaterThan { index, value } => args[index] > value,
            ArgComparator::GreaterEqual { index, value } => args[index] >= value,
            ArgComparator::MaskedEqual { index, mask, value } => args[index] & mask == value,
        }
    }
}

// ── SeccompRule ───────────────────────────────────────────────────────────────

/// A single seccomp filter rule.
///
/// A rule matches when `syscall_nr` equals the incoming syscall number AND
/// all `comparators` hold for the syscall arguments.  If `comparators` is
/// empty, the rule matches any argument combination for that syscall.
#[derive(Debug, Clone)]
pub struct SeccompRule {
    /// Syscall number this rule applies to.
    pub syscall_nr: u64,
    /// Action to apply when this rule matches.
    pub action: SeccompAction,
    /// Argument conditions (all must hold; empty = unconditional).
    pub comparators: Vec<ArgComparator>,
    /// Optional human-readable comment.
    pub comment: Option<String>,
}

impl SeccompRule {
    /// Create a simple unconditional rule.
    pub fn new(syscall_nr: u64, action: SeccompAction) -> Self {
        SeccompRule {
            syscall_nr,
            action,
            comparators: Vec::new(),
            comment: None,
        }
    }

    /// Add an argument comparator to this rule.
    pub fn with_comparator(mut self, cmp: ArgComparator) -> Self {
        self.comparators.push(cmp);
        self
    }

    /// Add a comment for debugging.
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Test whether this rule matches the given syscall and arguments.
    pub fn matches(&self, syscall_nr: u64, args: &[u64; 6]) -> bool {
        if self.syscall_nr != syscall_nr {
            return false;
        }
        self.comparators.iter().all(|c| c.matches(args))
    }
}

// ── SeccompFilter ─────────────────────────────────────────────────────────────

/// An ordered list of seccomp rules plus a default action.
///
/// Rules are evaluated in order; the first match wins.  If no rule matches,
/// `default_action` is applied.
#[derive(Debug, Clone)]
pub struct SeccompFilter {
    /// Ordered list of rules.  First match wins.
    pub rules: Vec<SeccompRule>,
    /// Action taken when no rule matches.
    pub default_action: SeccompAction,
    /// Human-readable filter name.
    pub name: String,
}

impl SeccompFilter {
    /// Create an empty filter with a `Kill` default action.
    pub fn new(name: impl Into<String>) -> Self {
        SeccompFilter {
            rules: Vec::new(),
            default_action: SeccompAction::Kill,
            name: name.into(),
        }
    }

    /// Set the default (catch-all) action.
    pub fn with_default(mut self, action: SeccompAction) -> Self {
        self.default_action = action;
        self
    }

    /// Add an unconditional allow rule for `syscall_nr`.
    pub fn allow(mut self, syscall_nr: u64) -> Self {
        self.rules.push(SeccompRule::new(syscall_nr, SeccompAction::Allow));
        self
    }

    /// Add an unconditional deny (kill) rule.
    pub fn deny(mut self, syscall_nr: u64) -> Self {
        self.rules.push(SeccompRule::new(syscall_nr, SeccompAction::Kill));
        self
    }

    /// Add an unconditional log-and-allow rule.
    pub fn log(mut self, syscall_nr: u64) -> Self {
        self.rules.push(SeccompRule::new(syscall_nr, SeccompAction::Log));
        self
    }

    /// Add an errno rule.
    pub fn errno(mut self, syscall_nr: u64, errno: i32) -> Self {
        self.rules.push(SeccompRule::new(syscall_nr, SeccompAction::Errno(errno)));
        self
    }

    /// Add an arbitrary rule.
    pub fn add_rule(mut self, rule: SeccompRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Evaluate the filter for `syscall_nr` and `args`.
    ///
    /// Returns the action from the first matching rule, or `default_action`.
    pub fn evaluate(&self, syscall_nr: u64, args: &[u64; 6]) -> SeccompAction {
        for rule in &self.rules {
            if rule.matches(syscall_nr, args) {
                return rule.action.clone();
            }
        }
        self.default_action.clone()
    }
}

// ── FilterEngine ─────────────────────────────────────────────────────────────

/// Per-process seccomp filter engine.
///
/// Supports a stack of installed filters.  Filters are evaluated from the
/// most-recently-installed to the oldest; the most restrictive action wins
/// across the stack (Kill > Trap > Errno > Log > Allow).
#[derive(Debug, Default)]
pub struct FilterEngine {
    /// Stack of installed filters (newest first).
    filters: Vec<SeccompFilter>,
    /// Audit log of matched syscalls.
    audit_log: Vec<AuditEntry>,
}

/// An entry in the seccomp audit log.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    /// Syscall that was evaluated.
    pub syscall_nr: u64,
    /// Action that was applied.
    pub action: SeccompAction,
    /// Name of the filter that matched.
    pub filter_name: String,
}

impl FilterEngine {
    /// Create a new, empty filter engine.
    pub fn new() -> Self {
        FilterEngine {
            filters: Vec::new(),
            audit_log: Vec::new(),
        }
    }

    /// Install a new filter.  The filter is pushed onto the stack.
    ///
    /// Once installed, a filter cannot be removed (Linux parity) — use
    /// `uninstall_filter` for testing/privileged contexts only.
    pub fn install_filter(&mut self, filter: SeccompFilter) {
        self.filters.push(filter);
    }

    /// Remove the most-recently-installed filter.
    ///
    /// **Security note**: in a real kernel this would not be permitted after
    /// `execve`; only available here for testing.
    pub fn uninstall_filter(&mut self) -> Option<SeccompFilter> {
        self.filters.pop()
    }

    /// Evaluate `syscall_nr` with `args` against all installed filters.
    ///
    /// Returns the most restrictive action found across all filters.
    /// Action severity order (most → least restrictive):
    /// `Kill > Trap > Errno > Trace > Log > Allow`
    pub fn evaluate(&mut self, syscall_nr: u64, args: &[u64; 6]) -> SeccompAction {
        if self.filters.is_empty() {
            return SeccompAction::Allow;
        }

        let mut result = SeccompAction::Allow;
        let mut matched_filter_name = String::from("<none>");

        for filter in self.filters.iter().rev() {
            let action = filter.evaluate(syscall_nr, args);
            if Self::action_severity(&action) > Self::action_severity(&result) {
                result = action.clone();
                matched_filter_name = filter.name.clone();
            }
        }

        // Audit log for Log actions or any non-allow.
        if !matches!(result, SeccompAction::Allow) {
            self.audit_log.push(AuditEntry {
                syscall_nr,
                action: result.clone(),
                filter_name: matched_filter_name,
            });
        }

        result
    }

    /// Numeric severity for action comparison (higher = more restrictive).
    fn action_severity(action: &SeccompAction) -> u8 {
        match action {
            SeccompAction::Allow => 0,
            SeccompAction::Log => 1,
            SeccompAction::Trace => 2,
            SeccompAction::Errno(_) => 3,
            SeccompAction::Trap => 4,
            SeccompAction::Kill => 5,
        }
    }

    /// Return a copy of the audit log.
    pub fn audit_log(&self) -> &[AuditEntry] {
        &self.audit_log
    }

    /// Clear the audit log.
    pub fn clear_audit_log(&mut self) {
        self.audit_log.clear();
    }

    /// Number of installed filters.
    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }
}

// ── Pledge-style Policy ───────────────────────────────────────────────────────

/// High-level pledge-style policy builder.
///
/// Generates a [`SeccompFilter`] from a set of capability flags, similar to
/// OpenBSD `pledge(2)`.
#[derive(Debug, Default)]
pub struct PledgePolicy {
    allow_stdio: bool,
    allow_network: bool,
    allow_exec: bool,
    allow_file_read: bool,
    allow_file_write: bool,
    allow_proc: bool,
}

impl PledgePolicy {
    /// Allow stdio syscalls (read, write, close, …).
    pub fn stdio(mut self) -> Self { self.allow_stdio = true; self }
    /// Allow networking syscalls (socket, connect, …).
    pub fn network(mut self) -> Self { self.allow_network = true; self }
    /// Allow exec syscalls.
    pub fn exec(mut self) -> Self { self.allow_exec = true; self }
    /// Allow file-read syscalls.
    pub fn file_read(mut self) -> Self { self.allow_file_read = true; self }
    /// Allow file-write syscalls.
    pub fn file_write(mut self) -> Self { self.allow_file_write = true; self }
    /// Allow process-control syscalls (fork, kill, …).
    pub fn proc(mut self) -> Self { self.allow_proc = true; self }

    /// Build a [`SeccompFilter`] from this policy.
    pub fn build(self, name: impl Into<String>) -> SeccompFilter {
        use syscalls::*;
        let mut f = SeccompFilter::new(name).with_default(SeccompAction::Kill);

        if self.allow_stdio {
            f = f.allow(SYS_READ).allow(SYS_WRITE).allow(SYS_CLOSE).allow(SYS_EXIT);
        }
        if self.allow_file_read {
            f = f.allow(SYS_OPEN).allow(SYS_STAT);
        }
        if self.allow_file_write {
            f = f.allow(SYS_OPEN);
        }
        if self.allow_network {
            f = f.allow(SYS_SOCKET).allow(SYS_CONNECT).allow(SYS_BIND)
                  .allow(SYS_LISTEN).allow(SYS_ACCEPT);
        }
        if self.allow_exec {
            f = f.allow(SYS_EXECVE);
        }
        if self.allow_proc {
            f = f.allow(SYS_FORK).allow(SYS_KILL);
        }
        f
    }
}

// ── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use super::syscalls::*;

    #[test]
    fn test_filter_allow() {
        let filter = SeccompFilter::new("test")
            .allow(SYS_READ)
            .with_default(SeccompAction::Kill);

        let action = filter.evaluate(SYS_READ, &[0; 6]);
        assert_eq!(action, SeccompAction::Allow);

        let action = filter.evaluate(SYS_SOCKET, &[0; 6]);
        assert_eq!(action, SeccompAction::Kill);
    }

    #[test]
    fn test_filter_errno() {
        let filter = SeccompFilter::new("test")
            .errno(SYS_SOCKET, 1) // EPERM
            .with_default(SeccompAction::Allow);

        let action = filter.evaluate(SYS_SOCKET, &[0; 6]);
        assert_eq!(action, SeccompAction::Errno(1));
    }

    #[test]
    fn test_comparator_masked_equal() {
        let cmp = ArgComparator::MaskedEqual { index: 0, mask: 0xFF, value: 0x02 };
        let mut args = [0u64; 6];
        args[0] = 0x1002; // 0x1002 & 0xFF = 0x02
        assert!(cmp.matches(&args));
    }

    #[test]
    fn test_engine_most_restrictive() {
        let mut engine = FilterEngine::new();
        // First filter: allow everything.
        engine.install_filter(
            SeccompFilter::new("allow-all").with_default(SeccompAction::Allow),
        );
        // Second filter: kill socket.
        engine.install_filter(
            SeccompFilter::new("no-socket")
                .deny(SYS_SOCKET)
                .with_default(SeccompAction::Allow),
        );
        // Engine should return Kill for SYS_SOCKET.
        let action = engine.evaluate(SYS_SOCKET, &[0; 6]);
        assert_eq!(action, SeccompAction::Kill);
    }

    #[test]
    fn test_pledge_policy() {
        let filter = PledgePolicy::default()
            .stdio()
            .file_read()
            .build("pledge-test");

        assert_eq!(filter.evaluate(SYS_READ, &[0; 6]), SeccompAction::Allow);
        assert_eq!(filter.evaluate(SYS_SOCKET, &[0; 6]), SeccompAction::Kill);
    }

    #[test]
    fn test_audit_log() {
        let mut engine = FilterEngine::new();
        engine.install_filter(
            SeccompFilter::new("log-test")
                .log(SYS_MMAP)
                .with_default(SeccompAction::Allow),
        );
        let action = engine.evaluate(SYS_MMAP, &[0; 6]);
        assert_eq!(action, SeccompAction::Log);
        assert_eq!(engine.audit_log().len(), 1);
    }
}
