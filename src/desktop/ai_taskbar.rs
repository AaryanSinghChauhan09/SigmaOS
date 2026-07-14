#![no_std]

/// AI-Powered Taskbar with Suggestions for SigmaOS
/// Based on 100-Improvement-Ideas.md #49: Taskbar with AI suggestions
/// Implements intelligent taskbar with AI-powered app suggestions

use core::sync::atomic::{AtomicU64, Ordering};

/// Taskbar item type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarItemType {
    Application = 0,
    System = 1,
    Notification = 2,
    Suggestion = 3,
}

/// Taskbar item
#[repr(C)]
pub struct TaskbarItem {
    pub id: u64,
    pub item_type: TaskbarItemType,
    pub name: [u8; 64],
    pub icon: [u8; 32],
    pub pinned: bool,
    pub running: bool,
    pub ai_relevance: f32,
}

impl TaskbarItem {
    pub fn new(id: u64, item_type: TaskbarItemType, name: &str) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        TaskbarItem {
            id,
            item_type,
            name: name_array,
            icon: [0u8; 32],
            pinned: false,
            running: false,
            ai_relevance: 0.0,
        }
    }
    
    pub fn name_str(&self) -> &str {
        unsafe {
            let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
            core::str::from_utf8_unchecked(&self.name[..len])
        }
    }
    
    pub fn set_ai_relevance(&mut self, relevance: f32) {
        self.ai_relevance = relevance.min(1.0).max(0.0);
    }
}

/// AI suggestion
#[repr(C)]
pub struct AISuggestion {
    pub id: u64,
    pub app_name: [u8; 64],
    pub reason: [u8; 128],
    pub confidence: f32,
}

impl AISuggestion {
    pub fn new(id: u64, app_name: &str, reason: &str) -> Self {
        let mut app_array = [0u8; 64];
        let app_bytes = app_name.as_bytes();
        let app_len = app_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(app_bytes.as_ptr(), app_array.as_mut_ptr(), app_len);
        }
        
        let mut reason_array = [0u8; 128];
        let reason_bytes = reason.as_bytes();
        let reason_len = reason_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(reason_bytes.as_ptr(), reason_array.as_mut_ptr(), reason_len);
        }
        
        AISuggestion {
            id,
            app_name: app_array,
            reason: reason_array,
            confidence: 0.0,
        }
    }
}

/// Usage context
#[repr(C)]
pub struct UsageContext {
    pub time_of_day: u8, // 0-23
    pub day_of_week: u8, // 0-6
    pub current_app: [u8; 64],
    pub recent_apps: Vec<[u8; 64]>,
}

impl UsageContext {
    pub fn new() -> Self {
        UsageContext {
            time_of_day: 12,
            day_of_week: 3,
            current_app: [0u8; 64],
            recent_apps: Vec::new(),
        }
    }
}

/// AI taskbar manager
pub struct AITaskbarManager {
    pub items: Vec<Option<TaskbarItem>>,
    pub suggestions: Vec<Option<AISuggestion>>,
    pub context: UsageContext,
    pub next_item_id: AtomicU64,
    pub next_suggestion_id: AtomicU64,
}

impl AITaskbarManager {
    pub fn new() -> Self {
        AITaskbarManager {
            items: Vec::new(),
            suggestions: Vec::new(),
            context: UsageContext::new(),
            next_item_id: AtomicU64::new(1),
            next_suggestion_id: AtomicU64::new(1),
        }
    }
    
    /// Add taskbar item
    pub fn add_item(&mut self, item_type: TaskbarItemType, name: &str) -> u64 {
        let id = self.next_item_id.fetch_add(1, Ordering::SeqCst);
        let item = TaskbarItem::new(id, item_type, name);
        self.items.push(Some(item));
        id
    }
    
    /// Pin item
    pub fn pin_item(&mut self, id: u64) -> bool {
        for item_option in &mut self.items {
            if let Some(ref mut item) = *item_option {
                if item.id == id {
                    item.pinned = true;
                    return true;
                }
            }
        }
        false
    }
    
    /// Unpin item
    pub fn unpin_item(&mut self, id: u64) -> bool {
        for item_option in &mut self.items {
            if let Some(ref mut item) = *item_option {
                if item.id == id {
                    item.pinned = false;
                    return true;
                }
            }
        }
        false
    }
    
    /// Set item running state
    pub fn set_running(&mut self, id: u64, running: bool) -> bool {
        for item_option in &mut self.items {
            if let Some(ref mut item) = *item_option {
                if item.id == id {
                    item.running = running;
                    return true;
                }
            }
        }
        false
    }
    
    /// Generate AI suggestions
    pub fn generate_suggestions(&mut self) -> Vec<&AISuggestion> {
        self.suggestions = Vec::new();
        
        // Simple heuristic-based suggestions
        // In real implementation, use ML model
        
        // Time-based suggestions
        if self.context.time_of_day >= 9 && self.context.time_of_day <= 17 {
            // Work hours
            let id = self.next_suggestion_id.fetch_add(1, Ordering::SeqCst);
            let suggestion = AISuggestion::new(id, "Code Editor", "Work hours detected");
            self.suggestions.push(Some(suggestion));
        }
        
        if self.context.time_of_day >= 18 && self.context.time_of_day <= 23 {
            // Evening
            let id = self.next_suggestion_id.fetch_add(1, Ordering::SeqCst);
            let suggestion = AISuggestion::new(id, "Media Player", "Evening relaxation time");
            self.suggestions.push(Some(suggestion));
        }
        
        let mut result = Vec::new();
        for suggestion_option in &self.suggestions {
            if let Some(ref suggestion) = *suggestion_option {
                result.push(suggestion);
            }
        }
        result
    }
    
    /// Update context
    pub fn update_context(&mut self, time_of_day: u8, day_of_week: u8, current_app: &str) {
        self.context.time_of_day = time_of_day;
        self.context.day_of_week = day_of_week;
        
        let app_bytes = current_app.as_bytes();
        let len = app_bytes.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(app_bytes.as_ptr(), self.context.current_app.as_mut_ptr(), len);
        }
    }
    
    /// Get items sorted by AI relevance
    pub fn get_sorted_items(&self) -> Vec<&TaskbarItem> {
        let mut items: Vec<&TaskbarItem> = Vec::new();
        
        for item_option in &self.items {
            if let Some(ref item) = *item_option {
                items.push(item);
            }
        }
        
        // Sort by AI relevance (descending)
        for i in 0..items.len() {
            for j in i+1..items.len() {
                if items[j].ai_relevance > items[i].ai_relevance {
                    let temp = items[i];
                    items[i] = items[j];
                    items[j] = temp;
                }
            }
        }
        
        items
    }
    
    /// Get pinned items
    pub fn get_pinned_items(&self) -> Vec<&TaskbarItem> {
        let mut pinned = Vec::new();
        for item_option in &self.items {
            if let Some(ref item) = *item_option {
                if item.pinned {
                    pinned.push(item);
                }
            }
        }
        pinned
    }
    
    /// Get running items
    pub fn get_running_items(&self) -> Vec<&TaskbarItem> {
        let mut running = Vec::new();
        for item_option in &self.items {
            if let Some(ref item) = *item_option {
                if item.running {
                    running.push(item);
                }
            }
        }
        running
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
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

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
