// SigmaOS Kernel Dmesg Ring Buffer & Syslog Logging Subsystem
// Reference: Linux kernel/printk/printk.c, /dev/kmsg, dmesg(1)

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum KernelLogLevel {
    Emerg = 0,   // System is unusable
    Alert = 1,   // Action must be taken immediately
    Crit = 2,    // Critical conditions
    Err = 3,     // Error conditions
    Warning = 4, // Warning conditions
    Notice = 5,  // Normal but significant condition
    Info = 6,    // Informational
    Debug = 7,   // Debug-level messages
}

impl KernelLogLevel {
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Emerg => "<0>",
            Self::Alert => "<1>",
            Self::Crit => "<2>",
            Self::Err => "<3>",
            Self::Warning => "<4>",
            Self::Notice => "<5>",
            Self::Info => "<6>",
            Self::Debug => "<7>",
        }
    }
}

/// Structured Kernel Log Record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelLogRecord {
    pub seqnum: u64,
    pub timestamp_ns: u64,
    pub level: KernelLogLevel,
    pub facility: u8,
    pub message: String,
}

/// Dmesg Circular Ring Buffer
pub struct DmesgRingBuffer {
    capacity: usize,
    buffer: VecDeque<KernelLogRecord>,
    next_seqnum: u64,
    min_console_level: KernelLogLevel,
}

impl DmesgRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: VecDeque::with_capacity(capacity),
            next_seqnum: 1,
            min_console_level: KernelLogLevel::Warning,
        }
    }

    /// Appends a new message to the kernel log ring buffer
    pub fn log(&mut self, level: KernelLogLevel, timestamp_ns: u64, message: &str) {
        let seq = self.next_seqnum;
        self.next_seqnum += 1;

        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }

        self.buffer.push_back(KernelLogRecord {
            seqnum: seq,
            timestamp_ns,
            level,
            facility: 0, // KERN facility
            message: String::from(message),
        });
    }

    /// Read all logs from the buffer
    pub fn read_all(&self) -> Vec<KernelLogRecord> {
        self.buffer.iter().cloned().collect()
    }

    /// Filter logs by maximum log level
    pub fn read_filtered(&self, max_level: KernelLogLevel) -> Vec<KernelLogRecord> {
        self.buffer
            .iter()
            .filter(|r| r.level <= max_level)
            .cloned()
            .collect()
    }

    /// Format record into standard dmesg output line: "[ 1.234567] <level> message"
    pub fn format_record(record: &KernelLogRecord) -> String {
        let sec = record.timestamp_ns / 1_000_000_000;
        let usec = (record.timestamp_ns % 1_000_000_000) / 1000;
        alloc::format!(
            "[{:5}.{:06}] {:?}: {}",
            sec,
            usec,
            record.level,
            record.message
        )
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn count(&self) -> usize {
        self.buffer.len()
    }
}
