// SigmaOS Capability-Based Security System
// Implements 64-bit hardware-enforced capability model

use std::string::String;
use std::sync::atomic::{AtomicU64, Ordering};
use std::vec::Vec;

/// Capability token representing access rights
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken {
    /// 64-bit capability bitmask
    bits: u64,
}

impl CapabilityToken {
    /// Create a new capability token with no permissions
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    pub fn allow_capability(&mut self, bit: u64) {
        self.bits |= bit;
    }
