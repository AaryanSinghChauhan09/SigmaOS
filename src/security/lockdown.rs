// SigmaOS Kernel Lockdown LSM
// Implements one-way security ratchets restricting user-to-kernel boundary breaches.

#![allow(dead_code)]
extern crate alloc;

use core::sync::atomic::{AtomicU8, Ordering};

/// Kernel Lockdown Level (One-way ratchet)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LockdownMode {
    None = 0,
    Integrity = 1,
    Confidentiality = 2,
}

/// Global Atomic Lockdown State
static CURRENT_LOCKDOWN: AtomicU8 = AtomicU8::new(LockdownMode::None as u8);

/// Protected kernel action under lockdown scrutiny
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockdownReason {
    DevMemAccess,
    RawIoPortAccess,
    UnsignedModuleLoading,
    KexecUnsignedKernel,
    MSRWrite,
    AcpiCustomMethodWrite,
    BpfWriteKernelMemory,
    TracefsKernelPointers,
}

/// Kernel Lockdown Controller
#[derive(Debug, Clone, Default)]
pub struct KernelLockdownController {
    pub denials_count: u64,
}

impl KernelLockdownController {
    pub fn new() -> Self {
        Self { denials_count: 0 }
    }

    /// Read current global lockdown level
    pub fn current_level() -> LockdownMode {
        match CURRENT_LOCKDOWN.load(Ordering::Relaxed) {
            1 => LockdownMode::Integrity,
            2 => LockdownMode::Confidentiality,
            _ => LockdownMode::None,
        }
    }

    /// One-way transition to higher lockdown level (cannot be lowered)
    pub fn set_level(new_level: LockdownMode) -> Result<LockdownMode, &'static str> {
        let current = Self::current_level();
        if new_level < current {
            return Err("EPERM: Kernel lockdown level can only be ratcheted higher, never lowered");
        }
        CURRENT_LOCKDOWN.store(new_level as u8, Ordering::SeqCst);
        Ok(new_level)
    }

    /// Check if action is permitted under the current lockdown level
    pub fn verify_action(&mut self, reason: LockdownReason) -> Result<(), &'static str> {
        let level = Self::current_level();

        let violates = match level {
            LockdownMode::None => false,
            LockdownMode::Integrity => matches!(
                reason,
                LockdownReason::DevMemAccess
                    | LockdownReason::RawIoPortAccess
                    | LockdownReason::UnsignedModuleLoading
                    | LockdownReason::KexecUnsignedKernel
                    | LockdownReason::MSRWrite
                    | LockdownReason::AcpiCustomMethodWrite
                    | LockdownReason::BpfWriteKernelMemory
            ),
            LockdownMode::Confidentiality => true, // Blocks all reasons including kernel pointer leaks
        };

        if violates {
            self.denials_count += 1;
            Err("Operation blocked by Kernel Lockdown LSM")
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_lockdown_ratchet_and_enforcement() {
        let mut lsm = KernelLockdownController::new();

        // Initially None -> allowed
        assert!(lsm.verify_action(LockdownReason::DevMemAccess).is_ok());

        // Ratchet to Integrity
        assert!(KernelLockdownController::set_level(LockdownMode::Integrity).is_ok());
        assert_eq!(
            KernelLockdownController::current_level(),
            LockdownMode::Integrity
        );

        // Raw memory access should now be blocked
        assert!(lsm.verify_action(LockdownReason::DevMemAccess).is_err());
        assert_eq!(lsm.denials_count, 1);

        // Attempting to lower to None must fail
        assert!(KernelLockdownController::set_level(LockdownMode::None).is_err());

        // Ratchet up to Confidentiality
        assert!(KernelLockdownController::set_level(LockdownMode::Confidentiality).is_ok());
        assert!(lsm
            .verify_action(LockdownReason::TracefsKernelPointers)
            .is_err());
    }
}
