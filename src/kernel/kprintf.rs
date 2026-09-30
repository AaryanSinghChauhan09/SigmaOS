// SigmaOS Kernel Printf (kprintf) — Formatted Kernel Output
// Inspired by Linux kernel/printk/printk.c
//
// Provides printk-style formatted output with log levels,
// a kernel log ring buffer, and hex dump utilities.

use std::fmt;
use std::fmt::Write;

// ──────────────────────────── Log Levels ──────────────────────────────────────

/// Kernel log levels matching Linux's printk levels (0 = most critical)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum KernelLogLevel {
    Emergency = 0, // System is unusable
    Alert = 1,     // Action must be taken immediately
    Critical = 2,  // Critical conditions
    Error = 3,     // Error conditions
    Warning = 4,   // Warning conditions
    Notice = 5,    // Normal but significant condition
    Info = 6,      // Informational
    Debug = 7,     // Debug-level messages
}

impl KernelLogLevel {
    /// Linux-style printk prefix
    pub fn prefix(self) -> &'static str {
        match self {
            KernelLogLevel::Emergency => "<0>",
            KernelLogLevel::Alert => "<1>",
            KernelLogLevel::Critical => "<2>",
            KernelLogLevel::Error => "<3>",
            KernelLogLevel::Warning => "<4>",
            KernelLogLevel::Notice => "<5>",
            KernelLogLevel::Info => "<6>",
            KernelLogLevel::Debug => "<7>",
        }
    }

    /// Human-readable label
    pub fn label(self) -> &'static str {
        match self {
            KernelLogLevel::Emergency => "EMERG",
            KernelLogLevel::Alert => "ALERT",
            KernelLogLevel::Critical => "CRIT",
            KernelLogLevel::Error => "ERR",
            KernelLogLevel::Warning => "WARN",
            KernelLogLevel::Notice => "NOTE",
            KernelLogLevel::Info => "INFO",
            KernelLogLevel::Debug => "DEBUG",
        }
    }

    /// VGA text mode color for this level (attribute byte)
    pub fn vga_color(self) -> u8 {
        match self {
            KernelLogLevel::Emergency => 0x4F, // White on red
            KernelLogLevel::Alert => 0x4E,     // Yellow on red
            KernelLogLevel::Critical => 0x0C,  // Light red on black
            KernelLogLevel::Error => 0x04,     // Red on black
            KernelLogLevel::Warning => 0x0E,   // Yellow on black
            KernelLogLevel::Notice => 0x0B,    // Light cyan on black
            KernelLogLevel::Info => 0x0F,      // White on black
            KernelLogLevel::Debug => 0x07,     // Light grey on black
        }
    }
}

impl fmt::Display for KernelLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

// ──────────────────────────── Kernel Log Entry ───────────────────────────────

/// A single entry in the kernel log ring buffer
#[derive(Debug, Clone)]
pub struct KernelLogEntry {
    /// Monotonic sequence number
    pub sequence: u64,
    /// Timestamp in microseconds since boot
    pub timestamp_us: u64,
    /// Log level
    pub level: KernelLogLevel,
    /// Subsystem/facility name
    pub facility: Option<String>,
    /// The log message
    pub message: String,
}

impl fmt::Display for KernelLogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.timestamp_us / 1_000_000;
        let usecs = self.timestamp_us % 1_000_000;
        if let Some(ref fac) = self.facility {
            write!(
                f,
                "[{:5}.{:06}] {}: {}: {}",
                secs,
                usecs,
                self.level.label(),
                fac,
                self.message
            )
        } else {
            write!(
                f,
                "[{:5}.{:06}] {}: {}",
                secs,
                usecs,
                self.level.label(),
                self.message
            )
        }
    }
}

// ──────────────────────────── Kernel Log Buffer ──────────────────────────────

/// Ring buffer for kernel log messages (like Linux's `__log_buf`)
pub struct KernelLogBuffer {
    entries: Vec<KernelLogEntry>,
    max_entries: usize,
    write_index: usize,
    sequence: u64,
    boot_time_us: u64,
}

impl KernelLogBuffer {
    /// Create a new log buffer with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            max_entries: capacity,
            write_index: 0,
            sequence: 0,
            boot_time_us: 0,
        }
    }

    /// Add a log entry
    pub fn push(&mut self, level: KernelLogLevel, facility: Option<&str>, message: String) {
        self.sequence += 1;
        let entry = KernelLogEntry {
            sequence: self.sequence,
            timestamp_us: self.current_time_us(),
            level,
            facility: facility.map(|s| s.to_string()),
            message,
        };

        if self.entries.len() < self.max_entries {
            self.entries.push(entry);
        } else {
            self.entries[self.write_index] = entry;
        }
        self.write_index = (self.write_index + 1) % self.max_entries;
    }

    /// Get all entries in chronological order
    pub fn entries(&self) -> Vec<&KernelLogEntry> {
        if self.entries.len() < self.max_entries {
            self.entries.iter().collect()
        } else {
            let mut result = Vec::with_capacity(self.max_entries);
            for i in 0..self.max_entries {
                let idx = (self.write_index + i) % self.max_entries;
                result.push(&self.entries[idx]);
            }
            result
        }
    }

    /// Get entries filtered by log level (level and above)
    pub fn entries_at_level(&self, max_level: KernelLogLevel) -> Vec<&KernelLogEntry> {
        self.entries()
            .into_iter()
            .filter(|e| e.level <= max_level)
            .collect()
    }

    /// Get the last N entries
    pub fn last_n(&self, n: usize) -> Vec<&KernelLogEntry> {
        let all = self.entries();
        let start = all.len().saturating_sub(n);
        all[start..].to_vec()
    }

    /// Total number of messages logged
    pub fn total_messages(&self) -> u64 {
        self.sequence
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.entries.clear();
        self.write_index = 0;
    }

    /// Format all entries as a dmesg-style output
    pub fn format_dmesg(&self) -> String {
        let mut output = String::new();
        for entry in self.entries() {
            let _ = writeln!(output, "{}", entry);
        }
        output
    }

    fn current_time_us(&self) -> u64 {
        // In hosted mode, use system time
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_micros() as u64)
            .unwrap_or(0)
            .saturating_sub(self.boot_time_us)
    }
}

impl Default for KernelLogBuffer {
    fn default() -> Self {
        Self::new(4096)
    }
}

// ──────────────────────────── Kernel Writer ──────────────────────────────────

/// A writer that supports formatted output to the kernel log
pub struct KernelWriter {
    buffer: String,
}

impl KernelWriter {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn into_string(self) -> String {
        self.buffer
    }

    pub fn as_str(&self) -> &str {
        &self.buffer
    }
}

impl Default for KernelWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for KernelWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

// ──────────────────────────── kprintf Functions ──────────────────────────────

/// Kernel printf — formatted output to the kernel log
///
/// Supports standard Rust format strings via the `write!` macro.
/// Use `kprintk` for log-level-aware output.
pub fn kprintf(log: &mut KernelLogBuffer, args: fmt::Arguments<'_>) {
    let mut writer = KernelWriter::new();
    let _ = writer.write_fmt(args);
    log.push(KernelLogLevel::Info, None, writer.into_string());
}

/// Kernel printk with log level — like Linux's `printk(KERN_INFO "msg")`
pub fn kprintk(
    log: &mut KernelLogBuffer,
    level: KernelLogLevel,
    facility: Option<&str>,
    args: fmt::Arguments<'_>,
) {
    let mut writer = KernelWriter::new();
    let _ = writer.write_fmt(args);
    log.push(level, facility, writer.into_string());
}

// ──────────────────────────── Hex Dump Utility ───────────────────────────────

/// Format a memory region as a hex dump
///
/// Produces output similar to `xxd` or Linux's `print_hex_dump`:
/// ```text
/// 00000000: 48 65 6C 6C 6F 20 57 6F 72 6C 64 21 00 00 00 00  Hello World!....
/// ```
pub fn hex_dump(data: &[u8], base_address: u64) -> String {
    let mut output = String::new();
    let bytes_per_line = 16;

    for (i, chunk) in data.chunks(bytes_per_line).enumerate() {
        let addr = base_address + (i * bytes_per_line) as u64;

        // Address
        let _ = write!(output, "{:08x}: ", addr);

        // Hex bytes
        for (j, byte) in chunk.iter().enumerate() {
            let _ = write!(output, "{:02X} ", byte);
            if j == 7 {
                output.push(' '); // Extra space between 8-byte groups
            }
        }

        // Padding for incomplete last line
        if chunk.len() < bytes_per_line {
            for j in chunk.len()..bytes_per_line {
                output.push_str("   ");
                if j == 7 {
                    output.push(' ');
                }
            }
        }

        // ASCII representation
        output.push(' ');
        for byte in chunk {
            if byte.is_ascii_graphic() || *byte == b' ' {
                output.push(*byte as char);
            } else {
                output.push('.');
            }
        }

        output.push('\n');
    }

    output
}

/// Format a single integer value in multiple representations
pub fn format_value(value: u64) -> String {
    format!(
        "dec={} hex=0x{:x} oct=0o{:o} bin=0b{:b}",
        value, value, value, value
    )
}

// ──────────────────────────── Macros ─────────────────────────────────────────

/// Format kernel log message (returns String, doesn't require log buffer)
pub fn kformat(args: fmt::Arguments<'_>) -> String {
    let mut writer = KernelWriter::new();
    let _ = writer.write_fmt(args);
    writer.into_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_levels() {
        assert!(KernelLogLevel::Emergency < KernelLogLevel::Debug);
        assert_eq!(KernelLogLevel::Info.label(), "INFO");
        assert_eq!(KernelLogLevel::Error.prefix(), "<3>");
    }

    #[test]
    fn test_kernel_writer() {
        let mut w = KernelWriter::new();
        write!(w, "Hello {} #{}", "World", 42).unwrap();
        assert_eq!(w.as_str(), "Hello World #42");
    }

    #[test]
    fn test_log_buffer() {
        let mut log = KernelLogBuffer::new(8);
        log.push(KernelLogLevel::Info, None, "test message".to_string());
        log.push(
            KernelLogLevel::Error,
            Some("kernel"),
            "error occurred".to_string(),
        );
        assert_eq!(log.total_messages(), 2);
        assert_eq!(log.entries().len(), 2);
    }

    #[test]
    fn test_log_buffer_overflow() {
        let mut log = KernelLogBuffer::new(4);
        for i in 0..8 {
            log.push(
                KernelLogLevel::Info,
                None,
                format!("message {}", i),
            );
        }
        assert_eq!(log.total_messages(), 8);
        assert_eq!(log.entries().len(), 4);
    }

    #[test]
    fn test_log_level_filter() {
        let mut log = KernelLogBuffer::new(16);
        log.push(KernelLogLevel::Debug, None, "debug".to_string());
        log.push(KernelLogLevel::Error, None, "error".to_string());
        log.push(KernelLogLevel::Info, None, "info".to_string());

        let errors = log.entries_at_level(KernelLogLevel::Error);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_hex_dump() {
        let data = b"Hello World!\x00\x01\x02\x03";
        let dump = hex_dump(data, 0x1000);
        assert!(dump.contains("1000:"));
        assert!(dump.contains("48 65 6C 6C"));
        assert!(dump.contains("Hello World!"));
    }

    #[test]
    fn test_hex_dump_multiline() {
        let data = vec![0x41u8; 32]; // 32 bytes of 'A'
        let dump = hex_dump(&data, 0);
        let lines: Vec<&str> = dump.trim().lines().collect();
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_format_value() {
        let s = format_value(255);
        assert!(s.contains("dec=255"));
        assert!(s.contains("hex=0xff"));
        assert!(s.contains("oct=0o377"));
        assert!(s.contains("bin=0b11111111"));
    }

    #[test]
    fn test_dmesg_format() {
        let mut log = KernelLogBuffer::new(16);
        log.push(KernelLogLevel::Info, Some("kernel"), "Boot complete".to_string());
        let dmesg = log.format_dmesg();
        assert!(dmesg.contains("INFO"));
        assert!(dmesg.contains("kernel"));
        assert!(dmesg.contains("Boot complete"));
    }
}
