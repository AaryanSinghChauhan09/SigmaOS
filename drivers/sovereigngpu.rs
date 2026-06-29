// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SovereignGPU Driver (Rust, no_std) — Replaces: drivers/SovereignGPU.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SovereignGPUDriver { active: bool }
impl SovereignGPUDriver {
    pub const fn new() -> Self { SovereignGPUDriver { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_DRV: SovereignGPUDriver = SovereignGPUDriver::new();
#[no_mangle]
pub unsafe extern "C" fn sovereigngpu_drv_init() -> SigmaStatus { G_DRV.init() }
#[no_mangle]
pub unsafe extern "C" fn sovereigngpu_drv_active() -> u8 { G_DRV.is_active() as u8 }