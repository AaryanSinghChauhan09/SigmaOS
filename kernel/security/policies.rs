// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/security/policies.rs — Cryptographic Policies
//
// Implements cryptographic policy management for SigmaOS
// Allows system-wide control over allowed algorithms, key sizes, and protocols
// Inspired by Fedora crypto-policies and OpenSSH cipher configuration
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U32 = u32;
type U64 = u64;
type I32 = i32;

pub const CRYPTO_POLICY_OK: I32 = 0;
pub const CRYPTO_POLICY_ERR_INVALID: I32 = -1;
pub const CRYPTO_POLICY_ERR_NOT_SUPPORTED: I32 = -2;

// ─── Policy Levels ─────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CryptoPolicyLevel {
    Legacy,      // Allow legacy algorithms for compatibility
    Default,     // Balanced security and compatibility
    Future,      // Forward-looking security, may break compatibility
    Maximum,     // Maximum security, strict requirements
}

// ─── Cipher Suites ───────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CipherSuite {
    Aes128Gcm,
    Aes256Gcm,
    ChaCha20Poly1305,
    Aes128Cbc,
    Aes256Cbc,
    // Legacy ciphers
    Des3Cbc,
    Rc4,
}

// ─── Key Exchange Methods ───────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum KeyExchange {
    Ecdhe,
    Dhe,
    Rsa,
    // Legacy
    Dh,
}

// ─── Signature Algorithms ───────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SignatureAlgorithm {
    Ed25519,
    EcdsaP256,
    EcdsaP384,
    RsaPss2048,
    RsaPss3072,
    RsaPss4096,
    // Legacy
    RsaPkcs12048,
    RsaPkcs13072,
    Sha1Rsa,
}

// ─── Hash Algorithms ─────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
    Blake3,
    // Legacy
    Sha1,
    Md5,
}

// ─── Cryptographic Policy ───────────────────────────────────────────────────

#[repr(C)]
pub struct CryptoPolicy {
    pub level: CryptoPolicyLevel,
    pub allowed_ciphers: [bool; 8],
    pub allowed_key_exchange: [bool; 4],
    pub allowed_signatures: [bool; 8],
    pub allowed_hashes: [bool; 6],
    pub min_key_size: U32,
    pub min_dh_group_size: U32,
    pub tls_version_min: U8,
    pub tls_version_max: U8,
}

impl CryptoPolicy {
    pub const fn new() -> Self {
        Self {
            level: CryptoPolicyLevel::Default,
            allowed_ciphers: [false; 8],
            allowed_key_exchange: [false; 4],
            allowed_signatures: [false; 8],
            allowed_hashes: [false; 6],
            min_key_size: 2048,
            min_dh_group_size: 2048,
            tls_version_min: 0x03, // TLS 1.2
            tls_version_max: 0x04, // TLS 1.3
        }
    }

    pub const fn legacy() -> Self {
        let mut policy = Self::new();
        policy.level = CryptoPolicyLevel::Legacy;
        policy.min_key_size = 1024;
        policy.min_dh_group_size = 1024;
        policy.tls_version_min = 0x01; // TLS 1.0
        policy
    }

    pub const fn default() -> Self {
        let mut policy = Self::new();
        policy.level = CryptoPolicyLevel::Default;
        policy
    }

    pub const fn future() -> Self {
        let mut policy = Self::new();
        policy.level = CryptoPolicyLevel::Future;
        policy.min_key_size = 3072;
        policy.min_dh_group_size = 3072;
        policy.tls_version_min = 0x04; // TLS 1.3 only
        policy
    }

    pub const fn maximum() -> Self {
        let mut policy = Self::new();
        policy.level = CryptoPolicyLevel::Maximum;
        policy.min_key_size = 4096;
        policy.min_dh_group_size = 4096;
        policy.tls_version_min = 0x04; // TLS 1.3 only
        policy
    }
}

// ─── Policy Manager ─────────────────────────────────────────────────────────

pub struct CryptoPolicyManager {
    pub current_policy: CryptoPolicy,
    pub policy_name: [U8; 64],
}

impl CryptoPolicyManager {
    pub const fn new() -> Self {
        Self {
            current_policy: CryptoPolicy::default(),
            policy_name: *b"DEFAULT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        }
    }

    /// Initialize policy manager
    pub unsafe fn init(&mut self) -> I32 {
        self.load_policy();
        CRYPTO_POLICY_OK
    }

    /// Set policy level
    pub unsafe fn set_policy_level(&mut self, level: CryptoPolicyLevel) -> I32 {
        self.current_policy = match level {
            CryptoPolicyLevel::Legacy => CryptoPolicy::legacy(),
            CryptoPolicyLevel::Default => CryptoPolicy::default(),
            CryptoPolicyLevel::Future => CryptoPolicy::future(),
            CryptoPolicyLevel::Maximum => CryptoPolicy::maximum(),
        };

        // Configure allowed algorithms based on level
        self.configure_allowed_algorithms();

        // Save policy
        self.save_policy();

        CRYPTO_POLICY_OK
    }

    /// Check if cipher is allowed
    pub fn is_cipher_allowed(&self, cipher: CipherSuite) -> bool {
        let index = cipher as usize;
        if index < self.current_policy.allowed_ciphers.len() {
            self.current_policy.allowed_ciphers[index]
        } else {
            false
        }
    }

    /// Check if key exchange is allowed
    pub fn is_key_exchange_allowed(&self, kex: KeyExchange) -> bool {
        let index = kex as usize;
        if index < self.current_policy.allowed_key_exchange.len() {
            self.current_policy.allowed_key_exchange[index]
        } else {
            false
        }
    }

    /// Check if signature algorithm is allowed
    pub fn is_signature_allowed(&self, sig: SignatureAlgorithm) -> bool {
        let index = sig as usize;
        if index < self.current_policy.allowed_signatures.len() {
            self.current_policy.allowed_signatures[index]
        } else {
            false
        }
    }

    /// Check if hash algorithm is allowed
    pub fn is_hash_allowed(&self, hash: HashAlgorithm) -> bool {
        let index = hash as usize;
        if index < self.current_policy.allowed_hashes.len() {
            self.current_policy.allowed_hashes[index]
        } else {
            false
        }
    }

    /// Check if key size meets minimum requirements
    pub fn is_key_size_allowed(&self, key_size: U32) -> bool {
        key_size >= self.current_policy.min_key_size
    }

    /// Get current policy
    pub fn get_policy(&self) -> &CryptoPolicy {
        &self.current_policy
    }

    /// Configure allowed algorithms based on policy level
    fn configure_allowed_algorithms(&mut self) {
        match self.current_policy.level {
            CryptoPolicyLevel::Legacy => {
                // Allow legacy algorithms
                self.current_policy.allowed_ciphers[CipherSuite::Aes128Gcm as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::Aes256Gcm as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::ChaCha20Poly1305 as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::Aes128Cbc as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::Aes256Cbc as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::Des3Cbc as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::Rc4 as usize] = true;

                self.current_policy.allowed_key_exchange[KeyExchange::Ecdhe as usize] = true;
                self.current_policy.allowed_key_exchange[KeyExchange::Dhe as usize] = true;
                self.current_policy.allowed_key_exchange[KeyExchange::Rsa as usize] = true;
                self.current_policy.allowed_key_exchange[KeyExchange::Dh as usize] = true;

                self.current_policy.allowed_signatures[SignatureAlgorithm::Ed25519 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::EcdsaP256 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::EcdsaP384 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::RsaPss2048 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::RsaPss3072 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::RsaPss4096 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::RsaPkcs12048 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::RsaPkcs13072 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::Sha1Rsa as usize] = true;

                self.current_policy.allowed_hashes[HashAlgorithm::Sha256 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Sha384 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Sha512 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Blake3 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Sha1 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Md5 as usize] = true;
            }
            CryptoPolicyLevel::Default => {
                // Balanced security
                self.current_policy.allowed_ciphers[CipherSuite::Aes128Gcm as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::Aes256Gcm as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::ChaCha20Poly1305 as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::Aes128Cbc as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::Aes256Cbc as usize] = true;

                self.current_policy.allowed_key_exchange[KeyExchange::Ecdhe as usize] = true;
                self.current_policy.allowed_key_exchange[KeyExchange::Dhe as usize] = true;
                self.current_policy.allowed_key_exchange[KeyExchange::Rsa as usize] = true;

                self.current_policy.allowed_signatures[SignatureAlgorithm::Ed25519 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::EcdsaP256 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::EcdsaP384 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::RsaPss2048 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::RsaPss3072 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::RsaPss4096 as usize] = true;

                self.current_policy.allowed_hashes[HashAlgorithm::Sha256 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Sha384 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Sha512 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Blake3 as usize] = true;
            }
            CryptoPolicyLevel::Future => {
                // Forward-looking security
                self.current_policy.allowed_ciphers[CipherSuite::Aes128Gcm as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::Aes256Gcm as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::ChaCha20Poly1305 as usize] = true;

                self.current_policy.allowed_key_exchange[KeyExchange::Ecdhe as usize] = true;

                self.current_policy.allowed_signatures[SignatureAlgorithm::Ed25519 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::EcdsaP256 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::EcdsaP384 as usize] = true;

                self.current_policy.allowed_hashes[HashAlgorithm::Sha256 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Sha384 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Sha512 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Blake3 as usize] = true;
            }
            CryptoPolicyLevel::Maximum => {
                // Maximum security
                self.current_policy.allowed_ciphers[CipherSuite::Aes256Gcm as usize] = true;
                self.current_policy.allowed_ciphers[CipherSuite::ChaCha20Poly1305 as usize] = true;

                self.current_policy.allowed_key_exchange[KeyExchange::Ecdhe as usize] = true;

                self.current_policy.allowed_signatures[SignatureAlgorithm::Ed25519 as usize] = true;
                self.current_policy.allowed_signatures[SignatureAlgorithm::EcdsaP384 as usize] = true;

                self.current_policy.allowed_hashes[HashAlgorithm::Sha384 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Sha512 as usize] = true;
                self.current_policy.allowed_hashes[HashAlgorithm::Blake3 as usize] = true;
            }
        }
    }

    /// Load policy from disk
    unsafe fn load_policy(&mut self) {
        // In real implementation, read from policy file
        // For now, use default
    }

    /// Save policy to disk
    unsafe fn save_policy(&self) {
        // In real implementation, write to policy file
    }
}

// ─── Global Policy Manager ───────────────────────────────────────────────────

static mut POLICY_MANAGER: CryptoPolicyManager = CryptoPolicyManager::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_policy_init() -> I32 {
    POLICY_MANAGER.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_policy_set_level(level: U32) -> I32 {
    let policy_level = match level {
        0 => CryptoPolicyLevel::Legacy,
        1 => CryptoPolicyLevel::Default,
        2 => CryptoPolicyLevel::Future,
        3 => CryptoPolicyLevel::Maximum,
        _ => return CRYPTO_POLICY_ERR_INVALID,
    };
    POLICY_MANAGER.set_policy_level(policy_level)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_policy_is_cipher_allowed(cipher: U32) -> bool {
    let cipher_suite = match cipher {
        0 => CipherSuite::Aes128Gcm,
        1 => CipherSuite::Aes256Gcm,
        2 => CipherSuite::ChaCha20Poly1305,
        3 => CipherSuite::Aes128Cbc,
        4 => CipherSuite::Aes256Cbc,
        5 => CipherSuite::Des3Cbc,
        6 => CipherSuite::Rc4,
        _ => return false,
    };
    POLICY_MANAGER.is_cipher_allowed(cipher_suite)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_policy_is_key_size_allowed(key_size: U32) -> bool {
    POLICY_MANAGER.is_key_size_allowed(key_size)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_crypto_policy_get_min_key_size() -> U32 {
    POLICY_MANAGER.current_policy.min_key_size
}
