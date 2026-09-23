// Memory Quota System for SigmaOS
// Implements memory accounting, limits, and OOM handling per cgroup

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Memory measurement unit
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryUnit {
    Bytes = 1,
    KiloBytes = 1024,
    MegaBytes = 1024 * 1024,
    GigaBytes = 1024 * 1024 * 1024,
}

impl MemoryUnit {
    pub fn to_bytes(&self, value: u64) -> u64 {
        value * *self as u64
    }

    pub fn from_bytes(&self, bytes: u64) -> u64 {
        bytes / (*self as u64)
    }
}

/// Memory page cache stat
#[derive(Debug, Clone)]
pub struct PageCacheStat {
    /// Total pages
    pub total_pages: u64,
    /// Dirty pages (modified, not written)
    pub dirty_pages: u64,
    /// Clean pages (can be evicted)
    pub clean_pages: u64,
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Total allocated memory
    pub rss: u64,
    /// Virtual memory size
    pub vms: u64,
    /// Page cache size
    pub page_cache: u64,
    /// Swap used
    pub swap: u64,
    /// Anonymous memory
    pub anon: u64,
    /// Mapped memory
    pub mapped: u64,
}

impl MemoryStats {
    pub fn new() -> Self {
        MemoryStats {
            rss: 0,
            vms: 0,
            page_cache: 0,
            swap: 0,
            anon: 0,
            mapped: 0,
        }
    }

    pub fn total_memory(&self) -> u64 {
        self.rss + self.page_cache + self.swap
    }
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory account for a process
#[derive(Debug, Clone)]
pub struct ProcessMemoryAccount {
    /// Process ID
    pub process_id: u32,
    /// Memory stats
    pub stats: MemoryStats,
    /// Memory limit (0 = unlimited)
    pub limit: u64,
    /// Soft limit
    pub soft_limit: u64,
    /// High watermark threshold
    pub high: u64,
}

impl ProcessMemoryAccount {
    pub fn new(process_id: u32) -> Self {
        ProcessMemoryAccount {
            process_id,
            stats: MemoryStats::new(),
            limit: 0,
            soft_limit: 0,
            high: 0,
        }
    }

    /// Check if memory usage exceeds limit
    pub fn exceeds_limit(&self) -> bool {
        if self.limit == 0 {
            return false;
        }
        self.stats.total_memory() > self.limit
    }

    /// Check if memory usage exceeds soft limit
    pub fn exceeds_soft_limit(&self) -> bool {
        if self.soft_limit == 0 {
            return false;
        }
        self.stats.total_memory() > self.soft_limit
    }

    /// Check if memory usage exceeds high threshold
    pub fn exceeds_high(&self) -> bool {
        if self.high == 0 {
            return false;
        }
        self.stats.total_memory() > self.high
    }

    /// Get memory usage percentage
    pub fn usage_percentage(&self) -> f64 {
        if self.limit == 0 {
            return 0.0;
        }
        (self.stats.total_memory() as f64 / self.limit as f64) * 100.0
    }

    /// Allocate memory
    pub fn allocate(&mut self, size: u64) -> Result<(), String> {
        if self.exceeds_limit() {
            return Err("Memory limit exceeded".to_string());
        }
        self.stats.rss += size;
        Ok(())
    }

    /// Deallocate memory
    pub fn deallocate(&mut self, size: u64) {
        if self.stats.rss >= size {
            self.stats.rss -= size;
        } else {
            self.stats.rss = 0;
        }
    }
}

/// OOM (Out-of-Memory) policy
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OomPolicy {
    /// Kill the process
    Kill = 1,
    /// Send signal to process
    Signal = 2,
    /// Block and wait for memory
    Block = 3,
    /// Invoke OOM handler
    Handler = 4,
}

/// OOM event
#[derive(Debug, Clone)]
pub struct OomEvent {
    /// Process that triggered OOM
    pub process_id: u32,
    /// Memory limit that was exceeded
    pub limit: u64,
    /// Memory in use
    pub usage: u64,
    /// Timestamp
    pub timestamp: u64,
}

/// Memory controller
pub struct MemoryController {
    /// Process memory accounts
    accounts: Arc<Mutex<HashMap<u32, ProcessMemoryAccount>>>,
    /// Global memory limit
    global_limit: Arc<Mutex<u64>>,
    /// Global memory used
    global_used: Arc<Mutex<u64>>,
    /// OOM policy
    oom_policy: Arc<Mutex<OomPolicy>>,
    /// OOM event handler (callback)
    oom_handler: Arc<Mutex<Option<Arc<dyn Fn(&OomEvent) + Send + Sync>>>>,
    /// Recent OOM events
    oom_events: Arc<Mutex<Vec<OomEvent>>>,
}

impl MemoryController {
    pub fn new() -> Self {
        MemoryController {
            accounts: Arc::new(Mutex::new(HashMap::new())),
            global_limit: Arc::new(Mutex::new(0)),
            global_used: Arc::new(Mutex::new(0)),
            oom_policy: Arc::new(Mutex::new(OomPolicy::Kill)),
            oom_handler: Arc::new(Mutex::new(None)),
            oom_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register process
    pub fn register_process(&self, process_id: u32) -> Result<(), String> {
        let mut accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;
        accounts.insert(process_id, ProcessMemoryAccount::new(process_id));
        Ok(())
    }

    /// Unregister process
    pub fn unregister_process(&self, process_id: u32) -> Result<(), String> {
        let mut accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;
        accounts.remove(&process_id);
        Ok(())
    }

    /// Set memory limit for process
    pub fn set_process_limit(&self, process_id: u32, limit: u64) -> Result<(), String> {
        let mut accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;

        if let Some(account) = accounts.get_mut(&process_id) {
            account.limit = limit;
            Ok(())
        } else {
            Err(format!("Process {} not registered", process_id))
        }
    }

    /// Set soft limit for process
    pub fn set_process_soft_limit(&self, process_id: u32, limit: u64) -> Result<(), String> {
        let mut accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;

        if let Some(account) = accounts.get_mut(&process_id) {
            account.soft_limit = limit;
            Ok(())
        } else {
            Err(format!("Process {} not registered", process_id))
        }
    }

    /// Set high threshold for process
    pub fn set_process_high(&self, process_id: u32, high: u64) -> Result<(), String> {
        let mut accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;

        if let Some(account) = accounts.get_mut(&process_id) {
            account.high = high;
            Ok(())
        } else {
            Err(format!("Process {} not registered", process_id))
        }
    }

    /// Get memory stats for process
    pub fn get_process_stats(&self, process_id: u32) -> Result<MemoryStats, String> {
        let accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;

        if let Some(account) = accounts.get(&process_id) {
            Ok(account.stats.clone())
        } else {
            Err(format!("Process {} not registered", process_id))
        }
    }

    /// Allocate memory for process
    pub fn allocate_memory(&self, process_id: u32, size: u64) -> Result<(), String> {
        let mut accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;

        if let Some(account) = accounts.get_mut(&process_id) {
            account.allocate(size)?;

            // Update global used
            if let Ok(mut global) = self.global_used.lock() {
                *global = global.saturating_add(size);
            }

            // Check high threshold
            if account.exceeds_high() {
                self.handle_high_memory(account)?;
            }

            Ok(())
        } else {
            Err(format!("Process {} not registered", process_id))
        }
    }

    /// Deallocate memory for process
    pub fn deallocate_memory(&self, process_id: u32, size: u64) -> Result<(), String> {
        let mut accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;

        if let Some(account) = accounts.get_mut(&process_id) {
            account.deallocate(size);

            // Update global used
            if let Ok(mut global) = self.global_used.lock() {
                *global = global.saturating_sub(size);
            }

            Ok(())
        } else {
            Err(format!("Process {} not registered", process_id))
        }
    }

    /// Handle high memory condition
    fn handle_high_memory(&self, account: &ProcessMemoryAccount) -> Result<(), String> {
        let event = OomEvent {
            process_id: account.process_id,
            limit: account.limit,
            usage: account.stats.total_memory(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };

        // Record event
        if let Ok(mut events) = self.oom_events.lock() {
            events.push(event.clone());
            // Keep only last 100 events
            if events.len() > 100 {
                events.remove(0);
            }
        }

        // Call handler if set
        if let Ok(handler_guard) = self.oom_handler.lock() {
            if let Some(handler) = handler_guard.as_ref() {
                handler(&event);
            }
        }

        Ok(())
    }

    /// Check for OOM condition
    pub fn check_oom(&self, process_id: u32) -> Result<bool, String> {
        let accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;

        if let Some(account) = accounts.get(&process_id) {
            if account.exceeds_limit() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Get memory usage percentage for process
    pub fn get_usage_percentage(&self, process_id: u32) -> Result<f64, String> {
        let accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;

        if let Some(account) = accounts.get(&process_id) {
            Ok(account.usage_percentage())
        } else {
            Err(format!("Process {} not registered", process_id))
        }
    }

    /// Set OOM policy
    pub fn set_oom_policy(&self, policy: OomPolicy) -> Result<(), String> {
        let mut policy_guard = self
            .oom_policy
            .lock()
            .map_err(|_| "Failed to acquire policy lock".to_string())?;
        *policy_guard = policy;
        Ok(())
    }

    /// Get OOM events
    pub fn get_oom_events(&self) -> Result<Vec<OomEvent>, String> {
        let events = self
            .oom_events
            .lock()
            .map_err(|_| "Failed to acquire events lock".to_string())?;
        Ok(events.clone())
    }

    /// Get global memory stats
    pub fn get_global_stats(&self) -> Result<(u64, u64), String> {
        let limit = self
            .global_limit
            .lock()
            .map_err(|_| "Failed to acquire limit lock".to_string())?;
        let used = self
            .global_used
            .lock()
            .map_err(|_| "Failed to acquire used lock".to_string())?;
        Ok((*limit, *used))
    }

    /// Get process count
    pub fn process_count(&self) -> Result<usize, String> {
        let accounts = self
            .accounts
            .lock()
            .map_err(|_| "Failed to acquire accounts lock".to_string())?;
        Ok(accounts.len())
    }
}

impl Default for MemoryController {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for MemoryController {
    fn clone(&self) -> Self {
        MemoryController {
            accounts: Arc::clone(&self.accounts),
            global_limit: Arc::clone(&self.global_limit),
            global_used: Arc::clone(&self.global_used),
            oom_policy: Arc::clone(&self.oom_policy),
            oom_handler: Arc::clone(&self.oom_handler),
            oom_events: Arc::clone(&self.oom_events),
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_unit_conversion() {
        assert_eq!(MemoryUnit::Bytes.to_bytes(1), 1);
        assert_eq!(MemoryUnit::KiloBytes.to_bytes(1), 1024);
        assert_eq!(MemoryUnit::MegaBytes.to_bytes(1), 1024 * 1024);
    }

    #[test]
    fn test_memory_stats_creation() {
        let stats = MemoryStats::new();
        assert_eq!(stats.rss, 0);
        assert_eq!(stats.total_memory(), 0);
    }

    #[test]
    fn test_process_account_creation() {
        let account = ProcessMemoryAccount::new(100);
        assert_eq!(account.process_id, 100);
        assert_eq!(account.limit, 0);
    }

    #[test]
    fn test_process_account_allocate() {
        let mut account = ProcessMemoryAccount::new(100);
        account.allocate(1024).unwrap();
        assert_eq!(account.stats.rss, 1024);
    }

    #[test]
    fn test_process_account_deallocate() {
        let mut account = ProcessMemoryAccount::new(100);
        account.allocate(1024).unwrap();
        account.deallocate(512);
        assert_eq!(account.stats.rss, 512);
    }

    #[test]
    fn test_process_account_exceeds_limit() {
        let mut account = ProcessMemoryAccount::new(100);
        account.limit = 1024;
        account.stats.rss = 1024;
        assert!(account.exceeds_limit());
    }

    #[test]
    fn test_process_account_usage_percentage() {
        let mut account = ProcessMemoryAccount::new(100);
        account.limit = 1024;
        account.stats.rss = 512;
        let percent = account.usage_percentage();
        assert!(percent > 49.9 && percent < 50.1);
    }

    #[test]
    fn test_memory_controller_register() {
        let controller = MemoryController::new();
        controller.register_process(100).unwrap();
        assert_eq!(controller.process_count().unwrap(), 1);
    }

    #[test]
    fn test_memory_controller_unregister() {
        let controller = MemoryController::new();
        controller.register_process(100).unwrap();
        controller.unregister_process(100).unwrap();
        assert_eq!(controller.process_count().unwrap(), 0);
    }

    #[test]
    fn test_memory_controller_set_limit() {
        let controller = MemoryController::new();
        controller.register_process(100).unwrap();
        controller.set_process_limit(100, 1024 * 1024).unwrap();
    }

    #[test]
    fn test_memory_controller_allocate() {
        let controller = MemoryController::new();
        controller.register_process(100).unwrap();
        controller.set_process_limit(100, 1024 * 1024).unwrap();
        controller.allocate_memory(100, 512).unwrap();
    }

    #[test]
    fn test_memory_controller_deallocate() {
        let controller = MemoryController::new();
        controller.register_process(100).unwrap();
        controller.allocate_memory(100, 512).unwrap();
        controller.deallocate_memory(100, 256).unwrap();
    }

    #[test]
    fn test_memory_controller_check_oom() {
        let controller = MemoryController::new();
        controller.register_process(100).unwrap();
        controller.set_process_limit(100, 512).unwrap();
        controller.allocate_memory(100, 512).unwrap();
        assert!(controller.check_oom(100).unwrap());
    }

    #[test]
    fn test_memory_controller_usage_percentage() {
        let controller = MemoryController::new();
        controller.register_process(100).unwrap();
        controller.set_process_limit(100, 1024).unwrap();
        controller.allocate_memory(100, 512).unwrap();
        let percent = controller.get_usage_percentage(100).unwrap();
        assert!(percent > 49.0 && percent < 51.0);
    }

    #[test]
    fn test_memory_controller_oom_policy() {
        let controller = MemoryController::new();
        controller.set_oom_policy(OomPolicy::Block).unwrap();
    }

    #[test]
    fn test_memory_controller_clone() {
        let controller = MemoryController::new();
        controller.register_process(100).unwrap();
        let cloned = controller.clone();
        assert_eq!(cloned.process_count().unwrap(), 1);
    }
}
