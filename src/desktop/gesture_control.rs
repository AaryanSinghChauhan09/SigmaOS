#![no_std]

/// Gesture Control System for SigmaOS
/// Based on 100-Improvement-Ideas.md #47: Gesture control system
/// Implements touchpad and touchscreen gesture recognition

use core::sync::atomic::{AtomicU64, Ordering};

/// Gesture type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureType {
    Tap = 0,
    DoubleTap = 1,
    SwipeLeft = 2,
    SwipeRight = 3,
    SwipeUp = 4,
    SwipeDown = 5,
    Pinch = 6,
    Spread = 7,
    Rotate = 8,
    LongPress = 9,
}

/// Touch point
#[repr(C)]
pub struct TouchPoint {
    pub x: i32,
    pub y: i32,
    pub id: u32,
    pub pressure: u8,
}

impl TouchPoint {
    pub fn new(x: i32, y: i32, id: u32) -> Self {
        TouchPoint {
            x,
            y,
            id,
            pressure: 0,
        }
    }
}

/// Gesture event
#[repr(C)]
pub struct GestureEvent {
    pub gesture_type: GestureType,
    pub touch_points: [Option<TouchPoint>; 5],
    pub timestamp: u64,
    pub confidence: f32,
}

impl GestureEvent {
    pub fn new(gesture_type: GestureType) -> Self {
        GestureEvent {
            gesture_type,
            touch_points: [None, None, None, None, None],
            timestamp: get_current_time(),
            confidence: 0.0,
        }
    }
    
    pub fn add_touch_point(&mut self, touch_point: TouchPoint) {
        for i in 0..5 {
            if self.touch_points[i].is_none() {
                self.touch_points[i] = Some(touch_point);
                break;
            }
        }
    }
}

/// Gesture action
#[repr(C)]
pub enum GestureAction {
    None = 0,
    SwitchWorkspace = 1,
    MinimizeWindow = 2,
    MaximizeWindow = 3,
    CloseWindow = 4,
    OpenLauncher = 5,
    ShowDesktop = 6,
    Scroll = 7,
    Zoom = 8,
    Rotate = 9,
}

/// Gesture binding
#[repr(C)]
pub struct GestureBinding {
    pub gesture_type: GestureType,
    pub action: GestureAction,
    pub enabled: bool,
}

impl GestureBinding {
    pub fn new(gesture_type: GestureType, action: GestureAction) -> Self {
        GestureBinding {
            gesture_type,
            action,
            enabled: true,
        }
    }
}

/// Gesture recognizer
pub struct GestureRecognizer {
    pub bindings: Vec<Option<GestureBinding>>,
    pub touch_history: Vec<Option<TouchPoint>>,
    pub next_binding_id: AtomicU64,
}

impl GestureRecognizer {
    pub fn new() -> Self {
        GestureRecognizer {
            bindings: Vec::new(),
            touch_history: Vec::new(),
            next_binding_id: AtomicU64::new(1),
        }
    }
    
    /// Add gesture binding
    pub fn add_binding(&mut self, gesture_type: GestureType, action: GestureAction) {
        let binding = GestureBinding::new(gesture_type, action);
        self.bindings.push(Some(binding));
    }
    
    /// Remove gesture binding
    pub fn remove_binding(&mut self, gesture_type: GestureType) -> bool {
        for binding_option in &mut self.bindings {
            if let Some(ref binding) = *binding_option {
                if binding.gesture_type == gesture_type {
                    *binding_option = None;
                    return true;
                }
            }
        }
        false
    }
    
    /// Process touch points and recognize gesture
    pub fn process_touch(&mut self, touch_points: &[TouchPoint]) -> Option<GestureEvent> {
        // Store touch points in history
        for touch_point in touch_points {
            self.touch_history.push(Some(*touch_point));
        }
        
        // Limit history size
        if self.touch_history.len() > 10 {
            self.touch_history.remove(0);
        }
        
        // Recognize gesture based on touch history
        if let Some(gesture) = self.recognize_gesture() {
            Some(gesture)
        } else {
            None
        }
    }
    
    fn recognize_gesture(&self) -> Option<GestureEvent> {
        if self.touch_history.len() < 2 {
            return None;
        }
        
        let first = self.touch_history[0];
        let last = self.touch_history[self.touch_history.len() - 1];
        
        if let (Some(ref first_touch), Some(ref last_touch)) = (first, last) {
            let dx = last_touch.x - first_touch.x;
            let dy = last_touch.y - first_touch.y;
            
            // Simple gesture recognition
            if dx.abs() > 100 && dy.abs() < 50 {
                let gesture_type = if dx > 0 { GestureType::SwipeRight } else { GestureType::SwipeLeft };
                let mut event = GestureEvent::new(gesture_type);
                event.add_touch_point(*first_touch);
                event.add_touch_point(*last_touch);
                event.confidence = 0.85;
                return Some(event);
            }
            
            if dy.abs() > 100 && dx.abs() < 50 {
                let gesture_type = if dy > 0 { GestureType::SwipeDown } else { GestureType::SwipeUp };
                let mut event = GestureEvent::new(gesture_type);
                event.add_touch_point(*first_touch);
                event.add_touch_point(*last_touch);
                event.confidence = 0.85;
                return Some(event);
            }
        }
        
        None
    }
    
    /// Execute action for gesture
    pub fn execute_action(&self, gesture_type: GestureType) -> Option<GestureAction> {
        for binding_option in &self.bindings {
            if let Some(ref binding) = *binding_option {
                if binding.enabled && binding.gesture_type == gesture_type {
                    return Some(binding.action);
                }
            }
        }
        None
    }
    
    /// Initialize default bindings
    pub fn initialize_defaults(&mut self) {
        self.add_binding(GestureType::SwipeLeft, GestureAction::SwitchWorkspace);
        self.add_binding(GestureType::SwipeRight, GestureAction::SwitchWorkspace);
        self.add_binding(GestureType::SwipeUp, GestureAction::ShowDesktop);
        self.add_binding(GestureType::SwipeDown, GestureAction::OpenLauncher);
        self.add_binding(GestureType::Pinch, GestureAction::Zoom);
        self.add_binding(GestureType::Spread, GestureAction::Zoom);
    }
}

/// Gesture control manager
pub struct GestureControlManager {
    pub recognizer: GestureRecognizer,
    pub enabled: bool,
}

impl GestureControlManager {
    pub fn new() -> Self {
        let mut recognizer = GestureRecognizer::new();
        recognizer.initialize_defaults();
        
        GestureControlManager {
            recognizer,
            enabled: true,
        }
    }
    
    /// Enable/disable gesture control
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Process touch input
    pub fn process_input(&mut self, touch_points: &[TouchPoint]) -> Option<GestureAction> {
        if !self.enabled {
            return None;
        }
        
        if let Some(gesture) = self.recognizer.process_touch(touch_points) {
            self.recognizer.execute_action(gesture.gesture_type)
        } else {
            None
        }
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1_000_000, Ordering::SeqCst)
}
