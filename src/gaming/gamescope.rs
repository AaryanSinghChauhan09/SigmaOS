// SPDX-License-Identifier: MIT
// SigmaOS - Gamescope Performance Monitoring
// Inspired by Gamescope (https://github.com/ValveSoftware/gamescope)
// Provides performance monitoring and compositing for games

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Gamescope config
#[derive(Debug, Clone, Copy)]
pub struct GamescopeConfig {
    pub frame_limit: u32,
    pub fps_limit: u32,
    pub borderless: bool,
    pub fullscreen: bool,
    pub force_resize: bool,
    pub output_width: u32,
    pub output_height: u32,
}

impl Default for GamescopeConfig {
    fn default() -> Self {
        Self {
            frame_limit: 0, // No limit
            fps_limit: 0,   // No limit
            borderless: false,
            fullscreen: true,
            force_resize: false,
            output_width: 1920,
            output_height: 1080,
        }
    }
}

/// Performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub fps: f32,
    pub frame_time: f32, // milliseconds
    pub cpu_usage: f32,  // percentage
    pub gpu_usage: f32,  // percentage
    pub vram_usage: u32, // MB
    pub ram_usage: u32,  // MB
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            fps: 0.0,
            frame_time: 0.0,
            cpu_usage: 0.0,
            gpu_usage: 0.0,
            vram_usage: 0,
            ram_usage: 0,
        }
    }

    /// Update FPS
    pub fn update_fps(&mut self, fps: f32) {
        self.fps = fps;
    }

    /// Update frame time
    pub fn update_frame_time(&mut self, frame_time: f32) {
        self.frame_time = frame_time;
    }

    /// Update CPU usage
    pub fn update_cpu_usage(&mut self, cpu: f32) {
        self.cpu_usage = cpu;
    }

    /// Update GPU usage
    pub fn update_gpu_usage(&mut self, gpu: f32) {
        self.gpu_usage = gpu;
    }

    /// Get FPS
    pub fn get_fps(&self) -> f32 {
        self.fps
    }

    /// Get frame time
    pub fn get_frame_time(&self) -> f32 {
        self.frame_time
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Gamescope instance
pub struct GamescopeInstance {
    pub config: GamescopeConfig,
    pub metrics: PerformanceMetrics,
    pub active: AtomicBool,
    pub game_pid: Option<u32>,
}

impl GamescopeInstance {
    pub fn new() -> Self {
        Self {
            config: GamescopeConfig::default(),
            metrics: PerformanceMetrics::new(),
            active: AtomicBool::new(false),
            game_pid: None,
        }
    }

    /// Initialize Gamescope
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Launch game with Gamescope
    pub fn launch_game(&mut self, command: String) -> Result<(), &'static str> {
        // In real implementation, would run:
        // gamescope -- {command}

        self.game_pid = Some(1234); // Simulated PID
        Ok(())
    }

    /// Stop game
    pub fn stop_game(&mut self) -> Result<(), &'static str> {
        self.game_pid = None;
        Ok(())
    }

    /// Set config
    pub fn set_config(&mut self, config: GamescopeConfig) {
        self.config = config;
    }

    /// Get config
    pub fn get_config(&self) -> GamescopeConfig {
        self.config
    }

    /// Get metrics
    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for GamescopeInstance {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Gamescope instance
static mut GAMESCOPE_INSTANCE: Option<GamescopeInstance> = None;

/// Initialize Gamescope
pub fn init_gamescope() {
    unsafe {
        GAMESCOPE_INSTANCE = Some(GamescopeInstance::new());
    }
}

/// Get global Gamescope instance
pub fn get_gamescope() -> Option<&'static mut GamescopeInstance> {
    unsafe { GAMESCOPE_INSTANCE.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamescope_init() {
        let mut gamescope = GamescopeInstance::new();
        gamescope.init();
        assert!(gamescope.is_active());
    }

    #[test]
    fn test_performance_metrics() {
        let metrics = PerformanceMetrics::new();
        metrics.update_fps(60.0);
        assert_eq!(metrics.get_fps(), 60.0);
    }

    #[test]
    fn test_update_frame_time() {
        let metrics = PerformanceMetrics::new();
        metrics.update_frame_time(16.67);
        assert_eq!(metrics.get_frame_time(), 16.67);
    }

    #[test]
    fn test_gamescope_config() {
        let config = GamescopeConfig::default();
        assert_eq!(config.fps_limit, 0);
        assert!(config.fullscreen);
    }
}
