/// SigmaOS: usr/ui/dash.rs
/// Floating Dock / Taskbar for Zenith Desktop.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaUsize = usize;

pub const MAX_DASH_APPS: SigmaUsize = 12;

#[derive(Copy, Clone)]
pub struct DashAppIcon {
    pub app_id: SigmaU32,
    pub icon_idx: SigmaU32,
    pub is_running: bool,
    pub is_focused: bool,
}

impl DashAppIcon {
    pub const fn empty() -> Self {
        DashAppIcon { app_id: 0, icon_idx: 0, is_running: false, is_focused: false }
    }
}

pub struct DashState {
    pub icons: [DashAppIcon; MAX_DASH_APPS],
    pub icon_count: SigmaUsize,
    pub width: SigmaU32,
    pub height: SigmaU32,
    pub x: SigmaU32,
    pub y: SigmaU32,
    pub auto_hide: bool,
}

static mut DASH: DashState = DashState {
    icons: [DashAppIcon::empty(); MAX_DASH_APPS],
    icon_count: 0,
    width: 600,
    height: 64,
    x: 0,
    y: 0,
    auto_hide: false,
};

#[no_mangle]
pub unsafe extern "C" fn dash_init() -> SigmaI32 {
    // Pin dash to bottom center
    DASH.y = 1080 - DASH.height - 10;
    DASH.x = (1920 - DASH.width) / 2;
    0
}

#[no_mangle]
pub unsafe extern "C" fn dash_add_app(app_id: SigmaU32, icon_idx: SigmaU32) -> SigmaI32 {
    if DASH.icon_count < MAX_DASH_APPS {
        let idx = DASH.icon_count;
        DASH.icons[idx].app_id = app_id;
        DASH.icons[idx].icon_idx = icon_idx;
        DASH.icons[idx].is_running = false;
        DASH.icon_count += 1;
        
        // Recalculate width
        DASH.width = (DASH.icon_count as u32) * 64 + 20;
        DASH.x = (1920 - DASH.width) / 2;
        return 0;
    }
    -12 // ENOMEM
}