// SPDX-License-Identifier: MIT
// SigmaOS - GameMode (Performance Optimizer for Games)
// Inspired by Linux GameMode (com.github.feralinteractive.GameMode)
// Provides temporary system optimization for gaming

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Game optimization state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOptimizationState {
    Inactive,
    Activating,
    Active,
    Deactivating,
}

/// Game mode configuration
#[derive(Debug, Clone, Copy)]
pub struct GameModeConfig {
    pub disable_compositor: bool,
    pub disable_kernel_samepage_merge: bool,
    pub ioprio_high: bool,
    pub renice_realtime: bool,
    pub inhibit_screensaver: bool,
    pub cpu_governor_performance: bool,
    pub gpu_performance_mode: bool,
}

impl Default for GameModeConfig {
    fn default() -> Self {
        Self {
            disable_compositor: true,
            disable_kernel_samepage_merge: true,
            ioprio_high: true,
            renice_realtime: true,
            inhibit_screensaver: true,
            cpu_governor_performance: true,
            gpu_performance_mode: true,
        }
    }
}

/// Game mode instance
pub struct GameModeInstance {
    pub pid: u32,
    pub game_name: String,
    pub state: GameOptimizationState,
    pub config: GameModeConfig,
    pub original_cpu_governor: Option<String>,
    pub original_gpu_profile: Option<String>,
    pub start_time: u64,
}

impl GameModeInstance {
    pub fn new(pid: u32, game_name: String, config: GameModeConfig) -> Self {
        Self {
            pid,
            game_name,
            state: GameOptimizationState::Inactive,
            config,
            original_cpu_governor: None,
            original_gpu_profile: None,
            start_time: 0,
        }
    }
    
    /// Activate game mode
    pub fn activate(&mut self) -> Result<(), &'static str> {
        self.state = GameOptimizationState::Activating;
        
        // Save original settings
        self.original_cpu_governor = Some("powersave".to_string());
        self.original_gpu_profile = Some("balanced".to_string());
        
        // Apply optimizations
        if self.config.cpu_governor_performance {
            // Set CPU governor to performance
            // In real implementation: echo performance > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
        }
        
        if self.config.gpu_performance_mode {
            // Set GPU to performance mode
            // In real implementation: Use GPU frequency scaling
        }
        
        if self.config.disable_compositor {
            // Disable compositor
            // In real implementation: Stop compositor service
        }
        
        if self.config.inhibit_screensaver {
            // Inhibit screensaver
            // In real implementation: Use systemd-inhibit
        }
        
        self.state = GameOptimizationState::Active;
        self.start_time = 0; // Would be actual timestamp
        
        Ok(())
    }
    
    /// Deactivate game mode
    pub fn deactivate(&mut self) -> Result<(), &'static str> {
        self.state = GameOptimizationState::Deactivating;
        
        // Restore original settings
        if let Some(ref governor) = self.original_cpu_governor {
            // Restore CPU governor
            // In real implementation: echo {governor} > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
        }
        
        if let Some(ref profile) = self.original_gpu_profile {
            // Restore GPU profile
            // In real implementation: Use GPU frequency scaling
        }
        
        if self.config.disable_compositor {
            // Re-enable compositor
            // In real implementation: Start compositor service
        }
        
        if self.config.inhibit_screensaver {
            // Uninhibit screensaver
            // In real implementation: Release systemd-inhibit lock
        }
        
        self.state = GameOptimizationState::Inactive;
        
        Ok(())
    }
    
    /// Get runtime
    pub fn get_runtime(&self) -> u64 {
        // In real implementation, would calculate from start_time
        0
    }
}

/// Game mode manager
pub struct GameModeManager {
    pub instances: BTreeMap<u32, GameModeInstance>,
    pub active_games: Vec<u32>,
    pub config: GameModeConfig,
    pub active: AtomicBool,
}

impl GameModeManager {
    pub fn new() -> Self {
        Self {
            instances: BTreeMap::new(),
            active_games: Vec::new(),
            config: GameModeConfig::default(),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize game mode manager
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.active.store(true, Ordering::Relaxed);
        Ok(())
    }
    
    /// Register game
    pub fn register_game(&mut self, pid: u32, game_name: String) -> Result<(), &'static str> {
        let instance = GameModeInstance::new(pid, game_name, self.config);
        self.instances.insert(pid, instance);
        Ok(())
    }
    
    /// Request game mode for PID
    pub fn request_game_mode(&mut self, pid: u32) -> Result<(), &'static str> {
        let instance = self.instances.get_mut(&pid).ok_or("Game not registered")?;
        
        instance.activate()?;
        self.active_games.push(pid);
        
        Ok(())
    }
    
    /// Release game mode for PID
    pub fn release_game_mode(&mut self, pid: u32) -> Result<(), &'static str> {
        let instance = self.instances.get_mut(&pid).ok_or("Game not registered")?;
        
        instance.deactivate()?;
        self.active_games.retain(|&p| p != pid);
        
        Ok(())
    }
    
    /// Get instance
    pub fn get_instance(&self, pid: u32) -> Option<&GameModeInstance> {
        self.instances.get(&pid)
    }
    
    /// Is game mode active for PID
    pub fn is_game_mode_active(&self, pid: u32) -> bool {
        self.active_games.contains(&pid)
    }
    
    /// Get active games count
    pub fn get_active_count(&self) -> u32 {
        self.active_games.len() as u32
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: GameModeConfig) {
        self.config = config;
    }
    
    /// Get configuration
    pub fn get_config(&self) -> GameModeConfig {
        self.config
    }
}

impl Default for GameModeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global game mode manager
static mut GAMEMODE_MANAGER: Option<GameModeManager> = None;

/// Initialize game mode manager
pub fn init_gamemode_manager() {
    unsafe {
        GAMEMODE_MANAGER = Some(GameModeManager::new());
    }
}

/// Get global game mode manager
pub fn get_gamemode_manager() -> Option<&'static mut GameModeManager> {
    unsafe {
        GAMEMODE_MANAGER.as_mut()
    }
}

/// Request game mode (for compatibility with Linux GameMode)
pub fn gamemode_request_start() -> i32 {
    if let Some(manager) = get_gamemode_manager() {
        // In real implementation, would use current PID
        let pid = 1234;
        if manager.register_game(pid, "unknown".to_string()).is_ok() {
            if manager.request_game_mode(pid).is_ok() {
                return 1; // Success
            }
        }
    }
    0 // Failure
}

/// Release game mode (for compatibility with Linux GameMode)
pub fn gamemode_request_end() -> i32 {
    if let Some(manager) = get_gamemode_manager() {
        let pid = 1234;
        if manager.release_game_mode(pid).is_ok() {
            return 1; // Success
        }
    }
    0 // Failure
}

/// Query game mode status
pub fn gamemode_query_status() -> i32 {
    if let Some(manager) = get_gamemode_manager() {
        if manager.get_active_count() > 0 {
            return 1; // Active
        }
    }
    0 // Inactive
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gamemode_manager_init() {
        let mut manager = GameModeManager::new();
        manager.init().unwrap();
        assert!(manager.active.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_register_game() {
        let mut manager = GameModeManager::new();
        manager.register_game(1234, "TestGame".to_string()).unwrap();
        assert!(manager.get_instance(1234).is_some());
    }
    
    #[test]
    fn test_request_game_mode() {
        let mut manager = GameModeManager::new();
        manager.register_game(1234, "TestGame".to_string()).unwrap();
        manager.request_game_mode(1234).unwrap();
        assert!(manager.is_game_mode_active(1234));
        assert_eq!(manager.get_instance(1234).unwrap().state, GameOptimizationState::Active);
    }
    
    #[test]
    fn test_release_game_mode() {
        let mut manager = GameModeManager::new();
        manager.register_game(1234, "TestGame".to_string()).unwrap();
        manager.request_game_mode(1234).unwrap();
        manager.release_game_mode(1234).unwrap();
        assert!(!manager.is_game_mode_active(1234));
        assert_eq!(manager.get_instance(1234).unwrap().state, GameOptimizationState::Inactive);
    }
    
    #[test]
    fn test_gamemode_config() {
        let config = GameModeConfig::default();
        assert!(config.disable_compositor);
        assert!(config.cpu_governor_performance);
    }
    
    #[test]
    fn test_gamemode_api() {
        init_gamemode_manager();
        assert_eq!(gamemode_query_status(), 0);
        assert_eq!(gamemode_request_start(), 1);
        assert_eq!(gamemode_query_status(), 1);
        assert_eq!(gamemode_request_end(), 1);
        assert_eq!(gamemode_query_status(), 0);
    }
}
