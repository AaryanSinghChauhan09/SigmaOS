#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Modern high-performance NVMe PCIe block storage driver
#[cfg(not(test))]
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeCmd {
    pub opcode: u8,
    pub nsid: u32,
    pub prp1: u64,
    pub prp2: u64,
}

pub struct NvmeSubmissionQueue {
    pub size: usize,
    pub head: usize,
    pub tail: usize,
}

impl NvmeSubmissionQueue {
    pub fn new(size: usize) -> Self {
        Self { size, head: 0, tail: 0 }
    }

    /// Submits a command and increments the doorbell tail pointer (doorbell write)
    pub fn submit_command(&mut self, _cmd: NvmeCmd) -> Result<usize, &'static str> {
        let next_tail = (self.tail + 1) % self.size;
        if next_tail == self.head {
            return Err("Submission queue is full");
        }
        let submitted_idx = self.tail;
        self.tail = next_tail;
        Ok(submitted_idx)
    }
}

pub struct NvmeCompletionQueue {
    pub size: usize,
    pub head: usize,
    pub phase: bool,
}

impl NvmeCompletionQueue {
    pub fn new(size: usize) -> Self {
        Self { size, head: 0, phase: true }
    }

    /// Reaps a completion entry, updating head and toggling phase bit on wrap
    pub fn reap_completion(&mut self) -> (usize, bool) {
        let reaped_head = self.head;
        self.head = (self.head + 1) % self.size;
        if self.head == 0 {
            self.phase = !self.phase; // phase bit flips on wrap
        }
        (reaped_head, self.phase)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmartTelemetry {
    pub temperature_c: u16,
    pub percentage_used: u8,
    pub data_units_read: u64,
    pub data_units_written: u64,
}

impl SmartTelemetry {
    pub fn new() -> Self {
        Self {
            temperature_c: 38, // 38C optimal temperature
            percentage_used: 1, // 1% life used
            data_units_read: 1048576, // 1GB read
            data_units_written: 2097152, // 2GB written
        }
    }
}

impl Default for SmartTelemetry {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulated AHCI SATA Command Header structure (HBA memory layout)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AhciCommandHeader {
    pub opts: u16,
    pub prdtl: u16,
    pub prdbc: u32,
    pub ctba: u64,
}

impl AhciCommandHeader {
    pub const fn new() -> Self {
        Self { opts: 0, prdtl: 0, prdbc: 0, ctba: 0 }
    }
}

/// Simulated AHCI Port MMIO Register Map
pub struct AhciPort {
    pub cmd_issue: u32,
    pub cmd_headers: [AhciCommandHeader; 32], // 32 command slots
}

impl AhciPort {
    pub const fn new() -> Self {
        Self {
            cmd_issue: 0,
            cmd_headers: [AhciCommandHeader::new(); 32],
        }
    }

    /// Allocates an empty slot for NCQ SATA command issues
    pub fn allocate_slot(&mut self) -> Option<usize> {
        for i in 0..32 {
            if (self.cmd_issue & (1 << i)) == 0 {
                self.cmd_issue |= 1 << i;
                return Some(i);
            }
        }
        None
    }

    /// Releases command slot upon SATA controller interrupt completion
    pub fn complete_slot(&mut self, slot: usize) {
        if slot < 32 {
            self.cmd_issue &= !(1 << slot);
        }
    }
}
