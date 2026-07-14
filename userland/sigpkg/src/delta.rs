// sigpkg delta: Binary Delta Patch Application
// Replaces full downloads with efficient diffs for OS updates

use crate::SigpkgError;

/// A binary patch for a package or CAS object.
pub struct DeltaPatch {
    pub target_hash: [u8; 32],
    pub source_hash: [u8; 32],
    pub compressed_diff: Vec<u8>,
}

pub struct DeltaEngine;

impl DeltaEngine {
    /// Apply a binary delta patch to a source buffer to produce the target buffer.
    /// This displaces things like ostree's static deltas or rpm-ostree payloads.
    pub fn apply_patch(source: &[u8], patch: &DeltaPatch) -> Result<Vec<u8>, SigpkgError> {
        // In a real implementation, this would use a native rust port of bsdiff or zstd-dict.
        // For the stub, we just pretend the patch decompresses directly into the target.
        if source.is_empty() {
            return Err(SigpkgError::InvalidMetadata);
        }

        let mut target = Vec::with_capacity(source.len() + patch.compressed_diff.len());
        target.extend_from_slice(source);
        // Pretend to apply diff
        target.extend_from_slice(&patch.compressed_diff);

        Ok(target)
    }

    /// Calculate the savings of a patch vs full download.
    pub fn calculate_savings(full_size: usize, patch_size: usize) -> f64 {
        if full_size == 0 {
            return 0.0;
        }
        let savings = full_size.saturating_sub(patch_size);
        (savings as f64 / full_size as f64) * 100.0
    }
}
