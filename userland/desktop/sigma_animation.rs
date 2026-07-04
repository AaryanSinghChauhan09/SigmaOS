// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_animation.rs — Animation Engine (spring physics + easing)
// Language: Rust (std) — OOP via AnimationEngine + Animator trait

use std::time::Instant;
use std::collections::BTreeMap;

// ── Easing Functions ──────────────────────────────────────────────────────────
pub fn ease_linear(t: f32)          -> f32 { t.clamp(0.0, 1.0) }
pub fn ease_in_quad(t: f32)         -> f32 { let t = t.clamp(0.0,1.0); t * t }
pub fn ease_out_quad(t: f32)        -> f32 { let t = t.clamp(0.0,1.0); t * (2.0 - t) }
pub fn ease_in_out_quad(t: f32)     -> f32 {
    let t = t.clamp(0.0,1.0);
    if t < 0.5 { 2.0*t*t } else { -1.0+(4.0-2.0*t)*t }
}
pub fn ease_out_cubic(t: f32)       -> f32 { let t = t.clamp(0.0,1.0)-1.0; t*t*t+1.0 }
pub fn ease_in_out_sine(t: f32)     -> f32 { let t = t.clamp(0.0,1.0); -(((t*core::f32::consts::PI).cos()-1.0)/2.0) }
pub fn ease_out_elastic(t: f32)     -> f32 {
    if t <= 0.0 { return 0.0; } if t >= 1.0 { return 1.0; }
    let c4 = (2.0 * core::f32::consts::PI) / 3.0;
    (2.0f32).powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
}
pub fn ease_out_bounce(t: f32)      -> f32 {
    let t = t.clamp(0.0,1.0);
    let n1 = 7.5625f32; let d1 = 2.75f32;
    if      t < 1.0/d1  { n1*t*t }
    else if t < 2.0/d1  { let t=t-1.5/d1;  n1*t*t+0.75 }
    else if t < 2.5/d1  { let t=t-2.25/d1; n1*t*t+0.9375 }
    else                 { let t=t-2.625/d1; n1*t*t+0.984375 }
}

// ── Spring ────────────────────────────────────────────────────────────────────
pub struct Spring {
    pub stiffness:   f32,   // k — how stiff (100-400 typical)
    pub damping:     f32,   // c — how much damping (10-40)
    pub mass:        f32,
    pub current:     f32,
    pub velocity:    f32,
    pub target:      f32,
}

impl Spring {
    pub fn new(stiffness: f32, damping: f32) -> Self {
        Self { stiffness, damping, mass: 1.0, current: 0.0, velocity: 0.0, target: 0.0 }
    }
    pub fn set_target(&mut self, t: f32) { self.target = t; }
    pub fn set_current(&mut self, v: f32) { self.current = v; self.velocity = 0.0; }

    pub fn step(&mut self, dt: f32) -> f32 {
        let force = -self.stiffness * (self.current - self.target)
                    - self.damping  * self.velocity;
        self.velocity += force / self.mass * dt;
        self.current  += self.velocity * dt;
        self.current
    }

    pub fn is_settled(&self) -> bool {
        (self.current - self.target).abs() < 0.001 && self.velocity.abs() < 0.001
    }
}

// ── Animation ─────────────────────────────────────────────────────────────────
pub type EasingFn = fn(f32) -> f32;

#[derive(Clone)]
pub struct Animation {
    pub id:         String,
    pub from:       f32,
    pub to:         f32,
    pub duration:   f32,   // seconds
    pub easing:     EasingFn,
    pub delay:      f32,
    pub elapsed:    f32,
    pub value:      f32,
    pub done:       bool,
    pub loop_count: i32,   // -1 = infinite
    pub loops_done: i32,
    pub auto_reverse: bool,
    pub reversed:   bool,
}

impl Animation {
    pub fn new(id: &str, from: f32, to: f32, dur: f32, easing: EasingFn) -> Self {
        Self {
            id: id.to_owned(), from, to, duration: dur, easing,
            delay: 0.0, elapsed: 0.0, value: from,
            done: false, loop_count: 1, loops_done: 0,
            auto_reverse: false, reversed: false,
        }
    }
    pub fn with_delay(mut self, d: f32) -> Self { self.delay = d; self }
    pub fn looping(mut self, n: i32)    -> Self { self.loop_count = n; self }
    pub fn auto_reverse(mut self)        -> Self { self.auto_reverse = true; self }

    pub fn step(&mut self, dt: f32) {
        if self.done { return; }
        self.elapsed += dt;
        let active_time = self.elapsed - self.delay;
        if active_time < 0.0 { self.value = self.from; return; }
        let t = (active_time / self.duration).min(1.0);
        let eased = (self.easing)(if self.reversed { 1.0 - t } else { t });
        self.value = self.from + (self.to - self.from) * eased;

        if t >= 1.0 {
            self.loops_done += 1;
            if self.loop_count > 0 && self.loops_done >= self.loop_count {
                self.done  = true;
                self.value = if self.reversed { self.from } else { self.to };
            } else {
                self.elapsed = self.delay;
                if self.auto_reverse { self.reversed = !self.reversed; }
            }
        }
    }
}

// ── Animation Engine ──────────────────────────────────────────────────────────
pub struct AnimationEngine {
    animations:  BTreeMap<String, Animation>,
    springs:     BTreeMap<String, Spring>,
    last_tick:   Instant,
    pub reduce_motion: bool,
}

impl AnimationEngine {
    pub fn new() -> Self {
        Self {
            animations: BTreeMap::new(), springs: BTreeMap::new(),
            last_tick: Instant::now(), reduce_motion: false,
        }
    }

    pub fn play(&mut self, anim: Animation) {
        self.animations.insert(anim.id.clone(), anim);
    }

    pub fn stop(&mut self, id: &str) { self.animations.remove(id); }

    pub fn add_spring(&mut self, id: &str, stiffness: f32, damping: f32) {
        self.springs.insert(id.to_owned(), Spring::new(stiffness, damping));
    }

    pub fn spring_to(&mut self, id: &str, target: f32) {
        if let Some(s) = self.springs.get_mut(id) { s.set_target(target); }
    }

    pub fn tick(&mut self) -> f32 {
        let now = Instant::now();
        let dt  = now.duration_since(self.last_tick).as_secs_f32().min(0.05);
        self.last_tick = now;
        if self.reduce_motion { return dt; }
        for anim in self.animations.values_mut() { anim.step(dt); }
        for spring in self.springs.values_mut()  { spring.step(dt); }
        // Remove completed non-looping animations
        self.animations.retain(|_, a| !a.done);
        dt
    }

    pub fn value(&self, id: &str) -> Option<f32> {
        self.animations.get(id).map(|a| a.value)
            .or_else(|| self.springs.get(id).map(|s| s.current))
    }

    pub fn is_done(&self, id: &str) -> bool {
        self.animations.get(id).map(|a| a.done).unwrap_or(true)
    }

    pub fn spring_settled(&self, id: &str) -> bool {
        self.springs.get(id).map(|s| s.is_settled()).unwrap_or(true)
    }

    // Convenience: window open animation (scale + opacity)
    pub fn window_open(&mut self, win_id: u32) {
        if self.reduce_motion { return; }
        self.play(Animation::new(&format!("win_{}_scale",   win_id), 0.9, 1.0, 0.25, ease_out_cubic));
        self.play(Animation::new(&format!("win_{}_opacity", win_id), 0.0, 1.0, 0.20, ease_out_quad));
    }

    pub fn window_close(&mut self, win_id: u32) {
        if self.reduce_motion { return; }
        self.play(Animation::new(&format!("win_{}_scale",   win_id), 1.0, 0.9, 0.18, ease_in_quad));
        self.play(Animation::new(&format!("win_{}_opacity", win_id), 1.0, 0.0, 0.15, ease_in_quad));
    }

    pub fn workspace_switch(&mut self, direction: i32) {
        if self.reduce_motion { return; }
        let from = if direction > 0 { 0.0 } else { -100.0 };
        let to   = if direction > 0 { 100.0 } else { 0.0 };
        self.add_spring("workspace_x", 300.0, 30.0);
        self.spring_to("workspace_x", to);
    }
}
