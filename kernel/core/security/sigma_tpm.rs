// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/security/sigma_tpm.rs — TPM Attestation Workflows
//
// Implements TPM 2.0 attestation workflows for SigmaOS.
// Provides secure boot verification, remote attestation, and key sealing.
// Inspired by: TPM 2.0 specification, Intel TXT, AMD SEV
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum PCR count.
const MAX_PCRS: SigmaUsize = 24;
/// PCR digest length (SHA-256).
const PCR_DIGEST_LEN: SigmaUsize = 32;
/// Attestation report length.
const ATTESTATION_REPORT_LEN: SigmaUsize = 512;
/// Sealed key length.
const SEALED_KEY_LEN: SigmaUsize = 256;

// ── TPM Algorithm ───────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TpmAlgorithm {
    /// SHA-256.
    Sha256 = 0,
    /// SHA-384.
    Sha384 = 1,
    /// SHA-512.
    Sha512 = 2,
    /// RSA-2048.
    Rsa2048 = 3,
    /// ECC P-256.
    EccP256 = 4,
}

// ── PCR Index ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PcrIndex {
    /// Boot code.
    Pcr0 = 0,
    /// Boot configuration.
    Pcr1 = 1,
    /// External code.
    Pcr2 = 2,
    /// MBR code.
    Pcr3 = 3,
    /// Boot loader.
    Pcr4 = 4,
    /// Boot configuration 2.
    Pcr5 = 5,
    /// System state.
    Pcr6 = 6,
    /// System state 2.
    Pcr7 = 7,
    /// Kernel code.
    Pcr8 = 8,
    /// Kernel configuration.
    Pcr9 = 9,
    /// Boot parameters.
    Pcr10 = 10,
    /// Boot parameters 2.
    Pcr11 = 11,
    /// EFI variables.
    Pcr12 = 12,
    /// EFI variables 2.
    Pcr13 = 13,
    /// EFI variables 3.
    Pcr14 = 14,
    /// Boot debug.
    Pcr15 = 15,
}

// ── PCR Value ───────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PcrValue {
    pub index: PcrIndex,
    pub digest: [SigmaU8; PCR_DIGEST_LEN],
    pub algorithm: TpmAlgorithm,
}

impl PcrValue {
    pub const fn new(index: PcrIndex, algorithm: TpmAlgorithm) -> Self {
        Self {
            index,
            digest: [0u8; PCR_DIGEST_LEN],
            algorithm,
        }
    }
}

// ── Attestation Report ───────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttestationReport {
    pub pcrs: [PcrValue; MAX_PCRS],
    pub pcr_count: SigmaU32,
    pub timestamp: SigmaU64,
    pub signature: [SigmaU8; 64],
    pub aik_pubkey: [SigmaU8; 32],
    pub valid: SigmaBool,
    pub _pad: [SigmaU8; 7],
}

impl AttestationReport {
    pub const fn new() -> Self {
        Self {
            pcrs: [PcrValue::new(PcrIndex::Pcr0, TpmAlgorithm::Sha256); MAX_PCRS],
            pcr_count: 0,
            timestamp: 0,
            signature: [0u8; 64],
            aik_pubkey: [0u8; 32],
            valid: false,
            _pad: [0u8; 7],
        }
    }
}

// ── Sealed Key ───────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SealedKey {
    pub data: [SigmaU8; SEALED_KEY_LEN],
    pub pcr_mask: SigmaU32,
    pub sealed: SigmaBool,
}

impl SealedKey {
    pub const fn new() -> Self {
        Self {
            data: [0u8; SEALED_KEY_LEN],
            pcr_mask: 0,
            sealed: false,
        }
    }
}

// ── TPM Manager ─────────────────────────────────────────────────────────
pub struct TpmManager {
    pub pcrs: [PcrValue; MAX_PCRS],
    pub pcr_count: SigmaUsize,
    pub aik_available: SigmaBool,
    pub secure_boot: SigmaBool,
    pub initialized: SigmaBool,
}

impl TpmManager {
    pub const fn new() -> Self {
        Self {
            pcrs: [PcrValue::new(PcrIndex::Pcr0, TpmAlgorithm::Sha256); MAX_PCRS],
            pcr_count: 0,
            aik_available: false,
            secure_boot: false,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
        self.secure_boot = true;
        // Initialize PCR values
        self.pcr_count = 24;
        for i in 0..MAX_PCRS {
            self.pcrs[i] = PcrValue::new(match i {
                0 => PcrIndex::Pcr0,
                1 => PcrIndex::Pcr1,
                2 => PcrIndex::Pcr2,
                3 => PcrIndex::Pcr3,
                4 => PcrIndex::Pcr4,
                5 => PcrIndex::Pcr5,
                6 => PcrIndex::Pcr6,
                7 => PcrIndex::Pcr7,
                8 => PcrIndex::Pcr8,
                9 => PcrIndex::Pcr9,
                10 => PcrIndex::Pcr10,
                11 => PcrIndex::Pcr11,
                12 => PcrIndex::Pcr12,
                13 => PcrIndex::Pcr13,
                14 => PcrIndex::Pcr14,
                15 => PcrIndex::Pcr15,
                _ => PcrIndex::Pcr0,
            }, TpmAlgorithm::Sha256);
        }
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Extend a PCR with new measurement.
    pub fn extend_pcr(&mut self, index: PcrIndex, data: &[SigmaU8]) -> SigmaI32 {
        if index as usize >= MAX_PCRS {
            return -1;
        }

        // Simplified PCR extension
        // In production: use actual TPM_Extend
        let pcr_idx = index as usize;
        let mut hash: SigmaU32 = 5381;
        for &byte in data.iter().chain(self.pcrs[pcr_idx].digest.iter()) {
            hash = hash.wrapping_mul(33).wrapping_add(byte as SigmaU32);
        }

        // Convert hash to bytes
        for i in 0..PCR_DIGEST_LEN {
            self.pcrs[pcr_idx].digest[i] = ((hash >> (i * 4)) & 0xFF) as SigmaU8;
        }

        0
    }

    /// Read a PCR value.
    pub fn read_pcr(&self, index: PcrIndex) -> Option<&PcrValue> {
        if index as usize >= MAX_PCRS {
            return None;
        }
        Some(&self.pcrs[index as usize])
    }

    /// Generate attestation report.
    pub fn generate_attestation(&self, pcr_mask: SigmaU32) -> AttestationReport {
        let mut report = AttestationReport::new();
        report.timestamp = 0; // In production: get timestamp

        let mut count = 0;
        for i in 0..MAX_PCRS {
            if (pcr_mask & (1 << i)) != 0 {
                report.pcrs[count] = self.pcrs[i];
                count += 1;
            }
        }
        report.pcr_count = count as SigmaU32;

        // In production: sign report with AIK
        report.valid = self.secure_boot;
        report
    }

    /// Verify attestation report.
    pub fn verify_attestation(&self, report: &AttestationReport, aik_pubkey: &[SigmaU8]) -> SigmaBool {
        // In production: verify signature with AIK public key
        // For now, check if report is marked valid
        report.valid
    }

    /// Seal a key to PCR values.
    pub fn seal_key(&self, data: &[SigmaU8], pcr_mask: SigmaU32) -> SealedKey {
        let mut sealed = SealedKey::new();
        sealed.pcr_mask = pcr_mask;

        // Copy data
        let len = data.len().min(SEALED_KEY_LEN);
        let mut i = 0;
        while i < len {
            sealed.data[i] = data[i];
            i += 1;
        }

        // In production: use TPM_Seal
        sealed.sealed = true;
        sealed
    }

    /// Unseal a key from PCR values.
    pub fn unseal_key(&self, sealed: &SealedKey) -> Option<[SigmaU8; SEALED_KEY_LEN]> {
        if !sealed.sealed {
            return None;
        }

        // In production: verify PCR values match pcr_mask
        // For now, return the data
        Some(sealed.data)
    }

    /// Generate Attestation Identity Key (AIK).
    pub fn generate_aik(&mut self) -> SigmaI32 {
        // In production: use TPM_CreatePrimary and TPM_Create
        self.aik_available = true;
        0
    }

    /// Check if AIK is available.
    pub fn aik_available(&self) -> SigmaBool {
        self.aik_available
    }

    /// Enable/disable secure boot.
    pub fn set_secure_boot(&mut self, enabled: SigmaBool) {
        self.secure_boot = enabled;
    }

    /// Check secure boot status.
    pub fn secure_boot(&self) -> SigmaBool {
        self.secure_boot
    }

    /// Reset all PCRs.
    pub fn reset_pcrs(&mut self) {
        for i in 0..MAX_PCRS {
            self.pcrs[i].digest = [0u8; PCR_DIGEST_LEN];
        }
    }

    /// Get PCR count.
    pub fn pcr_count(&self) -> SigmaUsize {
        self.pcr_count
    }
}

static mut G_TPM_MGR: TpmManager = TpmManager::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_init() {
    G_TPM_MGR.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_extend_pcr(
    index: SigmaU32,
    data: *const SigmaU8,
    data_len: SigmaUsize,
) -> SigmaI32 {
    let pcr_idx = match index {
        0 => PcrIndex::Pcr0,
        1 => PcrIndex::Pcr1,
        2 => PcrIndex::Pcr2,
        3 => PcrIndex::Pcr3,
        4 => PcrIndex::Pcr4,
        5 => PcrIndex::Pcr5,
        6 => PcrIndex::Pcr6,
        7 => PcrIndex::Pcr7,
        8 => PcrIndex::Pcr8,
        9 => PcrIndex::Pcr9,
        10 => PcrIndex::Pcr10,
        11 => PcrIndex::Pcr11,
        12 => PcrIndex::Pcr12,
        13 => PcrIndex::Pcr13,
        14 => PcrIndex::Pcr14,
        15 => PcrIndex::Pcr15,
        _ => return -1,
    };
    let d = core::slice::from_raw_parts(data, data_len.min(256));
    G_TPM_MGR.extend_pcr(pcr_idx, d)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_read_pcr(
    index: SigmaU32,
    out_digest: *mut SigmaU8,
) -> SigmaI32 {
    let pcr_idx = match index {
        0 => PcrIndex::Pcr0,
        1 => PcrIndex::Pcr1,
        2 => PcrIndex::Pcr2,
        3 => PcrIndex::Pcr3,
        4 => PcrIndex::Pcr4,
        5 => PcrIndex::Pcr5,
        6 => PcrIndex::Pcr6,
        7 => PcrIndex::Pcr7,
        8 => PcrIndex::Pcr8,
        9 => PcrIndex::Pcr9,
        10 => PcrIndex::Pcr10,
        11 => PcrIndex::Pcr11,
        12 => PcrIndex::Pcr12,
        13 => PcrIndex::Pcr13,
        14 => PcrIndex::Pcr14,
        15 => PcrIndex::Pcr15,
        _ => return -1,
    };
    if let Some(pcr) = G_TPM_MGR.read_pcr(pcr_idx) {
        for i in 0..PCR_DIGEST_LEN {
            core::ptr::write(out_digest.add(i), pcr.digest[i]);
        }
        0
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_generate_attestation(
    pcr_mask: SigmaU32,
    out_report: *mut AttestationReport,
) -> SigmaI32 {
    let report = G_TPM_MGR.generate_attestation(pcr_mask);
    core::ptr::write(out_report, report);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_verify_attestation(
    report: *const AttestationReport,
    aik_pubkey: *const SigmaU8,
    aik_len: SigmaUsize,
) -> SigmaU32 {
    let r = &*report;
    let aik = core::slice::from_raw_parts(aik_pubkey, aik_len.min(32));
    if G_TPM_MGR.verify_attestation(r, aik) { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_seal_key(
    data: *const SigmaU8,
    data_len: SigmaUsize,
    pcr_mask: SigmaU32,
    out_sealed: *mut SealedKey,
) -> SigmaI32 {
    let d = core::slice::from_raw_parts(data, data_len.min(SEALED_KEY_LEN));
    let sealed = G_TPM_MGR.seal_key(d, pcr_mask);
    core::ptr::write(out_sealed, sealed);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_unseal_key(
    sealed: *const SealedKey,
    out_data: *mut SigmaU8,
) -> SigmaI32 {
    let s = &*sealed;
    if let Some(data) = G_TPM_MGR.unseal_key(s) {
        for i in 0..SEALED_KEY_LEN {
            core::ptr::write(out_data.add(i), data[i]);
        }
        0
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_generate_aik() -> SigmaI32 {
    G_TPM_MGR.generate_aik()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_aik_available() -> SigmaU32 {
    if G_TPM_MGR.aik_available() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_set_secure_boot(enabled: SigmaU32) {
    G_TPM_MGR.set_secure_boot(enabled != 0);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_secure_boot() -> SigmaU32 {
    if G_TPM_MGR.secure_boot() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_reset_pcrs() {
    G_TPM_MGR.reset_pcrs()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_tpm_pcr_count() -> SigmaU32 {
    G_TPM_MGR.pcr_count() as SigmaU32
}
