// SPDX-License-Identifier: MIT
// SigmaOS - Omarchy Gaming Performance Suite
// Inspired by Omarchy (github.com/omacom/omarchy) gaming-focused Arch Linux configuration
// Provides automated CPU/GPU governor switching, in-game HUD telemetry, and developer stack management

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// GPU backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuBackend {
    NvidiaProprietary,
    NvidiaNouveau,
    AmdGpu,
    IntelXe,
}

/// Gaming performance profile
#[derive(Debug, Clone)]
pub struct GamingProfile {
    pub name: String,
    pub cpu_governor: String,
    pub gpu_boost_percent: u32,
    pub amdgpu_overdrive: bool,
    pub compositor_bypass: bool,
}

impl Default for GamingProfile {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            cpu_governor: "performance".to_string(),
            gpu_boost_percent: 20,
            amdgpu_overdrive: true,
            compositor_bypass: true,
        }
    }
}

/// Game trigger rules
#[derive(Debug, Clone)]
pub struct GameTrigger {
    pub name_contains: Vec<String>,
    pub category: String,
    pub exe_suffix: Vec<String>,
}

/// Sovereign Omarchy Gaming Governor
pub struct SovereignOmarchyGamingGovernor {
    pub enabled: bool,
    pub backend: GpuBackend,
    pub current_profile: GamingProfile,
    pub triggers: Vec<GameTrigger>,
    pub is_gaming_mode: bool,
    pub previous_cpu_governor: Option<String>,
}

impl SovereignOmarchyGamingGovernor {
    pub fn new() -> Self {
        Self {
            enabled: true,
            backend: GpuBackend::AmdGpu,
            current_profile: GamingProfile::default(),
            triggers: vec![
                GameTrigger {
                    name_contains: vec!["steam".to_string(), "wine".to_string(), "proton".to_string(), "lutris".to_string()],
                    category: "Game".to_string(),
                    exe_suffix: vec![".exe".to_string()],
                },
            ],
            is_gaming_mode: false,
            previous_cpu_governor: None,
        }
    }

    /// Detect game launch and activate gaming mode
    pub fn detect_game_launch(&mut self, process_name: &str) -> bool {
        if !self.enabled {
            return false;
        }

        for trigger in &self.triggers {
            if trigger.name_contains.iter().any(|name| process_name.contains(name)) {
                self.activate_gaming_mode();
                return true;
            }
        }
        false
    }

    /// Activate gaming performance mode
    pub fn activate_gaming_mode(&mut self) {
        if self.is_gaming_mode {
            return;
        }

        // Store previous CPU governor
        self.previous_cpu_governor = Some("schedutil".to_string());

        // Switch to performance governor
        self.set_cpu_governor(&self.current_profile.cpu_governor);

        // Boost GPU
        self.boost_gpu(self.current_profile.gpu_boost_percent);

        // Enable AMDGPU overdrive if applicable
        if self.backend == GpuBackend::AmdGpu && self.current_profile.amdgpu_overdrive {
            self.enable_amdgpu_overdrive();
        }

        // Bypass compositor for direct scanout
        if self.current_profile.compositor_bypass {
            self.set_compositor_bypass(true);
        }

        self.is_gaming_mode = true;
    }

    /// Restore normal performance mode
    pub fn restore_normal_mode(&mut self) {
        if !self.is_gaming_mode {
            return;
        }

        // Restore CPU governor
        if let Some(prev_governor) = &self.previous_cpu_governor {
            self.set_cpu_governor(prev_governor);
        }

        // Reset GPU boost
        self.reset_gpu_boost();

        // Disable compositor bypass
        self.set_compositor_bypass(false);

        self.is_gaming_mode = false;
    }

    fn set_cpu_governor(&self, governor: &str) {
        // In real implementation, this would write to /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
    }

    fn boost_gpu(&self, percent: u32) {
        match self.backend {
            GpuBackend::NvidiaProprietary => {
                // nvidia-smi -pl <limit>
            }
            GpuBackend::AmdGpu => {
                // /sys/class/drm/card0/device/pp_od_enable_voltage
            }
            GpuBackend::IntelXe => {
                // sysfs frequency scaling
            }
            GpuBackend::NvidiaNouveau => {
                // Governor only
            }
        }
    }

    fn reset_gpu_boost(&self) {
        // Reset GPU to default power limits
    }

    fn enable_amdgpu_overdrive(&self) {
        // Enable AMDGPU overdrive for extra headroom
    }

    fn set_compositor_bypass(&self, bypass: bool) {
        // Configure compositor for direct scanout in fullscreen
    }
}

impl Default for SovereignOmarchyGamingGovernor {
    fn default() -> Self {
        Self::new()
    }
}

/// HUD metrics snapshot
#[derive(Debug, Clone)]
pub struct HudMetrics {
    pub fps: f32,
    pub frame_time_ms: f32,
    pub frame_time_1_low_ms: f32,
    pub frame_time_0_1_low_ms: f32,
    pub gpu_temp: f32,
    pub gpu_utilization: f32,
    pub vram_usage_mb: u32,
    pub cpu_temp: f32,
    pub cpu_usage_per_core: Vec<f32>,
    pub ram_usage_mb: u32,
    pub network_latency_ms: f32,
}

impl Default for HudMetrics {
    fn default() -> Self {
        Self {
            fps: 0.0,
            frame_time_ms: 0.0,
            frame_time_1_low_ms: 0.0,
            frame_time_0_1_low_ms: 0.0,
            gpu_temp: 0.0,
            gpu_utilization: 0.0,
            vram_usage_mb: 0,
            cpu_temp: 0.0,
            cpu_usage_per_core: Vec::new(),
            ram_usage_mb: 0,
            network_latency_ms: 0.0,
        }
    }
}

/// Sovereign Omarchy HUD Engine
pub struct SovereignOmarchyHudEngine {
    pub enabled: bool,
    pub metrics: HudMetrics,
    pub position: String,
    pub font_size: u32,
    pub is_visible: bool,
}

impl SovereignOmarchyHudEngine {
    pub fn new() -> Self {
        Self {
            enabled: true,
            metrics: HudMetrics::default(),
            position: "top-left".to_string(),
            font_size: 24,
            is_visible: false,
        }
    }

    /// Update HUD metrics from system
    pub fn update_metrics(&mut self) {
        // In real implementation, this would read from:
        // - /sys/class/drm/card0/gt0/gt_cur_freq_mhz
        // - /sys/class/hwmon/hwmon*/temp1_input
        // - /proc/meminfo
        // - Frame timing from compositor
    }

    /// Toggle HUD visibility
    pub fn toggle(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Cycle HUD position
    pub fn cycle_position(&mut self) {
        let positions = vec!["top-left", "top-right", "bottom-left", "bottom-right", "center"];
        if let Some(idx) = positions.iter().position(|p| p == &self.position) {
            self.position = positions[(idx + 1) % positions.len()].to_string();
        }
    }

    /// Render HUD as overlay
    pub fn render(&self) -> String {
        if !self.is_visible {
            return String::new();
        }

        format!(
            "FPS: {:.1}\n\
             Frame Time: {:.2}ms\n\
             1% Low: {:.2}ms\n\
             0.1% Low: {:.2}ms\n\
             GPU: {:.0}°C {:.0}%\n\
             VRAM: {}MB\n\
             CPU: {:.0}°C\n\
             RAM: {}MB",
            self.metrics.fps,
            self.metrics.frame_time_ms,
            self.metrics.frame_time_1_low_ms,
            self.metrics.frame_time_0_1_low_ms,
            self.metrics.gpu_temp,
            self.metrics.gpu_utilization,
            self.metrics.vram_usage_mb,
            self.metrics.cpu_temp,
            self.metrics.ram_usage_mb
        )
    }
}

impl Default for SovereignOmarchyHudEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Developer stack definition
#[derive(Debug, Clone)]
pub struct DevStack {
    pub name: String,
    pub tools: Vec<String>,
    pub version_manager: String,
}

/// Sovereign Omarchy Developer Stacks
pub struct SovereignOmarchyDeveloperStacks {
    pub stacks: BTreeMap<String, DevStack>,
}

impl SovereignOmarchyDeveloperStacks {
    pub fn new() -> Self {
        let mut stacks = BTreeMap::new();

        stacks.insert(
            "rust".to_string(),
            DevStack {
                name: "rust".to_string(),
                tools: vec![
                    "rustup".to_string(),
                    "cargo".to_string(),
                    "rust-analyzer".to_string(),
                    "bacon".to_string(),
                ],
                version_manager: "rustup".to_string(),
            },
        );

        stacks.insert(
            "go".to_string(),
            DevStack {
                name: "go".to_string(),
                tools: vec!["go".to_string(), "gopls".to_string(), "golangci-lint".to_string()],
                version_manager: "direct".to_string(),
            },
        );

        stacks.insert(
            "node".to_string(),
            DevStack {
                name: "node".to_string(),
                tools: vec![
                    "node".to_string(),
                    "npm".to_string(),
                    "pnpm".to_string(),
                    "typescript".to_string(),
                    "prettier".to_string(),
                ],
                version_manager: "volta".to_string(),
            },
        );

        stacks.insert(
            "python".to_string(),
            DevStack {
                name: "python".to_string(),
                tools: vec!["python3".to_string(), "uv".to_string(), "ruff".to_string(), "pyright".to_string()],
                version_manager: "uv".to_string(),
            },
        );

        stacks.insert(
            "zig".to_string(),
            DevStack {
                name: "zig".to_string(),
                tools: vec!["zig".to_string(), "zls".to_string()],
                version_manager: "zigup".to_string(),
            },
        );

        stacks.insert(
            "nim".to_string(),
            DevStack {
                name: "nim".to_string(),
                tools: vec!["nim".to_string(), "nimble".to_string(), "nimlsp".to_string()],
                version_manager: "choosenim".to_string(),
            },
        );

        Self { stacks }
    }

    /// Install a development stack
    pub fn install_stack(&self, stack_name: &str) -> Result<(), String> {
        if let Some(stack) = self.stacks.get(stack_name) {
            // In real implementation, this would:
            // 1. Install tools to ~/.sigma/stacks/<name>/
            // 2. Inject PATH via ~/.config/sigma/env.d/
            // 3. Configure shell integration
            Ok(())
        } else {
            Err(format!("Stack '{}' not found", stack_name))
        }
    }

    /// List available stacks
    pub fn list_stacks(&self) -> Vec<String> {
        self.stacks.keys().cloned().collect()
    }
}

impl Default for SovereignOmarchyDeveloperStacks {
    fn default() -> Self {
        Self::new()
    }
}

/// VRR strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrrStrategy {
    Adaptive,
    Fixed(u32),
    CapToRefresh,
    LatencyFavor,
}

/// VRR event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VrrEvent {
    FrameDropped,
    PaceAdjusted,
    RangeEnforced,
}

/// Sovereign Omarchy VRR Pacing Controller
pub struct SovereignOmarchyVrrPacingController {
    pub strategy: VrrStrategy,
    pub min_hz: u32,
    pub max_hz: u32,
    pub allow_freesync: bool,
    pub allow_gsync: bool,
    pub events: Vec<VrrEvent>,
}

impl SovereignOmarchyVrrPacingController {
    pub fn new() -> Self {
        Self {
            strategy: VrrStrategy::Adaptive,
            min_hz: 48,
            max_hz: 165,
            allow_freesync: true,
            allow_gsync: true,
            events: Vec::new(),
        }
    }

    /// Detect VRR range from EDID
    pub fn detect_vrr_range(&mut self) -> (u32, u32) {
        // In real implementation, this would read EDID via DRM
        (self.min_hz, self.max_hz)
    }

    /// Adjust frame pacing for VRR
    pub fn adjust_pacing(&mut self, frame_delta_ms: f32) {
        match self.strategy {
            VrrStrategy::Adaptive => {
                // Adaptive pacing based on frame delta
            }
            VrrStrategy::Fixed(hz) => {
                // Fixed refresh rate
            }
            VrrStrategy::CapToRefresh => {
                // Cap to refresh rate
            }
            VrrStrategy::LatencyFavor => {
                // Favor lowest latency
            }
        }
    }
}

impl Default for SovereignOmarchyVrrPacingController {
    fn default() -> Self {
        Self::new()
    }
}

/// Shader cache entry
#[derive(Debug, Clone)]
pub struct ShaderCacheEntry {
    pub app_id: u32,
    pub driver_version: String,
    pub compile_timestamp: u64,
    pub is_valid: bool,
}

/// Sovereign Steam Shader Precache Manager
pub struct SovereignSteamShaderPrecacheManager {
    pub cache_entries: Vec<ShaderCacheEntry>,
    pub shader_cache_path: String,
}

impl SovereignSteamShaderPrecacheManager {
    pub fn new() -> Self {
        Self {
            cache_entries: Vec::new(),
            shader_cache_path: "~/.local/share/Steam/steamapps/shadercache/".to_string(),
        }
    }

    /// Scan for shader cache entries
    pub fn scan_cache(&mut self) {
        // In real implementation, this would scan the shader cache directory
    }

    /// Validate cache against current driver version
    pub fn validate_cache(&mut self, app_id: u32, current_driver_version: &str) -> bool {
        if let Some(entry) = self.cache_entries.iter_mut().find(|e| e.app_id == app_id) {
            entry.is_valid = entry.driver_version == current_driver_version;
            entry.is_valid
        } else {
            false
        }
    }

    /// Precache shaders for a game
    pub fn precache(&self, app_id: u32) -> Result<(), String> {
        // Trigger shader pre-compilation via glslc / RADV
        Ok(())
    }

    /// Purge stale cache entries
    pub fn purge_stale(&mut self) {
        self.cache_entries.retain(|e| e.is_valid);
    }
}

impl Default for SovereignSteamShaderPrecacheManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Master Omarchy Gaming Performance Suite
pub struct SovereignOmarchyGamingPerformanceSuite {
    pub governor: SovereignOmarchyGamingGovernor,
    pub hud: SovereignOmarchyHudEngine,
    pub dev_stacks: SovereignOmarchyDeveloperStacks,
    pub vrr_controller: SovereignOmarchyVrrPacingController,
    pub shader_manager: SovereignSteamShaderPrecacheManager,
}

impl SovereignOmarchyGamingPerformanceSuite {
    pub fn new() -> Self {
        Self {
            governor: SovereignOmarchyGamingGovernor::new(),
            hud: SovereignOmarchyHudEngine::new(),
            dev_stacks: SovereignOmarchyDeveloperStacks::new(),
            vrr_controller: SovereignOmarchyVrrPacingController::new(),
            shader_manager: SovereignSteamShaderPrecacheManager::new(),
        }
    }

    /// Initialize the gaming suite
    pub fn initialize(&mut self) {
        self.vrr_controller.detect_vrr_range();
        self.shader_manager.scan_cache();
    }

    /// Handle game launch
    pub fn on_game_launch(&mut self, process_name: &str) {
        if self.governor.detect_game_launch(process_name) {
            self.hud.is_visible = true;
            self.shader_manager.precache(440).ok(); // Example AppID
        }
    }

    /// Handle game exit
    pub fn on_game_exit(&mut self) {
        self.governor.restore_normal_mode();
        self.hud.is_visible = false;
    }
}

impl Default for SovereignOmarchyGamingPerformanceSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaming_governor_activation() {
        let mut governor = SovereignOmarchyGamingGovernor::new();
        assert!(!governor.is_gaming_mode);

        governor.detect_game_launch("steam");
        assert!(governor.is_gaming_mode);

        governor.restore_normal_mode();
        assert!(!governor.is_gaming_mode);
    }

    #[test]
    fn test_hud_toggle() {
        let mut hud = SovereignOmarchyHudEngine::new();
        assert!(!hud.is_visible);

        hud.toggle();
        assert!(hud.is_visible);

        hud.toggle();
        assert!(!hud.is_visible);
    }

    #[test]
    fn test_hud_position_cycle() {
        let mut hud = SovereignOmarchyHudEngine::new();
        assert_eq!(hud.position, "top-left");

        hud.cycle_position();
        assert_eq!(hud.position, "top-right");

        hud.cycle_position();
        assert_eq!(hud.position, "bottom-left");
    }

    #[test]
    fn test_dev_stacks() {
        let stacks = SovereignOmarchyDeveloperStacks::new();
        assert!(stacks.install_stack("rust").is_ok());
        assert!(stacks.install_stack("invalid").is_err());

        let list = stacks.list_stacks();
        assert!(list.contains(&"rust".to_string()));
        assert!(list.contains(&"go".to_string()));
    }

    #[test]
    fn test_vrr_controller() {
        let mut vrr = SovereignOmarchyVrrPacingController::new();
        let (min, max) = vrr.detect_vrr_range();
        assert_eq!(min, 48);
        assert_eq!(max, 165);
    }

    #[test]
    fn test_shader_manager() {
        let mut manager = SovereignSteamShaderPrecacheManager::new();
        manager.scan_cache();
        assert!(manager.validate_cache(440, "1.0.0"));
        manager.purge_stale();
    }

    #[test]
    fn test_gaming_suite_integration() {
        let mut suite = SovereignOmarchyGamingPerformanceSuite::new();
        suite.initialize();

        suite.on_game_launch("steam");
        assert!(suite.governor.is_gaming_mode);
        assert!(suite.hud.is_visible);

        suite.on_game_exit();
        assert!(!suite.governor.is_gaming_mode);
        assert!(!suite.hud.is_visible);
    }
}
