// SigmaOS File Shredder
// OOP-based secure file deletion with multiple overwrite passes

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, Seek, SeekFrom};
use std::path::Path;

/// Shredding algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShreddingAlgorithm {
    /// Single pass with zeros
    ZeroPass,
    /// Single pass with random data
    RandomPass,
    /// DoD 5220.22-M (3 passes: zeros, ones, random)
    Dod5220,
    /// Gutmann (35 passes with specific patterns)
    Gutmann,
    /// Schneier (7 passes: random, random, ones, zeros, random, random, random)
    Schneier,
}

/// Shredding result
#[derive(Debug, Clone)]
pub struct ShreddingResult {
    pub file_path: String,
    pub success: bool,
    pub passes_completed: usize,
    pub bytes_overwritten: u64,
    pub algorithm: ShreddingAlgorithm,
    pub message: String,
}

/// OOP trait for shredding strategies
pub trait ShreddingStrategy {
    /// Shred a file
    fn shred(&mut self, path: &Path) -> Result<ShreddingResult, ShredderError>;
    /// Get strategy name
    fn name(&self) -> &str;
    /// Get number of passes
    fn passes(&self) -> usize;
}

/// Zero pass shredder
pub struct ZeroPassShredder;

impl ShreddingStrategy for ZeroPassShredder {
    fn shred(&mut self, path: &Path) -> Result<ShreddingResult, ShredderError> {
        let metadata = fs::metadata(path)
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        let file_size = metadata.len();
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        // Overwrite with zeros
        let zero_buffer = vec![0u8; 8192];
        let mut bytes_written = 0u64;

        while bytes_written < file_size {
            let write_size = std::cmp::min(8192, (file_size - bytes_written) as usize);
            file.write_all(&zero_buffer[..write_size])
                .map_err(|e| ShredderError::IoError(e.to_string()))?;
            bytes_written += write_size as u64;
        }

        file.flush()
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        Ok(ShreddingResult {
            file_path: path.display().to_string(),
            success: true,
            passes_completed: 1,
            bytes_overwritten: bytes_written,
            algorithm: ShreddingAlgorithm::ZeroPass,
            message: "File shredded with single zero pass".to_string(),
        })
    }

    fn name(&self) -> &str {
        "ZeroPassShredder"
    }

    fn passes(&self) -> usize {
        1
    }
}

/// Random pass shredder
pub struct RandomPassShredder;

impl ShreddingStrategy for RandomPassShredder {
    fn shred(&mut self, path: &Path) -> Result<ShreddingResult, ShredderError> {
        let metadata = fs::metadata(path)
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        let file_size = metadata.len();
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        // Overwrite with random data
        let mut rng = rand::thread_rng();
        let mut bytes_written = 0u64;

        while bytes_written < file_size {
            let write_size = std::cmp::min(8192, (file_size - bytes_written) as usize);
            let mut random_buffer: Vec<u8> = (0..write_size).map(|_| rand::random()).collect();
            file.write_all(&random_buffer)
                .map_err(|e| ShredderError::IoError(e.to_string()))?;
            bytes_written += write_size as u64;
        }

        file.flush()
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        Ok(ShreddingResult {
            file_path: path.display().to_string(),
            success: true,
            passes_completed: 1,
            bytes_overwritten: bytes_written,
            algorithm: ShreddingAlgorithm::RandomPass,
            message: "File shredded with single random pass".to_string(),
        })
    }

    fn name(&self) -> &str {
        "RandomPassShredder"
    }

    fn passes(&self) -> usize {
        1
    }
}

/// DoD 5220.22-M shredder
pub struct Dod5220Shredder;

impl ShreddingStrategy for Dod5220Shredder {
    fn shred(&mut self, path: &Path) -> Result<ShreddingResult, ShredderError> {
        let metadata = fs::metadata(path)
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        let file_size = metadata.len();
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        let mut total_bytes_written = 0u64;

        // Pass 1: Zeros
        total_bytes_written += self.write_pattern(&mut file, file_size, 0x00)?;
        file.seek(SeekFrom::Start(0))
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        // Pass 2: Ones
        total_bytes_written += self.write_pattern(&mut file, file_size, 0xFF)?;
        file.seek(SeekFrom::Start(0))
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        // Pass 3: Random
        let mut rng = rand::thread_rng();
        let mut bytes_written = 0u64;
        while bytes_written < file_size {
            let write_size = std::cmp::min(8192, (file_size - bytes_written) as usize);
            let mut random_buffer: Vec<u8> = (0..write_size).map(|_| rand::random()).collect();
            file.write_all(&random_buffer)
                .map_err(|e| ShredderError::IoError(e.to_string()))?;
            bytes_written += write_size as u64;
        }
        total_bytes_written += bytes_written;

        file.flush()
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        Ok(ShreddingResult {
            file_path: path.display().to_string(),
            success: true,
            passes_completed: 3,
            bytes_overwritten: total_bytes_written,
            algorithm: ShreddingAlgorithm::Dod5220,
            message: "File shredded with DoD 5220.22-M (3 passes)".to_string(),
        })
    }

    fn name(&self) -> &str {
        "Dod5220Shredder"
    }

    fn passes(&self) -> usize {
        3
    }
}

impl Dod5220Shredder {
    fn write_pattern(&self, file: &mut File, file_size: u64, pattern: u8) -> Result<u64, ShredderError> {
        let buffer = vec![pattern; 8192];
        let mut bytes_written = 0u64;

        while bytes_written < file_size {
            let write_size = std::cmp::min(8192, (file_size - bytes_written) as usize);
            file.write_all(&buffer[..write_size])
                .map_err(|e| ShredderError::IoError(e.to_string()))?;
            bytes_written += write_size as u64;
        }

        Ok(bytes_written)
    }
}

/// Gutmann shredder (simplified 7-pass version for efficiency)
pub struct GutmannShredder;

impl ShreddingStrategy for GutmannShredder {
    fn shred(&mut self, path: &Path) -> Result<ShreddingResult, ShredderError> {
        let metadata = fs::metadata(path)
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        let file_size = metadata.len();
        let mut file = OpenOptions::new()
            .write(true)
            .open(path)
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        let mut total_bytes_written = 0u64;
        let patterns = vec![0x00, 0xFF, 0x55, 0xAA, 0x24, 0x92, 0x49];

        for pattern in patterns {
            total_bytes_written += self.write_pattern(&mut file, file_size, pattern)?;
            file.seek(SeekFrom::Start(0))
                .map_err(|e| ShredderError::IoError(e.to_string()))?;
        }

        file.flush()
            .map_err(|e| ShredderError::IoError(e.to_string()))?;

        Ok(ShreddingResult {
            file_path: path.display().to_string(),
            success: true,
            passes_completed: 7,
            bytes_overwritten: total_bytes_written,
            algorithm: ShreddingAlgorithm::Gutmann,
            message: "File shredded with Gutmann method (7 passes)".to_string(),
        })
    }

    fn name(&self) -> &str {
        "GutmannShredder"
    }

    fn passes(&self) -> usize {
        7
    }
}

impl GutmannShredder {
    fn write_pattern(&self, file: &mut File, file_size: u64, pattern: u8) -> Result<u64, ShredderError> {
        let buffer = vec![pattern; 8192];
        let mut bytes_written = 0u64;

        while bytes_written < file_size {
            let write_size = std::cmp::min(8192, (file_size - bytes_written) as usize);
            file.write_all(&buffer[..write_size])
                .map_err(|e| ShredderError::IoError(e.to_string()))?;
            bytes_written += write_size as u64;
        }

        Ok(bytes_written)
    }
}

/// OOP-based File Shredder Manager
pub struct FileShredder {
    strategy: Box<dyn ShreddingStrategy>,
    delete_after_shred: bool,
}

impl FileShredder {
    pub fn new(strategy: Box<dyn ShreddingStrategy>) -> Self {
        Self {
            strategy,
            delete_after_shred: true,
        }
    }

    /// Set whether to delete file after shredding
    pub fn with_delete_after(mut self, delete: bool) -> Self {
        self.delete_after_shred = delete;
        self
    }

    /// Shred a file
    pub fn shred(&mut self, path: &Path) -> Result<ShreddingResult, ShredderError> {
        if !path.exists() {
            return Err(ShredderError::FileNotFound(path.display().to_string()));
        }

        let result = self.strategy.shred(path)?;

        // Delete file after shredding if enabled
        if self.delete_after_shred && result.success {
            fs::remove_file(path)
                .map_err(|e| ShredderError::IoError(e.to_string()))?;
        }

        Ok(result)
    }

    /// Shred multiple files
    pub fn shred_multiple(&mut self, paths: &[&Path]) -> Vec<Result<ShreddingResult, ShredderError>> {
        paths.iter().map(|path| self.shred(path)).collect()
    }

    /// Get strategy name
    pub fn strategy_name(&self) -> &str {
        self.strategy.name()
    }

    /// Get number of passes
    pub fn passes(&self) -> usize {
        self.strategy.passes()
    }
}

impl Default for FileShredder {
    fn default() -> Self {
        Self::new(Box::new(Dod5220Shredder))
    }
}

/// Shredder errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShredderError {
    FileNotFound(String),
    PermissionDenied(String),
    IoError(String),
    FileTooLarge,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_pass_shredder() {
        let shredder = ZeroPassShredder;
        assert_eq!(shredder.name(), "ZeroPassShredder");
        assert_eq!(shredder.passes(), 1);
    }

    #[test]
    fn test_random_pass_shredder() {
        let shredder = RandomPassShredder;
        assert_eq!(shredder.name(), "RandomPassShredder");
        assert_eq!(shredder.passes(), 1);
    }

    #[test]
    fn test_dod5220_shredder() {
        let shredder = Dod5220Shredder;
        assert_eq!(shredder.name(), "Dod5220Shredder");
        assert_eq!(shredder.passes(), 3);
    }

    #[test]
    fn test_gutmann_shredder() {
        let shredder = GutmannShredder;
        assert_eq!(shredder.name(), "GutmannShredder");
        assert_eq!(shredder.passes(), 7);
    }

    #[test]
    fn test_file_shredder() {
        let shredder = FileShredder::default();
        assert_eq!(shredder.strategy_name(), "Dod5220Shredder");
        assert_eq!(shredder.passes(), 3);
    }
}
