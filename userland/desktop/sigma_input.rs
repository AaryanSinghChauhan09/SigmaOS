// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_input.rs — Input Manager (keyboard, mouse, touch, gestures)
// Language: Rust (std) — OOP via InputManager + gesture recognisers

use std::collections::{HashSet, VecDeque};
use std::time::{Instant, Duration};
use crate::userland::desktop::sigma_compositor::Point;

// ── Key ───────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Char(char), F(u8),
    Escape, Enter, Tab, Backspace, Delete, Space,
    Left, Right, Up, Down, Home, End, PageUp, PageDown,
    Super, Ctrl, Alt, Shift, AltGr,
    Print, ScrollLock, Pause, Insert, CapsLock, NumLock,
}

#[derive(Clone, Debug)]
pub struct KeyEvent { pub key: Key, pub pressed: bool, pub modifiers: Modifiers }

#[derive(Clone, Copy, Debug, Default)]
pub struct Modifiers { pub ctrl: bool, pub alt: bool, pub shift: bool, pub super_key: bool }

impl Modifiers {
    pub fn ctrl_shift(&self) -> bool { self.ctrl && self.shift }
    pub fn super_only(&self) -> bool { self.super_key && !self.ctrl && !self.alt && !self.shift }
}

// ── Mouse ─────────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton { Left, Right, Middle, X1, X2 }

#[derive(Clone, Debug)]
pub enum MouseEvent {
    Move  { pos: Point, delta: (i32,i32) },
    Press { pos: Point, btn: MouseButton },
    Release { pos: Point, btn: MouseButton },
    Scroll { pos: Point, dx: i32, dy: i32 },
    DblClick { pos: Point, btn: MouseButton },
}

// ── Touch ─────────────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct Touch { pub id: u32, pub pos: Point, pub pressure: f32 }

#[derive(Clone, Debug)]
pub enum TouchEvent { Down(Touch), Move(Touch), Up(Touch) }

// ── Gesture ───────────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub enum Gesture {
    Tap     { pos: Point, count: u8 },
    LongPress { pos: Point },
    Swipe   { direction: SwipeDir, distance: f32, velocity: f32 },
    Pinch   { scale: f32, center: Point },
    Pan     { delta: (i32,i32), velocity: (f32,f32) },
    TwoFingerTap { center: Point },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwipeDir { Up, Down, Left, Right }

// ── Input Event (unified) ─────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub enum InputEvent {
    Key(KeyEvent), Mouse(MouseEvent), Touch(TouchEvent), Gesture(Gesture),
}

// ── Shortcut ──────────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct Shortcut { pub key: Key, pub mods: Modifiers, pub action: String }

// ── Gesture Recogniser ────────────────────────────────────────────────────────
struct SwipeTracker {
    start:   Option<(Point, Instant)>,
    current: Point,
}
impl SwipeTracker {
    fn new() -> Self { Self { start: None, current: Point::default() } }
    fn touch_down(&mut self, p: Point) { self.start = Some((p, Instant::now())); self.current = p; }
    fn touch_move(&mut self, p: Point) { self.current = p; }
    fn touch_up(&mut self) -> Option<Gesture> {
        let (start, t) = self.start.take()?;
        let dt = t.elapsed().as_secs_f32();
        let dx = self.current.x - start.x;
        let dy = self.current.y - start.y;
        let dist = ((dx*dx + dy*dy) as f32).sqrt();
        if dist < 30.0 && dt < 0.3 {
            return Some(Gesture::Tap { pos: start, count: 1 });
        }
        if dist < 10.0 && dt > 0.5 {
            return Some(Gesture::LongPress { pos: start });
        }
        if dist >= 30.0 {
            let dir = if dx.abs() > dy.abs() {
                if dx > 0 { SwipeDir::Right } else { SwipeDir::Left }
            } else {
                if dy > 0 { SwipeDir::Down } else { SwipeDir::Up }
            };
            let vel = if dt > 0.0 { dist / dt } else { 0.0 };
            return Some(Gesture::Swipe { direction: dir, distance: dist, velocity: vel });
        }
        None
    }
}

// ── Input Manager ─────────────────────────────────────────────────────────────
pub struct InputManager {
    pub cursor:       Point,
    pub cursor_size:  u32,
    pressed_keys:     HashSet<Key>,
    pressed_btns:     HashSet<MouseButton>,
    pub modifiers:    Modifiers,
    shortcuts:        Vec<Shortcut>,
    event_queue:      VecDeque<InputEvent>,
    last_click:       Option<(Point, MouseButton, Instant)>,
    swipe:            SwipeTracker,
    pub scroll_speed: i32,
}

impl InputManager {
    pub fn new(screen_w: u32, screen_h: u32) -> Self {
        Self {
            cursor: Point { x: screen_w as i32 / 2, y: screen_h as i32 / 2 },
            cursor_size: 16,
            pressed_keys: HashSet::new(),
            pressed_btns: HashSet::new(),
            modifiers: Modifiers::default(),
            shortcuts: Vec::new(),
            event_queue: VecDeque::with_capacity(256),
            last_click: None,
            swipe: SwipeTracker::new(),
            scroll_speed: 3,
        }
    }

    pub fn register_shortcut(&mut self, key: Key, mods: Modifiers, action: &str) {
        self.shortcuts.push(Shortcut { key, mods, action: action.to_owned() });
    }

    fn update_modifiers(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Ctrl  => self.modifiers.ctrl      = pressed,
            Key::Alt   => self.modifiers.alt        = pressed,
            Key::Shift => self.modifiers.shift      = pressed,
            Key::Super => self.modifiers.super_key  = pressed,
            _ => {}
        }
    }

    pub fn key_event(&mut self, key: Key, pressed: bool) {
        self.update_modifiers(key, pressed);
        if pressed { self.pressed_keys.insert(key); } else { self.pressed_keys.remove(&key); }
        // Check shortcuts
        if pressed {
            for sc in &self.shortcuts {
                if sc.key == key && sc.mods == self.modifiers {
                    // Action would be dispatched here
                    let _ = &sc.action;
                }
            }
        }
        self.event_queue.push_back(InputEvent::Key(KeyEvent {
            key, pressed, modifiers: self.modifiers
        }));
    }

    pub fn mouse_move(&mut self, dx: i32, dy: i32, screen_w: u32, screen_h: u32) {
        let old = self.cursor;
        self.cursor.x = (self.cursor.x + dx).clamp(0, screen_w as i32 - 1);
        self.cursor.y = (self.cursor.y + dy).clamp(0, screen_h as i32 - 1);
        self.event_queue.push_back(InputEvent::Mouse(MouseEvent::Move {
            pos: self.cursor, delta: (self.cursor.x-old.x, self.cursor.y-old.y),
        }));
    }

    pub fn mouse_button(&mut self, btn: MouseButton, pressed: bool) {
        if pressed {
            self.pressed_btns.insert(btn);
            // Double-click detection (< 300ms)
            let dbl = if let Some((pos, b, t)) = &self.last_click {
                *b == btn && t.elapsed() < Duration::from_millis(300)
                    && (self.cursor.x - pos.x).abs() < 5
                    && (self.cursor.y - pos.y).abs() < 5
            } else { false };
            if dbl {
                self.event_queue.push_back(InputEvent::Mouse(MouseEvent::DblClick { pos: self.cursor, btn }));
                self.last_click = None;
            } else {
                self.event_queue.push_back(InputEvent::Mouse(MouseEvent::Press { pos: self.cursor, btn }));
                self.last_click = Some((self.cursor, btn, Instant::now()));
            }
        } else {
            self.pressed_btns.remove(&btn);
            self.event_queue.push_back(InputEvent::Mouse(MouseEvent::Release { pos: self.cursor, btn }));
        }
    }

    pub fn scroll(&mut self, dx: i32, dy: i32) {
        let s = self.scroll_speed;
        self.event_queue.push_back(InputEvent::Mouse(MouseEvent::Scroll {
            pos: self.cursor, dx: dx*s, dy: dy*s,
        }));
    }

    pub fn touch_down(&mut self, id: u32, x: i32, y: i32) {
        let p = Point{x,y}; self.swipe.touch_down(p);
        self.event_queue.push_back(InputEvent::Touch(TouchEvent::Down(
            Touch{id, pos: p, pressure: 1.0}
        )));
    }

    pub fn touch_move(&mut self, id: u32, x: i32, y: i32) {
        let p = Point{x,y}; self.swipe.touch_move(p);
        self.event_queue.push_back(InputEvent::Touch(TouchEvent::Move(
            Touch{id, pos: p, pressure: 1.0}
        )));
    }

    pub fn touch_up(&mut self, id: u32, x: i32, y: i32) {
        let p = Point{x,y};
        self.event_queue.push_back(InputEvent::Touch(TouchEvent::Up(
            Touch{id, pos: p, pressure: 0.0}
        )));
        if let Some(g) = self.swipe.touch_up() {
            self.event_queue.push_back(InputEvent::Gesture(g));
        }
    }

    pub fn poll(&mut self) -> Option<InputEvent> { self.event_queue.pop_front() }
    pub fn is_key_down(&self, k: Key) -> bool { self.pressed_keys.contains(&k) }
    pub fn is_btn_down(&self, b: MouseButton) -> bool { self.pressed_btns.contains(&b) }
    pub fn queue_len(&self) -> usize { self.event_queue.len() }
}
