// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Kernel OOP Traits (Rust, no_std)
//! =========================================================================

pub trait SigmaObject {
    fn initialize(&mut self) -> super::types::SigmaStatus;
    fn class_name(&self) -> &'static str;
}

pub trait SigmaSingleton: SigmaObject {
    fn get_instance() -> &'static mut Self;
}
