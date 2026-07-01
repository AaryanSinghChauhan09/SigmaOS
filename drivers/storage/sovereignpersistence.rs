// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SovereignPersistence Driver (Rust, no_std) — Replaces: drivers/storage/SovereignPersistence.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SovereignPersistenceDriver { active: bool }
impl SovereignPersistenceDriver {
    pub const fn new() -> Self { SovereignPersistenceDriver { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_DRV: SovereignPersistenceDriver = SovereignPersistenceDriver::new();
#[no_mangle]
pub unsafe extern "C" fn sovereignpersistence_drv_init() -> SigmaStatus { G_DRV.init() }
#[no_mangle]
pub unsafe extern "C" fn sovereignpersistence_drv_active() -> u8 { G_DRV.is_active() as u8 }