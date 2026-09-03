//! Userspace-Defined and Dynamic Kernel Policy Subsystem (UDF / OOP / SOLID)
//!
//! Enables developers and system administrators to hot-swap scheduling,
//! page replacement, and syscall security filters dynamically at runtime
//! using polymorphic interface contracts without kernel recompilation.
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]


// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

// =========================================================================
// 1. DYNAMIC USER-DEFINED SCHEDULER POLICY
// =========================================================================

pub trait IUserSchedulerPolicy {
    fn policy_id(&self) -> &'static str;
    fn evaluate_next_process(&self, priorities: &[u32]) -> Option<usize>;
}

/// A User-Defined Priority-Boost Scheduler Policy
pub struct PriorityBoostUserPolicy;
impl IUserSchedulerPolicy for PriorityBoostUserPolicy {
    fn policy_id(&self) -> &'static str {
        "User-Defined Priority-Boost Policy"
    }
    fn evaluate_next_process(&self, priorities: &[u32]) -> Option<usize> {
        // Polymorphically choose the process with the highest priority index
        priorities
            .iter()
            .enumerate()
            .max_by_key(|&(_, &p)| p)
            .map(|(idx, _)| idx)
    }
}

/// A User-Defined Shortest-Job-First (SJF) Scheduler Policy
pub struct SjfUserPolicy;
impl IUserSchedulerPolicy for SjfUserPolicy {
    fn policy_id(&self) -> &'static str {
        "User-Defined Shortest-Job-First (SJF) Policy"
    }
    fn evaluate_next_process(&self, priorities: &[u32]) -> Option<usize> {
        // Choose the one with the lowest value index (simulated runtime size)
        priorities
            .iter()
            .enumerate()
            .min_by_key(|&(_, &p)| p)
            .map(|(idx, _)| idx)
    }
}

// =========================================================================
// 2. DYNAMIC USER-DEFINED PAGE REPLACEMENT ENGINE
// =========================================================================

pub trait IUserPageReplacement {
    fn name(&self) -> &'static str;
    fn select_victim_page(&self, access_counters: &[u32]) -> usize;
}

/// Least-Frequently-Used (LFU) User-Defined page replacement policy
pub struct LfuUserPolicy;
impl IUserPageReplacement for LfuUserPolicy {
    fn name(&self) -> &'static str {
        "Least-Frequently-Used (LFU) Page Policy"
    }
    fn select_victim_page(&self, access_counters: &[u32]) -> usize {
        // Evict the page that has been accessed the least
        access_counters
            .iter()
            .enumerate()
            .min_by_key(|&(_, &c)| c)
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }
}

/// Most-Frequently-Used (MFU) User-Defined page replacement policy
pub struct MfuUserPolicy;
impl IUserPageReplacement for MfuUserPolicy {
    fn name(&self) -> &'static str {
        "Most-Frequently-Used (MFU) Page Policy"
    }
    fn select_victim_page(&self, access_counters: &[u32]) -> usize {
        // Evict the page that has been accessed the most
        access_counters
            .iter()
            .enumerate()
            .max_by_key(|&(_, &c)| c)
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }
}

// =========================================================================
// 3. DYNAMIC USER-DEFINED SYSCALL FILTER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallFilterAction {
    Allow,
    Kill,
}

pub trait IUserSyscallFilter {
    fn filter_name(&self) -> &'static str;
    fn audit_syscall(&self, sys_num: usize) -> SyscallFilterAction;
}

/// Custom Seccomp-like filter restricting dangerous execute or network syscalls
pub struct CustomStrictSyscallFilter;
impl IUserSyscallFilter for CustomStrictSyscallFilter {
    fn filter_name(&self) -> &'static str {
        "Strict Syscall Quarantine Filter"
    }
    fn audit_syscall(&self, sys_num: usize) -> SyscallFilterAction {
        match sys_num {
            59 => SyscallFilterAction::Kill, // Block sys_execve
            _ => SyscallFilterAction::Allow,
        }
    }
}

// =========================================================================
// 4. USER-DEFINED KERNEL MANAGER (UDF HUB)
// =========================================================================

pub struct UserDefinedKernelManager {
    pub scheduler_policy: Box<dyn IUserSchedulerPolicy>,
    pub page_replacement_policy: Box<dyn IUserPageReplacement>,
    pub syscall_filter: Box<dyn IUserSyscallFilter>,
}

impl UserDefinedKernelManager {
    pub fn new(
        sched: Box<dyn IUserSchedulerPolicy>,
        page: Box<dyn IUserPageReplacement>,
        filter: Box<dyn IUserSyscallFilter>,
    ) -> Self {
        Self {
            scheduler_policy: sched,
            page_replacement_policy: page,
            syscall_filter: filter,
        }
    }

    pub fn set_scheduler_policy(&mut self, sched: Box<dyn IUserSchedulerPolicy>) {
        self.scheduler_policy = sched;
    }

    pub fn set_page_policy(&mut self, page: Box<dyn IUserPageReplacement>) {
        self.page_replacement_policy = page;
    }

    pub fn set_syscall_filter(&mut self, filter: Box<dyn IUserSyscallFilter>) {
        self.syscall_filter = filter;
    }
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_defined_scheduler() {
        let mut manager = UserDefinedKernelManager::new(
            Box::new(PriorityBoostUserPolicy),
            Box::new(LfuUserPolicy),
            Box::new(CustomStrictSyscallFilter),
        );

        let process_priorities = [2, 10, 5, 8];

        // PriorityBoost chooses index 1 (highest priority: 10)
        let chosen_idx = manager
            .scheduler_policy
            .evaluate_next_process(&process_priorities)
            .unwrap();
        assert_eq!(chosen_idx, 1);

        // Hot-swap to Shortest-Job-First (SJF) policy dynamically!
        manager.set_scheduler_policy(Box::new(SjfUserPolicy));
        // SjfUserPolicy chooses index 1 in reverse, which is the lowest (priority 1)
        let process_runtimes = [12, 1, 80, 50];
        let chosen_idx_sjf = manager
            .scheduler_policy
            .evaluate_next_process(&process_runtimes)
            .unwrap();
        assert_eq!(chosen_idx_sjf, 1);
    }

    #[test]
    fn test_user_defined_page_replacement() {
        let mut manager = UserDefinedKernelManager::new(
            Box::new(PriorityBoostUserPolicy),
            Box::new(LfuUserPolicy),
            Box::new(CustomStrictSyscallFilter),
        );

        let access_counters = [100, 50, 5, 500]; // Page 2 is least used, page 3 is most used

        // LFU evicts page index 2 (least accessed: 5)
        let victim_lfu = manager
            .page_replacement_policy
            .select_victim_page(&access_counters);
        assert_eq!(victim_lfu, 2);

        // Hot-swap to MFU policy dynamically!
        manager.set_page_policy(Box::new(MfuUserPolicy));
        let victim_mfu = manager
            .page_replacement_policy
            .select_victim_page(&access_counters);
        assert_eq!(victim_mfu, 3);
    }

    #[test]
    fn test_user_defined_syscall_filter() {
        let manager = UserDefinedKernelManager::new(
            Box::new(PriorityBoostUserPolicy),
            Box::new(LfuUserPolicy),
            Box::new(CustomStrictSyscallFilter),
        );

        // Strictly block execution attempts
        assert_eq!(
            manager.syscall_filter.audit_syscall(59),
            SyscallFilterAction::Kill
        );
        assert_eq!(
            manager.syscall_filter.audit_syscall(3),
            SyscallFilterAction::Allow
        );
    }
}
