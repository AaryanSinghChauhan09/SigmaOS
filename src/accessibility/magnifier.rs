use crate::klib::Vec;
/// OOP-based Screen Magnifier for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 826
/// Implements screen magnification and zoom
use core::sync::atomic::{AtomicUsize, Ordering};

pub type MagnifierID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagnifierError {
    Success = 0,
    NotFound = 1,
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
                if (&**magnifier).id() == id {
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
                if (&**magnifier).id() == id {
                    return Some(&**magnifier);
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
}
