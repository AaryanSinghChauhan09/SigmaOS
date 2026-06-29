# SPDX-License-Identifier: GPL-2.0-or-later
//! crypto/cryptfs/derive.rs — SigmaOS CryptFS Key Derivation
//!
//! Replaces the prior zero-byte stub with Argon2id (memory-hard, modern
//! standard). This module runs in **userspace** (the init daemon calls it
//! and passes the derived key to the kernel via a secure channel), which
//! avoids pulling heap-heavy Argon2 code into the no_std kernel binary.
//!
//! # Migration path for existing volumes
//! Any volume encrypted with the old stub (derive_key returned [0u8; 32])
//! must be re-wrapped: decrypt with the zero key, re-encrypt with the new
//! key derived from the user's passphrase. See `sigmad/cryptfs_migrate.rs`.
//!
//! # TODO (Phase 2 — kernel inline)
//! Port to a no_std + no_alloc C-ABI binding against libsodium's
//! crypto_pwhash_* via the kabi/ FFI boundary when the kernel allocator
//! is mature enough to host Argon2's 64 MB working set.

use argon2::{Algorithm, Argon2, Params, Version};

/// Argon2id parameters: 64 MiB memory, 3 iterations, 4 threads, 32-byte output.
/// Matches OWASP 2023 minimum recommendation for interactive logins.
const MEM_KIB: u32 = 64 * 1024; // 64 MiB
const ITERATIONS: u32 = 3;
const PARALLELISM: u32 = 4;
const KEY_LEN: usize = 32;

/// Derive a 256-bit volume encryption key from a passphrase + 128-bit salt.
///
/// # Panics
/// Panics on invalid Argon2 parameters (compile-time constants — will never
/// panic in a correct build).
pub fn derive_key(password: &[u8], salt: &[u8; 16]) -> [u8; KEY_LEN] {
    let params = Params::new(MEM_KIB, ITERATIONS, PARALLELISM, Some(KEY_LEN))
        .expect("argon2 params are compile-time constants and are always valid");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut out = [0u8; KEY_LEN];
    argon2
        .hash_password_into(password, salt, &mut out)
        .expect("argon2 hash_password_into: static output buffer — never fails");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression test: derived key must never be the zero-byte stub output.
    /// This test MUST pass before any release that ships CryptFS.
    #[test]
    fn derive_key_is_nonzero() {
        let key = derive_key(b"correct-horse-battery-staple", &[0u8; 16]);
        assert_ne!(key, [0u8; 32], "derive_key returned zero bytes — stub not replaced!");
    }

    /// Same passphrase + salt always produces the same key (deterministic).
    #[test]
    fn derive_key_is_deterministic() {
        let salt = [0x42u8; 16];
        let a = derive_key(b"sigma-passphrase", &salt);
        let b = derive_key(b"sigma-passphrase", &salt);
        assert_eq!(a, b, "Key derivation must be deterministic");
    }

    /// Different passphrases produce different keys (no collisions in test space).
    #[test]
    fn different_passphrases_different_keys() {
        let salt = [0u8; 16];
        let k1 = derive_key(b"passphrase-one", &salt);
        let k2 = derive_key(b"passphrase-two", &salt);
        assert_ne!(k1, k2, "Different passphrases must produce different keys");
    }

    /// Different salts produce different keys (salt is not ignored).
    #[test]
    fn different_salts_different_keys() {
        let k1 = derive_key(b"same-passphrase", &[0u8; 16]);
        let k2 = derive_key(b"same-passphrase", &[1u8; 16]);
        assert_ne!(k1, k2, "Different salts must produce different keys");
    }
}
