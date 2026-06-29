// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SovereignNetStack Driver (Rust, no_std) — Replaces: drivers/network/SovereignNetStack.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SovereignNetStackDriver { active: bool }
impl SovereignNetStackDriver {
    pub const fn new() -> Self { SovereignNetStackDriver { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_DRV: SovereignNetStackDriver = SovereignNetStackDriver::new();
#[no_mangle]
pub unsafe extern "C" fn sovereignnetstack_drv_init() -> SigmaStatus { G_DRV.init() }
#[no_mangle]
pub unsafe extern "C" fn sovereignnetstack_drv_active() -> u8 { G_DRV.is_active() as u8 }