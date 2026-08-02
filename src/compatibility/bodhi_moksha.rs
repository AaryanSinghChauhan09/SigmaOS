// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Representation of an Enlightenment Foundation Libraries (EFL) Canvas Element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EflCanvasElement {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

/// Lightweight Moksha Desktop Profile Configurator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MokshaProfile {
    MinimalDistractionFree,
    LaptopsPowerSaver,
    StandardComposite,
}

/// Modular desktop layout components matching Bodhi Linux structures
pub struct MokshaDesktopManager {
    pub active_profile: MokshaProfile,
    pub panel_scale: u32, // zoom layout
    pub system_controls_enabled: bool,
    pub canvas_elements: [Option<EflCanvasElement>; 16],
}

impl MokshaDesktopManager {
    pub fn new() -> Self {
        Self {
            active_profile: MokshaProfile::MinimalDistractionFree,
            panel_scale: 100,
            system_controls_enabled: true,
            canvas_elements: [None; 16],
        }
    }

    pub fn switch_profile(&mut self, profile: MokshaProfile) {
        self.active_profile = profile;
        match profile {
            MokshaProfile::MinimalDistractionFree => {
                self.panel_scale = 80;
                self.system_controls_enabled = false;
            }
            MokshaProfile::LaptopsPowerSaver => {
                self.panel_scale = 90;
                self.system_controls_enabled = true;
            }
            MokshaProfile::StandardComposite => {
                self.panel_scale = 110;
                self.system_controls_enabled = true;
            }
        }
    }

    pub fn register_canvas_element(&mut self, element: EflCanvasElement) -> bool {
        for slot in &mut self.canvas_elements {
            if slot.is_none() {
                *slot = Some(element);
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moksha_desktop_profiles() {
        let mut manager = MokshaDesktopManager::new();
        assert_eq!(manager.active_profile, MokshaProfile::MinimalDistractionFree);

        manager.switch_profile(MokshaProfile::StandardComposite);
        assert_eq!(manager.active_profile, MokshaProfile::StandardComposite);
        assert_eq!(manager.panel_scale, 110);
        assert!(manager.system_controls_enabled);
    }

    #[test]
    fn test_efl_canvas_elements() {
        let mut manager = MokshaDesktopManager::new();
        let el = EflCanvasElement {
            id: 10,
            x: 0,
            y: 0,
            width: 1024,
            height: 768,
            visible: true,
        };
        assert!(manager.register_canvas_element(el));
        assert_eq!(manager.canvas_elements[0].unwrap().width, 1024);
    }
}
