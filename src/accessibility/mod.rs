// SigmaOS Accessibility Module
pub mod framework;
pub mod keyboard;
pub mod magnifier;
pub mod screenreader;

pub use framework::{
    AccessibilityCategory, AccessibilityError, AccessibilityFeature, AccessibilityFramework,
    AccessibilityProfile, AccessibilitySetting,
};

pub use screenreader::{
    BrailleDisplay, ScreenReader, SimpleBrailleDisplay, SimpleScreenReader, SimpleVoice, Voice,
    VoiceGender, VoiceID,
};

pub use magnifier::{
    ColorFilter, Magnifier, MagnifierID, MagnifierManager, SimpleColorFilter, SimpleMagnifier,
    SimpleMagnifierManager,
};

pub use keyboard::{
    KeyID, KeyType, OnScreenKeyboard, SimpleOnScreenKeyboard, SimpleStickyKeys, SimpleVirtualKey,
    StickyKeys, VirtualKey,
};
