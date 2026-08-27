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

// SigmaOS Accessibility Framework
// Advanced vision, hearing, mobility, and cognitive support

use crate::klib::BTreeMap;

/// Accessibility category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessibilityCategory {
    Vision,
    Hearing,
    Mobility,
    Cognitive,
    Neurodiversity,
}

/// Accessibility feature type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccessibilityFeature {
    ScreenReader,
    HighContrast,
    TextToSpeech,
    SpeechToText,
    Magnifier,
    ColorBlindMode,
    ReducedMotion,
    KeyboardNavigation,
    VoiceControl,
    DyslexiaFont,
}

/// Accessibility setting
#[derive(Debug, Clone)]
pub struct AccessibilitySetting {
    pub feature: AccessibilityFeature,
    pub enabled: bool,
    pub intensity: f64, // 0.0 to 1.0
    pub custom_params: BTreeMap<String, String>,
}

impl AccessibilitySetting {
    pub fn new(feature: AccessibilityFeature) -> Self {
        Self {
            feature,
            enabled: false,
            intensity: 0.5,
            custom_params: BTreeMap::new(),
        }
    }

    pub fn with_intensity(mut self, intensity: f64) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }

    pub fn with_param(mut self, key: String, value: String) -> Self {
        self.custom_params.insert(key, value);
        self
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

/// Accessibility profile
#[derive(Debug, Clone)]
pub struct AccessibilityProfile {
    pub name: String,
    pub category: AccessibilityCategory,
    pub settings: Vec<AccessibilitySetting>,
}

impl AccessibilityProfile {
    pub fn new(name: String, category: AccessibilityCategory) -> Self {
        Self {
            name,
            category,
            settings: Vec::new(),
        }
    }

    pub fn add_setting(mut self, setting: AccessibilitySetting) -> Self {
        self.settings.push(setting);
        self
    }

    pub fn get_setting(&self, feature: AccessibilityFeature) -> Option<&AccessibilitySetting> {
        self.settings.iter().find(|s| s.feature == feature)
    }

    pub fn enable_all(&mut self) {
        for setting in &mut self.settings {
            setting.enable();
        }
    }

    pub fn disable_all(&mut self) {
        for setting in &mut self.settings {
            setting.disable();
        }
    }
}

/// Accessibility framework
pub struct AccessibilityFramework {
    pub profiles: BTreeMap<String, AccessibilityProfile>,
    pub active_profile: Option<String>,
    pub global_settings: BTreeMap<AccessibilityFeature, AccessibilitySetting>,
}

impl AccessibilityFramework {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut framework = Self {
            profiles: BTreeMap::new(),
            active_profile: None,
            global_settings: BTreeMap::new(),
        };

        // Add default profiles
        framework.add_default_profiles();
        framework
    }

    fn add_default_profiles(&mut self) {
        // Vision impaired profile
        let mut vision_profile =
            AccessibilityProfile::new("Vision Impaired".to_string(), AccessibilityCategory::Vision)
                .add_setting(
                    AccessibilitySetting::new(AccessibilityFeature::ScreenReader)
                        .with_intensity(0.8),
                )
                .add_setting(
                    AccessibilitySetting::new(AccessibilityFeature::HighContrast)
                        .with_intensity(1.0),
                )
                .add_setting(
                    AccessibilitySetting::new(AccessibilityFeature::Magnifier).with_intensity(0.6),
                );
        vision_profile.enable_all();

        // Hearing impaired profile
        let mut hearing_profile = AccessibilityProfile::new(
            "Hearing Impaired".to_string(),
            AccessibilityCategory::Hearing,
        )
        .add_setting(
            AccessibilitySetting::new(AccessibilityFeature::TextToSpeech).with_intensity(0.9),
        )
        .add_setting(
            AccessibilitySetting::new(AccessibilityFeature::SpeechToText).with_intensity(0.8),
        );
        hearing_profile.enable_all();

        // Mobility impaired profile
        let mut mobility_profile = AccessibilityProfile::new(
            "Mobility Impaired".to_string(),
            AccessibilityCategory::Mobility,
        )
        .add_setting(
            AccessibilitySetting::new(AccessibilityFeature::KeyboardNavigation).with_intensity(1.0),
        )
        .add_setting(
            AccessibilitySetting::new(AccessibilityFeature::VoiceControl).with_intensity(0.7),
        )
        .add_setting(
            AccessibilitySetting::new(AccessibilityFeature::ReducedMotion).with_intensity(0.5),
        );
        mobility_profile.enable_all();

        // Cognitive support profile
        let mut cognitive_profile = AccessibilityProfile::new(
            "Cognitive Support".to_string(),
            AccessibilityCategory::Cognitive,
        )
        .add_setting(
            AccessibilitySetting::new(AccessibilityFeature::ReducedMotion).with_intensity(0.8),
        )
        .add_setting(
            AccessibilitySetting::new(AccessibilityFeature::DyslexiaFont).with_intensity(0.6),
        );
        cognitive_profile.enable_all();

        self.profiles
            .insert(vision_profile.name.clone(), vision_profile);
        self.profiles
            .insert(hearing_profile.name.clone(), hearing_profile);
        self.profiles
            .insert(mobility_profile.name.clone(), mobility_profile);
        self.profiles
            .insert(cognitive_profile.name.clone(), cognitive_profile);
    }

    pub fn add_profile(&mut self, profile: AccessibilityProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn activate_profile(&mut self, name: &str) -> Result<(), AccessibilityError> {
        if !self.profiles.contains_key(name) {
            return Err(AccessibilityError::ProfileNotFound);
        }
        self.active_profile = Some(name.to_string());
        if let Some(profile) = self.profiles.get_mut(name) {
            let profile: &mut AccessibilityProfile = profile;
            profile.enable_all();
        }
        Ok(())
    }

    pub fn get_active_profile(&self) -> Option<&AccessibilityProfile> {
        self.active_profile
            .as_ref()
            .and_then(|name| self.profiles.get(name))
    }

    pub fn get_profile(&self, name: &str) -> Option<&AccessibilityProfile> {
        self.profiles.get(&name.to_string())
    }

    pub fn list_profiles(&self) -> Vec<&AccessibilityProfile> {
        self.profiles.values().collect()
    }

    pub fn set_global_setting(&mut self, setting: AccessibilitySetting) {
        self.global_settings.insert(setting.feature, setting);
    }

    pub fn get_global_setting(
        &self,
        feature: AccessibilityFeature,
    ) -> Option<&AccessibilitySetting> {
        self.global_settings.get(&feature)
    }

    pub fn is_feature_enabled(&self, feature: AccessibilityFeature) -> bool {
        // Check active profile first
        if let Some(profile) = self.get_active_profile() {
            if let Some(setting) = profile.get_setting(feature) {
                return setting.enabled;
            }
        }

        // Fall back to global settings
        if let Some(setting) = self.get_global_setting(feature) {
            return setting.enabled;
        }

        false
    }
}

impl Default for AccessibilityFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// Accessibility errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessibilityError {
    ProfileNotFound,
    InvalidSetting,
    FeatureNotSupported,
}

/// UI Component metadata for automated WCAG 2.1 compliance audits
#[derive(Debug, Clone)]
pub struct AccessibilityComponent {
    pub id: String,
    pub role: String,
    pub label: Option<String>,
    pub contrast_ratio: f32, // e.g. 4.5
    pub keyboard_focusable: bool,
}

/// Automated WCAG compliance testing harness for UI components
pub struct AccessibilityTestingHarness {
    pub min_contrast_ratio: f32,
}

impl AccessibilityTestingHarness {
    pub fn new() -> Self {
        Self {
            min_contrast_ratio: 4.5,
        } // WCAG AA standard
    }

    /// Audit an individual UI component for WCAG 2.1 AA compliance
    pub fn audit_component(&self, component: &AccessibilityComponent) -> Vec<&'static str> {
        let mut violations = Vec::new();

        if component.label.is_none()
            || component
                .label
                .as_ref()
                .map_or(true, |l| l.trim().is_empty())
        {
            violations.push("Missing accessible label or ARIA name");
        }

        if component.contrast_ratio < self.min_contrast_ratio {
            violations.push("Insufficient text/background contrast ratio (< 4.5:1)");
        }

        if !component.keyboard_focusable && component.role == "button" {
            violations.push("Interactive button component is not keyboard focusable");
        }

        violations
    }

    /// Audit a full UI component tree
    pub fn audit_ui_tree(
        &self,
        components: &[AccessibilityComponent],
    ) -> BTreeMap<String, Vec<&'static str>> {
        let mut report = BTreeMap::new();
        for comp in components {
            let violations = self.audit_component(comp);
            if !violations.is_empty() {
                report.insert(comp.id.clone(), violations);
            }
        }
        report
    }
}

impl Default for AccessibilityTestingHarness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_creation() {
        let framework = AccessibilityFramework::new();
        assert_eq!(framework.profiles.len(), 4);
    }

    #[test]
    fn test_profile_activation() {
        let mut framework = AccessibilityFramework::new();
        assert!(framework.activate_profile("Vision Impaired").is_ok());
        assert_eq!(
            framework.active_profile,
            Some("Vision Impaired".to_string())
        );
    }

    #[test]
    fn test_invalid_profile() {
        let mut framework = AccessibilityFramework::new();
        assert!(framework.activate_profile("Nonexistent").is_err());
    }

    #[test]
    fn test_feature_enabled() {
        let mut framework = AccessibilityFramework::new();
        framework.activate_profile("Vision Impaired").unwrap();
        assert!(framework.is_feature_enabled(AccessibilityFeature::ScreenReader));
    }

    #[test]
    fn test_custom_profile() {
        let mut framework = AccessibilityFramework::new();
        let custom_profile =
            AccessibilityProfile::new("Custom".to_string(), AccessibilityCategory::Vision)
                .add_setting(AccessibilitySetting::new(
                    AccessibilityFeature::HighContrast,
                ));
        framework.add_profile(custom_profile);
        assert_eq!(framework.profiles.len(), 5);
    }

    #[test]
    fn test_accessibility_testing_harness() {
        let harness = AccessibilityTestingHarness::new();
        let valid_comp = AccessibilityComponent {
            id: "btn_ok".to_string(),
            role: "button".to_string(),
            label: Some("Submit".to_string()),
            contrast_ratio: 5.2,
            keyboard_focusable: true,
        };
        assert!(harness.audit_component(&valid_comp).is_empty());

        let invalid_comp = AccessibilityComponent {
            id: "btn_bad".to_string(),
            role: "button".to_string(),
            label: None,
            contrast_ratio: 2.1,
            keyboard_focusable: false,
        };
        let violations = harness.audit_component(&invalid_comp);
        assert_eq!(violations.len(), 3);

        let report = harness.audit_ui_tree(&[valid_comp, invalid_comp]);
        assert_eq!(report.len(), 1);
        assert!(report.contains_key("btn_bad"));
    }
}
