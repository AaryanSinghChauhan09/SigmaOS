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
