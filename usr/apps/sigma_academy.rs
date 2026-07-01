// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SigmaAcademy (Rust, no_std)
#[allow(dead_code)]
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

pub struct SigmaAcademy { active: bool }
impl SigmaAcademy {
    pub const fn new() -> Self { SigmaAcademy { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_INSTANCE: SigmaAcademy = SigmaAcademy::new();
#[no_mangle]
pub unsafe extern "C" fn sigma_academy_init() -> SigmaStatus { G_INSTANCE.init() }
#[no_mangle]
pub unsafe extern "C" fn sigma_academy_active() -> u8 { G_INSTANCE.is_active() as u8 }