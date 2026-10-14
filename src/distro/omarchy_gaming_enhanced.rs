// SPDX-License-Identifier: MIT
// SigmaOS - Enhanced Gaming Performance Suite
// Inspired by Omarchy Linux gaming optimizations
// Provides CPU/GPU governor switching, VRR, shader precaching, and gaming-specific optimizations

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// CPU governor modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuGovernor {
    Performance,
    Powersave,
    OnDemand,
    Conservative,
    Schedutil,
}

/// GPU performance profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuProfile {
    Performance,
    Balanced,
    PowerSaving,
}

/// Game trigger conditions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameTrigger {
    ProcessName(String),
    WindowTitle(String),
    SteamGameId(u32),
}

/// Gaming state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamingState {
    Inactive,
    Activating,
    Active,
    Deactivating,
}

/// VRR (Variable Refresh Rate) mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrrMode {
    Disabled,
    Enabled,
    AdaptiveSync,
    GSyncCompatible,
    FreeSyncCompatible,
}

/// HUD (Heads-Up Display) elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HudElement {
    Fps,
    CpuUsage,
    GpuUsage,
    RamUsage,
    Temperature,
    FrameTime,
    All,
}

/// Developer stack
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeveloperStack {
    Rust,
    Go,
    Node,
    Python,
    Zig,
    Nim,
    Cpp,
    Java,
}

/// Gaming performance suite
pub struct SovereignOmarchyGamingGovernor {
    pub cpu_governor: CpuGovernor,
    pub previous_cpu_governor: Option<CpuGovernor>,
    pub gpu_profile: GpuProfile,
    pub previous_gpu_profile: Option<GpuProfile>,
    pub gaming_state: GamingState,
    pub game_triggers: Vec<GameTrigger>,
    pub vrr_mode: VrrMode,
    pub previous_vrr_mode: Option<VrrMode>,
    pub hud_enabled: bool,
    pub hud_elements: Vec<HudElement>,
    pub hud_position: u32,
    pub developer_stacks: BTreeMap<DeveloperStack, bool>,
    pub vrr_range_min: u32,
    pub vrr_range_max: u32,
    pub shader_precache_enabled: bool,
    pub shader_cache_size: u64,
}

impl SovereignOmarchyGamingGovernor {
    pub fn new() -> Self {
        Self {
            cpu_governor: CpuGovernor::Powersave,
            previous_cpu_governor: None,
            gpu_profile: GpuProfile::Balanced,
            previous_gpu_profile: None,
            gaming_state: GamingState::Inactive,
            game_triggers: Vec::new(),
            vrr_mode: VrrMode::Disabled,
            previous_vrr_mode: None,
            hud_enabled: false,
            hud_elements: Vec::new(),
            hud_position: 0,
            developer_stacks: BTreeMap::new(),
            vrr_range_min: 48,
            vrr_range_max: 144,
            shader_precache_enabled: true,
            shader_cache_size: 1024 * 1024 * 1024, // 1GB
        }
    }

    /// Activate gaming mode
    pub fn activate_gaming_mode(&mut self) -> Result<(), &'static str> {
        if self.gaming_state == GamingState::Active {
            return Ok(());
        }

        self.gaming_state = GamingState::Activating;

        // Save previous governors
        self.previous_cpu_governor = Some(self.cpu_governor);
        self.previous_gpu_profile = Some(self.gpu_profile);
        self.previous_vrr_mode = Some(self.vrr_mode);

        // Set performance governors
        self.cpu_governor = CpuGovernor::Performance;
        self.gpu_profile = GpuProfile::Performance;

        // Enable VRR
        self.vrr_mode = VrrMode::AdaptiveSync;

        // Enable HUD
        self.hud_enabled = true;
        if self.hud_elements.is_empty() {
            self.hud_elements = vec![HudElement::Fps, HudElement::CpuUsage, HudElement::GpuUsage];
        }

        self.gaming_state = GamingState::Active;

        Ok(())
    }

    /// Deactivate gaming mode
    pub fn deactivate_gaming_mode(&mut self) -> Result<(), &'static str> {
        if self.gaming_state == GamingState::Inactive {
            return Ok(());
        }

        self.gaming_state = GamingState::Deactivating;

        // Restore previous governors
        if let Some(prev_cpu) = self.previous_cpu_governor {
            self.cpu_governor = prev_cpu;
        }
        if let Some(prev_gpu) = self.previous_gpu_profile {
            self.gpu_profile = prev_gpu;
        }
        if let Some(prev_vrr) = self.previous_vrr_mode {
            self.vrr_mode = prev_vrr;
        }

        // Disable HUD
        self.hud_enabled = false;

        self.gaming_state = GamingState::Inactive;

        Ok(())
    }

    /// Add game trigger
    pub fn add_game_trigger(&mut self, trigger: GameTrigger) {
        self.game_triggers.push(trigger);
    }

    /// Check if should activate gaming mode
    pub fn check_game_triggers(&self, process_name: &str, window_title: &str) -> bool {
        for trigger in &self.game_triggers {
            match trigger {
                GameTrigger::ProcessName(name) if name == process_name => return true,
                GameTrigger::WindowTitle(title) if title == window_title => return true,
                _ => {}
            }
        }
        false
    }

    /// Toggle HUD
    pub fn toggle_hud(&mut self) {
        self.hud_enabled = !self.hud_enabled;
    }

    /// Cycle HUD position
    pub fn cycle_hud_position(&mut self) {
        self.hud_position = (self.hud_position + 1) % 4;
    }

    /// Enable HUD element
    pub fn enable_hud_element(&mut self, element: HudElement) {
        if !self.hud_elements.contains(&element) {
            self.hud_elements.push(element);
        }
    }

    /// Disable HUD element
    pub fn disable_hud_element(&mut self, element: HudElement) {
        self.hud_elements.retain(|e| e != &element);
    }

    /// Enable developer stack
    pub fn enable_developer_stack(&mut self, stack: DeveloperStack) {
        self.developer_stacks.insert(stack, true);
    }

    /// Disable developer stack
    pub fn disable_developer_stack(&mut self, stack: DeveloperStack) {
        self.developer_stacks.insert(stack, false);
    }

    /// Set VRR range
    pub fn set_vrr_range(&mut self, min: u32, max: u32) {
        self.vrr_range_min = min;
        self.vrr_range_max = max;
    }

    /// Enable shader precache
    pub fn enable_shader_precache(&mut self, enabled: bool) {
        self.shader_precache_enabled = enabled;
    }

    /// Set shader cache size
    pub fn set_shader_cache_size(&mut self, size: u64) {
        self.shader_cache_size = size;
    }

    /// Get current gaming state
    pub fn get_gaming_state(&self) -> GamingState {
        self.gaming_state
    }

    /// Get current CPU governor
    pub fn get_cpu_governor(&self) -> CpuGovernor {
        self.cpu_governor
    }

    /// Get current GPU profile
    pub fn get_gpu_profile(&self) -> GpuProfile {
        self.gpu_profile
    }
}

impl Default for SovereignOmarchyGamingGovernor {
    fn default() -> Self {
        Self::new()
    }
}

/// HUD engine for gaming overlay
pub struct SovereignOmarchyHudEngine {
    pub enabled: AtomicBool,
    pub fps: AtomicU32,
    pub cpu_usage: AtomicU32,
    pub gpu_usage: AtomicU32,
    pub ram_usage: AtomicU32,
    pub temperature: AtomicU32,
    pub frame_time: AtomicU32,
}

impl SovereignOmarchyHudEngine {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            fps: AtomicU32::new(0),
            cpu_usage: AtomicU32::new(0),
            gpu_usage: AtomicU32::new(0),
            ram_usage: AtomicU32::new(0),
            temperature: AtomicU32::new(0),
            frame_time: AtomicU32::new(0),
        }
    }

    pub fn update_fps(&self, fps: u32) {
        self.fps.store(fps, Ordering::Relaxed);
    }

    pub fn update_cpu_usage(&self, usage: u32) {
        self.cpu_usage.store(usage, Ordering::Relaxed);
    }

    pub fn update_gpu_usage(&self, usage: u32) {
        self.gpu_usage.store(usage, Ordering::Relaxed);
    }

    pub fn update_ram_usage(&self, usage: u32) {
        self.ram_usage.store(usage, Ordering::Relaxed);
    }

    pub fn update_temperature(&self, temp: u32) {
        self.temperature.store(temp, Ordering::Relaxed);
    }

    pub fn update_frame_time(&self, time: u32) {
        self.frame_time.store(time, Ordering::Relaxed);
    }

    pub fn get_fps(&self) -> u32 {
        self.fps.load(Ordering::Relaxed)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }
}

impl Default for SovereignOmarchyHudEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// VRR pacing controller
pub struct SovereignOmarchyVrrPacingController {
    pub vrr_enabled: bool,
    pub min_refresh: u32,
    pub max_refresh: u32,
    pub current_refresh: u32,
    pub target_fps: u32,
}

impl SovereignOmarchyVrrPacingController {
    pub fn new() -> Self {
        Self {
            vrr_enabled: false,
            min_refresh: 48,
            max_refresh: 144,
            current_refresh: 60,
            target_fps: 60,
        }
    }

    pub fn enable_vrr(&mut self) {
        self.vrr_enabled = true;
    }

    pub fn disable_vrr(&mut self) {
        self.vrr_enabled = false;
    }

    pub fn set_vrr_range(&mut self, min: u32, max: u32) {
        self.min_refresh = min;
        self.max_refresh = max;
    }

    pub fn set_target_fps(&mut self, fps: u32) {
        self.target_fps = fps;
    }

    pub fn calculate_refresh_rate(&mut self) -> u32 {
        if self.vrr_enabled {
            // Calculate optimal refresh rate based on target FPS
            let optimal = self.target_fps.min(self.max_refresh).max(self.min_refresh);
            self.current_refresh = optimal;
        } else {
            self.current_refresh = 60; // Fixed 60Hz when VRR disabled
        }
        self.current_refresh
    }
}

impl Default for SovereignOmarchyVrrPacingController {
    fn default() -> Self {
        Self::new()
    }
}

/// Steam shader precache manager
pub struct SovereignSteamShaderPrecacheManager {
    pub enabled: bool,
    pub cache_size: u64,
    pub precached_games: Vec<String>,
}

impl SovereignSteamShaderPrecacheManager {
    pub fn new() -> Self {
        Self {
            enabled: true,
            cache_size: 1024 * 1024 * 1024, // 1GB
            precached_games: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_cache_size(&mut self, size: u64) {
        self.cache_size = size;
    }

    pub fn precache_game(&mut self, game_id: &str) {
        if !self.precached_games.contains(&game_id.to_string()) {
            self.precached_games.push(game_id.to_string());
        }
    }

    pub fn is_precached(&self, game_id: &str) -> bool {
        self.precached_games.contains(&game_id.to_string())
    }
}

impl Default for SovereignSteamShaderPrecacheManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Sovereign Omarchy Gaming Performance Suite
pub struct SovereignOmarchyGamingPerformanceSuite {
    pub governor: SovereignOmarchyGamingGovernor,
    pub hud: SovereignOmarchyHudEngine,
    pub vrr_pacing: SovereignOmarchyVrrPacingController,
    pub shader_precache: SovereignSteamShaderPrecacheManager,
}

impl SovereignOmarchyGamingPerformanceSuite {
    pub fn new() -> Self {
        Self {
            governor: SovereignOmarchyGamingGovernor::new(),
            hud: SovereignOmarchyHudEngine::new(),
            vrr_pacing: SovereignOmarchyVrrPacingController::new(),
            shader_precache: SovereignSteamShaderPrecacheManager::new(),
        }
    }

    pub fn activate_gaming_mode(&mut self) -> Result<(), &'static str> {
        self.governor.activate_gaming_mode()?;
        self.hud.set_enabled(true);
        self.vrr_pacing.enable_vrr();
        Ok(())
    }

    pub fn deactivate_gaming_mode(&mut self) -> Result<(), &'static str> {
        self.governor.deactivate_gaming_mode()?;
        self.hud.set_enabled(false);
        self.vrr_pacing.disable_vrr();
        Ok(())
    }

    pub fn update_hud_metrics(
        &self,
        fps: u32,
        cpu: u32,
        gpu: u32,
        ram: u32,
        temp: u32,
        frame_time: u32,
    ) {
        self.hud.update_fps(fps);
        self.hud.update_cpu_usage(cpu);
        self.hud.update_gpu_usage(gpu);
        self.hud.update_ram_usage(ram);
        self.hud.update_temperature(temp);
        self.hud.update_frame_time(frame_time);
    }

    pub fn calculate_optimal_refresh(&mut self) -> u32 {
        self.vrr_pacing.calculate_refresh_rate()
    }
}

impl Default for SovereignOmarchyGamingPerformanceSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Global gaming performance suite
static mut GAMING_SUITE: Option<SovereignOmarchyGamingPerformanceSuite> = None;

/// Initialize gaming performance suite
pub fn init_gaming_performance_suite() {
    unsafe {
        GAMING_SUITE = Some(SovereignOmarchyGamingPerformanceSuite::new());
    }
}

/// Get global gaming performance suite
pub fn get_gaming_performance_suite() -> Option<&'static mut SovereignOmarchyGamingPerformanceSuite>
{
    unsafe { GAMING_SUITE.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaming_governor_init() {
        let governor = SovereignOmarchyGamingGovernor::new();
        assert_eq!(governor.gaming_state, GamingState::Inactive);
        assert_eq!(governor.cpu_governor, CpuGovernor::Powersave);
    }

    #[test]
    fn test_activate_gaming_mode() {
        let mut governor = SovereignOmarchyGamingGovernor::new();
        governor.activate_gaming_mode().unwrap();
        assert_eq!(governor.gaming_state, GamingState::Active);
        assert_eq!(governor.cpu_governor, CpuGovernor::Performance);
        assert_eq!(governor.gpu_profile, GpuProfile::Performance);
    }

    #[test]
    fn test_deactivate_gaming_mode() {
        let mut governor = SovereignOmarchyGamingGovernor::new();
        governor.activate_gaming_mode().unwrap();
        governor.deactivate_gaming_mode().unwrap();
        assert_eq!(governor.gaming_state, GamingState::Inactive);
    }

    #[test]
    fn test_toggle_hud() {
        let mut governor = SovereignOmarchyGamingGovernor::new();
        governor.toggle_hud();
        assert!(governor.hud_enabled);
        governor.toggle_hud();
        assert!(!governor.hud_enabled);
    }

    #[test]
    fn test_hud_engine() {
        let hud = SovereignOmarchyHudEngine::new();
        hud.update_fps(60);
        assert_eq!(hud.get_fps(), 60);
    }

    #[test]
    fn test_vrr_pacing() {
        let mut vrr = SovereignOmarchyVrrPacingController::new();
        vrr.enable_vrr();
        vrr.set_target_fps(120);
        let refresh = vrr.calculate_refresh_rate();
        assert_eq!(refresh, 120);
    }

    #[test]
    fn test_shader_precache() {
        let mut precache = SovereignSteamShaderPrecacheManager::new();
        precache.precache_game("12345");
        assert!(precache.is_precached("12345"));
    }

    #[test]
    fn test_gaming_suite() {
        let mut suite = SovereignOmarchyGamingPerformanceSuite::new();
        suite.activate_gaming_mode().unwrap();
        assert_eq!(suite.governor.gaming_state, GamingState::Active);
    }
}
