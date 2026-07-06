// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Runlevel Manager
//! Orchestrates transitioning between boot targets/runlevels.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaI32 = i32;

pub const RUNLEVEL_HALT:     SigmaU8 = 0;
pub const RUNLEVEL_SINGLE:   SigmaU8 = 1; // Single-user, no network
pub const RUNLEVEL_MULTI:    SigmaU8 = 3; // Multi-user, network (default)
pub const RUNLEVEL_GUI:      SigmaU8 = 5; // Multi-user, network, display manager
pub const RUNLEVEL_REBOOT:   SigmaU8 = 6;

static mut CURRENT_RUNLEVEL: SigmaU8 = 0;
static mut TARGET_RUNLEVEL:  SigmaU8 = RUNLEVEL_MULTI;

#[no_mangle]
pub unsafe extern "C" fn sigma_runlevel_init() {
    CURRENT_RUNLEVEL = 0;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_runlevel_get_current() -> SigmaU8 {
    CURRENT_RUNLEVEL
}

/// Request a runlevel change. Returns 0 if transition accepted.
#[no_mangle]
pub unsafe extern "C" fn sigma_runlevel_set_target(rl: SigmaU8) -> SigmaI32 {
    if rl > 6 { return -1; }
    TARGET_RUNLEVEL = rl;
    0
}

/// Tick function to be called periodically by PID 1.
/// Detects if we need to transition to a new runlevel.
#[no_mangle]
pub unsafe extern "C" fn sigma_runlevel_tick() -> SigmaI32 {
    if CURRENT_RUNLEVEL != TARGET_RUNLEVEL {
        let old = CURRENT_RUNLEVEL;
        let new = TARGET_RUNLEVEL;
        
        // In a real system, here we would:
        // 1. Iterate over all registered services.
        // 2. If their required runlevel > new, send SIGTERM.
        // 3. If their required runlevel <= new, and they are Down, start them.

        if new == RUNLEVEL_HALT {
            // Initiate ACPI shutdown
        } else if new == RUNLEVEL_REBOOT {
            // Initiate keyboard controller reboot
        }
        
        CURRENT_RUNLEVEL = new;
        return 1; // indicates a transition happened
    }
    0
}
