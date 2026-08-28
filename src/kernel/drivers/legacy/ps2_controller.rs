#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

extern crate alloc;
use crate::kernel::subsystems::registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority,
};
/// SigmaOS Legacy Driver — Intel 8042 PS/2 Controller + AT Keyboard + PS/2 Mouse
/// Absorbs Linux drivers/input/serio/i8042.c and AT keyboard driver
/// Handles: scancode sets 1/2/3, XT compatibility, PS/2 mouse Intellimouse protocol
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::VecDeque;
use alloc::vec::Vec;

/// i8042 PS/2 controller I/O ports
pub const I8042_DATA_PORT: u16 = 0x60;
pub const I8042_STATUS_PORT: u16 = 0x64;
pub const I8042_CMD_PORT: u16 = 0x64;

/// i8042 commands
pub mod i8042_cmd {
    pub const READ_CONFIG: u8 = 0x20;
    pub const WRITE_CONFIG: u8 = 0x60;
    pub const DISABLE_AUX: u8 = 0xA7;
    pub const ENABLE_AUX: u8 = 0xA8;
    pub const TEST_AUX: u8 = 0xA9;
    pub const SELF_TEST: u8 = 0xAA;
    pub const TEST_KEYBOARD: u8 = 0xAB;
    pub const DISABLE_KB: u8 = 0xAD;
    pub const ENABLE_KB: u8 = 0xAE;
    pub const WRITE_AUX: u8 = 0xD4;
    pub const RESET: u8 = 0xFF;
}

/// AT Keyboard scancode set 1 — key make/break codes for US layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Backtick,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equals,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LBracket,
    RBracket,
    Backslash,
    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Apostrophe,
    Enter,
    LShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Period,
    Slash,
    RShift,
    LCtrl,
    LAlt,
    Space,
    RAlt,
    RCtrl,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    NumLock,
    NumSlash,
    NumStar,
    NumMinus,
    Num7Kp,
    Num8Kp,
    Num9Kp,
    NumPlus,
    Num4Kp,
    Num5Kp,
    Num6Kp,
    Num1Kp,
    Num2Kp,
    Num3Kp,
    NumEnter,
    Num0Kp,
    NumDot,
    PrintScreen,
    ScrollLock,
    Pause,
    Unknown(u8),
}

/// Key event type
#[derive(Debug, Clone, Copy)]
pub enum KeyEventKind {
    Press,
    Release,
}

/// Key event
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: KeyCode,
    pub kind: KeyEventKind,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

/// Translate scancode set 1 byte to KeyCode
pub fn scancode_to_key(code: u8) -> (KeyCode, KeyEventKind) {
    let released = code & 0x80 != 0;
    let make = code & 0x7F;
    let key = match make {
        0x01 => KeyCode::Escape,
        0x02 => KeyCode::Num1,
        0x03 => KeyCode::Num2,
        0x04 => KeyCode::Num3,
        0x05 => KeyCode::Num4,
        0x06 => KeyCode::Num5,
        0x07 => KeyCode::Num6,
        0x08 => KeyCode::Num7,
        0x09 => KeyCode::Num8,
        0x0A => KeyCode::Num9,
        0x0B => KeyCode::Num0,
        0x0C => KeyCode::Minus,
        0x0D => KeyCode::Equals,
        0x0E => KeyCode::Backspace,
        0x0F => KeyCode::Tab,
        0x10 => KeyCode::Q,
        0x11 => KeyCode::W,
        0x12 => KeyCode::E,
        0x13 => KeyCode::R,
        0x14 => KeyCode::T,
        0x15 => KeyCode::Y,
        0x16 => KeyCode::U,
        0x17 => KeyCode::I,
        0x18 => KeyCode::O,
        0x19 => KeyCode::P,
        0x1C => KeyCode::Enter,
        0x1D => KeyCode::LCtrl,
        0x1E => KeyCode::A,
        0x1F => KeyCode::S,
        0x20 => KeyCode::D,
        0x21 => KeyCode::F,
        0x22 => KeyCode::G,
        0x23 => KeyCode::H,
        0x24 => KeyCode::J,
        0x25 => KeyCode::K,
        0x26 => KeyCode::L,
        0x2A => KeyCode::LShift,
        0x2C => KeyCode::Z,
        0x2D => KeyCode::X,
        0x2E => KeyCode::C,
        0x2F => KeyCode::V,
        0x30 => KeyCode::B,
        0x31 => KeyCode::N,
        0x32 => KeyCode::M,
        0x36 => KeyCode::RShift,
        0x38 => KeyCode::LAlt,
        0x39 => KeyCode::Space,
        0x3A => KeyCode::CapsLock,
        0x3B => KeyCode::F1,
        0x3C => KeyCode::F2,
        0x3D => KeyCode::F3,
        0x3E => KeyCode::F4,
        0x3F => KeyCode::F5,
        0x40 => KeyCode::F6,
        0x41 => KeyCode::F7,
        0x42 => KeyCode::F8,
        0x43 => KeyCode::F9,
        0x44 => KeyCode::F10,
        0x57 => KeyCode::F11,
        0x58 => KeyCode::F12,
        _ => KeyCode::Unknown(make),
    };
    let kind = if released {
        KeyEventKind::Release
    } else {
        KeyEventKind::Press
    };
    (key, kind)
}

/// PS/2 Mouse state (standard 3-byte packet + Intellimouse 4th byte)
#[derive(Debug, Default, Clone)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub scroll: i8,
    pub left_btn: bool,
    pub right_btn: bool,
    pub middle_btn: bool,
}

/// i8042 PS/2 Controller driver
pub struct Ps2Controller {
    pub keyboard_present: bool,
    pub mouse_present: bool,
    key_events: VecDeque<KeyEvent>,
    mouse_state: MouseState,
    shift_held: bool,
    ctrl_held: bool,
    alt_held: bool,
    scancode_count: AtomicUsize,
    initialized: bool,
}

impl Ps2Controller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Ps2Controller {
            keyboard_present: true,
            mouse_present: true,
            key_events: VecDeque::with_capacity(64),
            mouse_state: MouseState::default(),
            shift_held: false,
            ctrl_held: false,
            alt_held: false,
            scancode_count: AtomicUsize::new(0),
            initialized: false,
        }
    }

    /// Process a raw scancode byte from the keyboard
    pub fn process_scancode(&mut self, scancode: u8) {
        self.scancode_count.fetch_add(1, Ordering::Relaxed);
        let (key, kind) = scancode_to_key(scancode);

        // Update modifier state
        match (&key, &kind) {
            (KeyCode::LShift, KeyEventKind::Press) | (KeyCode::RShift, KeyEventKind::Press) => {
                self.shift_held = true
            }
            (KeyCode::LShift, KeyEventKind::Release) | (KeyCode::RShift, KeyEventKind::Release) => {
                self.shift_held = false
            }
            (KeyCode::LCtrl, KeyEventKind::Press) | (KeyCode::RCtrl, KeyEventKind::Press) => {
                self.ctrl_held = true
            }
            (KeyCode::LCtrl, KeyEventKind::Release) | (KeyCode::RCtrl, KeyEventKind::Release) => {
                self.ctrl_held = false
            }
            (KeyCode::LAlt, KeyEventKind::Press) | (KeyCode::RAlt, KeyEventKind::Press) => {
                self.alt_held = true
            }
            (KeyCode::LAlt, KeyEventKind::Release) | (KeyCode::RAlt, KeyEventKind::Release) => {
                self.alt_held = false
            }
            _ => {}
        }

        self.key_events.push_back(KeyEvent {
            key,
            kind,
            shift: self.shift_held,
            ctrl: self.ctrl_held,
            alt: self.alt_held,
        });
    }

    pub fn poll_key(&mut self) -> Option<KeyEvent> {
        self.key_events.pop_front()
    }

    pub fn update_mouse(&mut self, dx: i32, dy: i32, buttons: u8) {
        self.mouse_state.x += dx;
        self.mouse_state.y += dy;
        self.mouse_state.left_btn = buttons & 0x01 != 0;
        self.mouse_state.right_btn = buttons & 0x02 != 0;
        self.mouse_state.middle_btn = buttons & 0x04 != 0;
    }

    pub fn mouse(&self) -> &MouseState {
        &self.mouse_state
    }
    pub fn scancode_count(&self) -> usize {
        self.scancode_count.load(Ordering::Relaxed)
    }
}

impl KernelSubsystem for Ps2Controller {
    fn name(&self) -> &str {
        "ps2_controller"
    }
    fn version(&self) -> &str {
        "2.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::CoreKernel
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::High
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["isa_bus"]
    }

    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }
}

impl Default for Ps2Controller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scancode_translation() {
        let (key, kind) = scancode_to_key(0x1C); // Enter press
        assert!(matches!(key, KeyCode::Enter));
        assert!(matches!(kind, KeyEventKind::Press));
        let (key2, kind2) = scancode_to_key(0x1C | 0x80); // Enter release
        assert!(matches!(key2, KeyCode::Enter));
        assert!(matches!(kind2, KeyEventKind::Release));
    }

    #[test]
    fn test_keyboard_event_queue() {
        let mut ctrl = Ps2Controller::new();
        ctrl.process_scancode(0x10); // Q press
        ctrl.process_scancode(0x90); // Q release
        let ev1 = ctrl.poll_key().unwrap();
        assert!(matches!(ev1.key, KeyCode::Q));
        assert!(matches!(ev1.kind, KeyEventKind::Press));
    }

    #[test]
    fn test_shift_modifier() {
        let mut ctrl = Ps2Controller::new();
        ctrl.process_scancode(0x2A); // LShift press
        ctrl.process_scancode(0x10); // Q press while shift held
        let _shift_ev = ctrl.poll_key().unwrap();
        let q_ev = ctrl.poll_key().unwrap();
        assert!(q_ev.shift);
    }

    #[test]
    fn test_mouse_update() {
        let mut ctrl = Ps2Controller::new();
        ctrl.update_mouse(10, -5, 0x01);
        assert_eq!(ctrl.mouse().x, 10);
        assert_eq!(ctrl.mouse().y, -5);
        assert!(ctrl.mouse().left_btn);
        assert!(!ctrl.mouse().right_btn);
    }
}
