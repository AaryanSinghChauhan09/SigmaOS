// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_wm.rs — Zenith Window Manager (tiling + floating)
// Language: Rust (std) — OOP via WindowManager struct + Layout trait

use std::collections::BTreeMap;
use crate::userland::desktop::sigma_compositor::{SurfaceId, Rect, Size, Point};

// ── Layout Trait (OOP interface) ─────────────────────────────────────────────

pub trait Layout: Send {
    fn name(&self) -> &'static str;
    fn arrange(&self, windows: &[SurfaceId], screen: Size,
               gaps: u32, out: &mut BTreeMap<SurfaceId, Rect>);
}

// ── Layout: Master-Stack ──────────────────────────────────────────────────────

pub struct MasterStackLayout { pub master_ratio: f32 }

impl Layout for MasterStackLayout {
    fn name(&self) -> &'static str { "master-stack" }
    fn arrange(&self, windows: &[SurfaceId], screen: Size,
               gaps: u32, out: &mut BTreeMap<SurfaceId, Rect>) {
        out.clear();
        if windows.is_empty() { return; }
        let g = gaps as i32;
        let w = screen.w as i32; let h = screen.h as i32;
        if windows.len() == 1 {
            out.insert(windows[0], Rect { x: g, y: g, w: (w - g*2) as u32, h: (h - g*2) as u32 });
            return;
        }
        let master_w = ((w as f32 * self.master_ratio) as i32).max(200);
        let stack_w  = w - master_w - g * 3;
        let stack_h  = (h - g * (windows.len() as i32)) / (windows.len() as i32 - 1);
        // Master
        out.insert(windows[0], Rect { x: g, y: g, w: master_w as u32, h: (h - g*2) as u32 });
        // Stack
        for (i, &id) in windows[1..].iter().enumerate() {
            let y = g + (stack_h + g) * i as i32;
            out.insert(id, Rect {
                x: master_w + g*2, y,
                w: stack_w.max(0) as u32, h: stack_h.max(0) as u32,
            });
        }
    }
}

// ── Layout: Grid ─────────────────────────────────────────────────────────────

pub struct GridLayout;

impl Layout for GridLayout {
    fn name(&self) -> &'static str { "grid" }
    fn arrange(&self, windows: &[SurfaceId], screen: Size,
               gaps: u32, out: &mut BTreeMap<SurfaceId, Rect>) {
        out.clear();
        let n = windows.len();
        if n == 0 { return; }
        let cols = (n as f64).sqrt().ceil() as u32;
        let rows = ((n as u32 + cols - 1) / cols).max(1);
        let g   = gaps as i32;
        let cw  = ((screen.w as i32 - g * (cols as i32 + 1)) / cols as i32).max(1);
        let ch  = ((screen.h as i32 - g * (rows as i32 + 1)) / rows as i32).max(1);
        for (i, &id) in windows.iter().enumerate() {
            let col = (i as u32 % cols) as i32;
            let row = (i as u32 / cols) as i32;
            out.insert(id, Rect {
                x: g + col * (cw + g), y: g + row * (ch + g),
                w: cw as u32, h: ch as u32,
            });
        }
    }
}

// ── Layout: BSP (Binary Space Partitioning) ──────────────────────────────────

pub struct BspLayout;

impl Layout for BspLayout {
    fn name(&self) -> &'static str { "bsp" }
    fn arrange(&self, windows: &[SurfaceId], screen: Size,
               gaps: u32, out: &mut BTreeMap<SurfaceId, Rect>) {
        out.clear();
        let g = gaps as i32;
        let area = Rect { x: g, y: g, w: (screen.w as i32 - g*2).max(0) as u32,
                          h: (screen.h as i32 - g*2).max(0) as u32 };
        Self::split(windows, area, true, gaps, out);
    }
}

impl BspLayout {
    fn split(windows: &[SurfaceId], area: Rect, horizontal: bool,
             gaps: u32, out: &mut BTreeMap<SurfaceId, Rect>) {
        let g = gaps as i32;
        match windows.len() {
            0 => {}
            1 => { out.insert(windows[0], area); }
            n => {
                let half = n / 2;
                if horizontal {
                    let hw = ((area.w as i32 - g) / 2).max(0);
                    let a1 = Rect { x: area.x, y: area.y, w: hw as u32, h: area.h };
                    let a2 = Rect { x: area.x + hw + g, y: area.y,
                                    w: (area.w as i32 - hw - g).max(0) as u32, h: area.h };
                    Self::split(&windows[..half], a1, !horizontal, gaps, out);
                    Self::split(&windows[half..], a2, !horizontal, gaps, out);
                } else {
                    let hh = ((area.h as i32 - g) / 2).max(0);
                    let a1 = Rect { x: area.x, y: area.y, w: area.w, h: hh as u32 };
                    let a2 = Rect { x: area.x, y: area.y + hh + g,
                                    w: area.w, h: (area.h as i32 - hh - g).max(0) as u32 };
                    Self::split(&windows[..half], a1, !horizontal, gaps, out);
                    Self::split(&windows[half..], a2, !horizontal, gaps, out);
                }
            }
        }
    }
}

// ── Workspace ─────────────────────────────────────────────────────────────────

pub struct Workspace {
    pub id:      usize,
    pub name:    String,
    pub windows: Vec<SurfaceId>,
    pub layout:  Box<dyn Layout>,
    pub gaps:    u32,
    pub floating: std::collections::HashSet<SurfaceId>,
}

impl Workspace {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id, name: name.to_owned(),
            windows: Vec::new(),
            layout:  Box::new(MasterStackLayout { master_ratio: 0.6 }),
            gaps:    8,
            floating: std::collections::HashSet::new(),
        }
    }
    pub fn add_window(&mut self, id: SurfaceId) { self.windows.push(id); }
    pub fn remove_window(&mut self, id: SurfaceId) {
        self.windows.retain(|&w| w != id);
        self.floating.remove(&id);
    }
    pub fn toggle_float(&mut self, id: SurfaceId) {
        if self.floating.contains(&id) { self.floating.remove(&id); }
        else { self.floating.insert(id); }
    }
    pub fn tiled_windows(&self) -> Vec<SurfaceId> {
        self.windows.iter().filter(|&&w| !self.floating.contains(&w)).copied().collect()
    }
    pub fn arrange(&self, screen: Size) -> BTreeMap<SurfaceId, Rect> {
        let mut out = BTreeMap::new();
        let tiled = self.tiled_windows();
        self.layout.arrange(&tiled, screen, self.gaps, &mut out);
        out
    }
}

// ── Window Manager ────────────────────────────────────────────────────────────

pub struct WindowManager {
    workspaces:    Vec<Workspace>,
    active_ws:     usize,
    screen:        Size,
    float_rects:   BTreeMap<SurfaceId, Rect>,
}

impl WindowManager {
    pub fn new(screen_w: u32, screen_h: u32) -> Self {
        let mut wm = Self {
            workspaces: Vec::new(),
            active_ws:  0,
            screen:     Size { w: screen_w, h: screen_h },
            float_rects: BTreeMap::new(),
        };
        for i in 1..=9 {
            wm.workspaces.push(Workspace::new(i - 1, &i.to_string()));
        }
        wm
    }

    pub fn active(&mut self) -> &mut Workspace { &mut self.workspaces[self.active_ws] }

    pub fn switch_workspace(&mut self, idx: usize) -> bool {
        if idx >= self.workspaces.len() { return false; }
        self.active_ws = idx; true
    }

    pub fn move_to_workspace(&mut self, id: SurfaceId, ws: usize) -> bool {
        if ws >= self.workspaces.len() { return false; }
        let cur = self.active_ws;
        self.workspaces[cur].remove_window(id);
        self.workspaces[ws].add_window(id);
        true
    }

    /// Get tile rects for all windows in the current workspace
    pub fn tile_rects(&self) -> BTreeMap<SurfaceId, Rect> {
        let ws = &self.workspaces[self.active_ws];
        let mut rects = ws.arrange(self.screen);
        // Add floating window rects
        for &id in &ws.floating {
            if let Some(&r) = self.float_rects.get(&id) { rects.insert(id, r); }
        }
        rects
    }

    pub fn set_float_rect(&mut self, id: SurfaceId, r: Rect) { self.float_rects.insert(id, r); }

    pub fn cycle_layout(&mut self) {
        let ws = &mut self.workspaces[self.active_ws];
        let next: Box<dyn Layout> = match ws.layout.name() {
            "master-stack" => Box::new(GridLayout),
            "grid"         => Box::new(BspLayout),
            _              => Box::new(MasterStackLayout { master_ratio: 0.6 }),
        };
        ws.layout = next;
    }

    pub fn current_workspace(&self) -> usize { self.active_ws }
    pub fn workspace_count(&self) -> usize { self.workspaces.len() }
    pub fn screen_size(&self) -> Size { self.screen }
}
