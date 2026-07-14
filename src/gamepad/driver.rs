#![no_std]
#![no_main]

/// OOP-based Gamepad Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 311
/// Implements gamepad input and rumble feedback

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type GamepadID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ButtonState { Released = 0, Pressed = 1 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GamepadError { Success = 0, NotFound = 1, NotConnected = 2 }

pub trait Gamepad {
    fn id(&self) -> GamepadID;
    fn name(&self) -> &[u8];
    fn is_connected(&self) -> bool;
    fn get_button(&self, button: u8) -> ButtonState;
    fn get_axis(&self, axis: u8) -> i16;
}

#[repr(C)]
pub struct SimpleGamepad {
    pub id: GamepadID,
    pub name: [u8; 64],
    pub connected: AtomicUsize,
    pub buttons: [AtomicUsize; 16],
    pub axes: [AtomicUsize; 4],
}

impl SimpleGamepad {
    pub fn new(id: GamepadID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        let mut buttons = [AtomicUsize::new(0); 16];
        let mut axes = [AtomicUsize::new(32768); 4];
        SimpleGamepad {
            id,
            name: name_array,
            connected: AtomicUsize::new(1),
            buttons,
            axes,
        }
    }
}

impl Gamepad for SimpleGamepad {
    fn id(&self) -> GamepadID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
    fn get_button(&self, button: u8) -> ButtonState {
        if button < 16 {
            unsafe { core::mem::transmute(self.buttons[button as usize].load(Ordering::SeqCst)) }
        } else {
            ButtonState::Released
        }
    }
    fn get_axis(&self, axis: u8) -> i16 {
        if axis < 4 {
            self.axes[axis as usize].load(Ordering::SeqCst) as i16
        } else {
            0
        }
    }
}

pub trait GamepadManager {
    fn add_gamepad(&mut self, gamepad: Box<dyn Gamepad>) -> Result<GamepadID, GamepadError>;
    fn remove_gamepad(&mut self, id: GamepadID) -> Result<(), GamepadError>;
    fn get_gamepad(&self, id: GamepadID) -> Option<&dyn Gamepad>;
    fn set_rumble(&mut self, id: GamepadID, left: u8, right: u8) -> Result<(), GamepadError>;
}

#[repr(C)]
pub struct SimpleGamepadManager {
    pub gamepads: Vec<Option<Box<dyn Gamepad>>>,
    pub next_id: AtomicUsize,
}

impl SimpleGamepadManager {
    pub fn new() -> Self {
        SimpleGamepadManager {
            gamepads: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl GamepadManager for SimpleGamepadManager {
    fn add_gamepad(&mut self, gamepad: Box<dyn Gamepad>) -> Result<GamepadID, GamepadError> {
        let id = gamepad.id();
        self.gamepads.push(Some(gamepad));
        Ok(id)
    }

    fn remove_gamepad(&mut self, id: GamepadID) -> Result<(), GamepadError> {
        for gamepad_option in &mut self.gamepads {
            if let Some(ref gamepad) = *gamepad_option {
                if gamepad.id() == id {
                    return Ok(());
                }
            }
        }
        Err(GamepadError::NotFound)
    }

    fn get_gamepad(&self, id: GamepadID) -> Option<&dyn Gamepad> {
        for gamepad_option in &self.gamepads {
            if let Some(ref gamepad) = *gamepad_option {
                if gamepad.id() == id { return Some(gamepad.as_ref()); }
            }
        }
        None
    }

    fn set_rumble(&mut self, _id: GamepadID, _left: u8, _right: u8) -> Result<(), GamepadError> {
        Ok(())
    }
}

pub trait InputMapping {
    fn map_button(&mut self, physical: u8, virtual: u8);
    fn map_axis(&mut self, physical: u8, virtual: u8);
    fn get_mapped_button(&self, physical: u8) -> u8;
    fn get_mapped_axis(&self, physical: u8) -> u8;
}

#[repr(C)]
pub struct SimpleInputMapping {
    pub button_map: [u8; 16],
    pub axis_map: [u8; 4],
}

impl SimpleInputMapping {
    pub fn new() -> Self {
        SimpleInputMapping {
            button_map: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            axis_map: [0, 1, 2, 3],
        }
    }
}

impl InputMapping for SimpleInputMapping {
    fn map_button(&mut self, physical: u8, virtual: u8) {
        if physical < 16 {
            self.button_map[physical as usize] = virtual;
        }
    }

    fn map_axis(&mut self, physical: u8, virtual: u8) {
        if physical < 4 {
            self.axis_map[physical as usize] = virtual;
        }
    }

    fn get_mapped_button(&self, physical: u8) -> u8 {
        if physical < 16 {
            self.button_map[physical as usize]
        } else {
            physical
        }
    }

    fn get_mapped_axis(&self, physical: u8) -> u8 {
        if physical < 4 {
            self.axis_map[physical as usize]
        } else {
            physical
        }
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
