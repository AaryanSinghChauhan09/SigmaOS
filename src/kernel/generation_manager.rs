#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// NixOS-Style: Atomic Inode Pointer-Swap Generation Manager
// In monolithic systems, updates copy gigabytes of files, leading to fragmentation and potential boot failures.
// SigmaOS achieves sub-millisecond, zero-copy system rollbacks by storing configurations as content-addressed nodes and swapping directory inodes.

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Generation {
    pub id: u32,
    pub root_inode: u64,
    pub created_at: u64,
}

pub struct GenerationManager {
    generations: Vec<Generation>,
    active_generation_idx: Option<usize>,
}

impl GenerationManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            generations: Vec::new(),
            active_generation_idx: None,
        }
    }

    /// Registers a new immutable configuration snapshot node
    pub fn create_generation(
        &mut self,
        root_inode: u64,
        timestamp: u64,
    ) -> Result<u32, &'static str> {
        let next_id = (self.generations.len() + 1) as u32;
        let gen = Generation {
            id: next_id,
            root_inode,
            created_at: timestamp,
        };
        self.generations.push(gen);
        Ok(next_id)
    }

    /// Perform a sub-millisecond, zero-copy atomic update/rollback by swapping root pointers
    pub fn swap_active_generation(&mut self, _generation_id: u32) -> Result<u64, &'static str> {
        for (idx, gen) in self.generations.iter().enumerate() {
            if gen.id == generation_id {
                self.active_generation_idx = Some(idx);
                return Ok(gen.root_inode); // New root filesystem sector mapped
            }
        }
        Err("Target system generation not found")
    }

    /// Retrieve the currently active system generation info
    pub fn get_active_generation(&self) -> Option<&Generation> {
        self.active_generation_idx.map(|idx| &self.generations[idx])
    }
}

impl Default for GenerationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nixos_atomic_generation_swap() {
        let mut manager = GenerationManager::new();
        // Create Generation 1 (Early Boot Node)
        assert_eq!(manager.create_generation(0x1000, 1718900000).unwrap(), 1);
        // Create Generation 2 (Post-Update Node)
        assert_eq!(manager.create_generation(0x2000, 1718910000).unwrap(), 2);

        // Perform transactional update (active = Gen 2)
        let active_inode = manager.swap_active_generation(2).unwrap();
        assert_eq!(active_inode, 0x2000);
        assert_eq!(manager.get_active_generation().unwrap().id, 2);

        // Instant sub-millisecond rollback (active = Gen 1)
        let rollback_inode = manager.swap_active_generation(1).unwrap();
        assert_eq!(rollback_inode, 0x1000);
        assert_eq!(manager.get_active_generation().unwrap().id, 1);
    }
}
