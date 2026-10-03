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
use core::sync::atomic::{AtomicUsize, Ordering};

pub type CipherID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherMode {
    ECB = 0,
    CBC = 1,
    GCM = 2,
    CTR = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherError {
    Success = 0,
    InvalidKey = 1,
    InvalidIV = 2,
    EncryptionFailed = 3,
    CryptoUnavailable = 4,
}

pub trait BlockCipher {
    fn id(&self) -> CipherID;
    fn block_size(&self) -> usize;
    fn key_size(&self) -> usize;
    fn encrypt(&self, plaintext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError>;
    fn decrypt(&self, ciphertext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError>;
}

#[repr(C)]
pub struct SimpleAES {
    pub id: CipherID,
    pub mode: AtomicUsize,
}

impl SimpleAES {
    pub fn new(id: CipherID, mode: CipherMode) -> Self {
        SimpleAES {
            id,
            mode: AtomicUsize::new(mode as usize),
        }
    }
}

impl BlockCipher for SimpleAES {
    fn id(&self) -> CipherID {
        self.id
    }

    fn block_size(&self) -> usize {
        16
    }

    fn key_size(&self) -> usize {
        32
    }

    fn encrypt(&self, plaintext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError> {
        if key.len() != 32 {
            return Err(CipherError::InvalidKey);
        }

        let mut ciphertext = Vec::new();
        let mut key_hash: usize = 0;

        for &byte in key {
            key_hash = key_hash.wrapping_add(byte as usize);
        }

        if let Some(iv_data) = iv {
            for &byte in iv_data {
                key_hash = key_hash.wrapping_add(byte as usize);
            }
        }

        for &byte in plaintext {
            ciphertext.push(byte.wrapping_add((key_hash % 256) as u8));
            key_hash = key_hash.wrapping_mul(17);
        }

        Ok(ciphertext)
    }

    fn decrypt(&self, ciphertext: &[u8], key: &[u8], iv: Option<&[u8]>) -> Result<Vec<u8>, CipherError> {
        if key.len() != 32 {
            return Err(CipherError::InvalidKey);
        }

        let mut plaintext = Vec::new();
        let mut key_hash: usize = 0;

        for &byte in key {
            key_hash = key_hash.wrapping_add(byte as usize);
        }

        if let Some(iv_data) = iv {
            for &byte in iv_data {
                key_hash = key_hash.wrapping_add(byte as usize);
            }
        }

        for &byte in ciphertext {
            plaintext.push(byte.wrapping_sub((key_hash % 256) as u8));
            key_hash = key_hash.wrapping_mul(17);
        }

        Ok(plaintext)
    }
}