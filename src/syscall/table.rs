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
// #![no_main]  // crate-root only

/// OOP-based Syscall Table for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 111
/// Implements syscall registration and dispatch table
/// Linux/BSD-inspired with seccomp/BPF filtering and pledge/unveil integration

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SyscallNumber = u64;
pub type SyscallHandler = fn(u64, u64, u64, u64, u64, u64) -> i64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallError { Success = 0, NotRegistered = 1, InvalidArgs = 2, PermissionDenied = 3, Filtered = 4 }

// Linux seccomp/BPF-inspired syscall filtering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallFilterAction {
    Allow = 0,
    Deny = 1,
    Trap = 2,
    KillProcess = 3,
    Trace = 4,
    Log = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SyscallFilterRule {
    pub syscall_number: SyscallNumber,
    pub action: SyscallFilterAction,
}

// OpenBSD pledge-inspired promise bits
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PledgePromises {
    pub stdio: bool,
    pub rpath: bool,
    pub wpath: bool,
    pub cpath: bool,
    pub dpath: bool,
    pub exec: bool,
    pub prot_exec: bool,
    pub unix: bool,
    pub inet: bool,
    pub dns: bool,
}

impl PledgePromises {
    pub fn new() -> Self {
        PledgePromises {
            stdio: true,
            rpath: false,
            wpath: false,
            cpath: false,
            dpath: false,
            exec: false,
            prot_exec: false,
            unix: false,
            inet: false,
            dns: false,
        }
    }
}

pub trait SyscallEntry {
    fn number(&self) -> SyscallNumber;
    fn name(&self) -> &[u8];
    fn handler(&self) -> SyscallHandler;
}

#[repr(C)]
pub struct SimpleSyscallEntry {
    pub number: SyscallNumber,
    pub name: [u8; 64],
    pub name_len: u8,
    pub handler: SyscallHandler,
}

impl SimpleSyscallEntry {
    pub fn new(number: SyscallNumber, name: &[u8], handler: SyscallHandler) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63) as u8;
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len as usize);
        }
        SimpleSyscallEntry {
            number,
            name: name_array,
            name_len,
            handler,
        }
    }
}

impl SyscallEntry for SimpleSyscallEntry {
    fn number(&self) -> SyscallNumber { self.number }
    fn name(&self) -> &[u8] {
        &self.name[..self.name_len as usize]
    }
    fn handler(&self) -> SyscallHandler { self.handler }
}

pub trait SyscallTable {
    fn register(&mut self, entry: Box<dyn SyscallEntry>) -> Result<(), SyscallError>;
    fn unregister(&mut self, number: SyscallNumber) -> Result<(), SyscallError>;
    fn get_handler(&self, number: SyscallNumber) -> Option<SyscallHandler>;
    fn list_syscalls(&self) -> Vec<SyscallNumber>;
    fn add_filter_rule(&mut self, rule: SyscallFilterRule) -> Result<(), SyscallError>;
    fn remove_filter_rule(&mut self, syscall_number: SyscallNumber) -> Result<(), SyscallError>;
    fn set_pledge_promises(&mut self, promises: PledgePromises) -> Result<(), SyscallError>;
    fn check_permission(&self, syscall_number: SyscallNumber) -> Result<(), SyscallError>;
}

#[repr(C)]
pub struct SimpleSyscallTable {
    pub entries: Vec<Option<Box<dyn SyscallEntry>>>,
    pub filter_rules: Vec<SyscallFilterRule>,
    pub pledge_promises: PledgePromises,
    pub filter_enabled: bool,
}

impl SimpleSyscallTable {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleSyscallTable {
            entries: Vec::new(),
            filter_rules: Vec::new(),
            pledge_promises: PledgePromises::new(),
            filter_enabled: false,
        }
    }

    pub fn register_common(&mut self) {
        let read_handler: SyscallHandler = |a, b, c, d, e, f| {
            (a + b + c + d + e + f) as i64
        };
        let read_entry = SimpleSyscallEntry::new(0, b"read", read_handler);
        self.entries.push(Some(Box::new(read_entry)));

        let write_handler: SyscallHandler = |a, b, c, d, e, f| {
            (a + b + c + d + e + f) as i64
        };
        let write_entry = SimpleSyscallEntry::new(1, b"write", write_handler);
        self.entries.push(Some(Box::new(write_entry)));

        let open_handler: SyscallHandler = |a, b, c, d, e, f| {
            (a + b + c + d + e + f) as i64
        };
        let open_entry = SimpleSyscallEntry::new(2, b"open", open_handler);
        self.entries.push(Some(Box::new(open_entry)));

        let close_handler: SyscallHandler = |a, b, c, d, e, f| {
            (a + b + c + d + e + f) as i64
        };
        let close_entry = SimpleSyscallEntry::new(3, b"close", close_handler);
        self.entries.push(Some(Box::new(close_entry)));

        let exit_handler: SyscallHandler = |a, b, c, d, e, f| {
            (a + b + c + d + e + f) as i64
        };
        let exit_entry = SimpleSyscallEntry::new(60, b"exit", exit_handler);
        self.entries.push(Some(Box::new(exit_entry)));
    }
}

impl SyscallTable for SimpleSyscallTable {
    fn register(&mut self, entry: Box<dyn SyscallEntry>) -> Result<(), SyscallError> {
        self.entries.push(Some(entry));
        Ok(())
    }

    fn unregister(&mut self, number: SyscallNumber) -> Result<(), SyscallError> {
        for entry_option in &mut self.entries {
            if let Some(ref entry) = *entry_option {
                if entry.number() == number {
                    return Ok(());
                }
            }
        }
        Err(SyscallError::NotRegistered)
    }

    fn get_handler(&self, number: SyscallNumber) -> Option<SyscallHandler> {
        for entry_option in &self.entries {
            if let Some(ref entry) = *entry_option {
                if entry.number() == number {
                    return Some(entry.handler());
                }
            }
        }
        None
    }

    fn list_syscalls(&self) -> Vec<SyscallNumber> {
        let mut numbers = Vec::new();
        for entry_option in &self.entries {
            if let Some(ref entry) = *entry_option {
                numbers.push(entry.number());
            }
        }
        numbers
    }

    fn add_filter_rule(&mut self, rule: SyscallFilterRule) -> Result<(), SyscallError> {
        self.filter_rules.push(rule);
        self.filter_enabled = true;
        Ok(())
    }

    fn remove_filter_rule(&mut self, syscall_number: SyscallNumber) -> Result<(), SyscallError> {
        if let Some(pos) = self.filter_rules.iter().position(|r| r.syscall_number == syscall_number) {
            self.filter_rules.remove(pos);
            Ok(())
        } else {
            Err(SyscallError::NotRegistered)
        }
    }

    fn set_pledge_promises(&mut self, promises: PledgePromises) -> Result<(), SyscallError> {
        self.pledge_promises = promises;
        Ok(())
    }

    fn check_permission(&self, syscall_number: SyscallNumber) -> Result<(), SyscallError> {
        if !self.filter_enabled {
            return Ok(());
        }

        for rule in &self.filter_rules {
            if rule.syscall_number == syscall_number {
                match rule.action {
                    SyscallFilterAction::Allow => return Ok(()),
                    SyscallFilterAction::Deny => return Err(SyscallError::PermissionDenied),
                    SyscallFilterAction::Trap => return Err(SyscallError::Filtered),
                    SyscallFilterAction::KillProcess => return Err(SyscallError::Filtered),
                    SyscallFilterAction::Trace => return Ok(()),
                    SyscallFilterAction::Log => return Ok(()),
                }
            }
        }

        // Check pledge promises
        match syscall_number {
            0 | 1 => { // read, write
                if !self.pledge_promises.stdio {
                    return Err(SyscallError::PermissionDenied);
                }
            }
            2 => { // open
                if !self.pledge_promises.rpath && !self.pledge_promises.wpath {
                    return Err(SyscallError::PermissionDenied);
                }
            }
            60 => { // exit
                if !self.pledge_promises.stdio {
                    return Err(SyscallError::PermissionDenied);
                }
            }
            _ => {}
        }

        Ok(())
    }
}

pub trait SyscallFilter {
    fn allow(&mut self, number: SyscallNumber);
    fn deny(&mut self, number: SyscallNumber);
    fn is_allowed(&self, number: SyscallNumber) -> bool;
}

#[repr(C)]
pub struct SimpleSyscallFilter {
    pub allowed: Vec<SyscallNumber>,
    pub denied: Vec<SyscallNumber>,
}

impl SimpleSyscallFilter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleSyscallFilter {
            allowed: Vec::new(),
            denied: Vec::new(),
        }
    }
}

impl SyscallFilter for SimpleSyscallFilter {
    fn allow(&mut self, number: SyscallNumber) {
        self.allowed.push(number);
    }

    fn deny(&mut self, number: SyscallNumber) {
        self.denied.push(number);
    }

    fn is_allowed(&self, number: SyscallNumber) -> bool {
        for &n in &self.denied {
            if n == number {
                return false;
            }
        }
        true
    }
}

pub trait SyscallAuditor {
    fn log_call(&mut self, number: SyscallNumber, args: [u64; 6], result: i64);
    fn get_log(&self) -> Vec<(SyscallNumber, [u64; 6], i64)>;
}

#[repr(C)]
pub struct SimpleSyscallAuditor {
    pub log: Vec<(SyscallNumber, [u64; 6], i64)>,
}

impl SimpleSyscallAuditor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleSyscallAuditor {
            log: Vec::new(),
        }
    }
}

impl SyscallAuditor for SimpleSyscallAuditor {
    fn log_call(&mut self, number: SyscallNumber, args: [u64; 6], result: i64) {
        self.log.push((number, args, result));
    }

    fn get_log(&self) -> Vec<(SyscallNumber, [u64; 6], i64)> {
        self.log.clone()
    }
}

use std::vec::Vec;
use std::boxed::Box;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_syscall_entry_cached_len() {
        let handler: SyscallHandler = |a, b, c, d, e, f| (a + b) as i64;
        let entry = SimpleSyscallEntry::new(10, b"sys_read", handler);

        assert_eq!(entry.number(), 10);
        assert_eq!(entry.name(), b"sys_read");
        assert_eq!(entry.name_len, 8);
        assert_eq!(entry.handler()(2, 3, 0, 0, 0, 0), 5);
    }

    #[test]
    fn test_syscall_filter_rules() {
        let mut table = SimpleSyscallTable::new();
        table.register_common();

        // Add a deny rule for open (syscall 2)
        let deny_rule = SyscallFilterRule {
            syscall_number: 2,
            action: SyscallFilterAction::Deny,
        };
        table.add_filter_rule(deny_rule).unwrap();

        // Check that open is denied
        assert!(table.check_permission(2).is_err());
        
        // Check that read is still allowed
        assert!(table.check_permission(0).is_ok());
    }

    #[test]
    fn test_pledge_promises() {
        let mut table = SimpleSyscallTable::new();
        table.register_common();

        // Set restrictive promises (no stdio)
        let promises = PledgePromises {
            stdio: false,
            rpath: false,
            wpath: false,
            cpath: false,
            dpath: false,
            exec: false,
            prot_exec: false,
            unix: false,
            inet: false,
            dns: false,
        };
        table.set_pledge_promises(promises).unwrap();

        // Check that read is denied due to stdio promise
        assert!(table.check_permission(0).is_err());
        
        // Allow stdio
        let promises = PledgePromises {
            stdio: true,
            rpath: false,
            wpath: false,
            cpath: false,
            dpath: false,
            exec: false,
            prot_exec: false,
            unix: false,
            inet: false,
            dns: false,
        };
        table.set_pledge_promises(promises).unwrap();

        // Check that read is now allowed
        assert!(table.check_permission(0).is_ok());
    }

    #[test]
    fn test_syscall_filter_actions() {
        let mut table = SimpleSyscallTable::new();
        table.register_common();

        // Test allow action
        let allow_rule = SyscallFilterRule {
            syscall_number: 0,
            action: SyscallFilterAction::Allow,
        };
        table.add_filter_rule(allow_rule).unwrap();
        assert!(table.check_permission(0).is_ok());

        // Test deny action
        let deny_rule = SyscallFilterRule {
            syscall_number: 1,
            action: SyscallFilterAction::Deny,
        };
        table.add_filter_rule(deny_rule).unwrap();
        assert!(table.check_permission(1).is_err());

        // Test trap action
        let trap_rule = SyscallFilterRule {
            syscall_number: 2,
            action: SyscallFilterAction::Trap,
        };
        table.add_filter_rule(trap_rule).unwrap();
        assert!(table.check_permission(2).is_err());
    }

    #[test]
    fn test_filter_rule_removal() {
        let mut table = SimpleSyscallTable::new();
        table.register_common();

        // Add a deny rule
        let deny_rule = SyscallFilterRule {
            syscall_number: 2,
            action: SyscallFilterAction::Deny,
        };
        table.add_filter_rule(deny_rule).unwrap();
        assert!(table.check_permission(2).is_err());

        // Remove the rule
        table.remove_filter_rule(2).unwrap();
        
        // Check that the syscall is now allowed
        assert!(table.check_permission(2).is_ok());
    }

    #[test]
    fn test_simple_syscall_filter() {
        let mut filter = SimpleSyscallFilter::new();
        filter.allow(1);
        filter.deny(2);

        assert!(filter.is_allowed(1));
        assert!(!filter.is_allowed(2));
        assert!(filter.is_allowed(3));
    }
}
