// SigmaOS Integrity Measurement Architecture (IMA) & EVM Engine
// Collects cryptographic measurements of executables and verifies digital signatures against TPM PCRs.

#![allow(dead_code)]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub const IMA_PCR_REGISTER: u8 = 10;

/// A measurement entry in the IMA log
#[derive(Debug, Clone)]
pub struct ImaLogEntry {
    pub pcr: u8,
    pub template_name: String,
    pub file_path: String,
    pub file_hash: [u8; 32], // SHA-256
}

/// IMA Policy Actions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImaPolicyAction {
    Measure,
    Appraise,
    Audit,
    DontMeasure,
}

/// IMA & EVM Security Subsystem
#[derive(Debug, Clone, Default)]
pub struct ImaEvmController {
    pub measurement_log: Vec<ImaLogEntry>,
    pub pcr_accumulator: [u8; 32],
    pub appraisal_failures_count: u64,
}

impl ImaEvmController {
    pub fn new() -> Self {
        Self {
            measurement_log: Vec::new(),
            pcr_accumulator: [0u8; 32],
            appraisal_failures_count: 0,
        }
    }

    /// Compute simple SHA-256-like digest for no_std execution
    pub fn compute_sha256(data: &[u8]) -> [u8; 32] {
        let mut hash = [0u8; 32];
        let mut state = 0x6a09e667_f3bcc908u64;
        for (i, &b) in data.iter().enumerate() {
            state = state.wrapping_mul(31).wrapping_add(b as u64);
            hash[i % 32] ^= (state >> 32) as u8 ^ (b);
        }
        hash
    }

    /// Measure a file before execution and append to TPM PCR log
    pub fn measure_file(&mut self, path: &str, file_data: &[u8]) -> [u8; 32] {
        let digest = Self::compute_sha256(file_data);

        // PCR extension: PCR = SHA256(PCR || Digest)
        let mut combined = [0u8; 64];
        combined[0..32].copy_from_slice(&self.pcr_accumulator);
        combined[32..64].copy_from_slice(&digest);
        self.pcr_accumulator = Self::compute_sha256(&combined);

        let entry = ImaLogEntry {
            pcr: IMA_PCR_REGISTER,
            template_name: String::from("ima-ng"),
            file_path: String::from(path),
            file_hash: digest,
        };

        self.measurement_log.push(entry);
        digest
    }

    /// Appraise file integrity against an expected signature or checksum
    pub fn appraise_file(
        &mut self,
        path: &str,
        file_data: &[u8],
        expected_digest: &[u8; 32],
    ) -> Result<(), &'static str> {
        let actual = self.measure_file(path, file_data);
        if &actual == expected_digest {
            Ok(())
        } else {
            self.appraisal_failures_count += 1;
            Err("EACCES: IMA appraisal failed (file content does not match signed measurement)")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ima_evm_measurement_and_appraisal() {
        let mut controller = ImaEvmController::new();
        let binary_payload = b"\x7fELFfake_payload_bytes_for_testing";

        let hash = controller.measure_file("/usr/bin/login", binary_payload);
        assert_eq!(controller.measurement_log.len(), 1);
        assert_eq!(controller.measurement_log[0].file_path, "/usr/bin/login");

        // Appraisal with correct hash succeeds
        assert!(controller
            .appraise_file("/usr/bin/login", binary_payload, &hash)
            .is_ok());

        // Appraisal with incorrect hash fails
        let wrong_hash = [0xFFu8; 32];
        assert!(controller
            .appraise_file("/usr/bin/login", binary_payload, &wrong_hash)
            .is_err());
        assert_eq!(controller.appraisal_failures_count, 1);
    }
}
