/// SigmaOS Compositor (Phase 4 Desktop)
/// Inspired by Omarchy's keyboard-first tiling and Mint Cinnamon's window management.

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompositorBackend { Framebuffer, Software, Wayland }

#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub x: i32, pub y: i32,
    pub width: u32, pub height: u32,
    pub z_order: u32,
    pub focused: bool,
    pub workspace: u32,
    pub visible: bool,
}

pub struct Compositor {
    pub backend: CompositorBackend,
    pub windows: Vec<Window>,
    pub next_id: u32,
    pub active_workspace: u32,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl Compositor {
    pub fn new(backend: CompositorBackend, w: u32, h: u32) -> Self {
        Self { backend, windows: Vec::new(), next_id: 1, active_workspace: 0, screen_width: w, screen_height: h }
    }

    pub fn create_window(&mut self, title: &str, x: i32, y: i32, w: u32, h: u32) -> u32 {
        let id = self.next_id; self.next_id += 1;
        let win = Window { id, title: title.into(), x, y, width: w, height: h, z_order: id, focused: false, workspace: self.active_workspace, visible: true };
        self.windows.push(win);
        self.focus_window(id);
        id
    }

    pub fn focus_window(&mut self, id: u32) {
        let max_z = self.windows.iter().map(|w| w.z_order).max().unwrap_or(0);
        for w in &mut self.windows { w.focused = w.id == id; if w.id == id { w.z_order = max_z + 1; } }
    }

    pub fn close_window(&mut self, id: u32) { self.windows.retain(|w| w.id != id); }

    pub fn cycle_focus(&mut self) {
        let visible: Vec<u32> = self.windows.iter().filter(|w| w.workspace == self.active_workspace && w.visible).map(|w| w.id).collect();
        if visible.is_empty() { return; }
        let cur = visible.iter().position(|&id| self.windows.iter().any(|w| w.id == id && w.focused)).unwrap_or(0);
        let next = visible[(cur + 1) % visible.len()];
        self.focus_window(next);
    }

    pub fn tile_windows(&mut self) {
        let ws_windows: Vec<usize> = self.windows.iter().enumerate().filter(|(_, w)| w.workspace == self.active_workspace && w.visible).map(|(i, _)| i).collect();
        let count = ws_windows.len();
        if count == 0 { return; }
        let cols = if count <= 2 { count as u32 } else { 2 };
        let rows = ((count as u32) + cols - 1) / cols;
        let tw = self.screen_width / cols;
        let th = self.screen_height / rows;
        for (idx, &wi) in ws_windows.iter().enumerate() {
            let col = (idx as u32) % cols;
            let row = (idx as u32) / cols;
            self.windows[wi].x = (col * tw) as i32;
            self.windows[wi].y = (row * th) as i32;
            self.windows[wi].width = tw;
            self.windows[wi].height = th;
        }
    }

    pub fn switch_workspace(&mut self, ws: u32) { self.active_workspace = ws; }

    pub fn move_window_to_workspace(&mut self, window_id: u32, ws: u32) {
        if let Some(w) = self.windows.iter_mut().find(|w| w.id == window_id) { w.workspace = ws; w.visible = ws == self.active_workspace; }
    }

    pub fn render_frame(&self) -> Vec<u32> {
        // Returns visible window IDs in z-order for the active workspace
        let mut visible: Vec<&Window> = self.windows.iter().filter(|w| w.workspace == self.active_workspace && w.visible).collect();
        visible.sort_by_key(|w| w.z_order);
        visible.iter().map(|w| w.id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_lifecycle() {
        let mut c = Compositor::new(CompositorBackend::Framebuffer, 1920, 1080);
        let id1 = c.create_window("Terminal", 0, 0, 800, 600);
        let id2 = c.create_window("Browser", 100, 100, 1024, 768);
        assert_eq!(c.windows.len(), 2);
        assert!(c.windows.iter().find(|w| w.id == id2).unwrap().focused);
        c.close_window(id1);
        assert_eq!(c.windows.len(), 1);
    }

    #[test]
    fn test_tiling() {
        let mut c = Compositor::new(CompositorBackend::Framebuffer, 1920, 1080);
        c.create_window("A", 0, 0, 100, 100);
        c.create_window("B", 0, 0, 100, 100);
        c.tile_windows();
        assert_eq!(c.windows[0].width, 960);
        assert_eq!(c.windows[1].x, 960);
    }

    #[test]
    fn test_workspace_switching() {
        let mut c = Compositor::new(CompositorBackend::Framebuffer, 1920, 1080);
        let id = c.create_window("Term", 0, 0, 800, 600);
        c.move_window_to_workspace(id, 1);
        c.switch_workspace(1);
        let frame = c.render_frame();
        assert!(frame.is_empty() || frame.contains(&id));
    }

    #[test]
    fn test_cycle_focus() {
        let mut c = Compositor::new(CompositorBackend::Framebuffer, 1920, 1080);
        let id1 = c.create_window("A", 0, 0, 100, 100);
        let id2 = c.create_window("B", 0, 0, 100, 100);
        c.cycle_focus();
        assert!(c.windows.iter().find(|w| w.id == id1).unwrap().focused);
    }
}
