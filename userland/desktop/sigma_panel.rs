// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_panel.rs — Zenith Top Panel (Dynamic Island status bar)
// Language: Rust (std) — OOP via Panel struct

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use crate::userland::desktop::sigma_theme::{Color, ThemeEngine, Palette};
use crate::userland::desktop::sigma_compositor::Rect;

// ── Panel Item Trait ──────────────────────────────────────────────────────────
pub trait PanelItem: Send {
    fn id(&self)    -> &'static str;
    fn width(&self) -> u32;
    fn tick(&mut self, now_ms: u64);
    fn text(&self)  -> String;
    fn color(&self, theme: &ThemeEngine) -> Color;
    fn click(&mut self) {}
}

// ── Clock ─────────────────────────────────────────────────────────────────────
pub struct ClockItem { pub show_seconds: bool }
impl PanelItem for ClockItem {
    fn id(&self) -> &'static str { "clock" }
    fn width(&self) -> u32 { if self.show_seconds { 130 } else { 90 } }
    fn tick(&mut self, _: u64) {}
    fn text(&self) -> String {
        let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let h = (secs / 3600) % 24;
        let m = (secs / 60) % 60;
        let s = secs % 60;
        if self.show_seconds { format!("{:02}:{:02}:{:02}", h, m, s) }
        else { format!("{:02}:{:02}", h, m) }
    }
    fn color(&self, t: &ThemeEngine) -> Color { t.get_color("text") }
}

// ── CPU Indicator ─────────────────────────────────────────────────────────────
pub struct CpuItem { pub pct: f32 }
impl PanelItem for CpuItem {
    fn id(&self) -> &'static str { "cpu" }
    fn width(&self) -> u32 { 70 }
    fn tick(&mut self, _: u64) {
        // Real: read from sigmad-health or /proc/stat
    }
    fn text(&self) -> String { format!("CPU {:.0}%", self.pct) }
    fn color(&self, t: &ThemeEngine) -> Color {
        if self.pct > 80.0 { t.get_color("error") }
        else if self.pct > 60.0 { t.get_color("warning") }
        else { t.get_color("muted") }
    }
}

// ── Memory Indicator ──────────────────────────────────────────────────────────
pub struct MemItem { pub used_mb: u32, pub total_mb: u32 }
impl PanelItem for MemItem {
    fn id(&self) -> &'static str { "mem" }
    fn width(&self) -> u32 { 90 }
    fn tick(&mut self, _: u64) {}
    fn text(&self) -> String { format!("MEM {}MB", self.used_mb) }
    fn color(&self, t: &ThemeEngine) -> Color {
        let pct = if self.total_mb > 0 { self.used_mb * 100 / self.total_mb } else { 0 };
        if pct > 85 { t.get_color("error") } else { t.get_color("muted") }
    }
}

// ── Battery ───────────────────────────────────────────────────────────────────
pub struct BatteryItem { pub pct: u8, pub charging: bool }
impl PanelItem for BatteryItem {
    fn id(&self) -> &'static str { "battery" }
    fn width(&self) -> u32 { 70 }
    fn tick(&mut self, _: u64) {
        // Read from /sys/class/power_supply/BAT0/
    }
    fn text(&self) -> String {
        let icon = if self.charging { "⚡" } else if self.pct > 70 { "🔋" } else if self.pct > 30 { "🪫" } else { "⚠" };
        format!("{} {}%", icon, self.pct)
    }
    fn color(&self, t: &ThemeEngine) -> Color {
        if self.pct < 20 && !self.charging { t.get_color("error") }
        else if self.charging { t.get_color("success") }
        else { t.get_color("muted") }
    }
}

// ── Network Indicator ─────────────────────────────────────────────────────────
pub struct NetworkItem { pub connected: bool, pub wifi: bool, pub ssid: String }
impl PanelItem for NetworkItem {
    fn id(&self) -> &'static str { "network" }
    fn width(&self) -> u32 { 100 }
    fn tick(&mut self, _: u64) {}
    fn text(&self) -> String {
        if !self.connected { return "⊗ offline".to_owned(); }
        if self.wifi { format!("▲ {}", if self.ssid.len() > 8 { &self.ssid[..8] } else { &self.ssid }) }
        else { "⬡ eth".to_owned() }
    }
    fn color(&self, t: &ThemeEngine) -> Color {
        if self.connected { t.get_color("success") } else { t.get_color("error") }
    }
}

// ── Active Window Title ───────────────────────────────────────────────────────
pub struct WindowTitleItem { pub title: String, pub max_len: usize }
impl PanelItem for WindowTitleItem {
    fn id(&self) -> &'static str { "window_title" }
    fn width(&self) -> u32 { 300 }
    fn tick(&mut self, _: u64) {}
    fn text(&self) -> String {
        if self.title.len() > self.max_len {
            format!("{}…", &self.title[..self.max_len])
        } else { self.title.clone() }
    }
    fn color(&self, t: &ThemeEngine) -> Color { t.get_color("text") }
}

// ── Panel ─────────────────────────────────────────────────────────────────────
pub struct Panel {
    pub bounds:   Rect,
    pub height:   u32,
    items_left:   Vec<Box<dyn PanelItem>>,
    items_center: Vec<Box<dyn PanelItem>>,
    items_right:  Vec<Box<dyn PanelItem>>,
}

impl Panel {
    pub fn new(screen_w: u32) -> Self {
        let mut p = Self {
            bounds: Rect { x: 0, y: 0, w: screen_w, h: 32 },
            height: 32,
            items_left:   Vec::new(),
            items_center: Vec::new(),
            items_right:  Vec::new(),
        };
        // Left: workspace indicator + window title
        p.items_left.push(Box::new(WindowTitleItem { title: "Σ SigmaOS".to_owned(), max_len: 30 }));
        // Center: clock
        p.items_center.push(Box::new(ClockItem { show_seconds: false }));
        // Right: system indicators
        p.items_right.push(Box::new(CpuItem { pct: 0.0 }));
        p.items_right.push(Box::new(MemItem { used_mb: 0, total_mb: 512 }));
        p.items_right.push(Box::new(NetworkItem { connected: true, wifi: true, ssid: "SigmaNet".to_owned() }));
        p.items_right.push(Box::new(BatteryItem { pct: 85, charging: false }));
        p
    }

    pub fn tick(&mut self, now_ms: u64) {
        for item in self.items_left.iter_mut()   { item.tick(now_ms); }
        for item in self.items_center.iter_mut() { item.tick(now_ms); }
        for item in self.items_right.iter_mut()  { item.tick(now_ms); }
    }

    pub fn set_window_title(&mut self, title: &str) {
        for item in &mut self.items_left {
            if item.id() == "window_title" {
                if let Some(wt) = (item as &mut dyn std::any::Any).downcast_mut::<WindowTitleItem>() {
                    wt.title = title.to_owned();
                }
            }
        }
    }

    pub fn update_cpu(&mut self, pct: f32) {
        for item in &mut self.items_right {
            if item.id() == "cpu" {
                if let Some(c) = (item as &mut dyn std::any::Any).downcast_mut::<CpuItem>() {
                    c.pct = pct;
                }
            }
        }
    }

    pub fn render_text_items(&self, theme: &ThemeEngine) -> Vec<(i32, i32, String, Color)> {
        let mut items = Vec::new();
        let mut x = 8i32;
        for item in &self.items_left {
            items.push((x, 8, item.text(), item.color(theme)));
            x += item.width() as i32 + 8;
        }
        // Center items
        let center_w: u32 = self.items_center.iter().map(|i| i.width()).sum();
        let mut cx = (self.bounds.w as i32 - center_w as i32) / 2;
        for item in &self.items_center {
            items.push((cx, 8, item.text(), item.color(theme)));
            cx += item.width() as i32 + 8;
        }
        // Right items
        let right_w: u32 = self.items_right.iter().map(|i| i.width() + 8).sum();
        let mut rx = self.bounds.w as i32 - right_w as i32;
        for item in &self.items_right {
            items.push((rx, 8, item.text(), item.color(theme)));
            rx += item.width() as i32 + 8;
        }
        items
    }
}
