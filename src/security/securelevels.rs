//! FreeBSD-style Securelevels System for SigmaOS
//! Implements a progressive security model (Securelevels -1 to 3) to protect system integrity.

#![no_std]

use core::sync::atomic::{AtomicI32, Ordering};

/// Securelevels represents the system security modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Securelevel {
    /// Permanently insecure mode. All protections are disabled, and securelevel cannot be raised or lowered.
    PermanentlyInsecure = -1,
    /// Insecure mode. Any transition is allowed, and securelevel can be raised.
    Insecure = 0,
    /// Secure mode. Securelevel can only be raised, not lowered. Disables raw disk writes, kernel memory writes, and immutable file modification.
    Secure = 1,
    /// Highly secure mode. Same as Secure, plus disables partitioning/formatting and setting system time back.
    HighlySecure = 2,
    /// Network secure mode. Same as HighlySecure, plus disables modifying firewall rules.
    NetworkSecure = 3,
}

impl Securelevel {
    pub fn from_i32(val: i32) -> Self {
        match val {
            -1 => Securelevel::PermanentlyInsecure,
            1 => Securelevel::Secure,
            2 => Securelevel::HighlySecure,
            3 => Securelevel::NetworkSecure,
            _ => Securelevel::Insecure,
        }
    }
}

/// Global system securelevel
static SYSTEM_SECURELEVEL: AtomicI32 = AtomicI32::new(0);

/// Get the current system securelevel
pub fn get_securelevel() -> Securelevel {
    Securelevel::from_i32(SYSTEM_SECURELEVEL.load(Ordering::SeqCst))
}

/// Set the system securelevel. Transitions are only permitted if they raise the securelevel
/// (or if current level is Insecure), unless the current level is PermanentlyInsecure.
pub fn set_securelevel(level: Securelevel) -> Result<(), &'static str> {
    let current_raw = SYSTEM_SECURELEVEL.load(Ordering::SeqCst);
    let current = Securelevel::from_i32(current_raw);

    if current == Securelevel::PermanentlyInsecure {
        return Err("Securelevel cannot be changed in PermanentlyInsecure mode");
    }

    if level == Securelevel::PermanentlyInsecure {
        return Err("Cannot transition system into PermanentlyInsecure mode at runtime");
    }

    if current_raw > 0 && (level as i32) <= current_raw {
        return Err("Securelevel can only be raised, not lowered");
    }

    SYSTEM_SECURELEVEL.store(level as i32, Ordering::SeqCst);
    Ok(())
}

/// Helper to temporarily override for unit tests (resetting level)
#[cfg(test)]
pub fn reset_securelevel_for_test() {
    SYSTEM_SECURELEVEL.store(0, Ordering::SeqCst);
}

/// Checks if raw disk block writes are permitted under the current securelevel
pub fn check_raw_disk_write_allowed() -> bool {
    get_securelevel() < Securelevel::Secure
}

/// Checks if writing to kernel memory (/dev/mem or /dev/kmem) is permitted under the current securelevel
pub fn check_kernel_memory_write_allowed() -> bool {
    get_securelevel() < Securelevel::Secure
}

/// Checks if changing system immutable/append-only file flags is permitted
pub fn check_immutable_flag_change_allowed() -> bool {
    get_securelevel() < Securelevel::Secure
}

/// Checks if partition table modification or formatting is permitted
pub fn check_disk_partition_allowed() -> bool {
    get_securelevel() < Securelevel::HighlySecure
}

/// Checks if setting system clock backward or adjusting by more than 1 second is permitted
pub fn check_time_adjustment_allowed(delta_seconds: i64) -> bool {
    if get_securelevel() >= Securelevel::HighlySecure {
        delta_seconds >= 0 && delta_seconds <= 1
    } else {
        true
    }
}

/// Checks if modifying firewall rules or clearing tables is permitted
pub fn check_firewall_modification_allowed() -> bool {
    get_securelevel() < Securelevel::NetworkSecure
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_securelevel_transitions() {
        reset_securelevel_for_test();
        assert_eq!(get_securelevel(), Securelevel::Insecure);

        // Transition from Insecure (0) to Secure (1) is allowed
        assert!(set_securelevel(Securelevel::Secure).is_ok());
        assert_eq!(get_securelevel(), Securelevel::Secure);

        // Lowering securelevel from Secure (1) to Insecure (0) is blocked
        assert!(set_securelevel(Securelevel::Insecure).is_err());
        assert_eq!(get_securelevel(), Securelevel::Secure);

        // Raising securelevel from Secure (1) to HighlySecure (2) is allowed
        assert!(set_securelevel(Securelevel::HighlySecure).is_ok());
        assert_eq!(get_securelevel(), Securelevel::HighlySecure);

        // Transition to PermanentlyInsecure is blocked
        assert!(set_securelevel(Securelevel::PermanentlyInsecure).is_err());
    }

    #[test]
    fn test_securelevel_policies() {
        reset_securelevel_for_test();
        assert!(check_raw_disk_write_allowed());
        assert!(check_kernel_memory_write_allowed());
        assert!(check_immutable_flag_change_allowed());
        assert!(check_disk_partition_allowed());
        assert!(check_time_adjustment_allowed(-10));
        assert!(check_firewall_modification_allowed());

        // Raise to Secure
        set_securelevel(Securelevel::Secure).unwrap();
        assert!(!check_raw_disk_write_allowed());
        assert!(!check_kernel_memory_write_allowed());
        assert!(!check_immutable_flag_change_allowed());
        assert!(check_disk_partition_allowed());
        assert!(check_firewall_modification_allowed());

        // Raise to HighlySecure
        set_securelevel(Securelevel::HighlySecure).unwrap();
        assert!(!check_disk_partition_allowed());
        assert!(!check_time_adjustment_allowed(-5));
        assert!(check_time_adjustment_allowed(1));
        assert!(check_firewall_modification_allowed());

        // Raise to NetworkSecure
        set_securelevel(Securelevel::NetworkSecure).unwrap();
        assert!(!check_firewall_modification_allowed());
    }
}
