// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SovereignAppStore (Rust, no_std)  — Replaces: usr/SovereignAppStore.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SovereignAppStore { active: bool }
impl SovereignAppStore {
    pub const fn new() -> Self { SovereignAppStore { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_INSTANCE: SovereignAppStore = SovereignAppStore::new();
#[no_mangle]
pub unsafe extern "C" fn sovereign_appstore_init() -> SigmaStatus { G_INSTANCE.init() }
#[no_mangle]
pub unsafe extern "C" fn sovereign_appstore_active() -> u8 { G_INSTANCE.is_active() as u8 }