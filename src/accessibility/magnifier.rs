extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type MagnifierID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagnifierError {
    Success = 0,
    NotFound = 1,
    InvalidZoom = 2,
}

pub trait Magnifier {
    fn id(&self) -> MagnifierID;
    fn zoom_level(&self) -> f32;
    fn set_zoom_level(&mut self, level: f32);
    fn follow_cursor(&self) -> bool;
    fn set_follow_cursor(&mut self, follow: bool);
}

#[repr(C)]
pub struct SimpleMagnifier {
    pub id: MagnifierID,
    pub zoom_level: AtomicUsize,
    pub follow_cursor: AtomicUsize,
}

impl SimpleMagnifier {
    pub fn new(id: MagnifierID) -> Self {
        SimpleMagnifier {
            id,
            zoom_level: AtomicUsize::new(200),
            follow_cursor: AtomicUsize::new(1),
        }
    }
}

impl Magnifier for SimpleMagnifier {
    fn id(&self) -> MagnifierID {
        self.id
    }
    fn zoom_level(&self) -> f32 {
        (self.zoom_level.load(Ordering::SeqCst) as f32) / 100.0
    }

    fn set_zoom_level(&mut self, level: f32) {
        self.zoom_level
            .store((level * 100.0) as usize, Ordering::SeqCst);
    }

    fn follow_cursor(&self) -> bool {
        self.follow_cursor.load(Ordering::SeqCst) == 1
    }

    fn set_follow_cursor(&mut self, follow: bool) {
        self.follow_cursor
            .store(if follow { 1 } else { 0 }, Ordering::SeqCst);
    }
}

pub trait MagnifierManager {
    fn create_magnifier(&mut self) -> Result<MagnifierID, MagnifierError>;
    fn destroy_magnifier(&mut self, id: MagnifierID) -> Result<(), MagnifierError>;
    fn get_magnifier(&self, id: MagnifierID) -> Option<&dyn Magnifier>;
}

#[repr(C)]
pub struct SimpleMagnifierManager {
    pub magnifiers: Vec<Option<Box<dyn Magnifier>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMagnifierManager {
    pub fn new() -> Self {
        SimpleMagnifierManager {
            magnifiers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MagnifierManager for SimpleMagnifierManager {
    fn create_magnifier(&mut self) -> Result<MagnifierID, MagnifierError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let magnifier = SimpleMagnifier::new(id);
        self.magnifiers.push(Some(Box::new(magnifier)));
        Ok(id)
    }

    fn destroy_magnifier(&mut self, id: MagnifierID) -> Result<(), MagnifierError> {
        for magnifier_option in &mut self.magnifiers {
            if let Some(ref magnifier) = *magnifier_option {
                if magnifier.id() == id {
                    *magnifier_option = None;
                    return Ok(());
                }
            }
        }
        Err(MagnifierError::NotFound)
    }

    fn get_magnifier(&self, id: MagnifierID) -> Option<&dyn Magnifier> {
        for magnifier_option in &self.magnifiers {
            if let Some(ref magnifier) = *magnifier_option {
                if magnifier.id() == id {
                    return Some(magnifier.as_ref());
                }
            }
        }
        None
    }
}

pub trait ColorFilter {
    fn enable_filter(&mut self, filter_type: u8);
    fn disable_filter(&mut self);
    fn is_filter_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleColorFilter {
    pub enabled: AtomicUsize,
    pub filter_type: AtomicUsize,
}

impl SimpleColorFilter {
    pub fn new() -> Self {
        SimpleColorFilter {
            enabled: AtomicUsize::new(0),
            filter_type: AtomicUsize::new(0),
        }
    }
}

impl ColorFilter for SimpleColorFilter {
    fn enable_filter(&mut self, filter_type: u8) {
        self.enabled.store(1, Ordering::SeqCst);
        self.filter_type
            .store(filter_type as usize, Ordering::SeqCst);
    }

    fn disable_filter(&mut self) {
        self.enabled.store(0, Ordering::SeqCst);
    }

    fn is_filter_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) == 1
    }
}

// =========================================================================
// ORCA & WINDOWS MAGNIFIER PARITY SCREEN MAGNIFIER ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagnifierViewMode {
    FullScreen,
    Lens,
    Docked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagnifierTrackingMode {
    FollowCursor,
    FollowFocus,
    FollowTextCaret,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorDeficiencyFilter {
    None,
    InvertedGrayscale,
    HighContrast,
    Protanopia,   // Red-blind
    Deuteranopia, // Green-blind
    Tritanopia,   // Blue-blind
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Advanced Orca & Windows Magnifier Parity Subsystem
pub struct SovereignMagnifierEngine {
    pub zoom_level: f32, // 1.0 (100%) to 16.0 (1600%)
    pub view_mode: MagnifierViewMode,
    pub tracking_mode: MagnifierTrackingMode,
    pub color_filter: ColorDeficiencyFilter,
    pub lens_size: (f32, f32), // Width, Height in lens mode
    pub current_center: ScreenPoint,
    pub screen_bounds: ScreenRect,
}

impl SovereignMagnifierEngine {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            zoom_level: 2.0,
            view_mode: MagnifierViewMode::FullScreen,
            tracking_mode: MagnifierTrackingMode::FollowCursor,
            color_filter: ColorDeficiencyFilter::None,
            lens_size: (300.0, 200.0),
            current_center: ScreenPoint {
                x: screen_width / 2.0,
                y: screen_height / 2.0,
            },
            screen_bounds: ScreenRect {
                x: 0.0,
                y: 0.0,
                width: screen_width,
                height: screen_height,
            },
        }
    }

    pub fn set_zoom(&mut self, level: f32) -> Result<(), MagnifierError> {
        if level < 1.0 || level > 16.0 {
            return Err(MagnifierError::InvalidZoom);
        }
        self.zoom_level = level;
        Ok(())
    }

    pub fn update_tracking(&mut self, cursor_pos: ScreenPoint, focus_rect: Option<ScreenRect>, text_caret: Option<ScreenPoint>) {
        let target = match self.tracking_mode {
            MagnifierTrackingMode::FollowCursor => cursor_pos,
            MagnifierTrackingMode::FollowFocus => {
                if let Some(rect) = focus_rect {
                    ScreenPoint {
                        x: rect.x + rect.width / 2.0,
                        y: rect.y + rect.height / 2.0,
                    }
                } else {
                    cursor_pos
                }
            }
            MagnifierTrackingMode::FollowTextCaret => text_caret.unwrap_or(cursor_pos),
        };

        self.current_center.x = target.x.clamp(0.0, self.screen_bounds.width);
        self.current_center.y = target.y.clamp(0.0, self.screen_bounds.height);
    }

    pub fn calculate_magnified_viewport(&self) -> ScreenRect {
        let vp_width = self.screen_bounds.width / self.zoom_level;
        let vp_height = self.screen_bounds.height / self.zoom_level;

        let half_w = vp_width / 2.0;
        let half_h = vp_height / 2.0;

        let min_x = (self.current_center.x - half_w).clamp(0.0, self.screen_bounds.width - vp_width);
        let min_y = (self.current_center.y - half_h).clamp(0.0, self.screen_bounds.height - vp_height);

        ScreenRect {
            x: min_x,
            y: min_y,
            width: vp_width,
            height: vp_height,
        }
    }

    pub fn apply_color_filter_pixel(&self, rgb: (u8, u8, u8)) -> (u8, u8, u8) {
        let (r, g, b) = (rgb.0 as f32, rgb.1 as f32, rgb.2 as f32);

        match self.color_filter {
            ColorDeficiencyFilter::None => rgb,
            ColorDeficiencyFilter::InvertedGrayscale => {
                let gray = 0.299 * r + 0.587 * g + 0.114 * b;
                let inv = 255.0 - gray;
                let val = inv.clamp(0.0, 255.0) as u8;
                (val, val, val)
            }
            ColorDeficiencyFilter::HighContrast => {
                let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
                if gray > 128 {
                    (255, 255, 255)
                } else {
                    (0, 0, 0)
                }
            }
            ColorDeficiencyFilter::Protanopia => {
                let nr = (0.56667 * r + 0.43333 * g).clamp(0.0, 255.0) as u8;
                let ng = (0.55833 * r + 0.44167 * g).clamp(0.0, 255.0) as u8;
                let nb = (0.24167 * g + 0.75833 * b).clamp(0.0, 255.0) as u8;
                (nr, ng, nb)
            }
            ColorDeficiencyFilter::Deuteranopia => {
                let nr = (0.625 * r + 0.375 * g).clamp(0.0, 255.0) as u8;
                let ng = (0.70 * r + 0.30 * g).clamp(0.0, 255.0) as u8;
                let nb = (0.30 * g + 0.70 * b).clamp(0.0, 255.0) as u8;
                (nr, ng, nb)
            }
            ColorDeficiencyFilter::Tritanopia => {
                let nr = (0.95 * r + 0.05 * g).clamp(0.0, 255.0) as u8;
                let ng = (0.43333 * g + 0.56667 * b).clamp(0.0, 255.0) as u8;
                let nb = (0.475 * g + 0.525 * b).clamp(0.0, 255.0) as u8;
                (nr, ng, nb)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnifier_manager() {
        let mut manager = SimpleMagnifierManager::new();
        let id = manager.create_magnifier().unwrap();
        assert!(manager.get_magnifier(id).is_some());
        assert_eq!(manager.get_magnifier(id).unwrap().zoom_level(), 2.0);

        assert!(manager.destroy_magnifier(id).is_ok());
        assert!(manager.get_magnifier(id).is_none());
    }

    #[test]
    fn test_color_filter() {
        let mut filter = SimpleColorFilter::new();
        assert!(!filter.is_filter_enabled());
        filter.enable_filter(3);
        assert!(filter.is_filter_enabled());
    }

    #[test]
    fn test_sovereign_magnifier_engine_zoom_and_viewport() {
        let mut engine = SovereignMagnifierEngine::new(1920.0, 1080.0);
        assert_eq!(engine.zoom_level, 2.0);

        assert!(engine.set_zoom(4.0).is_ok());
        assert_eq!(engine.zoom_level, 4.0);

        assert_eq!(engine.set_zoom(0.5), Err(MagnifierError::InvalidZoom));

        let vp = engine.calculate_magnified_viewport();
        assert_eq!(vp.width, 480.0);
        assert_eq!(vp.height, 270.0);
    }

    #[test]
    fn test_sovereign_magnifier_tracking_and_filters() {
        let mut engine = SovereignMagnifierEngine::new(1920.0, 1080.0);
        engine.tracking_mode = MagnifierTrackingMode::FollowFocus;

        let focus_rect = ScreenRect {
            x: 500.0,
            y: 400.0,
            width: 100.0,
            height: 50.0,
        };
        engine.update_tracking(ScreenPoint { x: 10.0, y: 10.0 }, Some(focus_rect), None);

        assert_eq!(engine.current_center.x, 550.0);
        assert_eq!(engine.current_center.y, 425.0);

        // Test high contrast filter
        engine.color_filter = ColorDeficiencyFilter::HighContrast;
        assert_eq!(engine.apply_color_filter_pixel((200, 200, 200)), (255, 255, 255));
        assert_eq!(engine.apply_color_filter_pixel((10, 10, 10)), (0, 0, 0));
    }
}
