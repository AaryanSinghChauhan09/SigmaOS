// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// OOP-based driver framework traits
// Zero-allocation, performance-optimized driver interfaces

pub struct Box<T: ?Sized> {
    _ptr: *mut T,
}

/// Core driver trait - all drivers must implement this
pub trait Driver {
    /// Initialize the driver
    fn init(&mut self) -> Result<(), DriverError>;
    
    /// Get driver name
    fn name(&self) -> &str;
    
    /// Get driver version
    fn version(&self) -> (u8, u8, u8);
    
    /// Check if driver is ready
    fn is_ready(&self) -> bool;
    
    /// Reset driver to initial state
    fn reset(&mut self) -> Result<(), DriverError>;
}

/// Device trait for hardware devices
pub trait Device: Driver {
    /// Get device ID
    fn device_id(&self) -> u32;
    
    /// Get device class
    fn device_class(&self) -> DeviceClass;
    
    /// Power management
    fn set_power_state(&mut self, state: PowerState) -> Result<(), DriverError>;
    
    /// Get current power state
    fn power_state(&self) -> PowerState;
}

/// Storage device trait
pub trait StorageDevice: Device {
    /// Read blocks from device
    fn read_blocks(&mut self, lba: u64, blocks: u16, buffer: &mut [u8]) -> Result<(), DriverError>;
    
    /// Write blocks to device
    fn write_blocks(&mut self, lba: u64, blocks: u16, buffer: &[u8]) -> Result<(), DriverError>;
    
    /// Get block size
    fn block_size(&self) -> u32;
    
    /// Get total blocks
    fn total_blocks(&self) -> u64;
    
    /// Flush cache
    fn flush(&mut self) -> Result<(), DriverError>;
}

/// Network device trait
pub trait NetworkDevice: Device {
    /// Send packet
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), DriverError>;
    
    /// Receive packet
    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, DriverError>;
    
    /// Get MAC address
    fn mac_address(&self) -> [u8; 6];
    
    /// Get MTU
    fn mtu(&self) -> u16;
    
    /// Set promiscuous mode
    fn set_promiscuous(&mut self, enabled: bool) -> Result<(), DriverError>;
}

/// Display device trait
pub trait DisplayDevice: Device {
    /// Set video mode
    fn set_mode(&mut self, width: u32, height: u32, bpp: u8) -> Result<(), DriverError>;
    
    /// Get current mode
    fn get_mode(&self) -> (u32, u32, u8);
    
    /// Write pixel
    fn write_pixel(&mut self, x: u32, y: u32, color: u32) -> Result<(), DriverError>;
    
    /// Fill rectangle
    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) -> Result<(), DriverError>;
    
    /// Copy buffer to framebuffer
    fn blit(&mut self, buffer: &[u8], x: u32, y: u32, width: u32, height: u32) -> Result<(), DriverError>;
}

/// Input device trait
pub trait InputDevice: Device {
    /// Read input event
    fn read_event(&mut self) -> Option<InputEvent>;
    
    /// Get device type
    fn input_type(&self) -> InputType;
}

/// Error types for drivers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    InitializationFailed,
    Timeout,
    InvalidParameter,
    HardwareError,
    Busy,
    NotReady,
    UnsupportedOperation,
    BufferTooSmall,
    InvalidState,
}

/// Device classes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Storage,
    Network,
    Display,
    Input,
    Audio,
    Other,
}

/// Power states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    D0,  // Fully on
    D1,  // Low power
    D2,  // Lower power
    D3,  // Sleep
    D4,  // Hibernate
}

/// Input event types
#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Keyboard { keycode: u8, pressed: bool },
    Mouse { x: i32, y: i32, buttons: u8 },
    Touch { x: i32, y: i32, pressed: bool },
}

/// Input device types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Keyboard,
    Mouse,
    Touchpad,
    Touchscreen,
    Other,
}

/// Driver registry for managing multiple drivers
pub trait DriverRegistry {
    /// Register a driver
    fn register(&mut self, driver: Box<dyn Driver>) -> Result<(), DriverError>;
    
    /// Unregister a driver by name
    fn unregister(&mut self, name: &str) -> Result<(), DriverError>;
    
    /// Get driver by name
    fn get_driver(&self, name: &str) -> Option<&dyn Driver>;
    
    /// Get mutable driver by name
    fn get_driver_mut(&mut self, name: &str) -> Option<&mut dyn Driver>;
    
    /// List all registered drivers
    fn list_drivers(&self) -> &[&str];
    
    /// Initialize all drivers
    fn init_all(&mut self) -> Result<(), DriverError>;
}
