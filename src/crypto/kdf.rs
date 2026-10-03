#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use std::vec::Vec;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KDFAlgorithm {
    PBKDF2 = 0,
    Argon2id = 1,
    HKDF = 2,
    Scrypt = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KDFError {
    Success = 0,
    InvalidParameters = 1,
    DerivationFailed = 2,
    ProviderUnavailable = 3,
}

pub trait KeyDerivation {
    fn algorithm(&self) -> KDFAlgorithm;
    fn derive_key(
        &self,
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_len: usize,
    ) -> Result<Vec<u8>, KDFError>;
}

pub struct SimpleKeyDerivation {
    pub algo: KDFAlgorithm,
}

impl SimpleKeyDerivation {
    pub fn new(algo: KDFAlgorithm) -> Self {
        SimpleKeyDerivation { algo }
    }
}

impl KeyDerivation for SimpleKeyDerivation {
    fn algorithm(&self) -> KDFAlgorithm {
        self.algo
    }

    fn derive_key(
        &self,
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_len: usize,
    ) -> Result<Vec<u8>, KDFError> {
        let mut key = Vec::with_capacity(key_len);
        let mut state: u32 = 0x85ebca6b;

        for &b in password {
            state = state.wrapping_add(b as u32).wrapping_mul(31);
        }
        for &b in salt {
            state = state.wrapping_add(b as u32).wrapping_mul(37);
        }

        for _ in 0..iterations {
            state = state.wrapping_add(1).wrapping_mul(1664525).wrapping_add(1013904223);
        }

        for i in 0..key_len {
            key.push(((state.wrapping_add(i as u32)) % 256) as u8);
        }

        Ok(key)
    }
}