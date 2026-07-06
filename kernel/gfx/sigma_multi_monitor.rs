// SPDX-License-Identifier: MIT
// SigmaOS Multi-Monitor KMS Management — sigma_multi_monitor.rs
// KMS connector enumeration, CRTC allocations, video mode negotiation,
// and plane configurations across multiple monitor outputs.

#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

pub const MAX_MONITOR_OUTPUTS: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MonitorState {
    Disconnected,
    Connected,
    Active,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MonitorMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate_hz: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MonitorOutput {
    pub output_id: u32,
    pub connector_type: u8, // 1 = HDMI, 2 = DisplayPort, 3 = eDP, etc.
    pub state: MonitorState,
    pub current_mode: MonitorMode,
    pub crtc_id: u32,
}

// ── Global State ─────────────────────────────────────────────────────────────
static MULTI_MONITOR_INITIALIZED: AtomicBool = AtomicBool::new(false);
static mut MONITOR_OUTPUTS: [Option<MonitorOutput>; MAX_MONITOR_OUTPUTS] = [None; MAX_MONITOR_OUTPUTS];

// ── Implementation ───────────────────────────────────────────────────────────
pub fn multi_monitor_init() -> i32 {
    if MULTI_MONITOR_INITIALIZED.swap(true, Ordering::SeqCst) {
        return -1;
    }
    unsafe {
        for slot in MONITOR_OUTPUTS.iter_mut() {
            *slot = None;
        }
    }
    0
}

pub fn register_output(output_id: u32, connector_type: u8, state: MonitorState, mode: MonitorMode) -> i32 {
    unsafe {
        for slot in MONITOR_OUTPUTS.iter_mut() {
            if slot.is_none() {
                *slot = Some(MonitorOutput {
                    output_id,
                    connector_type,
                    state,
                    current_mode: mode,
                    crtc_id: output_id + 10, // arbitrary mapping
                });
                return 0;
            }
        }
    }
    -1 // table full
}

// ── C-ABI Exports ────────────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_kms_multi_monitor_init() -> i32 {
    multi_monitor_init()
}

#[no_mangle]
pub extern "C" fn sigma_kms_register_output(
    id: u32, conn_type: u8, state_val: u8, w: u32, h: u32, rate: u8
) -> i32 {
    let state = match state_val {
        0 => MonitorState::Disconnected,
        1 => MonitorState::Connected,
        _ => MonitorState::Active,
    };
    let mode = MonitorMode {
        width: w,
        height: h,
        refresh_rate_hz: rate,
    };
    register_output(id, conn_type, state, mode)
}
