// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// crypto/sigma_kyber.rs — Kyber-1024 KEM (Post-Quantum)
// Replaces: sigma_kyber.cpp (C++ stub, removed)
//
// Language: Rust #![no_std] — no libc, no third-party crates
// Pattern: OOP via KyberKem struct + methods
//
// NOTE: This is a structural skeleton with correct parameter constants
// and the NTT/polynomial framework. Full NTT body to be completed per
// NIST FIPS 203 specification. Use liboqs test vectors for validation.

#![no_std]

// ── Parameters (Kyber-1024 / NIST ML-KEM-1024) ───────────────────────────────

pub const KYBER_N:         usize = 256;   // polynomial degree
pub const KYBER_Q:         u32   = 3329;  // prime modulus
pub const KYBER_K:         usize = 4;     // rank (Kyber-1024)
pub const KYBER_ETA1:      usize = 2;     // noise parameter
pub const KYBER_ETA2:      usize = 2;
pub const KYBER_DU:        usize = 11;    // compression bits (ciphertext u)
pub const KYBER_DV:        usize = 5;     // compression bits (ciphertext v)
pub const KYBER_SYMBYTES:  usize = 32;    // seed size
pub const PK_BYTES:        usize = 1568;  // public key size (Kyber-1024)
pub const SK_BYTES:        usize = 3168;  // secret key size
pub const CT_BYTES:        usize = 1568;  // ciphertext size
pub const SS_BYTES:        usize = 32;    // shared secret size

// ── Polynomial ────────────────────────────────────────────────────────────────

/// A polynomial in Z_q[x] / (x^n + 1)
#[derive(Clone, Copy)]
pub struct Poly {
    pub coeffs: [i16; KYBER_N],
}

impl Poly {
    pub const fn zero() -> Self { Self { coeffs: [0i16; KYBER_N] } }

    pub fn add(&self, rhs: &Poly) -> Poly {
        let mut r = Poly::zero();
        for i in 0..KYBER_N {
            r.coeffs[i] = barrett_reduce(self.coeffs[i] as i32 + rhs.coeffs[i] as i32);
        }
        r
    }

    pub fn sub(&self, rhs: &Poly) -> Poly {
        let mut r = Poly::zero();
        for i in 0..KYBER_N {
            r.coeffs[i] = barrett_reduce(self.coeffs[i] as i32 - rhs.coeffs[i] as i32);
        }
        r
    }
}

/// Barrett reduction: reduce a mod KYBER_Q
fn barrett_reduce(a: i32) -> i16 {
    const V: i32 = ((1 << 26) + KYBER_Q as i32 / 2) / KYBER_Q as i32;
    let t = V * a >> 26;
    let r = a - t * KYBER_Q as i32;
    r as i16
}

/// Montgomery reduction
fn montgomery_reduce(a: i32) -> i16 {
    const QINV: i32 = -3327; // q^-1 mod 2^16
    let u = (a as i16).wrapping_mul(QINV as i16);
    let t = (a - (u as i32) * KYBER_Q as i32) >> 16;
    t as i16
}

// ── NTT (Number Theoretic Transform) ─────────────────────────────────────────

// Precomputed zeta values for NTT (primitive 256th root of unity mod q = 17)
// zetas[i] = 17^bitrev7(i) mod 3329
const ZETAS: [i16; 128] = [
    2285, 2571, 2970, 1812, 1493, 1422,  287,  202,
    3158,  622, 1577,  182,  962, 2127, 1855, 1468,
     573, 2004,  264,  383, 2500, 1458, 1727, 3199,
    2648, 1017,  732,  608, 1787,  411, 3124, 1758,
    1223,  652, 2777, 1015, 2036, 1491, 3047, 1785,
     516, 3321,  408,  116,  196, 3303, 1350,  787,
    3233,  845, 2915, 2213, 2424, 2899, 2663, 3235,
    1735,  702,  672, 1558,  267, 1523, 2799,  715,
     268, 2662, 2159,  420, 2462,  669,  830, 1390,
    2408, 2361, 1355,  831, 2558, 1221, 2009,  781,
    2344, 2836, 1219, 2828, 1100, 2197,   45,  979,
    3273, 2633, 1467,  279,  491, 2977, 1595,  886,
    1853, 1673,  533,  140, 2724, 2289,  664, 1342,
     279,  584, 2498, 2469, 3112, 3063, 1024, 2704,
    1762,  505, 2861,  866,  229,  971, 2555, 2851,
    2917,    3, 1349,  869, 1708, 1068, 2891, 1898,
];

pub fn ntt(poly: &mut Poly) {
    let mut k = 0usize;
    let mut len = 128usize;
    while len >= 2 {
        let mut start = 0usize;
        while start < KYBER_N {
            k += 1;
            let zeta = ZETAS[k - 1] as i32;
            let mut j = start;
            while j < start + len {
                let t = montgomery_reduce(zeta * poly.coeffs[j + len] as i32);
                poly.coeffs[j + len] = poly.coeffs[j] - t;
                poly.coeffs[j]       = poly.coeffs[j] + t;
                j += 1;
            }
            start += 2 * len;
        }
        len /= 2;
    }
}

pub fn inv_ntt(poly: &mut Poly) {
    const INV_NTT_ZETAS: [i16; 128] = [
        // TODO: precompute inverse zeta table
        0i16; 128
    ];
    let _ = INV_NTT_ZETAS;
    // Inverse NTT body — mirror of ntt with inverse zetas
    // TODO: complete per FIPS 203 §4.3
}

// ── Key Generation & Encapsulation Stubs ─────────────────────────────────────

pub struct KyberKem {
    // Internal state — keys are caller-managed byte arrays
    _priv: (),
}

impl KyberKem {
    pub const fn new() -> Self { Self { _priv: () } }

    /// Generate a keypair.
    /// `pk` — public key buffer (PK_BYTES)
    /// `sk` — secret key buffer (SK_BYTES)
    /// `seed` — 32-byte random seed (from sigma-rng)
    pub fn keygen(&self, pk: &mut [u8; PK_BYTES], sk: &mut [u8; SK_BYTES], seed: &[u8; 32]) {
        // TODO: implement per NIST FIPS 203 ML-KEM.KeyGen
        // 1. Expand seed via SHAKE-256 → rho, sigma
        // 2. Generate matrix A from rho
        // 3. Sample s, e from CBD(sigma)
        // 4. NTT(s), NTT(e)
        // 5. t = NTT(A) * NTT(s) + NTT(e)
        // 6. Encode pk = (t, rho), sk = (s, pk, H(pk), z)
        let _ = (pk, sk, seed);
    }

    /// Encapsulate: produce ciphertext + shared secret.
    /// Returns (ciphertext CT_BYTES, shared_secret SS_BYTES)
    pub fn encapsulate(&self, pk: &[u8; PK_BYTES], rand: &[u8; 32],
                       ct: &mut [u8; CT_BYTES], ss: &mut [u8; SS_BYTES]) {
        // TODO: implement per FIPS 203 ML-KEM.Encaps
        let _ = (pk, rand, ct, ss);
    }

    /// Decapsulate: recover shared secret from ciphertext.
    pub fn decapsulate(&self, sk: &[u8; SK_BYTES], ct: &[u8; CT_BYTES],
                       ss: &mut [u8; SS_BYTES]) {
        // TODO: implement per FIPS 203 ML-KEM.Decaps
        let _ = (sk, ct, ss);
    }
}
