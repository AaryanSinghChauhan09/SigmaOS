//! SigmaOS LXQt Desktop Environment Integration
//! LXQt 1.2+ lightweight Qt-based desktop environment with SigmaOS customizations
//! Inspired by Lubuntu, Fedora LXQt Spin

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

/// LXQt panel position
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PanelPosition {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,
}

/// LXQt widget type
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WidgetType {
    Menu = 0,
    QuickLaunch = 1,
    TaskBar = 2,
    Tray = 3,
    Clock = 4,
    Spacer = 5,
    StatusNotifier = 6,
}

/// LXQt configuration
#[repr(C)]
pub struct LxQtConfig {
    pub panel_position: PanelPosition,
    pub panel_size: SigmaU32,
    pub panel_autohide: SigmaBool,
    pub icon_theme: [SigmaU8; 64],
    pub widget_style: [SigmaU8; 64],
    pub workspace_count: SigmaU32,
}

/// LXQt widget
#[repr(C)]
pub struct LxQtWidget {
    pub widget_type: WidgetType,
    pub name: [SigmaU8; 64],
    pub enabled: SigmaBool,
    pub position: SigmaU32,
}

/// LXQt desktop manager
#[repr(C)]
pub struct LxQtDesktop {
    pub initialized: SigmaBool,
    pub config: LxQtConfig,
    pub widgets: [LxQtWidget; 32],
    pub widget_count: SigmaU32,
    pub lxqt_version: [SigmaU8; 32],
}

static mut LXQT_DESKTOP: Option<LxQtDesktop> = None;

/// Initialize LXQt desktop
#[no_mangle]
pub unsafe extern "C" fn lxqt_desktop_init() -> SigmaI32 {
    LXQT_DESKTOP = Some(LxQtDesktop {
        initialized: false,
        config: LxQtConfig {
            panel_position: PanelPosition::Bottom,
            panel_size: 32,
            panel_autohide: false,
            icon_theme: [0; 64],
            widget_style: [0; 64],
            workspace_count: 4,
        },
        widgets: [LxQtWidget {
            widget_type: WidgetType::Menu,
            name: [0; 64],
            enabled: false,
            position: 0,
        }; 32],
        widget_count: 0,
        lxqt_version: [0; 32],
    });

    if let Some(desktop) = &mut LXQT_DESKTOP {
        // Set LXQt version
        let version = b"1.2\0";
        for i in 0..version.len().min(32) {
            desktop.lxqt_version[i] = version[i];
        }
        
        // Set default icon theme
        let theme = b"oxygen-icons\0";
        for i in 0..theme.len().min(64) {
            desktop.config.icon_theme[i] = theme[i];
        }
        
        // Set default widget style
        let style = b"Fusion\0";
        for i in 0..style.len().min(64) {
            desktop.config.widget_style[i] = style[i];
        }
        
        // Load default widgets
        load_default_widgets(desktop);
        
        desktop.initialized = true;
        return 0;
    }

    -1
}

/// Load default widgets
unsafe fn load_default_widgets(desktop: &mut LxQtDesktop) {
    // Add menu widget
    if desktop.widget_count < 32 {
        let idx = desktop.widget_count as usize;
        desktop.widgets[idx] = LxQtWidget {
            widget_type: WidgetType::Menu,
            name: [0; 64],
            enabled: true,
            position: 0,
        };
        
        let name = b"Application Menu\0";
        for i in 0..name.len().min(64) {
            desktop.widgets[idx].name[i] = name[i];
        }
        
        desktop.widget_count += 1;
    }

    // Add quick launch widget
    if desktop.widget_count < 32 {
        let idx = desktop.widget_count as usize;
        desktop.widgets[idx] = LxQtWidget {
            widget_type: WidgetType::QuickLaunch,
            name: [0; 64],
            enabled: true,
            position: 1,
        };
        
        let name = b"Quick Launch\0";
        for i in 0..name.len().min(64) {
            desktop.widgets[idx].name[i] = name[i];
        }
        
        desktop.widget_count += 1;
    }

    // Add task bar widget
    if desktop.widget_count < 32 {
        let idx = desktop.widget_count as usize;
        desktop.widgets[idx] = LxQtWidget {
            widget_type: WidgetType::TaskBar,
            name: [0; 64],
            enabled: true,
            position: 2,
        };
        
        let name = b"Task Bar\0";
        for i in 0..name.len().min(64) {
            desktop.widgets[idx].name[i] = name[i];
        }
        
        desktop.widget_count += 1;
    }

    // Add tray widget
    if desktop.widget_count < 32 {
        let idx = desktop.widget_count as usize;
        desktop.widgets[idx] = LxQtWidget {
            widget_type: WidgetType::Tray,
            name: [0; 64],
            enabled: true,
            position: 3,
        };
        
        let name = b"System Tray\0";
        for i in 0..name.len().min(64) {
            desktop.widgets[idx].name[i] = name[i];
        }
        
        desktop.widget_count += 1;
    }

    // Add clock widget
    if desktop.widget_count < 32 {
        let idx = desktop.widget_count as usize;
        desktop.widgets[idx] = LxQtWidget {
            widget_type: WidgetType::Clock,
            name: [0; 64],
            enabled: true,
            position: 4,
        };
        
        let name = b"World Clock\0";
        for i in 0..name.len().min(64) {
            desktop.widgets[idx].name[i] = name[i];
        }
        
        desktop.widget_count += 1;
    }
}

/// Set panel position
#[no_mangle]
pub unsafe extern "C" fn lxqt_set_panel_position(position: PanelPosition) -> SigmaI32 {
    if LXQT_DESKTOP.is_none() {
        return -1;
    }

    if let Some(desktop) = &mut LXQT_DESKTOP {
        desktop.config.panel_position = position;
        return 0;
    }

    -1
}

/// Set panel size
#[no_mangle]
pub unsafe extern "C" fn lxqt_set_panel_size(size: SigmaU32) -> SigmaI32 {
    if LXQT_DESKTOP.is_none() {
        return -1;
    }

    if let Some(desktop) = &mut LXQT_DESKTOP {
        desktop.config.panel_size = size;
        return 0;
    }

    -1
}

/// Enable/disable panel autohide
#[no_mangle]
pub unsafe extern "C" fn lxqt_set_panel_autohide(enabled: SigmaBool) -> SigmaI32 {
    if LXQT_DESKTOP.is_none() {
        return -1;
    }

    if let Some(desktop) = &mut LXQT_DESKTOP {
        desktop.config.panel_autohide = enabled;
        return 0;
    }

    -1
}

/// Set icon theme
#[no_mangle]
pub unsafe extern "C" fn lxqt_set_icon_theme(theme: *const SigmaU8) -> SigmaI32 {
    if LXQT_DESKTOP.is_none() || theme.is_null() {
        return -1;
    }

    if let Some(desktop) = &mut LXQT_DESKTOP {
        for i in 0..63.min(name_len(theme)) {
            desktop.config.icon_theme[i] = *theme.add(i);
        }
        return 0;
    }

    -1
}

/// Set widget style
#[no_mangle]
pub unsafe extern "C" fn lxqt_set_widget_style(style: *const SigmaU8) -> SigmaI32 {
    if LXQT_DESKTOP.is_none() || style.is_null() {
        return -1;
    }

    if let Some(desktop) = &mut LXQT_DESKTOP {
        for i in 0..63.min(name_len(style)) {
            desktop.config.widget_style[i] = *style.add(i);
        }
        return 0;
    }

    -1
}

/// Set workspace count
#[no_mangle]
pub unsafe extern "C" fn lxqt_set_workspace_count(count: SigmaU32) -> SigmaI32 {
    if LXQT_DESKTOP.is_none() {
        return -1;
    }

    if let Some(desktop) = &mut LXQT_DESKTOP {
        desktop.config.workspace_count = count;
        return 0;
    }

    -1
}

/// Add widget
#[no_mangle]
pub unsafe extern "C" fn lxqt_add_widget(
    widget_type: WidgetType,
    name: *const SigmaU8,
    position: SigmaU32,
) -> SigmaI32 {
    if LXQT_DESKTOP.is_none() || name.is_null() {
        return -1;
    }

    if let Some(desktop) = &mut LXQT_DESKTOP {
        if desktop.widget_count >= 32 {
            return -1;
        }

        let idx = desktop.widget_count as usize;

        desktop.widgets[idx] = LxQtWidget {
            widget_type,
            name: [0; 64],
            enabled: true,
            position,
        };

        // Copy name
        for i in 0..63.min(name_len(name)) {
            desktop.widgets[idx].name[i] = *name.add(i);
        }

        desktop.widget_count += 1;
        return 0;
    }

    -1
}

/// Remove widget
#[no_mangle]
pub unsafe extern "C" fn lxqt_remove_widget(name: *const SigmaU8) -> SigmaI32 {
    if LXQT_DESKTOP.is_none() || name.is_null() {
        return -1;
    }

    if let Some(desktop) = &mut LXQT_DESKTOP {
        for i in 0..desktop.widget_count as usize {
            if names_equal(desktop.widgets[i].name.as_ptr(), name) {
                // Remove by shifting
                for j in i..(desktop.widget_count as usize - 1) {
                    desktop.widgets[j] = desktop.widgets[j + 1];
                }
                desktop.widget_count -= 1;
                return 0;
            }
        }
    }

    -1
}

/// Enable/disable widget
#[no_mangle]
pub unsafe extern "C" fn lxqt_set_widget_enabled(name: *const SigmaU8, enabled: SigmaBool) -> SigmaI32 {
    if LXQT_DESKTOP.is_none() || name.is_null() {
        return -1;
    }

    if let Some(desktop) = &mut LXQT_DESKTOP {
        for i in 0..desktop.widget_count as usize {
            if names_equal(desktop.widgets[i].name.as_ptr(), name) {
                desktop.widgets[i].enabled = enabled;
                return 0;
            }
        }
    }

    -1
}

/// Get widget count
#[no_mangle]
pub unsafe extern "C" fn lxqt_widget_count() -> SigmaU32 {
    if let Some(desktop) = &LXQT_DESKTOP {
        desktop.widget_count
    } else {
        0
    }
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
    while *s.add(len) != 0 && len < 64 {
        len += 1;
    }
    len
}

/// Check if LXQt desktop is initialized
#[no_mangle]
pub unsafe extern "C" fn lxqt_desktop_initialized() -> SigmaBool {
    if let Some(desktop) = &LXQT_DESKTOP {
        desktop.initialized
    } else {
        false
    }
}
