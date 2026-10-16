// SigmaOS Libinput Touchpad Gesture Recognition Subsystem
// Reference: libinput (freedesktop.org), Linux evdev multitouch protocol B

#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;

/// Gesture Lifecycle State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureState {
    Begin,
    Update,
    End,
    Cancelled,
}

/// Gesture Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureType {
    Swipe,
    Pinch,
    Hold,
}

/// Swipe Gesture Event (e.g. 3-finger / 4-finger navigation or workspace switch)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GestureSwipeEvent {
    pub fingers: u32,
    pub state: GestureState,
    pub dx: f64,
    pub dy: f64,
    pub dx_unaccelerated: f64,
    pub dy_unaccelerated: f64,
}

/// Pinch Gesture Event (e.g. 2-finger zoom and rotate)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GesturePinchEvent {
    pub fingers: u32,
    pub state: GestureState,
    pub dx: f64,
    pub dy: f64,
    pub scale: f64,
    pub angle_delta: f64, // in degrees
}

/// Hold Gesture Event
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GestureHoldEvent {
    pub fingers: u32,
    pub state: GestureState,
}

/// Libinput Gesture Recognition Engine
pub struct LibinputGestureEngine {
    active_fingers: u32,
    accumulated_dx: f64,
    accumulated_dy: f64,
    current_scale: f64,
    swipe_threshold: f64,
    pinch_threshold: f64,
    gesture_in_progress: Option<GestureType>,
}

impl LibinputGestureEngine {
    pub fn new() -> Self {
        Self {
            active_fingers: 0,
            accumulated_dx: 0.0,
            accumulated_dy: 0.0,
            current_scale: 1.0,
            swipe_threshold: 15.0,
            pinch_threshold: 0.1,
            gesture_in_progress: None,
        }
    }

    /// Update touch contact point count
    pub fn update_touch_count(&mut self, fingers: u32) {
        if self.active_fingers != fingers {
            if fingers == 0 {
                self.reset();
            }
            self.active_fingers = fingers;
        }
    }

    /// Process motion delta from multiple touch slots
    pub fn process_touch_motion(&mut self, dx: f64, dy: f64) -> Option<GestureSwipeEvent> {
        if self.active_fingers < 3 {
            return None;
        }

        self.accumulated_dx += dx;
        self.accumulated_dy += dy;

        let total_motion = (self.accumulated_dx.powi(2) + self.accumulated_dy.powi(2)).sqrt();

        if self.gesture_in_progress.is_none() && total_motion >= self.swipe_threshold {
            self.gesture_in_progress = Some(GestureType::Swipe);
            return Some(GestureSwipeEvent {
                fingers: self.active_fingers,
                state: GestureState::Begin,
                dx,
                dy,
                dx_unaccelerated: dx,
                dy_unaccelerated: dy,
            });
        }

        if self.gesture_in_progress == Some(GestureType::Swipe) {
            return Some(GestureSwipeEvent {
                fingers: self.active_fingers,
                state: GestureState::Update,
                dx,
                dy,
                dx_unaccelerated: dx,
                dy_unaccelerated: dy,
            });
        }

        None
    }

    /// Process two-finger pinch-to-zoom distance delta
    pub fn process_pinch_delta(
        &mut self,
        distance_ratio: f64,
        angle_delta: f64,
    ) -> Option<GesturePinchEvent> {
        if self.active_fingers != 2 {
            return None;
        }

        self.current_scale *= distance_ratio;
        let diff = (self.current_scale - 1.0).abs();

        if self.gesture_in_progress.is_none() && diff >= self.pinch_threshold {
            self.gesture_in_progress = Some(GestureType::Pinch);
            return Some(GesturePinchEvent {
                fingers: 2,
                state: GestureState::Begin,
                dx: 0.0,
                dy: 0.0,
                scale: self.current_scale,
                angle_delta,
            });
        }

        if self.gesture_in_progress == Some(GestureType::Pinch) {
            return Some(GesturePinchEvent {
                fingers: 2,
                state: GestureState::Update,
                dx: 0.0,
                dy: 0.0,
                scale: self.current_scale,
                angle_delta,
            });
        }

        None
    }

    /// End ongoing gesture
    pub fn end_gesture(&mut self) -> Option<GestureState> {
        if self.gesture_in_progress.is_some() {
            self.reset();
            Some(GestureState::End)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.accumulated_dx = 0.0;
        self.accumulated_dy = 0.0;
        self.current_scale = 1.0;
        self.gesture_in_progress = None;
    }
}
