//! Input Module
//!
//! Input device management for SigmaOS.

pub mod input_device_manager;

pub use input_device_manager::{
    InputDevice, InputDeviceManager, InputDeviceStatistics, InputDeviceType,
    KeyboardConfig, MouseConfig, TouchpadConfig,
};
