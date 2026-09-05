use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Target CPU Architectures supported by SigmaOS Multi-Arch HAL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArchitecture {
    X86_32,
    X86_64,
    Armv7,
    AArch64,
    Riscv32,
    Riscv64,
    LoongArch64,
    Ppc64Le,
}

/// System Interrupt Controller Abstraction (x86 PIC/APIC/IOAPIC, ARM GICv2/v3, RISC-V PLIC/CLINT, LoongArch ExtIOI, PowerPC XIVE)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptControllerKind {
    X86Pic8259,
    X86ApicIoApic,
    ArmGicV2,
    ArmGicV3,
    RiscvPlicClint,
    LoongArchExtIoi,
    PpcXive,
}

/// Architecture-specific register context snapshot
#[derive(Debug, Clone)]
pub enum CpuRegisterContext {
    X86_32 {
        eax: u32,
        ebx: u32,
        ecx: u32,
        edx: u32,
        esi: u32,
        edi: u32,
        esp: u32,
        eip: u32,
        eflags: u32,
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
    Armv7 {
        r: [u32; 13],
        sp: u32,
        lr: u32,
        pc: u32,
        cpsr: u32,
        ttbr0: u32,
    },
    AArch64 {
        x: [u64; 31],
        sp: u64,
        pc: u64,
        pstate: u64,
        ttbr0_el1: u64,
    },
    Riscv32 {
        x: [u32; 32],
        pc: u32,
        sstatus: u32,
        satp: u32,
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
}

/// MMIO Page Fault Information
#[derive(Debug, Clone)]
pub struct MmioPageFault {
    pub faulting_address: u64,
    pub is_write: bool,
    pub instruction_pointer: u64,
    pub target_arch: TargetArchitecture,
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
            TargetArchitecture::X86_32 => InterruptControllerKind::X86Pic8259,
            TargetArchitecture::X86_64 => InterruptControllerKind::X86ApicIoApic,
            TargetArchitecture::Armv7 => InterruptControllerKind::ArmGicV2,
            TargetArchitecture::AArch64 => InterruptControllerKind::ArmGicV3,
            TargetArchitecture::Riscv32 | TargetArchitecture::Riscv64 => InterruptControllerKind::RiscvPlicClint,
            TargetArchitecture::LoongArch64 => InterruptControllerKind::LoongArchExtIoi,
            TargetArchitecture::Ppc64Le => InterruptControllerKind::PpcXive,
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
            TargetArchitecture::X86_32 => CpuRegisterContext::X86_32 {
                eax: 0,
                ebx: 0,
                ecx: 0,
                edx: 0,
                esi: 0,
                edi: 0,
                esp: 0x00C00000,
                eip: 0x00010000,
                eflags: 0x202,
                cr3: 0x1000,
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
            TargetArchitecture::Armv7 => CpuRegisterContext::Armv7 {
                r: [0u32; 13],
                sp: 0x20000000,
                lr: 0,
                pc: 0x00010000,
                cpsr: 0x10,
                ttbr0: 0x1000,
            },
            TargetArchitecture::AArch64 => CpuRegisterContext::AArch64 {
                x: [0u64; 31],
                sp: 0x40000000,
                pc: 0x00400000,
                pstate: 0x3C5,
                ttbr0_el1: 0x2000,
            },
            TargetArchitecture::Riscv32 => CpuRegisterContext::Riscv32 {
                x: [0u32; 32],
                pc: 0x80000000,
                sstatus: 0x00000020,
                satp: 0x80001000,
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let null_fault = MmioPageFault {
            faulting_address: 0,
            is_write: false,
            instruction_pointer: 0x400000,
            target_arch: TargetArchitecture::X86_64,
        };
        assert!(hal_x86.handle_mmio_page_fault(&null_fault).is_err());

        let hal_arm = MultiArchHalManager::new(TargetArchitecture::AArch64);
        assert_eq!(hal_arm.irq_controller, InterruptControllerKind::ArmGicV3);
        if let CpuRegisterContext::AArch64 { sp, .. } = hal_arm.create_default_context() {
            assert_eq!(sp, 0x40000000);
        } else {
            panic!("Expected AArch64 register context");
        }

        let hal_riscv = MultiArchHalManager::new(TargetArchitecture::Riscv64);
        assert_eq!(hal_riscv.irq_controller, InterruptControllerKind::RiscvPlicClint);
        if let CpuRegisterContext::Riscv64 { pc, .. } = hal_riscv.create_default_context() {
            assert_eq!(pc, 0x80000000);
        } else {
            panic!("Expected Riscv64 register context");
        }

        let hal_loongarch = MultiArchHalManager::new(TargetArchitecture::LoongArch64);
        assert_eq!(hal_loongarch.irq_controller, InterruptControllerKind::LoongArchExtIoi);
        if let CpuRegisterContext::LoongArch64 { era, .. } = hal_loongarch.create_default_context() {
            assert_eq!(era, 0x9000000000000000);
        } else {
            panic!("Expected LoongArch64 register context");
        }

        let hal_ppc = MultiArchHalManager::new(TargetArchitecture::Ppc64Le);
        assert_eq!(hal_ppc.irq_controller, InterruptControllerKind::PpcXive);
        if let CpuRegisterContext::Ppc64Le { nip, .. } = hal_ppc.create_default_context() {
            assert_eq!(nip, 0x0000000000000100);
        } else {
            panic!("Expected Ppc64Le register context");
        }

        let hal_x86_32 = MultiArchHalManager::new(TargetArchitecture::X86_32);
        assert_eq!(hal_x86_32.irq_controller, InterruptControllerKind::X86Pic8259);

        let hal_armv7 = MultiArchHalManager::new(TargetArchitecture::Armv7);
        assert_eq!(hal_armv7.irq_controller, InterruptControllerKind::ArmGicV2);

        let hal_rv32 = MultiArchHalManager::new(TargetArchitecture::Riscv32);
        assert_eq!(hal_rv32.irq_controller, InterruptControllerKind::RiscvPlicClint);
    }
}
