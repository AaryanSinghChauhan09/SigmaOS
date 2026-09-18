/// SigmaOS Core Customization, Gamification & Lightweight Bundle Manager
/// Zero-dependency, OOP-centric, persona-driven tool manager

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ==========================================
// 1. ZENITH DESKTOP PROFILE SWITCHER
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ZenithProfile {
    Developer,
    Gamer,
    Minimalist,
    Accessibility,
    Student,
    Compliance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerformanceProfile {
    pub cpu_frequency_cap_hz: u64,
    pub scheduler_quantum_ms: u32,
    pub enable_gpu_overclock: bool,
    pub enable_screen_reader: bool,
}

pub struct ProfileSwitcher {
    pub active_profile: ZenithProfile,
    pub perf_state: PerformanceProfile,
}

impl ProfileSwitcher {
    pub fn new() -> Self {
        Self {
            active_profile: ZenithProfile::Minimalist,
            perf_state: PerformanceProfile {
                cpu_frequency_cap_hz: 1_000_000_000, // 1 GHz
                scheduler_quantum_ms: 80,
                enable_gpu_overclock: false,
                enable_screen_reader: false,
            },
        }
    }

    /// Dynamically alters the hardware power-state and visual rendering loop profile
    pub fn switch_profile(&mut self, profile: ZenithProfile) {
        self.active_profile = profile;
        self.perf_state = match profile {
            ZenithProfile::Developer => PerformanceProfile {
                cpu_frequency_cap_hz: 3_200_000_000, // 3.2 GHz
                scheduler_quantum_ms: 20,
                enable_gpu_overclock: false,
                enable_screen_reader: false,
            },
            ZenithProfile::Gamer => PerformanceProfile {
                cpu_frequency_cap_hz: 4_200_000_000, // 4.2 GHz (Overclock active)
                scheduler_quantum_ms: 10,
                enable_gpu_overclock: true,
                enable_screen_reader: false,
            },
            ZenithProfile::Minimalist => PerformanceProfile {
                cpu_frequency_cap_hz: 800_000_000, // 800 MHz (Energy saving)
                scheduler_quantum_ms: 80,
                enable_gpu_overclock: false,
                enable_screen_reader: false,
            },
            ZenithProfile::Accessibility => PerformanceProfile {
                cpu_frequency_cap_hz: 2_000_000_000, // 2 GHz
                scheduler_quantum_ms: 40,
                enable_gpu_overclock: false,
                enable_screen_reader: true, // Screen reader voice buffers active
            },
            ZenithProfile::Student => PerformanceProfile {
                cpu_frequency_cap_hz: 1_800_000_000,
                scheduler_quantum_ms: 50,
                enable_gpu_overclock: false,
                enable_screen_reader: false,
            },
            ZenithProfile::Compliance => PerformanceProfile {
                cpu_frequency_cap_hz: 2_200_000_000,
                scheduler_quantum_ms: 30,
                enable_gpu_overclock: false,
                enable_screen_reader: false,
            },
        };
    }
}

impl Default for ProfileSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. GAMIFIED PRODUCTIVITY LAYER (XP & STREAK REGISTRY)
// ==========================================
pub struct GamifiedProductivity {
    pub total_xp: u64,
    pub level: u32,
    pub daily_streak: u32,
    pub last_task_timestamp: u64,
    pub completed_tasks_count: u32,
}

impl GamifiedProductivity {
    pub fn new() -> Self {
        Self {
            total_xp: 0,
            level: 1,
            daily_streak: 1,
            last_task_timestamp: 0,
            completed_tasks_count: 0,
        }
    }

    /// Awards XP points for productive system events and updates streaks / levels
    pub fn complete_task(&mut self, timestamp: u64, task_weight_xp: u64) {
        self.completed_tasks_count += 1;
        self.total_xp += task_weight_xp;

        // Check streak status: standard 1-day unix timestamp mapping (86400 seconds)
        if self.last_task_timestamp > 0 {
            let diff = timestamp.saturating_sub(self.last_task_timestamp);
            if diff <= 86400 {
                self.daily_streak += 1; // Streak preserved!
            } else if diff > 172800 {
                self.daily_streak = 1; // Streak broken, reset
            }
        }

        self.last_task_timestamp = timestamp;

        // Level-up scaling equation: Level = sqrt(total_xp) / 10
        let float_xp = self.total_xp as f64;
        let next_level = (sqrt_emulated(float_xp) / 10.0) as u32;
        if next_level > self.level {
            self.level = next_level;
        }
    }
}

impl Default for GamifiedProductivity {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple square root emulator
fn sqrt_emulated(val: f64) -> f64 {
    if val <= 0.0 {
        return 0.0;
    }
    let mut x = val;
    for _ in 0..10 {
        x = 0.5 * (x + val / x);
    }
    x
}

// ==========================================
// 3. GAME HUB ADAPTIVE DIFFICULTY BALANCER (AI Engine)
// ==========================================
pub struct GameDifficultyBalancer {
    pub player_actions_count: usize,
    pub total_wins_count: u32,
    pub avg_reaction_time_ms: f64,
    pub base_difficulty_multiplier: f64, // 0.0 to 1.0 (easy to hard)
}

impl GameDifficultyBalancer {
    pub fn new() -> Self {
        Self {
            player_actions_count: 0,
            total_wins_count: 0,
            avg_reaction_time_ms: 250.0,     // Default average in ms
            base_difficulty_multiplier: 0.5, // Standard Medium difficulty
        }
    }

    pub fn track_player_performance(&mut self, action_time_ms: f64, won: bool) {
        self.player_actions_count += 1;
        if won {
            self.total_wins_count += 1;
        }

        self.avg_reaction_time_ms = (self.avg_reaction_time_ms * 0.9) + (action_time_ms * 0.1);

        let win_ratio = self.total_wins_count as f64 / self.player_actions_count as f64;

        if self.avg_reaction_time_ms < 200.0 && win_ratio > 0.7 {
            self.base_difficulty_multiplier = (self.base_difficulty_multiplier + 0.1).min(1.0);
        } else if self.avg_reaction_time_ms > 350.0 || win_ratio < 0.3 {
            self.base_difficulty_multiplier = (self.base_difficulty_multiplier - 0.1).max(0.1);
        }
    }
}

impl Default for GameDifficultyBalancer {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 4. CORE OS BUNDLE & OPTIONAL PERSONA ADD-ONS
// ==========================================

#[derive(Debug, Clone)]
pub struct CoreOsToolBundle {
    pub text_editor_active: bool,     // Lightweight micro-text editor
    pub compression_util_active: bool, // Native .zip, .tar.gz, .7z support
    pub net_diagnostics_active: bool,  // ping, traceroute, curl, wget
    pub system_monitor_active: bool,   // Resource dashboard (CPU, RAM, Disk, PIDs)
    pub snapshot_rollback_active: bool,// Timeshift-lite rollback safety
}

impl CoreOsToolBundle {
    pub fn new() -> Self {
        Self {
            text_editor_active: true,
            compression_util_active: true,
            net_diagnostics_active: true,
            system_monitor_active: true,
            snapshot_rollback_active: true,
        }
    }
}

impl Default for CoreOsToolBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct OptionalPersonaAddonSuite {
    pub persona_type: ZenithProfile,
    pub tool_names: Vec<String>,
    pub is_loaded: bool,
}

pub struct LightweightBundleManager {
    pub core_bundle: CoreOsToolBundle,
    pub persona_addons: BTreeMap<ZenithProfile, OptionalPersonaAddonSuite>,
    pub active_persona: ZenithProfile,
}

impl LightweightBundleManager {
    pub fn new() -> Self {
        let mut addons = BTreeMap::new();

        addons.insert(
            ZenithProfile::Developer,
            OptionalPersonaAddonSuite {
                persona_type: ZenithProfile::Developer,
                tool_names: vec!["file_converter".to_string(), "lightweight_ide_overlay".to_string()],
                is_loaded: false,
            },
        );

        addons.insert(
            ZenithProfile::Compliance,
            OptionalPersonaAddonSuite {
                persona_type: ZenithProfile::Compliance,
                tool_names: vec!["universal_package_fetcher".to_string(), "audit_checklist_generator".to_string()],
                is_loaded: false,
            },
        );

        addons.insert(
            ZenithProfile::Student,
            OptionalPersonaAddonSuite {
                persona_type: ZenithProfile::Student,
                tool_names: vec!["pomodoro_timer".to_string(), "flashcard_quiz_overlay".to_string()],
                is_loaded: false,
            },
        );

        addons.insert(
            ZenithProfile::Gamer,
            OptionalPersonaAddonSuite {
                persona_type: ZenithProfile::Gamer,
                tool_names: vec!["gpu_scheduler_microtool".to_string(), "latency_ping_tracker".to_string()],
                is_loaded: false,
            },
        );

        Self {
            core_bundle: CoreOsToolBundle::new(),
            persona_addons: addons,
            active_persona: ZenithProfile::Minimalist,
        }
    }

    pub fn activate_persona_addons(&mut self, profile: ZenithProfile) {
        // Unload old persona add-ons
        if let Some(old_addon) = self.persona_addons.get_mut(&self.active_persona) {
            old_addon.is_loaded = false;
        }

        self.active_persona = profile;

        // Load new persona add-ons
        if let Some(new_addon) = self.persona_addons.get_mut(&profile) {
            new_addon.is_loaded = true;
        }
    }

    pub fn get_active_tools(&self) -> Vec<String> {
        let mut tools = vec![
            "text_editor".to_string(),
            "compression_utility".to_string(),
            "net_diagnostics".to_string(),
            "system_monitor".to_string(),
            "snapshot_rollback".to_string(),
        ];

        if let Some(addon) = self.persona_addons.get(&self.active_persona) {
            if addon.is_loaded {
                tools.extend(addon.tool_names.clone());
            }
        }

        tools
    }
}

impl Default for LightweightBundleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_switcher_modes() {
        let mut switcher = ProfileSwitcher::new();
        assert_eq!(switcher.active_profile, ZenithProfile::Minimalist);
        assert_eq!(switcher.perf_state.cpu_frequency_cap_hz, 1_000_000_000);

        switcher.switch_profile(ZenithProfile::Gamer);
        assert_eq!(switcher.active_profile, ZenithProfile::Gamer);
        assert_eq!(switcher.perf_state.cpu_frequency_cap_hz, 4_200_000_000);
        assert!(switcher.perf_state.enable_gpu_overclock);
    }

    #[test]
    fn test_gamified_xp_progression() {
        let mut prod = GamifiedProductivity::new();
        assert_eq!(prod.level, 1);
        assert_eq!(prod.total_xp, 0);

        prod.complete_task(100, 5000);
        assert_eq!(prod.total_xp, 5000);
        assert_eq!(prod.level, 7);
    }

    #[test]
    fn test_lightweight_bundle_manager() {
        let mut mgr = LightweightBundleManager::new();
        assert_eq!(mgr.get_active_tools().len(), 5); // 5 core tools

        mgr.activate_persona_addons(ZenithProfile::Developer);
        let active_dev_tools = mgr.get_active_tools();
        assert_eq!(active_dev_tools.len(), 7); // 5 core + 2 dev add-ons
        assert!(active_dev_tools.contains(&"file_converter".to_string()));

        mgr.activate_persona_addons(ZenithProfile::Student);
        let active_student_tools = mgr.get_active_tools();
        assert_eq!(active_student_tools.len(), 7); // 5 core + 2 student add-ons
        assert!(active_student_tools.contains(&"pomodoro_timer".to_string()));
    }
}
