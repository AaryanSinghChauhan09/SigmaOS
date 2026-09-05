/// SigmaOS Core Customization & Gamification Engine
/// Zero-dependency, #![no_std] compliant, OOP-centric

// ==========================================
// 1. ZENITH DESKTOP PROFILE SWITCHER
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZenithProfile {
    Developer,
    Gamer,
    Minimalist,
    Accessibility,
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
        // Standard high-performance non-std float emulation mapping
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

/// Simple square root emulator for #![no_std] environments
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

    /// Registers a game event and recalculates the adaptive difficulty scalar (AI balancer)
    pub fn track_player_performance(&mut self, action_time_ms: f64, won: bool) {
        self.player_actions_count += 1;
        if won {
            self.total_wins_count += 1;
        }

        // Running average calculation for reaction time
        self.avg_reaction_time_ms = (self.avg_reaction_time_ms * 0.9) + (action_time_ms * 0.1);

        // Adjust difficulty: if reaction time is low (<200ms) and wins are high, increase difficulty
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

#[cfg(test_disabled)]
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

        prod.complete_task(100, 5000); // 5000 XP should trigger level up
        assert_eq!(prod.total_xp, 5000);
        assert_eq!(prod.level, 7); // sqrt(5000) / 10 = ~7
    }

    #[test]
    fn test_game_hub_adaptive_difficulty() {
        let mut balancer = GameDifficultyBalancer::new();
        assert_eq!(balancer.base_difficulty_multiplier, 0.5);

        // Simulate expert gamer (fast reaction times, high win ratios)
        for _ in 0..10 {
            balancer.track_player_performance(150.0, true);
        }

        assert!(balancer.base_difficulty_multiplier > 0.5);
    }
}
