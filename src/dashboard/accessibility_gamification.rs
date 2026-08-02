#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS UI/UX Accessibility Overlay & Gamification Engine
// Implements accessibility presets (high contrast, color filters) and gamified task/achievement tracking to boost user engagement and operational compliance.

use std::collections::HashMap;

/// Accessibility color filters for users with visual impairments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFilter {
    None,
    Grayscale,
    Protanopia,   // Red-green color blindness
    Deuteranopia, // Green-red color blindness
    Tritanopia,   // Blue-yellow color blindness
}

/// Adaptive accessibility overlay settings
#[derive(Debug, Clone)]
pub struct AccessibilityOverlay {
    pub high_contrast: bool,
    pub color_filter: ColorFilter,
    pub voice_cue_enabled: bool,
    pub large_text_percentage: u32, // e.g., 100 for normal, 150 for large, 200 for extra-large
}

impl Default for AccessibilityOverlay {
    fn default() -> Self {
        Self {
            high_contrast: false,
            color_filter: ColorFilter::None,
            voice_cue_enabled: false,
            large_text_percentage: 100,
        }
    }
}

impl AccessibilityOverlay {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets high contrast mode and updates the active multiplier
    pub fn set_high_contrast(&mut self, enabled: bool) {
        self.high_contrast = enabled;
    }

    /// Updates color filter setting
    pub fn set_color_filter(&mut self, filter: ColorFilter) {
        self.color_filter = filter;
    }

    /// Toggles screen reader voice cues
    pub fn toggle_voice_cues(&mut self) {
        self.voice_cue_enabled = !self.voice_cue_enabled;
    }

    /// Modifies font scaling
    pub fn set_text_scale(&mut self, percentage: u32) {
        self.large_text_percentage = percentage;
    }

    /// Simulates color transformation of an RGB pixel based on current filter
    pub fn apply_filter_to_pixel(&self, r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        match self.color_filter {
            ColorFilter::None => (r, g, b),
            ColorFilter::Grayscale => {
                let gray = ((r as u32 + g as u32 + b as u32) / 3) as u8;
                (gray, gray, gray)
            }
            ColorFilter::Protanopia => {
                // Simplified simulation: reduce red component and shift towards blue/green
                let new_r = ((r as f32 * 0.567) + (g as f32 * 0.433)) as u8;
                let new_g = ((r as f32 * 0.558) + (g as f32 * 0.442)) as u8;
                let new_b = ((g as f32 * 0.242) + (b as f32 * 0.758)) as u8;
                (new_r, new_g, new_b)
            }
            ColorFilter::Deuteranopia => {
                // Simplified simulation
                let new_r = ((r as f32 * 0.625) + (g as f32 * 0.375)) as u8;
                let new_g = ((r as f32 * 0.7) + (g as f32 * 0.3)) as u8;
                let new_b = b;
                (new_r, new_g, new_b)
            }
            ColorFilter::Tritanopia => {
                // Simplified simulation
                let new_r = r;
                let new_g = ((g as f32 * 0.7) + (b as f32 * 0.3)) as u8;
                let new_b = ((g as f32 * 0.475) + (b as f32 * 0.525)) as u8;
                (new_r, new_g, new_b)
            }
        }
    }
}

// =========================================================================
// GAMIFIED PRODUCTIVITY TRACKER
// =========================================================================

/// Achievement or compliance badge unlocked by the user
#[derive(Debug, Clone)]
pub struct Trophy {
    pub id: String,
    pub title: String,
    pub description: String,
    pub points_awarded: u32,
    pub is_unlocked: bool,
}

pub struct GamifiedProductivityTracker {
    pub trophies: HashMap<String, Trophy>,
    pub xp_points: u32,
    pub daily_streak: u32,
    pub consecutive_audits: u32,
}

impl GamifiedProductivityTracker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut tracker = Self {
            trophies: HashMap::new(),
            xp_points: 0,
            daily_streak: 0,
            consecutive_audits: 0,
        };
        tracker.register_default_trophies();
        tracker
    }

    fn register_default_trophies(&mut self) {
        self.register_trophy(
            "COMPLY_GDPR".to_string(),
            "GDPR Sentinel".to_string(),
            "Achieved 100% GDPR compliance score on all data nodes.".to_string(),
            100,
        );
        self.register_trophy(
            "PAYROLL_OK".to_string(),
            "Statutory Specialist".to_string(),
            "Ran labour law payroll and certified EPF/ESI contributions on time.".to_string(),
            150,
        );
        self.register_trophy(
            "ACCESSIBLE_OS".to_string(),
            "Inclusion Ambassador".to_string(),
            "Activated accessibility presets to personalize visual workspace.".to_string(),
            50,
        );
        self.register_trophy(
            "AUDIT_STREAK".to_string(),
            "Compliance Streak".to_string(),
            "Maintained a 7-day streak of perfect security & compliance checks.".to_string(),
            200,
        );
    }

    pub fn register_trophy(&mut self, id: String, title: String, description: String, points: u32) {
        self.trophies.insert(
            id.clone(),
            Trophy {
                id,
                title,
                description,
                points_awarded: points,
                is_unlocked: false,
            },
        );
    }

    /// Unlock a trophy by ID and award corresponding XP points
    pub fn unlock_trophy(&mut self, id: &str) -> bool {
        if let Some(trophy) = self.trophies.get_mut(id) {
            if !trophy.is_unlocked {
                trophy.is_unlocked = true;
                self.xp_points += trophy.points_awarded;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Record a successfully completed audit or task, updating streaks and XP
    pub fn record_successful_action(&mut self) {
        self.consecutive_audits += 1;
        self.xp_points += 10; // 10 XP per task

        // Check streak achievements
        if self.consecutive_audits >= 7 {
            self.unlock_trophy("AUDIT_STREAK");
        }
    }

    /// Increments daily usage streak
    pub fn increment_streak(&mut self) {
        self.daily_streak += 1;
        self.xp_points += self.daily_streak * 5; // bonus XP proportional to streak!
    }

    pub fn get_unlocked_trophies(&self) -> Vec<Trophy> {
        self.trophies
            .values()
            .filter(|t| t.is_unlocked)
            .cloned()
            .collect()
    }

    /// Computes user's gamified compliance level based on XP
    pub fn get_level(&self) -> u32 {
        (self.xp_points / 100) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_filter_transformations() {
        let overlay = AccessibilityOverlay {
            high_contrast: true,
            color_filter: ColorFilter::Grayscale,
            voice_cue_enabled: false,
            large_text_percentage: 120,
        };

        let pixel = overlay.apply_filter_to_pixel(100, 150, 200);
        assert_eq!(pixel, (150, 150, 150)); // Grayscale conversion is average
    }

    #[test]
    fn test_gamified_productivity() {
        let mut tracker = GamifiedProductivityTracker::new();
        assert_eq!(tracker.xp_points, 0);
        assert_eq!(tracker.get_level(), 1);

        // Unlock statutory specialist
        let unlocked = tracker.unlock_trophy("PAYROLL_OK");
        assert!(unlocked);
        assert_eq!(tracker.xp_points, 150);
        assert_eq!(tracker.get_level(), 2);

        // Fail to unlock twice
        let unlocked_again = tracker.unlock_trophy("PAYROLL_OK");
        assert!(!unlocked_again);
        assert_eq!(tracker.xp_points, 150);

        // Record some successful compliance actions
        for _ in 0..7 {
            tracker.record_successful_action();
        }

        // Streak badge must be unlocked
        let unlocked_trophies = tracker.get_unlocked_trophies();
        assert!(unlocked_trophies.iter().any(|t| t.id == "AUDIT_STREAK"));
    }
}
