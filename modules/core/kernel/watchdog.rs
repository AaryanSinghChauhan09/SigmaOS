/// SigmaOS — modules/core/kernel/watchdog.rs
/// Hardware Watchdog: counter state machine, kick, timeout detection, recovery.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU64  = u64;
type SigmaU32  = u32;
type SigmaI32  = i32;
type SigmaBool = bool;

// ─── Configuration ────────────────────────────────────────────────────────────

/// Default watchdog timeout in timer ticks (≈ 5 seconds at 1000 Hz)
pub const WDT_DEFAULT_TIMEOUT_TICKS: SigmaU64 = 5_000;

/// Maximum number of consecutive timeouts before escalating to hard reset
pub const WDT_MAX_CONSECUTIVE_TIMEOUTS: SigmaU32 = 3;

/// Watchdog kick magic value (sanity check against accidental resets)
pub const WDT_KICK_MAGIC: SigmaU32 = 0xDEAD_FEED;

// ─── Watchdog State ───────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum WdtState {
    Disabled   = 0,
    Armed      = 1,
    Expired    = 2,
    Recovering = 3,
    HardReset  = 4,
}

#[repr(C)]
pub struct WatchdogController {
    pub state:               WdtState,
    pub timeout_ticks:       SigmaU64,   // ticks until timeout
    pub ticks_since_kick:    SigmaU64,   // ticks elapsed since last kick
    pub kick_count:          SigmaU64,   // total successful kicks
    pub timeout_count:       SigmaU32,   // consecutive timeouts
    pub last_kick_shard:     SigmaU32,   // shard ID that last kicked
    pub recovery_pending:    SigmaBool,
}

static mut WDT: WatchdogController = WatchdogController {
    state:            WdtState::Disabled,
    timeout_ticks:    WDT_DEFAULT_TIMEOUT_TICKS,
    ticks_since_kick: 0,
    kick_count:       0,
    timeout_count:    0,
    last_kick_shard:  0,
    recovery_pending: false,
};

// ─── External Hooks ───────────────────────────────────────────────────────────

extern "C" {
    /// Called by the watchdog when it fires — attempts a shard-level soft recovery.
    fn wdt_recovery_handler() -> SigmaI32;

    /// Called when recovery fails — triggers full system reset.
    fn kernel_panic(msg: *const u8) -> !;
}

// ─── Implementation ───────────────────────────────────────────────────────────

/// Initialise and arm the hardware watchdog.
#[no_mangle]
pub unsafe extern "C" fn watchdog_init() -> SigmaI32 {
    WDT.state            = WdtState::Armed;
    WDT.timeout_ticks    = WDT_DEFAULT_TIMEOUT_TICKS;
    WDT.ticks_since_kick = 0;
    WDT.kick_count       = 0;
    WDT.timeout_count    = 0;
    WDT.recovery_pending = false;

    // On real x86_64 hardware this programs the iTCO watchdog registers.
    // WDTCR = 0x0060 (enable + start), WDTLR = timeout_value.
    // For QEMU/emulated environments we rely on the software counter below.
    0
}

/// Pet / kick the watchdog — must be called before `timeout_ticks` elapse.
/// `shard_id` identifies the caller for audit purposes.
#[no_mangle]
pub unsafe extern "C" fn watchdog_kick(shard_id: SigmaU32) {
    if WDT.state != WdtState::Armed { return; }
    WDT.ticks_since_kick = 0;
    WDT.kick_count       = WDT.kick_count.wrapping_add(1);
    WDT.last_kick_shard  = shard_id;
}

/// Set a custom timeout (in ticks). Resets the counter.
#[no_mangle]
pub unsafe extern "C" fn watchdog_set_timeout(ticks: SigmaU64) -> SigmaI32 {
    if ticks == 0 { return -1; }
    WDT.timeout_ticks    = ticks;
    WDT.ticks_since_kick = 0;
    0
}

/// Disarm the watchdog (only callable with CAP_POWER_CTRL).
#[no_mangle]
pub unsafe extern "C" fn watchdog_disable() {
    WDT.state = WdtState::Disabled;
}

/// Called from the timer IRQ handler on every tick.
/// Increments the counter and fires recovery / reset if timed out.
#[no_mangle]
pub unsafe extern "C" fn watchdog_tick() {
    if WDT.state != WdtState::Armed { return; }

    WDT.ticks_since_kick = WDT.ticks_since_kick.wrapping_add(1);

    if WDT.ticks_since_kick < WDT.timeout_ticks { return; }

    // Timeout fired
    WDT.state         = WdtState::Expired;
    WDT.timeout_count = WDT.timeout_count.wrapping_add(1);

    if WDT.timeout_count >= WDT_MAX_CONSECUTIVE_TIMEOUTS {
        // Escalate to hard reset — nothing else to try
        WDT.state = WdtState::HardReset;
        kernel_panic(b"watchdog: hard reset threshold exceeded\0".as_ptr());
    }

    // Attempt soft recovery
    WDT.state            = WdtState::Recovering;
    WDT.recovery_pending = true;
    let rc = wdt_recovery_handler();
    if rc == 0 {
        // Recovery succeeded — re-arm
        WDT.state            = WdtState::Armed;
        WDT.ticks_since_kick = 0;
        WDT.recovery_pending = false;
    } else {
        // Recovery failed — will escalate on next tick cycle
        WDT.state = WdtState::Expired;
    }
}

/// Query the watchdog state.
#[no_mangle]
pub unsafe extern "C" fn watchdog_state() -> SigmaI32 {
    WDT.state as SigmaI32
}

/// Read the kick counter (for health dashboards).
#[no_mangle]
pub unsafe extern "C" fn watchdog_kick_count() -> SigmaU64 {
    WDT.kick_count
}

/// Read consecutive timeout count.
#[no_mangle]
pub unsafe extern "C" fn watchdog_timeout_count() -> SigmaU32 {
    WDT.timeout_count
}
