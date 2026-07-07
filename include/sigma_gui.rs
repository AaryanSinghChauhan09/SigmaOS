// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// include/sigma_gui.rs — GUI Framework Header
//
// Defines GUI structures and function signatures for the SigmaOS
// desktop environment including window management, drawing primitives,
// and event handling.
//
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum screen width.
const MAX_WIDTH: SigmaUsize = 3840;
/// Maximum screen height.
const MAX_HEIGHT: SigmaUsize = 2160;
/// Maximum number of windows.
const MAX_WINDOWS: SigmaUsize = 64;
/// Window title length.
const WINDOW_TITLE_LEN: SigmaUsize = 64;

// ── Color ───────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: SigmaU8,
    pub g: SigmaU8,
    pub b: SigmaU8,
    pub a: SigmaU8,
}

// ── Rect ───────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rect {
    pub x: SigmaI32,
    pub y: SigmaI32,
    pub width: SigmaU32,
    pub height: SigmaU32,
}

// ── Window ─────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Window {
    pub window_id: SigmaU32,
    pub title: [SigmaU8; WINDOW_TITLE_LEN],
    pub rect: Rect,
    pub visible: SigmaBool,
    pub focused: SigmaBool,
    pub _pad: [SigmaU8; 7],
}

// ── Event Types ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EventType {
    KeyPress = 0,
    KeyRelease = 1,
    MouseMove = 2,
    MousePress = 3,
    MouseRelease = 4,
    Resize = 5,
    Close = 6,
}

// ── Event ───────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub window_id: SigmaU32,
    pub x: SigmaI32,
    pub y: SigmaI32,
    pub key_code: SigmaU32,
    pub _pad: [SigmaU8; 4],
}

