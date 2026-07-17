#![no_std]
#![no_main]

/// OOP-based Event System for SigmaOS
/// Implements event handling using OOP principles with traits and structs
/// No dependency on external event frameworks

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Event ID
pub type EventID = usize;

/// Event trait (OOP interface)
pub trait Event {
    /// Get event ID
    fn id(&self) -> EventID;
    /// Get event type
    fn event_type(&self) -> EventType;
    /// Get timestamp
    fn timestamp(&self) -> u64;
    /// Get source
    fn source(&self) -> EventSource;
}

/// Event type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EventType {
    System = 0,
    User = 1,
    Hardware = 2,
    Network = 3,
    Custom = 4,
}

/// Event source
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EventSource {
    Kernel = 0,
    UserSpace = 1,
    Device = 2,
    Network = 3,
    Unknown = 4,
}

/// Simple event (OOP: Concrete event class)
#[repr(C)]
pub struct SimpleEvent {
    pub id: EventID,
    pub event_type: EventType,
    pub timestamp: u64,
    pub source: EventSource,
    pub data: [u8; 256],
    pub data_len: usize,
}

impl SimpleEvent {
    pub fn new(event_type: EventType, source: EventSource, data: &[u8]) -> Self {
        let mut data_array = [0u8; 256];
        let len = data.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), data_array.as_mut_ptr(), len);
        }

        SimpleEvent {
            id: generate_event_id(),
            event_type,
            timestamp: get_current_time(),
            source,
            data: data_array,
            data_len: len,
        }
    }
}

impl Event for SimpleEvent {
    fn id(&self) -> EventID {
        self.id
    }

    fn event_type(&self) -> EventType {
        self.event_type
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn source(&self) -> EventSource {
        self.source
    }
}

/// Event listener trait (OOP interface)
pub trait EventListener {
    /// Handle event
    fn handle(&mut self, event: &dyn Event) -> EventResult;
    /// Get listener info
    fn info(&self) -> ListenerInfo;
}

/// Event result
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EventResult {
    Handled = 0,
    Ignored = 1,
    Consumed = 2,
    Error = 3,
}

/// Listener info
#[repr(C)]
pub struct ListenerInfo {
    pub listener_type: ListenerType,
    pub priority: Priority,
    pub capability: ListenerCapability,
}

impl ListenerInfo {
    pub fn new(listener_type: ListenerType) -> Self {
        ListenerInfo {
            listener_type,
            priority: Priority::Normal,
            capability: ListenerCapability::new(),
        }
    }
}

/// Listener type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ListenerType {
    System = 0,
    Application = 1,
    Device = 2,
    Network = 3,
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

/// Listener capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ListenerCapability {
    pub can_handle: bool,
    pub can_block: bool,
    pub can_modify: bool,
}

impl ListenerCapability {
    pub fn new() -> Self {
        ListenerCapability {
            can_handle: false,
            can_block: false,
            can_modify: false,
        }
    }

    pub fn full() -> Self {
        ListenerCapability {
            can_handle: true,
            can_block: true,
            can_modify: true,
        }
    }
}

/// Simple event listener (OOP: Concrete listener class)
pub struct SimpleEventListener {
    pub listener_type: ListenerType,
    pub priority: Priority,
    pub capability: ListenerCapability,
    pub callback: Option<fn(&dyn Event) -> EventResult>,
    pub event_count: AtomicUsize,
}

impl SimpleEventListener {
    pub fn new(listener_type: ListenerType, priority: Priority, capability: ListenerCapability) -> Self {
        SimpleEventListener {
            listener_type,
            priority,
            capability,
            callback: None,
            event_count: AtomicUsize::new(0),
        }
    }

    pub fn set_callback(&mut self, callback: fn(&dyn Event) -> EventResult) {
        self.callback = Some(callback);
    }
}

impl EventListener for SimpleEventListener {
    fn handle(&mut self, event: &dyn Event) -> EventResult {
        if !self.capability.can_handle {
            return EventResult::Ignored;
        }

        self.event_count.fetch_add(1, Ordering::SeqCst);

        if let Some(callback) = self.callback {
            callback(event)
        } else {
            EventResult::Handled
        }
    }

    fn info(&self) -> ListenerInfo {
        ListenerInfo {
            listener_type: self.listener_type,
            priority: self.priority,
            capability: self.capability,
        }
    }
}

/// Event bus trait (OOP interface)
pub trait EventBus {
    /// Register listener
    fn register_listener(&mut self, listener: Box<dyn EventListener>) -> Result<usize, EventError>;
    /// Unregister listener
    fn unregister_listener(&mut self, id: usize) -> Result<(), EventError>;
    /// Publish event
    fn publish(&mut self, event: &dyn Event) -> EventResult;
    /// Subscribe to event type
    fn subscribe(&mut self, listener_id: usize, event_type: EventType) -> Result<(), EventError>;
    /// Unsubscribe from event type
    fn unsubscribe(&mut self, listener_id: usize, event_type: EventType) -> Result<(), EventError>;
    /// Get bus statistics
    fn stats(&self) -> EventStats;
}

/// Event error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EventError {
    Success = 0,
    ListenerNotFound = 1,
    AlreadyRegistered = 2,
    PermissionDenied = 3,
    InvalidEvent = 4,
}

/// Event statistics
#[repr(C)]
pub struct EventStats {
    pub total_listeners: usize,
    pub total_events: u64,
    pub handled_events: u64,
    pub ignored_events: u64,
}

impl EventStats {
    pub fn new() -> Self {
        EventStats {
            total_listeners: 0,
            total_events: 0,
            handled_events: 0,
            ignored_events: 0,
        }
    }
}

/// Simple event bus (OOP: Concrete bus class)
pub struct SimpleEventBus {
    listeners: Vec<Option<Box<dyn EventListener>>>,
    subscriptions: Vec<(usize, EventType)>,
    next_listener_id: AtomicUsize,
    stats: EventStats,
    capability: BusCapability,
}

/// Bus capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BusCapability {
    pub can_register: bool,
    pub can_unregister: bool,
    pub can_publish: bool,
}

impl BusCapability {
    pub fn new() -> Self {
        BusCapability {
            can_register: false,
            can_unregister: false,
            can_publish: false,
        }
    }

    pub fn full() -> Self {
        BusCapability {
            can_register: true,
            can_unregister: true,
            can_publish: true,
        }
    }
}

impl SimpleEventBus {
    pub fn new(capability: BusCapability) -> Self {
        SimpleEventBus {
            listeners: Vec::new(),
            subscriptions: Vec::new(),
            next_listener_id: AtomicUsize::new(1),
            stats: EventStats::new(),
            capability,
        }
    }
}

impl EventBus for SimpleEventBus {
    fn register_listener(&mut self, listener: Box<dyn EventListener>) -> Result<usize, EventError> {
        if !self.capability.can_register {
            return Err(EventError::PermissionDenied);
        }

        let id = self.next_listener_id.fetch_add(1, Ordering::SeqCst);
        self.listeners.push(Some(listener));
        self.stats.total_listeners += 1;
        Ok(id)
    }

    fn unregister_listener(&mut self, id: usize) -> Result<(), EventError> {
        if !self.capability.can_unregister {
            return Err(EventError::PermissionDenied);
        }

        if id < self.listeners.len() {
            self.listeners[id] = None;
            self.subscriptions.retain(|&(lid, _)| lid != id);
            self.stats.total_listeners -= 1;
            Ok(())
        } else {
            Err(EventError::ListenerNotFound)
        }
    }

    fn publish(&mut self, event: &dyn Event) -> EventResult {
        if !self.capability.can_publish {
            return EventResult::Error;
        }

        self.stats.total_events += 1;

        let mut handled = false;
        let mut consumed = false;

        for (listener_id, event_type) in &self.subscriptions {
            if *event_type == event.event_type() {
                if let Some(ref mut listener) = self.listeners[*listener_id] {
                    let result = listener.handle(event);
                    match result {
                        EventResult::Handled => handled = true,
                        EventResult::Consumed => {
                            handled = true;
                            consumed = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        if consumed {
            self.stats.handled_events += 1;
            EventResult::Consumed
        } else if handled {
            self.stats.handled_events += 1;
            EventResult::Handled
        } else {
            self.stats.ignored_events += 1;
            EventResult::Ignored
        }
    }

    fn subscribe(&mut self, listener_id: usize, event_type: EventType) -> Result<(), EventError> {
        if listener_id >= self.listeners.len() {
            return Err(EventError::ListenerNotFound);
        }

        if self.listeners[listener_id].is_none() {
            return Err(EventError::ListenerNotFound);
        }

        self.subscriptions.push((listener_id, event_type));
        Ok(())
    }

    fn unsubscribe(&mut self, listener_id: usize, event_type: EventType) -> Result<(), EventError> {
        let mut index = None;
        for (i, &(lid, et)) in self.subscriptions.iter().enumerate() {
            if lid == listener_id && et == event_type {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            self.subscriptions.remove(i);
            Ok(())
        } else {
            Err(EventError::ListenerNotFound)
        }
    }

    fn stats(&self) -> EventStats {
        self.stats
    }
}

/// Generate unique event ID
fn generate_event_id() -> EventID {
    static mut COUNTER: AtomicUsize = unsafe { AtomicUsize::new(1) };
    unsafe { COUNTER.fetch_add(1, Ordering::SeqCst) }
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

    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            core::ptr::copy(self.data.add(index + 1), self.data.add(index), self.len - index - 1);
            self.len -= 1;
            item
        }
    }

    fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&(usize, EventType)) -> bool,
    {
        let mut write = 0;
        for read in 0..self.len {
            unsafe {
                let item = &*self.data.add(read);
                if f(item) {
                    if write != read {
                        let item_copy = core::ptr::read(self.data.add(read));
                        core::ptr::write(self.data.add(write), item_copy);
                    }
                    write += 1;
                }
            }
        }
        self.len = write;
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
