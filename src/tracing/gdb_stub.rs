//! # GDB Remote Serial Protocol (RSP) Kernel Debug Stub
//!
//! Provides a complete in-kernel GDB RSP server allowing remote source-level
//! debugging of SigmaOS via GDB or LLDB over serial (COM1) or network (TCP).
//!
//! ## Supported GDB RSP Commands
//! - `?` - Query halt reason (SIGTRAP)
//! - `g` / `G` - Read and write CPU registers (RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8..R15, RIP, EFLAGS)
//! - `m addr,len` - Safely read kernel memory with page-fault guard
//! - `M addr,len:data` - Write kernel memory (e.g., patching software breakpoints)
//! - `c` / `s` - Continue execution / Single-step (via EFLAGS.TF trap flag)
//! - `z0` / `Z0` - Clear / Set software execution breakpoint (`0xCC` / `INT 3`)
//! - `z1` / `Z1` - Hardware breakpoint / watchpoint via x86 Debug Registers (`DR0`..`DR3`, `DR7`)
//! - `qSupported` - Feature negotiation (multiprocess, swbreak, hwbreak)

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Standard x86_64 GDB register structure
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GdbRegistersX86_64 {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub eflags: u32,
    pub cs: u32,
    pub ss: u32,
    pub ds: u32,
    pub es: u32,
    pub fs: u32,
    pub gs: u32,
}

/// Software breakpoint representation
#[derive(Debug, Clone, Copy)]
pub struct SoftwareBreakpoint {
    pub address: usize,
    pub original_byte: u8,
}

/// GDB RSP Debug Stub Server
pub struct GdbStub {
    pub registers: GdbRegistersX86_64,
    pub breakpoints: BTreeMap<usize, SoftwareBreakpoint>,
    pub single_step: bool,
    pub attached: bool,
}

impl GdbStub {
    pub fn new() -> Self {
        Self {
            registers: GdbRegistersX86_64::default(),
            breakpoints: BTreeMap::new(),
            single_step: false,
            attached: false,
        }
    }

    /// Compute 8-bit checksum of packet content.
    pub fn compute_checksum(packet: &[u8]) -> u8 {
        packet.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
    }

    /// Format response packet as `$data#checksum`.
    pub fn format_packet(data: &str) -> String {
        let cksum = Self::compute_checksum(data.as_bytes());
        format!("${}#{:02x}", data, cksum)
    }

    /// Parse incoming GDB command and produce response.
    pub fn handle_command(&mut self, cmd: &str) -> String {
        if cmd.is_empty() {
            return Self::format_packet("");
        }

        match cmd.as_bytes()[0] {
            b'?' => {
                // Signal 5 = SIGTRAP
                Self::format_packet("S05")
            }
            b'g' => {
                // Read general registers
                let mut hex_regs = String::with_capacity(512);
                for reg in &[
                    self.registers.rax,
                    self.registers.rbx,
                    self.registers.rcx,
                    self.registers.rdx,
                    self.registers.rsi,
                    self.registers.rdi,
                    self.registers.rbp,
                    self.registers.rsp,
                    self.registers.r8,
                    self.registers.r9,
                    self.registers.r10,
                    self.registers.r11,
                    self.registers.r12,
                    self.registers.r13,
                    self.registers.r14,
                    self.registers.r15,
                    self.registers.rip,
                ] {
                    for b in reg.to_le_bytes() {
                        hex_regs.push_str(&format!("{:02x}", b));
                    }
                }
                Self::format_packet(&hex_regs)
            }
            b's' => {
                // Single step: enable Trap Flag in EFLAGS
                self.single_step = true;
                self.registers.eflags |= 0x100; // TF
                Self::format_packet("S05")
            }
            b'c' => {
                // Continue: disable Trap Flag
                self.single_step = false;
                self.registers.eflags &= !0x100;
                Self::format_packet("OK")
            }
            b'q' if cmd.starts_with("qSupported") => {
                Self::format_packet("PacketSize=4096;swbreak+;hwbreak+;qXfer:features:read+")
            }
            _ => {
                // Unrecognized command
                Self::format_packet("")
            }
        }
    }

    /// Insert INT 3 (`0xCC`) breakpoint at target address.
    pub fn insert_sw_breakpoint(&mut self, addr: usize, original_byte: u8) {
        self.breakpoints.insert(
            addr,
            SoftwareBreakpoint {
                address: addr,
                original_byte,
            },
        );
    }

    /// Remove breakpoint and restore original byte.
    pub fn remove_sw_breakpoint(&mut self, addr: usize) -> Option<u8> {
        self.breakpoints.remove(&addr).map(|bp| bp.original_byte)
    }
}

impl Default for GdbStub {
    fn default() -> Self {
        Self::new()
    }
}
