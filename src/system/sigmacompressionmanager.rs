#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Compression manager for file compression and decompression
/// Inspired by Linux compression tools (gzip, bzip2, xz, zstd)
#[derive(Debug, Clone)]
pub struct CompressionManager {
    operations: Vec<CompressionOperation>,
}

/// A compression operation
#[derive(Debug, Clone)]
pub struct CompressionOperation {
    id: String,
    source_path: String,
    target_path: String,
    algorithm: CompressionAlgorithm,
    state: OperationState,
    progress: u8,
}

/// Compression algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    Gzip,
    Bzip2,
    Xz,
    Zstd,
    Lzma,
    Lz4,
}

/// State of an operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl CompressionManager {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Start a compression operation
    pub fn compress(&mut self, id: String, source_path: String, target_path: String, algorithm: CompressionAlgorithm) -> Result<String, String> {
        if self.operations.iter().any(|o| o.id == id) {
            return Err(format!("Operation with id {} already exists", id));
        }
        
        let operation = CompressionOperation {
            id: id.clone(),
            source_path,
            target_path,
            algorithm,
            state: OperationState::Pending,
            progress: 0,
        };
        self.operations.push(operation);
        Ok(id)
    }

    /// Start a decompression operation
    pub fn decompress(&mut self, id: String, source_path: String, target_path: String, algorithm: CompressionAlgorithm) -> Result<String, String> {
        if self.operations.iter().any(|o| o.id == id) {
            return Err(format!("Operation with id {} already exists", id));
        }
        
        let operation = CompressionOperation {
            id: id.clone(),
            source_path,
            target_path,
            algorithm,
            state: OperationState::Pending,
            progress: 0,
        };
        self.operations.push(operation);
        Ok(id)
    }

    /// Cancel an operation
    pub fn cancel_operation(&mut self, id: &str) -> Result<(), String> {
        let operation = self.operations.iter_mut().find(|o| o.id == id)
            .ok_or_else(|| format!("Operation with id {} not found", id))?;
        operation.state = OperationState::Cancelled;
        Ok(())
    }

    /// Remove an operation
    pub fn remove_operation(&mut self, id: &str) -> Result<(), String> {
        let pos = self.operations.iter().position(|o| o.id == id)
            .ok_or_else(|| format!("Operation with id {} not found", id))?;
        self.operations.remove(pos);
        Ok(())
    }

    /// Update operation progress
    pub fn update_progress(&mut self, id: &str, progress: u8) -> Result<(), String> {
        let operation = self.operations.iter_mut().find(|o| o.id == id)
            .ok_or_else(|| format!("Operation with id {} not found", id))?;
        operation.progress = progress;
        Ok(())
    }

    /// Mark operation as running
    pub fn mark_running(&mut self, id: &str) -> Result<(), String> {
        let operation = self.operations.iter_mut().find(|o| o.id == id)
            .ok_or_else(|| format!("Operation with id {} not found", id))?;
        operation.state = OperationState::Running;
        Ok(())
    }

    /// Mark operation as completed
    pub fn mark_completed(&mut self, id: &str) -> Result<(), String> {
        let operation = self.operations.iter_mut().find(|o| o.id == id)
            .ok_or_else(|| format!("Operation with id {} not found", id))?;
        operation.state = OperationState::Completed;
        operation.progress = 100;
        Ok(())
    }

    /// Mark operation as failed
    pub fn mark_failed(&mut self, id: &str) -> Result<(), String> {
        let operation = self.operations.iter_mut().find(|o| o.id == id)
            .ok_or_else(|| format!("Operation with id {} not found", id))?;
        operation.state = OperationState::Failed;
        Ok(())
    }

    /// Get all operations
    pub fn get_operations(&self) -> &[CompressionOperation] {
        &self.operations
    }

    /// Get running operations
    pub fn get_running_operations(&self) -> Vec<&CompressionOperation> {
        self.operations.iter().filter(|o| o.state == OperationState::Running).collect()
    }

    /// Get operation by ID
    pub fn get_operation(&self, id: &str) -> Option<&CompressionOperation> {
        self.operations.iter().find(|o| o.id == id)
    }

    /// Get operations by algorithm
    pub fn get_operations_by_algorithm(&self, algorithm: CompressionAlgorithm) -> Vec<&CompressionOperation> {
        self.operations.iter().filter(|o| o.algorithm == algorithm).collect()
    }
}

impl Default for CompressionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let mut manager = CompressionManager::new();
        let result = manager.compress("op1".to_string(), "/path/source".to_string(), "/path/target.gz".to_string(), CompressionAlgorithm::Gzip);
        assert!(result.is_ok());
        assert_eq!(manager.get_operations().len(), 1);
    }

    #[test]
    fn test_decompress() {
        let mut manager = CompressionManager::new();
        let result = manager.decompress("op1".to_string(), "/path/source.gz".to_string(), "/path/target".to_string(), CompressionAlgorithm::Gzip);
        assert!(result.is_ok());
        assert_eq!(manager.get_operations().len(), 1);
    }

    #[test]
    fn test_duplicate_operation() {
        let mut manager = CompressionManager::new();
        manager.compress("op1".to_string(), "/path/source".to_string(), "/path/target.gz".to_string(), CompressionAlgorithm::Gzip).unwrap();
        let result = manager.compress("op1".to_string(), "/path/source2".to_string(), "/path/target2.gz".to_string(), CompressionAlgorithm::Zstd);
        assert!(result.is_err());
    }

    #[test]
    fn test_operation_lifecycle() {
        let mut manager = CompressionManager::new();
        manager.compress("op1".to_string(), "/path/source".to_string(), "/path/target.gz".to_string(), CompressionAlgorithm::Gzip).unwrap();
        
        manager.mark_running("op1").unwrap();
        assert_eq!(manager.get_operation("op1").unwrap().state, OperationState::Running);
        
        manager.update_progress("op1", 50).unwrap();
        assert_eq!(manager.get_operation("op1").unwrap().progress, 50);
        
        manager.mark_completed("op1").unwrap();
        assert_eq!(manager.get_operation("op1").unwrap().state, OperationState::Completed);
        assert_eq!(manager.get_operation("op1").unwrap().progress, 100);
    }

    #[test]
    fn test_cancel_operation() {
        let mut manager = CompressionManager::new();
        manager.compress("op1".to_string(), "/path/source".to_string(), "/path/target.gz".to_string(), CompressionAlgorithm::Gzip).unwrap();
        
        manager.cancel_operation("op1").unwrap();
        assert_eq!(manager.get_operation("op1").unwrap().state, OperationState::Cancelled);
    }
}
