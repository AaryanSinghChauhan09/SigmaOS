//! RNG interfaces for the kernel library.
//!
//! `SigmaRng` is a deterministic, non-cryptographic generator intended only
//! for simulation. `OsRng` is currently unavailable and fails closed; it does
//! not pretend that a fixed seed is operating-system entropy.

use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RngError {
    EntropyUnavailable,
}

pub trait Rng {
    fn next_u8(&self) -> Result<u8, RngError>;
    fn next_u32(&self) -> Result<u32, RngError>;
    fn next_u64(&self) -> Result<u64, RngError>;
    fn fill_bytes(&self, dest: &mut [u8]) -> Result<(), RngError>;
}

/// Deterministic xorshift* generator for simulations and reproducible tests.
/// This type is not suitable for keys, nonces, tokens, or other secrets.
pub struct SigmaRng {
    state: AtomicU64,
}

impl SigmaRng {
    pub const fn new() -> Self {
        Self {
            state: AtomicU64::new(0x123456789ABCDEF0),
        }
    }

    /// Set deterministic simulation state. Zero is valid and produces the
    /// documented all-zero xorshift stream; it is never cryptographic entropy.
    pub fn seed(&self, seed: u64) {
        self.state.store(seed, Ordering::SeqCst);
    }

    fn step(&self) -> u64 {
        let mut current = self.state.load(Ordering::Relaxed);
        loop {
            let mut next = current;
            next ^= next >> 12;
            next ^= next << 25;
            next ^= next >> 27;
            match self.state.compare_exchange_weak(
                current,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return next.wrapping_mul(0x2545F4914F6CDD1D),
                Err(observed) => current = observed,
            }
        }
    }
}

impl Default for SigmaRng {
    fn default() -> Self {
        Self::new()
    }
}

impl Rng for SigmaRng {
    fn next_u8(&self) -> Result<u8, RngError> {
        Ok(self.step() as u8)
    }

    fn next_u32(&self) -> Result<u32, RngError> {
        Ok(self.step() as u32)
    }

    fn next_u64(&self) -> Result<u64, RngError> {
        Ok(self.step())
    }

    fn fill_bytes(&self, dest: &mut [u8]) -> Result<(), RngError> {
        for chunk in dest.chunks_mut(8) {
            let bytes = self.step().to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
        Ok(())
    }
}

/// OS-backed secure randomness placeholder. A syscall/provider has not been
/// integrated, so every operation returns an error without modifying outputs.
pub struct OsRng;

impl OsRng {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for OsRng {
    fn default() -> Self {
        Self::new()
    }
}

impl Rng for OsRng {
    fn next_u8(&self) -> Result<u8, RngError> {
        Err(RngError::EntropyUnavailable)
    }

    fn next_u32(&self) -> Result<u32, RngError> {
        Err(RngError::EntropyUnavailable)
    }

    fn next_u64(&self) -> Result<u64, RngError> {
        Err(RngError::EntropyUnavailable)
    }

    fn fill_bytes(&self, _dest: &mut [u8]) -> Result<(), RngError> {
        Err(RngError::EntropyUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::{OsRng, Rng, RngError, SigmaRng};

    #[test]
    fn deterministic_rng_is_repeatable_and_explicitly_separate() {
        let a = SigmaRng::new();
        let b = SigmaRng::new();
        assert_eq!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn os_rng_fails_closed_without_modifying_buffer() {
        let rng = OsRng::new();
        let mut bytes = [0xA5; 16];
        assert_eq!(
            rng.fill_bytes(&mut bytes),
            Err(RngError::EntropyUnavailable)
        );
        assert_eq!(bytes, [0xA5; 16]);
        assert_eq!(rng.next_u64(), Err(RngError::EntropyUnavailable));
    }
}
