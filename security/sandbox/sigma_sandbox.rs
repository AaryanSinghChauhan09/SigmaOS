//! SigmaOS QubesOS-Style Sandboxing
//! Domain-based security isolation for applications
//! Inspired by QubesOS, Flatpak, and Firejail

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

/// Sandbox domain type
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DomainType {
    Dom0 = 0,        // Admin domain
    Work = 1,         // Work domain
    Personal = 2,     // Personal domain
    Untrusted = 3,    // Untrusted domain
    Vault = 4,        // Vault domain (high security)
    Disposable = 5,   // Disposable domain
}

/// Sandbox policy
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SandboxPolicy {
    AllowAll = 0,
    DenyAll = 1,
    Whitelist = 2,
    Blacklist = 3,
}

/// Resource type
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResourceType {
    Network = 0,
    Filesystem = 1,
    Device = 2,
    IPC = 3,
    Process = 4,
}

/// Sandbox domain
#[repr(C)]
pub struct SandboxDomain {
    pub domain_id: SigmaU64,
    pub domain_type: DomainType,
    pub name: [SigmaU8; 64],
    pub network_enabled: SigmaBool,
    pub filesystem_policy: SandboxPolicy,
    pub device_policy: SandboxPolicy,
    pub ipc_policy: SandboxPolicy,
    pub process_count: SigmaU32,
    pub memory_limit: SigmaU64,
    pub cpu_limit: SigmaU32,
}

/// Sandbox rule
#[repr(C)]
pub struct SandboxRule {
    pub resource_type: ResourceType,
    pub allowed: SigmaBool,
    pub path: [SigmaU8; 256],
    pub port: SigmaU16,
}

/// Sandbox manager
#[repr(C)]
pub struct SandboxManager {
    pub initialized: SigmaBool,
    pub domains: [SandboxDomain; 32],
    pub domain_count: SigmaU32,
    pub active_domain: SigmaU64,
    pub rules: [SandboxRule; 512],
    pub rule_count: SigmaU32,
    pub default_policy: SandboxPolicy,
}

static mut SANDBOX_MANAGER: Option<SandboxManager> = None;

/// Initialize sandbox manager
#[no_mangle]
pub unsafe extern "C" fn sandbox_manager_init() -> SigmaI32 {
    SANDBOX_MANAGER = Some(SandboxManager {
        initialized: false,
        domains: [SandboxDomain {
            domain_id: 0,
            domain_type: DomainType::Dom0,
            name: [0; 64],
            network_enabled: true,
            filesystem_policy: SandboxPolicy::AllowAll,
            device_policy: SandboxPolicy::AllowAll,
            ipc_policy: SandboxPolicy::AllowAll,
            process_count: 0,
            memory_limit: 0,
            cpu_limit: 0,
        }; 32],
        domain_count: 0,
        active_domain: 0,
        rules: [SandboxRule {
            resource_type: ResourceType::Network,
            allowed: false,
            path: [0; 256],
            port: 0,
        }; 512],
        rule_count: 0,
        default_policy: SandboxPolicy::Whitelist,
    });

    if let Some(manager) = &mut SANDBOX_MANAGER {
        // Create default domains
        create_default_domains(manager);
        
        manager.initialized = true;
        return 0;
    }

    -1
}

/// Create default domains
unsafe fn create_default_domains(manager: &mut SandboxManager) {
    // Create Dom0 (admin)
    if manager.domain_count < 32 {
        let idx = manager.domain_count as usize;
        manager.domains[idx] = SandboxDomain {
            domain_id: manager.domain_count as SigmaU64 + 1,
            domain_type: DomainType::Dom0,
            name: [0; 64],
            network_enabled: true,
            filesystem_policy: SandboxPolicy::AllowAll,
            device_policy: SandboxPolicy::AllowAll,
            ipc_policy: SandboxPolicy::AllowAll,
            process_count: 0,
            memory_limit: 0,
            cpu_limit: 0,
        };
        
        let name = b"dom0\0";
        for i in 0..name.len().min(64) {
            manager.domains[idx].name[i] = name[i];
        }
        
        manager.domain_count += 1;
        manager.active_domain = 1;
    }

    // Create Work domain
    if manager.domain_count < 32 {
        let idx = manager.domain_count as usize;
        manager.domains[idx] = SandboxDomain {
            domain_id: manager.domain_count as SigmaU64 + 1,
            domain_type: DomainType::Work,
            name: [0; 64],
            network_enabled: true,
            filesystem_policy: SandboxPolicy::Whitelist,
            device_policy: SandboxPolicy::Whitelist,
            ipc_policy: SandboxPolicy::Whitelist,
            process_count: 0,
            memory_limit: 4294967296, // 4GB
            cpu_limit: 50,
        };
        
        let name = b"work\0";
        for i in 0..name.len().min(64) {
            manager.domains[idx].name[i] = name[i];
        }
        
        manager.domain_count += 1;
    }

    // Create Untrusted domain
    if manager.domain_count < 32 {
        let idx = manager.domain_count as usize;
        manager.domains[idx] = SandboxDomain {
            domain_id: manager.domain_count as SigmaU64 + 1,
            domain_type: DomainType::Untrusted,
            name: [0; 64],
            network_enabled: false,
            filesystem_policy: SandboxPolicy::DenyAll,
            device_policy: SandboxPolicy::DenyAll,
            ipc_policy: SandboxPolicy::DenyAll,
            process_count: 0,
            memory_limit: 1073741824, // 1GB
            cpu_limit: 10,
        };
        
        let name = b"untrusted\0";
        for i in 0..name.len().min(64) {
            manager.domains[idx].name[i] = name[i];
        }
        
        manager.domain_count += 1;
    }
}

/// Create domain
#[no_mangle]
pub unsafe extern "C" fn sandbox_create_domain(
    name: *const SigmaU8,
    domain_type: DomainType,
    memory_limit: SigmaU64,
    cpu_limit: SigmaU32,
) -> SigmaU64 {
    if SANDBOX_MANAGER.is_none() || name.is_null() {
        return 0;
    }

    if let Some(manager) = &mut SANDBOX_MANAGER {
        if manager.domain_count >= 32 {
            return 0;
        }

        let idx = manager.domain_count as usize;
        let domain_id = manager.domain_count as SigmaU64 + 1;

        manager.domains[idx] = SandboxDomain {
            domain_id,
            domain_type,
            name: [0; 64],
            network_enabled: domain_type != DomainType::Untrusted,
            filesystem_policy: match domain_type {
                DomainType::Dom0 => SandboxPolicy::AllowAll,
                DomainType::Vault => SandboxPolicy::DenyAll,
                _ => SandboxPolicy::Whitelist,
            },
            device_policy: match domain_type {
                DomainType::Dom0 => SandboxPolicy::AllowAll,
                _ => SandboxPolicy::Whitelist,
            },
            ipc_policy: SandboxPolicy::Whitelist,
            process_count: 0,
            memory_limit,
            cpu_limit,
        };

        // Copy name
        for i in 0..63.min(name_len(name)) {
            manager.domains[idx].name[i] = *name.add(i);
        }

        manager.domain_count += 1;
        domain_id
    } else {
        0
    }
}

/// Switch domain
#[no_mangle]
pub unsafe extern "C" fn sandbox_switch_domain(domain_id: SigmaU64) -> SigmaI32 {
    if SANDBOX_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut SANDBOX_MANAGER {
        for i in 0..manager.domain_count as usize {
            if manager.domains[i].domain_id == domain_id {
                manager.active_domain = domain_id;
                return 0;
            }
        }
    }

    -1
}

/// Add sandbox rule
#[no_mangle]
pub unsafe extern "C" fn sandbox_add_rule(
    domain_id: SigmaU64,
    resource_type: ResourceType,
    allowed: SigmaBool,
    path: *const SigmaU8,
    port: SigmaU16,
) -> SigmaI32 {
    if SANDBOX_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut SANDBOX_MANAGER {
        if manager.rule_count >= 512 {
            return -1;
        }

        let idx = manager.rule_count as usize;

        manager.rules[idx] = SandboxRule {
            resource_type,
            allowed,
            path: [0; 256],
            port,
        };

        // Copy path
        if !path.is_null() {
            for i in 0..255.min(name_len(path)) {
                manager.rules[idx].path[i] = *path.add(i);
            }
        }

        manager.rule_count += 1;
        return 0;
    }

    -1
}

/// Set domain policy
#[no_mangle]
pub unsafe extern "C" fn sandbox_set_policy(
    domain_id: SigmaU64,
    resource_type: ResourceType,
    policy: SandboxPolicy,
) -> SigmaI32 {
    if SANDBOX_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut SANDBOX_MANAGER {
        for i in 0..manager.domain_count as usize {
            if manager.domains[i].domain_id == domain_id {
                match resource_type {
                    ResourceType::Filesystem => manager.domains[i].filesystem_policy = policy,
                    ResourceType::Device => manager.domains[i].device_policy = policy,
                    ResourceType::IPC => manager.domains[i].ipc_policy = policy,
                    _ => {}
                }
                return 0;
            }
        }
    }

    -1
}

/// Enable/disable network for domain
#[no_mangle]
pub unsafe extern "C" fn sandbox_set_network(
    domain_id: SigmaU64,
    enabled: SigmaBool,
) -> SigmaI32 {
    if SANDBOX_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut SANDBOX_MANAGER {
        for i in 0..manager.domain_count as usize {
            if manager.domains[i].domain_id == domain_id {
                manager.domains[i].network_enabled = enabled;
                return 0;
            }
        }
    }

    -1
}

/// Get domain count
#[no_mangle]
pub unsafe extern "C" fn sandbox_domain_count() -> SigmaU32 {
    if let Some(manager) = &SANDBOX_MANAGER {
        manager.domain_count
    } else {
        0
    }
}

/// Get active domain
#[no_mangle]
pub unsafe extern "C" fn sandbox_active_domain() -> SigmaU64 {
    if let Some(manager) = &SANDBOX_MANAGER {
        manager.active_domain
    } else {
        0
    }
}

/// Helper: Get string length
unsafe fn name_len(s: *const SigmaU8) -> usize {
    if s.is_null() {
        return 0;
    }
    let mut len = 0;
    while *s.add(len) != 0 && len < 256 {
        len += 1;
    }
    len
}

/// Check if sandbox manager is initialized
#[no_mangle]
pub unsafe extern "C" fn sandbox_manager_initialized() -> SigmaBool {
    if let Some(manager) = &SANDBOX_MANAGER {
        manager.initialized
    } else {
        false
    }
}
