// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SovereignControl (Rust, no_std)  — Replaces: usr/SovereignControl.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SovereignControl { active: bool }
impl SovereignControl {
    pub const fn new() -> Self { SovereignControl { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_INSTANCE: SovereignControl = SovereignControl::new();
#[no_mangle]
pub unsafe extern "C" fn sovereign_control_init() -> SigmaStatus { G_INSTANCE.init() }
#[no_mangle]
pub unsafe extern "C" fn sovereign_control_active() -> u8 { G_INSTANCE.is_active() as u8 }