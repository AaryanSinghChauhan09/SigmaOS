#![no_std]
#![no_main]

/// OOP-based Input Handler for SigmaOS
/// Implements input handling using OOP principles with traits and structs
/// No dependency on external input frameworks

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Input device type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum InputDeviceType {
    Keyboard = 0,
    Mouse = 1,
    Touchscreen = 2,
    Gamepad = 3,
    Custom = 4,
}

/// Input event trait (OOP interface)
pub trait InputEvent {
    /// Get event type
    fn event_type(&self) -> InputEventType;
    /// Get timestamp
    fn timestamp(&self) -> u64;
    /// Get device ID
    fn device_id(&self) -> usize;
}

/// Input event type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum InputEventType {
    KeyPress = 0,
    KeyRelease = 1,
    MouseMove = 2,
    MousePress = 3,
    MouseRelease = 4,
    MouseScroll = 5,
    TouchStart = 6,
    TouchMove = 7,
    TouchEnd = 8,
    Custom = 9,
}

/// Keyboard event (OOP: Concrete event class)
#[repr(C)]
pub struct KeyEvent {
    pub timestamp: u64,
    pub device_id: usize,
    pub keycode: u32,
    pub modifiers: KeyModifiers,
    pub pressed: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

impl KeyEvent {
    pub fn new(device_id: usize, keycode: u32, pressed: bool) -> Self {
        KeyEvent {
            timestamp: get_current_time(),
            device_id,
            keycode,
            modifiers: KeyModifiers {
                shift: false,
                ctrl: false,
                alt: false,
                super_key: false,
            },
            pressed,
        }
    }
}

impl InputEvent for KeyEvent {
    fn event_type(&self) -> InputEventType {
        if self.pressed {
            InputEventType::KeyPress
        } else {
            InputEventType::KeyRelease
        }
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn device_id(&self) -> usize {
        self.device_id
    }
}

/// Mouse event (OOP: Concrete event class)
#[repr(C)]
pub struct MouseEvent {
    pub timestamp: u64,
    pub device_id: usize,
    pub x: i32,
    pub y: i32,
    pub buttons: MouseButtons,
    pub event_type: InputEventType,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MouseButtons {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

impl MouseEvent {
    pub fn new(device_id: usize, x: i32, y: i32, event_type: InputEventType) -> Self {
        MouseEvent {
            timestamp: get_current_time(),
            device_id,
            x,
            y,
            buttons: MouseButtons {
                left: false,
                right: false,
                middle: false,
            },
            event_type,
        }
    }
}

impl InputEvent for MouseEvent {
    fn event_type(&self) -> InputEventType {
        self.event_type
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn device_id(&self) -> usize {
        self.device_id
    }
}

/// Input handler trait (OOP interface)
pub trait InputHandler {
    /// Handle input event
    fn handle_event(&mut self, event: &dyn InputEvent) -> InputResult;
    /// Register callback
    fn register_callback(&mut self, callback: fn(&dyn InputEvent));
    /// Get handler info
    fn info(&self) -> InputHandlerInfo;
}

/// Input result
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum InputResult {
    Handled = 0,
    Ignored = 1,
    Propagated = 2,
}

/// Input handler info
#[repr(C)]
pub struct InputHandlerInfo {
    pub handler_type: HandlerType,
    pub priority: Priority,
    pub capability: HandlerCapability,
}

impl InputHandlerInfo {
    pub fn new(handler_type: HandlerType) -> Self {
        InputHandlerInfo {
            handler_type,
            priority: Priority::Normal,
            capability: HandlerCapability::new(),
        }
    }
}

/// Handler type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HandlerType {
    Keyboard = 0,
    Mouse = 1,
    Touch = 2,
    Global = 3,
}

/// Priority level
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Handler capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HandlerCapability {
    pub can_handle: bool,
    pub can_block: bool,
    pub can_modify: bool,
}

impl HandlerCapability {
    pub fn new() -> Self {
        HandlerCapability {
            can_handle: false,
            can_block: false,
            can_modify: false,
        }
    }

    pub fn full() -> Self {
        HandlerCapability {
            can_handle: true,
            can_block: true,
            can_modify: true,
        }
    }
}

/// Simple input handler (OOP: Concrete handler class)
pub struct SimpleInputHandler {
    pub handler_type: HandlerType,
    pub priority: Priority,
    pub capability: HandlerCapability,
    pub callback: Option<fn(&dyn InputEvent)>,
    pub event_count: AtomicUsize,
}

impl SimpleInputHandler {
    pub fn new(handler_type: HandlerType, priority: Priority, capability: HandlerCapability) -> Self {
        SimpleInputHandler {
            handler_type,
            priority,
            capability,
            callback: None,
            event_count: AtomicUsize::new(0),
        }
    }
}

impl InputHandler for SimpleInputHandler {
    fn handle_event(&mut self, event: &dyn InputEvent) -> InputResult {
        if !self.capability.can handle {
            return InputResult::Ignored;
        }

        self.event_count.fetch_add(1, Ordering::SeqCst);

        if let Some(callback) = self.callback {
            callback(event);
        }

        InputResult::Handled
    }

    fn register_callback(&mut self, callback: fn(&dyn InputEvent)) {
        self.callback = Some(callback);
    }

    fn info(&self) -> InputHandlerInfo {
        InputHandlerInfo {
            handler_type: self.handler_type,
            priority: self.priority,
            capability: self.capability,
        }
    }
}

/// Input manager trait (OOP interface)
pub trait InputManager {
    /// Register handler
    fn register_handler(&mut self, handler: Box<dyn InputHandler>) -> Result<usize, InputError>;
    /// Unregister handler
    fn unregister_handler(&mut self, id: usize) -> Result<(), InputError>;
    /// Dispatch event
    fn dispatch_event(&mut self, event: &dyn InputEvent) -> InputResult;
    /// Add device
    fn add_device(&mut self, device: InputDevice) -> Result<usize, InputError>;
    /// Remove device
    fn remove_device(&mut self, id: usize) -> Result<(), InputError>;
    /// Get manager statistics
    fn stats(&self) -> InputStats;
}

/// Input error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum InputError {
    Success = 0,
    HandlerNotFound = 1,
    DeviceNotFound = 2,
    PermissionDenied = 3,
    InvalidEvent = 4,
}

/// Input device (OOP: Device object)
#[repr(C)]
pub struct InputDevice {
    pub id: usize,
    pub device_type: InputDeviceType,
    pub name: [u8; 64],
    pub enabled: AtomicBool,
    pub capability: DeviceCapability,
}

/// Device capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceCapability {
    pub can_enable: bool,
    pub can_disable: bool,
    pub can_configure: bool,
}

impl DeviceCapability {
    pub fn new() -> Self {
        DeviceCapability {
            can_enable: false,
            can_disable: false,
            can_configure: false,
        }
    }

    pub fn full() -> Self {
        DeviceCapability {
            can_enable: true,
            can_disable: true,
            can_configure: true,
        }
    }
}

impl InputDevice {
    pub fn new(id: usize, device_type: InputDeviceType, name: &[u8], capability: DeviceCapability) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }

        InputDevice {
            id,
            device_type,
            name: name_array,
            enabled: AtomicBool::new(true),
            capability,
        }
    }

    pub fn enable(&self) -> Result<(), InputError> {
        if !self.capability.can_enable {
            return Err(InputError::PermissionDenied);
        }
        self.enabled.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn disable(&self) -> Result<(), InputError> {
        if !self.capability.can_disable {
            return Err(InputError::PermissionDenied);
        }
        self.enabled.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }
}

/// Input statistics
#[repr(C)]
pub struct InputStats {
    pub total_events: u64,
    pub handled_events: u64,
    pub ignored_events: u64,
    pub active_handlers: usize,
    pub active_devices: usize,
}

impl InputStats {
    pub fn new() -> Self {
        InputStats {
            total_events: 0,
            handled_events: 0,
            ignored_events: 0,
            active_handlers: 0,
            active_devices: 0,
        }
    }
}

/// Simple input manager (OOP: Concrete manager class)
pub struct SimpleInputManager {
    handlers: Vec<Option<Box<dyn InputHandler>>>,
    devices: Vec<Option<NonNull<InputDevice>>>,
    stats: InputStats,
    capability: ManagerCapability,
}

/// Manager capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ManagerCapability {
    pub can_register_handlers: bool,
    pub can_unregister_handlers: bool,
    pub can_add_devices: bool,
    pub can_remove_devices: bool,
}

impl ManagerCapability {
    pub fn new() -> Self {
        ManagerCapability {
            can_register_handlers: false,
            can_unregister_handlers: false,
            can_add_devices: false,
            can_remove_devices: false,
        }
    }

    pub fn full() -> Self {
        ManagerCapability {
            can_register_handlers: true,
            can_unregister_handlers: true,
            can_add_devices: true,
            can_remove_devices: true,
        }
    }
}

impl SimpleInputManager {
    pub fn new(capability: ManagerCapability) -> Self {
        SimpleInputManager {
            handlers: Vec::new(),
            devices: Vec::new(),
            stats: InputStats::new(),
            capability,
        }
    }
}

impl InputManager for SimpleInputManager {
    fn register_handler(&mut self, handler: Box<dyn InputHandler>) -> Result<usize, InputError> {
        if !self.capability.can_register_handlers {
            return Err(InputError::PermissionDenied);
        }

        let id = self.handlers.len();
        self.handlers.push(Some(handler));
        Ok(id)
    }

    fn unregister_handler(&mut self, id: usize) -> Result<(), InputError> {
        if !self.capability.can_unregister_handlers {
            return Err(InputError::PermissionDenied);
        }

        if id < self.handlers.len() {
            self.handlers[id] = None;
            Ok(())
        } else {
            Err(InputError::HandlerNotFound)
        }
    }

    fn dispatch_event(&mut self, event: &dyn InputEvent) -> InputResult {
        self.stats.total_events += 1;

        // Check if device is enabled
        let device_enabled = unsafe {
            let mut enabled = true;
            for device_option in &self.devices {
                if let Some(device_ptr) = *device_option {
                    let device = &*device_ptr.as_ptr();
                    if device.id == event.device_id() {
                        enabled = device.is_enabled();
                        break;
                    }
                }
            }
            enabled
        };

        if !device_enabled {
            self.stats.ignored_events += 1;
            return InputResult::Ignored;
        }

        // Dispatch to handlers
        for handler_option in &mut self.handlers {
            if let Some(ref mut handler) = *handler_option {
                let result = handler.handle_event(event);
                if result == InputResult::Handled {
                    self.stats.handled_events += 1;
                    return result;
                }
            }
        }

        self.stats.ignored_events += 1;
        InputResult::Ignored
    }

    fn add_device(&mut self, device: InputDevice) -> Result<usize, InputError> {
        if !self.capability.can_add_devices {
            return Err(InputError::PermissionDenied);
        }

        let id = device.id;
        let device_ptr = unsafe {
            let ptr = alloc(mem::size_of::<InputDevice>()) as *mut InputDevice;
            if ptr.is_null() {
                return Err(InputError::DeviceNotFound);
            }
            core::ptr::write(ptr, device);
            NonNull::new_unchecked(ptr)
        };

        self.devices.push(Some(device_ptr));
        Ok(id)
    }

    fn remove_device(&mut self, id: usize) -> Result<(), InputError> {
        if !self.capability.can_remove_devices {
            return Err(InputError::PermissionDenied);
        }

        unsafe {
            let mut index = None;
            for (i, device_option) in self.devices.iter().enumerate() {
                if let Some(device_ptr) = *device_option {
                    let device = &*device_ptr.as_ptr();
                    if device.id == id {
                        index = Some(i);
                        break;
                    }
                }
            }

            if let Some(i) = index {
                if let Some(device_ptr) = self.devices[i] {
                    core::ptr::drop_in_place(device_ptr.as_ptr());
                    free(device_ptr.as_ptr() as *mut u8);
                }
                self.devices[i] = None;
                Ok(())
            } else {
                Err(InputError::DeviceNotFound)
            }
        }
    }

    fn stats(&self) -> InputStats {
        let mut stats = self.stats.clone();
        stats.active_handlers = self.handlers.iter().filter(|x| x.is_some()).count();
        stats.active_devices = self.devices.iter().filter(|x| x.is_some()).count();
        stats
    }
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static mut COUNTER: u64 = 0;
    unsafe {
        COUNTER += 1_000_000;
        COUNTER
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
