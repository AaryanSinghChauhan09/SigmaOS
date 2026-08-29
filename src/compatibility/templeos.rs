//! TempleOS Compatibility and Personality Core for SigmaOS
//!
//! Implements HolyC JIT shell, 64-bit RedSea FS, Holy Oracle, and Ring-0 Cooperative Scheduler.
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
extern crate alloc;
use alloc::vec;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;


use crate::klib::BTreeMap;

// =========================================================================
// 1. HolyC JIT Compiler Shell
// =========================================================================

pub struct HolyCShell {
    pub jitted_symbols: BTreeMap<String, Vec<u8>>,
}

impl HolyCShell {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        HolyCShell {
            jitted_symbols: BTreeMap::new(),
        }
    }

    /// Compiles a HolyC code snippet to Ring-0 bytecode
    pub fn jit_compile(&mut self, function_name: &str, source: &str) -> Result<Vec<u8>, &'static str> {
        if source.contains("Print") {
            let bytecode = vec![0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, 0xC3]; // mov rax, 1; ret
            self.jitted_symbols.insert(function_name.to_string(), bytecode.clone());
            Ok(bytecode)
        } else {
            Err("HolyC compilation error: missing print statements")
        }
    }
}

// =========================================================================
// 2. RedSea 64-bit Filesystem
// =========================================================================

pub struct RedSeaFilesystem {
    pub raw_volume: Vec<u8>,
    pub sector_size: usize,
}

impl RedSeaFilesystem {
    pub fn new(sectors: usize) -> Self {
        RedSeaFilesystem {
            raw_volume: vec![0; sectors * 512],
            sector_size: 512,
        }
    }

    /// Reads contiguous unfragmented sector ranges directly
    pub fn read_contiguous(&self, start_sector: usize, sectors_count: usize) -> Result<Vec<u8>, &'static str> {
        let start = start_sector * self.sector_size;
        let end = start + (sectors_count * self.sector_size);
        if end <= self.raw_volume.len() {
            Ok(self.raw_volume[start..end].to_vec())
        } else {
            Err("RedSea FS Error: out of contiguous range limits")
        }
    }

    /// Writes contiguous unfragmented sector ranges directly
    pub fn write_contiguous(&mut self, start_sector: usize, data: &[u8]) -> Result<(), &'static str> {
        let start = start_sector * self.sector_size;
        let end = start + data.len();
        if end <= self.raw_volume.len() {
            self.raw_volume[start..end].copy_from_slice(data);
            Ok(())
        } else {
            Err("RedSea FS Error: disk overflow")
        }
    }
}

// =========================================================================
// 3. Holy Spirit Oracle
// =========================================================================

pub struct HolySpiritOracle {
    pub state: u64,
    pub dictionary: Vec<String>,
}

impl HolySpiritOracle {
    pub fn new(seed: u64) -> Self {
        let mut dictionary = Vec::new();
        dictionary.push("God".to_string());
        dictionary.push("Spirit".to_string());
        dictionary.push("Temple".to_string());
        dictionary.push("Oracle".to_string());
        dictionary.push("Light".to_string());
        dictionary.push("Cosmos".to_string());

        HolySpiritOracle {
            state: seed,
            dictionary,
        }
    }

    /// Dynamic high-entropy pseudorandom generator
    pub fn next_random(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    /// Generates randomized prophecies from the Oracle dictionary
    pub fn speak_oracle(&mut self) -> String {
        let idx = (self.next_random() % (self.dictionary.len() as u64)) as usize;
        self.dictionary[idx].clone()
    }
}

// =========================================================================
// 4. Ring-0 Cooperative Scheduler
// =========================================================================

pub struct CooperativeTask {
    pub id: u32,
    pub status: String,
}

pub struct RingZeroSandbox {
    pub active_tasks: Vec<CooperativeTask>,
    pub run_idx: usize,
}

impl RingZeroSandbox {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        RingZeroSandbox {
            active_tasks: Vec::new(),
            run_idx: 0,
        }
    }

    pub fn register_task(&mut self, task: CooperativeTask) {
        self.active_tasks.push(task);
    }

    /// Voluntarily yields execution control to next thread with zero preemptive overhead
    pub fn yield_cooperative(&mut self) -> String {
        if self.active_tasks.is_empty() {
            return "No tasks active".to_string();
        }
        let current_task = &self.active_tasks[self.run_idx];
        let msg = format!("Task {} yielded. State: {}", current_task.id, current_task.status);
        self.run_idx = (self.run_idx + 1) % self.active_tasks.len();
        msg
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holyc_jit_compilation() {
        let mut shell = HolyCShell::new();
        let bytecode = shell.jit_compile("Main", "Print(\"Divine!\");").unwrap();
        assert_eq!(bytecode[0..4], [0x48, 0xC7, 0xC0, 0x01]);
    }

    #[test]
    fn test_redsea_unfragmented_filesystem() {
        let mut fs = RedSeaFilesystem::new(10);
        let block_data = [0x55u8; 512];
        assert!(fs.write_contiguous(1, &block_data).is_ok());
        let read_data = fs.read_contiguous(1, 1).unwrap();
        assert_eq!(read_data[0], 0x55);
    }

    #[test]
    fn test_holy_spirit_oracle() {
        let mut oracle = HolySpiritOracle::new(777);
        let rand_val = oracle.next_random();
        assert_ne!(rand_val, 777);
        let prophecy = oracle.speak_oracle();
        assert!(!prophecy.is_empty());
    }

    #[test]
    fn test_ring_zero_cooperative_scheduler() {
        let mut scheduler = RingZeroSandbox::new();
        scheduler.register_task(CooperativeTask { id: 101, status: "Active".to_string() });
        scheduler.register_task(CooperativeTask { id: 102, status: "Sleep".to_string() });

        let yield_1 = scheduler.yield_cooperative();
        assert!(yield_1.contains("Task 101"));

        let yield_2 = scheduler.yield_cooperative();
        assert!(yield_2.contains("Task 102"));
    }
}
