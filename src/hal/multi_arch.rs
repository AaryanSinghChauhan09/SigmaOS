use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Target CPU Architectures supported by SigmaOS Multi-Arch HAL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArchitecture {
    X86,
    X86_64,
    AArch64,
    Armv7,
    Riscv64,
    LoongArch64,
    Ppc64Le,
    Mips64,
    S390x,
    Sparc64,
}

impl TargetArchitecture {
    /// Returns the canonical GNU target triplet string for cross-compilation toolchains.
    pub fn to_gnu_triplet(&self) -> &'static str {
        match self {
            TargetArchitecture::X86 => "i686-sigmaos-linux-gnu",
            TargetArchitecture::X86_64 => "x86_64-sigmaos-linux-gnu",
            TargetArchitecture::AArch64 => "aarch64-sigmaos-linux-gnu",
            TargetArchitecture::Armv7 => "armv7-sigmaos-linux-gnueabihf",
            TargetArchitecture::Riscv64 => "riscv64-sigmaos-linux-gnu",
            TargetArchitecture::LoongArch64 => "loongarch64-sigmaos-linux-gnu",
            TargetArchitecture::Ppc64Le => "powerpc64le-sigmaos-linux-gnu",
            TargetArchitecture::Mips64 => "mips64el-sigmaos-linux-gnu",
            TargetArchitecture::S390x => "s390x-sigmaos-linux-gnu",
            TargetArchitecture::Sparc64 => "sparc64-sigmaos-linux-gnu",
        }
    }
}

/// System Interrupt Controller Abstraction (x86 PIC/APIC, ARM GICv2/v3, RISC-V PLIC, LoongArch ExtIOI, PowerPC XIVE, MIPS GIC, IBM S390x SCLP, SPARC Monddo)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptControllerKind {
    X86PicApic,
    X86ApicIoApic,
    ArmGicV2,
    ArmGicV3,
    RiscvPlicClint,
    LoongArchExtIoi,
    PpcXive,
    MipsGic,
    S390xSclp,
    Sparc64Monddo,
}

/// Architecture-specific register context snapshot
#[derive(Debug, Clone)]
pub enum CpuRegisterContext {
    X86 {
        eax: u32,
        ebx: u32,
        ecx: u32,
        edx: u32,
        esi: u32,
        edi: u32,
        esp: u32,
        eip: u32,
        eflags: u32,
        cr0: u32,
        cr3: u32,
    },
    X86_64 {
        rax: u64,
        rbx: u64,
        rcx: u64,
        rdx: u64,
        rsi: u64,
        rdi: u64,
        rsp: u64,
        rip: u64,
        rflags: u64,
        cr3: u64,
    },
    AArch64 {
        x: [u64; 31],
        sp: u64,
        pc: u64,
        pstate: u64,
        ttbr0_el1: u64,
    },
    Armv7 {
        r: [u32; 13],
        sp: u32,
        lr: u32,
        pc: u32,
        cpsr: u32,
    },
    Riscv64 {
        x: [u64; 32],
        pc: u64,
        sstatus: u64,
        satp: u64,
    },
    LoongArch64 {
        r: [u64; 32],
        era: u64,
        prmd: u64,
        pgdl: u64,
    },
    Ppc64Le {
        gpr: [u64; 32],
        nip: u64,
        msr: u64,
        lr: u64,
    },
    Mips64 {
        gpr: [u64; 32],
        pc: u64,
        status: u64,
        cause: u64,
    },
    S390x {
        gprs: [u64; 16],
        psw_mask: u64,
        psw_addr: u64,
    },
    Sparc64 {
        gpr: [u64; 32],
        pc: u64,
        tpc: u64,
        pstate: u64,
    },
}

/// MMIO Page Fault Information
#[derive(Debug, Clone)]
pub struct MmioPageFault {
    pub faulting_address: u64,
    pub is_write: bool,
    pub instruction_pointer: u64,
    pub target_arch: TargetArchitecture,
}

/// Dynamic Multi-Architecture ELF Binary Header Inspector
#[derive(Debug, Clone)]
pub struct MultiArchElfHeader {
    pub e_machine: u16,
    pub is_64bit: bool,
    pub is_little_endian: bool,
    pub entry_point: u64,
}

impl MultiArchElfHeader {
    /// Detects the target architecture from ELF `e_machine` values.
    pub fn detect_architecture(&self) -> Result<TargetArchitecture, &'static str> {
        match self.e_machine {
            0x03 => Ok(TargetArchitecture::X86),
            0x3E => Ok(TargetArchitecture::X86_64),
            0x28 => Ok(TargetArchitecture::Armv7),
            0xB7 => Ok(TargetArchitecture::AArch64),
            0xF3 => Ok(TargetArchitecture::Riscv64),
            0x102 => Ok(TargetArchitecture::LoongArch64),
            0x15 => Ok(TargetArchitecture::Ppc64Le),
            0x08 => Ok(TargetArchitecture::Mips64),
            0x16 => Ok(TargetArchitecture::S390x),
            0x2B => Ok(TargetArchitecture::Sparc64),
            _ => Err("Unsupported or unknown ELF e_machine architecture identifier"),
        }
    }
}

/// Multi-Architecture Hardware Abstraction Layer Manager
pub struct MultiArchHalManager {
    pub current_arch: TargetArchitecture,
    pub irq_controller: InterruptControllerKind,
    pub timer_frequency_hz: u64,
    pub registered_irq_handlers: Vec<(u32, String)>,
}

impl MultiArchHalManager {
    pub fn new(arch: TargetArchitecture) -> Self {
        let irq_controller = match arch {
            TargetArchitecture::X86 => InterruptControllerKind::X86PicApic,
            TargetArchitecture::X86_64 => InterruptControllerKind::X86ApicIoApic,
            TargetArchitecture::AArch64 => InterruptControllerKind::ArmGicV3,
            TargetArchitecture::Armv7 => InterruptControllerKind::ArmGicV2,
            TargetArchitecture::Riscv64 => InterruptControllerKind::RiscvPlicClint,
            TargetArchitecture::LoongArch64 => InterruptControllerKind::LoongArchExtIoi,
            TargetArchitecture::Ppc64Le => InterruptControllerKind::PpcXive,
            TargetArchitecture::Mips64 => InterruptControllerKind::MipsGic,
            TargetArchitecture::S390x => InterruptControllerKind::S390xSclp,
            TargetArchitecture::Sparc64 => InterruptControllerKind::Sparc64Monddo,
        };

        Self {
            current_arch: arch,
            irq_controller,
            timer_frequency_hz: 1000,
            registered_irq_handlers: Vec::new(),
        }
    }

    pub fn register_irq_handler(&mut self, irq: u32, handler_name: &str) -> Result<(), &'static str> {
        if self.registered_irq_handlers.iter().any(|(i, _)| *i == irq) {
            return Err("IRQ handler already registered");
        }
        self.registered_irq_handlers.push((irq, handler_name.to_string()));
        Ok(())
    }

    pub fn handle_mmio_page_fault(&self, fault: &MmioPageFault) -> Result<String, &'static str> {
        if fault.faulting_address == 0 {
            return Err("NULL pointer MMIO page fault violation");
        }

        let access = if fault.is_write { "WRITE" } else { "READ" };
        Ok(format!(
            "[{:?}] Handled MMIO {} page fault at 0x{:016X} (RIP/PC: 0x{:016X})",
            fault.target_arch, access, fault.faulting_address, fault.instruction_pointer
        ))
    }

    pub fn create_default_context(&self) -> CpuRegisterContext {
        match self.current_arch {
            TargetArchitecture::X86 => CpuRegisterContext::X86 {
                eax: 0,
                ebx: 0,
                ecx: 0,
                edx: 0,
                esi: 0,
                edi: 0,
                esp: 0xC0000000,
                eip: 0x00100000,
                eflags: 0x00000202,
                cr0: 0x80000001,
                cr3: 0x00001000,
            },
            TargetArchitecture::X86_64 => CpuRegisterContext::X86_64 {
                rax: 0,
                rbx: 0,
                rcx: 0,
                rdx: 0,
                rsi: 0,
                rdi: 0,
                rsp: 0x7FFF0000,
                rip: 0x400000,
                rflags: 0x202,
                cr3: 0x1000,
            },
            TargetArchitecture::AArch64 => CpuRegisterContext::AArch64 {
                x: [0u64; 31],
                sp: 0x40000000,
                pc: 0x00400000,
                pstate: 0x3C5,
                ttbr0_el1: 0x2000,
            },
            TargetArchitecture::Armv7 => CpuRegisterContext::Armv7 {
                r: [0u32; 13],
                sp: 0x80000000,
                lr: 0,
                pc: 0x00010000,
                cpsr: 0x10,
            },
            TargetArchitecture::Riscv64 => CpuRegisterContext::Riscv64 {
                x: [0u64; 32],
                pc: 0x80000000,
                sstatus: 0x00000020,
                satp: 0x8000000000003000,
            },
            TargetArchitecture::LoongArch64 => CpuRegisterContext::LoongArch64 {
                r: [0u64; 32],
                era: 0x9000000000000000,
                prmd: 0x4,
                pgdl: 0x1000,
            },
            TargetArchitecture::Ppc64Le => CpuRegisterContext::Ppc64Le {
                gpr: [0u64; 32],
                nip: 0x0000000000000100,
                msr: 0x8000000000009033,
                lr: 0,
            },
            TargetArchitecture::Mips64 => CpuRegisterContext::Mips64 {
                gpr: [0u64; 32],
                pc: 0xFFFFFFFF80000000,
                status: 0x24000000,
                cause: 0,
            },
            TargetArchitecture::S390x => CpuRegisterContext::S390x {
                gprs: [0u64; 16],
                psw_mask: 0x0705000180000000,
                psw_addr: 0x0000000000010000,
            },
            TargetArchitecture::Sparc64 => CpuRegisterContext::Sparc64 {
                gpr: [0u64; 32],
                pc: 0x0000000000400000,
                tpc: 0,
                pstate: 0x16,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnu_triplets() {
        assert_eq!(TargetArchitecture::X86.to_gnu_triplet(), "i686-sigmaos-linux-gnu");
        assert_eq!(TargetArchitecture::X86_64.to_gnu_triplet(), "x86_64-sigmaos-linux-gnu");
        assert_eq!(TargetArchitecture::AArch64.to_gnu_triplet(), "aarch64-sigmaos-linux-gnu");
        assert_eq!(TargetArchitecture::Armv7.to_gnu_triplet(), "armv7-sigmaos-linux-gnueabihf");
        assert_eq!(TargetArchitecture::Riscv64.to_gnu_triplet(), "riscv64-sigmaos-linux-gnu");
        assert_eq!(TargetArchitecture::Mips64.to_gnu_triplet(), "mips64el-sigmaos-linux-gnu");
        assert_eq!(TargetArchitecture::S390x.to_gnu_triplet(), "s390x-sigmaos-linux-gnu");
        assert_eq!(TargetArchitecture::Sparc64.to_gnu_triplet(), "sparc64-sigmaos-linux-gnu");
    }

    #[test]
    fn test_elf_header_detection() {
        let elf = MultiArchElfHeader {
            e_machine: 0xF3,
            is_64bit: true,
            is_little_endian: true,
            entry_point: 0x80000000,
        };
        assert_eq!(elf.detect_architecture().unwrap(), TargetArchitecture::Riscv64);

        let unknown_elf = MultiArchElfHeader {
            e_machine: 0xFFFF,
            is_64bit: true,
            is_little_endian: true,
            entry_point: 0x0,
        };
        assert!(unknown_elf.detect_architecture().is_err());
    }

    #[test]
    fn test_x86_32bit_hal_manager() {
        let hal_x86 = MultiArchHalManager::new(TargetArchitecture::X86);
        assert_eq!(hal_x86.irq_controller, InterruptControllerKind::X86PicApic);
        if let CpuRegisterContext::X86 { esp, eip, eflags, .. } = hal_x86.create_default_context() {
            assert_eq!(esp, 0xC0000000);
            assert_eq!(eip, 0x00100000);
            assert_eq!(eflags, 0x00000202);
        } else {
            panic!("Expected X86 register context");
        }
    }

    #[test]
    fn test_multi_arch_hal_manager() {
        let mut hal_x86 = MultiArchHalManager::new(TargetArchitecture::X86_64);
        assert_eq!(hal_x86.irq_controller, InterruptControllerKind::X86ApicIoApic);
        assert!(hal_x86.register_irq_handler(33, "keyboard_irq").is_ok());
        assert!(hal_x86.register_irq_handler(33, "keyboard_irq_dup").is_err());

        let fault = MmioPageFault {
            faulting_address: 0xFED00000,
            is_write: true,
            instruction_pointer: 0xFFFFFFFF80100000,
            target_arch: TargetArchitecture::X86_64,
        };
        let fault_res = hal_x86.handle_mmio_page_fault(&fault).unwrap();
        assert!(fault_res.contains("Handled MMIO WRITE"));

        let hal_armv7 = MultiArchHalManager::new(TargetArchitecture::Armv7);
        assert_eq!(hal_armv7.irq_controller, InterruptControllerKind::ArmGicV2);
        if let CpuRegisterContext::Armv7 { sp, pc, .. } = hal_armv7.create_default_context() {
            assert_eq!(sp, 0x80000000);
            assert_eq!(pc, 0x00010000);
        } else {
            panic!("Expected Armv7 register context");
        }

        let hal_mips = MultiArchHalManager::new(TargetArchitecture::Mips64);
        assert_eq!(hal_mips.irq_controller, InterruptControllerKind::MipsGic);
        if let CpuRegisterContext::Mips64 { pc, status, .. } = hal_mips.create_default_context() {
            assert_eq!(pc, 0xFFFFFFFF80000000);
            assert_eq!(status, 0x24000000);
        } else {
            panic!("Expected Mips64 register context");
        }

        let hal_s390x = MultiArchHalManager::new(TargetArchitecture::S390x);
        assert_eq!(hal_s390x.irq_controller, InterruptControllerKind::S390xSclp);
        if let CpuRegisterContext::S390x { psw_addr, .. } = hal_s390x.create_default_context() {
            assert_eq!(psw_addr, 0x0000000000010000);
        } else {
            panic!("Expected S390x register context");
        }

        let hal_sparc = MultiArchHalManager::new(TargetArchitecture::Sparc64);
        assert_eq!(hal_sparc.irq_controller, InterruptControllerKind::Sparc64Monddo);
        if let CpuRegisterContext::Sparc64 { pc, pstate, .. } = hal_sparc.create_default_context() {
            assert_eq!(pc, 0x0000000000400000);
            assert_eq!(pstate, 0x16);
        } else {
            panic!("Expected Sparc64 register context");
        }
    }
}
