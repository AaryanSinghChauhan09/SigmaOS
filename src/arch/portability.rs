#![no_std]
#![no_main]

/// OOP-based ARM64 + RISC-V Portability for SigmaOS
/// Based on Roadmap Item: ARM64 + RISC-V Portability

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Architecture { X86_64 = 0, ARM64 = 1, RISCV64 = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Endianness { Little = 0, Big = 1 }

pub trait ArchitecturePort {
    fn arch(&self) -> Architecture;
    fn endianness(&self) -> Endianness;
    fn word_size(&self) -> usize;
}

#[repr(C)]
pub struct SimpleArchitecturePort {
    pub arch: Architecture,
    pub endianness: Endianness,
    pub word_size: usize,
}

impl SimpleArchitecturePort {
    pub fn new(arch: Architecture, endianness: Endianness, word_size: usize) -> Self {
        SimpleArchitecturePort { arch, endianness, word_size }
    }
}

impl ArchitecturePort for SimpleArchitecturePort {
    fn arch(&self) -> Architecture { self.arch }
    fn endianness(&self) -> Endianness { self.endianness }
    fn word_size(&self) -> usize { self.word_size }
}

pub trait ARM64Support {
    fn init_arm64(&mut self) -> Result<(), PortError>;
    fn handle_exception(&mut self, exception: usize) -> Result<(), PortError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PortError { Success = 0, UnsupportedArch = 1, InitFailed = 2 }

pub struct SimpleARM64Support {
    pub port: SimpleArchitecturePort,
}

impl SimpleARM64Support {
    pub fn new() -> Self {
        SimpleARM64Support {
            port: SimpleArchitecturePort::new(Architecture::ARM64, Endianness::Little, 64),
        }
    }
}

impl ARM64Support for SimpleARM64Support {
    fn init_arm64(&mut self) -> Result<(), PortError> {
        Ok(())
    }
    fn handle_exception(&mut self, _exception: usize) -> Result<(), PortError> {
        Ok(())
    }
}

pub trait RISCVSupport {
    fn init_riscv(&mut self) -> Result<(), PortError>;
    fn handle_trap(&mut self, trap: usize) -> Result<(), PortError>;
}

pub struct SimpleRISCVSupport {
    pub port: SimpleArchitecturePort,
}

impl SimpleRISCVSupport {
    pub fn new() -> Self {
        SimpleRISCVSupport {
            port: SimpleArchitecturePort::new(Architecture::RISCV64, Endianness::Little, 64),
        }
    }
}

impl RISCVSupport for SimpleRISCVSupport {
    fn init_riscv(&mut self) -> Result<(), PortError> {
        Ok(())
    }
    fn handle_trap(&mut self, _trap: usize) -> Result<(), PortError> {
        Ok(())
    }
}

pub trait MultiArchSupport {
    fn detect_architecture(&self) -> Architecture;
    fn switch_architecture(&mut self, arch: Architecture) -> Result<(), PortError>;
}

pub struct SimpleMultiArchSupport {
    pub current_arch: AtomicUsize,
    pub arm64: SimpleARM64Support,
    pub riscv: SimpleRISCVSupport,
}

impl SimpleMultiArchSupport {
    pub fn new() -> Self {
        SimpleMultiArchSupport {
            current_arch: AtomicUsize::new(Architecture::X86_64 as usize),
            arm64: SimpleARM64Support::new(),
            riscv: SimpleRISCVSupport::new(),
        }
    }
}

impl MultiArchSupport for SimpleMultiArchSupport {
    fn detect_architecture(&self) -> Architecture {
        unsafe { core::mem::transmute(self.current_arch.load(Ordering::SeqCst)) }
    }
    fn switch_architecture(&mut self, arch: Architecture) -> Result<(), PortError> {
        self.current_arch.store(arch as usize, Ordering::SeqCst);
        match arch {
            Architecture::ARM64 => self.arm64.init_arm64(),
            Architecture::RISCV64 => self.riscv.init_riscv(),
            _ => Ok(()),
        }
    }
}
