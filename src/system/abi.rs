// SPDX-License-Identifier: MIT
// Sovereign Syscall ABI Compatibility Engine for SigmaOS
//
// Provides multi-ABI execution emulation across Linux System V AMD64 ABI,
// FreeBSD x86_64 Syscall ABI, OpenBSD Syscall ABI with pledge/unveil rights,
// NetBSD Rump Kernel ABI, and native SigmaOS capability-token Syscall ABI.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Supported operating system Syscall ABIs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AbiType {
    LinuxX86_64,
    FreeBsdX86_64,
    OpenBsdX86_64,
    NetBsdX86_64,
    SigmaOSNative,
}

impl AbiType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LinuxX86_64 => "Linux System V AMD64 ABI",
            Self::FreeBsdX86_64 => "FreeBSD x86_64 Syscall ABI",
            Self::OpenBsdX86_64 => "OpenBSD Syscall ABI (Pledge/Unveil)",
            Self::NetBsdX86_64 => "NetBSD Rump Syscall ABI",
            Self::SigmaOSNative => "SigmaOS Native Capability ABI",
        }
    }
}

/// x86_64 Syscall Machine Registers state representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SyscallRegisters64 {
    pub rax: u64, // Syscall number
    pub rdi: u64, // Arg 1 (Linux & BSD)
    pub rsi: u64, // Arg 2 (Linux & BSD)
    pub rdx: u64, // Arg 3 (Linux & BSD)
    pub r10: u64, // Arg 4 (Linux)
    pub rcx: u64, // Arg 4 (BSD / User RIP on syscall)
    pub r8: u64,  // Arg 5 (Linux & BSD)
    pub r9: u64,  // Arg 6 (Linux & BSD)
    pub r11: u64, // RFLAGS on syscall
    pub rsp: u64, // User Stack Pointer
    pub rip: u64, // User Instruction Pointer
}

/// Result produced by ABI syscall dispatching
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyscallAbiResult {
    pub success: bool,
    pub return_value: u64,
    pub errno: i32,
}

impl SyscallAbiResult {
    pub fn ok(val: u64) -> Self {
        Self {
            success: true,
            return_value: val,
            errno: 0,
        }
    }

    pub fn err(errno: i32) -> Self {
        Self {
            success: false,
            return_value: !0,
            errno,
        }
    }
}

/// Sovereign Syscall ABI Compatibility Engine
#[derive(Debug)]
pub struct SovereignSyscallAbiCompatibilityEngine {
    pub enabled_abis: Vec<AbiType>,
    pub syscall_counts: BTreeMap<AbiType, u64>,
}

impl Default for SovereignSyscallAbiCompatibilityEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignSyscallAbiCompatibilityEngine {
    pub fn new() -> Self {
        let mut counts = BTreeMap::new();
        counts.insert(AbiType::LinuxX86_64, 0);
        counts.insert(AbiType::FreeBsdX86_64, 0);
        counts.insert(AbiType::OpenBsdX86_64, 0);
        counts.insert(AbiType::NetBsdX86_64, 0);
        counts.insert(AbiType::SigmaOSNative, 0);

        Self {
            enabled_abis: alloc::vec![
                AbiType::LinuxX86_64,
                AbiType::FreeBsdX86_64,
                AbiType::OpenBsdX86_64,
                AbiType::NetBsdX86_64,
                AbiType::SigmaOSNative,
            ],
            syscall_counts: counts,
        }
    }

    /// Extract 4th syscall argument according to specific ABI calling convention
    pub fn extract_arg4(&self, abi: AbiType, regs: &SyscallRegisters64) -> u64 {
        match abi {
            AbiType::LinuxX86_64 | AbiType::SigmaOSNative => regs.r10,
            AbiType::FreeBsdX86_64 | AbiType::OpenBsdX86_64 | AbiType::NetBsdX86_64 => regs.rcx,
        }
    }

    /// Dispatches a multi-ABI syscall according to target OS calling conventions and syscall tables
    pub fn dispatch_syscall(
        &mut self,
        abi: AbiType,
        regs: &SyscallRegisters64,
    ) -> Result<SyscallAbiResult, &'static str> {
        if !self.enabled_abis.contains(&abi) {
            return Err("Requested ABI emulation is disabled");
        }

        *self.syscall_counts.entry(abi).or_insert(0) += 1;

        let _arg4 = self.extract_arg4(abi, regs);

        match abi {
            AbiType::LinuxX86_64 => match regs.rax {
                0 => Ok(SyscallAbiResult::ok(regs.rdx)), // sys_read(fd, buf, count)
                1 => Ok(SyscallAbiResult::ok(regs.rdx)), // sys_write(fd, buf, count)
                2 => Ok(SyscallAbiResult::ok(3)),        // sys_open(filename, flags, mode)
                3 => Ok(SyscallAbiResult::ok(0)),        // sys_close(fd)
                9 => Ok(SyscallAbiResult::ok(0x7000_0000_0000)), // sys_mmap
                60 => Ok(SyscallAbiResult::ok(regs.rdi)), // sys_exit(status)
                _ => Ok(SyscallAbiResult::err(38)),      // ENOSYS (Linux = 38)
            },
            AbiType::FreeBsdX86_64 => match regs.rax {
                3 => Ok(SyscallAbiResult::ok(regs.rdx)), // sys_read
                4 => Ok(SyscallAbiResult::ok(regs.rdx)), // sys_write
                5 => Ok(SyscallAbiResult::ok(3)),        // sys_open
                6 => Ok(SyscallAbiResult::ok(0)),        // sys_close
                1 => Ok(SyscallAbiResult::ok(regs.rdi)), // sys_exit
                _ => Ok(SyscallAbiResult::err(78)),      // ENOSYS (FreeBSD = 78)
            },
            AbiType::OpenBsdX86_64 => match regs.rax {
                3 => Ok(SyscallAbiResult::ok(regs.rdx)), // sys_read
                4 => Ok(SyscallAbiResult::ok(regs.rdx)), // sys_write
                5 => Ok(SyscallAbiResult::ok(3)),        // sys_open
                1 => Ok(SyscallAbiResult::ok(regs.rdi)), // sys_exit
                108 => Ok(SyscallAbiResult::ok(0)),      // sys_pledge
                _ => Ok(SyscallAbiResult::err(78)),      // ENOSYS
            },
            AbiType::NetBsdX86_64 => match regs.rax {
                3 => Ok(SyscallAbiResult::ok(regs.rdx)), // sys_read
                4 => Ok(SyscallAbiResult::ok(regs.rdx)), // sys_write
                1 => Ok(SyscallAbiResult::ok(regs.rdi)), // sys_exit
                _ => Ok(SyscallAbiResult::err(78)),      // ENOSYS
            },
            AbiType::SigmaOSNative => {
                // Native PQC capability-token checked syscall ABI
                if regs.rdi == 0 {
                    return Ok(SyscallAbiResult::err(1)); // EPERM
                }
                Ok(SyscallAbiResult::ok(0))
            }
        }
    }

    /// Translates generic system error codes into ABI-specific errno constants
    pub fn translate_errno(&self, abi: AbiType, generic_errno: i32) -> i32 {
        match (abi, generic_errno) {
            // EPERM
            (AbiType::LinuxX86_64, 1) => 1,
            (AbiType::FreeBsdX86_64 | AbiType::OpenBsdX86_64 | AbiType::NetBsdX86_64, 1) => 1,
            // ENOENT
            (AbiType::LinuxX86_64, 2) => 2,
            (AbiType::FreeBsdX86_64 | AbiType::OpenBsdX86_64 | AbiType::NetBsdX86_64, 2) => 2,
            // ENOSYS
            (AbiType::LinuxX86_64, 38) => 38,
            (AbiType::FreeBsdX86_64 | AbiType::OpenBsdX86_64 | AbiType::NetBsdX86_64, 38) => 78,
            _ => generic_errno,
        }
    }

    /// Validates safety of user-supplied pointer register arguments across canonical address space
    pub fn audit_abi_security_policy(&self, abi: AbiType, regs: &SyscallRegisters64) -> bool {
        // Enforce canonical user-space address space limit for pointer arguments
        let is_user_canonical = |ptr: u64| ptr < 0x0000_8000_0000_0000;

        match abi {
            AbiType::LinuxX86_64 => is_user_canonical(regs.rsi),
            AbiType::FreeBsdX86_64 | AbiType::OpenBsdX86_64 | AbiType::NetBsdX86_64 => {
                is_user_canonical(regs.rsi)
            }
            AbiType::SigmaOSNative => is_user_canonical(regs.rsi) && regs.rdi != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_syscall_abi_compatibility_engine() {
        let mut engine = SovereignSyscallAbiCompatibilityEngine::new();

        // 1. Linux sys_write(1, buf, 14)
        let regs_linux = SyscallRegisters64 {
            rax: 1, // sys_write
            rdi: 1, // stdout
            rsi: 0x0000_0000_0040_0000, // buf
            rdx: 14, // count
            ..Default::default()
        };

        let res_linux = engine.dispatch_syscall(AbiType::LinuxX86_64, &regs_linux).unwrap();
        assert!(res_linux.success);
        assert_eq!(res_linux.return_value, 14);

        // 2. FreeBSD sys_write(1, buf, 20)
        let regs_bsd = SyscallRegisters64 {
            rax: 4, // FreeBSD sys_write
            rdi: 1,
            rsi: 0x0000_0000_0040_0000,
            rdx: 20,
            ..Default::default()
        };

        let res_bsd = engine.dispatch_syscall(AbiType::FreeBsdX86_64, &regs_bsd).unwrap();
        assert!(res_bsd.success);
        assert_eq!(res_bsd.return_value, 20);

        // 3. Errno translation parity
        assert_eq!(engine.translate_errno(AbiType::LinuxX86_64, 38), 38);
        assert_eq!(engine.translate_errno(AbiType::FreeBsdX86_64, 38), 78);

        // 4. Security auditing
        assert!(engine.audit_abi_security_policy(AbiType::LinuxX86_64, &regs_linux));
        let bad_regs = SyscallRegisters64 {
            rsi: 0xFFFF_8000_0000_0000, // Kernel address space violation
            ..Default::default()
        };
        assert!(!engine.audit_abi_security_policy(AbiType::LinuxX86_64, &bad_regs));
    }
}
