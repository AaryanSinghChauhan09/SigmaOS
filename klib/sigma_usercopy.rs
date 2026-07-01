// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: sigma_usercopy (Rust, no_std)
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SigmaUsercopy { active: bool }
impl SigmaUsercopy {
    pub const fn new() -> Self { Self { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
}