// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SovereignBrowser (Rust, no_std)  — Replaces: usr/SovereignBrowser.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SovereignBrowser { active: bool }
impl SovereignBrowser {
    pub const fn new() -> Self { SovereignBrowser { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_INSTANCE: SovereignBrowser = SovereignBrowser::new();
#[no_mangle]
pub unsafe extern "C" fn sovereign_browser_init() -> SigmaStatus { G_INSTANCE.init() }
#[no_mangle]
pub unsafe extern "C" fn sovereign_browser_active() -> u8 { G_INSTANCE.is_active() as u8 }