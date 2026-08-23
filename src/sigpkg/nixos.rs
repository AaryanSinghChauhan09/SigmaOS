// NixOS Declarative Profile Generations & Content-Addressed Store Manager
// Inspired by NixOS /nix/store generations, profiles, atomic rollbacks, and hash verification.

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Content-addressed Nix store package entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NixStorePackage {
    pub hash: String,
    pub name: String,
    pub version: String,
    pub store_path: String,
    pub dependencies: Vec<String>,
}

impl NixStorePackage {
    pub fn new(hash: &str, name: &str, version: &str) -> Self {
        let store_path = format!("/nix/store/{}-{}-{}", hash, name, version);
        Self {
            hash: hash.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            store_path,
            dependencies: Vec::new(),
        }
    }
}

/// NixOS Declarative Generation Profile
#[derive(Debug, Clone)]
pub struct NixGeneration {
    pub generation_id: u32,
    pub profile_name: String,
    pub packages: BTreeMap<String, NixStorePackage>,
}

impl NixGeneration {
    pub fn new(id: u32, profile_name: &str) -> Self {
        Self {
            generation_id: id,
            profile_name: profile_name.to_string(),
            packages: BTreeMap::new(),
        }
    }
}

/// NixOS Profile Store Manager supporting atomic rollbacks
pub struct NixProfileStore {
    pub active_generation: u32,
    pub generations: BTreeMap<u32, NixGeneration>,
    pub next_generation_id: u32,
}

impl NixProfileStore {
    pub fn new() -> Self {
        let mut store = Self {
            active_generation: 1,
            generations: BTreeMap::new(),
            next_generation_id: 1,
        };
        let initial_gen = NixGeneration::new(1, "default");
        store.generations.insert(1, initial_gen);
        store
    }

    pub fn create_generation(&mut self, profile_name: &str) -> u32 {
        self.next_generation_id += 1;
        let new_id = self.next_generation_id;
        let mut new_gen = NixGeneration::new(new_id, profile_name);

        // Copy packages from active generation
        if let Some(active_gen) = self.generations.get(&self.active_generation) {
            new_gen.packages = active_gen.packages.clone();
        }

        self.generations.insert(new_id, new_gen);
        self.active_generation = new_id;
        new_id
    }

    pub fn add_package_to_active(&mut self, pkg: NixStorePackage) -> Result<(), &'static str> {
        let gen = self.generations.get_mut(&self.active_generation).ok_or("Active generation not found")?;
        gen.packages.insert(pkg.name.clone(), pkg);
        Ok(())
    }

    pub fn rollback_generation(&mut self, target_gen_id: u32) -> Result<(), &'static str> {
        if self.generations.contains_key(&target_gen_id) {
            self.active_generation = target_gen_id;
            Ok(())
        } else {
            Err("Target generation ID not found in Nix store")
        }
    }
}

impl Default for NixProfileStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nixos_store_and_rollback() {
        let mut store = NixProfileStore::new();
        let pkg1 = NixStorePackage::new("a1b2c3d4", "bash", "5.2");
        store.add_package_to_active(pkg1).unwrap();

        let gen1_id = store.active_generation;

        // Create generation 2 and add second package
        let gen2_id = store.create_generation("developer-env");
        let pkg2 = NixStorePackage::new("e5f6g7h8", "git", "2.40");
        store.add_package_to_active(pkg2).unwrap();

        assert_eq!(store.generations.get(&gen2_id).unwrap().packages.len(), 2);

        // Rollback to generation 1
        assert!(store.rollback_generation(gen1_id).is_ok());
        assert_eq!(store.active_generation, gen1_id);
        assert_eq!(store.generations.get(&gen1_id).unwrap().packages.len(), 1);
    }
}
