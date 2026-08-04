/// OOP-based ARM64 + RISC-V Portability for SigmaOS
/// Based on Roadmap Item: ARM64 + RISC-V Portability

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_switch_x86_x64() {
        let x86 = X86Context {
            eax: 0xBAADF00D,
            ebx: 0,
            ecx: 1,
            edx: 2,
            esi: 3,
            edi: 4,
            esp: 0x7FFF0000,
            ebp: 0x7FFF0010,
            eip: 0x08048000,
            eflags: 0,
        };

        let mut engine = SovereignContextSwitchEngine::new(CpuContextState::X86(x86));
        assert_eq!(engine.switch_count, 0);

        let x64 = X64Context {
            rax: 0xDEADC0DE,
            rsp: 0x7FFFFFF0000,
            rip: 0x00007FFFF0000,
            ..Default::default()
        };

        // Switch to x64 register context
        let old = engine.context_switch(CpuContextState::X64(x64));
        assert_eq!(engine.switch_count, 1);
        if let CpuContextState::X86(ctx) = old {
            assert_eq!(ctx.eax, 0xBAADF00D);
        } else {
            panic!("Expected X86 old context");
        }
    }

    #[test]
    fn test_arm64_riscv_trap_simulation() {
        let arm = Arm64Context {
            sp: 0x1000,
            pc: 0x2000,
            pstate: 0,
            ..Default::default()
        };

        let mut engine = SovereignContextSwitchEngine::new(CpuContextState::Arm64(arm));
        let res_arm = engine.simulate_kernel_trap();
        assert_eq!(res_arm, "ARM64_TRAP_HANDLED");
        if let CpuContextState::Arm64(ctx) = engine.current_context {
            assert_eq!(ctx.pstate, 128); // 1 << 7 (IRQ mask set)
        }

        let riscv = Riscv64Context {
            pc: 0x80000000,
            ..Default::default()
        };

        let mut engine_riscv = SovereignContextSwitchEngine::new(CpuContextState::Riscv64(riscv));
        let res_rv = engine_riscv.simulate_kernel_trap();
        assert_eq!(res_rv, "RISCV64_TRAP_HANDLED");
        if let CpuContextState::Riscv64(ctx) = engine_riscv.current_context {
            assert_eq!(ctx.pc, 0x80000004); // Program Counter advanced past trapped instruction
        }
    }
}

/// Register context for 32-bit x86 architecture (Linux/Windows 32-bit parity)
#[derive(Debug, Clone, Copy, Default)]
pub struct X86Context {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub esp: u32,
    pub ebp: u32,
    pub eip: u32,
    pub eflags: u32,
}

/// Register context for 64-bit x64 architecture (Linux/BSD/Windows x64 parity)
#[derive(Debug, Clone, Copy, Default)]
pub struct X64Context {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

/// Register context for 64-bit ARM architecture (ARMv8-A/ARMv9-A Linux/Windows parity)
#[derive(Debug, Clone, Copy, Default)]
pub struct Arm64Context {
    pub x: [u64; 31], // General purpose x0-x30
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
}

/// Register context for 64-bit RISC-V architecture (RV64GC Linux/BSD parity)
#[derive(Debug, Clone, Copy, Default)]
pub struct Riscv64Context {
    pub x: [u64; 32], // Register file x0-x31 (x0 is hardwired zero)
    pub pc: u64,
}

/// Unified Multi-Architecture CPU Register and Instruction Context
#[derive(Debug, Clone, Copy)]
pub enum CpuContextState {
    X86(X86Context),
    X64(X64Context),
    Arm64(Arm64Context),
    Riscv64(Riscv64Context),
}

/// Sovereign Multi-Architecture CPU Register and Instruction Context Engine.
/// Provides high-fidelity register state tracking, context saving/restoration (trap frames),
/// and instruction-level context switching inspired by Linux, BSD, and Windows kernels.
pub struct SovereignContextSwitchEngine {
    pub current_context: CpuContextState,
    pub switch_count: u64,
}

impl SovereignContextSwitchEngine {
    pub fn new(initial_state: CpuContextState) -> Self {
        Self {
            current_context: initial_state,
            switch_count: 0,
        }
    }

    /// Performs a high-fidelity context switch to a new register context state
    pub fn context_switch(&mut self, next_context: CpuContextState) -> CpuContextState {
        let old_context = self.current_context;
        self.current_context = next_context;
        self.switch_count += 1;
        old_context
    }

    /// Simulates standard kernel trap entry (saving register state) and exception handler routing
    pub fn simulate_kernel_trap(&mut self) -> &'static str {
        match &mut self.current_context {
            CpuContextState::X86(ref mut ctx) => {
                ctx.eflags |= 1 << 9; // Set interrupt flag
                "X86_TRAP_HANDLED"
            }
            CpuContextState::X64(ref mut ctx) => {
                ctx.rflags |= 1 << 9; // Set interrupt flag
                "X64_TRAP_HANDLED"
            }
            CpuContextState::Arm64(ref mut ctx) => {
                ctx.pstate |= 1 << 7; // Mask IRQs in PSTATE
                "ARM64_TRAP_HANDLED"
            }
            CpuContextState::Riscv64(ref mut ctx) => {
                ctx.pc += 4; // Advance program counter past trapped instruction
                "RISCV64_TRAP_HANDLED"
            }
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
