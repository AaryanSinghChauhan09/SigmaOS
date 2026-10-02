//! AArch64 (ARM64) Architecture Support
//! Inspired by Linux arch/arm64/ and FreeBSD sys/arm64/.
//! Targets: Raspberry Pi 5, Apple M-series, Qualcomm Snapdragon, AWS Graviton.
//!
//! References:
//! - ARM Architecture Reference Manual: https://developer.arm.com/documentation/
//! - Linux arch/arm64/: https://github.com/torvalds/linux/tree/master/arch/arm64
//! - FreeBSD sys/arm64/: https://github.com/freebsd/freebsd-src/tree/main/sys/arm64

/// AArch64 general-purpose registers (x0-x30 + sp + pc)
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct AArch64Regs {
    pub x: [u64; 31],  // x0-x30 (x30 = link register)
    pub sp: u64,        // stack pointer
    pub pc: u64,        // program counter
    pub pstate: u64,    // processor state (NZCV + DAIF + CurrentEL + SPSel)
}

impl AArch64Regs {
    /// Get the link register (return address).
    pub fn lr(&self) -> u64 { self.x[30] }
    /// Get the frame pointer.
    pub fn fp(&self) -> u64 { self.x[29] }
    /// Get the first argument register.
    pub fn arg0(&self) -> u64 { self.x[0] }
    /// Check if execution is at EL1 (kernel mode).
    pub fn is_el1(&self) -> bool { (self.pstate >> 2) & 0x3 == 1 }
    /// Check if execution is at EL0 (userspace).
    pub fn is_el0(&self) -> bool { (self.pstate >> 2) & 0x3 == 0 }
}

/// AArch64 PSTATE flags
pub mod pstate {
    pub const N: u64 = 1 << 31; // Negative flag
    pub const Z: u64 = 1 << 30; // Zero flag
    pub const C: u64 = 1 << 29; // Carry flag
    pub const V: u64 = 1 << 28; // Overflow flag
    pub const D: u64 = 1 << 9;  // Debug exception mask
    pub const A: u64 = 1 << 8;  // SError interrupt mask
    pub const I: u64 = 1 << 7;  // IRQ mask
    pub const F: u64 = 1 << 6;  // FIQ mask
    pub const SP: u64 = 1 << 0; // Stack pointer select (1=SP_ELx, 0=SP_EL0)
}

/// AArch64 system register IDs (MSR/MRS instructions)
#[derive(Debug, Clone, Copy)]
pub enum SysReg {
    /// Translation Table Base Register 0 (TTBR0_EL1 — userspace page tables)
    Ttbr0El1,
    /// Translation Table Base Register 1 (TTBR1_EL1 — kernel page tables)
    Ttbr1El1,
    /// Translation Control Register (TCR_EL1)
    TcrEl1,
    /// Memory Attribute Indirection Register (MAIR_EL1)
    MairEl1,
    /// Saved Program Status Register (SPSR_EL1)
    SpsrEl1,
    /// Exception Link Register (ELR_EL1 — return address for exceptions)
    ElrEl1,
    /// System Control Register (SCTLR_EL1 — MMU enable, caches, alignment)
    SctlrEl1,
    /// Exception Syndrome Register (ESR_EL1 — fault type and info)
    EsrEl1,
    /// Fault Address Register (FAR_EL1 — faulting virtual address)
    FarEl1,
    /// Counter Frequency Register (CNTFRQ_EL0)
    CntfrqEl0,
    /// Virtual Counter (CNTVCT_EL0 — monotonic clock)
    CntvctEl0,
    /// Process ID Register (TPIDR_EL0 — thread-local storage pointer)
    TpidrEl0,
}

/// AArch64 page table descriptor bits (4KB granule, 48-bit VA)
pub mod pte {
    pub const VALID: u64 = 1 << 0;
    pub const TABLE: u64 = 1 << 1;     // 1=table descriptor, 0=block
    pub const USER: u64 = 1 << 6;      // AP[1]: 0=EL1 only, 1=EL0+EL1
    pub const READ_ONLY: u64 = 1 << 7; // AP[2]: 0=RW, 1=RO
    pub const ACCESSED: u64 = 1 << 10; // AF: Access Flag
    pub const NON_GLOBAL: u64 = 1 << 11; // nG: 1=ASID applies
    pub const PXN: u64 = 1 << 53;      // Privileged Execute-Never
    pub const UXN: u64 = 1 << 54;      // Unprivileged Execute-Never (user XN)
    pub const PHYS_ADDR_MASK: u64 = 0x0000_FFFF_FFFF_F000; // bits[47:12]
}

/// AArch64 exception vector table entry types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionType {
    SynchronousEl1t,    // Synchronous from EL1 with SP_EL0
    IrqEl1t,
    FiqEl1t,
    SErrorEl1t,
    SynchronousEl1h,    // Synchronous from EL1 with SP_EL1 (kernel fault)
    IrqEl1h,
    FiqEl1h,
    SErrorEl1h,
    SynchronousEl0_64,  // Synchronous from EL0 (64-bit AArch64 userspace)
    IrqEl0_64,
    FiqEl0_64,
    SErrorEl0_64,
    SynchronousEl0_32,  // Synchronous from EL0 (32-bit AArch32 compat)
    IrqEl0_32,
    FiqEl0_32,
    SErrorEl0_32,
}

/// AArch64 ESR_EL1 exception class (EC field, bits[31:26])
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionClass {
    Unknown         = 0x00,
    Wf              = 0x01,  // WFI/WFE
    SvcAArch64      = 0x15,  // SVC in AArch64
    MsrMrs          = 0x18,  // MSR/MRS access trap
    InstructionAbortLower = 0x20, // Instruction abort from lower EL
    InstructionAbortSame  = 0x21, // Instruction abort from same EL
    PcAlignment     = 0x22,  // PC alignment fault
    DataAbortLower  = 0x24,  // Data abort from lower EL
    DataAbortSame   = 0x25,  // Data abort from same EL
    SpAlignment     = 0x26,  // SP alignment fault
    FpAArch64       = 0x2C,  // FP/SIMD exception
    SError          = 0x2F,  // SError interrupt
    Brk             = 0x3C,  // BRK instruction (software breakpoint)
}

/// Decode ESR_EL1 into a human-readable exception class.
pub fn decode_esr(esr: u64) -> ExceptionClass {
    match (esr >> 26) & 0x3F {
        0x00 => ExceptionClass::Unknown,
        0x15 => ExceptionClass::SvcAArch64,
        0x20 => ExceptionClass::InstructionAbortLower,
        0x21 => ExceptionClass::InstructionAbortSame,
        0x24 => ExceptionClass::DataAbortLower,
        0x25 => ExceptionClass::DataAbortSame,
        0x2C => ExceptionClass::FpAArch64,
        0x3C => ExceptionClass::Brk,
        _ => ExceptionClass::Unknown,
    }
}

/// AArch64 SCTLR_EL1 control bits
pub mod sctlr {
    pub const M: u64 = 1 << 0;    // MMU enable
    pub const A: u64 = 1 << 1;    // Alignment check enable
    pub const C: u64 = 1 << 2;    // Data cache enable
    pub const SA: u64 = 1 << 3;   // SP alignment check (EL1)
    pub const SA0: u64 = 1 << 4;  // SP alignment check (EL0)
    pub const I: u64 = 1 << 12;   // Instruction cache enable
    pub const UCT: u64 = 1 << 15; // CTR_EL0 access at EL0
    pub const SPAN: u64 = 1 << 23; // Set privileged access never
    pub const UCI: u64 = 1 << 26;  // DC CVAU/CVAC/CIVAC at EL0
    pub const TCMA0: u64 = 1 << 30; // Tag Check Mask at EL0 (MTE)
}

/// Architecture-specific CPU feature detection for AArch64
#[derive(Debug, Clone, Default)]
pub struct AArch64Features {
    pub has_pac: bool,      // Pointer Authentication (ARMv8.3)
    pub has_bti: bool,      // Branch Target Identification (ARMv8.5)
    pub has_mte: bool,      // Memory Tagging Extension (ARMv8.5)
    pub has_sve: bool,      // Scalable Vector Extension
    pub has_sme: bool,      // Scalable Matrix Extension (ARMv9)
    pub has_crypto: bool,   // Cryptographic extensions (AES, SHA)
    pub has_fp: bool,       // Floating-point / NEON
    pub has_lse: bool,      // Large System Extension (atomics, ARMv8.1)
    pub has_crc32: bool,    // CRC32 instructions
    pub num_cores: u32,
    pub implementer: u8,    // 0x41=ARM, 0x50=Apple, 0x51=Qualcomm, 0x61=Apple
}

impl AArch64Features {
    /// Detect features by reading ID registers (simulated).
    pub fn detect() -> Self {
        // In production: read ID_AA64ISAR0_EL1, ID_AA64ISAR1_EL1, etc.
        Self {
            has_pac: true,
            has_bti: true,
            has_mte: false,
            has_sve: false,
            has_sme: false,
            has_crypto: true,
            has_fp: true,
            has_lse: true,
            has_crc32: true,
            num_cores: 4,
            implementer: 0x41, // ARM Ltd
        }
    }

    pub fn implementer_name(&self) -> &'static str {
        match self.implementer {
            0x41 => "ARM",
            0x50 => "Ampere",
            0x51 => "Qualcomm",
            0x61 => "Apple",
            0x68 => "Huawei/HiSilicon",
            0xC0 => "Ampere",
            _ => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_defaults() {
        let regs = AArch64Regs::default();
        assert_eq!(regs.lr(), 0);
        assert_eq!(regs.sp, 0);
    }

    #[test]
    fn test_el_detection() {
        let mut regs = AArch64Regs::default();
        regs.pstate = 0b0100; // CurrentEL = 01 (EL1)
        assert!(regs.is_el1());
        assert!(!regs.is_el0());
    }

    #[test]
    fn test_esr_decode_svc() {
        let esr = 0x15 << 26; // SVC64
        assert_eq!(decode_esr(esr), ExceptionClass::SvcAArch64);
    }

    #[test]
    fn test_esr_decode_data_abort() {
        let esr = (0x24u64 << 26) | 0b00100; // DataAbortLower + Translation fault
        assert_eq!(decode_esr(esr), ExceptionClass::DataAbortLower);
    }

    #[test]
    fn test_feature_detection() {
        let feat = AArch64Features::detect();
        assert!(feat.has_fp);
        assert!(feat.has_lse);
        assert_eq!(feat.implementer_name(), "ARM");
    }
}
