//! SigmaOS System Locale
//!
//! System locale and language settings manager for SigmaOS. Configures the
//! system language, regional formats, and input methods.
//!
//! # Modules
//!
//! - [`locale`]: Regional format settings (dates, numbers, currency) and locale data.
//! - [`language`]: System language selection and input method configuration.

pub mod locale;
pub mod language;

/// Initialize the system-locale subsystem.
///
/// Call once at startup to enumerate installed locales, detect the active
/// input method framework, and load the current language configuration.
pub fn init() {
    // TODO: enumerate locales, detect input method framework, load config
}
