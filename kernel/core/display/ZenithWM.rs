#![no_std]
#![allow(dead_code)]

/// SigmaOS Zenith Window Manager (ZenithWM)
/// A static, master-stack tiling window manager stub for no_std environments.

const MAX_WINDOWS: usize = 32;

#[derive(Copy, Clone, PartialEq)]
pub enum Layout {
    MasterStack,
    Fullscreen,
    Floating,
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

#[derive(Copy, Clone)]
pub struct Window {
    pub surface_idx: i32,
    pub active: bool,
    pub rect: Rect,
}

pub struct ZenithWindowManager {
    windows: [Window; MAX_WINDOWS],
    window_count: usize,
    layout: Layout,
    screen_rect: Rect,
    master_ratio: f32,
}

impl ZenithWindowManager {
    pub const fn new() -> Self {
        let empty_window = Window {
            surface_idx: -1,
            active: false,
            rect: Rect { x: 0, y: 0, w: 0, h: 0 },
        };
        Self {
            windows: [empty_window; MAX_WINDOWS],
            window_count: 0,
            layout: Layout::MasterStack,
            screen_rect: Rect { x: 0, y: 0, w: 1920, h: 1080 },
            master_ratio: 0.5, // Master takes 50% width
        }
    }

    pub fn set_screen(&mut self, w: u32, h: u32) {
        self.screen_rect.w = w;
        self.screen_rect.h = h;
        self.arrange();
    }

    pub fn manage_surface(&mut self, surface_idx: i32) -> Result<(), &'static str> {
        if self.window_count >= MAX_WINDOWS {
            return Err("Max windows reached");
        }

        for win in self.windows.iter_mut() {
            if !win.active {
                win.active = true;
                win.surface_idx = surface_idx;
                self.window_count += 1;
                self.arrange();
                return Ok(());
            }
        }
        Err("Could not find slot")
    }

    pub fn unmanage_surface(&mut self, surface_idx: i32) {
        for win in self.windows.iter_mut() {
            if win.active && win.surface_idx == surface_idx {
                win.active = false;
                win.surface_idx = -1;
                self.window_count -= 1;
                self.arrange();
                break;
            }
        }
    }

    /// Calculate standard Master-Stack layout rectangles
    pub fn arrange(&mut self) {
        if self.window_count == 0 { return; }

        if self.layout == Layout::Fullscreen {
            for win in self.windows.iter_mut() {
                if win.active {
                    win.rect = self.screen_rect;
                }
            }
            return;
        }

        if self.layout == Layout::MasterStack {
            let mut active_indices = [0usize; MAX_WINDOWS];
            let mut count = 0;
            
            for (i, win) in self.windows.iter().enumerate() {
                if win.active {
                    active_indices[count] = i;
                    count += 1;
                }
            }

            if count == 1 {
                self.windows[active_indices[0]].rect = self.screen_rect;
                return;
            }

            // Calculate Master Rect
            let master_w = (self.screen_rect.w as f32 * self.master_ratio) as u32;
            self.windows[active_indices[0]].rect = Rect {
                x: self.screen_rect.x,
                y: self.screen_rect.y,
                w: master_w,
                h: self.screen_rect.h,
            };

            // Calculate Stack Rects
            let stack_count = count - 1;
            let stack_x = self.screen_rect.x + master_w as i32;
            let stack_w = self.screen_rect.w - master_w;
            let stack_h = self.screen_rect.h / (stack_count as u32);

            for i in 1..count {
                self.windows[active_indices[i]].rect = Rect {
                    x: stack_x,
                    y: self.screen_rect.y + (stack_h * (i - 1) as u32) as i32,
                    w: stack_w,
                    h: stack_h,
                };
            }
        }
    }
}

static mut G_ZENITH_WM: ZenithWindowManager = ZenithWindowManager::new();

#[no_mangle]
pub unsafe extern "C" fn zenith_wm_manage(surface_idx: i32) {
    let _ = G_ZENITH_WM.manage_surface(surface_idx);
}

#[no_mangle]
pub unsafe extern "C" fn zenith_wm_unmanage(surface_idx: i32) {
    G_ZENITH_WM.unmanage_surface(surface_idx);
}
