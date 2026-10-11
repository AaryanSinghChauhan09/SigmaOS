// SigmaOS ACPI System Locality Information Table (SLIT) Parser
// Extracts multi-socket NUMA distance matrices from firmware ACPI tables.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

pub const SLIT_SIGNATURE: &[u8; 4] = b"SLIT";
pub const LOCAL_NUMA_DISTANCE: u8 = 10;

/// ACPI SLIT Table Header and Locality Distance Matrix
#[derive(Debug, Clone)]
pub struct AcpiSlitTable {
    pub localities_count: usize,
    pub matrix: Vec<u8>, // localities_count * localities_count entries
}

impl AcpiSlitTable {
    /// Parse raw ACPI SLIT table from physical memory
    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 44 {
            return Err("SLIT table too small for header");
        }

        if &data[0..4] != SLIT_SIGNATURE {
            return Err("Invalid SLIT table signature");
        }

        // Validate table checksum
        let sum: u8 = data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        if sum != 0 {
            return Err("SLIT table checksum verification failed");
        }

        // Offset 36 contains number of localities as 64-bit integer
        let localities_count = u64::from_le_bytes(
            data[36..44]
                .try_into()
                .map_err(|_| "Failed to parse localities count")?,
        ) as usize;

        let matrix_size = localities_count * localities_count;
        if data.len() < 44 + matrix_size {
            return Err("SLIT table payload truncated");
        }

        let matrix = data[44..44 + matrix_size].to_vec();

        Ok(Self {
            localities_count,
            matrix,
        })
    }

    /// Construct a synthetic/default symmetrical NUMA matrix (for fallback or testing)
    pub fn new_default_symmetrical(nodes: usize, remote_distance: u8) -> Self {
        let mut matrix = vec![0u8; nodes * nodes];
        for from in 0..nodes {
            for to in 0..nodes {
                matrix[from * nodes + to] = if from == to {
                    LOCAL_NUMA_DISTANCE
                } else {
                    remote_distance
                };
            }
        }
        Self {
            localities_count: nodes,
            matrix,
        }
    }

    /// Query distance between two NUMA localities (node IDs)
    pub fn get_distance(&self, from_node: usize, to_node: usize) -> Option<u8> {
        if from_node < self.localities_count && to_node < self.localities_count {
            Some(self.matrix[from_node * self.localities_count + to_node])
        } else {
            None
        }
    }

    /// Find closest NUMA neighbor for a node
    pub fn find_closest_neighbor(&self, node: usize) -> Option<(usize, u8)> {
        if node >= self.localities_count || self.localities_count <= 1 {
            return None;
        }

        let mut closest_node = 0;
        let mut min_distance = u8::MAX;

        for target in 0..self.localities_count {
            if target != node {
                if let Some(dist) = self.get_distance(node, target) {
                    if dist < min_distance {
                        min_distance = dist;
                        closest_node = target;
                    }
                }
            }
        }

        if min_distance < u8::MAX {
            Some((closest_node, min_distance))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acpi_slit_distances() {
        let slit = AcpiSlitTable::new_default_symmetrical(4, 21);
        assert_eq!(slit.get_distance(0, 0), Some(10));
        assert_eq!(slit.get_distance(0, 1), Some(21));
        assert_eq!(slit.get_distance(2, 3), Some(21));
        assert_eq!(slit.find_closest_neighbor(0), Some((1, 21)));
    }
}
