// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SigmaAI (Rust, no_std)
#[allow(dead_code)]
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

pub struct SigmaAI { active: bool }
impl SigmaAI {
    pub const fn new() -> Self { SigmaAI { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_INSTANCE: SigmaAI = SigmaAI::new();
#[no_mangle]
pub unsafe extern "C" fn sigma_ai_init() -> SigmaStatus { G_INSTANCE.init() }
#[no_mangle]
pub unsafe extern "C" fn sigma_ai_active() -> u8 { G_INSTANCE.is_active() as u8 }