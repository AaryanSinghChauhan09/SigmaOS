// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: SigmaWarehouse (Rust, no_std)
#[allow(dead_code)]
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

pub struct SigmaWarehouse { active: bool }
impl SigmaWarehouse {
    pub const fn new() -> Self { SigmaWarehouse { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_INSTANCE: SigmaWarehouse = SigmaWarehouse::new();
#[no_mangle]
pub unsafe extern "C" fn sigma_warehouse_init() -> SigmaStatus { G_INSTANCE.init() }
#[no_mangle]
pub unsafe extern "C" fn sigma_warehouse_active() -> u8 { G_INSTANCE.is_active() as u8 }