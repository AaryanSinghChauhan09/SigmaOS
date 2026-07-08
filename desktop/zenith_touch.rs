// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// desktop/zenith_touch.rs — Zenith Touch and Gesture Support
//
// Implements touch and gesture support for tablets and touch-enabled devices
// including multi-touch tracking, gesture recognition, and touch event handling
//
// Language: Rust (std for userland services)

use std::collections::HashMap;

// ─── Touch Point ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchPoint {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32, // 0.0 to 1.0
    pub major_axis: f32,
    pub minor_axis: f32,
    pub orientation: f32, // radians
    pub active: bool,
}

// ─── Touch Event ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum TouchEventType {
    Down,
    Move,
    Up,
    Cancel,
}

#[derive(Debug, Clone)]
pub struct TouchEvent {
    pub event_type: TouchEventType,
    pub points: Vec<TouchPoint>,
    pub timestamp: u64,
}

// ─── Gesture Types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GestureType {
    Tap,
    DoubleTap,
    LongPress,
    Swipe,
    Pinch,
    Rotate,
    Pan,
    Scroll,
}

#[derive(Debug, Clone)]
pub struct GestureEvent {
    pub gesture_type: GestureType,
    pub center_x: f32,
    pub center_y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub scale: f32,
    pub rotation: f32,
    pub velocity: f32,
    pub timestamp: u64,
}

// ─── Touch State ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TouchState {
    pub active_points: HashMap<u32, TouchPoint>,
    pub gesture_start_points: HashMap<u32, TouchPoint>,
    pub gesture_start_time: u64,
    pub last_gesture_time: u64,
    pub last_center_x: f32,
    pub last_center_y: f32,
    pub last_scale: f32,
    pub last_rotation: f32,
}

impl TouchState {
    pub fn new() -> Self {
        TouchState {
            active_points: HashMap::new(),
            gesture_start_points: HashMap::new(),
            gesture_start_time: 0,
            last_gesture_time: 0,
            last_center_x: 0.0,
            last_center_y: 0.0,
            last_scale: 1.0,
            last_rotation: 0.0,
        }
    }
}

// ─── Gesture Configuration ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct GestureConfig {
    pub tap_timeout: u64, // milliseconds
    pub double_tap_timeout: u64,
    pub long_press_timeout: u64,
    pub swipe_threshold: f32,
    pub pinch_threshold: f32,
    pub rotate_threshold: f32,
    pub scroll_threshold: f32,
    pub min_velocity: f32,
}

impl Default for GestureConfig {
    fn default() -> Self {
        GestureConfig {
            tap_timeout: 300,
            double_tap_timeout: 500,
            long_press_timeout: 500,
            swipe_threshold: 50.0,
            pinch_threshold: 10.0,
            rotate_threshold: 0.1,
            scroll_threshold: 10.0,
            min_velocity: 100.0,
        }
    }
}

// ─── Touch Manager ───────────────────────────────────────────────────────────

pub struct TouchManager {
    pub state: TouchState,
    pub config: GestureConfig,
    pub enabled: bool,
    pub multi_touch_enabled: bool,
    pub initialized: bool,
}

impl TouchManager {
    pub fn new() -> Self {
        TouchManager {
            state: TouchState::new(),
            config: GestureConfig::default(),
            enabled: true,
            multi_touch_enabled: true,
            initialized: false,
        }
    }

    /// Initialize touch manager
    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Process touch event
    pub fn process_event(&mut self, event: &TouchEvent) -> Vec<GestureEvent> {
        let mut gestures = Vec::new();

        match event.event_type {
            TouchEventType::Down => {
                for point in &event.points {
                    self.state.active_points.insert(point.id, *point);
                    self.state.gesture_start_points.insert(point.id, *point);
                }
                if self.state.gesture_start_time == 0 {
                    self.state.gesture_start_time = event.timestamp;
                }
            }
            TouchEventType::Move => {
                for point in &event.points {
                    if let Some(existing) = self.state.active_points.get_mut(&point.id) {
                        *existing = *point;
                    }
                }
                gestures.extend(self.detect_gestures(event));
            }
            TouchEventType::Up => {
                for point in &event.points {
                    self.state.active_points.remove(&point.id);
                }
                gestures.extend(self.detect_end_gestures(event));
                if self.state.active_points.is_empty() {
                    self.state.gesture_start_time = 0;
                    self.state.gesture_start_points.clear();
                }
            }
            TouchEventType::Cancel => {
                self.state.active_points.clear();
                self.state.gesture_start_points.clear();
                self.state.gesture_start_time = 0;
            }
        }

        gestures
    }

    /// Detect ongoing gestures
    fn detect_gestures(&mut self, event: &TouchEvent) -> Vec<GestureEvent> {
        let mut gestures = Vec::new();
        let point_count = self.state.active_points.len() as f32;

        if point_count == 0 {
            return gestures;
        }

        let (center_x, center_y) = self.calculate_center();
        let now = event.timestamp;

        // Detect pinch
        if point_count >= 2.0 && self.multi_touch_enabled {
            let scale = self.calculate_scale();
            if (scale - self.state.last_scale).abs() > self.config.pinch_threshold {
                gestures.push(GestureEvent {
                    gesture_type: GestureType::Pinch,
                    center_x,
                    center_y,
                    delta_x: 0.0,
                    delta_y: 0.0,
                    scale,
                    rotation: 0.0,
                    velocity: 0.0,
                    timestamp: now,
                });
                self.state.last_scale = scale;
            }
        }

        // Detect rotation
        if point_count >= 2.0 && self.multi_touch_enabled {
            let rotation = self.calculate_rotation();
            if (rotation - self.state.last_rotation).abs() > self.config.rotate_threshold {
                gestures.push(GestureEvent {
                    gesture_type: GestureType::Rotate,
                    center_x,
                    center_y,
                    delta_x: 0.0,
                    delta_y: 0.0,
                    scale: 1.0,
                    rotation,
                    velocity: 0.0,
                    timestamp: now,
                });
                self.state.last_rotation = rotation;
            }
        }

        // Detect pan/scroll
        let delta_x = center_x - self.state.last_center_x;
        let delta_y = center_y - self.state.last_center_y;
        let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();

        if distance > self.config.scroll_threshold {
            let elapsed = (now - self.state.last_gesture_time) as f32 / 1000.0;
            let velocity = if elapsed > 0.0 { distance / elapsed } else { 0.0 };

            if point_count == 1.0 {
                gestures.push(GestureEvent {
                    gesture_type: GestureType::Pan,
                    center_x,
                    center_y,
                    delta_x,
                    delta_y,
                    scale: 1.0,
                    rotation: 0.0,
                    velocity,
                    timestamp: now,
                });
            } else {
                gestures.push(GestureEvent {
                    gesture_type: GestureType::Scroll,
                    center_x,
                    center_y,
                    delta_x,
                    delta_y,
                    scale: 1.0,
                    rotation: 0.0,
                    velocity,
                    timestamp: now,
                });
            }

            self.state.last_center_x = center_x;
            self.state.last_center_y = center_y;
            self.state.last_gesture_time = now;
        }

        gestures
    }

    /// Detect end gestures (tap, double tap, long press, swipe)
    fn detect_end_gestures(&mut self, event: &TouchEvent) -> Vec<GestureEvent> {
        let mut gestures = Vec::new();
        let now = event.timestamp;
        let elapsed = now - self.state.gesture_start_time;

        if self.state.active_points.len() == 1 {
            if let Some((_, start_point)) = self.state.gesture_start_points.iter().next() {
                if let Some((_, end_point)) = event.points.iter().enumerate().next() {
                    let delta_x = end_point.x - start_point.x;
                    let delta_y = end_point.y - start_point.y;
                    let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();

                    // Detect tap
                    if elapsed < self.config.tap_timeout && distance < self.config.swipe_threshold {
                        gestures.push(GestureEvent {
                            gesture_type: GestureType::Tap,
                            center_x: end_point.x,
                            center_y: end_point.y,
                            delta_x: 0.0,
                            delta_y: 0.0,
                            scale: 1.0,
                            rotation: 0.0,
                            velocity: 0.0,
                            timestamp: now,
                        });
                    }
                    // Detect long press
                    else if elapsed > self.config.long_press_timeout && distance < self.config.swipe_threshold {
                        gestures.push(GestureEvent {
                            gesture_type: GestureType::LongPress,
                            center_x: end_point.x,
                            center_y: end_point.y,
                            delta_x: 0.0,
                            delta_y: 0.0,
                            scale: 1.0,
                            rotation: 0.0,
                            velocity: 0.0,
                            timestamp: now,
                        });
                    }
                    // Detect swipe
                    else if distance > self.config.swipe_threshold {
                        let elapsed_sec = elapsed as f32 / 1000.0;
                        let velocity = if elapsed_sec > 0.0 { distance / elapsed_sec } else { 0.0 };

                        if velocity > self.config.min_velocity {
                            gestures.push(GestureEvent {
                                gesture_type: GestureType::Swipe,
                                center_x: end_point.x,
                                center_y: end_point.y,
                                delta_x,
                                delta_y,
                                scale: 1.0,
                                rotation: 0.0,
                                velocity,
                                timestamp: now,
                            });
                        }
                    }
                }
            }
        }

        gestures
    }

    /// Calculate center of active touch points
    fn calculate_center(&self) -> (f32, f32) {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let count = self.state.active_points.len() as f32;

        if count == 0.0 {
            return (0.0, 0.0);
        }

        for point in self.state.active_points.values() {
            sum_x += point.x;
            sum_y += point.y;
        }

        (sum_x / count, sum_y / count)
    }

    /// Calculate scale (for pinch gesture)
    fn calculate_scale(&self) -> f32 {
        let points: Vec<&TouchPoint> = self.state.active_points.values().collect();
        if points.len() < 2 {
            return 1.0;
        }

        let p0 = points[0];
        let p1 = points[1];
        let current_distance = ((p1.x - p0.x).powi(2) + (p1.y - p0.y).powi(2)).sqrt();

        if let Some(start_p0) = self.state.gesture_start_points.get(&p0.id) {
            if let Some(start_p1) = self.state.gesture_start_points.get(&p1.id) {
                let start_distance = ((start_p1.x - start_p0.x).powi(2) + (start_p1.y - start_p0.y).powi(2)).sqrt();
                if start_distance > 0.0 {
                    return current_distance / start_distance;
                }
            }
        }

        1.0
    }

    /// Calculate rotation (for rotate gesture)
    fn calculate_rotation(&self) -> f32 {
        let points: Vec<&TouchPoint> = self.state.active_points.values().collect();
        if points.len() < 2 {
            return 0.0;
        }

        let p0 = points[0];
        let p1 = points[1];
        let current_angle = (p1.y - p0.y).atan2(p1.x - p0.x);

        if let Some(start_p0) = self.state.gesture_start_points.get(&p0.id) {
            if let Some(start_p1) = self.state.gesture_start_points.get(&p1.id) {
                let start_angle = (start_p1.y - start_p0.y).atan2(start_p1.x - start_p0.x);
                return current_angle - start_angle;
            }
        }

        0.0
    }

    /// Enable/disable touch
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Enable/disable multi-touch
    pub fn set_multi_touch_enabled(&mut self, enabled: bool) {
        self.multi_touch_enabled = enabled;
    }

    /// Set gesture configuration
    pub fn set_config(&mut self, config: GestureConfig) {
        self.config = config;
    }

    /// Get active touch points
    pub fn get_active_points(&self) -> Vec<TouchPoint> {
        self.state.active_points.values().cloned().collect()
    }

    /// Check if touch is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if multi-touch is enabled
    pub fn is_multi_touch_enabled(&self) -> bool {
        self.multi_touch_enabled
    }
}

// ─── Touch Event Handler Trait ───────────────────────────────────────────────

pub trait TouchEventHandler {
    fn on_touch_event(&mut self, event: &TouchEvent);
    fn on_gesture(&mut self, gesture: &GestureEvent);
}

// ─── Default Touch Event Handler ───────────────────────────────────────────────

pub struct DefaultTouchHandler {
    pub touch_manager: TouchManager,
}

impl DefaultTouchHandler {
    pub fn new() -> Self {
        DefaultTouchHandler {
            touch_manager: TouchManager::new(),
        }
    }

    pub fn init(&mut self) {
        self.touch_manager.init();
    }
}

impl TouchEventHandler for DefaultTouchHandler {
    fn on_touch_event(&mut self, event: &TouchEvent) {
        let gestures = self.touch_manager.process_event(event);
        for gesture in gestures {
            self.on_gesture(&gesture);
        }
    }

    fn on_gesture(&mut self, gesture: &GestureEvent) {
        // Default implementation - can be overridden
        match gesture.gesture_type {
            GestureType::Tap => {
                // Handle tap
            }
            GestureType::DoubleTap => {
                // Handle double tap
            }
            GestureType::LongPress => {
                // Handle long press
            }
            GestureType::Swipe => {
                // Handle swipe
            }
            GestureType::Pinch => {
                // Handle pinch
            }
            GestureType::Rotate => {
                // Handle rotate
            }
            GestureType::Pan => {
                // Handle pan
            }
            GestureType::Scroll => {
                // Handle scroll
            }
        }
    }
}
