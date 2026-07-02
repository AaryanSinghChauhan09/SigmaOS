// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_notifications.rs — Notification System
// Language: Rust (std) — OOP via NotificationManager + Notification trait

use std::collections::VecDeque;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

// ── Notification Urgency ──────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Urgency { Low, Normal, Critical }

// ── Action ───────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Action { pub id: String, pub label: String }

// ── Notification ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Notification {
    pub id:        u32,
    pub app_name:  String,
    pub summary:   String,
    pub body:      String,
    pub icon:      Option<String>,
    pub urgency:   Urgency,
    pub timeout_ms: Option<u32>,  // None = persistent
    pub actions:   Vec<Action>,
    pub created_at: u64,          // unix ms
    pub dismissed: bool,
    pub replaces:  Option<u32>,   // ID of notification to replace
}

impl Notification {
    pub fn new(app: &str, summary: &str, body: &str) -> Self {
        Self {
            id: 0, // assigned by manager
            app_name:  app.to_owned(),
            summary:   summary.to_owned(),
            body:      body.to_owned(),
            icon:      None,
            urgency:   Urgency::Normal,
            timeout_ms: Some(5000),
            actions:   Vec::new(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64,
            dismissed: false,
            replaces:  None,
        }
    }

    pub fn with_urgency(mut self, u: Urgency) -> Self { self.urgency = u; self }
    pub fn with_timeout(mut self, ms: u32) -> Self { self.timeout_ms = Some(ms); self }
    pub fn persistent(mut self) -> Self { self.timeout_ms = None; self }
    pub fn with_icon(mut self, icon: &str) -> Self { self.icon = Some(icon.to_owned()); self }
    pub fn with_action(mut self, id: &str, label: &str) -> Self {
        self.actions.push(Action { id: id.to_owned(), label: label.to_owned() }); self
    }
    pub fn replacing(mut self, id: u32) -> Self { self.replaces = Some(id); self }

    pub fn is_expired(&self, now_ms: u64) -> bool {
        if let Some(timeout) = self.timeout_ms {
            now_ms > self.created_at + timeout as u64
        } else { false }
    }
}

// ── Do-Not-Disturb ───────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct DndSchedule {
    pub enabled:    bool,
    pub start_hour: u8, // 0-23
    pub end_hour:   u8,
}

impl DndSchedule {
    pub fn is_active(&self, hour: u8) -> bool {
        if !self.enabled { return false; }
        if self.start_hour <= self.end_hour {
            hour >= self.start_hour && hour < self.end_hour
        } else {
            hour >= self.start_hour || hour < self.end_hour
        }
    }
}

// ── Event ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum NotifEvent {
    Added(u32),
    Dismissed(u32),
    ActionInvoked { notif_id: u32, action_id: String },
    Closed(u32),
}

// ── Notification Manager ──────────────────────────────────────────────────────

pub const MAX_HISTORY:  usize = 64;
pub const MAX_ACTIVE:   usize = 8;  // max notifications shown at once

pub struct NotificationManager {
    active:     Vec<Notification>,
    history:    VecDeque<Notification>,
    next_id:    u32,
    dnd:        DndSchedule,
    dnd_manual: bool,
    events:     VecDeque<NotifEvent>,
    max_active: usize,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            active: Vec::new(), history: VecDeque::new(),
            next_id: 1, dnd: DndSchedule::default(), dnd_manual: false,
            events: VecDeque::new(), max_active: MAX_ACTIVE,
        }
    }

    pub fn notify(&mut self, mut n: Notification) -> u32 {
        // Handle replacement
        if let Some(rep_id) = n.replaces {
            self.active.retain(|a| a.id != rep_id);
        }
        n.id = self.next_id; self.next_id += 1;

        // Check DND for non-critical
        if n.urgency != Urgency::Critical && (self.dnd_manual || self.is_dnd_active()) {
            self.history.push_front(n.clone());
            if self.history.len() > MAX_HISTORY { self.history.pop_back(); }
            return n.id;
        }

        let id = n.id;
        // Evict oldest non-critical if at max
        if self.active.len() >= self.max_active {
            let oldest_idx = self.active.iter().position(|a| a.urgency != Urgency::Critical);
            if let Some(i) = oldest_idx {
                let old = self.active.remove(i);
                self.history.push_front(old);
                if self.history.len() > MAX_HISTORY { self.history.pop_back(); }
            }
        }
        self.active.push(n);
        self.events.push_back(NotifEvent::Added(id));
        id
    }

    pub fn dismiss(&mut self, id: u32) {
        if let Some(pos) = self.active.iter().position(|n| n.id == id) {
            let mut n = self.active.remove(pos);
            n.dismissed = true;
            self.history.push_front(n);
            if self.history.len() > MAX_HISTORY { self.history.pop_back(); }
            self.events.push_back(NotifEvent::Dismissed(id));
        }
    }

    pub fn dismiss_all(&mut self) {
        let ids: Vec<u32> = self.active.iter().map(|n| n.id).collect();
        for id in ids { self.dismiss(id); }
    }

    pub fn invoke_action(&mut self, notif_id: u32, action_id: &str) {
        self.events.push_back(NotifEvent::ActionInvoked {
            notif_id, action_id: action_id.to_owned(),
        });
        // Auto-dismiss after action (unless persistent)
        if let Some(n) = self.active.iter().find(|n| n.id == notif_id) {
            if n.timeout_ms.is_some() { self.dismiss(notif_id); }
        }
    }

    /// Expire timed-out notifications, call on each frame/tick
    pub fn tick(&mut self, now_ms: u64) {
        let expired: Vec<u32> = self.active.iter()
            .filter(|n| n.is_expired(now_ms))
            .map(|n| n.id).collect();
        for id in expired { self.dismiss(id); }
    }

    pub fn set_dnd(&mut self, enabled: bool) { self.dnd_manual = enabled; }
    pub fn toggle_dnd(&mut self) { self.dnd_manual = !self.dnd_manual; }
    pub fn set_dnd_schedule(&mut self, start: u8, end: u8) {
        self.dnd = DndSchedule { enabled: true, start_hour: start, end_hour: end };
    }
    fn is_dnd_active(&self) -> bool {
        let hour = (SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_default().as_secs() / 3600 % 24) as u8;
        self.dnd.is_active(hour)
    }

    pub fn active(&self)  -> &[Notification] { &self.active }
    pub fn history(&self) -> impl Iterator<Item=&Notification> { self.history.iter() }
    pub fn next_event(&mut self) -> Option<NotifEvent> { self.events.pop_front() }
    pub fn active_count(&self) -> usize { self.active.len() }
    pub fn is_dnd(&self) -> bool { self.dnd_manual || self.is_dnd_active() }
}
