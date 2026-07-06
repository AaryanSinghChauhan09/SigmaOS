// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/pkg/sigpkg.rs — Universal Package Manager
// Unify .deb, .rpm, Flatpak, Snap into single package format
//
// Features:
//   - Universal package format compatible with multiple package managers
//   - Automatic dependency resolution across package types
//   - Transactional updates with rollback capability
//   - Repository management for multiple sources
//   - India context: Optimized for Indian mirror networks
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Package Format Support ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageFormat {
    SigPkg,    // Native SigmaOS format
    Deb,       // Debian format
    Rpm,       // Red Hat format
    Flatpak,   // Flatpak format
    Snap,      // Snap format
    AppImage,  // AppImage format
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub description: String,
    pub maintainer: String,
    pub license: String,
    pub homepage: String,
    pub formats: Vec<PackageFormat>,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub size_bytes: u64,
    pub installed_size_bytes: u64,
    pub checksum: String,
}

// ── Repository Management ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub priority: u32,
    pub gpg_key: Option<String>,
    pub mirror_regions: Vec<String>,  // For India-specific mirrors
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorConfig {
    pub region: String,
    pub mirrors: Vec<String>,
    pub auto_select: bool,
}

// ── Transaction Management ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionState {
    Pending,
    Downloading,
    Installing,
    Configuring,
    Completed,
    Failed(String),
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub operations: Vec<PackageOperation>,
    pub state: TransactionState,
    pub start_time: String,
    pub end_time: Option<String>,
    pub rollback_point: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageOperation {
    Install(String),
    Remove(String),
    Upgrade(String),
    Downgrade(String, String),  // package, target_version
    Configure(String),
}

// ── Dependency Resolution ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, PackageMetadata>,
    pub edges: HashMap<String, Vec<String>>,  // package -> dependencies
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionResult {
    pub install_order: Vec<String>,
    pub conflicts: Vec<String>,
    pub missing_dependencies: Vec<String>,
    pub circular_dependencies: Vec<Vec<String>>,
}

// ── Package Query ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageQuery {
    pub name_pattern: Option<String>,
    pub category: Option<String>,
    pub installed_only: bool,
    pub upgradable_only: bool,
    pub format: Option<PackageFormat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub metadata: PackageMetadata,
    pub installed: bool,
    pub installed_version: Option<String>,
    pub upgradable: bool,
    pub latest_version: String,
    pub repository: String,
}

// ── India-Specific Optimization ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndiaMirrorConfig {
    pub enabled: bool,
    pub preferred_region: String,  // North, South, East, West, Central
    pub cdn_url: String,
    pub fallback_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfile {
    pub bandwidth_mbps: u32,
    pub data_limit_gb: Option<u64>,
    pub offline_mode: bool,
    pub update_frequency: String,
}

// ── Package Manager Engine ───────────────────────────────────────────────

pub struct SigPkgEngine {
    repositories: Vec<Repository>,
    installed_packages: HashMap<String, PackageMetadata>,
    dependency_graph: DependencyGraph,
    current_transaction: Option<Transaction>,
    india_config: IndiaMirrorConfig,
    network_profile: NetworkProfile,
}

impl SigPkgEngine {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
            installed_packages: HashMap::new(),
            dependency_graph: DependencyGraph {
                nodes: HashMap::new(),
                edges: HashMap::new(),
            },
            current_transaction: None,
            india_config: IndiaMirrorConfig {
                enabled: true,
                preferred_region: "Central".to_string(),
                cdn_url: "https://cdn.sigmaos.in/packages".to_string(),
                fallback_urls: vec![
                    "https://mirror.sigmaos.in/packages".to_string(),
                    "https://cdn1.sigmaos.in/packages".to_string(),
                ],
            },
            network_profile: NetworkProfile {
                bandwidth_mbps: 100,
                data_limit_gb: None,
                offline_mode: false,
                update_frequency: "daily".to_string(),
            },
        }
    }

    /// Add repository
    pub fn add_repository(&mut self, repo: Repository) -> Result<(), String> {
        // In production: Validate repository and fetch metadata
        self.repositories.push(repo);
        Ok(())
    }

    /// Remove repository
    pub fn remove_repository(&mut self, name: &str) -> Result<(), String> {
        self.repositories.retain(|r| r.name != name);
        Ok(())
    }

    /// Search packages
    pub fn search(&self, query: PackageQuery) -> Vec<PackageInfo> {
        // In production: Search across all repositories
        // For now: Return empty result
        Vec::new()
    }

    /// Resolve dependencies
    pub fn resolve_dependencies(&self, packages: &[String]) -> ResolutionResult {
        // In production: Build dependency graph and resolve
        // For now: Return mock result
        ResolutionResult {
            install_order: packages.to_vec(),
            conflicts: Vec::new(),
            missing_dependencies: Vec::new(),
            circular_dependencies: Vec::new(),
        }
    }

    /// Begin transaction
    pub fn begin_transaction(&mut self, operations: Vec<PackageOperation>) -> Result<String, String> {
        let transaction_id = format!("txn_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        
        let transaction = Transaction {
            id: transaction_id.clone(),
            operations,
            state: TransactionState::Pending,
            start_time: chrono::Utc::now().to_rfc3339(),
            end_time: None,
            rollback_point: None,
        };
        
        self.current_transaction = Some(transaction);
        Ok(transaction_id)
    }

    /// Commit transaction
    pub fn commit_transaction(&mut self) -> Result<(), String> {
        if let Some(ref mut transaction) = self.current_transaction {
            // In production: Execute all operations
            transaction.state = TransactionState::Completed;
            transaction.end_time = Some(chrono::Utc::now().to_rfc3339());
            Ok(())
        } else {
            Err("No active transaction".to_string())
        }
    }

    /// Rollback transaction
    pub fn rollback_transaction(&mut self) -> Result<(), String> {
        if let Some(ref mut transaction) = self.current_transaction {
            // In production: Rollback all operations
            transaction.state = TransactionState::RolledBack;
            transaction.end_time = Some(chrono::Utc::now().to_rfc3339());
            Ok(())
        } else {
            Err("No active transaction".to_string())
        }
    }

    /// Install package
    pub fn install(&mut self, package: &str) -> Result<Transaction, String> {
        let operations = vec![PackageOperation::Install(package.to_string())];
        let txn_id = self.begin_transaction(operations)?;
        self.commit_transaction()?;
        self.current_transaction.clone().ok_or("Transaction lost".to_string())
    }

    /// Remove package
    pub fn remove(&mut self, package: &str) -> Result<Transaction, String> {
        let operations = vec![PackageOperation::Remove(package.to_string())];
        let txn_id = self.begin_transaction(operations)?;
        self.commit_transaction()?;
        self.current_transaction.clone().ok_or("Transaction lost".to_string())
    }

    /// Upgrade package
    pub fn upgrade(&mut self, package: &str) -> Result<Transaction, String> {
        let operations = vec![PackageOperation::Upgrade(package.to_string())];
        let txn_id = self.begin_transaction(operations)?;
        self.commit_transaction()?;
        self.current_transaction.clone().ok_or("Transaction lost".to_string())
    }

    /// Update package cache
    pub fn update_cache(&mut self) -> Result<(), String> {
        // In production: Fetch metadata from all repositories
        // Use India-specific mirrors for faster downloads
        Ok(())
    }

    /// Upgrade all packages
    pub fn upgrade_all(&mut self) -> Result<Transaction, String> {
        // In production: Get list of upgradable packages
        let operations = Vec::new();  // Mock: no operations
        let txn_id = self.begin_transaction(operations)?;
        self.commit_transaction()?;
        self.current_transaction.clone().ok_or("Transaction lost".to_string())
    }

    /// Get India mirror configuration
    pub fn get_india_config(&self) -> &IndiaMirrorConfig {
        &self.india_config
    }

    /// Set India mirror configuration
    pub fn set_india_config(&mut self, config: IndiaMirrorConfig) {
        self.india_config = config;
    }

    /// Get network profile
    pub fn get_network_profile(&self) -> &NetworkProfile {
        &self.network_profile
    }

    /// Set network profile
    pub fn set_network_profile(&mut self, profile: NetworkProfile) {
        self.network_profile = profile;
    }

    /// Get installed packages
    pub fn get_installed_packages(&self) -> Vec<&PackageMetadata> {
        self.installed_packages.values().collect()
    }
}

impl Default for SigPkgEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn sigpkg_engine_create() -> *mut SigPkgEngine {
    Box::into_raw(Box::new(SigPkgEngine::new()))
}

#[no_mangle]
pub extern "C" fn sigpkg_engine_destroy(engine: *mut SigPkgEngine) {
    unsafe {
        if !engine.is_null() {
            let _ = Box::from_raw(engine);
        }
    }
}

#[no_mangle]
pub extern "C" fn sigpkg_install(engine: *mut SigPkgEngine,
                               package: *const u8, package_len: usize,
                               out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if engine.is_null() || package.is_null() { return -1; }
        let package = String::from_utf8_unchecked(
            std::slice::from_raw_parts(package, package_len));
        match (*engine).install(&package) {
            Ok(transaction) => {
                let json = serde_json::to_string(&transaction).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn sigpkg_remove(engine: *mut SigPkgEngine,
                               package: *const u8, package_len: usize,
                               out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if engine.is_null() || package.is_null() { return -1; }
        let package = String::from_utf8_unchecked(
            std::slice::from_raw_parts(package, package_len));
        match (*engine).remove(&package) {
            Ok(transaction) => {
                let json = serde_json::to_string(&transaction).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn sigpkg_update_cache(engine: *mut SigPkgEngine) -> i32 {
    unsafe {
        if engine.is_null() { return -1; }
        match (*engine).update_cache() {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}
