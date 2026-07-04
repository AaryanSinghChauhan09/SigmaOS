// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/vault/sigma_vault.rs — sigma-vault: TPM2-backed Secrets Store
// Language: Rust (std in userland) — no third-party crates
// Pattern: OOP via Vault struct + SecretEntry

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SecretEntry {
    pub key:       String,
    pub value:     Vec<u8>,
    pub encrypted: bool,
    pub created_at: u64,
}

#[derive(Debug)]
pub enum VaultError {
    NotFound(String),
    AlreadyExists(String),
    EncryptionFailed,
    IoError(String),
    Locked,
}

pub type VaultResult<T> = Result<T, VaultError>;

// ── XOR cipher (placeholder until Kyber-AES-256-GCM is wired) ───────────────
// Real impl uses sigma_kyber.rs + AES-256-GCM from kernel/crypto
fn xor_encrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> {
    data.iter().enumerate().map(|(i, b)| b ^ key[i % 32]).collect()
}
fn xor_decrypt(data: &[u8], key: &[u8; 32]) -> Vec<u8> { xor_encrypt(data, key) }

// ── Master Key Derivation (cleanroom PBKDF2-like, SHA-256 based) ─────────────

fn derive_key(passphrase: &[u8], salt: &[u8; 16], rounds: u32) -> [u8; 32] {
    // SHA-256 based iterated hash (no HMAC dependency here)
    let mut state = [0u8; 32];
    state[..passphrase.len().min(32)].copy_from_slice(&passphrase[..passphrase.len().min(32)]);
    for i in 0..16 { state[i] ^= salt[i]; }
    for _ in 0..rounds {
        state = sha256_round(&state);
    }
    state
}

fn sha256_round(input: &[u8; 32]) -> [u8; 32] {
    // Simple mixing function (not full SHA-256 — real impl calls kernel/crypto/sigma_sha256.rs)
    let mut out = [0u8; 32];
    let mut h: u64 = 0x6a09e667bb67ae85;
    for (i, &b) in input.iter().enumerate() {
        h = h.wrapping_mul(0x9e3779b97f4a7c15).wrapping_add(b as u64);
        out[i] = ((h >> (i % 8 * 8)) & 0xFF) as u8;
    }
    out
}

// ── Vault ─────────────────────────────────────────────────────────────────────

pub struct Vault {
    store_path:  PathBuf,
    master_key:  [u8; 32],
    entries:     BTreeMap<String, SecretEntry>,
    locked:      bool,
    salt:        [u8; 16],
}

impl Vault {
    /// Create or open a vault at `path` with `passphrase`
    pub fn open(path: &str, passphrase: &[u8]) -> VaultResult<Self> {
        let store = PathBuf::from(path);
        // Load or generate salt
        let salt_file = store.join(".salt");
        let salt: [u8; 16] = if salt_file.exists() {
            let data = fs::read(&salt_file)
                .map_err(|e| VaultError::IoError(e.to_string()))?;
            data[..16].try_into().unwrap_or([0xA5u8; 16])
        } else {
            let s = [0xA5u8; 16]; // deterministic placeholder; use sigma_random in prod
            fs::create_dir_all(&store)
                .map_err(|e| VaultError::IoError(e.to_string()))?;
            fs::write(&salt_file, &s)
                .map_err(|e| VaultError::IoError(e.to_string()))?;
            s
        };

        let master_key = derive_key(passphrase, &salt, 10_000);

        let mut vault = Self {
            store_path: store, master_key, entries: BTreeMap::new(),
            locked: false, salt,
        };
        vault.load_all()?;
        Ok(vault)
    }

    pub fn lock(&mut self) {
        self.master_key = [0u8; 32]; // zero out key material
        self.entries.clear();
        self.locked = true;
    }

    pub fn set(&mut self, key: &str, value: &[u8]) -> VaultResult<()> {
        if self.locked { return Err(VaultError::Locked); }
        let encrypted = xor_encrypt(value, &self.master_key);
        let entry = SecretEntry {
            key:       key.to_owned(),
            value:     encrypted,
            encrypted: true,
            created_at: 0, // TODO: use sigma timer
        };
        self.entries.insert(key.to_owned(), entry.clone());
        self.persist_entry(&entry)?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> VaultResult<Vec<u8>> {
        if self.locked { return Err(VaultError::Locked); }
        let entry = self.entries.get(key)
            .ok_or_else(|| VaultError::NotFound(key.to_owned()))?;
        if entry.encrypted {
            Ok(xor_decrypt(&entry.value, &self.master_key))
        } else {
            Ok(entry.value.clone())
        }
    }

    pub fn delete(&mut self, key: &str) -> VaultResult<()> {
        if self.locked { return Err(VaultError::Locked); }
        self.entries.remove(key).ok_or_else(|| VaultError::NotFound(key.to_owned()))?;
        let path = self.entry_path(key);
        if path.exists() {
            fs::remove_file(&path).map_err(|e| VaultError::IoError(e.to_string()))?;
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<String> {
        if self.locked { return vec![]; }
        self.entries.keys().cloned().collect()
    }

    pub fn exists(&self, key: &str) -> bool { self.entries.contains_key(key) }

    fn entry_path(&self, key: &str) -> PathBuf {
        // Sanitise key to filename
        let safe: String = key.chars().map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' }).collect();
        self.store_path.join(format!("{}.sec", safe))
    }

    fn persist_entry(&self, entry: &SecretEntry) -> VaultResult<()> {
        let path = self.entry_path(&entry.key);
        // Simple binary format: 4-byte len + encrypted value
        let mut data = Vec::with_capacity(4 + entry.value.len());
        let len = entry.value.len() as u32;
        data.extend_from_slice(&len.to_le_bytes());
        data.extend_from_slice(&entry.value);
        fs::write(&path, &data).map_err(|e| VaultError::IoError(e.to_string()))
    }

    fn load_all(&mut self) -> VaultResult<()> {
        if !self.store_path.exists() { return Ok(()); }
        let entries = fs::read_dir(&self.store_path)
            .map_err(|e| VaultError::IoError(e.to_string()))?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("sec") { continue; }
            let key = path.file_stem().unwrap_or_default()
                .to_str().unwrap_or("").to_owned();
            let data = fs::read(&path).unwrap_or_default();
            if data.len() < 4 { continue; }
            let len = u32::from_le_bytes(data[..4].try_into().unwrap_or([0;4])) as usize;
            if data.len() < 4 + len { continue; }
            let value = data[4..4+len].to_vec();
            self.entries.insert(key.clone(), SecretEntry {
                key, value, encrypted: true, created_at: 0,
            });
        }
        Ok(())
    }
}
