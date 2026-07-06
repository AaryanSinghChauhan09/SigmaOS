//! SigmaOS Feature Flags System
//! Inspired by Gentoo Portage USE flags
//! Provides fine-grained control over package compilation and system configuration

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaUsize = usize;

/// Feature flag definition
#[repr(C)]
pub struct FeatureFlag {
    pub name: [SigmaU8; 64],
    pub description: [SigmaU8; 256],
    pub enabled: bool,
    pub global: bool,
    pub dependencies: [SigmaU64; 16],
    pub dep_count: SigmaU32,
    pub conflicts: [SigmaU64; 16],
    pub conflict_count: SigmaU32,
}

impl FeatureFlag {
    pub const fn empty() -> Self {
        Self {
            name: [0; 64],
            description: [0; 256],
            enabled: false,
            global: false,
            dependencies: [0; 16],
            dep_count: 0,
            conflicts: [0; 16],
            conflict_count: 0,
        }
    }
}

/// Feature flag manager
#[repr(C)]
pub struct FeatureFlagManager {
    pub flags: [FeatureFlag; 512],
    pub flag_count: SigmaU32,
    pub initialized: bool,
}

static mut FEATURE_MANAGER: Option<FeatureFlagManager> = None;

/// Initialize feature flag system
#[no_mangle]
pub unsafe extern "C" fn sigma_features_init() -> SigmaI32 {
    FEATURE_MANAGER = Some(FeatureFlagManager {
        flags: [FeatureFlag::empty(); 512],
        flag_count: 0,
        initialized: false,
    });

    if let Some(manager) = &mut FEATURE_MANAGER {
        // Add default global feature flags
        add_default_flags(manager);
        manager.initialized = true;
        return 0;
    }

    -1
}

/// Add default feature flags
unsafe fn add_default_flags(manager: &mut FeatureFlagManager) {
    // Bluetooth support
    add_flag(manager, b"bluetooth\0", b"Bluetooth support\0", false, true, &[], &[]);
    
    // D-Bus IPC system
    add_flag(manager, b"dbus\0", b"D-Bus IPC system\0", true, true, &[], &[]);
    
    // Wayland display server
    add_flag(manager, b"wayland\0", b"Wayland display server support\0", true, true, &[], &[b"wayland\0"]);
    
    // X11 display server
    add_flag(manager, b"x11\0", b"X11 display server support\0", false, true, &[], &[b"wayland\0"]);
    
    // Network support
    add_flag(manager, b"network\0", b"Network support\0", true, true, &[], &[]);
    
    // Sound support
    add_flag(manager, b"sound\0", b"Sound support\0", true, true, &[], &[]);
    
    // Graphics support
    add_flag(manager, b"graphics\0", b"Graphics support\0", true, true, &[], &[]);
    
    // Security features
    add_flag(manager, b"security\0", b"Security features\0", true, true, &[], &[]);
    
    // Debug symbols
    add_flag(manager, b"debug\0", b"Debug symbols\0", false, true, &[], &[]);
    
    // Optimization level
    add_flag(manager, b"optimize\0", b"Optimization level\0", true, true, &[], &[]);
}

/// Add a feature flag
unsafe fn add_flag(
    manager: &mut FeatureFlagManager,
    name: &[u8],
    description: &[u8],
    enabled: bool,
    global: bool,
    dependencies: &[&[u8]],
    conflicts: &[&[u8]],
) -> SigmaU64 {
    if manager.flag_count >= 512 {
        return 0;
    }

    let idx = manager.flag_count as usize;
    let flag = &mut manager.flags[idx];

    // Copy name
    for i in 0..name.len().min(63) {
        flag.name[i] = name[i];
    }
    flag.name[name.len().min(63)] = 0;

    // Copy description
    for i in 0..description.len().min(255) {
        flag.description[i] = description[i];
    }
    flag.description[description.len().min(255)] = 0;

    flag.enabled = enabled;
    flag.global = global;

    // Store dependencies (resolve names to IDs later)
    for i in 0..dependencies.len().min(16) {
        // Placeholder - will be resolved during dependency resolution
        flag.dependencies[i] = 0;
    }
    flag.dep_count = dependencies.len() as SigmaU32;

    // Store conflicts (resolve names to IDs later)
    for i in 0..conflicts.len().min(16) {
        // Placeholder - will be resolved during dependency resolution
        flag.conflicts[i] = 0;
    }
    flag.conflict_count = conflicts.len() as SigmaU32;

    let flag_id = manager.flag_count as SigmaU64 + 1;
    manager.flag_count += 1;
    flag_id
}

/// Find feature flag by name
#[no_mangle]
pub unsafe extern "C" fn sigma_features_find(name: *const SigmaU8) -> SigmaU64 {
    if FEATURE_MANAGER.is_none() || name.is_null() {
        return 0;
    }

    if let Some(manager) = &FEATURE_MANAGER {
        for i in 0..manager.flag_count as usize {
            let flag = &manager.flags[i];
            if names_equal(flag.name.as_ptr(), name) {
                return (i + 1) as SigmaU64;
            }
        }
    }

    0
}

/// Check if feature flag is enabled
#[no_mangle]
pub unsafe extern "C" fn sigma_features_enabled(flag_id: SigmaU64) -> bool {
    if FEATURE_MANAGER.is_none() || flag_id == 0 {
        return false;
    }

    if let Some(manager) = &FEATURE_MANAGER {
        let idx = (flag_id - 1) as usize;
        if idx < manager.flag_count as usize {
            return manager.flags[idx].enabled;
        }
    }

    false
}

/// Enable feature flag
#[no_mangle]
pub unsafe extern "C" fn sigma_features_enable(flag_id: SigmaU64) -> SigmaI32 {
    if FEATURE_MANAGER.is_none() || flag_id == 0 {
        return -1;
    }

    if let Some(manager) = &mut FEATURE_MANAGER {
        let idx = (flag_id - 1) as usize;
        if idx < manager.flag_count as usize {
            manager.flags[idx].enabled = true;
            return 0;
        }
    }

    -1
}

/// Disable feature flag
#[no_mangle]
pub unsafe extern "C" fn sigma_features_disable(flag_id: SigmaU64) -> SigmaI32 {
    if FEATURE_MANAGER.is_none() || flag_id == 0 {
        return -1;
    }

    if let Some(manager) = &mut FEATURE_MANAGER {
        let idx = (flag_id - 1) as usize;
        if idx < manager.flag_count as usize {
            manager.flags[idx].enabled = false;
            return 0;
        }
    }

    -1
}

/// Resolve feature flag dependencies
#[no_mangle]
pub unsafe extern "C" fn sigma_features_resolve() -> SigmaI32 {
    if FEATURE_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut FEATURE_MANAGER {
        // Resolve dependency names to IDs
        for i in 0..manager.flag_count as usize {
            let flag = &mut manager.flags[i];
            for j in 0..flag.dep_count as usize {
                // Resolve dependency name to ID
                // For now, placeholder
                flag.dependencies[j] = 0;
            }
            for j in 0..flag.conflict_count as usize {
                // Resolve conflict name to ID
                // For now, placeholder
                flag.conflicts[j] = 0;
            }
        }

        // Check for circular dependencies
        if has_circular_dependencies(manager) {
            return -1;
        }

        // Enable dependencies of enabled flags
        for i in 0..manager.flag_count as usize {
            if manager.flags[i].enabled {
                enable_dependencies(manager, i);
            }
        }

        // Disable conflicts of enabled flags
        for i in 0..manager.flag_count as usize {
            if manager.flags[i].enabled {
                disable_conflicts(manager, i);
            }
        }

        return 0;
    }

    -1
}

/// Check for circular dependencies
unsafe fn has_circular_dependencies(manager: &FeatureFlagManager) -> bool {
    // Simple DFS-based cycle detection
    let mut visited = [false; 512];
    let mut rec_stack = [false; 512];

    for i in 0..manager.flag_count as usize {
        if !visited[i] {
            if dfs_cycle_check(manager, i, &mut visited, &mut rec_stack) {
                return true;
            }
        }
    }

    false
}

unsafe fn dfs_cycle_check(
    manager: &FeatureFlagManager,
    idx: usize,
    visited: &mut [bool; 512],
    rec_stack: &mut [bool; 512],
) -> bool {
    if !visited[idx] {
        visited[idx] = true;
        rec_stack[idx] = true;

        let flag = &manager.flags[idx];
        for j in 0..flag.dep_count as usize {
            let dep_id = flag.dependencies[j] as usize;
            if dep_id > 0 && dep_id <= manager.flag_count as usize {
                let dep_idx = dep_id - 1;
                if !visited[dep_idx] {
                    if dfs_cycle_check(manager, dep_idx, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack[dep_idx] {
                    return true;
                }
            }
        }

        rec_stack[idx] = false;
        return false;
    }

    false
}

/// Enable all dependencies of一个 flag
unsafe fn enable_dependencies(manager: &mut FeatureFlagManager, idx: usize) {
    let flag = manager.flags[idx];
    for j in 0..flag.dep_count as usize {
        let dep_id = flag.dependencies[j] as usize;
        if dep_id > 0 && dep_id <= manager.flag_count as usize {
            let dep_idx = dep_id - 1;
            manager.flags[dep_idx].enabled = true;
            // Recursively enable dependencies
            enable_dependencies(manager, dep_idx);
        }
    }
}

/// Disable all conflicts of一个 flag
unsafe fn disable_conflicts(manager: &mut FeatureFlagManager, idx: usize) {
    let flag = manager.flags[idx];
    for j in 0..flag.conflict_count as usize {
        let conflict_id = flag.conflicts[j] as usize;
        if conflict_id > 0 && conflict_id <= manager.flag_count as usize {
            let conflict_idx = conflict_id - 1;
            manager.flags[conflict_idx].enabled = false;
        }
    }
}

/// Compare two null-terminated strings
unsafe fn names_equal(a: *const SigmaU8, b: *const SigmaU8) -> bool {
    let mut i = 0;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca == 0 && cb == 0 {
            return true;
        }
        if ca != cb {
            return false;
        }
        if ca == 0 || cb == 0 {
            return false;
        }
        i += 1;
    }
}

/// Get feature flag count
#[no_mangle]
pub unsafe extern "C" fn sigma_features_count() -> SigmaU32 {
    if let Some(manager) = &FEATURE_MANAGER {
        manager.flag_count
    } else {
        0
    }
}

/// Check if feature flag system is initialized
#[no_mangle]
pub unsafe extern "C" fn sigma_features_initialized() -> bool {
    if let Some(manager) = &FEATURE_MANAGER {
        manager.initialized
    } else {
        false
    }
}
