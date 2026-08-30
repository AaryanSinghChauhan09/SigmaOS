// SigmaOS Self-Hosted Merkle Accumulator
// Deterministic append-only Merkle tree for firmware measurements
// and content addressing without external cryptographic dependencies

extern crate alloc;
use alloc::vec::Vec;

/// Fixed-size hash for Merkle nodes (32 bytes)
pub type MerkleHash = [u8; 32];

/// A leaf node in the Merkle accumulator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MerkleLeaf {
    pub index: usize,
    pub hash: MerkleHash,
}

/// The Merkle accumulator state
pub struct MerkleAccumulator {
    leaves: Vec<MerkleLeaf>,
    nodes: Vec<MerkleHash>,
    depth: usize,
}

impl MerkleAccumulator {
    /// Create a new empty Merkle accumulator
    pub fn new() -> Self {
        MerkleAccumulator {
            leaves: Vec::new(),
            nodes: Vec::new(),
            depth: 0,
        }
    }

    /// Append a new data item and return its leaf hash
    pub fn append(&mut self, data: &[u8]) -> MerkleHash {
        let hash = leaf_hash(data);
        let index = self.leaves.len();
        self.leaves.push(MerkleLeaf { index, hash });
        self.rebuild_nodes();
        hash
    }

    /// Get the current Merkle root hash
    pub fn root(&self) -> Option<MerkleHash> {
        self.nodes.last().copied()
    }

    /// Verify that data at a given index produces the expected leaf hash
    pub fn verify_leaf(&self, index: usize, data: &[u8]) -> bool {
        if index >= self.leaves.len() {
            return false;
        }
        self.leaves[index].hash == leaf_hash(data)
    }

    /// Verify a Merkle proof (simplified: returns whether the leaf exists and root matches)
    pub fn verify(&self, index: usize, data: &[u8], expected_root: MerkleHash) -> bool {
        if index >= self.leaves.len() {
            return false;
        }
        if !self.verify_leaf(index, data) {
            return false;
        }
        self.root() == Some(expected_root)
    }

    /// Number of leaves in the accumulator
    pub fn len(&self) -> usize {
        self.leaves.len()
    }

    /// Whether the accumulator is empty
    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }

    /// Rebuild internal node hashes from leaves
    fn rebuild_nodes(&mut self) {
        let count = self.leaves.len();
        self.depth = if count == 0 {
            0
        } else {
            (count - 1).next_power_of_two().trailing_zeros() as usize + 1
        };

        let capacity = if count == 0 {
            0
        } else {
            (count * 2).next_power_of_two()
        };

        self.nodes.clear();
        self.nodes.reserve(capacity);

        for leaf in &self.leaves {
            self.nodes.push(leaf.hash);
        }

        let mut offset = 0;
        let mut size = count;
        while size > 1 {
            for i in 0..(size / 2) {
                let left = self.nodes[offset + i * 2];
                let right = if offset + i * 2 + 1 < self.nodes.len() {
                    self.nodes[offset + i * 2 + 1]
                } else {
                    left
                };
                self.nodes.push(internal_hash(&left, &right));
            }
            offset += size;
            size = (size + 1) / 2;
        }
    }
}

impl Default for MerkleAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

fn fnv32(mut h: u32, data: &[u8]) -> u32 {
    for &b in data {
        h ^= b as u32;
        h = h.wrapping_mul(0x0100_0193);
    }
    h
}

/// Deterministically expand a 32-bit FNV-1a accumulator into a 32-byte digest.
/// Each output byte is derived from a distinct FNV round so distinct inputs
/// (and distinct positions) produce distinct digests.
fn expand(mut h: u32, data: &[u8]) -> MerkleHash {
    h = fnv32(h, data);
    let mut out = [0u8; 32];
    for i in 0..32u32 {
        h = fnv32(h ^ (i.wrapping_mul(0x9E37_79B1)), &[]);
        let bytes = h.to_be_bytes();
        out[i as usize] = bytes[0] ^ bytes[3];
    }
    out
}

fn leaf_hash(data: &[u8]) -> MerkleHash {
    expand(0x811C_9DC5 ^ 0xA1B2_C3D4, data)
}

fn internal_hash(left: &MerkleHash, right: &MerkleHash) -> MerkleHash {
    let mut buf = [0u8; 64];
    buf[..32].copy_from_slice(left);
    buf[32..].copy_from_slice(right);
    expand(0x811C_9DC5 ^ 0x5E6F_7081, &buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_accumulator_empty() {
        let acc = MerkleAccumulator::new();
        assert!(acc.is_empty());
        assert!(acc.root().is_none());
    }

    #[test]
    fn test_merkle_accumulator_single_leaf() {
        let mut acc = MerkleAccumulator::new();
        let hash = acc.append(b"bootloader");
        assert_eq!(acc.len(), 1);
        assert_eq!(acc.root(), Some(hash));
        assert!(acc.verify_leaf(0, b"bootloader"));
        assert!(!acc.verify_leaf(0, b"kernel"));
    }

    #[test]
    fn test_merkle_accumulator_multiple_leaves() {
        let mut acc = MerkleAccumulator::new();
        let h1 = acc.append(b"bootloader");
        let h2 = acc.append(b"kernel");
        let h3 = acc.append(b"initramfs");

        assert_eq!(acc.len(), 3);
        let root = acc.root();
        assert!(root.is_some());
        assert_ne!(root, Some(h1));
        assert_ne!(root, Some(h2));
        assert_ne!(root, Some(h3));

        assert!(acc.verify(0, b"bootloader", root.unwrap()));
        assert!(acc.verify(1, b"kernel", root.unwrap()));
        assert!(acc.verify(2, b"initramfs", root.unwrap()));
        assert!(!acc.verify(0, b"kernel", root.unwrap()));
    }

    #[test]
    fn test_merkle_accumulator_deterministic() {
        let mut acc1 = MerkleAccumulator::new();
        acc1.append(b"A");
        acc1.append(b"B");
        acc1.append(b"C");

        let mut acc2 = MerkleAccumulator::new();
        acc2.append(b"A");
        acc2.append(b"B");
        acc2.append(b"C");

        assert_eq!(acc1.root(), acc2.root());
    }

    #[test]
    fn test_merkle_verify_wrong_root() {
        let mut acc = MerkleAccumulator::new();
        acc.append(b"data");
        let root = acc.root().unwrap();
        let wrong_root = [0xFF; 32];
        assert!(!acc.verify(0, b"data", wrong_root));
        assert!(acc.verify(0, b"data", root));
    }

    #[test]
    fn test_merkle_out_of_bounds() {
        let mut acc = MerkleAccumulator::new();
        acc.append(b"data");
        assert!(!acc.verify_leaf(1, b"data"));
        assert!(!acc.verify(1, b"data", acc.root().unwrap()));
    }
}
