// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Kernel HAL (Rust, no_std)
//! =========================================================================

pub trait SigmaHAL {
    fn read_register(&self, address: usize) -> u32;
    fn write_register(&mut self, address: usize, value: u32);
    fn irq_enable(&mut self);
    fn irq_disable(&mut self);
}
