#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Index manager for file and content indexing
/// Inspired by Linux file indexing tools (locate, mlocate, updatedb)
#[derive(Debug, Clone)]
pub struct IndexManager {
    indexes: Vec<Index>,
}

/// An index of files or content
#[derive(Debug, Clone)]
pub struct Index {
    id: String,
    name: String,
    path: String,
    index_type: IndexType,
    state: IndexState,
    entry_count: usize,
}

/// Type of index
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexType {
    FileIndex,
    ContentIndex,
    MetadataIndex,
    HybridIndex,
}

/// State of an index
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexState {
    Building,
    Ready,
    Updating,
    Corrupted,
    Disabled,
}

/// Index entry
#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub path: String,
    pub file_type: FileType,
    pub size: u64,
    pub modified: u64,
    pub permissions: u32,
}

/// File type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Fifo,
    Socket,
}

impl IndexManager {
    pub fn new() -> Self {
        Self {
            indexes: Vec::new(),
        }
    }

    /// Create a new index
    pub fn create_index(&mut self, id: String, name: String, path: String, index_type: IndexType) -> Result<String, String> {
        if self.indexes.iter().any(|i| i.id == id) {
            return Err(format!("Index with id {} already exists", id));
        }
        
        let index = Index {
            id: id.clone(),
            name,
            path,
            index_type,
            state: IndexState::Building,
            entry_count: 0,
        };
        self.indexes.push(index);
        Ok(id)
    }

    /// Remove an index by ID
    pub fn remove_index(&mut self, id: &str) -> Result<(), String> {
        let pos = self.indexes.iter().position(|i| i.id == id)
            .ok_or_else(|| format!("Index with id {} not found", id))?;
        self.indexes.remove(pos);
        Ok(())
    }

    /// Update an index
    pub fn update_index(&mut self, id: &str) -> Result<(), String> {
        let index = self.indexes.iter_mut().find(|i| i.id == id)
            .ok_or_else(|| format!("Index with id {} not found", id))?;
        index.state = IndexState::Updating;
        Ok(())
    }

    /// Mark index as ready after update
    pub fn mark_index_ready(&mut self, id: &str) -> Result<(), String> {
        let index = self.indexes.iter_mut().find(|i| i.id == id)
            .ok_or_else(|| format!("Index with id {} not found", id))?;
        index.state = IndexState::Ready;
        Ok(())
    }

    /// Disable an index
    pub fn disable_index(&mut self, id: &str) -> Result<(), String> {
        let index = self.indexes.iter_mut().find(|i| i.id == id)
            .ok_or_else(|| format!("Index with id {} not found", id))?;
        index.state = IndexState::Disabled;
        Ok(())
    }

    /// Enable an index
    pub fn enable_index(&mut self, id: &str) -> Result<(), String> {
        let index = self.indexes.iter_mut().find(|i| i.id == id)
            .ok_or_else(|| format!("Index with id {} not found", id))?;
        index.state = IndexState::Building;
        Ok(())
    }

    /// Set entry count for an index
    pub fn set_entry_count(&mut self, id: &str, count: usize) -> Result<(), String> {
        let index = self.indexes.iter_mut().find(|i| i.id == id)
            .ok_or_else(|| format!("Index with id {} not found", id))?;
        index.entry_count = count;
        Ok(())
    }

    /// Get all indexes
    pub fn get_indexes(&self) -> &[Index] {
        &self.indexes
    }

    /// Get ready indexes
    pub fn get_ready_indexes(&self) -> Vec<&Index> {
        self.indexes.iter().filter(|i| i.state == IndexState::Ready).collect()
    }

    /// Get index by ID
    pub fn get_index(&self, id: &str) -> Option<&Index> {
        self.indexes.iter().find(|i| i.id == id)
    }

    /// Get indexes by type
    pub fn get_indexes_by_type(&self, index_type: IndexType) -> Vec<&Index> {
        self.indexes.iter().filter(|i| i.index_type == index_type).collect()
    }
}

impl Default for IndexManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_index() {
        let mut manager = IndexManager::new();
        let result = manager.create_index("idx1".to_string(), "Main Index".to_string(), "/".to_string(), IndexType::FileIndex);
        assert!(result.is_ok());
        assert_eq!(manager.get_indexes().len(), 1);
    }

    #[test]
    fn test_duplicate_index() {
        let mut manager = IndexManager::new();
        manager.create_index("idx1".to_string(), "Main Index".to_string(), "/".to_string(), IndexType::FileIndex).unwrap();
        let result = manager.create_index("idx1".to_string(), "Other Index".to_string(), "/home".to_string(), IndexType::ContentIndex);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_index() {
        let mut manager = IndexManager::new();
        manager.create_index("idx1".to_string(), "Main Index".to_string(), "/".to_string(), IndexType::FileIndex).unwrap();
        let result = manager.remove_index("idx1");
        assert!(result.is_ok());
        assert_eq!(manager.get_indexes().len(), 0);
    }

    #[test]
    fn test_update_index() {
        let mut manager = IndexManager::new();
        manager.create_index("idx1".to_string(), "Main Index".to_string(), "/".to_string(), IndexType::FileIndex).unwrap();
        
        manager.update_index("idx1").unwrap();
        assert_eq!(manager.get_index("idx1").unwrap().state, IndexState::Updating);
        
        manager.mark_index_ready("idx1").unwrap();
        assert_eq!(manager.get_index("idx1").unwrap().state, IndexState::Ready);
    }

    #[test]
    fn test_get_ready_indexes() {
        let mut manager = IndexManager::new();
        manager.create_index("idx1".to_string(), "Main Index".to_string(), "/".to_string(), IndexType::FileIndex).unwrap();
        manager.create_index("idx2".to_string(), "Content Index".to_string(), "/home".to_string(), IndexType::ContentIndex).unwrap();
        
        manager.mark_index_ready("idx1").unwrap();
        
        let ready = manager.get_ready_indexes();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, "idx1");
    }
}
