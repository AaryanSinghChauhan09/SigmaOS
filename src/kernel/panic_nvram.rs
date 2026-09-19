// SPDX-License-Identifier: MIT
// SigmaOS Persistent EFI NVRAM Panic Dump Engine
// Persistent crash analytics and panic log dumper for bare-metal diagnostics

#![allow(dead_code)]

use std::vec::Vec;
use std::string::String;

/// Persistent Panic Record
#[derive(Debug, Clone)]
pub struct PanicRecord {
    pub timestamp_ms: u64,
    pub cpu_id: u32,
    pub registers_rip: u64,
    pub panic_message: String,
    pub stack_backtrace: Vec<u64>,
}

/// Sovereign EFI NVRAM Panic Logger
#[derive(Debug)]
pub struct SovereignEfiPanicLogger {
    pub nvram_storage: Vec<u8>,
    pub max_capacity_bytes: usize,
    pub record_count: usize,
}

impl SovereignEfiPanicLogger {
    pub fn new(max_bytes: usize) -> Self {
        Self {
            nvram_storage: Vec::new(),
            max_capacity_bytes: max_bytes,
            record_count: 0,
        }
    }

    pub fn record_panic(&mut self, record: PanicRecord) -> Result<(), &'static str> {
        let serialized = format!(
            "CRASH|TS={}|CPU={}|RIP=0x{:X}|MSG={}\n",
            record.timestamp_ms, record.cpu_id, record.registers_rip, record.panic_message
        );

        let bytes = serialized.as_bytes();
        if self.nvram_storage.len() + bytes.len() > self.max_capacity_bytes {
            return Err("EFI NVRAM panic storage full");
        }

        self.nvram_storage.extend_from_slice(bytes);
        self.record_count += 1;
        Ok(())
    }

    pub fn retrieve_logs(&self) -> String {
        String::from_utf8_lossy(&self.nvram_storage).to_string()
    }

    pub fn clear_nvram(&mut self) {
        self.nvram_storage.clear();
        self.record_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_efi_panic_logger() {
        let mut logger = SovereignEfiPanicLogger::new(4096);
        let record = PanicRecord {
            timestamp_ms: 1000200,
            cpu_id: 2,
            registers_rip: 0xFFFFFFFF81000500,
            panic_message: String::from("Kernel panic - NULL pointer dereference"),
            stack_backtrace: vec![0xFFFFFFFF81000100, 0xFFFFFFFF81000200],
        };

        assert!(logger.record_panic(record).is_ok());
        let logs = logger.retrieve_logs();
        assert!(logs.contains("NULL pointer dereference"));
        assert_eq!(logger.record_count, 1);
    }
}
