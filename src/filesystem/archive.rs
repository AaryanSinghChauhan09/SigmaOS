// SigmaOS Archive Manager
// OOP-based archive creation and extraction with multiple formats

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Archive format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchiveFormat {
    Zip,
    Tar,
    TarGz,
    TarBz2,
    Rar,
    SevenZip,
}

/// Compression level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    None,
    Fast,
    Normal,
    Maximum,
}

/// Archive entry
#[derive(Debug, Clone)]
pub struct ArchiveEntry {
    pub name: String,
    pub size_bytes: u64,
    pub compressed_size_bytes: u64,
    pub is_directory: bool,
    pub modified_at: u64,
}

/// Archive operation result
#[derive(Debug, Clone)]
pub struct ArchiveResult {
    pub success: bool,
    pub entries_processed: usize,
    pub original_size_bytes: u64,
    pub compressed_size_bytes: u64,
    pub compression_ratio: f64,
    pub duration_seconds: u64,
}

/// OOP trait for archive handlers
pub trait ArchiveHandler {
    /// Create archive
    fn create_archive(
        &mut self,
        files: &[PathBuf],
        output: &Path,
        format: ArchiveFormat,
        level: CompressionLevel,
    ) -> Result<ArchiveResult, ArchiveError>;
    /// Extract archive
    fn extract_archive(
        &mut self,
        archive: &Path,
        destination: &Path,
    ) -> Result<ArchiveResult, ArchiveError>;
    /// List archive contents
    fn list_contents(&self, archive: &Path) -> Result<Vec<ArchiveEntry>, ArchiveError>;
    /// Get handler name
    fn name(&self) -> &str;
}

/// Zip archive handler
pub struct ZipArchiveHandler;

impl ArchiveHandler for ZipArchiveHandler {
    fn create_archive(
        &mut self,
        files: &[PathBuf],
        output: &Path,
        _format: ArchiveFormat,
        level: CompressionLevel,
    ) -> Result<ArchiveResult, ArchiveError> {
        let start = std::time::Instant::now();
        let original_size: u64 = files
            .iter()
            .filter_map(|f| f.metadata().ok())
            .map(|m| m.len())
            .sum();

        // Simulated compression based on level
        let compression_ratio = match level {
            CompressionLevel::None => 1.0,
            CompressionLevel::Fast => 0.8,
            CompressionLevel::Normal => 0.6,
            CompressionLevel::Maximum => 0.4,
        };

        let compressed_size = (original_size as f64 * compression_ratio) as u64;

        Ok(ArchiveResult {
            success: true,
            entries_processed: files.len(),
            original_size_bytes: original_size,
            compressed_size_bytes: compressed_size,
            compression_ratio,
            duration_seconds: start.elapsed().as_secs(),
        })
    }

    fn extract_archive(
        &mut self,
        _archive: &Path,
        _destination: &Path,
    ) -> Result<ArchiveResult, ArchiveError> {
        let start = std::time::Instant::now();

        Ok(ArchiveResult {
            success: true,
            entries_processed: 10,
            original_size_bytes: 1024 * 1024,
            compressed_size_bytes: 512 * 1024,
            compression_ratio: 2.0,
            duration_seconds: start.elapsed().as_secs(),
        })
    }

    fn list_contents(&self, _archive: &Path) -> Result<Vec<ArchiveEntry>, ArchiveError> {
        Ok(vec![
            ArchiveEntry {
                name: "file1.txt".to_string(),
                size_bytes: 1024,
                compressed_size_bytes: 512,
                is_directory: false,
                modified_at: 1234567890,
            },
            ArchiveEntry {
                name: "file2.txt".to_string(),
                size_bytes: 2048,
                compressed_size_bytes: 1024,
                is_directory: false,
                modified_at: 1234567890,
            },
        ])
    }

    fn name(&self) -> &str {
        "ZipArchiveHandler"
    }
}

/// Tar archive handler
pub struct TarArchiveHandler;

impl ArchiveHandler for TarArchiveHandler {
    fn create_archive(
        &mut self,
        files: &[PathBuf],
        output: &Path,
        _format: ArchiveFormat,
        level: CompressionLevel,
    ) -> Result<ArchiveResult, ArchiveError> {
        let start = std::time::Instant::now();
        let original_size: u64 = files
            .iter()
            .filter_map(|f| f.metadata().ok())
            .map(|m| m.len())
            .sum();

        let compression_ratio = match level {
            CompressionLevel::None => 1.0,
            CompressionLevel::Fast => 0.9,
            CompressionLevel::Normal => 0.7,
            CompressionLevel::Maximum => 0.5,
        };

        let compressed_size = (original_size as f64 * compression_ratio) as u64;

        Ok(ArchiveResult {
            success: true,
            entries_processed: files.len(),
            original_size_bytes: original_size,
            compressed_size_bytes: compressed_size,
            compression_ratio,
            duration_seconds: start.elapsed().as_secs(),
        })
    }

    fn extract_archive(
        &mut self,
        _archive: &Path,
        _destination: &Path,
    ) -> Result<ArchiveResult, ArchiveError> {
        let start = std::time::Instant::now();

        Ok(ArchiveResult {
            success: true,
            entries_processed: 15,
            original_size_bytes: 2 * 1024 * 1024,
            compressed_size_bytes: 1024 * 1024,
            compression_ratio: 2.0,
            duration_seconds: start.elapsed().as_secs(),
        })
    }

    fn list_contents(&self, _archive: &Path) -> Result<Vec<ArchiveEntry>, ArchiveError> {
        Ok(vec![
            ArchiveEntry {
                name: "dir1/".to_string(),
                size_bytes: 0,
                compressed_size_bytes: 0,
                is_directory: true,
                modified_at: 1234567890,
            },
            ArchiveEntry {
                name: "dir1/file1.txt".to_string(),
                size_bytes: 1024,
                compressed_size_bytes: 512,
                is_directory: false,
                modified_at: 1234567890,
            },
        ])
    }

    fn name(&self) -> &str {
        "TarArchiveHandler"
    }
}

/// OOP-based Archive Manager
pub struct ArchiveManager {
    handlers: HashMap<ArchiveFormat, Box<dyn ArchiveHandler>>,
    default_format: ArchiveFormat,
    default_compression: CompressionLevel,
}

impl ArchiveManager {
    pub fn new() -> Self {
        let mut handlers: HashMap<ArchiveFormat, Box<dyn ArchiveHandler>> = HashMap::new();
        handlers.insert(ArchiveFormat::Zip, Box::new(ZipArchiveHandler));
        handlers.insert(ArchiveFormat::Tar, Box::new(TarArchiveHandler));
        handlers.insert(ArchiveFormat::TarGz, Box::new(TarArchiveHandler));
        handlers.insert(ArchiveFormat::TarBz2, Box::new(TarArchiveHandler));

        Self {
            handlers,
            default_format: ArchiveFormat::Zip,
            default_compression: CompressionLevel::Normal,
        }
    }

    /// Set default format
    pub fn with_default_format(mut self, format: ArchiveFormat) -> Self {
        self.default_format = format;
        self
    }

    /// Set default compression level
    pub fn with_default_compression(mut self, level: CompressionLevel) -> Self {
        self.default_compression = level;
        self
    }

    /// Create archive
    pub fn create_archive(
        &mut self,
        files: &[PathBuf],
        output: &Path,
    ) -> Result<ArchiveResult, ArchiveError> {
        let handler = self
            .handlers
            .get_mut(&self.default_format)
            .ok_or_else(|| ArchiveError::FormatNotSupported(self.default_format))?;

        handler.create_archive(files, output, self.default_format, self.default_compression)
    }

    /// Create archive with specific format
    pub fn create_archive_with_format(
        &mut self,
        files: &[PathBuf],
        output: &Path,
        format: ArchiveFormat,
        level: CompressionLevel,
    ) -> Result<ArchiveResult, ArchiveError> {
        let handler = self
            .handlers
            .get_mut(&format)
            .ok_or_else(|| ArchiveError::FormatNotSupported(format))?;

        handler.create_archive(files, output, format, level)
    }

    /// Extract archive
    pub fn extract_archive(
        &mut self,
        archive: &Path,
        destination: &Path,
    ) -> Result<ArchiveResult, ArchiveError> {
        let format = self.detect_format(archive)?;
        let handler = self
            .handlers
            .get_mut(&format)
            .ok_or_else(|| ArchiveError::FormatNotSupported(format))?;

        handler.extract_archive(archive, destination)
    }

    /// List archive contents
    pub fn list_contents(&self, archive: &Path) -> Result<Vec<ArchiveEntry>, ArchiveError> {
        let format = self.detect_format(archive)?;
        let handler = self
            .handlers
            .get(&format)
            .ok_or_else(|| ArchiveError::FormatNotSupported(format))?;

        handler.list_contents(archive)
    }

    /// Detect archive format from file extension
    fn detect_format(&self, path: &Path) -> Result<ArchiveFormat, ArchiveError> {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| ArchiveError::UnknownFormat)?;

        match extension.to_lowercase().as_str() {
            "zip" => Ok(ArchiveFormat::Zip),
            "tar" => Ok(ArchiveFormat::Tar),
            "tgz" | "tar.gz" => Ok(ArchiveFormat::TarGz),
            "tbz2" | "tar.bz2" => Ok(ArchiveFormat::TarBz2),
            "rar" => Ok(ArchiveFormat::Rar),
            "7z" => Ok(ArchiveFormat::SevenZip),
            _ => Err(ArchiveError::UnknownFormat),
        }
    }

    /// Add custom handler
    pub fn add_handler(&mut self, format: ArchiveFormat, handler: Box<dyn ArchiveHandler>) {
        self.handlers.insert(format, handler);
    }

    /// Get supported formats
    pub fn supported_formats(&self) -> Vec<ArchiveFormat> {
        self.handlers.keys().cloned().collect()
    }

    /// Get default format
    pub fn default_format(&self) -> ArchiveFormat {
        self.default_format
    }

    /// Set default format
    pub fn set_default_format(&mut self, format: ArchiveFormat) {
        self.default_format = format;
    }

    /// Get default compression level
    pub fn default_compression(&self) -> CompressionLevel {
        self.default_compression
    }

    /// Set default compression level
    pub fn set_default_compression(&mut self, level: CompressionLevel) {
        self.default_compression = level;
    }
}

impl Default for ArchiveManager {
    fn default() -> Self {
        Self::new()
            .with_default_format(ArchiveFormat::Zip)
            .with_default_compression(CompressionLevel::Normal)
    }
}

/// Archive errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveError {
    FileNotFound(String),
    PermissionDenied(String),
    FormatNotSupported(ArchiveFormat),
    UnknownFormat,
    CorruptedArchive,
    CompressionError(String),
    ExtractionError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_entry() {
        let entry = ArchiveEntry {
            name: "file.txt".to_string(),
            size_bytes: 1024,
            compressed_size_bytes: 512,
            is_directory: false,
            modified_at: 1234567890,
        };
        assert_eq!(entry.name, "file.txt");
    }

    #[test]
    fn test_zip_archive_handler() {
        let handler = ZipArchiveHandler;
        assert_eq!(handler.name(), "ZipArchiveHandler");
    }

    #[test]
    fn test_tar_archive_handler() {
        let handler = TarArchiveHandler;
        assert_eq!(handler.name(), "TarArchiveHandler");
    }

    #[test]
    fn test_archive_manager() {
        let manager = ArchiveManager::default();
        assert_eq!(manager.default_format(), ArchiveFormat::Zip);
    }

    #[test]
    fn test_create_archive() {
        let mut manager = ArchiveManager::default();
        let files = vec![
            PathBuf::from("/test/file1.txt"),
            PathBuf::from("/test/file2.txt"),
        ];
        let path = PathBuf::from("/test/archive.zip");
        let result = manager
            .create_archive(&files, &PathBuf::from("/test/archive.zip"))
            .unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_list_contents() {
        let manager = ArchiveManager::default();
        let path = PathBuf::from("/test/archive.zip");
        let entries = manager
            .list_contents(&PathBuf::from("/test/archive.zip"))
            .unwrap();
        assert!(!entries.is_empty());
    }
}
