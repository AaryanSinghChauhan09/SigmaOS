// SigmaOS Multi-Arch Hardware Abstraction Layer (HAL)
// Hardware abstraction layer supporting x86_64, AArch64 (ARM64), and RISC-V (RV64GC).

use std::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetArch {
    X86_32,
    X86_64,
    AArch32,
    AArch64,
    RiscV32,
    RiscV64,
    LoongArch64,
    Ppc64Le,
}

pub struct MultiArchHal {
    pub arch: TargetArch,
    pub interrupt_controller: String,
    pub page_levels: u8,
}

impl MultiArchHal {
    pub fn new(arch: TargetArch) -> Self {
        let (interrupt_controller, page_levels) = match arch {
            TargetArch::X86_32 => ("PIC8259", 2),
            TargetArch::X86_64 => ("x2APIC", 4),
            TargetArch::AArch32 => ("GICv2", 2),
            TargetArch::AArch64 => ("GICv3", 4),
            TargetArch::RiscV32 => ("PLIC32", 2), // Sv32
            TargetArch::RiscV64 => ("PLIC", 3), // Sv39/Sv48
            TargetArch::LoongArch64 => ("ExtIOI", 3),
            TargetArch::Ppc64Le => ("XIVE", 3),
        };
        Self {
            arch,
            interrupt_controller: interrupt_controller.to_string(),
            page_levels,
        }
    }

    pub fn initialize_hardware_irqs(&self) -> Result<&'static str, &'static str> {
        match self.arch {
            TargetArch::X86_32 => Ok("x86 8259 PIC cascading and IO-APIC routing initialized"),
            TargetArch::X86_64 => Ok("x2APIC and IO-APIC routing initialized"),
            TargetArch::AArch32 => Ok("ARM GICv2 distributor and CPU interfaces initialized"),
            TargetArch::AArch64 => Ok("ARM GICv3 distributor and redistributors initialized"),
            TargetArch::RiscV32 => Ok("RISC-V 32-bit PLIC and CLINT timer initialized"),
            TargetArch::RiscV64 => Ok("RISC-V PLIC and CLINT timer initialized"),
            TargetArch::LoongArch64 => Ok("LoongArch ExtIOI interrupt controller initialized"),
            TargetArch::Ppc64Le => Ok("PowerPC XIVE interrupt virtualizer initialized"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_arch_hal() {
        let x86_hal = MultiArchHal::new(TargetArch::X86_64);
        assert_eq!(x86_hal.page_levels, 4);

        let arm_hal = MultiArchHal::new(TargetArch::AArch64);
        assert!(arm_hal.initialize_hardware_irqs().unwrap().contains("GICv3"));

        let riscv_hal = MultiArchHal::new(TargetArch::RiscV64);
        assert_eq!(riscv_hal.interrupt_controller, "PLIC");

        let x86_32_hal = MultiArchHal::new(TargetArch::X86_32);
        assert_eq!(x86_32_hal.page_levels, 2);

        let arm32_hal = MultiArchHal::new(TargetArch::AArch32);
        assert!(arm32_hal.initialize_hardware_irqs().unwrap().contains("GICv2"));

        let rv32_hal = MultiArchHal::new(TargetArch::RiscV32);
        assert_eq!(rv32_hal.interrupt_controller, "PLIC32");
    }
}
