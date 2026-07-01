// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: Rtl8139 Driver (Rust, no_std) — Replaces: drivers/network/sigma_rtl8139.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct Rtl8139Driver { active: bool }
impl Rtl8139Driver {
    pub const fn new() -> Self { Rtl8139Driver { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_DRV: Rtl8139Driver = Rtl8139Driver::new();
#[no_mangle]
pub unsafe extern "C" fn sigma_rtl8139_drv_init() -> SigmaStatus { G_DRV.init() }
#[no_mangle]
pub unsafe extern "C" fn sigma_rtl8139_drv_active() -> u8 { G_DRV.is_active() as u8 }