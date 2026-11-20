// SPDX-License-Identifier: MIT
// SigmaOS SOLID Principles Implementation
// Demonstrates Single Responsibility, Open/Closed, Liskov Substitution,
// Interface Segregation, and Dependency Inversion principles

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// ============================================================================
// SINGLE RESPONSIBILITY PRINCIPLE (SRP)
// Each class should have only one reason to change
// ============================================================================

/// Configuration parser - only handles parsing
pub trait ConfigParser {
    fn parse(&self, content: &str) -> Result<ConfigData, ParseError>;
    fn validate(&self, data: &ConfigData) -> Result<(), ValidationError>;
}

/// INI-style configuration parser
pub struct IniConfigParser;

impl ConfigParser for IniConfigParser {
    fn parse(&self, content: &str) -> Result<ConfigData, ParseError> {
        let mut config = ConfigData::new();
        let mut current_section = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len()-1].to_string();
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().trim_matches('"').trim_matches('\'').to_string();
                config.set_value(&current_section, &key, value);
            }
        }

        Ok(config)
    }

    fn validate(&self, data: &ConfigData) -> Result<(), ValidationError> {
        // Validate configuration data
        Ok(())
    }
}

/// YAML-style configuration parser
pub struct YamlConfigParser;

impl ConfigParser for YamlConfigParser {
    fn parse(&self, content: &str) -> Result<ConfigData, ParseError> {
        // YAML parsing implementation
        let mut config = ConfigData::new();
        // ... YAML parsing logic
        Ok(config)
    }

    fn validate(&self, data: &ConfigData) -> Result<(), ValidationError> {
        Ok(())
    }
}

/// Configuration data - only handles data storage
#[derive(Debug, Clone)]
pub struct ConfigData {
    pub sections: HashMap<String, HashMap<String, String>>,
    pub global_values: HashMap<String, String>,
}

impl ConfigData {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
            global_values: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, section: &str, key: &str, value: String) {
        if section.is_empty() {
            self.global_values.insert(key.to_string(), value);
        } else {
            self.sections
                .entry(section.to_string())
                .or_insert_with(HashMap::new)
                .insert(key.to_string(), value);
        }
    }

    pub fn get_value(&self, section: &str, key: &str) -> Option<&String> {
        if section.is_empty() {
            self.global_values.get(key)
        } else {
            self.sections.get(section)?.get(key)
        }
    }
}

/// Configuration manager - only handles file operations
pub struct ConfigManager {
    parser: Box<dyn ConfigParser>,
    config_dir: PathBuf,
    configs: HashMap<String, ConfigData>,
}

impl ConfigManager {
    pub fn new(parser: Box<dyn ConfigParser>, config_dir: PathBuf) -> Self {
        Self {
            parser,
            config_dir,
            configs: HashMap::new(),
        }
    }

    pub fn load_config(&mut self, name: &str) -> Result<(), io::Error> {
        let path = self.config_dir.join(format!("{}.conf", name));
        if !path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&path)?;
        let config = self.parser.parse(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        self.parser.validate(&config)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        self.configs.insert(name.to_string(), config);
        Ok(())
    }

    pub fn save_config(&self, name: &str) -> Result<(), io::Error> {
        if let Some(config) = self.configs.get(name) {
            let path = self.config_dir.join(format!("{}.conf", name));
            let content = self.serialize_config(config);
            fs::write(&path, content)?;
        }
        Ok(())
    }

    fn serialize_config(&self, config: &ConfigData) -> String {
        // Serialize configuration back to string
        let mut content = String::new();
        // ... serialization logic
        content
    }
}

/// Parse errors
#[derive(Debug)]
pub enum ParseError {
    InvalidSyntax(String),
    MissingSection(String),
    InvalidValue(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
            ParseError::MissingSection(msg) => write!(f, "Missing section: {}", msg),
            ParseError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Validation errors
#[derive(Debug)]
pub enum ValidationError {
    RequiredField(String),
    InvalidValue(String),
    ConstraintViolation(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::RequiredField(field) => write!(f, "Required field missing: {}", field),
            ValidationError::InvalidValue(value) => write!(f, "Invalid value: {}", value),
            ValidationError::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

// ============================================================================
// OPEN/CLOSED PRINCIPLE (OCP)
// Open for extension, closed for modification
// ============================================================================

/// Package source trait - open for extension
pub trait PackageSource {
    fn fetch_package(&self, name: &str) -> Result<PackageInfo, PackageError>;
    fn get_source_name(&self) -> &str;
    fn is_available(&self) -> bool;
}

/// Official repository source
pub struct OfficialRepoSource {
    base_url: String,
}

impl OfficialRepoSource {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

impl PackageSource for OfficialRepoSource {
    fn fetch_package(&self, name: &str) -> Result<PackageInfo, PackageError> {
        // Implementation for official repos
        Ok(PackageInfo::new(name.to_string(), "1.0.0".to_string()))
    }

    fn get_source_name(&self) -> &str {
        "official"
    }

    fn is_available(&self) -> bool {
        true
    }
}

/// AUR source
pub struct AurSource {
    api_url: String,
}

impl AurSource {
    pub fn new(api_url: String) -> Self {
        Self { api_url }
    }
}

impl PackageSource for AurSource {
    fn fetch_package(&self, name: &str) -> Result<PackageInfo, PackageError> {
        // Implementation for AUR
        Ok(PackageInfo::new(name.to_string(), "1.0.0".to_string()))
    }

    fn get_source_name(&self) -> &str {
        "aur"
    }

    fn is_available(&self) -> bool {
        true
    }
}

/// Local file source
pub struct LocalSource {
    local_path: PathBuf,
}

impl LocalSource {
    pub fn new(local_path: PathBuf) -> Self {
        Self { local_path }
    }
}

impl PackageSource for LocalSource {
    fn fetch_package(&self, name: &str) -> Result<PackageInfo, PackageError> {
        // Implementation for local files
        Ok(PackageInfo::new(name.to_string(), "1.0.0".to_string()))
    }

    fn get_source_name(&self) -> &str {
        "local"
    }

    fn is_available(&self) -> bool {
        self.local_path.exists()
    }
}

/// Package manager - works with any package source
pub struct PackageManager {
    sources: Vec<Box<dyn PackageSource>>,
}

impl PackageManager {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    pub fn add_source(&mut self, source: Box<dyn PackageSource>) {
        self.sources.push(source);
    }

    pub fn install_package(&self, name: &str) -> Result<PackageInfo, PackageError> {
        // Try each source until one succeeds
        for source in &self.sources {
            if source.is_available() {
                if let Ok(package) = source.fetch_package(name) {
                    return Ok(package);
                }
            }
        }
        Err(PackageError::PackageNotFound(name.to_string()))
    }
}

/// Package information
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub size: u64,
}

impl PackageInfo {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: String::new(),
            size: 0,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }
}

/// Package errors
#[derive(Debug)]
pub enum PackageError {
    PackageNotFound(String),
    DownloadFailed(String, String),
    InstallationFailed(String, String),
    InvalidPackageName,
}

impl fmt::Display for PackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageError::PackageNotFound(name) => write!(f, "Package not found: {}", name),
            PackageError::DownloadFailed(name, error) => write!(f, "Download failed for {}: {}", name, error),
            PackageError::InstallationFailed(name, error) => write!(f, "Installation failed for {}: {}", name, error),
            PackageError::InvalidPackageName => write!(f, "Invalid package name"),
        }
    }
}

impl std::error::Error for PackageError {}

// ============================================================================
// LISKOV SUBSTITUTION PRINCIPLE (LSP)
// Objects of a superclass should be replaceable with objects of a subclass
// ============================================================================

/// Storage backend trait
pub trait StorageBackend {
    fn read(&self, key: &str) -> Result<Vec<u8>, StorageError>;
    fn write(&self, key: &str, data: &[u8]) -> Result<(), StorageError>;
    fn delete(&self, key: &str) -> Result<(), StorageError>;
    fn exists(&self, key: &str) -> bool;
}

/// File system storage backend
pub struct FileSystemStorage {
    base_path: PathBuf,
}

impl FileSystemStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

impl StorageBackend for FileSystemStorage {
    fn read(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        let path = self.base_path.join(key);
        fs::read(&path).map_err(|e| StorageError::ReadFailed(key.to_string(), e.to_string()))
    }

    fn write(&self, key: &str, data: &[u8]) -> Result<(), StorageError> {
        let path = self.base_path.join(key);
        fs::write(&path, data).map_err(|e| StorageError::WriteFailed(key.to_string(), e.to_string()))
    }

    fn delete(&self, key: &str) -> Result<(), StorageError> {
        let path = self.base_path.join(key);
        fs::remove_file(&path).map_err(|e| StorageError::DeleteFailed(key.to_string(), e.to_string()))
    }

    fn exists(&self, key: &str) -> bool {
        let path = self.base_path.join(key);
        path.exists()
    }
}

/// Database storage backend
pub struct DatabaseStorage {
    connection_string: String,
}

impl DatabaseStorage {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

impl StorageBackend for DatabaseStorage {
    fn read(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        // Database read implementation
        Ok(vec![])
    }

    fn write(&self, key: &str, data: &[u8]) -> Result<(), StorageError> {
        // Database write implementation
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), StorageError> {
        // Database delete implementation
        Ok(())
    }

    fn exists(&self, key: &str) -> bool {
        // Database exists implementation
        true
    }
}

/// Cache manager - works with any storage backend
pub struct CacheManager {
    storage: Box<dyn StorageBackend>,
    ttl: u64,
}

impl CacheManager {
    pub fn new(storage: Box<dyn StorageBackend>, ttl: u64) -> Self {
        Self { storage, ttl }
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        if self.storage.exists(key) {
            self.storage.read(key).map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn set(&self, key: &str, data: &[u8]) -> Result<(), StorageError> {
        self.storage.write(key, data)
    }

    pub fn delete(&self, key: &str) -> Result<(), StorageError> {
        self.storage.delete(key)
    }
}

/// Storage errors
#[derive(Debug)]
pub enum StorageError {
    ReadFailed(String, String),
    WriteFailed(String, String),
    DeleteFailed(String, String),
    ConnectionFailed(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::ReadFailed(key, error) => write!(f, "Read failed for {}: {}", key, error),
            StorageError::WriteFailed(key, error) => write!(f, "Write failed for {}: {}", key, error),
            StorageError::DeleteFailed(key, error) => write!(f, "Delete failed for {}: {}", key, error),
            StorageError::ConnectionFailed(error) => write!(f, "Connection failed: {}", error),
        }
    }
}

impl std::error::Error for StorageError {}

// ============================================================================
// INTERFACE SEGREGATION PRINCIPLE (ISP)
// Clients should not be forced to depend on interfaces they do not use
// ============================================================================

/// Package downloader trait - focused interface
pub trait PackageDownloader {
    fn download(&self, package: &PackageInfo) -> Result<PathBuf, DownloadError>;
    fn get_mirrors(&self) -> Vec<Mirror>;
    fn set_mirrors(&mut self, mirrors: Vec<Mirror>) -> Result<(), DownloadError>;
}

/// Package installer trait - focused interface
pub trait PackageInstaller {
    fn install(&self, package_path: &Path) -> Result<(), InstallError>;
    fn remove(&self, package_name: &str) -> Result<(), InstallError>;
    fn verify(&self, package_path: &Path) -> Result<VerifyResult, InstallError>;
}

/// Package searcher trait - focused interface
pub trait PackageSearcher {
    fn search(&self, query: &str) -> Result<Vec<PackageInfo>, SearchError>;
    fn get_package_info(&self, name: &str) -> Result<PackageInfo, SearchError>;
}

/// HTTP downloader implementation
pub struct HttpDownloader {
    mirrors: Vec<Mirror>,
    timeout: u64,
}

impl HttpDownloader {
    pub fn new(mirrors: Vec<Mirror>, timeout: u64) -> Self {
        Self { mirrors, timeout }
    }
}

impl PackageDownloader for HttpDownloader {
    fn download(&self, package: &PackageInfo) -> Result<PathBuf, DownloadError> {
        // HTTP download implementation
        Ok(PathBuf::from("/tmp/package.tar.gz"))
    }

    fn get_mirrors(&self) -> Vec<Mirror> {
        self.mirrors.clone()
    }

    fn set_mirrors(&mut self, mirrors: Vec<Mirror>) -> Result<(), DownloadError> {
        self.mirrors = mirrors;
        Ok(())
    }
}

/// File system installer implementation
pub struct FileSystemInstaller {
    install_dir: PathBuf,
}

impl FileSystemInstaller {
    pub fn new(install_dir: PathBuf) -> Self {
        Self { install_dir }
    }
}

impl PackageInstaller for FileSystemInstaller {
    fn install(&self, package_path: &Path) -> Result<(), InstallError> {
        // File system installation implementation
        Ok(())
    }

    fn remove(&self, package_name: &str) -> Result<(), InstallError> {
        // File system removal implementation
        Ok(())
    }

    fn verify(&self, package_path: &Path) -> Result<VerifyResult, InstallError> {
        // File system verification implementation
        Ok(VerifyResult::new(true, "Verified".to_string()))
    }
}

/// Database searcher implementation
pub struct DatabaseSearcher {
    connection_string: String,
}

impl DatabaseSearcher {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

impl PackageSearcher for DatabaseSearcher {
    fn search(&self, query: &str) -> Result<Vec<PackageInfo>, SearchError> {
        // Database search implementation
        Ok(vec![])
    }

    fn get_package_info(&self, name: &str) -> Result<PackageInfo, SearchError> {
        // Database get package info implementation
        Ok(PackageInfo::new(name.to_string(), "1.0.0".to_string()))
    }
}

/// Mirror information
#[derive(Debug, Clone)]
pub struct Mirror {
    pub url: String,
    pub priority: i32,
}

/// Verification result
#[derive(Debug)]
pub struct VerifyResult {
    pub valid: bool,
    pub message: String,
}

impl VerifyResult {
    pub fn new(valid: bool, message: String) -> Self {
        Self { valid, message }
    }
}

/// Download errors
#[derive(Debug)]
pub enum DownloadError {
    NetworkError(String),
    TimeoutError,
    MirrorNotFound,
    InvalidUrl(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            DownloadError::TimeoutError => write!(f, "Download timeout"),
            DownloadError::MirrorNotFound => write!(f, "No mirrors available"),
            DownloadError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
        }
    }
}

impl std::error::Error for DownloadError {}

/// Install errors
#[derive(Debug)]
pub enum InstallError {
    PermissionDenied,
    DiskFull,
    InvalidPackage,
    AlreadyInstalled(String),
    DependencyMissing(String),
}

impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallError::PermissionDenied => write!(f, "Permission denied"),
            InstallError::DiskFull => write!(f, "Disk full"),
            InstallError::InvalidPackage => write!(f, "Invalid package"),
            InstallError::AlreadyInstalled(name) => write!(f, "Package already installed: {}", name),
            InstallError::DependencyMissing(dep) => write!(f, "Missing dependency: {}", dep),
        }
    }
}

impl std::error::Error for InstallError {}

/// Search errors
#[derive(Debug)]
pub enum SearchError {
    ConnectionFailed(String),
    QueryFailed(String),
    InvalidQuery(String),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            SearchError::QueryFailed(msg) => write!(f, "Query failed: {}", msg),
            SearchError::InvalidQuery(query) => write!(f, "Invalid query: {}", query),
        }
    }
}

impl std::error::Error for SearchError {}

// ============================================================================
// DEPENDENCY INVERSION PRINCIPLE (DIP)
// High-level modules should not depend on low-level modules
// Both should depend on abstractions
// ============================================================================

/// Package service - depends on abstractions
pub struct PackageService {
    downloader: Box<dyn PackageDownloader>,
    installer: Box<dyn PackageInstaller>,
    searcher: Box<dyn PackageSearcher>,
}

impl PackageService {
    pub fn new(
        downloader: Box<dyn PackageDownloader>,
        installer: Box<dyn PackageInstaller>,
        searcher: Box<dyn PackageSearcher>,
    ) -> Self {
        Self {
            downloader,
            installer,
            searcher,
        }
    }

    pub fn install_package(&self, name: &str) -> Result<InstallResult, ServiceError> {
        // Search for package
        let package_info = self.searcher
            .get_package_info(name)
            .map_err(|e| ServiceError::SearchFailed(e.to_string()))?;
        
        // Download package
        let package_path = self.downloader
            .download(&package_info)
            .map_err(|e| ServiceError::DownloadFailed(e.to_string()))?;
        
        // Install package
        self.installer
            .install(&package_path)
            .map_err(|e| ServiceError::InstallFailed(e.to_string()))?;
        
        Ok(InstallResult::new(name.to_string(), package_info.version))
    }

    pub fn search_packages(&self, query: &str) -> Result<Vec<PackageInfo>, ServiceError> {
        self.searcher
            .search(query)
            .map_err(|e| ServiceError::SearchFailed(e.to_string()))
    }
}

/// Service errors
#[derive(Debug)]
pub enum ServiceError {
    SearchFailed(String),
    DownloadFailed(String),
    InstallFailed(String),
    ConfigurationError(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::SearchFailed(msg) => write!(f, "Search failed: {}", msg),
            ServiceError::DownloadFailed(msg) => write!(f, "Download failed: {}", msg),
            ServiceError::InstallFailed(msg) => write!(f, "Install failed: {}", msg),
            ServiceError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}

/// Installation result
#[derive(Debug)]
pub struct InstallResult {
    pub package_name: String,
    pub version: String,
    pub size: u64,
}

impl InstallResult {
    pub fn new(package_name: String, version: String) -> Self {
        Self {
            package_name,
            version,
            size: 0,
        }
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }
}

// ============================================================================
// COMPOSITION OVER INHERITANCE
// Use composition to build complex objects from simpler ones
// ============================================================================

/// Package manager using composition
pub struct AdvancedPackageManager {
    config_manager: ConfigManager,
    package_service: PackageService,
    cache_manager: CacheManager,
    logger: Logger,
}

impl AdvancedPackageManager {
    pub fn new(
        config_manager: ConfigManager,
        package_service: PackageService,
        cache_manager: CacheManager,
        logger: Logger,
    ) -> Self {
        Self {
            config_manager,
            package_service,
            cache_manager,
            logger,
        }
    }

    pub fn install_package(&self, name: &str) -> Result<InstallResult, ServiceError> {
        self.logger.info(&format!("Installing package: {}", name));
        
        // Check cache first
        if let Ok(Some(cached)) = self.cache_manager.get(name) {
            self.logger.info("Package found in cache");
            // Return cached result
        }
        
        // Install package
        let result = self.package_service.install_package(name)?;
        
        // Cache the result
        let _ = self.cache_manager.set(name, b"installed");
        
        self.logger.info(&format!("Package installed: {}", name));
        Ok(result)
    }
}

/// Logger - simple logging interface
pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }

    pub fn info(&self, message: &str) {
        if self.level >= LogLevel::Info {
            println!("[INFO] {}", message);
        }
    }

    pub fn error(&self, message: &str) {
        if self.level >= LogLevel::Error {
            eprintln!("[ERROR] {}", message);
        }
    }
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ini_config_parser() {
        let parser = IniConfigParser;
        let content = r#"
        [section1]
        key1=value1
        key2=value2
        
        [section2]
        key3=value3
        "#;

        let config = parser.parse(content).unwrap();
        assert_eq!(config.get_value("section1", "key1"), Some(&"value1".to_string()));
        assert_eq!(config.get_value("section2", "key3"), Some(&"value3".to_string()));
    }

    #[test]
    fn test_package_manager_with_sources() {
        let mut pm = PackageManager::new();
        pm.add_source(Box::new(OfficialRepoSource::new("https://repo.example.com".to_string())));
        pm.add_source(Box::new(AurSource::new("https://aur.example.com".to_string())));

        let result = pm.install_package("test-package");
        assert!(result.is_ok());
    }

    #[test]
    fn test_cache_manager_with_file_storage() {
        let storage = FileSystemStorage::new(PathBuf::from("/tmp/test_cache"));
        let cache = CacheManager::new(Box::new(storage), 3600);

        cache.set("test-key", b"test-value").unwrap();
        let result = cache.get("test-key").unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_package_service() {
        let downloader = Box::new(HttpDownloader::new(vec![], 30));
        let installer = Box::new(FileSystemInstaller::new(PathBuf::from("/tmp")));
        let searcher = Box::new(DatabaseSearcher::new("test".to_string()));

        let service = PackageService::new(downloader, installer, searcher);
        let result = service.search_packages("test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_package_manager() {
        let config_manager = ConfigManager::new(Box::new(IniConfigParser), PathBuf::from("/tmp"));
        
        let downloader = Box::new(HttpDownloader::new(vec![], 30));
        let installer = Box::new(FileSystemInstaller::new(PathBuf::from("/tmp")));
        let searcher = Box::new(DatabaseSearcher::new("test".to_string()));
        let package_service = PackageService::new(downloader, installer, searcher);
        
        let storage = FileSystemStorage::new(PathBuf::from("/tmp/cache"));
        let cache_manager = CacheManager::new(Box::new(storage), 3600);
        
        let logger = Logger::new(LogLevel::Info);
        
        let manager = AdvancedPackageManager::new(
            config_manager,
            package_service,
            cache_manager,
            logger,
        );

        let result = manager.install_package("test-package");
        assert!(result.is_ok());
    }
}
