// Immutable Store System for SigmaOS
// Implements /sigma/store inspired by Nix's /nix/store
// Provides immutable package storage with content-addressable paths

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorePath {
    pub hash: String,
    pub name: String,
    pub full_path: PathBuf,
    pub references: Vec<String>,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreEntry {
    pub path: StorePath,
    pub metadata: EntryMetadata,
    pub registration_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntryMetadata {
    pub version: String,
    pub description: String,
    pub license: String,
    pub dependencies: Vec<String>,
    pub build_time: Option<i64>,
}

pub struct ImmutableStore {
    store_root: PathBuf,
    store_db: HashMap<String, StoreEntry>,
}

impl ImmutableStore {
    pub fn new(store_root: PathBuf) -> Result<Self, std::io::Error> {
        let store_path = store_root.join("store");
        fs::create_dir_all(&store_path)?;
        
        let store_db = Self::load_store_db(&store_root)?;
        
        Ok(ImmutableStore {
            store_root,
            store_db,
        })
    }

    /// Add a package to the immutable store
    pub fn add_package(&mut self, source_path: &Path, package_name: &str) -> Result<StorePath, std::io::Error> {
        // Calculate content hash
        let hash = self.calculate_hash(source_path)?;
        
        // Create store path
        let store_path = StorePath {
            hash: hash.clone(),
            name: package_name.to_string(),
            full_path: self.store_root.join("store").join(format!("{}-{}", hash, package_name)),
            references: Vec::new(),
            size: self.calculate_size(source_path)?,
        };
        
        // Copy to store (immutable)
        fs::create_dir_all(&store_path.full_path)?;
        self.copy_directory(source_path, &store_path.full_path)?;
        
        // Make immutable (chmod -R a-w)
        self.make_immutable(&store_path.full_path)?;
        
        // Register in database
        let entry = StoreEntry {
            path: store_path.clone(),
            metadata: EntryMetadata {
                version: "1.0.0".to_string(),
                description: format!("Package: {}", package_name),
                license: "MIT".to_string(),
                dependencies: Vec::new(),
                build_time: Some(chrono::Utc::now().timestamp()),
            },
            registration_time: chrono::Utc::now().timestamp(),
        };
        
        self.store_db.insert(hash.clone(), entry);
        self.save_store_db()?;
        
        Ok(store_path)
    }

    /// Get a package from the store by hash
    pub fn get_package(&self, hash: &str) -> Option<&StoreEntry> {
        self.store_db.get(hash)
    }

    /// Create a garbage collection root
    pub fn add_gc_root(&self, path: &Path) -> Result<(), std::io::Error> {
        let gc_roots_dir = self.store_root.join("gcroots");
        fs::create_dir_all(&gc_roots_dir)?;
        
        let root_name = path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("root");
        
        let root_path = gc_roots_dir.join(root_name);
        fs::write(&root_path, path.to_string_lossy().as_bytes())?;
        
        Ok(())
    }

    /// Perform garbage collection (remove unreferenced store paths)
    pub fn garbage_collect(&mut self) -> Result<usize, std::io::Error> {
        let mut to_remove = Vec::new();
        let referenced = self.collect_referenced_paths()?;
        
        for (hash, entry) in &self.store_db {
            if !referenced.contains(&entry.path.full_path) {
                to_remove.push(hash.clone());
            }
        }
        
        for hash in &to_remove {
            if let Some(entry) = self.store_db.remove(hash) {
                // Make mutable before removal
                self.make_mutable(&entry.path.full_path)?;
                fs::remove_dir_all(&entry.path.full_path)?;
            }
        }
        
        self.save_store_db()?;
        Ok(to_remove.len())
    }

    /// Query store for packages matching a pattern
    pub fn query(&self, pattern: &str) -> Vec<&StoreEntry> {
        self.store_db.values()
            .filter(|entry| entry.path.name.contains(pattern))
            .collect()
    }

    /// Verify store integrity
    pub fn verify_integrity(&self) -> Result<Vec<String>, std::io::Error> {
        let mut corrupted = Vec::new();
        
        for (hash, entry) in &self.store_db {
            if !entry.path.full_path.exists() {
                corrupted.push(format!("Missing: {}", hash));
                continue;
            }
            
            let calculated_hash = self.calculate_hash(&entry.path.full_path)?;
            if calculated_hash != *hash {
                corrupted.push(format!("Corrupted: {} (expected {}, got {})", 
                    entry.path.name, hash, calculated_hash));
            }
        }
        
        Ok(corrupted)
    }

    /// Optimize store by deduplicating identical files
    pub fn optimize(&mut self) -> Result<usize, std::io::Error> {
        let mut dedup_count = 0;
        let mut file_hashes: HashMap<String, Vec<PathBuf>> = HashMap::new();
        
        // Collect file hashes
        for entry in self.store_db.values() {
            if let Ok(files) = self.collect_files(&entry.path.full_path) {
                for file in files {
                    if let Ok(hash) = self.calculate_file_hash(&file) {
                        file_hashes.entry(hash)
                            .or_insert_with(Vec::new)
                            .push(file);
                    }
                }
            }
        }
        
        // Deduplicate
        for (hash, files) in file_hashes {
            if files.len() > 1 {
                let canonical = files.first().unwrap();
                for duplicate in &files[1..] {
                    fs::remove_file(duplicate)?;
                    self.create_hard_link(canonical, duplicate)?;
                    dedup_count += 1;
                }
            }
        }
        
        Ok(dedup_count)
    }

    fn calculate_hash(&self, path: &Path) -> Result<String, std::io::Error> {
        let mut hasher = Sha256::new();
        
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let file_hash = self.calculate_file_hash(&entry.path())?;
                hasher.update(file_hash.as_bytes());
                hasher.update(entry.file_name().to_string_lossy().as_bytes());
            }
        } else {
            let file_hash = self.calculate_file_hash(path)?;
            hasher.update(file_hash.as_bytes());
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn calculate_file_hash(&self, path: &Path) -> Result<String, std::io::Error> {
        let contents = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn calculate_size(&self, path: &Path) -> Result<u64, std::io::Error> {
        let mut total = 0u64;
        
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    total += self.calculate_size(&entry.path())?;
                } else {
                    total += entry.metadata()?.len();
                }
            }
        } else {
            total = fs::metadata(path)?.len();
        }
        
        Ok(total)
    }

    fn make_immutable(&self, path: &Path) -> Result<(), std::io::Error> {
        // On Unix systems, we'd use chmod to remove write permissions
        // For now, this is a placeholder
        Ok(())
    }

    fn make_mutable(&self, path: &Path) -> Result<(), std::io::Error> {
        // Make path mutable for garbage collection
        Ok(())
    }

    fn copy_directory(&self, src: &Path, dst: &Path) -> Result<(), std::io::Error> {
        let mut cmd = Command::new("cp");
        cmd.arg("-r")
           .arg(src)
           .arg(dst);
        cmd.status()?;
        Ok(())
    }

    fn create_hard_link(&self, src: &Path, dst: &Path) -> Result<(), std::io::Error> {
        fs::hard_link(src, dst)?;
        Ok(())
    }

    fn collect_files(&self, path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut files = Vec::new();
        
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_file() {
                files.push(entry.path());
            } else if entry.path().is_dir() {
                files.extend(self.collect_files(&entry.path())?);
            }
        }
        
        Ok(files)
    }

    fn collect_referenced_paths(&self) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut referenced = Vec::new();
        let gc_roots_dir = self.store_root.join("gcroots");
        
        if gc_roots_dir.exists() {
            for entry in fs::read_dir(&gc_roots_dir)? {
                let entry = entry?;
                let root_path = fs::read_to_string(entry.path())?;
                referenced.push(PathBuf::from(root_path.trim()));
            }
        }
        
        // Also add current profile
        let current_profile = self.store_root.join("profiles").join("current");
        if current_profile.exists() {
            referenced.push(current_profile);
        }
        
        Ok(referenced)
    }

    fn load_store_db(store_root: &Path) -> Result<HashMap<String, StoreEntry>, std::io::Error> {
        let db_path = store_root.join("store").join("db.json");
        
        if db_path.exists() {
            let content = fs::read_to_string(&db_path)?;
            let db: HashMap<String, StoreEntry> = serde_json::from_str(&content)?;
            Ok(db)
        } else {
            Ok(HashMap::new())
        }
    }

    fn save_store_db(&self) -> Result<(), std::io::Error> {
        let db_path = self.store_root.join("store").join("db.json");
        let content = serde_json::to_string_pretty(&self.store_db)?;
        fs::write(&db_path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_store_creation() {
        let temp_dir = tempdir().unwrap();
        let store_root = temp_dir.path().to_path_buf();
        
        let store = ImmutableStore::new(store_root).unwrap();
        assert_eq!(store.store_db.len(), 0);
    }

    #[test]
    fn test_hash_calculation() {
        let temp_dir = tempdir().unwrap();
        let store_root = temp_dir.path().to_path_buf();
        
        let store = ImmutableStore::new(store_root).unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();
        
        let hash = store.calculate_hash(&test_file).unwrap();
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA256 hex length
    }
}
