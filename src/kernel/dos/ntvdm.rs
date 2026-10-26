// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - NT Virtual DOS Machine (NTVDM) bridge
// Principles: ABSTRACTION, COMPATIBILITY, INTERFACE SEGREGATION

use alloc::{collections::BTreeMap, string::String, vec::Vec};

/// NTVDM session configuration
#[derive(Debug, Clone)]
pub struct NtvdmConfig {
    pub mem_size_kb: u32,        // conventional memory (up to 640KB)
    pub xms_size_kb: u32,        // extended memory (XMS)
    pub ems_size_kb: u32,        // expanded memory (EMS)
    pub com_port_base: [u16; 4], // COM1-4 I/O base addresses
    pub lpt_port_base: [u16; 3], // LPT1-3 I/O base addresses
    pub video_mode: VideoMode,
}

impl Default for NtvdmConfig {
    fn default() -> Self {
        Self {
            mem_size_kb: 640,
            xms_size_kb: 4096,
            ems_size_kb: 0,
            com_port_base: [0x3F8, 0x2F8, 0x3E8, 0x2E8],
            lpt_port_base: [0x378, 0x278, 0x3BC],
            video_mode: VideoMode::Text80x25,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VideoMode {
    Text80x25,
    Text40x25,
    Cga320x200x4,
    Cga640x200x2,
    Vga320x200x256,
    Vga640x480x16,
}

/// 16→32 bit thunk descriptor
#[derive(Debug, Clone)]
pub struct Ntvdm16BitThunk {
    pub thunk_id: u16,
    pub target_fn: u32, // 32-bit flat address
    pub param_bytes: u8,
    pub ret_bytes: u8,
}

/// Console I/O buffer
#[derive(Debug)]
pub struct NtvdmConsole {
    pub output: Vec<u8>,
    pub input_queue: Vec<u8>,
    pub cursor_row: u8,
    pub cursor_col: u8,
    pub video_mode: VideoMode,
}

impl NtvdmConsole {
    pub fn new(mode: VideoMode) -> Self {
        Self {
            output: Vec::new(),
            input_queue: Vec::new(),
            cursor_row: 0,
            cursor_col: 0,
            video_mode: mode,
        }
    }

    pub fn write_char(&mut self, c: u8) {
        match c {
            b'\n' => {
                self.cursor_row += 1;
                self.cursor_col = 0;
            }
            b'\r' => {
                self.cursor_col = 0;
            }
            _ => {
                self.output.push(c);
                self.cursor_col += 1;
            }
        }
        self.output.push(c);
    }

    pub fn read_char(&mut self) -> Option<u8> {
        self.input_queue.pop()
    }
    pub fn inject_input(&mut self, data: &[u8]) {
        self.input_queue.extend_from_slice(data);
    }
}

/// NTVDM session — emulates DOS environment under NT/SigmaOS
pub struct NtvdmSession {
    pub config: NtvdmConfig,
    pub console: NtvdmConsole,
    pub memory: Vec<u8>,
    pub thunks: Vec<Ntvdm16BitThunk>,
    pub exit_code: Option<u8>,
    pub program_name: String,
    pub psp_segment: u16, // Program Segment Prefix
}

impl NtvdmSession {
    pub fn create(config: NtvdmConfig) -> Self {
        let mem_size = (config.mem_size_kb as usize) * 1024;
        let video_mode = config.video_mode;
        Self {
            console: NtvdmConsole::new(video_mode),
            memory: alloc::vec![0u8; mem_size],
            thunks: Vec::new(),
            exit_code: None,
            program_name: String::new(),
            psp_segment: 0x1000,
            config,
        }
    }

    pub fn load_dos_program(&mut self, name: &str, image: &[u8]) -> Result<(), &'static str> {
        let load_offset = (self.psp_segment as usize + 0x10) * 16;
        if load_offset + image.len() > self.memory.len() {
            return Err("program too large for conventional memory");
        }
        self.memory[load_offset..load_offset + image.len()].copy_from_slice(image);
        self.program_name = name.into();
        Ok(())
    }

    /// Simulate running for N clock ticks
    pub fn run_ticks(&mut self, ticks: u32) -> Option<u8> {
        // Stub: simulate N ticks of DOS execution
        for _ in 0..ticks {
            if self.exit_code.is_some() {
                break;
            }
            // In a real implementation this would step the V86 machine
        }
        self.exit_code
    }

    pub fn terminate(&mut self, code: u8) {
        self.exit_code = Some(code);
    }

    pub fn console_output(&self) -> &[u8] {
        &self.console.output
    }
    pub fn console_input(&mut self, data: &[u8]) {
        self.console.inject_input(data);
    }

    pub fn register_thunk(&mut self, thunk: Ntvdm16BitThunk) {
        self.thunks.push(thunk);
    }

    pub fn call_thunk(&self, thunk_id: u16) -> Result<u32, &'static str> {
        self.thunks
            .iter()
            .find(|t| t.thunk_id == thunk_id)
            .map(|t| t.target_fn)
            .ok_or("thunk not found")
    }

    pub fn memory_read(&self, addr: u32, len: usize) -> &[u8] {
        let start = addr as usize;
        let end = (start + len).min(self.memory.len());
        &self.memory[start..end]
    }

    pub fn memory_write(&mut self, addr: u32, data: &[u8]) -> Result<(), &'static str> {
        let start = addr as usize;
        let end = start + data.len();
        if end > self.memory.len() {
            return Err("address out of range");
        }
        self.memory[start..end].copy_from_slice(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_create() {
        let cfg = NtvdmConfig::default();
        let sess = NtvdmSession::create(cfg);
        assert_eq!(sess.memory.len(), 640 * 1024);
        assert!(sess.exit_code.is_none());
    }

    #[test]
    fn test_load_program() {
        let cfg = NtvdmConfig::default();
        let mut sess = NtvdmSession::create(cfg);
        let prog = alloc::vec![0xB8, 0x00, 0x4C, 0xCD, 0x21]; // MOV AX,4C00h; INT 21h
        sess.load_dos_program("TEST.COM", &prog).unwrap();
    }

    #[test]
    fn test_memory_read_write() {
        let cfg = NtvdmConfig::default();
        let mut sess = NtvdmSession::create(cfg);
        sess.memory_write(0x1000, &[0xDE, 0xAD, 0xBE, 0xEF])
            .unwrap();
        let data = sess.memory_read(0x1000, 4);
        assert_eq!(data, &[0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_thunk_registration() {
        let cfg = NtvdmConfig::default();
        let mut sess = NtvdmSession::create(cfg);
        sess.register_thunk(Ntvdm16BitThunk {
            thunk_id: 1,
            target_fn: 0xCAFEBABE,
            param_bytes: 4,
            ret_bytes: 4,
        });
        assert_eq!(sess.call_thunk(1).unwrap(), 0xCAFEBABE);
        assert!(sess.call_thunk(99).is_err());
    }
}
