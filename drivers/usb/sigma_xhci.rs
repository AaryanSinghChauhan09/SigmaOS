// SPDX-License-Identifier: GPL-2.0-or-later
//! SIGMAOS: Xhci Driver (Rust, no_std) — Replaces: drivers/usb/sigma_xhci.cpp
pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
pub struct XhciDriver { active: bool }
impl XhciDriver {
    pub const fn new() -> Self { XhciDriver { active: false } }
    pub fn init(&mut self) -> SigmaStatus { self.active = true; SIGMA_OK }
    pub fn is_active(&self) -> bool { self.active }
}
static mut G_DRV: XhciDriver = XhciDriver::new();
#[no_mangle]
pub unsafe extern "C" fn sigma_xhci_drv_init() -> SigmaStatus { G_DRV.init() }
#[no_mangle]
pub unsafe extern "C" fn sigma_xhci_drv_active() -> u8 { G_DRV.is_active() as u8 }