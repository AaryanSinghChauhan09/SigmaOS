use crate::klib::Vec;
/// OOP-based Accessibility Keyboard for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 836
/// Implements on-screen keyboard and accessibility input
use core::sync::atomic::{AtomicUsize, Ordering};

pub type KeyID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    Character = 0,
    Modifier = 1,
    Function = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardError {
    Success = 0,
    NotFound = 1,
}

pub trait VirtualKey {
    fn id(&self) -> KeyID;
    fn label(&self) -> &[u8];
    fn key_type(&self) -> KeyType;
    fn is_pressed(&self) -> bool;
    fn set_pressed(&self, pressed: bool);
}

#[repr(C)]
pub struct SimpleVirtualKey {
    pub id: KeyID,
    pub label: [u8; 8],
    pub key_type: AtomicUsize,
    pub pressed: AtomicUsize,
}

impl SimpleVirtualKey {
    pub fn new(id: KeyID, label: &[u8], key_type: KeyType) -> Self {
        let mut label_array = [0u8; 8];
        let label_len = label.len().min(7);
        for i in 0..label_len {
            label_array[i] = label[i];
        }
        SimpleVirtualKey {
            id,
            label: label_array,
            key_type: AtomicUsize::new(key_type as usize),
            pressed: AtomicUsize::new(0),
        }
    }
}

impl VirtualKey for SimpleVirtualKey {
    fn id(&self) -> KeyID {
        self.id
    }
    fn label(&self) -> &[u8] {
        let len = self.label.iter().position(|&b| b == 0).unwrap_or(8);
        &self.label[..len]
    }
    fn key_type(&self) -> KeyType {
        match self.key_type.load(Ordering::SeqCst) {
            0 => KeyType::Character,
            1 => KeyType::Modifier,
            _ => KeyType::Function,
        }
    }
    fn is_pressed(&self) -> bool {
        self.pressed.load(Ordering::SeqCst) == 1
    }
    fn set_pressed(&self, pressed: bool) {
        self.pressed
            .store(if pressed { 1 } else { 0 }, Ordering::SeqCst);
    }
}

pub trait OnScreenKeyboard {
    fn press_key(&mut self, key_id: KeyID) -> Result<(), KeyboardError>;
    fn release_key(&mut self, key_id: KeyID) -> Result<(), KeyboardError>;
    fn get_key(&self, id: KeyID) -> Option<&dyn VirtualKey>;
    fn set_layout(&mut self, layout: &[u8]);
}

#[repr(C)]
pub struct SimpleOnScreenKeyboard {
    pub keys: Vec<Option<Box<dyn VirtualKey>>>,
    pub layout: [u8; 32],
    pub next_id: AtomicUsize,
}

impl SimpleOnScreenKeyboard {
    pub fn new() -> Self {
        let mut layout_array = [0u8; 32];
        let layout_len = b"QWERTY".len().min(31);
        for i in 0..layout_len {
            layout_array[i] = b"QWERTY"[i];
        }
        SimpleOnScreenKeyboard {
            keys: Vec::new(),
            layout: layout_array,
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OnScreenKeyboard for SimpleOnScreenKeyboard {
    fn press_key(&mut self, key_id: KeyID) -> Result<(), KeyboardError> {
        for key_option in &mut self.keys {
            if let Some(ref mut key) = *key_option {
                if key.id() == key_id {
                    key.set_pressed(true);
                    return Ok(());
                }
            }
        }
        Err(KeyboardError::NotFound)
    }

    fn release_key(&mut self, key_id: KeyID) -> Result<(), KeyboardError> {
        for key_option in &mut self.keys {
            if let Some(ref mut key) = *key_option {
                if key.id() == key_id {
                    key.set_pressed(false);
                    return Ok(());
                }
            }
        }
        Err(KeyboardError::NotFound)
    }

    fn get_key(&self, id: KeyID) -> Option<&dyn VirtualKey> {
        for key_option in &self.keys {
            if let Some(ref key) = *key_option {
                if key.id() == id {
                    return Some(key.as_ref());
                }
            }
        }
        None
    }

    fn set_layout(&mut self, layout: &[u8]) {
        let layout_len = layout.len().min(31);
        for i in 0..layout_len {
            self.layout[i] = layout[i];
        }
    }
}

pub trait StickyKeys {
    fn enable_sticky(&mut self, key_id: KeyID);
    fn disable_sticky(&mut self, key_id: KeyID);
    fn is_sticky(&self, key_id: KeyID) -> bool;
}

#[repr(C)]
pub struct SimpleStickyKeys {
    pub sticky_keys: Vec<KeyID>,
}

impl SimpleStickyKeys {
    pub fn new() -> Self {
        SimpleStickyKeys {
            sticky_keys: Vec::new(),
        }
    }
}

impl StickyKeys for SimpleStickyKeys {
    fn enable_sticky(&mut self, key_id: KeyID) {
        if !self.sticky_keys.contains(&key_id) {
            self.sticky_keys.push(key_id);
        }
    }

    fn disable_sticky(&mut self, key_id: KeyID) {
        for i in 0..self.sticky_keys.len() {
            if self.sticky_keys[i] == key_id {
                self.sticky_keys.remove(i);
                return;
            }
        }
    }

    fn is_sticky(&self, key_id: KeyID) -> bool {
        self.sticky_keys.contains(&key_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_keys() {
        let mut keyboard = SimpleOnScreenKeyboard::new();
        let key = SimpleVirtualKey::new(10, b"Ctrl", KeyType::Modifier);
        keyboard.keys.push(Some(Box::new(key)));

        assert!(keyboard.get_key(10).is_some());
        assert_eq!(keyboard.get_key(10).unwrap().key_type(), KeyType::Modifier);
        assert!(!keyboard.get_key(10).unwrap().is_pressed());

        keyboard.press_key(10).unwrap();
        assert!(keyboard.get_key(10).unwrap().is_pressed());

        keyboard.release_key(10).unwrap();
        assert!(!keyboard.get_key(10).unwrap().is_pressed());
    }

    #[test]
    fn test_sticky_keys() {
        let mut sticky = SimpleStickyKeys::new();
        assert!(!sticky.is_sticky(5));
        sticky.enable_sticky(5);
        assert!(sticky.is_sticky(5));
        sticky.disable_sticky(5);
        assert!(!sticky.is_sticky(5));
    }
}

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
