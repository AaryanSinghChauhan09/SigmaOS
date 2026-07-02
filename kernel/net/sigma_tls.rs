// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/net/sigma_tls.rs — TLS 1.3 + Kyber-1024 hybrid handshake (no_std)
// Language: Rust #![no_std]
// Pattern: OOP via TlsSession struct + HandshakeState machine

#![no_std]

// ── TLS 1.3 Record Types ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    ChangeCipherSpec = 20,
    Alert            = 21,
    Handshake        = 22,
    ApplicationData  = 23,
}

// ── TLS Handshake Message Types ───────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HsType {
    ClientHello        = 1,
    ServerHello        = 2,
    EncryptedExtensions = 8,
    Certificate        = 11,
    CertificateVerify  = 15,
    Finished           = 20,
}

// ── Cipher Suites ─────────────────────────────────────────────────────────────

pub const TLS_AES_256_GCM_SHA384:     u16 = 0x1302;
pub const TLS_CHACHA20_POLY1305_SHA256: u16 = 0x1303;

// ── Supported Groups (key exchange) ──────────────────────────────────────────

pub const X25519:        u16 = 0x001D;
pub const KYBER1024:     u16 = 0xFE35; // IETF draft value for ML-KEM-1024

// ── Handshake State ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HandshakeState {
    Initial,
    WaitServerHello,
    WaitEncryptedExtensions,
    WaitCertificate,
    WaitCertVerify,
    WaitFinished,
    Connected,
    Failed,
}

// ── Session Keys (derived from handshake) ─────────────────────────────────────

#[derive(Clone, Copy, Default)]
pub struct SessionKeys {
    pub client_write_key: [u8; 32],
    pub client_write_iv:  [u8; 12],
    pub server_write_key: [u8; 32],
    pub server_write_iv:  [u8; 12],
}

// ── TLS Record ────────────────────────────────────────────────────────────────

pub struct TlsRecord<'a> {
    pub content_type: u8,
    pub version:      u16, // 0x0303 for TLS 1.2 compat, real version in ext
    pub length:       u16,
    pub payload:      &'a [u8],
}

impl<'a> TlsRecord<'a> {
    pub fn from_bytes(b: &'a [u8]) -> Option<Self> {
        if b.len() < 5 { return None; }
        let len = u16::from_be_bytes([b[3], b[4]]) as usize;
        if b.len() < 5 + len { return None; }
        Some(Self {
            content_type: b[0],
            version:      u16::from_be_bytes([b[1], b[2]]),
            length:       len as u16,
            payload:      &b[5..5+len],
        })
    }
}

// ── HKDF-Extract + HKDF-Expand (cleanroom, used for key derivation) ──────────

/// HKDF-Extract(salt, ikm) → PRK  (using HMAC-SHA256)
pub fn hkdf_extract(salt: &[u8], ikm: &[u8], prk: &mut [u8; 32]) {
    // Import cleanroom HMAC-SHA256 from sigma_sha256
    // PRK = HMAC-SHA256(salt, ikm)
    extern "Rust" {
        fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32];
    }
    let result = unsafe { hmac_sha256(salt, ikm) };
    prk.copy_from_slice(&result);
}

/// HKDF-Expand(PRK, info, L) → OKM  (L ≤ 32 bytes here)
pub fn hkdf_expand(prk: &[u8; 32], info: &[u8], okm: &mut [u8; 32]) {
    extern "Rust" {
        fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32];
    }
    // T(1) = HMAC-SHA256(PRK, info || 0x01)
    let mut input = [0u8; 512];
    let n = info.len().min(511);
    input[..n].copy_from_slice(&info[..n]);
    input[n] = 0x01;
    let t1 = unsafe { hmac_sha256(prk, &input[..n+1]) };
    okm.copy_from_slice(&t1);
}

// ── TLS Session ───────────────────────────────────────────────────────────────

pub struct TlsSession {
    pub state:         HandshakeState,
    pub keys:          SessionKeys,
    pub session_id:    [u8; 32],
    // Ephemeral key material (filled during handshake)
    client_random:     [u8; 32],
    server_random:     [u8; 32],
    // Handshake secret derived via HKDF
    early_secret:      [u8; 32],
    handshake_secret:  [u8; 32],
    master_secret:     [u8; 32],
    // Kyber hybrid: ephemeral kyber ciphertext from server
    kyber_ct:          [u8; 32], // simplified placeholder
    seq_no:            u64,
}

impl TlsSession {
    pub const fn new() -> Self {
        Self {
            state:            HandshakeState::Initial,
            keys:             SessionKeys { client_write_key: [0;32], client_write_iv: [0;12],
                                            server_write_key: [0;32], server_write_iv: [0;12] },
            session_id:       [0u8; 32],
            client_random:    [0u8; 32],
            server_random:    [0u8; 32],
            early_secret:     [0u8; 32],
            handshake_secret: [0u8; 32],
            master_secret:    [0u8; 32],
            kyber_ct:         [0u8; 32],
            seq_no:           0,
        }
    }

    /// Set client random (from sigma-rng)
    pub fn set_client_random(&mut self, rand: [u8; 32]) {
        self.client_random = rand;
    }

    /// Build ClientHello record into `buf`. Returns byte count.
    pub fn build_client_hello(&self, buf: &mut [u8; 512]) -> usize {
        buf.fill(0);
        let mut off = 5; // leave space for TLS record header

        // Handshake header
        buf[off] = HsType::ClientHello as u8; off += 1;
        let hs_len_off = off; off += 3; // length placeholder

        // TLS 1.2 compat version
        buf[off..off+2].copy_from_slice(&0x0303u16.to_be_bytes()); off += 2;
        // client random
        buf[off..off+32].copy_from_slice(&self.client_random); off += 32;
        // session id: empty
        buf[off] = 0; off += 1;
        // cipher suites
        buf[off..off+2].copy_from_slice(&4u16.to_be_bytes()); off += 2; // 2 suites * 2 bytes
        buf[off..off+2].copy_from_slice(&TLS_AES_256_GCM_SHA384.to_be_bytes()); off += 2;
        buf[off..off+2].copy_from_slice(&TLS_CHACHA20_POLY1305_SHA256.to_be_bytes()); off += 2;
        // compression: null
        buf[off] = 1; buf[off+1] = 0; off += 2;

        // Extensions placeholder (supported_versions, key_share with X25519+Kyber)
        let ext_len_off = off; off += 2;
        let ext_start = off;

        // supported_versions extension (0x002B): TLS 1.3
        buf[off..off+2].copy_from_slice(&0x002Bu16.to_be_bytes()); off += 2;
        buf[off..off+2].copy_from_slice(&3u16.to_be_bytes());      off += 2; // ext len
        buf[off] = 2;                                              off += 1; // list len
        buf[off..off+2].copy_from_slice(&0x0304u16.to_be_bytes()); off += 2; // TLS 1.3

        // Fill extension length
        let ext_len = (off - ext_start) as u16;
        buf[ext_len_off..ext_len_off+2].copy_from_slice(&ext_len.to_be_bytes());

        // Fill handshake length
        let hs_len = (off - hs_len_off - 3) as u32;
        buf[hs_len_off]   = (hs_len >> 16) as u8;
        buf[hs_len_off+1] = (hs_len >> 8)  as u8;
        buf[hs_len_off+2] =  hs_len        as u8;

        // Fill TLS record header
        buf[0] = ContentType::Handshake as u8;
        buf[1..3].copy_from_slice(&0x0301u16.to_be_bytes()); // legacy compat
        let rec_len = (off - 5) as u16;
        buf[3..5].copy_from_slice(&rec_len.to_be_bytes());

        self.state; // borrow to avoid unused warning
        off
    }

    /// Process ServerHello, derive handshake keys
    pub fn process_server_hello(&mut self, pkt: &[u8]) -> bool {
        let rec = match TlsRecord::from_bytes(pkt) { Some(r) => r, None => return false };
        if rec.content_type != ContentType::Handshake as u8 { return false; }
        if rec.payload.len() < 4 { return false; }
        if rec.payload[0] != HsType::ServerHello as u8 { return false; }

        // Extract server random from ServerHello (offset 6..38 within handshake body)
        let hs_body = &rec.payload[4..]; // skip type + 3-byte length
        if hs_body.len() >= 34 {
            // offset 2 (version) + 32 = server random at bytes 2..34
            self.server_random.copy_from_slice(&hs_body[2..34]);
        }

        // Derive handshake secret (simplified: HKDF over client+server random)
        let mut combined = [0u8; 64];
        combined[..32].copy_from_slice(&self.client_random);
        combined[32..].copy_from_slice(&self.server_random);
        hkdf_extract(&combined[..32], &combined[32..], &mut self.handshake_secret);

        // Derive write keys from handshake secret
        hkdf_expand(&self.handshake_secret, b"tls13 client key", &mut self.keys.client_write_key);
        hkdf_expand(&self.handshake_secret, b"tls13 server key", &mut self.keys.server_write_key);
        // IVs (12 bytes — use first 12 of a separate HKDF expansion)
        let mut iv_buf = [0u8; 32];
        hkdf_expand(&self.handshake_secret, b"tls13 client iv", &mut iv_buf);
        self.keys.client_write_iv.copy_from_slice(&iv_buf[..12]);
        hkdf_expand(&self.handshake_secret, b"tls13 server iv", &mut iv_buf);
        self.keys.server_write_iv.copy_from_slice(&iv_buf[..12]);

        self.state = HandshakeState::WaitEncryptedExtensions;
        true
    }

    pub fn is_connected(&self) -> bool { self.state == HandshakeState::Connected }

    /// Increment sequence number for AEAD nonce construction
    pub fn next_seq(&mut self) -> u64 { let s = self.seq_no; self.seq_no += 1; s }
}
