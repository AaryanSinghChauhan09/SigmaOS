// SigmaOS Unified Text & DPI Scaling Engine
// Zero-dependency #![no_std] system-wide font/DPI scaling manager with per-app overrides & accessibility profiles

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalingProfile {
    Standard,             // 1.0x (96 DPI)
    HighDpi2K,            // 1.25x (120 DPI)
    UltraDpi4K,           // 2.0x (192 DPI)
    AccessibilityEnlarged,// 1.5x (144 DPI)
    Compact,              // 0.85x (81 DPI)
}

impl ScalingProfile {
    pub fn scale_factor(&self) -> f32 {
        match self {
            ScalingProfile::Standard => 1.0,
            ScalingProfile::HighDpi2K => 1.25,
            ScalingProfile::UltraDpi4K => 2.0,
            ScalingProfile::AccessibilityEnlarged => 1.5,
            ScalingProfile::Compact => 0.85,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppScalingOverride {
    pub app_id: String,
    pub scale_factor: f32,
    pub custom_font_family: Option<String>,
    pub min_font_size_pt: u32,
}

pub struct UnifiedTextScalingEngine {
    current_profile: ScalingProfile,
    custom_system_scale_factor: Option<f32>,
    app_overrides: Vec<AppScalingOverride>,
    high_contrast_font_bold: bool,
}

impl UnifiedTextScalingEngine {
    pub fn new() -> Self {
        Self {
            current_profile: ScalingProfile::Standard,
            custom_system_scale_factor: None,
            app_overrides: Vec::new(),
            high_contrast_font_bold: false,
        }
    }

    pub fn set_profile(&mut self, profile: ScalingProfile) {
        self.current_profile = profile;
    }

    pub fn set_custom_scale_factor(&mut self, scale_factor: f32) {
        self.custom_system_scale_factor = Some(scale_factor.max(0.5).min(4.0));
    }

    pub fn set_app_override(
        &mut self,
        app_id: &str,
        scale_factor: f32,
        custom_font_family: Option<&str>,
        min_font_size_pt: u32,
    ) {
        let entry = AppScalingOverride {
            app_id: String::from(app_id),
            scale_factor,
            custom_font_family: custom_font_family.map(String::from),
            min_font_size_pt,
        };

        if let Some(pos) = self.app_overrides.iter().position(|a| a.app_id == app_id) {
            self.app_overrides[pos] = entry;
        } else {
            self.app_overrides.push(entry);
        }
    }

    pub fn remove_app_override(&mut self, app_id: &str) -> bool {
        if let Some(pos) = self.app_overrides.iter().position(|a| a.app_id == app_id) {
            self.app_overrides.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn resolve_scale_factor(&self, app_id: Option<&str>) -> f32 {
        if let Some(id) = app_id {
            if let Some(override_entry) = self.app_overrides.iter().find(|a| a.app_id == id) {
                return override_entry.scale_factor;
            }
        }

        self.custom_system_scale_factor
            .unwrap_or_else(|| self.current_profile.scale_factor())
    }

    pub fn set_high_contrast_font_bold(&mut self, enabled: bool) {
        self.high_contrast_font_bold = enabled;
    }

    pub fn is_high_contrast_font_bold(&self) -> bool {
        self.high_contrast_font_bold
    }
}

impl Default for UnifiedTextScalingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_text_scaling_engine() {
        let mut engine = UnifiedTextScalingEngine::new();
        assert_eq!(engine.resolve_scale_factor(None), 1.0);

        engine.set_profile(ScalingProfile::UltraDpi4K);
        assert_eq!(engine.resolve_scale_factor(None), 2.0);

        engine.set_app_override("terminal_app", 1.2, Some("Fira Code"), 12);
        assert_eq!(engine.resolve_scale_factor(Some("terminal_app")), 1.2);
        assert_eq!(engine.resolve_scale_factor(Some("browser_app")), 2.0);
    }
}
