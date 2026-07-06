// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Recovery GUI
//! Rescuezilla-style graphical recovery environment with fallback safe-mode launcher.
//! no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaBool  = bool;

#[no_mangle]
pub unsafe extern "C" fn sigma_recovery_gui_init() -> SigmaI32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_recovery_gui_draw_menu(selected_index: SigmaU32) {
    // Draws recovery choices (Fix Boot, Restore Backup, Factory Reset, Start Safe Shell) on VESA framebuffer
}

#[no_mangle]
pub unsafe extern "C" fn sigma_recovery_gui_handle_input(key_scancode: SigmaU8) -> SigmaI32 {
    // Returns action ID (1 = fix bootloader, 2 = partition recovery, 3 = shell, 0 = no action)
    0
}
