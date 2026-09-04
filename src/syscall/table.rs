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

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SyscallNumber = u64;
pub type SyscallHandler = fn(u64, u64, u64, u64, u64, u64) -> i64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallError { Success = 0, NotRegistered = 1, InvalidArgs = 2 }

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
}

#[repr(C)]
pub struct SimpleSyscallTable {
    pub entries: Vec<Option<Box<dyn SyscallEntry>>>,
}

impl SimpleSyscallTable {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleSyscallTable {
            entries: Vec::new(),
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
    fn test_simple_syscall_filter() {
        let mut filter = SimpleSyscallFilter::new();
        filter.allow(1);
        filter.deny(2);

        assert!(filter.is_allowed(1));
        assert!(!filter.is_allowed(2));
        assert!(filter.is_allowed(3));
    }
}
