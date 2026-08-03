// SigmaOS Accessibility Module
pub mod framework;
pub mod screenreader;
pub mod magnifier;
pub mod keyboard;

pub use framework::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
};

pub use screenreader::{
    VoiceID, VoiceGender, SimpleVoice, Voice, ScreenReader, SimpleScreenReader, BrailleDisplay, SimpleBrailleDisplay,
};

pub use magnifier::{
    MagnifierID, Magnifier, SimpleMagnifier, MagnifierManager, SimpleMagnifierManager, ColorFilter, SimpleColorFilter,
};

pub use keyboard::{
    KeyID, KeyType, VirtualKey, SimpleVirtualKey, OnScreenKeyboard, SimpleOnScreenKeyboard, StickyKeys, SimpleStickyKeys,
};
