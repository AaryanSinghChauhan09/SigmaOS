// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Zenith Compositor
//! Minimal Wayland-compatible event loop.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaI32 = i32;

pub const MAX_WINDOWS: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ZenithWindow {
    pub id: SigmaI32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub z_index: u32,
    pub active: bool,
    pub mapped: bool,
}

static mut WINDOWS: [ZenithWindow; MAX_WINDOWS] = [ZenithWindow {
    id: 0, x: 0, y: 0, width: 0, height: 0, z_index: 0, active: false, mapped: false,
}; MAX_WINDOWS];

static mut NEXT_WINDOW_ID: SigmaI32 = 1;

#[no_mangle]
pub unsafe extern "C" fn zenith_init() {
    NEXT_WINDOW_ID = 1;
}

#[no_mangle]
pub unsafe extern "C" fn zenith_create_window(x: i32, y: i32, width: u32, height: u32) -> SigmaI32 {
    for i in 0..MAX_WINDOWS {
        if !WINDOWS[i].active {
            WINDOWS[i].id = NEXT_WINDOW_ID;
            NEXT_WINDOW_ID += 1;
            WINDOWS[i].x = x;
            WINDOWS[i].y = y;
            WINDOWS[i].width = width;
            WINDOWS[i].height = height;
            
            // Put on top
            let mut max_z = 0;
            for j in 0..MAX_WINDOWS {
                if WINDOWS[j].active && WINDOWS[j].z_index > max_z {
                    max_z = WINDOWS[j].z_index;
                }
            }
            WINDOWS[i].z_index = max_z + 1;
            
            WINDOWS[i].mapped = false;
            WINDOWS[i].active = true;
            return WINDOWS[i].id;
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn zenith_map_window(id: SigmaI32) -> SigmaI32 {
    for i in 0..MAX_WINDOWS {
        if WINDOWS[i].active && WINDOWS[i].id == id {
            WINDOWS[i].mapped = true;
            return 0;
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn zenith_destroy_window(id: SigmaI32) -> SigmaI32 {
    for i in 0..MAX_WINDOWS {
        if WINDOWS[i].active && WINDOWS[i].id == id {
            WINDOWS[i].active = false;
            return 0;
        }
    }
    -1
}
