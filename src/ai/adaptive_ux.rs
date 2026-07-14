#![no_std]

/// Adaptive UX Personalization Agent for SigmaOS
/// Based on 100-Improvement-Ideas.md #53: Adaptive UX personalization agent
/// Implements AI-driven UI adaptation based on user behavior

use core::sync::atomic::{AtomicU64, Ordering};

/// User behavior pattern
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehaviorPattern {
    FrequentApp = 0,
    TimeOfDay = 1,
    Location = 2,
    TaskContext = 3,
}

/// UX preference
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UXPreference {
    Theme = 0,
    Layout = 1,
    FontSize = 2,
    IconSize = 3,
    AnimationSpeed = 4,
}

/// User action
#[repr(C)]
pub struct UserAction {
    pub id: u64,
    pub action_type: [u8; 64],
    pub app_name: [u8; 64],
    pub timestamp: u64,
    pub context: [u8; 128],
}

impl UserAction {
    pub fn new(id: u64, action_type: &str, app_name: &str, context: &str) -> Self {
        let mut type_array = [0u8; 64];
        let type_bytes = action_type.as_bytes();
        let type_len = type_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(type_bytes.as_ptr(), type_array.as_mut_ptr(), type_len);
        }
        
        let mut app_array = [0u8; 64];
        let app_bytes = app_name.as_bytes();
        let app_len = app_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(app_bytes.as_ptr(), app_array.as_mut_ptr(), app_len);
        }
        
        let mut context_array = [0u8; 128];
        let context_bytes = context.as_bytes();
        let context_len = context_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(context_bytes.as_ptr(), context_array.as_mut_ptr(), context_len);
        }
        
        UserAction {
            id,
            action_type: type_array,
            app_name: app_array,
            timestamp: get_current_time(),
            context: context_array,
        }
    }
}

/// UX adaptation rule
#[repr(C)]
pub struct UXAdaptationRule {
    pub id: u64,
    pub pattern: BehaviorPattern,
    pub preference: UXPreference,
    pub value: [u8; 64],
    pub confidence: f32,
}

impl UXAdaptationRule {
    pub fn new(id: u64, pattern: BehaviorPattern, preference: UXPreference, value: &str) -> Self {
        let mut value_array = [0u8; 64];
        let value_bytes = value.as_bytes();
        let value_len = value_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(value_bytes.as_ptr(), value_array.as_mut_ptr(), value_len);
        }
        
        UXAdaptationRule {
            id,
            pattern,
            preference,
            value: value_array,
            confidence: 0.0,
        }
    }
}

/// Adaptive UX agent
pub struct AdaptiveUXAgent {
    pub user_actions: Vec<Option<UserAction>>,
    pub adaptation_rules: Vec<Option<UXAdaptationRule>>,
    pub next_action_id: AtomicU64,
    pub next_rule_id: AtomicU64,
    pub adaptation_enabled: bool,
}

impl AdaptiveUXAgent {
    pub fn new() -> Self {
        AdaptiveUXAgent {
            user_actions: Vec::new(),
            adaptation_rules: Vec::new(),
            next_action_id: AtomicU64::new(1),
            next_rule_id: AtomicU64::new(1),
            adaptation_enabled: true,
        }
    }
    
    /// Record user action
    pub fn record_action(&mut self, action_type: &str, app_name: &str, context: &str) -> u64 {
        let id = self.next_action_id.fetch_add(1, Ordering::SeqCst);
        let action = UserAction::new(id, action_type, app_name, context);
        self.user_actions.push(Some(action));
        id
    }
    
    /// Analyze patterns
    pub fn analyze_patterns(&self) -> Vec<&UserAction> {
        let mut recent_actions = Vec::new();
        
        // Get recent actions (last 100)
        let start = if self.user_actions.len() > 100 {
            self.user_actions.len() - 100
        } else {
            0
        };
        
        for i in start..self.user_actions.len() {
            if let Some(ref action) = self.user_actions[i] {
                recent_actions.push(action);
            }
        }
        
        recent_actions
    }
    
    /// Create adaptation rule
    pub fn create_rule(&mut self, pattern: BehaviorPattern, preference: UXPreference, value: &str) -> u64 {
        let id = self.next_rule_id.fetch_add(1, Ordering::SeqCst);
        let rule = UXAdaptationRule::new(id, pattern, preference, value);
        self.adaptation_rules.push(Some(rule));
        id
    }
    
    /// Apply adaptation
    pub fn apply_adaptation(&self, preference: UXPreference) -> Option<&str> {
        for rule_option in &self.adaptation_rules {
            if let Some(ref rule) = *rule_option {
                if rule.preference == preference && rule.confidence > 0.7 {
                    let value_str = unsafe {
                        let len = rule.value.iter().position(|&b| b == 0).unwrap_or(64);
                        core::str::from_utf8_unchecked(&rule.value[..len])
                    };
                    return Some(value_str);
                }
            }
        }
        None
    }
    
    /// Learn from user actions
    pub fn learn(&mut self) {
        if !self.adaptation_enabled {
            return;
        }
        
        let actions = self.analyze_patterns();
        
        // Simple pattern learning
        // In real implementation, use ML model
        let mut app_counts: [u32; 10] = [0; 10];
        
        for action in actions {
            let app_name = unsafe {
                let len = action.app_name.iter().position(|&b| b == 0).unwrap_or(64);
                core::str::from_utf8_unchecked(&action.app_name[..len])
            };
            
            // Hash app name to index
            let hash = self.simple_hash(app_name);
            let index = (hash % 10) as usize;
            app_counts[index] += 1;
        }
        
        // Create rules for frequently used apps
        for i in 0..10 {
            if app_counts[i] > 10 {
                let id = self.next_rule_id.fetch_add(1, Ordering::SeqCst);
                let rule = UXAdaptationRule::new(id, BehaviorPattern::FrequentApp, UXPreference::Layout, "optimized");
                rule.confidence = 0.8;
                self.adaptation_rules.push(Some(rule));
            }
        }
    }
    
    fn simple_hash(&self, s: &str) -> u32 {
        let mut hash: u32 = 0;
        for c in s.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(c as u32);
        }
        hash
    }
    
    /// Get adaptation rules
    pub fn get_rules(&self) -> Vec<&UXAdaptationRule> {
        let mut rules = Vec::new();
        for rule_option in &self.adaptation_rules {
            if let Some(ref rule) = *rule_option {
                rules.push(rule);
            }
        }
        rules
    }
    
    /// Enable/disable adaptation
    pub fn set_adaptation_enabled(&mut self, enabled: bool) {
        self.adaptation_enabled = enabled;
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

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1_000_000, Ordering::SeqCst)
}
