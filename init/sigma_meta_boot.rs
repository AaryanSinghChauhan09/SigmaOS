// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SigmaMetaBoot (Rust, no_std)
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct SigmaMetaBoot { ready: bool }
impl SigmaMetaBoot {
    pub const fn new() -> Self { Self { ready: false } }
    pub fn init(&mut self) -> SigmaStatus { self.ready = true; SIGMA_OK }
}