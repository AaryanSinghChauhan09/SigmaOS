// SigmaOS-theming-native
// Native theming engine integration — bridges SigmaOS themes with toolkit rendering.

pub mod engine;
pub mod bridge;

/// Initialise the native theming engine.
///
/// Call this once at application startup to auto-detect the active
/// toolkit and load the appropriate rendering bridge.
pub fn init() {
    // TODO: detect toolkit, load bridge, apply active theme
}
