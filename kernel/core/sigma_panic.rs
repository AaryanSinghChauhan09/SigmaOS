// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/core/sigma_panic.rs — Kernel Panic Handler + Self-Healing
// Language: Rust #![no_std]
// Pattern: OOP via PanicHandler struct + recovery strategies

#![no_std]
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

// ── Panic Counter ─────────────────────────────────────────────────────────────
static PANIC_COUNT:    AtomicU32  = AtomicU32::new(0);
static IN_PANIC:       AtomicBool = AtomicBool::new(false);
static LAST_PANIC_EIP: AtomicU32  = AtomicU32::new(0);

// ── CPU Register Snapshot ─────────────────────────────────────────────────────
#[repr(C)]
pub struct CpuState {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64, pub rsp: u64,
    pub r8:  u64, pub r9:  u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64, pub rflags: u64, pub cs: u64, pub ss: u64,
    pub cr0: u64, pub cr2: u64, pub cr3: u64, pub cr4: u64,
}

// ── Recovery Strategy ─────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAction {
    Reboot,          // full system reboot
    RestartProcess,  // restart the faulting process only
    SafeMode,        // reboot into minimal safe-mode profile
    Halt,            // halt (for hard faults with no recovery path)
    Continue,        // attempt to continue (speculative — risky)
}

// ── Panic Record ──────────────────────────────────────────────────────────────
pub struct PanicRecord {
    pub message:   [u8; 256],
    pub msg_len:   usize,
    pub state:     CpuState,
    pub pid:       u32,
    pub count:     u32,
    pub action:    RecoveryAction,
}

impl PanicRecord {
    pub fn message_str(&self) -> &[u8] { &self.message[..self.msg_len] }
}

// ── Panic Handler ─────────────────────────────────────────────────────────────
pub struct PanicHandler {
    records:    [Option<PanicRecord>; 8],
    head:       usize,
    max_before_reboot: u32,
}

impl PanicHandler {
    pub const fn new() -> Self {
        Self { records: [const { None }; 8], head: 0, max_before_reboot: 3 }
    }

    pub fn handle(&mut self, msg: &[u8], state: &CpuState, pid: u32) -> RecoveryAction {
        // Detect double-panic (nested panic → halt)
        if IN_PANIC.swap(true, Ordering::SeqCst) {
            return RecoveryAction::Halt;
        }

        let count = PANIC_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
        LAST_PANIC_EIP.store(state.rip as u32, Ordering::Relaxed);

        let action = self.choose_action(count, pid);

        // Store record
        let mut rec = PanicRecord {
            message: [0u8; 256],
            msg_len: msg.len().min(256),
            state: *state,
            pid, count, action,
        };
        rec.message[..rec.msg_len].copy_from_slice(&msg[..rec.msg_len]);

        let slot = self.head % 8;
        self.records[slot] = Some(rec);
        self.head += 1;

        IN_PANIC.store(false, Ordering::SeqCst);
        action
    }

    fn choose_action(&self, count: u32, pid: u32) -> RecoveryAction {
        if count >= self.max_before_reboot { return RecoveryAction::Reboot; }
        if pid == 1 { return RecoveryAction::SafeMode; } // init crashed
        if pid > 1  { return RecoveryAction::RestartProcess; }
        RecoveryAction::Reboot
    }

    /// Print panic to framebuffer (blue screen equivalent)
    pub fn display_panic(msg: &[u8], state: &CpuState) {
        // In production: write to VESA framebuffer + serial UART
        // Here: just a marker — implementation in sigma_vesa.zig
        let _ = (msg, state);
    }

    /// Execute the chosen recovery action
    pub fn execute(action: RecoveryAction) -> ! {
        match action {
            RecoveryAction::Halt => {
                unsafe { core::arch::asm!("cli; hlt", options(noreturn)) }
            }
            RecoveryAction::Reboot => {
                // Triple-fault or ACPI reset
                unsafe {
                    // Write 0xFE to keyboard controller (triggers reset on x86)
                    core::arch::asm!(
                        "outb %al, $0x64",
                        in("al") 0xFEu8,
                        options(att_syntax)
                    );
                    core::arch::asm!("hlt", options(noreturn));
                }
            }
            RecoveryAction::SafeMode | RecoveryAction::RestartProcess => {
                // These require the process manager — fall back to halt
                unsafe { core::arch::asm!("cli; hlt", options(noreturn)) }
            }
            RecoveryAction::Continue => {
                // Cannot loop in a no-return fn — halt conservatively
                unsafe { core::arch::asm!("cli; hlt", options(noreturn)) }
            }
        }
    }

    pub fn panic_count() -> u32 { PANIC_COUNT.load(Ordering::Relaxed) }
}

// ── Rust #[panic_handler] ─────────────────────────────────────────────────────
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let msg = b"kernel panic";
    // We can't access the PanicHandler instance here without global state
    // In a full impl: use a global static PanicHandler
    PanicHandler::display_panic(msg, &unsafe { core::mem::zeroed() });
    PanicHandler::execute(RecoveryAction::Reboot)
}
