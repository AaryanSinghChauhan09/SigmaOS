// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SigmaLogic (Rust, no_std)
#[allow(dead_code)]
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

pub struct SigmaLogic { active: bool }
impl SigmaLogic {
    pub const fn new() -> Self { SigmaLogic { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_INSTANCE: SigmaLogic = SigmaLogic::new();
#[no_mangle]
pub unsafe extern "C" fn sigma_logic_init() -> SigmaStatus { G_INSTANCE.init() }
#[no_mangle]
pub unsafe extern "C" fn sigma_logic_active() -> u8 { G_INSTANCE.is_active() as u8 }