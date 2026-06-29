// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Kernel Types (Rust, no_std)
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK: SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

pub type SigmaHandle = usize;
pub type VirtualAddress = usize;
pub type PhysicalAddress = usize;
