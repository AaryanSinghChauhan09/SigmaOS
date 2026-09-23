// SigmaOS Sovereign File Hierarchy System (FHS) Engine
// Zero-dependency Rust #![no_std] / std implementation of Linux & BSD inspired FHS translation & mapping.

#[cfg(not(test))]
use alloc::string::{String, ToString};
#[cfg(not(test))]
use alloc::vec::Vec;
#[cfg(not(test))]
use alloc::format;

#[cfg(test)]
use std::string::String;
#[cfg(test)]
use std::vec::Vec;

/// FHS Directory Classification Category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FhsDirectoryKind {
    UsrMergeSymlink,
    BaseSystemBinary,
    ThirdPartyLocalBinary,
    StorePackagePath,
    UserWorkspacePath,
    VolatileRuntimePath,
    KernelVirtualFsPath,
}

/// Path Translation Mapping Rules
#[derive(Debug, Clone)]
pub struct FhsPathRule {
    pub source_prefix: String,
    pub target_prefix: String,
    pub kind: FhsDirectoryKind,
}

/// Sovereign File Hierarchy System Engine
#[derive(Debug, Clone)]
pub struct SovereignFhsHierarchyEngine {
    pub rules: Vec<FhsPathRule>,
    pub usr_merge_enabled: bool,
    pub freebsd_local_enabled: bool,
    pub nix_store_enabled: bool,
}

impl SovereignFhsHierarchyEngine {
    pub fn new() -> Self {
        let mut rules = Vec::new();

        // Linux UsrMerge Aliases
        rules.push(FhsPathRule {
            source_prefix: String::from("/bin/"),
            target_prefix: String::from("/usr/bin/"),
            kind: FhsDirectoryKind::UsrMergeSymlink,
        });
        rules.push(FhsPathRule {
            source_prefix: String::from("/sbin/"),
            target_prefix: String::from("/usr/bin/"),
            kind: FhsDirectoryKind::UsrMergeSymlink,
        });
        rules.push(FhsPathRule {
            source_prefix: String::from("/lib/"),
            target_prefix: String::from("/usr/lib/"),
            kind: FhsDirectoryKind::UsrMergeSymlink,
        });
        rules.push(FhsPathRule {
            source_prefix: String::from("/lib64/"),
            target_prefix: String::from("/usr/lib/"),
            kind: FhsDirectoryKind::UsrMergeSymlink,
        });

        // FreeBSD Local / Base System Separation
        rules.push(FhsPathRule {
            source_prefix: String::from("/usr/local/bin/"),
            target_prefix: String::from("/usr/local/bin/"),
            kind: FhsDirectoryKind::ThirdPartyLocalBinary,
        });

        // NixOS Store Resolution
        rules.push(FhsPathRule {
            source_prefix: String::from("/sigma/store/"),
            target_prefix: String::from("@store/"),
            kind: FhsDirectoryKind::StorePackagePath,
        });

        // Sovereign Namespace Mappings
        rules.push(FhsPathRule {
            source_prefix: String::from("/home/"),
            target_prefix: String::from("@user/"),
            kind: FhsDirectoryKind::UserWorkspacePath,
        });
        rules.push(FhsPathRule {
            source_prefix: String::from("/run/"),
            target_prefix: String::from("@volatile/"),
            kind: FhsDirectoryKind::VolatileRuntimePath,
        });

        Self {
            rules,
            usr_merge_enabled: true,
            freebsd_local_enabled: true,
            nix_store_enabled: true,
        }
    }

    /// Translates a given Linux/BSD FHS path into its canonical target path
    pub fn resolve_path(&self, path: &str) -> String {
        for rule in &self.rules {
            if path.starts_with(&rule.source_prefix) {
                let remainder = &path[rule.source_prefix.len()..];
                return format!("{}{}", rule.target_prefix, remainder);
            }
            if path == &rule.source_prefix[..rule.source_prefix.len() - 1] {
                return rule.target_prefix.clone();
            }
        }
        String::from(path)
    }

    /// Identifies the directory classification of a given path
    pub fn classify_path(&self, path: &str) -> FhsDirectoryKind {
        if path.starts_with("/bin/") || path.starts_with("/sbin/") || path.starts_with("/lib/") || path.starts_with("/lib64/") {
            FhsDirectoryKind::UsrMergeSymlink
        } else if path.starts_with("/usr/local/") {
            FhsDirectoryKind::ThirdPartyLocalBinary
        } else if path.starts_with("/sigma/store/") {
            FhsDirectoryKind::StorePackagePath
        } else if path.starts_with("/home/") {
            FhsDirectoryKind::UserWorkspacePath
        } else if path.starts_with("/run/") || path.starts_with("/tmp/") {
            FhsDirectoryKind::VolatileRuntimePath
        } else if path.starts_with("/proc/") || path.starts_with("/sys/") || path.starts_with("/dev/") {
            FhsDirectoryKind::KernelVirtualFsPath
        } else {
            FhsDirectoryKind::BaseSystemBinary
        }
    }
}

impl Default for SovereignFhsHierarchyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fhs_hierarchy_engine() {
        let engine = SovereignFhsHierarchyEngine::new();

        // UsrMerge path resolution
        assert_eq!(engine.resolve_path("/bin/ls"), "/usr/bin/ls");
        assert_eq!(engine.resolve_path("/sbin/ip"), "/usr/bin/ip");
        assert_eq!(engine.resolve_path("/lib64/libc.so"), "/usr/lib/libc.so");

        // NixOS Store and Sovereign Namespace resolution
        assert_eq!(engine.resolve_path("/sigma/store/hash-app/bin/app"), "@store/hash-app/bin/app");
        assert_eq!(engine.resolve_path("/home/jules/file.txt"), "@user/jules/file.txt");

        // Classification
        assert_eq!(engine.classify_path("/bin/bash"), FhsDirectoryKind::UsrMergeSymlink);
        assert_eq!(engine.classify_path("/usr/local/bin/htop"), FhsDirectoryKind::ThirdPartyLocalBinary);
        assert_eq!(engine.classify_path("/proc/1/status"), FhsDirectoryKind::KernelVirtualFsPath);
    }
}
