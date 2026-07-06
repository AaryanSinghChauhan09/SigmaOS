//! SigmaOS Theme Store and Extensions
//! Unified interface for theme management and desktop extensions
//! Inspired by GNOME Extensions, KDE Themes, and Plasma Addons

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaF32 = f32;
type SigmaF64 = f64;
type SigmaBool = bool;
type SigmaUsize = usize;

/// Theme type
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ThemeType {
    GTK = 0,
    Qt = 1,
    Icon = 2,
    Cursor = 3,
    Sound = 4,
    WindowDecoration = 5,
}

/// Extension type
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExtensionType {
    GNOME = 0,
    KDE = 1,
    XFCE = 2,
    LXQt = 3,
    Zenith = 4,
}

/// Extension state
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExtensionState {
    Disabled = 0,
    Enabled = 1,
    Error = 2,
    Incompatible = 3,
}

/// Theme
#[repr(C)]
pub struct Theme {
    pub id: SigmaU64,
    pub name: [SigmaU8; 128],
    pub author: [SigmaU8; 64],
    pub version: [SigmaU8; 32],
    pub theme_type: ThemeType,
    pub description: [SigmaU8; 512],
    pub preview_image: [SigmaU8; 512],
    pub installed: SigmaBool,
    pub active: SigmaBool,
}

/// Extension
#[repr(C)]
pub struct Extension {
    pub id: SigmaU64,
    pub name: [SigmaU8; 128],
    pub author: [SigmaU8; 64],
    pub version: [SigmaU8; 32],
    pub extension_type: ExtensionType,
    pub description: [SigmaU8; 512],
    pub uuid: [SigmaU8; 128],
    pub state: ExtensionState,
    pub auto_start: SigmaBool,
}

/// Extension preference
#[repr(C)]
pub struct ExtensionPreference {
    pub key: [SigmaU8; 128],
    pub value: [SigmaU8; 512],
    pub value_type: SigmaU32,
}

/// Theme store manager
#[repr(C)]
pub struct ThemeStoreManager {
    pub initialized: SigmaBool,
    pub themes: [Theme; 256],
    pub theme_count: SigmaU32,
    pub extensions: [Extension; 128],
    pub extension_count: SigmaU32,
    pub active_theme: SigmaU64,
    pub repository_url: [SigmaU8; 256],
}

static mut THEME_MANAGER: Option<ThemeStoreManager> = None;

/// Initialize theme store manager
#[no_mangle]
pub unsafe extern "C" fn theme_store_init(repository_url: *const SigmaU8) -> SigmaI32 {
    THEME_MANAGER = Some(ThemeStoreManager {
        initialized: false,
        themes: [Theme {
            id: 0,
            name: [0; 128],
            author: [0; 64],
            version: [0; 32],
            theme_type: ThemeType::GTK,
            description: [0; 512],
            preview_image: [0; 512],
            installed: false,
            active: false,
        }; 256],
        theme_count: 0,
        extensions: [Extension {
            id: 0,
            name: [0; 128],
            author: [0; 64],
            version: [0; 32],
            extension_type: ExtensionType::GNOME,
            description: [0; 512],
            uuid: [0; 128],
            state: ExtensionState::Disabled,
            auto_start: false,
        }; 128],
        extension_count: 0,
        active_theme: 0,
        repository_url: [0; 256],
    });

    if let Some(manager) = &mut THEME_MANAGER {
        // Copy repository URL
        if !repository_url.is_null() {
            for i in 0..255.min(name_len(repository_url)) {
                manager.repository_url[i] = *repository_url.add(i);
            }
        }
        
        // Load default themes
        load_default_themes(manager);
        
        // Load default extensions
        load_default_extensions(manager);
        
        manager.initialized = true;
        return 0;
    }

    -1
}

/// Load default themes
unsafe fn load_default_themes(manager: &mut ThemeStoreManager) {
    // Add Adwaita theme
    if manager.theme_count < 256 {
        let idx = manager.theme_count as usize;
        manager.themes[idx] = Theme {
            id: manager.theme_count as SigmaU64 + 1,
            name: [0; 128],
            author: [0; 64],
            version: [0; 32],
            theme_type: ThemeType::GTK,
            description: [0; 512],
            preview_image: [0; 512],
            installed: true,
            active: true,
        };
        
        let name = b"Adwaita\0";
        for i in 0..name.len().min(128) {
            manager.themes[idx].name[i] = name[i];
        }
        
        let author = b"GNOME Project\0";
        for i in 0..author.len().min(64) {
            manager.themes[idx].author[i] = author[i];
        }
        
        let version = b"44.0\0";
        for i in 0..version.len().min(32) {
            manager.themes[idx].version[i] = version[i];
        }
        
        manager.active_theme = manager.themes[idx].id;
        manager.theme_count += 1;
    }
}

/// Load default extensions
unsafe fn load_default_extensions(manager: &mut ThemeStoreManager) {
    // Add Dash to Dock extension
    if manager.extension_count < 128 {
        let idx = manager.extension_count as usize;
        manager.extensions[idx] = Extension {
            id: manager.extension_count as SigmaU64 + 1,
            name: [0; 128],
            author: [0; 64],
            version: [0; 32],
            extension_type: ExtensionType::GNOME,
            description: [0; 512],
            uuid: [0; 128],
            state: ExtensionState::Enabled,
            auto_start: true,
        };
        
        let name = b"Dash to Dock\0";
        for i in 0..name.len().min(128) {
            manager.extensions[idx].name[i] = name[i];
        }
        
        let uuid = b"dash-to-dock@micxgx.gmail.com\0";
        for i in 0..uuid.len().min(128) {
            manager.extensions[idx].uuid[i] = uuid[i];
        }
        
        manager.extension_count += 1;
    }
}

/// Install theme
#[no_mangle]
pub unsafe extern "C" fn theme_install(theme_id: SigmaU64) -> SigmaI32 {
    if THEME_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut THEME_MANAGER {
        for i in 0..manager.theme_count as usize {
            if manager.themes[i].id == theme_id {
                manager.themes[i].installed = true;
                return 0;
            }
        }
    }

    -1
}

/// Uninstall theme
#[no_mangle]
pub unsafe extern "C" fn theme_uninstall(theme_id: SigmaU64) -> SigmaI32 {
    if THEME_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) &mut THEME_MANAGER {
        for i in 0..manager.theme_count as usize {
            if manager.themes[i].id == theme_id {
                if manager.themes[i].active {
                    return -2; // Cannot uninstall active theme
                }
                manager.themes[i].installed = false;
                return 0;
            }
        }
    }

    -1
}

/// Set active theme
#[no_mangle]
pub unsafe extern "C" fn theme_set_active(theme_id: SigmaU64) -> SigmaI32 {
    if THEME_MANAGER.is_none() {
        return -1;
    }

    if let Some(manager) = &mut THEME_MANAGER {
        // Deactivate current theme
        for i in 0..manager.theme_count as usize {
            if manager.themes[i].id == manager.active_theme {
                manager.themes[i].active = false;
            }
        }
        
        // Activate new theme
        for i in 0..manager.theme_count as usize {
            if manager.themes[i].id == theme_id {
                if !manager.themes[i].installed {
                    return -2; // Theme not installed
                }
                manager.themes[i].active = true;
                manager.active_theme = theme_id;
                return 0;
            }
        }
    }

    -1
}

/// Get active theme
#[no_mangle]
pub unsafe extern "C" fn theme_get_active() -> SigmaU64 {
    if let Some(manager) = &THEME_MANAGER {
        manager.active_theme
    } else {
        0
    }
}

/// List themes
#[no_mangle]
pub unsafe extern "C" fn theme_list(
    themes: *mut Theme,
    max_themes: SigmaU32,
    count: *mut SigmaU32,
) -> SigmaI32 {
    if THEME_MANAGER.is_none() || themes.is_null() || count.is_null() {
        return -1;
    }

    if let Some(manager) = &THEME_MANAGER {
        let mut found: SigmaU32 = 0;
        for i in 0..manager.theme_count as usize {
            if found < max_themes {
                *themes.add(found as usize) = manager.themes[i];
                found += 1;
            }
        }
        *count = found;
        return 0;
    }

    -1
}

/// Search themes
#[no_mangle]
pub unsafe extern "C" fn theme_search(
    query: *const SigmaU8,
    results: *mut Theme,
    max_results: SigmaU32,
    count: *mut SigmaU32,
) -> SigmaI32 {
    if THEME_MANAGER.is_none() || query.is_null() || results.is_null() || count.is_null() {
        return -1;
    }

    if let Some(manager) = &THEME_MANAGER {
        let mut found: SigmaU32 = 0;
        for i in 0..manager.theme_count as usize {
            if found < max_results && contains(manager.themes[i].name.as_ptr(), query) {
                *results.add(found as usize) = manager.themes[i];
                found += 1;
            }
        }
        *count = found;
        return 0;
    }

    -1
}

/// Install extension
#[no_mangle]
pub unsafe extern "C" fn extension_install(uuid: *const SigmaU8) -> SigmaI32 {
    if THEME_MANAGER.is_none() || uuid.is_null() {
        return -1;
    }

    if let Some(manager) = &mut THEME_MANAGER {
        // Check if extension already exists
        for i in 0..manager.extension_count as usize {
            if names_equal(manager.extensions[i].uuid.as_ptr(), uuid) {
                return -2; // Already installed
            }
        }
        
        if manager.extension_count >= 128 {
            return -3;
        }
        
        let idx = manager.extension_count as usize;
        manager.extensions[idx] = Extension {
            id: manager.extension_count as SigmaU64 + 1,
            name: [0; 128],
            author: [0; 64],
            version: [0; 32],
            extension_type: ExtensionType::GNOME,
            description: [0; 512],
            uuid: [0; 128],
            state: ExtensionState::Disabled,
            auto_start: false,
        };
        
        // Copy UUID
        for i in 0..127.min(name_len(uuid)) {
            manager.extensions[idx].uuid[i] = *uuid.add(i);
        }
        
        manager.extension_count += 1;
        return 0;
    }

    -1
}

/// Uninstall extension
#[no_mangle]
pub unsafe extern "C" fn extension_uninstall(uuid: *const SigmaU8) -> SigmaI32 {
    if THEME_MANAGER.is_none() || uuid.is_null() {
        return -1;
    }

    if let Some(manager) = &mut THEME_MANAGER {
        for i in 0..manager.extension_count as usize {
            if names_equal(manager.extensions[i].uuid.as_ptr(), uuid) {
                if manager.extensions[i].state == ExtensionState::Enabled {
                    // Disable first
                    manager.extensions[i].state = ExtensionState::Disabled;
                }
                
                // Remove by shifting
                for j in i..(manager.extension_count as usize - 1) {
                    manager.extensions[j] = manager.extensions[j + 1];
                }
                manager.extension_count -= 1;
                return 0;
            }
        }
    }

    -1
}

/// Enable extension
#[no_mangle]
pub unsafe extern "C" fn extension_enable(uuid: *const SigmaU8) -> SigmaI32 {
    if THEME_MANAGER.is_none() || uuid.is_null() {
        return -1;
    }

    if let Some(manager) = &mut THEME_MANAGER {
        for i in 0..manager.extension_count as usize {
            if names_equal(manager.extensions[i].uuid.as_ptr(), uuid) {
                manager.extensions[i].state = ExtensionState::Enabled;
                return 0;
            }
        }
    }

    -1
}

/// Disable extension
#[no_mangle]
pub unsafe extern "C" fn extension_disable(uuid: *const SigmaU8) -> SigmaI32 {
    if THEME_MANAGER.is_none() || uuid.is_null() {
        return -1;
    }

    if let Some(manager) = &mut THEME_MANAGER {
        for i in 0..manager.extension_count as usize {
            if names_equal(manager.extensions[i].uuid.as_ptr(), uuid) {
                manager.extensions[i].state = ExtensionState::Disabled;
                return 0;
            }
        }
    }

    -1
}

/// List extensions
#[no_mangle]
pub unsafe extern "C" fn extension_list(
    extensions: *mut Extension,
    max_extensions: SigmaU32,
    count: *mut SigmaU32,
) -> SigmaI32 {
    if THEME_MANAGER.is_none() || extensions.is_null() || count.is_null() {
        return -1;
    }

    if let Some(manager) = &THEME_MANAGER {
        let mut found: SigmaU32 = 0;
        for i in 0..manager.extension_count as usize {
            if found < max_extensions {
                *extensions.add(found as usize) = manager.extensions[i];
                found += 1;
            }
        }
        *count = found;
        return 0;
    }

    -1
}

/// Get extension preferences
#[no_mangle]
pub unsafe extern "C" fn extension_get_preferences(
    uuid: *const SigmaU8,
    preferences: *mut ExtensionPreference,
    max_prefs: SigmaU32,
    count: *mut SigmaU32,
) -> SigmaI32 {
    if THEME_MANAGER.is_none() || uuid.is_null() || preferences.is_null() || count.is_null() {
        return -1;
    }

    if let Some(manager) = &THEME_MANAGER {
        // In real implementation, get extension preferences
        *count = 0;
        return 0;
    }

    -1
}

/// Set extension preference
#[no_mangle]
pub unsafe extern "C" fn extension_set_preference(
    uuid: *const SigmaU8,
    key: *const SigmaU8,
    value: *const SigmaU8,
) -> SigmaI32 {
    if THEME_MANAGER.is_none() || uuid.is_null() || key.is_null() || value.is_null() {
        return -1;
    }

    if let Some(manager) = &THEME_MANAGER {
        // In real implementation, set extension preference
        return 0;
    }

    -1
}

/// Get theme count
#[no_mangle]
pub unsafe extern "C" fn theme_count() -> SigmaU32 {
    if let Some(manager) = &THEME_MANAGER {
        manager.theme_count
    } else {
        0
    }
}

/// Get extension count
#[no_mangle]
pub unsafe extern "C" fn extension_count() -> SigmaU32 {
    if let Some(manager) = &THEME_MANAGER {
        manager.extension_count
    } else {
        0
    }
}

/// Helper: Check if string contains substring
unsafe fn contains(s: *const SigmaU8, substr: *const SigmaU8) -> SigmaBool {
    if s.is_null() || substr.is_null() {
        return false;
    }
    
    let s_len = name_len(s);
    let sub_len = name_len(substr);
    
    if sub_len > s_len {
        return false;
    }
    
    for i in 0..=(s_len - sub_len) {
        let mut match_found = true;
        for j in 0..sub_len {
            if *s.add(i + j) != *substr.add(j) {
                match_found = false;
                break;
            }
        }
        if match_found {
            return true;
        }
    }
    
    false
}

/// Helper: Compare two null-terminated strings
unsafe fn names_equal(a: *const SigmaU8, b: *const SigmaU8) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }
    
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

/// Helper: Get string length
unsafe fn name_len(s: *const SigmaU8) -> usize {
    if s.is_null() {
        return 0;
    }
    let mut len = 0;
    while *s.add(len) != 0 && len < 512 {
        len += 1;
    }
    len
}

/// Check if theme store is initialized
#[no_mangle]
pub unsafe extern "C" fn theme_store_initialized() -> SigmaBool {
    if let Some(manager) = &THEME_MANAGER {
        manager.initialized
    } else {
        false
    }
}
