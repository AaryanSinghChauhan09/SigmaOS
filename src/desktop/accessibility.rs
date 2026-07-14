#![no_std]

/// Accessibility Suite for SigmaOS
/// Based on 100-Improvement-Ideas.md #45: Accessibility suite (screen reader, magnifier)
/// Implements screen reader, magnifier, and accessibility features

use core::sync::atomic::{AtomicU64, Ordering};

/// Accessibility feature type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityFeature {
    ScreenReader = 0,
    Magnifier = 1,
    HighContrast = 2,
    TextToSpeech = 3,
    SpeechToText = 4,
    ColorBlindMode = 5,
}

/// Magnifier mode
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagnifierMode {
    FullScreen = 0,
    Lens = 1,
    Docked = 2,
}

/// Screen reader voice
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceType {
    Male1 = 0,
    Male2 = 1,
    Female1 = 2,
    Female2 = 3,
}

/// Screen reader settings
#[repr(C)]
pub struct ScreenReaderSettings {
    pub enabled: bool,
    pub voice: VoiceType,
    pub speech_rate: u8, // 0-100
    pub pitch: u8, // 0-100
    pub volume: u8, // 0-100
}

impl ScreenReaderSettings {
    pub fn new() -> Self {
        ScreenReaderSettings {
            enabled: false,
            voice: VoiceType::Female1,
            speech_rate: 50,
            pitch: 50,
            volume: 80,
        }
    }
}

/// Magnifier settings
#[repr(C)]
pub struct MagnifierSettings {
    pub enabled: bool,
    pub mode: MagnifierMode,
    pub zoom_level: u8, // 100-500
    pub follow_cursor: bool,
    pub follow_focus: bool,
}

impl MagnifierSettings {
    pub fn new() -> Self {
        MagnifierSettings {
            enabled: false,
            mode: MagnifierMode::Lens,
            zoom_level: 200,
            follow_cursor: true,
            follow_focus: true,
        }
    }
}

/// High contrast settings
#[repr(C)]
pub struct HighContrastSettings {
    pub enabled: bool,
    pub theme: u8, // 0=high contrast black, 1=high contrast white
}

impl HighContrastSettings {
    pub fn new() -> Self {
        HighContrastSettings {
            enabled: false,
            theme: 0,
        }
    }
}

/// Accessibility manager
pub struct AccessibilityManager {
    pub screen_reader: ScreenReaderSettings,
    pub magnifier: MagnifierSettings,
    pub high_contrast: HighContrastSettings,
    pub enabled_features: Vec<Option<AccessibilityFeature>>,
}

impl AccessibilityManager {
    pub fn new() -> Self {
        AccessibilityManager {
            screen_reader: ScreenReaderSettings::new(),
            magnifier: MagnifierSettings::new(),
            high_contrast: HighContrastSettings::new(),
            enabled_features: Vec::new(),
        }
    }
    
    /// Enable accessibility feature
    pub fn enable_feature(&mut self, feature: AccessibilityFeature) -> Result<(), AccessibilityError> {
        match feature {
            AccessibilityFeature::ScreenReader => {
                self.screen_reader.enabled = true;
                self.enabled_features.push(Some(feature));
            }
            AccessibilityFeature::Magnifier => {
                self.magnifier.enabled = true;
                self.enabled_features.push(Some(feature));
            }
            AccessibilityFeature::HighContrast => {
                self.high_contrast.enabled = true;
                self.enabled_features.push(Some(feature));
            }
            _ => {
                self.enabled_features.push(Some(feature));
            }
        }
        Ok(())
    }
    
    /// Disable accessibility feature
    pub fn disable_feature(&mut self, feature: AccessibilityFeature) -> Result<(), AccessibilityError> {
        match feature {
            AccessibilityFeature::ScreenReader => {
                self.screen_reader.enabled = false;
            }
            AccessibilityFeature::Magnifier => {
                self.magnifier.enabled = false;
            }
            AccessibilityFeature::HighContrast => {
                self.high_contrast.enabled = false;
            }
            _ => {}
        }
        
        for feature_option in &mut self.enabled_features {
            if let Some(ref enabled_feature) = *feature_option {
                if *enabled_feature == feature {
                    *feature_option = None;
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if feature is enabled
    pub fn is_feature_enabled(&self, feature: AccessibilityFeature) -> bool {
        match feature {
            AccessibilityFeature::ScreenReader => self.screen_reader.enabled,
            AccessibilityFeature::Magnifier => self.magnifier.enabled,
            AccessibilityFeature::HighContrast => self.high_contrast.enabled,
            _ => {
                for feature_option in &self.enabled_features {
                    if let Some(ref enabled_feature) = *feature_option {
                        if *enabled_feature == feature {
                            return true;
                        }
                    }
                }
                false
            }
        }
    }
    
    /// Set screen reader voice
    pub fn set_screen_reader_voice(&mut self, voice: VoiceType) {
        self.screen_reader.voice = voice;
    }
    
    /// Set screen reader speech rate
    pub fn set_speech_rate(&mut self, rate: u8) {
        self.screen_reader.speech_rate = rate.min(100);
    }
    
    /// Set magnifier zoom level
    pub fn set_magnifier_zoom(&mut self, zoom: u8) {
        self.magnifier.zoom_level = zoom.max(100).min(500);
    }
    
    /// Set magnifier mode
    pub fn set_magnifier_mode(&mut self, mode: MagnifierMode) {
        self.magnifier.mode = mode;
    }
    
    /// Read text (screen reader)
    pub fn read_text(&self, text: &str) -> Result<(), AccessibilityError> {
        if !self.screen_reader.enabled {
            return Err(AccessibilityError::FeatureNotEnabled);
        }
        // In real implementation, use TTS engine
        Ok(())
    }
    
    /// Get enabled features
    pub fn get_enabled_features(&self) -> Vec<AccessibilityFeature> {
        let mut features = Vec::new();
        
        if self.screen_reader.enabled {
            features.push(AccessibilityFeature::ScreenReader);
        }
        if self.magnifier.enabled {
            features.push(AccessibilityFeature::Magnifier);
        }
        if self.high_contrast.enabled {
            features.push(AccessibilityFeature::HighContrast);
        }
        
        for feature_option in &self.enabled_features {
            if let Some(ref feature) = *feature_option {
                if !features.contains(feature) {
                    features.push(*feature);
                }
            }
        }
        
        features
    }
}

/// Accessibility error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AccessibilityError {
    Success = 0,
    FeatureNotEnabled = 1,
    FeatureNotFound = 2,
    InvalidSetting = 3,
    TTSEngineError = 4,
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn contains(&self, item: &T) -> bool {
        for i in 0..self.len {
            unsafe {
                let current = core::ptr::read(self.data.add(i));
                if core::mem::discriminant(&current) == core::mem::discriminant(item) {
                    return true;
                }
            }
        }
        false
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
