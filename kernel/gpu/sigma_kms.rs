// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS GPU Kernel Mode Setting (KMS)
//! Manages display controllers and outputs.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;

pub const MAX_DISPLAYS: usize = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub bpp: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KmsConnector {
    pub id: u32,
    pub connected: bool,
    pub current_mode: DisplayMode,
    pub active: bool,
}

static mut CONNECTORS: [KmsConnector; MAX_DISPLAYS] = [KmsConnector {
    id: 0, connected: false, 
    current_mode: DisplayMode { width: 0, height: 0, refresh_rate: 0, bpp: 0 },
    active: false
}; MAX_DISPLAYS];

#[no_mangle]
pub unsafe extern "C" fn sigma_kms_init() {
    for i in 0..MAX_DISPLAYS {
        CONNECTORS[i].active = false;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_kms_register_connector(id: u32, connected: bool) -> i32 {
    for i in 0..MAX_DISPLAYS {
        if !CONNECTORS[i].active {
            CONNECTORS[i].id = id;
            CONNECTORS[i].connected = connected;
            CONNECTORS[i].active = true;
            return i as i32;
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn sigma_kms_set_mode(
    connector_idx: i32, 
    width: u32, height: u32, refresh: u32, bpp: u8
) -> i32 {
    if connector_idx < 0 || connector_idx as usize >= MAX_DISPLAYS { return -1; }
    let conn = &mut CONNECTORS[connector_idx as usize];
    if !conn.active || !conn.connected { return -1; }
    
    // In a real driver, this sends hardware commands to the display controller to change clocks/timing
    
    conn.current_mode.width = width;
    conn.current_mode.height = height;
    conn.current_mode.refresh_rate = refresh;
    conn.current_mode.bpp = bpp;
    0
}
