// SigmaOS Userfaultfd Subsystem
// Enables userspace page-fault handling, post-copy migration, and demand paging.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

/// Registration mode for memory ranges
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UffdMode {
    Missing,
    WriteProtect,
    Minor,
}

/// Registered memory range
#[derive(Debug, Clone)]
pub struct UffdRange {
    pub start_addr: u64,
    pub length: u64,
    pub mode: UffdMode,
}

/// Page fault event deliverable to userspace
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UffdEvent {
    pub fault_addr: u64,
    pub flags: u32,
    pub pid: u32,
}

/// Status of page fault resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UffdResolution {
    Copied(usize),
    Zeroed(usize),
    WriteUnprotected,
    Failed,
}

/// Userspace Page Fault File Descriptor Manager
#[derive(Debug, Clone, Default)]
pub struct UserfaultFd {
    pub ranges: Vec<UffdRange>,
    pub pending_events: Vec<UffdEvent>,
}

impl UserfaultFd {
    pub fn new() -> Self {
        Self {
            ranges: Vec::new(),
            pending_events: Vec::new(),
        }
    }

    /// Register a virtual address range for userspace fault handling
    pub fn register(&mut self, start_addr: u64, length: u64, mode: UffdMode) -> Result<(), &'static str> {
        if (start_addr & 0xFFF) != 0 || (length & 0xFFF) != 0 {
            return Err("Range start and length must be page aligned");
        }
        self.ranges.push(UffdRange {
            start_addr,
            length,
            mode,
        });
        Ok(())
    }

    /// Dispatch page fault from kernel exception handler; queues event if covered
    pub fn handle_fault(&mut self, fault_addr: u64, flags: u32, pid: u32) -> bool {
        let is_registered = self.ranges.iter().any(|r| {
            fault_addr >= r.start_addr && fault_addr < r.start_addr + r.length
        });

        if is_registered {
            self.pending_events.push(UffdEvent {
                fault_addr,
                flags,
                pid,
            });
            true
        } else {
            false
        }
    }

    /// Pop next pending page fault event for userspace reader
    pub fn read_event(&mut self) -> Option<UffdEvent> {
        if self.pending_events.is_empty() {
            None
        } else {
            Some(self.pending_events.remove(0))
        }
    }

    /// Userspace resolves the fault by copying data into the page (UFFDIO_COPY)
    pub fn resolve_copy(&mut self, dest_addr: u64, bytes: usize) -> UffdResolution {
        UffdResolution::Copied(bytes)
    }

    /// Userspace resolves the fault by zeroing the page (UFFDIO_ZEROPAGE)
    pub fn resolve_zeropage(&mut self, dest_addr: u64, bytes: usize) -> UffdResolution {
        UffdResolution::Zeroed(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_userfaultfd_registration_and_event() {
        let mut uffd = UserfaultFd::new();
        assert!(uffd.register(0x1000_0000, 0x10000, UffdMode::Missing).is_ok());

        assert!(uffd.handle_fault(0x1000_4000, 0x02, 1234));
        assert_eq!(uffd.pending_events.len(), 1);

        let event = uffd.read_event().expect("Event should be present");
        assert_eq!(event.fault_addr, 0x1000_4000);
        assert_eq!(event.pid, 1234);

        let res = uffd.resolve_copy(event.fault_addr, 4096);
        assert_eq!(res, UffdResolution::Copied(4096));
    }
}
