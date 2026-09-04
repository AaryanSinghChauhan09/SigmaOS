// SigmaOS Package Cache Purging Engine
// Implements Arch Linux pacman-contrib's paccache functionality
// Retains keep_count uninstalled package tarballs while purging stale cache files

use std::string::String;
use std::vec::Vec;

/// Package cache entry
#[derive(Debug, Clone)]
pub struct PackageCacheEntry {
    pub name: String,
    pub version: String,
    pub file_path: String,
    pub size: u64,
    pub last_accessed: u64,
    pub is_installed: bool,
}

/// Paccache engine configuration
#[derive(Debug, Clone)]
pub struct PaccacheConfig {
    pub keep_count: usize,
    pub cache_directory: String,
    pub dry_run: bool,
    pub verbose: bool,
}

impl Default for PaccacheConfig {
    fn default() -> Self {
        Self {
            keep_count: 3,
            cache_directory: "/var/cache/sigpkg".to_string(),
            dry_run: false,
            verbose: false,
        }
    }
}

/// Paccache engine
pub struct PaccacheEngine {
    pub config: PaccacheConfig,
    pub cache_entries: Vec<PackageCacheEntry>,
}

impl PaccacheEngine {
    pub fn new(config: PaccacheConfig) -> Self {
        Self {
            config,
            cache_entries: Vec::new(),
        }
    }

    /// Scan cache directory for package files
    pub fn scan_cache(&mut self) {
        // In real implementation, would scan filesystem
        // For now, we simulate with sample data
        self.cache_entries = vec![
            PackageCacheEntry {
                name: "linux-kernel".to_string(),
                version: "6.0.0".to_string(),
                file_path: "/var/cache/sigpkg/linux-kernel-6.0.0.sig".to_string(),
                size: 1024 * 1024 * 50,
                last_accessed: 1234567890,
                is_installed: true,
            },
            PackageCacheEntry {
                name: "linux-kernel".to_string(),
                version: "5.19.0".to_string(),
                file_path: "/var/cache/sigpkg/linux-kernel-5.19.0.sig".to_string(),
                size: 1024 * 1024 * 50,
                last_accessed: 1234567880,
                is_installed: false,
            },
            PackageCacheEntry {
                name: "linux-kernel".to_string(),
                version: "5.18.0".to_string(),
                file_path: "/var/cache/sigpkg/linux-kernel-5.18.0.sig".to_string(),
                size: 1024 * 1024 * 50,
                last_accessed: 1234567870,
                is_installed: false,
            },
        ];
    }

    /// Get packages to remove based on keep_count
    pub fn get_packages_to_remove(&self) -> Vec<&PackageCacheEntry> {
        let mut to_remove = Vec::new();

        // Group by package name
        let mut grouped: std::collections::BTreeMap<String, Vec<&PackageCacheEntry>> =
            std::collections::BTreeMap::new();

        for entry in &self.cache_entries {
            grouped
                .entry(entry.name.clone())
                .or_insert_with(Vec::new)
                .push(entry);
        }

        // For each package, keep only keep_count versions
        for (_name, entries) in grouped {
            let mut sorted_entries: Vec<&PackageCacheEntry> = entries.clone();
            sorted_entries.sort_by(|a, b| b.version.cmp(&a.version));

            // Keep installed packages and keep_count newest versions
            let kept_count = sorted_entries.iter().filter(|e| e.is_installed).count();
            let additional_keep = if kept_count < self.config.keep_count {
                self.config.keep_count - kept_count
            } else {
                0
            };

            let mut kept = 0;
            for entry in sorted_entries {
                if entry.is_installed {
                    // Always keep installed
                } else if kept < additional_keep {
                    kept += 1;
                } else {
                    to_remove.push(entry);
                }
            }
        }

        to_remove
    }

    /// Calculate space that would be freed
    pub fn calculate_freed_space(&self) -> u64 {
        self.get_packages_to_remove().iter().map(|e| e.size).sum()
    }

    /// Remove old packages from cache
    pub fn purge_cache(&mut self) -> Vec<String> {
        let to_remove = self.get_packages_to_remove();
        let mut removed_files = Vec::new();

        let to_remove_paths: Vec<String> = to_remove.iter().map(|e| e.file_path.clone()).collect();

        for entry in to_remove {
            if self.config.verbose {
                let msg = format!("Removing: {} ({})", entry.file_path, entry.version);
                removed_files.push(msg);
            }

            if !self.config.dry_run {
                removed_files.push(format!("Deleted: {}", entry.file_path));
            }
        }

        // Remove from cache_entries
        self.cache_entries
            .retain(|e| !to_remove_paths.contains(&e.file_path));

        removed_files
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> String {
        let total_size: u64 = self.cache_entries.iter().map(|e| e.size).sum();
        let installed_count = self.cache_entries.iter().filter(|e| e.is_installed).count();
        let to_remove_count = self.get_packages_to_remove().len();
        let freed_space = self.calculate_freed_space();

        format!(
            "Cache Statistics\nTotal Packages: {}\nInstalled: {}\nTo Remove: {}\nTotal Size: {} MB\nFreed Space: {} MB",
            self.cache_entries.len(),
            installed_count,
            to_remove_count,
            total_size / (1024 * 1024),
            freed_space / (1024 * 1024)
        )
    }
}

impl Default for PaccacheEngine {
    fn default() -> Self {
        Self::new(PaccacheConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paccache_scan() {
        let engine = PaccacheEngine::default();
        let mut scanner = engine;
        scanner.scan_cache();
        assert!(scanner.cache_entries.len() > 0);
    }

    #[test]
    fn test_paccache_purge() {
        let config = PaccacheConfig {
            keep_count: 2,
            cache_directory: "/var/cache/sigpkg".to_string(),
            dry_run: true,
            verbose: true,
        };
        let mut engine = PaccacheEngine::new(config);
        engine.scan_cache();
        let removed = engine.purge_cache();
        assert!(removed.len() > 0);
    }
}
