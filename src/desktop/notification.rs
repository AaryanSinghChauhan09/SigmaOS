#![no_std]
#![no_main]

/// OOP-based Desktop Notification for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 746
/// Implements notification system and alerts

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type NotificationID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NotificationUrgency { Low = 0, Normal = 1, Critical = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NotificationError { Success = 0, NotFound = 1 }

pub trait Notification {
    fn id(&self) -> NotificationID;
    fn title(&self) -> &[u8];
    fn body(&self) -> &[u8];
    fn urgency(&self) -> NotificationUrgency;
    fn app_name(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleNotification {
    pub id: NotificationID,
    pub title: [u8; 128],
    pub body: [u8; 512],
    pub urgency: AtomicUsize,
    pub app_name: [u8; 64],
}

impl SimpleNotification {
    pub fn new(id: NotificationID, title: &[u8], body: &[u8], urgency: NotificationUrgency, app_name: &[u8]) -> Self {
        let mut title_array = [0u8; 128];
        let mut body_array = [0u8; 512];
        let mut app_array = [0u8; 64];
        let title_len = title.len().min(127);
        let body_len = body.len().min(511);
        let app_len = app_name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(title.as_ptr(), title_array.as_mut_ptr(), title_len);
            core::ptr::copy_nonoverlapping(body.as_ptr(), body_array.as_mut_ptr(), body_len);
            core::ptr::copy_nonoverlapping(app_name.as_ptr(), app_array.as_mut_ptr(), app_len);
        }
        SimpleNotification {
            id,
            title: title_array,
            body: body_array,
            urgency: AtomicUsize::new(urgency as usize),
            app_name: app_array,
        }
    }
}

impl Notification for SimpleNotification {
    fn id(&self) -> NotificationID { self.id }
    fn title(&self) -> &[u8] {
        let len = self.title.iter().position(|&b| b == 0).unwrap_or(128);
        &self.title[..len]
    }
    fn body(&self) -> &[u8] {
        let len = self.body.iter().position(|&b| b == 0).unwrap_or(512);
        &self.body[..len]
    }
    fn urgency(&self) -> NotificationUrgency { unsafe { core::mem::transmute(self.urgency.load(Ordering::SeqCst)) } }
    fn app_name(&self) -> &[u8] {
        let len = self.app_name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.app_name[..len]
    }
}

pub trait NotificationManager {
    fn show_notification(&mut self, notification: Box<dyn Notification>) -> Result<NotificationID, NotificationError>;
    fn dismiss_notification(&mut self, id: NotificationID) -> Result<(), NotificationError>;
    fn get_notification(&self, id: NotificationID) -> Option<&dyn Notification>;
    fn get_active_notifications(&self) -> Vec<NotificationID>;
}

#[repr(C)]
pub struct SimpleNotificationManager {
    pub notifications: Vec<Option<Box<dyn Notification>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNotificationManager {
    pub fn new() -> Self {
        SimpleNotificationManager {
            notifications: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl NotificationManager for SimpleNotificationManager {
    fn show_notification(&mut self, notification: Box<dyn Notification>) -> Result<NotificationID, NotificationError> {
        let id = notification.id();
        self.notifications.push(Some(notification));
        Ok(id)
    }
    
    fn dismiss_notification(&mut self, id: NotificationID) -> Result<(), NotificationError> {
        for notification_option in &mut self.notifications {
            if let Some(ref notification) = *notification_option {
                if notification.id() == id {
                    return Ok(());
                }
            }
        }
        Err(NotificationError::NotFound)
    }
    
    fn get_notification(&self, id: NotificationID) -> Option<&dyn Notification> {
        for notification_option in &self.notifications {
            if let Some(ref notification) = *notification_option {
                if notification.id() == id { return Some(notification.as_ref()); }
            }
        }
        None
    }
    
    fn get_active_notifications(&self) -> Vec<NotificationID> {
        let mut ids = Vec::new();
        for notification_option in &self.notifications {
            if let Some(ref notification) = *notification_option {
                ids.push(notification.id());
            }
        }
        ids
    }
}

pub trait DoNotDisturb {
    fn enable_dnd(&mut self, enabled: bool);
    fn is_dnd_enabled(&self) -> bool;
    def set_dnd_exceptions(&mut self, app_name: &[u8]);
}

#[repr(C)]
pub struct SimpleDoNotDisturb {
    pub enabled: AtomicUsize,
    pub exceptions: Vec<[u8; ]>,
}

impl SimpleDoNotDisturb {
    pub fn new() -> Self {
        SimpleDoNotDisturb {
            enabled: AtomicUsize::new(0),
            exceptions: Vec::new(),
        }
    }
}

impl DoNotDisturb for SimpleDoNotDisturb {
    fn enable_dnd(&mut self, enabled: bool) {
        self.enabled.store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
    }
    
    fn is_dnd_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
    
    fn set_dnd_exceptions(&mut self, app_name: &[u8]) {
        let mut app_array = [0u8; 64];
        let app_len = app_name.len().min(63);
        for i in 0..app_len {
            app_array[i] = app_name[i];
        }
        self.exceptions.push(app_array);
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
