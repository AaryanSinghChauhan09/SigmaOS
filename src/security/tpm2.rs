//! TPM 2.0 Interface
//! Inspired by the TCG TPM 2.0 specification and Linux TPM subsystem.
//! Provides: PCR extend/read, sealed key storage, attestation quotes.
//!
//! References:
//! - TCG TPM 2.0 Library Spec: https://trustedcomputinggroup.org/resource/tpm-library-specification/
//! - Linux drivers/char/tpm/
//! - tpm2-tools: https://github.com/tpm2-software/tpm2-tools

use crate::crypto::entropy;

/// TPM 2.0 PCR bank algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TpmHashAlg {
    Sha1 = 0x0004,
    Sha256 = 0x000B,
    Sha384 = 0x000C,
    Sha512 = 0x000D,
}

impl TpmHashAlg {
    pub fn digest_size(&self) -> usize {
        match self {
            Self::Sha1 => 20,
            Self::Sha256 => 32,
            Self::Sha384 => 48,
            Self::Sha512 => 64,
        }
    }
}

/// TPM 2.0 PCR (Platform Configuration Register)
/// PCR[i] = H(PCR[i] || new_data) — monotonically extending hash chain
#[derive(Debug, Clone)]
pub struct TpmPcr {
    pub index: u8,
    pub alg: TpmHashAlg,
    pub value: Vec<u8>,
}

impl TpmPcr {
    pub fn new(index: u8, alg: TpmHashAlg) -> Self {
        Self {
            index,
            alg,
            value: vec![0u8; alg.digest_size()],
        }
    }

    /// Extend the PCR: PCR = H(PCR || data). Simulated with XorShift mixing.
    pub fn extend(&mut self, data: &[u8]) {
        let digest_size = self.alg.digest_size();
        let mut new_val = self.value.clone();
        // Simulate hash: XOR the PCR with data bytes cyclically
        for (i, &b) in data.iter().enumerate() {
            new_val[i % digest_size] ^= b.rotate_left((i % 8) as u32);
        }
        // Mix with position-dependent constants (simulates SHA-256 diffusion)
        for i in 0..digest_size {
            new_val[i] = new_val[i].wrapping_add(new_val[(i + 1) % digest_size]);
            new_val[i] ^= new_val[(i + 7) % digest_size].rotate_left(3);
        }
        self.value = new_val;
    }

    /// Reset PCR to all zeros (only allowed for PCRs 16-23 in TPM 2.0).
    pub fn reset(&mut self) -> Result<(), &'static str> {
        if self.index < 16 {
            return Err("Cannot reset PCR 0-15 (hardware-only reset)");
        }
        self.value = vec![0u8; self.alg.digest_size()];
        Ok(())
    }
}

/// TPM 2.0 sealed key blob — key material protected by PCR policy
#[derive(Debug, Clone)]
pub struct TpmSealedKey {
    /// The sealed data (key material)
    data: Vec<u8>,
    /// PCR selection mask used when sealing
    pub pcr_mask: u32,
    /// PCR digest snapshot at seal time (used for unseal verification)
    pcr_digest: Vec<u8>,
    /// Whether this key has been unsealed
    pub unsealed: bool,
}

/// TPM 2.0 attestation quote
#[derive(Debug, Clone)]
pub struct TpmQuote {
    /// Nonce provided by verifier (anti-replay)
    pub nonce: Vec<u8>,
    /// PCR values included in the quote
    pub pcr_values: Vec<(u8, Vec<u8>)>,
    /// TPM signature over (nonce || pcr_digest)
    pub signature: Vec<u8>,
    /// Signing key certificate
    pub ak_cert: Vec<u8>,
}

/// TPM 2.0 device interface
pub struct Tpm2Device {
    /// Platform Configuration Registers (24 PCRs per bank)
    pub pcrs: Vec<TpmPcr>,
    /// Active hash algorithm for PCR bank
    pub hash_alg: TpmHashAlg,
    /// Whether the TPM is initialized and available
    pub available: bool,
    /// Endorsement key (simulated — real TPM generates this in hardware)
    ek_public: Vec<u8>,
    /// Attestation key pair (signing key for quotes)
    ak_private: Vec<u8>,
    ak_public: Vec<u8>,
    /// Handles for loaded keys
    pub loaded_keys: u32,
    pub pcr_extends: u64,
    pub seals: u64,
    pub unseals: u64,
    pub quotes: u64,
}

impl Tpm2Device {
    /// Initialize the TPM 2.0 device.
    pub fn init(alg: TpmHashAlg) -> Self {
        let mut ek = vec![0u8; 32];
        let mut ak_priv = vec![0u8; 32];
        let mut ak_pub = vec![0u8; 32];
        entropy::get_entropy_bytes(&mut ek);
        entropy::get_entropy_bytes(&mut ak_priv);
        entropy::get_entropy_bytes(&mut ak_pub);
        let pcrs = (0..24).map(|i| TpmPcr::new(i, alg)).collect();
        Self {
            pcrs,
            hash_alg: alg,
            available: true,
            ek_public: ek,
            ak_private: ak_priv,
            ak_public: ak_pub,
            loaded_keys: 0,
            pcr_extends: 0,
            seals: 0,
            unseals: 0,
            quotes: 0,
        }
    }

    /// Extend a PCR with measurement data.
    pub fn pcr_extend(&mut self, pcr_index: u8, data: &[u8]) -> Result<(), &'static str> {
        if !self.available {
            return Err("TPM not available");
        }
        let pcr = self
            .pcrs
            .get_mut(pcr_index as usize)
            .ok_or("Invalid PCR index")?;
        pcr.extend(data);
        self.pcr_extends += 1;
        Ok(())
    }

    /// Read the current value of a PCR.
    pub fn pcr_read(&self, pcr_index: u8) -> Result<&[u8], &'static str> {
        if !self.available {
            return Err("TPM not available");
        }
        self.pcrs
            .get(pcr_index as usize)
            .map(|p| p.value.as_slice())
            .ok_or("Invalid PCR index")
    }

    /// Seal data to a set of PCR values (TPM2_Seal).
    /// The sealed blob can only be unsealed when those PCRs have the same values.
    pub fn seal(&mut self, data: &[u8], pcr_mask: u32) -> Result<TpmSealedKey, &'static str> {
        if !self.available {
            return Err("TPM not available");
        }
        // Snapshot current PCR values for the policy digest
        let mut pcr_digest = Vec::new();
        for i in 0..24u8 {
            if (pcr_mask >> i) & 1 == 1 {
                pcr_digest.extend_from_slice(&self.pcrs[i as usize].value);
            }
        }
        // Encrypt data with TPM's storage key (simulated XOR with EK)
        let mut sealed_data = data.to_vec();
        for (i, b) in sealed_data.iter_mut().enumerate() {
            *b ^= self.ek_public[i % 32];
        }
        self.seals += 1;
        Ok(TpmSealedKey {
            data: sealed_data,
            pcr_mask,
            pcr_digest,
            unsealed: false,
        })
    }

    /// Unseal data (TPM2_Unseal) — only succeeds if PCRs match the policy.
    pub fn unseal(&mut self, blob: &mut TpmSealedKey) -> Result<Vec<u8>, &'static str> {
        if !self.available {
            return Err("TPM not available");
        }
        // Verify PCR policy
        let mut current_digest = Vec::new();
        for i in 0..24u8 {
            if (blob.pcr_mask >> i) & 1 == 1 {
                current_digest.extend_from_slice(&self.pcrs[i as usize].value);
            }
        }
        if current_digest != blob.pcr_digest {
            return Err("PCR policy mismatch: system state has changed since sealing");
        }
        // Decrypt with storage key
        let mut plaintext = blob.data.clone();
        for (i, b) in plaintext.iter_mut().enumerate() {
            *b ^= self.ek_public[i % 32];
        }
        blob.unsealed = true;
        self.unseals += 1;
        Ok(plaintext)
    }

    /// Generate an attestation quote (TPM2_Quote) for a set of PCRs.
    pub fn quote(&mut self, pcr_mask: u32, nonce: &[u8]) -> Result<TpmQuote, &'static str> {
        if !self.available {
            return Err("TPM not available");
        }
        let mut pcr_values = Vec::new();
        let mut hash_input = nonce.to_vec();
        for i in 0..24u8 {
            if (pcr_mask >> i) & 1 == 1 {
                pcr_values.push((i, self.pcrs[i as usize].value.clone()));
                hash_input.extend_from_slice(&self.pcrs[i as usize].value);
            }
        }
        // Simulate signing with AK private key
        let mut sig = vec![0u8; 64];
        for (i, b) in sig.iter_mut().enumerate() {
            *b = hash_input[i % hash_input.len()] ^ self.ak_private[i % 32];
        }
        self.quotes += 1;
        Ok(TpmQuote {
            nonce: nonce.to_vec(),
            pcr_values,
            signature: sig,
            ak_cert: self.ak_public.clone(),
        })
    }
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_pcr_extend_changes_value() {
        let mut tpm = Tpm2Device::init(TpmHashAlg::Sha256);
        let initial = tpm.pcr_read(0).unwrap().to_vec();
        tpm.pcr_extend(0, b"kernel-image-hash").unwrap();
        let after = tpm.pcr_read(0).unwrap();
        assert_ne!(after, initial.as_slice(), "PCR should change after extend");
    }

    #[test]
    fn test_seal_unseal_roundtrip() {
        let mut tpm = Tpm2Device::init(TpmHashAlg::Sha256);
        tpm.pcr_extend(7, b"secure-boot-state").unwrap();
        let secret = b"my-disk-encryption-key";
        let mut blob = tpm.seal(secret, 1 << 7).unwrap();
        let recovered = tpm.unseal(&mut blob).unwrap();
        assert_eq!(recovered, secret);
        assert!(blob.unsealed);
    }

    #[test]
    fn test_unseal_fails_after_pcr_change() {
        let mut tpm = Tpm2Device::init(TpmHashAlg::Sha256);
        let mut blob = tpm.seal(b"secret", 1 << 0).unwrap();
        tpm.pcr_extend(0, b"unexpected-measurement").unwrap();
        assert!(tpm.unseal(&mut blob).is_err());
    }

    #[test]
    fn test_quote_generation() {
        let mut tpm = Tpm2Device::init(TpmHashAlg::Sha256);
        tpm.pcr_extend(0, b"boot").unwrap();
        let quote = tpm.quote(0b111, b"verifier-nonce").unwrap();
        assert_eq!(quote.nonce, b"verifier-nonce");
        assert!(!quote.pcr_values.is_empty());
        assert_eq!(quote.signature.len(), 64);
    }

    #[test]
    fn test_pcr_reset_restricted() {
        let mut tpm = Tpm2Device::init(TpmHashAlg::Sha256);
        tpm.pcr_extend(0, b"data").unwrap();
        assert!(tpm.pcrs[0].reset().is_err()); // PCR 0-15 cannot be reset by software
        tpm.pcr_extend(16, b"data").unwrap();
        assert!(tpm.pcrs[16].reset().is_ok()); // PCR 16-23 are resettable
    }
}
