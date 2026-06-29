// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SovereignVPN Driver (Rust, no_std) — Replaces: drivers/network/SovereignVPN.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SovereignVPNDriver { active: bool }
impl SovereignVPNDriver {
    pub const fn new() -> Self { SovereignVPNDriver { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_DRV: SovereignVPNDriver = SovereignVPNDriver::new();
#[no_mangle]
pub unsafe extern "C" fn sovereignvpn_drv_init() -> SigmaStatus { G_DRV.init() }
#[no_mangle]
pub unsafe extern "C" fn sovereignvpn_drv_active() -> u8 { G_DRV.is_active() as u8 }