//! Hyprland-inspired tiling layout algorithms for SigmaOS window manager.
//!
//! Upstream inspiration:
//! - Hyprland dwindle layout: <https://wiki.hyprland.org/Configuring/Dwindle-Layout/>
//! - dwm master+stack:        <https://dwm.suckless.org/>

/// Per-layout configuration knobs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutConfig {
    /// Gap between windows in pixels.
    pub gaps: u32,
    /// Border thickness in pixels.
    pub border_px: u32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            gaps: 6,
            border_px: 2,
        }
    }
}

/// Axis of the next split in a dwindle layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitAxis {
    Horizontal,
    Vertical,
}

/// A rectangle on screen, in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    /// Split into (primary, secondary) along the given axis.
    pub fn split(self, axis: SplitAxis, gap: u32) -> (Rect, Rect) {
        match axis {
            SplitAxis::Horizontal => {
                let half = self.w.saturating_sub(gap) / 2;
                let a = Rect { x: self.x, y: self.y, w: half, h: self.h };
                let b = Rect {
                    x: self.x + half as i32 + gap as i32,
                    y: self.y,
                    w: self.w.saturating_sub(half + gap),
                    h: self.h,
                };
                (a, b)
            }
            SplitAxis::Vertical => {
                let half = self.h.saturating_sub(gap) / 2;
                let a = Rect { x: self.x, y: self.y, w: self.w, h: half };
                let b = Rect {
                    x: self.x,
                    y: self.y + half as i32 + gap as i32,
                    w: self.w,
                    h: self.h.saturating_sub(half + gap),
                };
                (a, b)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Dwindle layout
// ---------------------------------------------------------------------------

/// Hyprland-style dwindle layout: each new window recursively splits the
/// remaining space, alternating axis on every level.
///
/// Maintains an ordered list of `(window_id, geometry)` pairs.
#[derive(Debug, Default)]
pub struct DwindleLayout {
    pub config: LayoutConfig,
    tiles: Vec<(u64, Rect)>,
}

impl DwindleLayout {
    pub fn new(config: LayoutConfig) -> Self {
        Self {
            config,
            tiles: Vec::new(),
        }
    }

    /// Add a window and recompute the whole spiral for `screen`.
    pub fn add_window(&mut self, id: u64, screen: Rect) {
        self.tiles.push((id, screen)); // placeholder rect, recomputed below
        self.recompute(screen);
    }

    /// Remove a window and recompute.
    pub fn remove_window(&mut self, id: u64, screen: Rect) -> bool {
        let before = self.tiles.len();
        self.tiles.retain(|(wid, _)| *wid != id);
        let removed = self.tiles.len() < before;
        if removed {
            self.recompute(screen);
        }
        removed
    }

    /// Recompute all tile geometries from scratch (recursive spiral split).
    fn recompute(&mut self, screen: Rect) {
        let ids: Vec<u64> = self.tiles.iter().map(|(id, _)| *id).collect();
        let rects = Self::spiral_split(screen, &ids, self.config.gaps, SplitAxis::Horizontal);
        self.tiles = ids.into_iter().zip(rects).collect();
    }

    fn spiral_split(area: Rect, ids: &[u64], gap: u32, axis: SplitAxis) -> Vec<Rect> {
        match ids.len() {
            0 => vec![],
            1 => vec![area],
            _ => {
                let (primary, rest) = area.split(axis, gap);
                let next_axis = match axis {
                    SplitAxis::Horizontal => SplitAxis::Vertical,
                    SplitAxis::Vertical => SplitAxis::Horizontal,
                };
                let mut out = vec![primary];
                out.extend(Self::spiral_split(rest, &ids[1..], gap, next_axis));
                out
            }
        }
    }

    /// Geometry for a window, if tracked.
    pub fn geometry(&self, id: u64) -> Option<Rect> {
        self.tiles.iter().find(|(wid, _)| *wid == id).map(|(_, r)| *r)
    }

    /// Number of managed windows.
    pub fn count(&self) -> usize {
        self.tiles.len()
    }

    /// All (window_id, rect) pairs.
    pub fn tiles(&self) -> &[(u64, Rect)] {
        &self.tiles
    }
}

// ---------------------------------------------------------------------------
// Master-stack layout
// ---------------------------------------------------------------------------

/// dwm-style master + stack layout. The first window (master) occupies the
/// left half; remaining windows share the right half as a vertical stack.
#[derive(Debug, Default)]
pub struct MasterStackLayout {
    pub config: LayoutConfig,
    windows: Vec<u64>,
}

impl MasterStackLayout {
    pub fn new(config: LayoutConfig) -> Self {
        Self {
            config,
            windows: Vec::new(),
        }
    }

    /// Add a window. First added becomes master.
    pub fn add_window(&mut self, id: u64) {
        if !self.windows.contains(&id) {
            self.windows.push(id);
        }
    }

    /// Remove a window.
    pub fn remove_window(&mut self, id: u64) -> bool {
        let before = self.windows.len();
        self.windows.retain(|&w| w != id);
        self.windows.len() < before
    }

    /// Promote a window to master position.
    pub fn promote(&mut self, id: u64) {
        if let Some(pos) = self.windows.iter().position(|&w| w == id) {
            self.windows.swap(0, pos);
        }
    }

    /// Compute geometries: master on left, stack on right.
    ///
    /// Returns `(master_id, master_rect, stack_tiles)`.
    pub fn compute(&self, screen: Rect) -> Option<(u64, Rect, Vec<(u64, Rect)>)> {
        if self.windows.is_empty() {
            return None;
        }
        let master_id = self.windows[0];
        let stack = &self.windows[1..];
        let gap = self.config.gaps;

        let (master_rect, stack_area) = screen.split(SplitAxis::Horizontal, gap);

        let stack_tiles: Vec<(u64, Rect)> = if stack.is_empty() {
            // No stack — master takes full screen
            return Some((master_id, screen, vec![]));
        } else {
            let n = stack.len() as u32;
            let slot_h = stack_area.h.saturating_sub(gap * (n - 1)) / n;
            stack
                .iter()
                .enumerate()
                .map(|(i, &wid)| {
                    let y = stack_area.y + (i as i32) * (slot_h as i32 + gap as i32);
                    (
                        wid,
                        Rect {
                            x: stack_area.x,
                            y,
                            w: stack_area.w,
                            h: slot_h,
                        },
                    )
                })
                .collect()
        };
        Some((master_id, master_rect, stack_tiles))
    }

    pub fn count(&self) -> usize {
        self.windows.len()
    }
}

// ---------------------------------------------------------------------------
// Layout kind enum + selector
// ---------------------------------------------------------------------------

/// Available layout kinds for output binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutKind {
    Dwindle,
    Master,
    Stacking,
}

/// Select a layout description string for a [`WmMode`] or an explicit kind.
///
/// Returns the layout display name.
pub fn layout_of_name(kind: LayoutKind) -> &'static str {
    match kind {
        LayoutKind::Dwindle => "dwindle",
        LayoutKind::Master => "master-stack",
        LayoutKind::Stacking => "stacking",
    }
}

/// A unified layout container (Hyprland-inspired).
pub enum Layout {
    Dwindle(DwindleLayout),
    MasterStack(MasterStackLayout),
}

impl Layout {
    pub fn add_window(&mut self, id: u64) {
        let dummy = Rect { x: 0, y: 0, w: 1920, h: 1080 };
        match self {
            Layout::Dwindle(l) => l.add_window(id, dummy),
            Layout::MasterStack(l) => l.add_window(id),
        }
    }

    pub fn remove_window(&mut self, id: u64) {
        let dummy = Rect { x: 0, y: 0, w: 1920, h: 1080 };
        match self {
            Layout::Dwindle(l) => { l.remove_window(id, dummy); }
            Layout::MasterStack(l) => { l.remove_window(id); }
        }
    }

    pub fn window_count(&self) -> usize {
        match self {
            Layout::Dwindle(l) => l.count(),
            Layout::MasterStack(l) => l.count(),
        }
    }
}

/// Construct a [`Layout`] of the requested kind.
pub fn layout_of(kind: LayoutKind) -> Layout {
    match kind {
        LayoutKind::Dwindle => Layout::Dwindle(DwindleLayout::new(LayoutConfig::default())),
        LayoutKind::Master | LayoutKind::Stacking => Layout::MasterStack(MasterStackLayout::new(LayoutConfig::default())),
    }
}

// ---------------------------------------------------------------------------
// Inline tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn screen() -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: 1920,
            h: 1080,
        }
    }

    #[test]
    fn dwindle_single_window_fills_screen() {
        let cfg = LayoutConfig::default();
        let mut d = DwindleLayout::new(cfg);
        d.add_window(1, screen());
        let g = d.geometry(1).unwrap();
        assert_eq!(g, screen());
        assert_eq!(d.count(), 1);
    }

    #[test]
    fn dwindle_two_windows_split_horizontally() {
        let cfg = LayoutConfig { gaps: 0, border_px: 0 };
        let mut d = DwindleLayout::new(cfg);
        let s = Rect { x: 0, y: 0, w: 1000, h: 800 };
        d.add_window(1, s);
        d.add_window(2, s);
        let g1 = d.geometry(1).unwrap();
        let g2 = d.geometry(2).unwrap();
        // First window gets left half, second gets right half
        assert_eq!(g1.w, 500);
        assert_eq!(g2.w, 500);
        assert_eq!(g1.x, 0);
        assert_eq!(g2.x, 500);
    }

    #[test]
    fn dwindle_remove_window_recomputes() {
        let mut d = DwindleLayout::new(LayoutConfig::default());
        let s = screen();
        d.add_window(1, s);
        d.add_window(2, s);
        d.add_window(3, s);
        assert_eq!(d.count(), 3);
        assert!(d.remove_window(2, s));
        assert_eq!(d.count(), 2);
    }

    #[test]
    fn master_stack_single_window_takes_full_screen() {
        let mut ms = MasterStackLayout::new(LayoutConfig::default());
        ms.add_window(1);
        let result = ms.compute(screen()).unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, screen()); // master = full screen
        assert!(result.2.is_empty());
    }

    #[test]
    fn master_stack_two_windows_split() {
        let cfg = LayoutConfig { gaps: 0, border_px: 0 };
        let mut ms = MasterStackLayout::new(cfg);
        ms.add_window(10);
        ms.add_window(20);
        let s = Rect { x: 0, y: 0, w: 1000, h: 800 };
        let (master_id, master_rect, stack) = ms.compute(s).unwrap();
        assert_eq!(master_id, 10);
        assert_eq!(master_rect.w, 500);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].0, 20);
    }

    #[test]
    fn master_stack_promote() {
        let mut ms = MasterStackLayout::new(LayoutConfig::default());
        ms.add_window(1);
        ms.add_window(2);
        ms.add_window(3);
        ms.promote(3);
        let (master_id, _, _) = ms.compute(screen()).unwrap();
        assert_eq!(master_id, 3);
    }

    #[test]
    fn layout_of_returns_correct_names() {
        assert_eq!(layout_of(LayoutKind::Dwindle), "dwindle");
        assert_eq!(layout_of(LayoutKind::Master), "master-stack");
        assert_eq!(layout_of(LayoutKind::Stacking), "stacking");
    }

    #[test]
    fn rect_split_horizontal_no_gap() {
        let r = Rect { x: 0, y: 0, w: 1000, h: 600 };
        let (a, b) = r.split(SplitAxis::Horizontal, 0);
        assert_eq!(a.w, 500);
        assert_eq!(b.w, 500);
        assert_eq!(b.x, 500);
    }

    #[test]
    fn rect_split_vertical_no_gap() {
        let r = Rect { x: 0, y: 0, w: 800, h: 600 };
        let (a, b) = r.split(SplitAxis::Vertical, 0);
        assert_eq!(a.h, 300);
        assert_eq!(b.h, 300);
        assert_eq!(b.y, 300);
    }
}
