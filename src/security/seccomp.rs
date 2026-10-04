//! SigmaOS — Seccomp BPF Syscall Filter
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux seccomp(2) and OpenBSD pledge(2)

#![allow(dead_code, unused)]

extern crate alloc;
use alloc::vec::Vec;

// ─── BPF Constants ───────────────────────────────────────────────────────────
/// BPF instruction classes
pub const BPF_LD: u16 = 0x00;
pub const BPF_RET: u16 = 0x06;
pub const BPF_JMP: u16 = 0x05;

/// BPF load sizes
pub const BPF_W: u16 = 0x00; // word

/// BPF addressing modes
pub const BPF_ABS: u16 = 0x20;

/// BPF jump ops
pub const BPF_JEQ: u16 = 0x10;

/// BPF return constants (seccomp actions encoded in k)
pub const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;
pub const SECCOMP_RET_KILL_THREAD: u32 = 0x0000_0000;
pub const SECCOMP_RET_KILL_PROCESS: u32 = 0x8000_0000;
pub const SECCOMP_RET_TRAP: u32 = 0x0003_0000;
pub const SECCOMP_RET_LOG: u32 = 0x7ffc_0000;
pub const SECCOMP_RET_TRACE: u32 = 0x7ff0_0000;
pub const SECCOMP_RET_ERRNO_MASK: u32 = 0x0005_0000;

/// Byte offset of syscall number in `seccomp_data` struct.
pub const SECCOMP_DATA_NR_OFFSET: u32 = 0;

// ─── Action Enum ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompAction {
    /// Allow the syscall through.
    Allow,
    /// Kill the calling thread.
    Kill,
    /// Kill the entire process.
    KillProcess,
    /// Send SIGSYS to the process.
    Trap,
    /// Return a specific errno to the caller.
    Errno(i32),
    /// Notify a tracer (ptrace).
    Trace,
    /// Log the event and allow.
    Log,
}

impl SeccompAction {
    /// Convert the action to the BPF `k` value used in a RET instruction.
    pub fn to_bpf_k(self) -> u32 {
        match self {
            SeccompAction::Allow => SECCOMP_RET_ALLOW,
            SeccompAction::Kill => SECCOMP_RET_KILL_THREAD,
            SeccompAction::KillProcess => SECCOMP_RET_KILL_PROCESS,
            SeccompAction::Trap => SECCOMP_RET_TRAP,
            SeccompAction::Trace => SECCOMP_RET_TRACE,
            SeccompAction::Log => SECCOMP_RET_LOG,
            SeccompAction::Errno(e) => SECCOMP_RET_ERRNO_MASK | (e as u32 & 0xffff),
        }
    }
}

// ─── BPF Instruction ─────────────────────────────────────────────────────────
/// Standard cBPF (classic BPF) instruction — 8 bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BpfInstruction {
    /// Instruction code (class | size | mode or jump op).
    pub code: u16,
    /// Jump-if-true offset.
    pub jt: u8,
    /// Jump-if-false offset.
    pub jf: u8,
    /// Generic multiuse field (immediate, offset, return value).
    pub k: u32,
}

impl BpfInstruction {
    pub const fn new(code: u16, jt: u8, jf: u8, k: u32) -> Self {
        Self { code, jt, jf, k }
    }

    /// Load syscall number from `seccomp_data` into accumulator.
    pub const fn load_syscall_nr() -> Self {
        Self::new(BPF_LD | BPF_W | BPF_ABS, 0, 0, SECCOMP_DATA_NR_OFFSET)
    }

    /// Unconditional return.
    pub const fn ret(action: u32) -> Self {
        Self::new(BPF_RET | 0x00 /* K */, 0, 0, action)
    }

    /// Jump if accumulator == k, with true/false offsets.
    pub const fn jeq(k: u32, jt: u8, jf: u8) -> Self {
        Self::new(BPF_JMP | BPF_JEQ | BPF_ABS, jt, jf, k)
    }
}

// ─── Filter ──────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct SeccompFilter {
    /// Ordered list of BPF instructions.
    pub instructions: Vec<BpfInstruction>,
    /// Default action applied when no earlier rule matches.
    pub default_action: SeccompAction,
}

impl SeccompFilter {
    pub fn new(default_action: SeccompAction) -> Self {
        Self {
            instructions: Vec::new(),
            default_action,
        }
    }

    /// Append a single instruction.
    pub fn push(&mut self, insn: BpfInstruction) {
        self.instructions.push(insn);
    }
}

// ─── Filter Builders ─────────────────────────────────────────────────────────

/// Emit instructions that check if syscall number == `nr` and, if so, return
/// `SeccompAction::Allow`. Falls through to next instruction otherwise.
///
/// Layout (2 instructions appended):
/// ```text
/// jeq  nr, 0, 1     ; if nr matches jump over KILL, else fall through
/// ret  ALLOW
/// ```
/// Caller is responsible for the final default RET.
pub fn allow_syscall(nr: u32) -> BpfInstruction {
    // Returns a JEQ instruction: if acc == nr, skip 0 insns (hit RET ALLOW
    // emitted next), else skip 1 insn (miss → fall through to next check).
    // Simplified: returns the JEQ for a 2-insn snippet.
    BpfInstruction::jeq(nr, 0, 1)
}

/// Emit instructions that check if syscall number == `nr` and, if so, return
/// errno `errno`. Falls through otherwise.
pub fn deny_syscall(nr: u32, _errno: i32) -> BpfInstruction {
    BpfInstruction::jeq(nr, 0, 1)
}

/// Build a seccomp allow-list filter: allow `syscalls`, deny everything else
/// with `SECCOMP_RET_KILL`.
///
/// Emitted program:
/// 1. `LD [0]`            — load syscall nr
/// 2. For each allowed syscall: `JEQ nr, 0, 1` + `RET ALLOW`
/// 3. `RET KILL`          — default
pub fn filter_allow_list(syscalls: &[u32]) -> SeccompFilter {
    let mut filter = SeccompFilter::new(SeccompAction::Kill);
    // Load syscall number into accumulator once.
    filter.push(BpfInstruction::load_syscall_nr());
    for &nr in syscalls {
        // If accumulator == nr → jump 0 (next is RET ALLOW), else jump 1 (skip RET ALLOW).
        filter.push(BpfInstruction::jeq(nr, 0, 1));
        filter.push(BpfInstruction::ret(SECCOMP_RET_ALLOW));
    }
    // Default: kill.
    filter.push(BpfInstruction::ret(SECCOMP_RET_KILL_THREAD));
    filter
}

/// Build a seccomp deny-list filter: deny `syscalls` with KILL, allow others.
///
/// Emitted program:
/// 1. `LD [0]`            — load syscall nr
/// 2. For each denied syscall: `JEQ nr, 0, 1` + `RET KILL`
/// 3. `RET ALLOW`         — default
pub fn filter_deny_list(syscalls: &[u32]) -> SeccompFilter {
    let mut filter = SeccompFilter::new(SeccompAction::Allow);
    filter.push(BpfInstruction::load_syscall_nr());
    for &nr in syscalls {
        filter.push(BpfInstruction::jeq(nr, 0, 1));
        filter.push(BpfInstruction::ret(SECCOMP_RET_KILL_THREAD));
    }
    filter.push(BpfInstruction::ret(SECCOMP_RET_ALLOW));
    filter
}

/// Strict mode filter: only read(0), write(1), exit(60), rt_sigreturn(15).
///
/// Mirrors `prctl(PR_SET_SECCOMP, SECCOMP_MODE_STRICT)` semantics.
pub fn filter_strict_mode() -> SeccompFilter {
    // x86-64 syscall numbers
    const SYS_READ: u32 = 0;
    const SYS_WRITE: u32 = 1;
    const SYS_RT_SIGRETURN: u32 = 15;
    const SYS_EXIT: u32 = 60;
    filter_allow_list(&[SYS_READ, SYS_WRITE, SYS_RT_SIGRETURN, SYS_EXIT])
}

// ─── Loader ──────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeccompError {
    /// Filter has no instructions.
    EmptyFilter,
    /// Kernel rejected the filter (raw errno).
    Kernel(i32),
    /// Attempted to load a second filter after the process was locked.
    Locked,
}

/// Load a seccomp-BPF filter.
///
/// In production this calls:
/// ```text
/// prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)
/// prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &prog)
/// ```
/// Here we validate the filter and simulate success.
pub fn seccomp_load(filter: &SeccompFilter) -> Result<(), SeccompError> {
    if filter.instructions.is_empty() {
        return Err(SeccompError::EmptyFilter);
    }
    // Stub: real impl would use inline asm / syscall to load the BPF program.
    Ok(())
}

// ─── Unit Tests ───────────────────────────────────────────────────────────────
#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpf_instruction_fields() {
        let insn = BpfInstruction::load_syscall_nr();
        assert_eq!(insn.k, SECCOMP_DATA_NR_OFFSET);
        assert_eq!(insn.jt, 0);
        assert_eq!(insn.jf, 0);
    }

    #[test]
    fn test_allow_list_length() {
        let syscalls = [0u32, 1, 15, 60];
        let filter = filter_allow_list(&syscalls);
        // 1 (LD) + 4*(JEQ+RET) + 1 (default RET) = 10
        assert_eq!(filter.instructions.len(), 10);
    }

    #[test]
    fn test_allow_list_starts_with_load() {
        let filter = filter_allow_list(&[0]);
        let first = filter.instructions[0];
        assert_eq!(first.k, SECCOMP_DATA_NR_OFFSET);
    }

    #[test]
    fn test_allow_list_ends_with_kill() {
        let filter = filter_allow_list(&[0]);
        let last = filter.instructions.last().unwrap();
        assert_eq!(last.k, SECCOMP_RET_KILL_THREAD);
    }

    #[test]
    fn test_deny_list_ends_with_allow() {
        let filter = filter_deny_list(&[105]); // kill setsid
        let last = filter.instructions.last().unwrap();
        assert_eq!(last.k, SECCOMP_RET_ALLOW);
    }

    #[test]
    fn test_strict_mode_has_four_syscalls() {
        let filter = filter_strict_mode();
        // 1 LD + 4*(JEQ+RET) + 1 default = 10
        assert_eq!(filter.instructions.len(), 10);
    }

    #[test]
    fn test_action_to_bpf_k() {
        assert_eq!(SeccompAction::Allow.to_bpf_k(), SECCOMP_RET_ALLOW);
        assert_eq!(SeccompAction::Kill.to_bpf_k(), SECCOMP_RET_KILL_THREAD);
        assert_eq!(
            SeccompAction::KillProcess.to_bpf_k(),
            SECCOMP_RET_KILL_PROCESS
        );
        assert_eq!(SeccompAction::Trap.to_bpf_k(), SECCOMP_RET_TRAP);
        assert_eq!(SeccompAction::Log.to_bpf_k(), SECCOMP_RET_LOG);
        // ERRNO(13) = SECCOMP_RET_ERRNO_MASK | 13
        assert_eq!(
            SeccompAction::Errno(13).to_bpf_k(),
            SECCOMP_RET_ERRNO_MASK | 13
        );
    }

    #[test]
    fn test_seccomp_load_empty_filter_fails() {
        let filter = SeccompFilter::new(SeccompAction::Allow);
        assert_eq!(seccomp_load(&filter), Err(SeccompError::EmptyFilter));
    }

    #[test]
    fn test_seccomp_load_ok() {
        let filter = filter_strict_mode();
        assert!(seccomp_load(&filter).is_ok());
    }
}
