// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SovereignZeroNet Driver (Rust, no_std) — Replaces: drivers/network/SovereignZeroNet.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SovereignZeroNetDriver { active: bool }
impl SovereignZeroNetDriver {
    pub const fn new() -> Self { SovereignZeroNetDriver { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_DRV: SovereignZeroNetDriver = SovereignZeroNetDriver::new();
#[no_mangle]
pub unsafe extern "C" fn sovereignzeronet_drv_init() -> SigmaStatus { G_DRV.init() }
#[no_mangle]
pub unsafe extern "C" fn sovereignzeronet_drv_active() -> u8 { G_DRV.is_active() as u8 }