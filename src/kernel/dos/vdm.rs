// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Virtual 8086 Mode Machine (V86)
// Principles: LOW LEVEL CONTROL, DIRECT DEVICE CONTROL, MEMORY DISCIPLINE, DEFENSIVE PROGRAMMING

use alloc::{vec::Vec, collections::BTreeMap};

/// x86 register file for V86 mode
#[derive(Debug, Clone, Default)]
pub struct V86Registers {
    pub ax: u16, pub bx: u16, pub cx: u16, pub dx: u16,
    pub si: u16, pub di: u16, pub bp: u16, pub sp: u16,
    pub cs: u16, pub ds: u16, pub es: u16, pub ss: u16,
    pub fs: u16, pub gs: u16,
    pub ip: u16,
    pub flags: u16,
}

impl V86Registers {
    // High/low byte accessors
    pub fn al(&self) -> u8 { (self.ax & 0xFF) as u8 }
    pub fn ah(&self) -> u8 { (self.ax >> 8) as u8 }
    pub fn bl(&self) -> u8 { (self.bx & 0xFF) as u8 }
    pub fn bh(&self) -> u8 { (self.bx >> 8) as u8 }
    pub fn cl(&self) -> u8 { (self.cx & 0xFF) as u8 }
    pub fn ch(&self) -> u8 { (self.cx >> 8) as u8 }
    pub fn dl(&self) -> u8 { (self.dx & 0xFF) as u8 }
    pub fn dh(&self) -> u8 { (self.dx >> 8) as u8 }

    pub fn set_al(&mut self, v: u8) { self.ax = (self.ax & 0xFF00) | v as u16; }
    pub fn set_ah(&mut self, v: u8) { self.ax = (self.ax & 0x00FF) | ((v as u16) << 8); }

    // Flag bits
    pub fn carry(&self) -> bool { (self.flags & 0x0001) != 0 }
    pub fn zero(&self) -> bool { (self.flags & 0x0040) != 0 }
    pub fn sign(&self) -> bool { (self.flags & 0x0080) != 0 }
    pub fn interrupt_enable(&self) -> bool { (self.flags & 0x0200) != 0 }
    pub fn vm_flag(&self) -> bool { (self.flags & 0x0800) != 0 }  // V86 mode flag

    pub fn set_carry(&mut self, v: bool) { if v { self.flags |= 0x0001; } else { self.flags &= !0x0001; } }
    pub fn set_zero(&mut self, v: bool) { if v { self.flags |= 0x0040; } else { self.flags &= !0x0040; } }

    pub fn linear_ip(&self) -> u32 { (self.cs as u32) * 16 + self.ip as u32 }
    pub fn linear_sp(&self) -> u32 { (self.ss as u32) * 16 + self.sp as u32 }
}

/// Port I/O trap handler
#[derive(Debug, Clone)]
pub struct PortTrap {
    pub port: u16,
    pub description: &'static str,
}

/// Virtual 8086 Mode machine state
pub struct V86Machine {
    pub regs: V86Registers,
    pub memory: Vec<u8>,       // 1MB real-mode address space
    pub port_traps: BTreeMap<u16, PortTrap>,
    pub int_vectors: [u32; 256], // IVT at 0000:0000
    pub halted: bool,
    pub step_count: u64,
}

impl V86Machine {
    pub fn new(mem_kb: u32) -> Self {
        let size = (mem_kb as usize).min(1024) * 1024;
        let mut m = Self {
            regs: V86Registers::default(),
            memory: alloc::vec![0u8; size],
            port_traps: BTreeMap::new(),
            int_vectors: [0u32; 256],
            halted: false,
            step_count: 0,
        };
        // Set initial segment registers
        m.regs.cs = 0xF000;
        m.regs.ip = 0xFFF0;
        m.regs.ds = 0x0000;
        m.regs.ss = 0x0000;
        m.regs.sp = 0xFFFE;
        m.regs.flags = 0x0002; // always-1 bit
        m
    }

    /// Load a binary image at a real-mode address
    pub fn load_image(&mut self, seg: u16, off: u16, data: &[u8]) -> Result<(), &'static str> {
        let addr = (seg as usize) * 16 + off as usize;
        let end = addr + data.len();
        if end > self.memory.len() { return Err("image exceeds memory"); }
        self.memory[addr..end].copy_from_slice(data);
        Ok(())
    }

    pub fn mem_read8(&self, seg: u16, off: u16) -> u8 {
        let addr = (seg as usize) * 16 + off as usize;
        if addr < self.memory.len() { self.memory[addr] } else { 0xFF }
    }

    pub fn mem_write8(&mut self, seg: u16, off: u16, val: u8) {
        let addr = (seg as usize) * 16 + off as usize;
        if addr < self.memory.len() { self.memory[addr] = val; }
    }

    pub fn mem_read16(&self, seg: u16, off: u16) -> u16 {
        let lo = self.mem_read8(seg, off) as u16;
        let hi = self.mem_read8(seg, off.wrapping_add(1)) as u16;
        lo | (hi << 8)
    }

    pub fn mem_write16(&mut self, seg: u16, off: u16, val: u16) {
        self.mem_write8(seg, off, (val & 0xFF) as u8);
        self.mem_write8(seg, off.wrapping_add(1), (val >> 8) as u8);
    }

    /// Register a port I/O trap
    pub fn trap_port_io(&mut self, port: u16, description: &'static str) {
        self.port_traps.insert(port, PortTrap { port, description });
    }

    /// Fetch the next opcode byte and advance IP
    fn fetch8(&mut self) -> u8 {
        let byte = self.mem_read8(self.regs.cs, self.regs.ip);
        self.regs.ip = self.regs.ip.wrapping_add(1);
        byte
    }

    fn fetch16(&mut self) -> u16 {
        let lo = self.fetch8() as u16;
        let hi = self.fetch8() as u16;
        lo | (hi << 8)
    }

    /// Handle a software interrupt
    pub fn handle_int(&mut self, n: u8) {
        // Read IVT entry (4 bytes: offset, segment)
        let ivt_addr = (n as u16) * 4;
        let off = self.mem_read16(0, ivt_addr);
        let seg = self.mem_read16(0, ivt_addr + 2);
        // Push flags, CS, IP for IRET
        let flags = self.regs.flags;
        let cs = self.regs.cs;
        let ip = self.regs.ip;
        self.stack_push16(flags);
        self.stack_push16(cs);
        self.stack_push16(ip);
        // Jump to handler
        self.regs.cs = seg;
        self.regs.ip = off;
    }

    fn stack_push16(&mut self, val: u16) {
        self.regs.sp = self.regs.sp.wrapping_sub(2);
        let sp = self.regs.sp;
        let ss = self.regs.ss;
        self.mem_write16(ss, sp, val);
    }

    fn stack_pop16(&mut self) -> u16 {
        let sp = self.regs.sp;
        let ss = self.regs.ss;
        let val = self.mem_read16(ss, sp);
        self.regs.sp = self.regs.sp.wrapping_add(2);
        val
    }

    /// Execute one instruction (simplified decoder)
    pub fn step(&mut self) -> Result<(), &'static str> {
        if self.halted { return Err("machine halted"); }
        let opcode = self.fetch8();
        self.step_count += 1;

        match opcode {
            0x90 => {}  // NOP
            0xF4 => { self.halted = true; }  // HLT
            0xEB => {   // JMP short
                let rel = self.fetch8() as i8;
                self.regs.ip = self.regs.ip.wrapping_add(rel as u16);
            }
            0xCD => {   // INT n
                let n = self.fetch8();
                self.handle_int(n);
            }
            0xCF => {   // IRET
                let ip = self.stack_pop16();
                let cs = self.stack_pop16();
                let flags = self.stack_pop16();
                self.regs.ip = ip;
                self.regs.cs = cs;
                self.regs.flags = flags;
            }
            0xB8..=0xBF => {  // MOV r16, imm16
                let reg = opcode - 0xB8;
                let imm = self.fetch16();
                let regs = &mut self.regs;
                match reg {
                    0 => regs.ax = imm,
                    1 => regs.cx = imm,
                    2 => regs.dx = imm,
                    3 => regs.bx = imm,
                    4 => regs.sp = imm,
                    5 => regs.bp = imm,
                    6 => regs.si = imm,
                    7 => regs.di = imm,
                    _ => {}
                }
            }
            _ => {}  // unimplemented opcode — real impl would raise #UD
        }
        Ok(())
    }

    /// Run up to `max_steps` instructions
    pub fn run(&mut self, max_steps: u64) {
        for _ in 0..max_steps {
            if self.halted { break; }
            if self.step().is_err() { break; }
        }
    }

    /// Set IVT entry
    pub fn set_interrupt_vector(&mut self, n: u8, seg: u16, off: u16) {
        let addr = (n as u16) * 4;
        self.mem_write16(0, addr, off);
        self.mem_write16(0, addr + 2, seg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_creation() {
        let m = V86Machine::new(640);
        assert_eq!(m.memory.len(), 640 * 1024);
        assert!(!m.halted);
        assert_eq!(m.regs.cs, 0xF000);
    }

    #[test]
    fn test_load_and_run_nop_hlt() {
        let mut m = V86Machine::new(64);
        m.regs.cs = 0x0000;
        m.regs.ip = 0x0100;
        m.load_image(0x0000, 0x0100, &[0x90, 0xF4]).unwrap(); // NOP; HLT
        m.run(10);
        assert!(m.halted);
        assert_eq!(m.step_count, 2);
    }

    #[test]
    fn test_mov_ax_imm() {
        let mut m = V86Machine::new(64);
        m.regs.cs = 0x0000;
        m.regs.ip = 0x0200;
        // MOV AX, 0x1234; HLT
        m.load_image(0x0000, 0x0200, &[0xB8, 0x34, 0x12, 0xF4]).unwrap();
        m.run(10);
        assert_eq!(m.regs.ax, 0x1234);
        assert!(m.halted);
    }

    #[test]
    fn test_port_trap_registration() {
        let mut m = V86Machine::new(64);
        m.trap_port_io(0x60, "PS/2 keyboard");
        m.trap_port_io(0x64, "PS/2 controller");
        assert_eq!(m.port_traps.len(), 2);
        assert!(m.port_traps.contains_key(&0x60));
    }

    #[test]
    fn test_mem_read_write() {
        let mut m = V86Machine::new(64);
        m.mem_write8(0x1000, 0x0000, 0xAB);
        m.mem_write16(0x1000, 0x0001, 0xBEEF);
        assert_eq!(m.mem_read8(0x1000, 0x0000), 0xAB);
        assert_eq!(m.mem_read16(0x1000, 0x0001), 0xBEEF);
    }
}
