//! SigmaOS — Landlock v4 Security Module
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux Landlock LSM (Mickaël Salaün)
//! Syscalls: landlock_create_ruleset, landlock_add_rule, landlock_restrict_self

#![allow(dead_code, unused)]

extern crate alloc;
use alloc::vec::Vec;

/// Landlock ABI version supported by this module.
pub const LANDLOCK_ABI_VERSION: u32 = 4;

// ─── FS Access Bitflags ───────────────────────────────────────────────────────
pub mod fs_access {
    pub const EXECUTE:     u64 = 1 << 0;
    pub const WRITE_FILE:  u64 = 1 << 1;
    pub const READ_FILE:   u64 = 1 << 2;
    pub const READ_DIR:    u64 = 1 << 3;
    pub const REMOVE_DIR:  u64 = 1 << 4;
    pub const REMOVE_FILE: u64 = 1 << 5;
    pub const MAKE_CHAR:   u64 = 1 << 6;
    pub const MAKE_DIR:    u64 = 1 << 7;
    pub const MAKE_REG:    u64 = 1 << 8;
    pub const MAKE_SOCK:   u64 = 1 << 9;
    pub const MAKE_FIFO:   u64 = 1 << 10;
    pub const MAKE_BLOCK:  u64 = 1 << 11;
    pub const MAKE_SYM:    u64 = 1 << 12;
    pub const REFER:       u64 = 1 << 13;
    pub const TRUNCATE:    u64 = 1 << 14;

    /// All FS access rights combined (ABI v4)
    pub const ALL: u64 = EXECUTE | WRITE_FILE | READ_FILE | READ_DIR
        | REMOVE_DIR | REMOVE_FILE | MAKE_CHAR | MAKE_DIR | MAKE_REG
        | MAKE_SOCK | MAKE_FIFO | MAKE_BLOCK | MAKE_SYM | REFER | TRUNCATE;
}

// ─── NET Access Bitflags (v4 additions) ───────────────────────────────────────
pub mod net_access {
    pub const BIND_TCP:    u64 = 1 << 0;
    pub const CONNECT_TCP: u64 = 1 << 1;
    pub const BIND_UDP:    u64 = 1 << 2;
    pub const CONNECT_UDP: u64 = 1 << 3;

    pub const ALL: u64 = BIND_TCP | CONNECT_TCP | BIND_UDP | CONNECT_UDP;
}

// ─── Ruleset Attribute ────────────────────────────────────────────────────────
/// Corresponds to `landlock_ruleset_attr`.
#[derive(Debug, Clone, Copy)]
pub struct LandlockRuleset {
    /// Handled FS access rights (bit mask from fs_access).
    pub handled_access_fs: u64,
    /// Handled network access rights (bit mask from net_access). ABI v4+.
    pub handled_access_net: u64,
}

impl LandlockRuleset {
    pub const fn new(fs: u64, net: u64) -> Self {
        Self { handled_access_fs: fs, handled_access_net: net }
    }
}

// ─── Rule Enum ────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum LandlockRule {
    /// Path-beneath rule: restrict access to a path via an open fd.
    PathBeneath {
        /// Allowed access rights on this path (subset of ruleset's FS mask).
        access: u64,
        /// File descriptor referencing the path.
        fd: i32,
    },
    /// Network port rule (ABI v4 addition).
    NetPort {
        /// Allowed network access rights on this port.
        access: u64,
        /// TCP/UDP port number.
        port: u16,
    },
}

// ─── Error Type ───────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LandlockError {
    /// Kernel ABI is older than required.
    UnsupportedAbi,
    /// An argument to a syscall was invalid.
    InvalidArgument,
    /// `landlock_restrict_self` was already called on this thread.
    AlreadyEnforced,
    /// Raw syscall error code.
    Syscall(i32),
}

// ─── Context ─────────────────────────────────────────────────────────────────
/// Holds the ruleset configuration and accumulated rules before enforcement.
pub struct LandlockContext {
    /// Ruleset attribute (what access rights are handled).
    pub ruleset: LandlockRuleset,
    /// Accumulated rules to add.
    pub rules: Vec<LandlockRule>,
    /// Whether `landlock_enforce` has already been called.
    enforced: bool,
}

impl LandlockContext {
    pub fn new(ruleset: LandlockRuleset) -> Self {
        Self { ruleset, rules: Vec::new(), enforced: false }
    }
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// Create a new Landlock context with the given FS and network access masks.
pub fn landlock_create_context(fs_access: u64, net_access: u64) -> LandlockContext {
    let ruleset = LandlockRuleset::new(fs_access, net_access);
    LandlockContext::new(ruleset)
}

/// Add a path-beneath rule to the context.
///
/// # Errors
/// Returns `LandlockError::InvalidArgument` if `access` exceeds the ruleset's
/// `handled_access_fs` mask or if `path_fd` is negative.
pub fn landlock_add_path_rule(
    ctx: &mut LandlockContext,
    path_fd: i32,
    access: u64,
) -> Result<(), LandlockError> {
    if path_fd < 0 {
        return Err(LandlockError::InvalidArgument);
    }
    if access & !ctx.ruleset.handled_access_fs != 0 {
        return Err(LandlockError::InvalidArgument);
    }
    ctx.rules.push(LandlockRule::PathBeneath { access, fd: path_fd });
    Ok(())
}

/// Add a network port rule to the context.
///
/// # Errors
/// Returns `LandlockError::InvalidArgument` if `access` exceeds the ruleset's
/// `handled_access_net` mask.
pub fn landlock_add_net_rule(
    ctx: &mut LandlockContext,
    port: u16,
    access: u64,
) -> Result<(), LandlockError> {
    if access & !ctx.ruleset.handled_access_net != 0 {
        return Err(LandlockError::InvalidArgument);
    }
    ctx.rules.push(LandlockRule::NetPort { access, port });
    Ok(())
}

/// Enforce the Landlock sandbox by calling `restrict_self`.
///
/// In a real kernel this would:
/// 1. `sys_landlock_create_ruleset(&ruleset_attr, size, 0)` → ruleset_fd
/// 2. For each rule: `sys_landlock_add_rule(ruleset_fd, rule_type, attr, 0)`
/// 3. `sys_landlock_restrict_self(ruleset_fd, 0)`
///
/// # Errors
/// Returns `LandlockError::AlreadyEnforced` if called more than once.
pub fn landlock_enforce(ctx: &mut LandlockContext) -> Result<(), LandlockError> {
    if ctx.enforced {
        return Err(LandlockError::AlreadyEnforced);
    }
    // Simulate syscall: create ruleset
    let _ruleset_fd = sys_landlock_create_ruleset(&ctx.ruleset)?;
    // Simulate syscall: add each rule
    for rule in &ctx.rules {
        sys_landlock_add_rule(_ruleset_fd, rule)?;
    }
    // Simulate syscall: restrict self
    sys_landlock_restrict_self(_ruleset_fd)?;
    ctx.enforced = true;
    Ok(())
}

// ─── Syscall Simulation Layer ─────────────────────────────────────────────────
// These are no_std-safe simulations. On real hardware they would use
// `core::arch::asm!` with the appropriate syscall numbers.

/// Simulates `landlock_create_ruleset(attr, size, 0)`.
/// Returns a simulated ruleset fd (always 42 in test/stub mode).
fn sys_landlock_create_ruleset(attr: &LandlockRuleset) -> Result<i32, LandlockError> {
    // Validate that at least one access right is specified.
    if attr.handled_access_fs == 0 && attr.handled_access_net == 0 {
        return Err(LandlockError::InvalidArgument);
    }
    // Stub: real impl would be:
    // unsafe { core::arch::asm!("syscall", ...) }
    Ok(42) // synthetic fd
}

/// Simulates `landlock_add_rule(ruleset_fd, rule_type, rule_attr, 0)`.
fn sys_landlock_add_rule(ruleset_fd: i32, rule: &LandlockRule) -> Result<(), LandlockError> {
    if ruleset_fd < 0 {
        return Err(LandlockError::Syscall(-9)); // EBADF
    }
    match rule {
        LandlockRule::PathBeneath { fd, .. } if *fd < 0 => {
            Err(LandlockError::InvalidArgument)
        }
        _ => Ok(()),
    }
}

/// Simulates `landlock_restrict_self(ruleset_fd, 0)`.
fn sys_landlock_restrict_self(ruleset_fd: i32) -> Result<(), LandlockError> {
    if ruleset_fd < 0 {
        return Err(LandlockError::Syscall(-9)); // EBADF
    }
    Ok(())
}

// ─── Unit Tests ───────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_context() {
        let ctx = landlock_create_context(fs_access::ALL, net_access::ALL);
        assert_eq!(ctx.ruleset.handled_access_fs, fs_access::ALL);
        assert_eq!(ctx.ruleset.handled_access_net, net_access::ALL);
        assert!(ctx.rules.is_empty());
    }

    #[test]
    fn test_add_path_rule_accumulates() {
        let mut ctx = landlock_create_context(fs_access::ALL, 0);
        landlock_add_path_rule(&mut ctx, 3, fs_access::READ_FILE).unwrap();
        landlock_add_path_rule(&mut ctx, 4, fs_access::EXECUTE).unwrap();
        assert_eq!(ctx.rules.len(), 2);
        match ctx.rules[0] {
            LandlockRule::PathBeneath { fd, access } => {
                assert_eq!(fd, 3);
                assert_eq!(access, fs_access::READ_FILE);
            }
            _ => panic!("wrong rule type"),
        }
    }

    #[test]
    fn test_add_path_rule_invalid_fd() {
        let mut ctx = landlock_create_context(fs_access::ALL, 0);
        let err = landlock_add_path_rule(&mut ctx, -1, fs_access::READ_FILE).unwrap_err();
        assert_eq!(err, LandlockError::InvalidArgument);
    }

    #[test]
    fn test_add_path_rule_exceeds_mask() {
        let mut ctx = landlock_create_context(fs_access::READ_FILE, 0);
        // EXECUTE not in mask
        let err = landlock_add_path_rule(&mut ctx, 3, fs_access::EXECUTE).unwrap_err();
        assert_eq!(err, LandlockError::InvalidArgument);
    }

    #[test]
    fn test_add_net_rule_accumulates() {
        let mut ctx = landlock_create_context(0, net_access::ALL);
        landlock_add_net_rule(&mut ctx, 443, net_access::CONNECT_TCP).unwrap();
        landlock_add_net_rule(&mut ctx, 80, net_access::BIND_TCP).unwrap();
        assert_eq!(ctx.rules.len(), 2);
        match ctx.rules[0] {
            LandlockRule::NetPort { port, access } => {
                assert_eq!(port, 443);
                assert_eq!(access, net_access::CONNECT_TCP);
            }
            _ => panic!("wrong rule type"),
        }
    }

    #[test]
    fn test_add_net_rule_invalid_access() {
        let mut ctx = landlock_create_context(0, net_access::BIND_TCP);
        // CONNECT_TCP not in mask
        let err = landlock_add_net_rule(&mut ctx, 443, net_access::CONNECT_TCP).unwrap_err();
        assert_eq!(err, LandlockError::InvalidArgument);
    }

    #[test]
    fn test_enforce_sets_flag() {
        let mut ctx = landlock_create_context(fs_access::READ_FILE, net_access::BIND_TCP);
        landlock_add_path_rule(&mut ctx, 3, fs_access::READ_FILE).unwrap();
        landlock_add_net_rule(&mut ctx, 8080, net_access::BIND_TCP).unwrap();
        landlock_enforce(&mut ctx).unwrap();
        assert!(ctx.enforced);
    }

    #[test]
    fn test_enforce_double_call_fails() {
        let mut ctx = landlock_create_context(fs_access::READ_FILE, 0);
        landlock_enforce(&mut ctx).unwrap();
        let err = landlock_enforce(&mut ctx).unwrap_err();
        assert_eq!(err, LandlockError::AlreadyEnforced);
    }

    #[test]
    fn test_fs_access_bits_are_unique() {
        let bits = [
            fs_access::EXECUTE, fs_access::WRITE_FILE, fs_access::READ_FILE,
            fs_access::READ_DIR, fs_access::REMOVE_DIR, fs_access::REMOVE_FILE,
            fs_access::MAKE_CHAR, fs_access::MAKE_DIR, fs_access::MAKE_REG,
            fs_access::MAKE_SOCK, fs_access::MAKE_FIFO, fs_access::MAKE_BLOCK,
            fs_access::MAKE_SYM, fs_access::REFER, fs_access::TRUNCATE,
        ];
        let mut seen: u64 = 0;
        for &b in &bits {
            assert_eq!(seen & b, 0, "duplicate bit: {b:#x}");
            seen |= b;
        }
    }

    #[test]
    fn test_net_access_bits_are_unique() {
        let bits = [
            net_access::BIND_TCP, net_access::CONNECT_TCP,
            net_access::BIND_UDP, net_access::CONNECT_UDP,
        ];
        let mut seen: u64 = 0;
        for &b in &bits {
            assert_eq!(seen & b, 0, "duplicate bit: {b:#x}");
            seen |= b;
        }
    }
}
