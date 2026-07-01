// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: E1000 Driver (Rust, no_std) — Replaces: drivers/network/sigma_e1000.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct E1000Driver { active: bool }
impl E1000Driver {
    pub const fn new() -> Self { E1000Driver { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_DRV: E1000Driver = E1000Driver::new();
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_drv_init() -> SigmaStatus { G_DRV.init() }
#[no_mangle]
pub unsafe extern "C" fn sigma_e1000_drv_active() -> u8 { G_DRV.is_active() as u8 }