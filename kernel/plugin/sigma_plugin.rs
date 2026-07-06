//! SigmaOS Plugin Architecture
//! Extensible plugin system for kernel and userland components
//! Inspired by Linux kernel modules, Firefox extensions, VS Code extensions

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

/// Plugin type
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PluginType {
    KernelDriver = 0,
    Filesystem = 1,
    Network = 2,
    Desktop = 3,
    Security = 4,
    AI = 5,
    Application = 6,
}

/// Plugin state
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PluginState {
    Unloaded = 0,
    Loaded = 1,
    Initialized = 2,
    Running = 3,
    Error = 4,
}

/// Plugin capability
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PluginCapability {
    Read = 0,
    Write = 1,
    Execute = 2,
    Network = 3,
    Hardware = 4,
    System = 5,
}

/// Plugin metadata
#[repr(C)]
pub struct PluginMetadata {
    pub plugin_id: SigmaU64,
    pub name: [SigmaU8; 64],
    pub version: [SigmaU8; 32],
    pub author: [SigmaU8; 64],
    pub description: [SigmaU8; 256],
    pub plugin_type: PluginType,
    pub api_version: SigmaU32,
    pub min_os_version: SigmaU32,
}

/// Plugin dependency
#[repr(C)]
pub struct PluginDependency {
    pub plugin_id: SigmaU64,
    pub min_version: SigmaU32,
}

/// Plugin
#[repr(C)]
pub struct Plugin {
    pub metadata: PluginMetadata,
    pub state: PluginState,
    pub dependencies: [PluginDependency; 16],
    pub dependency_count: SigmaU32,
    pub capabilities: [PluginCapability; 8],
    pub capability_count: SigmaU32,
    pub load_time: SigmaI64,
    pub init_fn: SigmaU64,
    pub cleanup_fn: SigmaU64,
}

/// Plugin manager
#[repr(C)]
pub struct PluginManager {
    pub initialized: SigmaBool,
    pub plugins: [Plugin; 128],
    pub plugin_count: SigmaU32,
    pub active_plugins: SigmaU32,
    pub api_version: SigmaU32,
    pub os_version: SigmaU32,
}

static mut PLUGIN_MANAGER: Option<PluginManager> = None;

/// Initialize plugin manager
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_init(api_version: SigmaU32, os_version: SigmaU32) -> SigmaI32 {
    PLUGIN_MANAGER = Some(PluginManager {
        initialized: false,
        plugins: [Plugin {
            metadata: PluginMetadata {
                plugin_id: 0,
                name: [0; 64],
                version: [0; 32],
                author: [0; 64],
                description: [0; 256],
                plugin_type: PluginType::KernelDriver,
                api_version: 0,
                min_os_version: 0,
            },
            state: PluginState::Unloaded,
            dependencies: [PluginDependency {
                plugin_id: 0,
                min_version: 0,
            }; 16],
            dependency_count: 0,
            capabilities: [PluginCapability::Read; 8],
            capability_count: 0,
            load_time: 0,
            init_fn: 0,
            cleanup_fn: 0,
        }; 128],
        plugin_count: 0,
        active_plugins: 0,
        api_version,
        os_version,
    });

    if let Some(manager) = &mut PLUGIN_MANAGER {
        manager.initialized = true;
        return 0;
    }

    -1
}

/// Register plugin
#[no_mangle]
pub unsafe extern "C" fn plugin_register(
    name: *const SigmaU8,
    version: *const SigmaU8,
    author: *const SigmaU8,
    description: *const SigmaU8,
    plugin_type: PluginType,
    api_version: SigmaU32,
    min_os_version: SigmaU32,
) -> SigmaU64 {
    if PLUGIN_MANAGER.is_none() || name.is_null() {
        return 0;
    }

    if let Some(manager) = &mut PLUGIN_MANAGER {
        if manager.plugin_count >= 128 {
            return 0;
        }

        let idx = manager.plugin_count as usize;
        let plugin_id = manager.plugin_count as SigmaU64 + 1;

        manager.plugins[idx] = Plugin {
            metadata: PluginMetadata {
                plugin_id,
                name: [0; 64],
                version: [0; 32],
                author: [0; 64],
                description: [0; 256],
                plugin_type,
                api_version,
                min_os_version,
            },
            state: PluginState::Unloaded,
            dependencies: [PluginDependency {
                plugin_id: 0,
                min_version: 0,
            }; 16],
            dependency_count: 0,
            capabilities: [PluginCapability::Read; 8],
            capability_count: 0,
            load_time: 0,
            init_fn: 0,
            cleanup_fn: 0,
        };

        // Copy name
        for i in 0..63.min(name_len(name)) {
            manager.plugins[idx].metadata.name[i] = *name.add(i);
        }

        // Copy version
        if !version.is_null() {
            for i in 0..31.min(name_len(version)) {
                manager.plugins[idx].metadata.version[i] = *version.add(i);
            }
        }

        // Copy author
        if !author.is_null() {
            for i in 0..63.min(name_len(author)) {
                manager.plugins[idx].metadata.author[i] = *author.add(i);
            }
        }

        // Copy description
        if !description.is_null() {
            for i in 0..255.min(name_len(description)) {
                manager.plugins[idx].metadata.description[i] = *description.add(i);
            }
        }

        manager.plugin_count += 1;
        plugin_id
    } else {
        0
    }
}

/// Load plugin
#[no_mangle]
pub unsafe extern "C" fn plugin_load(plugin_id: SigmaU64) -> SigmaI32 {
    if PLUGIN_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut PLUGIN_MANAGER {
        for i in 0..manager.plugin_count as usize {
            if manager.plugins[i].metadata.plugin_id == plugin_id {
                // Check API version compatibility
                if manager.plugins[i].metadata.api_version != manager.api_version {
                    manager.plugins[i].state = PluginState::Error;
                    return -2;
                }

                // Check OS version compatibility
                if manager.plugins[i].metadata.min_os_version > manager.os_version {
                    manager.plugins[i].state = PluginState::Error;
                    return -3;
                }

                // Check dependencies
                for j in 0..manager.plugins[i].dependency_count as usize {
                    let dep = manager.plugins[i].dependencies[j];
                    let dep_loaded = is_plugin_loaded(manager, dep.plugin_id);
                    if !dep_loaded {
                        return -4;
                    }
                }

                // Load plugin
                manager.plugins[i].state = PluginState::Loaded;
                manager.plugins[i].load_time = get_timestamp();
                return 0;
            }
        }
    }

    -1
}

/// Initialize plugin
#[no_mangle]
pub unsafe extern "C" fn plugin_initialize(plugin_id: SigmaU64) -> SigmaI32 {
    if PLUGIN_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut PLUGIN_MANAGER {
        for i in 0..manager.plugin_count as usize {
            if manager.plugins[i].metadata.plugin_id == plugin_id {
                if manager.plugins[i].state != PluginState::Loaded {
                    return -2;
                }

                // Call init function if available
                if manager.plugins[i].init_fn != 0 {
                    // In real implementation, call the function pointer
                }

                manager.plugins[i].state = PluginState::Initialized;
                manager.active_plugins += 1;
                return 0;
            }
        }
    }

    -1
}

/// Unload plugin
#[no_mangle]
pub unsafe extern "C" fn plugin_unload(plugin_id: SigmaU64) -> SigmaI32 {
    if PLUGIN_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut PLUGIN_MANAGER {
        for i in 0..manager.plugin_count as usize {
            if manager.plugins[i].metadata.plugin_id == plugin_id {
                // Check if other plugins depend on this one
                for j in 0..manager.plugin_count as usize {
                    if i != j {
                        for k in 0..manager.plugins[j].dependency_count as usize {
                            if manager.plugins[j].dependencies[k].plugin_id == plugin_id {
                                return -2; // Plugin is in use
                            }
                        }
                    }
                }

                // Call cleanup function if available
                if manager.plugins[i].cleanup_fn != 0 {
                    // In real implementation, call the function pointer
                }

                manager.plugins[i].state = PluginState::Unloaded;
                if manager.active_plugins > 0 {
                    manager.active_plugins -= 1;
                }
                return 0;
            }
        }
    }

    -1
}

/// Add dependency
#[no_mangle]
pub unsafe extern "C" fn plugin_add_dependency(
    plugin_id: SigmaU64,
    dep_plugin_id: SigmaU64,
    min_version: SigmaU32,
) -> SigmaI32 {
    if PLUGIN_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut PLUGIN_MANAGER {
        for i in 0..manager.plugin_count as usize {
            if manager.plugins[i].metadata.plugin_id == plugin_id {
                if manager.plugins[i].dependency_count >= 16 {
                    return -2;
                }

                let idx = manager.plugins[i].dependency_count as usize;
                manager.plugins[i].dependencies[idx] = PluginDependency {
                    plugin_id: dep_plugin_id,
                    min_version,
                };
                manager.plugins[i].dependency_count += 1;
                return 0;
            }
        }
    }

    -1
}

/// Add capability
#[no_mangle]
pub unsafe extern "C" fn plugin_add_capability(
    plugin_id: SigmaU64,
    capability: PluginCapability,
) -> SigmaI32 {
    if PLUGIN_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut PLUGIN_MANAGER {
        for i in 0..manager.plugin_count as usize {
            if manager.plugins[i].metadata.plugin_id == plugin_id {
                if manager.plugins[i].capability_count >= 8 {
                    return -2;
                }

                let idx = manager.plugins[i].capability_count as usize;
                manager.plugins[i].capabilities[idx] = capability;
                manager.plugins[i].capability_count += 1;
                return 0;
            }
        }
    }

    -1
}

/// Get plugin state
#[no_mangle]
pub unsafe extern "C" fn plugin_get_state(plugin_id: SigmaU64) -> PluginState {
    if let Some(manager) = &PLUGIN_MANAGER {
        for i in 0..manager.plugin_count as usize {
            if manager.plugins[i].metadata.plugin_id == plugin_id {
                return manager.plugins[i].state;
            }
        }
    }
    PluginState::Unloaded
}

/// Get plugin count
#[no_mangle]
pub unsafe extern "C" fn plugin_count() -> SigmaU32 {
    if let Some(manager) = &PLUGIN_MANAGER {
        manager.plugin_count
    } else {
        0
    }
}

/// Get active plugin count
#[no_mangle]
pub unsafe extern "C" fn plugin_active_count() -> SigmaU32 {
    if let Some(manager) = &PLUGIN_MANAGER {
        manager.active_plugins
    } else {
        0
    }
}

/// Helper: Check if plugin is loaded
unsafe fn is_plugin_loaded(manager: &PluginManager, plugin_id: SigmaU64) -> SigmaBool {
    for i in 0..manager.plugin_count as usize {
        if manager.plugins[i].metadata.plugin_id == plugin_id {
            return manager.plugins[i].state == PluginState::Loaded ||
                   manager.plugins[i].state == PluginState::Initialized ||
                   manager.plugins[i].state == PluginState::Running;
        }
    }
    false
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

/// Helper: Get current timestamp
unsafe fn get_timestamp() -> SigmaI64 {
    0
}

/// Check if plugin manager is initialized
#[no_mangle]
pub unsafe extern "C" fn plugin_manager_initialized() -> SigmaBool {
    if let Some(manager) = &PLUGIN_MANAGER {
        manager.initialized
    } else {
        false
    }
}
