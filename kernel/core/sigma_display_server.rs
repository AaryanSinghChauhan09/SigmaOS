// SigmaOS — Display Server / Wayland Compositor (Zenith)
// Sovereign implementation — frame buffer management, window manager, compositing
#![no_std]
#![allow(dead_code)]
use core::sync::atomic::{AtomicU32, Ordering};

// ─── Pixel / Color Types ─────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8); // R G B A

impl Rgba {
    pub const BLACK:       Rgba = Rgba(0,   0,   0,   255);
    pub const WHITE:       Rgba = Rgba(255, 255, 255, 255);
    pub const TRANSPARENT: Rgba = Rgba(0,   0,   0,   0);
    pub const SIGMA_BLUE:  Rgba = Rgba(0,   120, 215, 255);

    pub fn blend(&self, fg: Rgba) -> Rgba {
        let a = fg.3 as u32;
        let ia = 255 - a;
        Rgba(
            ((fg.0 as u32 * a + self.0 as u32 * ia) / 255) as u8,
            ((fg.1 as u32 * a + self.1 as u32 * ia) / 255) as u8,
            ((fg.2 as u32 * a + self.2 as u32 * ia) / 255) as u8,
            255,
        )
    }
    pub fn to_u32_argb(&self) -> u32 {
        ((self.3 as u32) << 24) | ((self.0 as u32) << 16) | ((self.1 as u32) << 8) | (self.2 as u32)
    }
}

// ─── Framebuffer ─────────────────────────────────────────────────────────────
pub const FB_MAX_WIDTH:  usize = 3840;
pub const FB_MAX_HEIGHT: usize = 2160;
pub const FB_BPP: usize = 4; // bytes per pixel (ARGB)

pub struct Framebuffer {
    pub base:   u64,  // physical address of framebuffer memory
    pub width:  u32,
    pub height: u32,
    pub stride: u32,  // bytes per row
    pub bpp:    u8,
}

impl Framebuffer {
    pub const fn new(base: u64, width: u32, height: u32) -> Self {
        Framebuffer { base, width, height, stride: width * FB_BPP as u32, bpp: 32 }
    }
    fn ptr(&self) -> *mut u32 { self.base as *mut u32 }

    pub fn put_pixel(&self, x: u32, y: u32, color: Rgba) {
        if x >= self.width || y >= self.height { return; }
        let off = (y * self.stride / FB_BPP as u32 + x) as isize;
        unsafe { self.ptr().offset(off).write_volatile(color.to_u32_argb()); }
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Rgba {
        if x >= self.width || y >= self.height { return Rgba::BLACK; }
        let off = (y * self.stride / FB_BPP as u32 + x) as isize;
        let v = unsafe { self.ptr().offset(off).read_volatile() };
        Rgba(((v >> 16) & 0xFF) as u8, ((v >> 8) & 0xFF) as u8, (v & 0xFF) as u8, ((v >> 24) & 0xFF) as u8)
    }
    pub fn fill_rect(&self, x: u32, y: u32, w: u32, h: u32, color: Rgba) {
        let cv = color.to_u32_argb();
        for row in y..y.saturating_add(h).min(self.height) {
            for col in x..x.saturating_add(w).min(self.width) {
                let off = (row * self.stride / FB_BPP as u32 + col) as isize;
                unsafe { self.ptr().offset(off).write_volatile(cv); }
            }
        }
    }
    pub fn clear(&self, color: Rgba) {
        self.fill_rect(0, 0, self.width, self.height, color);
    }
    pub fn copy_region(&self, src_x: u32, src_y: u32, dst_x: u32, dst_y: u32, w: u32, h: u32) {
        for row in 0..h {
            for col in 0..w {
                let c = self.get_pixel(src_x + col, src_y + row);
                self.put_pixel(dst_x + col, dst_y + row, c);
            }
        }
    }
    pub fn draw_hline(&self, x: u32, y: u32, len: u32, color: Rgba) {
        self.fill_rect(x, y, len, 1, color);
    }
    pub fn draw_vline(&self, x: u32, y: u32, len: u32, color: Rgba) {
        self.fill_rect(x, y, 1, len, color);
    }
    pub fn draw_rect_border(&self, x: u32, y: u32, w: u32, h: u32, thickness: u32, color: Rgba) {
        self.draw_hline(x, y, w, color);
        self.draw_hline(x, y + h - 1, w, color);
        self.draw_vline(x, y, h, color);
        self.draw_vline(x + w - 1, y, h, color);
        let _ = thickness;
    }
}

// ─── Window ──────────────────────────────────────────────────────────────────
pub const MAX_WINDOWS: usize = 64;
pub const TITLE_LEN:   usize = 64;

#[derive(Clone, Copy, PartialEq)]
pub enum WindowState { Normal, Minimized, Maximized, Fullscreen, Tiled }

#[derive(Clone, Copy)]
pub struct Window {
    pub id:       u32,
    pub x:        i32,
    pub y:        i32,
    pub width:    u32,
    pub height:   u32,
    pub z_order:  u32,
    pub state:    WindowState,
    pub focused:  bool,
    pub decorated: bool,
    pub title:    [u8; TITLE_LEN],
    pub title_len: u8,
    pub bg_color: Rgba,
    pub pid:      u32,
    pub visible:  bool,
    pub dirty:    bool,
}

impl Window {
    pub const fn new(id: u32, x: i32, y: i32, w: u32, h: u32) -> Self {
        Window {
            id, x, y, width: w, height: h,
            z_order: id, state: WindowState::Normal,
            focused: false, decorated: true,
            title: [0u8; TITLE_LEN], title_len: 0,
            bg_color: Rgba(40, 40, 40, 255),
            pid: 0, visible: true, dirty: true,
        }
    }
    pub fn set_title(&mut self, title: &[u8]) {
        let n = title.len().min(TITLE_LEN);
        self.title[..n].copy_from_slice(&title[..n]);
        self.title_len = n as u8;
        self.dirty = true;
    }
    pub fn contains(&self, mx: i32, my: i32) -> bool {
        mx >= self.x && my >= self.y &&
        mx < self.x + self.width as i32 &&
        my < self.y + self.height as i32
    }
}

// ─── Tiling Window Manager (master-stack layout) ─────────────────────────────
pub struct TilingWm {
    pub windows:      [Window; MAX_WINDOWS],
    pub n_windows:    usize,
    pub focused_id:   u32,
    pub fb_width:     u32,
    pub fb_height:    u32,
    pub master_ratio: f32,  // 0.0–1.0 fraction for master window
    pub gap:          u32,  // pixels between windows
    pub next_id:      AtomicU32,
}

impl TilingWm {
    pub const fn new(fb_width: u32, fb_height: u32) -> Self {
        const W: Window = Window::new(0, 0, 0, 0, 0);
        TilingWm {
            windows: [W; MAX_WINDOWS],
            n_windows: 0, focused_id: 0,
            fb_width, fb_height,
            master_ratio: 0.6,
            gap: 6,
            next_id: AtomicU32::new(1),
        }
    }

    pub fn create_window(&mut self, w: u32, h: u32, pid: u32) -> Option<u32> {
        if self.n_windows >= MAX_WINDOWS { return None; }
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.windows[self.n_windows] = Window::new(id, 0, 0, w, h);
        self.windows[self.n_windows].pid = pid;
        self.n_windows += 1;
        self.retile();
        Some(id)
    }

    pub fn destroy_window(&mut self, id: u32) {
        let pos = self.windows[..self.n_windows].iter().position(|w| w.id == id);
        if let Some(i) = pos {
            self.windows[i] = self.windows[self.n_windows - 1];
            self.n_windows -= 1;
            self.retile();
        }
    }

    pub fn focus(&mut self, id: u32) {
        for w in &mut self.windows[..self.n_windows] {
            w.focused = w.id == id;
        }
        self.focused_id = id;
    }

    /// Master-stack tiling: first window = large master, rest = stack on right.
    pub fn retile(&mut self) {
        let g = self.gap;
        let h = self.fb_height;
        let w = self.fb_width;
        let n = self.n_windows;
        if n == 0 { return; }

        if n == 1 {
            let win = &mut self.windows[0];
            win.x = g as i32; win.y = g as i32;
            win.width  = w.saturating_sub(g * 2);
            win.height = h.saturating_sub(g * 2);
            win.dirty  = true;
            return;
        }

        // Master on left
        let master_w = ((w as f32 * self.master_ratio) as u32).saturating_sub(g + g / 2);
        let stack_w  = w.saturating_sub(master_w + g * 3);
        let stack_h  = (h.saturating_sub(g * (n as u32))) / (n as u32 - 1);

        let m = &mut self.windows[0];
        m.x = g as i32; m.y = g as i32;
        m.width  = master_w;
        m.height = h.saturating_sub(g * 2);
        m.dirty  = true;

        let stack_x = (master_w + g * 2) as i32;
        for (i, win) in self.windows[1..self.n_windows].iter_mut().enumerate() {
            win.x = stack_x;
            win.y = (g + i as u32 * (stack_h + g)) as i32;
            win.width  = stack_w;
            win.height = stack_h;
            win.dirty  = true;
        }
    }

    /// Render all visible windows onto framebuffer.
    pub fn render(&mut self, fb: &Framebuffer) {
        // Dark background
        fb.clear(Rgba(18, 18, 18, 255));
        // Sort by z-order
        for i in 0..self.n_windows {
            let w = &self.windows[i];
            if !w.visible { continue; }
            let bx = w.x.max(0) as u32;
            let by = w.y.max(0) as u32;
            // Window background
            fb.fill_rect(bx, by, w.width, w.height, w.bg_color);
            // Title bar
            let tb_color = if w.focused { Rgba::SIGMA_BLUE } else { Rgba(60, 60, 60, 255) };
            fb.fill_rect(bx, by, w.width, 28, tb_color);
            // Title text (first 8 chars as block indicator — real impl uses font renderer)
            // Border
            let border_c = if w.focused { Rgba(0, 160, 255, 255) } else { Rgba(80, 80, 80, 255) };
            fb.draw_rect_border(bx, by, w.width, w.height, 2, border_c);
        }
    }

    /// Handle Super+{1,2,3} layout switch.
    pub fn set_layout(&mut self, layout: u8) {
        match layout {
            1 => self.master_ratio = 0.6,  // master-stack
            2 => self.master_ratio = 0.5,  // equal split
            3 => {
                // Full-screen focused window
                if let Some(w) = self.windows[..self.n_windows].iter_mut().find(|w| w.focused) {
                    w.state = WindowState::Fullscreen;
                    w.x = 0; w.y = 0;
                    w.width = self.fb_width;
                    w.height = self.fb_height;
                    w.dirty = true;
                }
                return;
            }
            _ => {}
        }
        self.retile();
    }

    /// Snap window to screen edge.
    pub fn snap_to_edge(&mut self, id: u32, edge: u8) {
        // edge: 0=left, 1=right, 2=top, 3=bottom, 4=maximize
        let w_screen = self.fb_width;
        let h_screen = self.fb_height;
        let g = self.gap;
        if let Some(win) = self.windows[..self.n_windows].iter_mut().find(|w| w.id == id) {
            match edge {
                0 => { win.x = g as i32; win.y = g as i32; win.width = w_screen/2 - g*2; win.height = h_screen - g*2; }
                1 => { win.x = (w_screen/2 + g) as i32; win.y = g as i32; win.width = w_screen/2 - g*2; win.height = h_screen - g*2; }
                2 => { win.x = g as i32; win.y = g as i32; win.width = w_screen - g*2; win.height = h_screen/2 - g*2; }
                3 => { win.x = g as i32; win.y = (h_screen/2 + g) as i32; win.width = w_screen - g*2; win.height = h_screen/2 - g*2; }
                4 => { win.x = 0; win.y = 0; win.width = w_screen; win.height = h_screen; win.state = WindowState::Maximized; }
                _ => {}
            }
            win.dirty = true;
        }
    }
}

// ─── Application Launcher ────────────────────────────────────────────────────
pub const MAX_APPS: usize = 256;
pub const APP_NAME_LEN: usize = 32;

#[derive(Clone, Copy)]
pub struct AppEntry {
    pub name: [u8; APP_NAME_LEN],
    pub nlen: u8,
    pub exec: [u8; 64],
    pub elen: u8,
    pub category: u8, // 0=System 1=Internet 2=Productivity 3=Games 4=Tools
}

impl AppEntry {
    pub const fn new() -> Self {
        AppEntry { name: [0u8; APP_NAME_LEN], nlen: 0, exec: [0u8; 64], elen: 0, category: 0 }
    }
}

pub struct AppLauncher {
    pub apps:  [AppEntry; MAX_APPS],
    pub count: usize,
    pub query: [u8; APP_NAME_LEN],
    pub qlen:  usize,
    pub visible: bool,
}

impl AppLauncher {
    pub const fn new() -> Self {
        const E: AppEntry = AppEntry::new();
        AppLauncher { apps: [E; MAX_APPS], count: 0, query: [0u8; APP_NAME_LEN], qlen: 0, visible: false }
    }
    pub fn register_app(&mut self, name: &[u8], exec: &[u8], cat: u8) {
        if self.count >= MAX_APPS { return; }
        let e = &mut self.apps[self.count];
        let nn = name.len().min(APP_NAME_LEN); e.name[..nn].copy_from_slice(&name[..nn]); e.nlen = nn as u8;
        let en = exec.len().min(64);            e.exec[..en].copy_from_slice(&exec[..en]);  e.elen = en as u8;
        e.category = cat;
        self.count += 1;
    }
    pub fn show(&mut self) { self.visible = true; self.qlen = 0; }
    pub fn hide(&mut self) { self.visible = false; }
    pub fn input_char(&mut self, c: u8) {
        if self.qlen < APP_NAME_LEN { self.query[self.qlen] = c; self.qlen += 1; }
    }
    pub fn backspace(&mut self) { if self.qlen > 0 { self.qlen -= 1; } }
    /// Returns indices of matching apps (fuzzy prefix match).
    pub fn search(&self, out: &mut [usize]) -> usize {
        let q = &self.query[..self.qlen];
        let mut n = 0;
        for (i, app) in self.apps[..self.count].iter().enumerate() {
            if n >= out.len() { break; }
            let name = &app.name[..app.nlen as usize];
            // Simple case-insensitive prefix/contains match
            if q.is_empty() || contains_ci(name, q) {
                out[n] = i; n += 1;
            }
        }
        n
    }
}

fn contains_ci(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() { return false; }
    let hn = haystack.len(); let nn = needle.len();
    'outer: for start in 0..=hn - nn {
        for i in 0..nn {
            let hc = to_lower(haystack[start + i]);
            let nc = to_lower(needle[i]);
            if hc != nc { continue 'outer; }
        }
        return true;
    }
    false
}
fn to_lower(c: u8) -> u8 { if c >= b'A' && c <= b'Z' { c + 32 } else { c } }

// ─── System Tray ─────────────────────────────────────────────────────────────
pub struct SystemTray {
    pub time_secs:   u64,
    pub battery_pct: u8,
    pub wifi_rssi:   i8,
    pub wifi_ssid:   [u8; 32],
    pub ssid_len:    u8,
    pub bt_connected: bool,
    pub volume_pct:  u8,
    pub brightness_pct: u8,
    pub notifications: u8,
}

impl SystemTray {
    pub const fn new() -> Self {
        SystemTray {
            time_secs: 0, battery_pct: 100, wifi_rssi: -60,
            wifi_ssid: [0u8; 32], ssid_len: 0,
            bt_connected: false, volume_pct: 70,
            brightness_pct: 80, notifications: 0,
        }
    }
    pub fn update_time(&mut self, unix_secs: u64) { self.time_secs = unix_secs; }
    pub fn update_battery(&mut self, pct: u8) { self.battery_pct = pct.min(100); }
    pub fn update_wifi(&mut self, rssi: i8, ssid: &[u8]) {
        self.wifi_rssi = rssi;
        let n = ssid.len().min(32);
        self.wifi_ssid[..n].copy_from_slice(&ssid[..n]);
        self.ssid_len = n as u8;
    }
    /// Format HH:MM from unix seconds.
    pub fn time_fmt(&self) -> [u8; 5] {
        let secs_in_day = self.time_secs % 86400;
        let h = secs_in_day / 3600;
        let m = (secs_in_day % 3600) / 60;
        let digits = b"0123456789";
        [digits[(h / 10) as usize], digits[(h % 10) as usize],
         b':', digits[(m / 10) as usize], digits[(m % 10) as usize]]
    }
    /// Render tray bar at top-right of framebuffer.
    pub fn render(&self, fb: &Framebuffer) {
        let bar_h = 28u32;
        fb.fill_rect(0, 0, fb.width, bar_h, Rgba(25, 25, 25, 240));
        // Time (top-right area)
        let time = self.time_fmt();
        // In real impl: render font glyphs; here we mark the region
        fb.fill_rect(fb.width.saturating_sub(80), 4, 70, 20, Rgba(50, 50, 50, 200));
        // Battery indicator
        let bat_w = (self.battery_pct as u32 * 30 / 100).max(1);
        let bat_color = if self.battery_pct > 20 { Rgba(80, 200, 80, 255) } else { Rgba(220, 50, 50, 255) };
        fb.fill_rect(fb.width.saturating_sub(120), 8, bat_w, 12, bat_color);
        fb.draw_rect_border(fb.width.saturating_sub(120), 8, 32, 12, 1, Rgba(150, 150, 150, 255));
        let _ = time;
    }
}
