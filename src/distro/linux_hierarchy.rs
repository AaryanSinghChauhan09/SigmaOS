#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]

// SigmaOS Linux-Inspired Hierarchical Architecture Subsystem
// Implements 3-tier organizational topology, subsystem bridge resolution, and centralized documentation routing.

use std::collections::BTreeMap;
use std::format;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArchitectureTierLevel {
    Tier1CoreKernelBoot,
    Tier2ServicesStack,
    Tier3UserspaceApplications,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchitectureModuleNode {
    pub name: String,
    pub tier: ArchitectureTierLevel,
    pub primary_path: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateSubsystemAlias {
    pub legacy_alias: String,
    pub canonical_module: String,
    pub tier: ArchitectureTierLevel,
}

#[derive(Debug, Clone)]
pub struct LinuxHierarchyArchitectureGovernor {
    pub modules: BTreeMap<String, ArchitectureModuleNode>,
    pub subsystem_bridges: Vec<DuplicateSubsystemAlias>,
    pub documentation_categories: BTreeMap<String, String>,
}

impl LinuxHierarchyArchitectureGovernor {
    pub fn new() -> Self {
        let mut modules = BTreeMap::new();

        // Tier 1: Bootloader & Kernel Core
        modules.insert(
            "boot".to_string(),
            ArchitectureModuleNode {
                name: "boot".to_string(),
                tier: ArchitectureTierLevel::Tier1CoreKernelBoot,
                primary_path: "src/boot".to_string(),
                description: "Bootloader & Early Init".to_string(),
            },
        );
        modules.insert(
            "arch".to_string(),
            ArchitectureModuleNode {
                name: "arch".to_string(),
                tier: ArchitectureTierLevel::Tier1CoreKernelBoot,
                primary_path: "src/arch".to_string(),
                description: "Architecture-specific (x86_64, ARM, RISC-V)".to_string(),
            },
        );
        modules.insert(
            "kernel".to_string(),
            ArchitectureModuleNode {
                name: "kernel".to_string(),
                tier: ArchitectureTierLevel::Tier1CoreKernelBoot,
                primary_path: "src/kernel".to_string(),
                description: "Core Microkernel (process, memory, irq)".to_string(),
            },
        );
        modules.insert(
            "klib".to_string(),
            ArchitectureModuleNode {
                name: "klib".to_string(),
                tier: ArchitectureTierLevel::Tier1CoreKernelBoot,
                primary_path: "src/klib".to_string(),
                description: "Kernel Primitive Library (no_std)".to_string(),
            },
        );
        modules.insert(
            "drivers".to_string(),
            ArchitectureModuleNode {
                name: "drivers".to_string(),
                tier: ArchitectureTierLevel::Tier1CoreKernelBoot,
                primary_path: "src/drivers".to_string(),
                description: "Essential Hardware Drivers".to_string(),
            },
        );

        // Tier 2: Services & Protocol Stack
        modules.insert(
            "fs".to_string(),
            ArchitectureModuleNode {
                name: "fs".to_string(),
                tier: ArchitectureTierLevel::Tier2ServicesStack,
                primary_path: "src/fs".to_string(),
                description: "Filesystem & Storage Engines".to_string(),
            },
        );
        modules.insert(
            "net".to_string(),
            ArchitectureModuleNode {
                name: "net".to_string(),
                tier: ArchitectureTierLevel::Tier2ServicesStack,
                primary_path: "src/net".to_string(),
                description: "Networking Stack (TCP/IP/UDP)".to_string(),
            },
        );
        modules.insert(
            "ipc".to_string(),
            ArchitectureModuleNode {
                name: "ipc".to_string(),
                tier: ArchitectureTierLevel::Tier2ServicesStack,
                primary_path: "src/ipc".to_string(),
                description: "Inter-Process Communication Primitives".to_string(),
            },
        );
        modules.insert(
            "security".to_string(),
            ArchitectureModuleNode {
                name: "security".to_string(),
                tier: ArchitectureTierLevel::Tier2ServicesStack,
                primary_path: "src/security".to_string(),
                description: "Capability & Landlock Security".to_string(),
            },
        );

        // Tier 3: Userspace & Applications
        modules.insert(
            "userland".to_string(),
            ArchitectureModuleNode {
                name: "userland".to_string(),
                tier: ArchitectureTierLevel::Tier3UserspaceApplications,
                primary_path: "src/userland".to_string(),
                description: "Userspace System Utilities & Shell".to_string(),
            },
        );
        modules.insert(
            "desktop".to_string(),
            ArchitectureModuleNode {
                name: "desktop".to_string(),
                tier: ArchitectureTierLevel::Tier3UserspaceApplications,
                primary_path: "src/desktop".to_string(),
                description: "Zenith Wayland Compositor & GUI".to_string(),
            },
        );
        modules.insert(
            "ai".to_string(),
            ArchitectureModuleNode {
                name: "ai".to_string(),
                tier: ArchitectureTierLevel::Tier3UserspaceApplications,
                primary_path: "src/ai".to_string(),
                description: "AI Native Agent Runtime".to_string(),
            },
        );
        modules.insert(
            "virtualization".to_string(),
            ArchitectureModuleNode {
                name: "virtualization".to_string(),
                tier: ArchitectureTierLevel::Tier3UserspaceApplications,
                primary_path: "src/virtualization".to_string(),
                description: "Hypervisor & Container Runtimes".to_string(),
            },
        );

        // Aliases / Bridge mappings
        let subsystem_bridges = vec![
            DuplicateSubsystemAlias {
                legacy_alias: "memory".to_string(),
                canonical_module: "kernel/memory".to_string(),
                tier: ArchitectureTierLevel::Tier1CoreKernelBoot,
            },
            DuplicateSubsystemAlias {
                legacy_alias: "mm".to_string(),
                canonical_module: "kernel/memory".to_string(),
                tier: ArchitectureTierLevel::Tier1CoreKernelBoot,
            },
            DuplicateSubsystemAlias {
                legacy_alias: "network".to_string(),
                canonical_module: "net".to_string(),
                tier: ArchitectureTierLevel::Tier2ServicesStack,
            },
            DuplicateSubsystemAlias {
                legacy_alias: "networking".to_string(),
                canonical_module: "net".to_string(),
                tier: ArchitectureTierLevel::Tier2ServicesStack,
            },
            DuplicateSubsystemAlias {
                legacy_alias: "driver".to_string(),
                canonical_module: "drivers".to_string(),
                tier: ArchitectureTierLevel::Tier1CoreKernelBoot,
            },
            DuplicateSubsystemAlias {
                legacy_alias: "device".to_string(),
                canonical_module: "dev".to_string(),
                tier: ArchitectureTierLevel::Tier1CoreKernelBoot,
            },
            DuplicateSubsystemAlias {
                legacy_alias: "shell".to_string(),
                canonical_module: "userland/shell".to_string(),
                tier: ArchitectureTierLevel::Tier3UserspaceApplications,
            },
        ];

        // Centralized Documentation Categories
        let mut documentation_categories = BTreeMap::new();
        documentation_categories
            .insert("architecture".to_string(), "docs/architecture/".to_string());
        documentation_categories.insert("kernel".to_string(), "docs/kernel/".to_string());
        documentation_categories.insert("userland".to_string(), "docs/userland/".to_string());
        documentation_categories.insert("filesystem".to_string(), "docs/filesystem/".to_string());
        documentation_categories.insert("networking".to_string(), "docs/networking/".to_string());
        documentation_categories.insert("security".to_string(), "docs/security/".to_string());
        documentation_categories.insert("api".to_string(), "docs/api/".to_string());
        documentation_categories.insert("guides".to_string(), "docs/guides/".to_string());

        Self {
            modules,
            subsystem_bridges,
            documentation_categories,
        }
    }

    pub fn resolve_subsystem_alias(&self, alias: &str) -> String {
        for bridge in &self.subsystem_bridges {
            if bridge.legacy_alias == alias {
                return bridge.canonical_module.clone();
            }
        }
        alias.to_string()
    }

    pub fn resolve_documentation_path(&self, category: &str) -> Option<String> {
        self.documentation_categories.get(category).cloned()
    }

    pub fn validate_topology_integrity(&self) -> bool {
        let has_tier1 = self
            .modules
            .values()
            .any(|m| m.tier == ArchitectureTierLevel::Tier1CoreKernelBoot);
        let has_tier2 = self
            .modules
            .values()
            .any(|m| m.tier == ArchitectureTierLevel::Tier2ServicesStack);
        let has_tier3 = self
            .modules
            .values()
            .any(|m| m.tier == ArchitectureTierLevel::Tier3UserspaceApplications);
        has_tier1 && has_tier2 && has_tier3 && !self.subsystem_bridges.is_empty()
    }
}

impl Default for LinuxHierarchyArchitectureGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_hierarchy_governor() {
        let governor = LinuxHierarchyArchitectureGovernor::new();
        assert!(governor.validate_topology_integrity());

        assert_eq!(governor.resolve_subsystem_alias("memory"), "kernel/memory");
        assert_eq!(governor.resolve_subsystem_alias("networking"), "net");
        assert_eq!(governor.resolve_subsystem_alias("driver"), "drivers");
        assert_eq!(governor.resolve_subsystem_alias("shell"), "userland/shell");

        assert_eq!(
            governor.resolve_documentation_path("kernel").unwrap(),
            "docs/kernel/"
        );
        assert_eq!(
            governor.resolve_documentation_path("security").unwrap(),
            "docs/security/"
        );
    }
}
