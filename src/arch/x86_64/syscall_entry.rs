//! x86_64 fast-syscall entry configuration.
//!
//! This module owns the architectural contract for `SYSCALL`/`SYSRETQ`.
//! Writing model-specific registers is intentionally unsafe and is not
//! performed by the host-side library initializer. A boot path must install
//! this only after the GDT, TSS, kernel stack, and user page-table policy are
//! active.

use core::arch::asm;

pub const IA32_STAR: u32 = 0xC000_0081;
pub const IA32_LSTAR: u32 = 0xC000_0082;
pub const IA32_FMASK: u32 = 0xC000_0084;
pub const IA32_KERNEL_GS_BASE: u32 = 0xC000_0102;

pub const KERNEL_CS: u16 = 0x08;
/// STAR stores the user code selector base; SYSRETQ derives user CS/SS from it.
pub const SYSRET_USER_CS_BASE: u16 = 0x20;
pub const USER_CS: u16 = SYSRET_USER_CS_BASE | 3;
pub const USER_DS: u16 = (SYSRET_USER_CS_BASE + 8) | 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallEntryError {
    NonCanonicalEntry,
    MisalignedEntry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyscallEntryConfig {
    pub entry_point: u64,
    pub kernel_gs_base: u64,
    pub star: u64,
    pub rflags_mask: u64,
}

impl SyscallEntryConfig {
    pub fn new(entry_point: u64, kernel_gs_base: u64) -> Result<Self, SyscallEntryError> {
        if entry_point & 0xF != 0 {
            return Err(SyscallEntryError::MisalignedEntry);
        }
        if !is_canonical(entry_point) {
            return Err(SyscallEntryError::NonCanonicalEntry);
        }

        Ok(Self {
            entry_point,
            kernel_gs_base,
            star: ((SYSRET_USER_CS_BASE as u64) << 48) | ((KERNEL_CS as u64) << 32),
            // Mask IF, direction, and alignment-check flags on syscall entry.
            rflags_mask: (1 << 9) | (1 << 10) | (1 << 18),
        })
    }

    /// Install the syscall MSRs on the current CPU.
    ///
    /// # Safety
    /// The caller must execute in ring 0, provide a kernel-owned entry stub,
    /// and guarantee that `kernel_gs_base` points to valid per-CPU state.
    pub unsafe fn install(self) {
        write_msr(IA32_STAR, self.star);
        write_msr(IA32_LSTAR, self.entry_point);
        write_msr(IA32_FMASK, self.rflags_mask);
        write_msr(IA32_KERNEL_GS_BASE, self.kernel_gs_base);
    }
}

#[inline]
const fn is_canonical(address: u64) -> bool {
    let upper = address >> 48;
    upper == 0 || upper == 0xFFFF
}

#[inline]
unsafe fn write_msr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    asm!(
        "wrmsr",
        in("ecx") msr,
        in("eax") low,
        in("edx") high,
        options(nostack, preserves_flags)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_syscall_selector_contract() {
        let config = SyscallEntryConfig::new(0xFFFF_8000_0040_0000, 0xFFFF_8000_0010_0000).unwrap();
        assert_eq!(config.star >> 32, 0x20_0008);
        assert_eq!(USER_CS, 0x23);
        assert_eq!(USER_DS, 0x2B);
        assert_ne!(config.rflags_mask & (1 << 9), 0);
    }

    #[test]
    fn rejects_invalid_entry_points() {
        assert_eq!(
            SyscallEntryConfig::new(0x400001, 0),
            Err(SyscallEntryError::MisalignedEntry)
        );
        assert_eq!(
            SyscallEntryConfig::new(0x0001_0000_0000_0000, 0),
            Err(SyscallEntryError::NonCanonicalEntry)
        );
    }
}
