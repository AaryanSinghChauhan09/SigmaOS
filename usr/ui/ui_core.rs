// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SovereignUICore (Rust, no_std)  — Replaces: usr/SovereignUICore.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SovereignUICore { active: bool }
impl SovereignUICore {
    pub const fn new() -> Self { SovereignUICore { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_INSTANCE: SovereignUICore = SovereignUICore::new();
#[no_mangle]
pub unsafe extern "C" fn sovereign_ui_core_init() -> SigmaStatus { G_INSTANCE.init() }
#[no_mangle]
pub unsafe extern "C" fn sovereign_ui_core_active() -> u8 { G_INSTANCE.is_active() as u8 }