// SPDX-License-Identifier: MIT
// SigmaOS Design Patterns Implementation
// Implements Factory, Strategy, Observer, Command patterns inspired by Arch Linux

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// FACTORY PATTERN
// Creates objects without specifying the exact class of object to be created
// ============================================================================

/// Package factory - creates different types of packages
pub struct PackageFactory {
    package_types: HashMap<String, Box<dyn PackageBuilder>>,
}

impl PackageFactory {
    pub fn new() -> Self {
        Self {
            package_types: HashMap::new(),
        }
    }

    pub fn register_builder(&mut self, type_name: String, builder: Box<dyn PackageBuilder>) {
        self.package_types.insert(type_name, builder);
    }

    pub fn create_package(&self, type_name: &str, name: &str, version: &str) -> Result<Package, PackageError> {
        let builder = self.package_types
            .get(type_name)
            .ok_or_else(|| PackageError::UnknownType(type_name.to_string()))?;
        
        builder.build_package(name, version)
    }

    pub fn list_types(&self) -> Vec<&String> {
        self.package_types.keys().collect()
    }
}

/// Package builder trait
pub trait PackageBuilder {
    fn build_package(&self, name: &str, version: &str) -> Result<Package, PackageError>;
    fn get_package_type(&self) -> &str;
}

/// Binary package builder
pub struct BinaryPackageBuilder;

impl PackageBuilder for BinaryPackageBuilder {
    fn build_package(&self, name: &str, version: &str) -> Result<Package, PackageError> {
        Ok(Package::new(name.to_string(), version.to_string(), PackageType::Binary))
    }

    fn get_package_type(&self) -> &str {
        "binary"
    }
}

/// Source package builder
pub struct SourcePackageBuilder;

impl PackageBuilder for SourcePackageBuilder {
    fn build_package(&self, name: &str, version: &str) -> Result<Package, PackageError> {
        Ok(Package::new(name.to_string(), version.to_string(), PackageType::Source))
    }

    fn get_package_type(&self) -> &str {
        "source"
    }
}

/// AUR package builder
pub struct AurPackageBuilder;

impl PackageBuilder for AurPackageBuilder {
    fn build_package(&self, name: &str, version: &str) -> Result<Package, PackageError> {
        Ok(Package::new(name.to_string(), version.to_string(), PackageType::Aur))
    }

    fn get_package_type(&self) -> &str {
        "aur"
    }
}

/// Package types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageType {
    Binary,
    Source,
    Aur,
}

/// Package information
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub package_type: PackageType,
    pub dependencies: Vec<String>,
    pub size: u64,
    pub checksum: String,
}

impl Package {
    pub fn new(name: String, version: String, package_type: PackageType) -> Self {
        Self {
            name,
            version,
            package_type,
            dependencies: Vec::new(),
            size: 0,
            checksum: String::new(),
        }
    }

    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn with_checksum(mut self, checksum: String) -> Self {
        self.checksum = checksum;
        self
    }
}

/// Package errors
#[derive(Debug)]
pub enum PackageError {
    UnknownType(String),
    BuildFailed(String),
    InvalidName(String),
    InvalidVersion(String),
}

impl fmt::Display for PackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageError::UnknownType(t) => write!(f, "Unknown package type: {}", t),
            PackageError::BuildFailed(msg) => write!(f, "Package build failed: {}", msg),
            PackageError::InvalidName(name) => write!(f, "Invalid package name: {}", name),
            PackageError::InvalidVersion(v) => write!(f, "Invalid package version: {}", v),
        }
    }
}

impl std::error::Error for PackageError {}

// ============================================================================
// STRATEGY PATTERN
// Defines a family of algorithms, encapsulates each one, and makes them
// interchangeable
// ============================================================================

/// Compression strategy trait
pub trait CompressionStrategy {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError>;
    fn get_compression_ratio(&self, original_size: u64, compressed_size: u64) -> f64;
    fn get_name(&self) -> &str;
}

/// Gzip compression strategy
pub struct GzipStrategy {
    level: u32,
}

impl GzipStrategy {
    pub fn new(level: u32) -> Self {
        Self { level }
    }
}

impl CompressionStrategy for GzipStrategy {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Gzip compression implementation
        Ok(data.to_vec())
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Gzip decompression implementation
        Ok(data.to_vec())
    }

    fn get_compression_ratio(&self, original_size: u64, compressed_size: u64) -> f64 {
        if original_size == 0 {
            return 0.0;
        }
        compressed_size as f64 / original_size as f64
    }

    fn get_name(&self) -> &str {
        "gzip"
    }
}

/// Zstd compression strategy
pub struct ZstdStrategy {
    level: u32,
}

impl ZstdStrategy {
    pub fn new(level: u32) -> Self {
        Self { level }
    }
}

impl CompressionStrategy for ZstdStrategy {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Zstd compression implementation
        Ok(data.to_vec())
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // Zstd decompression implementation
        Ok(data.to_vec())
    }

    fn get_compression_ratio(&self, original_size: u64, compressed_size: u64) -> f64 {
        if original_size == 0 {
            return 0.0;
        }
        compressed_size as f64 / original_size as f64
    }

    fn get_name(&self) -> &str {
        "zstd"
    }
}

/// LZ4 compression strategy
pub struct Lz4Strategy {
    acceleration: u32,
}

impl Lz4Strategy {
    pub fn new(acceleration: u32) -> Self {
        Self { acceleration }
    }
}

impl CompressionStrategy for Lz4Strategy {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // LZ4 compression implementation
        Ok(data.to_vec())
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        // LZ4 decompression implementation
        Ok(data.to_vec())
    }

    fn get_compression_ratio(&self, original_size: u64, compressed_size: u64) -> f64 {
        if original_size == 0 {
            return 0.0;
        }
        compressed_size as f64 / original_size as f64
    }

    fn get_name(&self) -> &str {
        "lz4"
    }
}

/// Compression manager - uses strategy pattern
pub struct CompressionManager {
    strategies: HashMap<String, Box<dyn CompressionStrategy>>,
    default_strategy: String,
}

impl CompressionManager {
    pub fn new() -> Self {
        Self {
            strategies: HashMap::new(),
            default_strategy: "gzip".to_string(),
        }
    }

    pub fn register_strategy(&mut self, name: String, strategy: Box<dyn CompressionStrategy>) {
        self.strategies.insert(name, strategy);
    }

    pub fn set_default_strategy(&mut self, name: &str) {
        self.default_strategy = name.to_string();
    }

    pub fn compress(&self, data: &[u8], strategy_name: Option<&str>) -> Result<Vec<u8>, CompressionError> {
        let strategy_name = strategy_name.unwrap_or(&self.default_strategy);
        let strategy = self.strategies
            .get(strategy_name)
            .ok_or_else(|| CompressionError::UnknownStrategy(strategy_name.to_string()))?;
        
        strategy.compress(data)
    }

    pub fn decompress(&self, data: &[u8], strategy_name: Option<&str>) -> Result<Vec<u8>, CompressionError> {
        let strategy_name = strategy_name.unwrap_or(&self.default_strategy);
        let strategy = self.strategies
            .get(strategy_name)
            .ok_or_else(|| CompressionError::UnknownStrategy(strategy_name.to_string()))?;
        
        strategy.decompress(data)
    }

    pub fn get_available_strategies(&self) -> Vec<&String> {
        self.strategies.keys().collect()
    }

    pub fn get_compression_ratio(&self, strategy_name: &str, original_size: u64, compressed_size: u64) -> Result<f64, CompressionError> {
        let strategy = self.strategies
            .get(strategy_name)
            .ok_or_else(|| CompressionError::UnknownStrategy(strategy_name.to_string()))?;
        
        Ok(strategy.get_compression_ratio(original_size, compressed_size))
    }
}

/// Compression errors
#[derive(Debug)]
pub enum CompressionError {
    UnknownStrategy(String),
    CompressionFailed(String),
    DecompressionFailed(String),
    InvalidData(String),
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::UnknownStrategy(s) => write!(f, "Unknown compression strategy: {}", s),
            CompressionError::CompressionFailed(msg) => write!(f, "Compression failed: {}", msg),
            CompressionError::DecompressionFailed(msg) => write!(f, "Decompression failed: {}", msg),
            CompressionError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for CompressionError {}

// ============================================================================
// OBSERVER PATTERN
// Defines a one-to-many dependency between objects so that when one object
// changes state, all its dependents are notified automatically
// ============================================================================

/// Event types for system events
#[derive(Debug, Clone)]
pub enum SystemEvent {
    PackageInstalled { name: String, version: String },
    PackageRemoved { name: String },
    PackageUpdated { name: String, old_version: String, new_version: String },
    ServiceStarted { name: String },
    ServiceStopped { name: String },
    ServiceFailed { name: String, error: String },
    SystemUpdateAvailable { count: usize },
    SystemUpdateCompleted { success: bool },
    UserLoggedIn { username: String },
    UserLoggedOut { username: String },
}

/// Observer trait - receives notifications
pub trait EventObserver {
    fn on_event(&self, event: &SystemEvent);
    fn get_observer_name(&self) -> &str;
}

/// Logger observer
pub struct LoggerObserver {
    name: String,
    level: LogLevel,
}

impl LoggerObserver {
    pub fn new(name: String, level: LogLevel) -> Self {
        Self { name, level }
    }
}

impl EventObserver for LoggerObserver {
    fn on_event(&self, event: &SystemEvent) {
        match self.level {
            LogLevel::Debug => println!("[DEBUG] {:?}", event),
            LogLevel::Info => println!("[INFO] {:?}", event),
            LogLevel::Warn => eprintln!("[WARN] {:?}", event),
            LogLevel::Error => eprintln!("[ERROR] {:?}", event),
        }
    }

    fn get_observer_name(&self) -> &str {
        &self.name
    }
}

/// Metrics observer
pub struct MetricsObserver {
    name: String,
    metrics: Arc<Mutex<HashMap<String, u64>>>,
}

impl MetricsObserver {
    pub fn new(name: String) -> Self {
        Self {
            name,
            metrics: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get_metric(&self, key: &str) -> u64 {
        self.metrics.lock().unwrap().get(key).copied().unwrap_or(0)
    }
}

impl EventObserver for MetricsObserver {
    fn on_event(&self, event: &SystemEvent) {
        let mut metrics = self.metrics.lock().unwrap();
        
        match event {
            SystemEvent::PackageInstalled { .. } => {
                *metrics.entry("packages_installed".to_string()).or_insert(0) += 1;
            }
            SystemEvent::PackageRemoved { .. } => {
                *metrics.entry("packages_removed".to_string()).or_insert(0) += 1;
            }
            SystemEvent::ServiceStarted { .. } => {
                *metrics.entry("services_started".to_string()).or_insert(0) += 1;
            }
            SystemEvent::ServiceStopped { .. } => {
                *metrics.entry("services_stopped".to_string()).or_insert(0) += 1;
            }
            SystemEvent::UserLoggedIn { .. } => {
                *metrics.entry("user_logins".to_string()).or_insert(0) += 1;
            }
            SystemEvent::UserLoggedOut { .. } => {
                *metrics.entry("user_logouts".to_string()).or_insert(0) += 1;
            }
            _ => {}
        }
    }

    fn get_observer_name(&self) -> &str {
        &self.name
    }
}

/// Notification observer
pub struct NotificationObserver {
    name: String,
    notifications: Arc<Mutex<Vec<String>>>,
}

impl NotificationObserver {
    pub fn new(name: String) -> Self {
        Self {
            name,
            notifications: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_notifications(&self) -> Vec<String> {
        self.notifications.lock().unwrap().clone()
    }
}

impl EventObserver for NotificationObserver {
    fn on_event(&self, event: &SystemEvent) {
        let mut notifications = self.notifications.lock().unwrap();
        
        let message = match event {
            SystemEvent::PackageInstalled { name, version } => {
                format!("Package {} {} installed", name, version)
            }
            SystemEvent::PackageRemoved { name } => {
                format!("Package {} removed", name)
            }
            SystemEvent::ServiceStarted { name } => {
                format!("Service {} started", name)
            }
            SystemEvent::ServiceStopped { name } => {
                format!("Service {} stopped", name)
            }
            SystemEvent::SystemUpdateAvailable { count } => {
                format!("{} updates available", count)
            }
            SystemEvent::UserLoggedIn { username } => {
                format!("User {} logged in", username)
            }
            _ => return,
        };
        
        notifications.push(message);
    }

    fn get_observer_name(&self) -> &str {
        &self.name
    }
}

/// Event bus - subject in observer pattern
pub struct EventBus {
    observers: Vec<Box<dyn EventObserver>>,
    event_queue: Arc<Mutex<Vec<SystemEvent>>>,
    running: AtomicBool,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
            event_queue: Arc::new(Mutex::new(Vec::new())),
            running: AtomicBool::new(false),
        }
    }

    pub fn subscribe(&mut self, observer: Box<dyn EventObserver>) {
        self.observers.push(observer);
    }

    pub fn unsubscribe(&mut self, name: &str) -> bool {
        if let Some(pos) = self.observers.iter().position(|o| o.get_observer_name() == name) {
            self.observers.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn publish(&self, event: SystemEvent) {
        // Queue the event for processing
        let mut queue = self.event_queue.lock().unwrap();
        queue.push(event);
    }

    pub fn process_events(&self) {
        let mut queue = self.event_queue.lock().unwrap();
        let events: Vec<SystemEvent> = queue.drain(..).collect();
        drop(queue);

        for event in events {
            for observer in &self.observers {
                observer.on_event(&event);
            }
        }
    }

    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn get_observer_count(&self) -> usize {
        self.observers.len()
    }
}

// ============================================================================
// COMMAND PATTERN
// Encapsulates a request as an object, thereby letting you parameterize
// clients with different requests, queue or log requests, and support undoable operations
// ============================================================================

/// Command trait
pub trait Command {
    fn execute(&self) -> Result<(), CommandError>;
    fn undo(&self) -> Result<(), CommandError>;
    fn get_description(&self) -> &str;
    fn get_command_name(&self) -> &str;
}

/// Install package command
pub struct InstallPackageCommand {
    package_name: String,
    version: String,
    options: InstallOptions,
    executed: AtomicBool,
    description: String,
}

impl InstallPackageCommand {
    pub fn new(package_name: String, version: String, options: InstallOptions) -> Self {
        let description = format!("Install package {} {}", package_name, version);
        Self {
            package_name,
            version,
            options,
            executed: AtomicBool::new(false),
            description,
        }
    }
}

impl Command for InstallPackageCommand {
    fn execute(&self) -> Result<(), CommandError> {
        if self.executed.load(Ordering::SeqCst) {
            return Err(CommandError::AlreadyExecuted);
        }
        
        // Execute installation
        println!("Installing package: {} {}", self.package_name, self.version);
        self.executed.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn undo(&self) -> Result<(), CommandError> {
        if !self.executed.load(Ordering::SeqCst) {
            return Err(CommandError::NotExecuted);
        }
        
        // Undo installation
        println!("Removing package: {}", self.package_name);
        self.executed.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_command_name(&self) -> &str {
        "install_package"
    }
}

/// Remove package command
pub struct RemovePackageCommand {
    package_name: String,
    executed: AtomicBool,
    description: String,
}

impl RemovePackageCommand {
    pub fn new(package_name: String) -> Self {
        let description = format!("Remove package {}", package_name);
        Self {
            package_name,
            executed: AtomicBool::new(false),
            description,
        }
    }
}

impl Command for RemovePackageCommand {
    fn execute(&self) -> Result<(), CommandError> {
        if self.executed.load(Ordering::SeqCst) {
            return Err(CommandError::AlreadyExecuted);
        }
        
        // Execute removal
        println!("Removing package: {}", self.package_name);
        self.executed.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn undo(&self) -> Result<(), CommandError> {
        if !self.executed.load(Ordering::SeqCst) {
            return Err(CommandError::NotExecuted);
        }
        
        // Undo removal (reinstall)
        println!("Reinstalling package: {}", self.package_name);
        self.executed.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_command_name(&self) -> &str {
        "remove_package"
    }
}

/// Update package command
pub struct UpdatePackageCommand {
    package_name: String,
    old_version: String,
    new_version: String,
    executed: AtomicBool,
    description: String,
}

impl UpdatePackageCommand {
    pub fn new(package_name: String, old_version: String, new_version: String) -> Self {
        let description = format!("Update package {} from {} to {}", package_name, old_version, new_version);
        Self {
            package_name,
            old_version,
            new_version,
            executed: AtomicBool::new(false),
            description,
        }
    }
}

impl Command for UpdatePackageCommand {
    fn execute(&self) -> Result<(), CommandError> {
        if self.executed.load(Ordering::SeqCst) {
            return Err(CommandError::AlreadyExecuted);
        }
        
        // Execute update
        println!("Updating package: {} {} -> {}", self.package_name, self.old_version, self.new_version);
        self.executed.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn undo(&self) -> Result<(), CommandError> {
        if !self.executed.load(Ordering::SeqCst) {
            return Err(CommandError::NotExecuted);
        }
        
        // Undo update (downgrade)
        println!("Downgrading package: {} {} -> {}", self.package_name, self.new_version, self.old_version);
        self.executed.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn get_description(&self) -> &str {
        &self.description
    }

    fn get_command_name(&self) -> &str {
        "update_package"
    }
}

/// Install options
#[derive(Debug, Clone)]
pub struct InstallOptions {
    pub force: bool,
    pub overwrite: bool,
    pub nodeps: bool,
    pub needed: bool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            force: false,
            overwrite: false,
            nodeps: false,
            needed: false,
        }
    }
}

/// Command errors
#[derive(Debug)]
pub enum CommandError {
    AlreadyExecuted,
    NotExecuted,
    ExecutionFailed(String),
    UndoFailed(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::AlreadyExecuted => write!(f, "Command already executed"),
            CommandError::NotExecuted => write!(f, "Command not executed"),
            CommandError::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            CommandError::UndoFailed(msg) => write!(f, "Undo failed: {}", msg),
        }
    }
}

impl std::error::Error for CommandError {}

/// Command history manager
pub struct CommandHistory {
    commands: Vec<Box<dyn Command>>,
    current_index: i64,
    max_history: usize,
}

impl CommandHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            commands: Vec::new(),
            current_index: -1,
            max_history,
        }
    }

    pub fn execute(&mut self, command: Box<dyn Command>) -> Result<(), CommandError> {
        command.execute()?;
        self.commands.push(command);
        
        // Trim history if needed
        if self.commands.len() > self.max_history {
            self.commands.remove(0);
        }
        
        self.current_index = self.commands.len() as i64 - 1;
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), CommandError> {
        if self.current_index < 0 {
            return Err(CommandError::NotExecuted);
        }
        
        let index = self.current_index as usize;
        if index >= self.commands.len() {
            return Err(CommandError::NotExecuted);
        }
        
        self.commands[index].undo()?;
        self.current_index -= 1;
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), CommandError> {
        if self.current_index >= self.commands.len() as i64 - 1 {
            return Err(CommandError::AlreadyExecuted);
        }
        
        self.current_index += 1;
        let index = self.current_index as usize;
        self.commands[index].execute()
    }

    pub fn get_history(&self) -> Vec<&str> {
        self.commands.iter().map(|c| c.get_description()).collect()
    }

    pub fn get_current_command(&self) -> Option<&str> {
        if self.current_index >= 0 && (self.current_index as usize) < self.commands.len() {
            Some(self.commands[self.current_index as usize].get_description())
        } else {
            None
        }
    }

    pub fn can_undo(&self) -> bool {
        self.current_index >= 0
    }

    pub fn can_redo(&self) -> bool {
        self.current_index < self.commands.len() as i64 - 1
    }
}

// ============================================================================
// SINGLETON PATTERN
// Ensures a class has only one instance and provides a global point of access
// ============================================================================

/// System configuration singleton
pub struct SystemConfig {
    instance: Option<Arc<SystemConfig>>,
    config: HashMap<String, String>,
    initialized: AtomicBool,
}

impl SystemConfig {
    pub fn new() -> Self {
        Self {
            instance: None,
            config: HashMap::new(),
            initialized: AtomicBool::new(false),
        }
    }

    pub fn get_instance() -> Arc<SystemConfig> {
        // In a real implementation, this would use a proper singleton pattern
        // with thread safety. For simplicity, we'll use a static instance.
        static mut INSTANCE: Option<Arc<SystemConfig>> = None;
        
        unsafe {
            if INSTANCE.is_none() {
                INSTANCE = Some(Arc::new(SystemConfig::new()));
            }
            INSTANCE.as_ref().unwrap().clone()
        }
    }

    pub fn initialize(&mut self) -> Result<(), ConfigError> {
        if self.initialized.load(Ordering::SeqCst) {
            return Err(ConfigError::AlreadyInitialized);
        }
        
        // Initialize configuration
        self.config.insert("system_name".to_string(), "SigmaOS".to_string());
        self.config.insert("version".to_string(), "1.0.0".to_string());
        self.config.insert("arch".to_string(), "x86_64".to_string());
        
        self.initialized.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }
}

/// Configuration errors
#[derive(Debug)]
pub enum ConfigError {
    AlreadyInitialized,
    NotInitialized,
    InvalidKey(String),
    InvalidValue(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::AlreadyInitialized => write!(f, "Configuration already initialized"),
            ConfigError::NotInitialized => write!(f, "Configuration not initialized"),
            ConfigError::InvalidKey(key) => write!(f, "Invalid configuration key: {}", key),
            ConfigError::InvalidValue(value) => write!(f, "Invalid configuration value: {}", value),
        }
    }
}

impl std::error::Error for ConfigError {}

// ============================================================================
// BUILDER PATTERN
// Constructs complex objects step by step
// ============================================================================

/// Package builder
pub struct PackageBuilder2 {
    name: String,
    version: String,
    description: String,
    dependencies: Vec<String>,
    size: u64,
    checksum: String,
    package_type: PackageType,
}

impl PackageBuilder2 {
    pub fn new(name: String, version: String, package_type: PackageType) -> Self {
        Self {
            name,
            version,
            description: String::new(),
            dependencies: Vec::new(),
            size: 0,
            checksum: String::new(),
            package_type,
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn checksum(mut self, checksum: String) -> Self {
        self.checksum = checksum;
        self
    }

    pub fn build(self) -> Package {
        Package {
            name: self.name,
            version: self.version,
            package_type: self.package_type,
            dependencies: self.dependencies,
            size: self.size,
            checksum: self.checksum,
        }
    }
}

/// Service builder
pub struct ServiceBuilder {
    name: String,
    description: String,
    exec_start: String,
    restart_policy: RestartPolicy,
    dependencies: Vec<String>,
    environment: HashMap<String, String>,
}

impl ServiceBuilder {
    pub fn new(name: String, exec_start: String) -> Self {
        Self {
            name,
            description: String::new(),
            exec_start,
            restart_policy: RestartPolicy::No,
            dependencies: Vec::new(),
            environment: HashMap::new(),
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn restart_policy(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }

    pub fn dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn environment(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }

    pub fn build(self) -> Service {
        Service {
            name: self.name,
            description: self.description,
            exec_start: self.exec_start,
            restart_policy: self.restart_policy,
            dependencies: self.dependencies,
            environment: self.environment,
        }
    }
}

/// Service information
#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub exec_start: String,
    pub restart_policy: RestartPolicy,
    pub dependencies: Vec<String>,
    pub environment: HashMap<String, String>,
}

/// Restart policies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    OnSuccess,
    OnFailure,
    Always,
}

// ============================================================================
// LOG LEVELS
// ============================================================================

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
    fn test_package_factory() {
        let mut factory = PackageFactory::new();
        factory.register_builder("binary".to_string(), Box::new(BinaryPackageBuilder));
        factory.register_builder("source".to_string(), Box::new(SourcePackageBuilder));
        factory.register_builder("aur".to_string(), Box::new(AurPackageBuilder));

        let package = factory.create_package("binary", "test-package", "1.0.0").unwrap();
        assert_eq!(package.name, "test-package");
        assert_eq!(package.version, "1.0.0");
        assert_eq!(package.package_type, PackageType::Binary);
    }

    #[test]
    fn test_compression_strategy() {
        let mut manager = CompressionManager::new();
        manager.register_strategy("gzip".to_string(), Box::new(GzipStrategy::new(6)));
        manager.register_strategy("zstd".to_string(), Box::new(ZstdStrategy::new(3)));
        manager.register_strategy("lz4".to_string(), Box::new(Lz4Strategy::new(1)));

        let data = b"test data";
        let compressed = manager.compress(data, Some("gzip")).unwrap();
        let decompressed = manager.decompress(&compressed, Some("gzip")).unwrap();
        assert_eq!(data, &decompressed[..]);
    }

    #[test]
    fn test_observer_pattern() {
        let mut event_bus = EventBus::new();
        
        let logger = LoggerObserver::new("logger".to_string(), LogLevel::Info);
        let metrics = MetricsObserver::new("metrics".to_string());
        let notifications = NotificationObserver::new("notifications".to_string());
        
        event_bus.subscribe(Box::new(logger));
        event_bus.subscribe(Box::new(metrics));
        event_bus.subscribe(Box::new(notifications));
        
        event_bus.publish(SystemEvent::PackageInstalled {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
        });
        
        event_bus.process_events();
        
        assert_eq!(event_bus.get_observer_count(), 3);
    }

    #[test]
    fn test_command_pattern() {
        let mut history = CommandHistory::new(10);
        
        let install_cmd = InstallPackageCommand::new(
            "test-package".to_string(),
            "1.0.0".to_string(),
            InstallOptions::default(),
        );
        
        history.execute(Box::new(install_cmd)).unwrap();
        
        let remove_cmd = RemovePackageCommand::new("test-package".to_string());
        history.execute(Box::new(remove_cmd)).unwrap();
        
        assert!(history.can_undo());
        history.undo().unwrap();
        assert!(history.can_redo());
        history.redo().unwrap();
    }

    #[test]
    fn test_builder_pattern() {
        let package = PackageBuilder2::new("test-package".to_string(), "1.0.0".to_string(), PackageType::Binary)
            .description("Test package".to_string())
            .dependencies(vec!["dep1".to_string(), "dep2".to_string()])
            .size(1024 * 1024)
            .checksum("abc123".to_string())
            .build();
        
        assert_eq!(package.name, "test-package");
        assert_eq!(package.version, "1.0.0");
        assert_eq!(package.package_type, PackageType::Binary);
        assert_eq!(package.dependencies.len(), 2);
        assert_eq!(package.size, 1024 * 1024);
    }

    #[test]
    fn test_service_builder() {
        let service = ServiceBuilder::new("test-service".to_string(), "/usr/bin/test".to_string())
            .description("Test service".to_string())
            .restart_policy(RestartPolicy::Always)
            .dependencies(vec!["network.target".to_string()])
            .environment("DEBUG".to_string(), "true".to_string())
            .build();
        
        assert_eq!(service.name, "test-service");
        assert_eq!(service.restart_policy, RestartPolicy::Always);
        assert_eq!(service.dependencies.len(), 1);
        assert_eq!(service.environment.get("DEBUG"), Some(&"true".to_string()));
    }
}
