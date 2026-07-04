// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// zenith_desktop/compositor/sigma_compositor.rs — Zenith Wayland Compositor
//
// Implements the core Zenith compositor layer:
//   - Window manager (tiling + floating + stacking modes)
//   - Surface registry (per-client wl_surface tracking)
//   - Input dispatch (keyboard, mouse, touch)
//   - GPU-accelerated rendering pipeline (DRM/KMS + OpenGL ES)
//   - Glassmorphism visual effects (blur, translucency, shadows)
//   - Multi-monitor support
//   - Accessibility integration (focus highlighting)
//
// Architecture:
//   sigma-zenithd (compositor daemon)
//   ├─ WaylandServer  (protocol handling)
//   ├─ SurfaceManager (window tracking)
//   ├─ RenderPipeline (GPU → framebuffer)
//   ├─ InputRouter    (libinput events → focused surface)
//   └─ LayoutEngine   (tiling/floating/stacking)
//
// Language: Rust (std — userland daemon)

use std::collections::HashMap;
use std::os::unix::net::UnixListener;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ── Geometry primitives ────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: i32, pub y: i32,
    pub w: u32, pub h: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self { Rect { x, y, w, h } }
    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && py >= self.y
        && px < self.x + self.w as i32
        && py < self.y + self.h as i32
    }
    pub fn area(&self) -> u64 { self.w as u64 * self.h as u64 }
}

// ── Monitor/output descriptor ──────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Monitor {
    pub id:         u32,
    pub name:       String,
    pub rect:       Rect,
    pub refresh_hz: u32,
    pub scale:      f32,       // HiDPI scale factor (e.g. 2.0 for 2x)
    pub primary:    bool,
}

impl Monitor {
    pub fn primary_1080p() -> Self {
        Monitor {
            id: 0, name: "eDP-1".to_string(),
            rect: Rect::new(0, 0, 1920, 1080),
            refresh_hz: 60, scale: 1.0, primary: true,
        }
    }
}

// ── Window surface ─────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowState { Normal, Minimized, Maximized, Fullscreen }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutMode { Tiling, Floating, Stacking }

#[derive(Debug, Clone)]
pub struct Surface {
    pub id:          u32,
    pub client_pid:  u32,
    pub app_id:      String,
    pub title:       String,
    pub rect:        Rect,
    pub state:       WindowState,
    pub focused:     bool,
    pub opacity:     f32,      // 0.0 = transparent, 1.0 = opaque
    pub blur_radius: u32,      // glassmorphism blur
    pub shadow_px:   u32,      // drop shadow size
    pub workspace:   u32,
    pub monitor_id:  u32,
    pub z_order:     u32,      // paint order (lower = below)
}

impl Surface {
    pub fn new(id: u32, client_pid: u32, app_id: &str, title: &str) -> Self {
        Surface {
            id, client_pid,
            app_id: app_id.to_string(),
            title: title.to_string(),
            rect: Rect::new(100, 100, 800, 600),
            state: WindowState::Normal,
            focused: false,
            opacity: 0.92,      // glassmorphism default
            blur_radius: 20,
            shadow_px: 8,
            workspace: 0,
            monitor_id: 0,
            z_order: id,
        }
    }

    pub fn is_visible(&self) -> bool { self.state != WindowState::Minimized }
}

// ── Workspace ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Workspace {
    pub id:     u32,
    pub name:   String,
    pub layout: LayoutMode,
}

impl Workspace {
    pub fn new(id: u32) -> Self {
        Workspace { id, name: format!("WS{}", id + 1), layout: LayoutMode::Tiling }
    }
}

// ── Layout engine ──────────────────────────────────────────────────────────
pub struct LayoutEngine;

impl LayoutEngine {
    /// Auto-tile surfaces within a monitor's usable area.
    /// Binary-space partitioning: each new window splits the current tile.
    pub fn tile(surfaces: &mut [Surface], monitor: &Monitor) {
        let visible: Vec<usize> = surfaces.iter().enumerate()
            .filter(|(_, s)| s.is_visible() && s.monitor_id == monitor.id && s.state == WindowState::Normal)
            .map(|(i, _)| i)
            .collect();

        if visible.is_empty() { return; }

        let usable = Rect::new(
            monitor.rect.x, monitor.rect.y + 30, // 30px top panel
            monitor.rect.w, monitor.rect.h - 30,
        );

        let mut regions: Vec<Rect> = vec![usable];

        for (idx, &si) in visible.iter().enumerate() {
            if idx >= regions.len() {
                // Split the last region horizontally
                let last = *regions.last().unwrap();
                let half = last.h / 2;
                let top = Rect::new(last.x, last.y, last.w, half);
                let bot = Rect::new(last.x, last.y + half as i32, last.w, last.h - half);
                let n = regions.len() - 1;
                regions[n] = top;
                regions.push(bot);
            }
            surfaces[si].rect = regions[idx];
        }
    }

    /// Snap a floating window to the nearest edge/corner.
    pub fn snap_to_edge(surface: &mut Surface, monitor: &Monitor, threshold: i32) {
        let m = &monitor.rect;
        let s = &mut surface.rect;

        // Left edge
        if (s.x - m.x).abs() < threshold { s.x = m.x; }
        // Right edge
        if ((s.x + s.w as i32) - (m.x + m.w as i32)).abs() < threshold {
            s.x = m.x + m.w as i32 - s.w as i32;
        }
        // Top edge
        if (s.y - (m.y + 30)).abs() < threshold { s.y = m.y + 30; }
        // Bottom edge
        if ((s.y + s.h as i32) - (m.y + m.h as i32)).abs() < threshold {
            s.y = m.y + m.h as i32 - s.h as i32;
        }
    }

    /// Quarter-tile shortcut (Win+Arrow)
    pub fn snap_quarter(surface: &mut Surface, monitor: &Monitor, quadrant: u8) {
        let m = &monitor.rect;
        let hw = m.w / 2; let hh = (m.h - 30) / 2;
        surface.rect = match quadrant {
            0 => Rect::new(m.x,             m.y + 30,              hw, hh), // top-left
            1 => Rect::new(m.x + hw as i32, m.y + 30,              hw, hh), // top-right
            2 => Rect::new(m.x,             m.y + 30 + hh as i32,  hw, hh), // bottom-left
            3 => Rect::new(m.x + hw as i32, m.y + 30 + hh as i32,  hw, hh), // bottom-right
            _ => surface.rect,
        };
    }
}

// ── Input event ────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress    { key: u32, mods: u32, pressed: bool },
    MouseMove   { x: f64, y: f64 },
    MouseButton { x: f64, y: f64, button: u32, pressed: bool },
    MouseScroll { x: f64, y: f64, dx: f64, dy: f64 },
    TouchBegin  { id: u32, x: f64, y: f64 },
    TouchMove   { id: u32, x: f64, y: f64 },
    TouchEnd    { id: u32 },
}

// ── Render frame ───────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct RenderFrame {
    pub surfaces: Vec<(Rect, f32, u32)>, // (rect, opacity, blur_radius)
    pub cursor:   (f64, f64),
    pub timestamp: u64, // microseconds
}

// ── Compositor ────────────────────────────────────────────────────────────
pub struct Compositor {
    surfaces:     HashMap<u32, Surface>,
    monitors:     Vec<Monitor>,
    workspaces:   Vec<Workspace>,
    next_id:      u32,
    focused_id:   Option<u32>,
    cursor:       (f64, f64),
    layout_mode:  LayoutMode,
    active_ws:    u32,
    frame_count:  u64,
    last_frame:   Instant,
    running:      bool,
}

impl Compositor {
    pub fn new() -> Self {
        let monitors = vec![Monitor::primary_1080p()];
        let workspaces = (0..4).map(Workspace::new).collect();
        Compositor {
            surfaces: HashMap::new(),
            monitors, workspaces,
            next_id: 1,
            focused_id: None,
            cursor: (960.0, 540.0),
            layout_mode: LayoutMode::Tiling,
            active_ws: 0,
            frame_count: 0,
            last_frame: Instant::now(),
            running: true,
        }
    }

    // ── Surface management ────────────────────────────────────────────────

    pub fn add_surface(&mut self, client_pid: u32, app_id: &str, title: &str) -> u32 {
        let id = self.next_id; self.next_id += 1;
        let mut s = Surface::new(id, client_pid, app_id, title);
        s.workspace  = self.active_ws;
        s.monitor_id = 0;
        self.surfaces.insert(id, s);
        self.retile();
        self.focus(id);
        id
    }

    pub fn remove_surface(&mut self, id: u32) {
        self.surfaces.remove(&id);
        if self.focused_id == Some(id) {
            // Focus next available window
            self.focused_id = self.surfaces.keys().next().copied();
        }
        self.retile();
    }

    pub fn focus(&mut self, id: u32) {
        if let Some(old) = self.focused_id {
            if let Some(s) = self.surfaces.get_mut(&old) { s.focused = false; }
        }
        if let Some(s) = self.surfaces.get_mut(&id) { s.focused = true; }
        self.focused_id = Some(id);
    }

    pub fn surface_at(&self, x: f64, y: f64) -> Option<u32> {
        // Find topmost (highest z_order) surface at point
        self.surfaces.values()
            .filter(|s| s.is_visible() && s.rect.contains(x as i32, y as i32))
            .max_by_key(|s| s.z_order)
            .map(|s| s.id)
    }

    // ── Layout ────────────────────────────────────────────────────────────

    fn retile(&mut self) {
        if self.layout_mode != LayoutMode::Tiling { return; }
        let monitor = self.monitors[0].clone();
        let mut surfaces: Vec<Surface> = self.surfaces.values().cloned().collect();
        LayoutEngine::tile(&mut surfaces, &monitor);
        for s in surfaces { self.surfaces.insert(s.id, s); }
    }

    pub fn set_layout(&mut self, mode: LayoutMode) {
        self.layout_mode = mode;
        self.retile();
    }

    pub fn maximize(&mut self, id: u32) {
        if let Some(s) = self.surfaces.get_mut(&id) {
            let m = &self.monitors[0];
            if s.state == WindowState::Maximized {
                s.state = WindowState::Normal;
                s.rect  = Rect::new(100, 130, 800, 600);
            } else {
                s.state = WindowState::Maximized;
                s.rect  = Rect::new(m.rect.x, m.rect.y + 30, m.rect.w, m.rect.h - 30);
            }
        }
    }

    pub fn switch_workspace(&mut self, ws: u32) {
        self.active_ws = ws.min(self.workspaces.len() as u32 - 1);
    }

    // ── Input handling ────────────────────────────────────────────────────

    pub fn handle_input(&mut self, event: InputEvent) {
        match event {
            InputEvent::MouseMove { x, y } => {
                self.cursor = (x, y);
            }
            InputEvent::MouseButton { x, y, button: 1, pressed: true } => {
                if let Some(id) = self.surface_at(x, y) {
                    self.focus(id);
                }
            }
            InputEvent::KeyPress { key, mods, pressed: true } => {
                self.handle_keybind(key, mods);
            }
            _ => {}
        }
    }

    fn handle_keybind(&mut self, key: u32, mods: u32) {
        const MOD_SUPER: u32 = 1 << 6;  // Windows/Command key
        const MOD_SHIFT: u32 = 1 << 0;
        const MOD_CTRL:  u32 = 1 << 2;

        const KEY_Q: u32 = 16; const KEY_T: u32 = 20; const KEY_D: u32 = 32;
        const KEY_F: u32 = 33; const KEY_LEFT: u32 = 105; const KEY_RIGHT: u32 = 106;
        const KEY_UP: u32 = 103; const KEY_DOWN: u32 = 108;
        const KEY_1: u32 = 2; // 1-4 for workspaces

        if mods & MOD_SUPER != 0 {
            match key {
                KEY_Q => { // Super+Q: close focused window
                    if let Some(id) = self.focused_id { self.remove_surface(id); }
                }
                KEY_T => { // Super+T: open terminal (send IPC)
                    self.spawn_app("sigma-terminal");
                }
                KEY_D => { // Super+D: app launcher
                    self.spawn_app("sigma-launcher");
                }
                KEY_F => { // Super+F: maximize
                    if let Some(id) = self.focused_id { self.maximize(id); }
                }
                KEY_LEFT  => { // Super+Left: snap left half
                    if let Some(id) = self.focused_id {
                        let m = self.monitors[0].clone();
                        if let Some(s) = self.surfaces.get_mut(&id) {
                            s.rect = Rect::new(m.rect.x, m.rect.y + 30, m.rect.w / 2, m.rect.h - 30);
                            s.state = WindowState::Normal;
                        }
                    }
                }
                KEY_RIGHT => { // Super+Right: snap right half
                    if let Some(id) = self.focused_id {
                        let m = self.monitors[0].clone();
                        if let Some(s) = self.surfaces.get_mut(&id) {
                            s.rect = Rect::new(m.rect.x + (m.rect.w/2) as i32, m.rect.y + 30, m.rect.w / 2, m.rect.h - 30);
                            s.state = WindowState::Normal;
                        }
                    }
                }
                KEY_1..=4 => { // Super+1-4: switch workspace
                    self.switch_workspace(key - KEY_1);
                }
                _ => {}
            }
        }
    }

    fn spawn_app(&self, app_id: &str) {
        // In production: send Wayland extension request or exec
        eprintln!("[zenith] spawn: {}", app_id);
    }

    // ── Render frame ──────────────────────────────────────────────────────

    pub fn render(&mut self) -> RenderFrame {
        self.frame_count += 1;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;

        // Collect surfaces sorted by z_order (back to front)
        let mut visible: Vec<&Surface> = self.surfaces.values()
            .filter(|s| s.is_visible() && s.workspace == self.active_ws)
            .collect();
        visible.sort_by_key(|s| s.z_order);

        let surfaces = visible.iter().map(|s| (s.rect, s.opacity, s.blur_radius)).collect();

        // Track frame time
        let elapsed = self.last_frame.elapsed();
        self.last_frame = Instant::now();
        let fps = 1.0 / elapsed.as_secs_f64();
        if self.frame_count % 300 == 0 {
            eprintln!("[zenith] frame={} fps={:.1} surfaces={}", self.frame_count, fps, visible.len());
        }

        RenderFrame { surfaces, cursor: self.cursor, timestamp: now }
    }

    // ── Status ────────────────────────────────────────────────────────────

    pub fn window_list(&self) -> Vec<(u32, String, String)> {
        let mut list: Vec<_> = self.surfaces.values()
            .map(|s| (s.id, s.app_id.clone(), s.title.clone()))
            .collect();
        list.sort_by_key(|(id, _, _)| *id);
        list
    }

    pub fn focused_title(&self) -> Option<String> {
        self.focused_id.and_then(|id| self.surfaces.get(&id)).map(|s| s.title.clone())
    }
}

// ── Wayland IPC socket ────────────────────────────────────────────────────
/// sigma-zenithd listens on $XDG_RUNTIME_DIR/zenith.socket
/// and accepts JSON commands from client apps.
pub fn run_compositor_daemon() {
    let socket_path = std::env::var("ZENITH_SOCKET")
        .unwrap_or_else(|_| "/run/user/1000/zenith.socket".to_string());

    let _ = std::fs::remove_file(&socket_path);
    let compositor = Arc::new(Mutex::new(Compositor::new()));

    // Start render loop in background thread
    let comp_render = compositor.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_millis(16)); // ~60fps
            let mut c = comp_render.lock().unwrap();
            let _frame = c.render();
            // In production: submit frame to DRM/KMS via libdrm
        }
    });

    // IPC server
    match UnixListener::bind(&socket_path) {
        Ok(listener) => {
            eprintln!("[sigma-zenithd] Listening on {}", socket_path);
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        use std::io::{BufRead, BufReader, Write};
                        let comp = compositor.clone();
                        std::thread::spawn(move || {
                            let reader = BufReader::new(stream.try_clone().unwrap());
                            for line in reader.lines() {
                                let Ok(line) = line else { break };
                                let response = handle_ipc_command(&line, &comp);
                                let _ = stream.write_all(response.as_bytes());
                                let _ = stream.write_all(b"\n");
                            }
                        });
                    }
                    Err(e) => eprintln!("[zenith] IPC error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("[zenith] Cannot bind socket: {}", e),
    }
}

fn handle_ipc_command(cmd: &str, comp: &Arc<Mutex<Compositor>>) -> String {
    let mut c = comp.lock().unwrap();
    match cmd.trim() {
        "list" => {
            let windows = c.window_list();
            serde_json_lite(&windows.iter()
                .map(|(id, app, title)| format!("{{\"id\":{},\"app\":\"{}\",\"title\":\"{}\"}}", id, app, title))
                .collect::<Vec<_>>())
        }
        "focused" => c.focused_title().unwrap_or_else(|| "none".to_string()),
        s if s.starts_with("focus ") => {
            if let Ok(id) = s[6..].trim().parse::<u32>() {
                c.focus(id);
                format!("{{\"ok\":true,\"id\":{}}}", id)
            } else { r#"{"error":"invalid id"}"#.to_string() }
        }
        s if s.starts_with("open ") => {
            let app_id = s[5..].trim();
            let id = c.add_surface(0, app_id, app_id);
            format!("{{\"ok\":true,\"id\":{}}}", id)
        }
        s if s.starts_with("close ") => {
            if let Ok(id) = s[6..].trim().parse::<u32>() {
                c.remove_surface(id);
                format!("{{\"ok\":true,\"id\":{}}}", id)
            } else { r#"{"error":"invalid id"}"#.to_string() }
        }
        "tile" => { c.set_layout(LayoutMode::Tiling); r#"{"ok":true,"mode":"tile"}"#.to_string() }
        "float" => { c.set_layout(LayoutMode::Floating); r#"{"ok":true,"mode":"float"}"#.to_string() }
        "status" => format!("{{\"frames\":{},\"surfaces\":{}}}", c.frame_count, c.surfaces.len()),
        _ => r#"{"error":"unknown command"}"#.to_string(),
    }
}

fn serde_json_lite(items: &[String]) -> String {
    format!("[{}]", items.join(","))
}

// ── sigma-zenith CLI ──────────────────────────────────────────────────────
#[cfg(feature = "cli")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("daemon") | None => run_compositor_daemon(),
        Some("status") => {
            // Query running daemon
            use std::io::{Read, Write};
            if let Ok(mut s) = std::os::unix::net::UnixStream::connect("/run/user/1000/zenith.socket") {
                let _ = s.write_all(b"status\n");
                let mut buf = String::new();
                let _ = s.read_to_string(&mut buf);
                println!("{}", buf);
            } else {
                eprintln!("sigma-zenithd is not running");
            }
        }
        Some(cmd) => eprintln!("Unknown command: {}", cmd),
    }
}
