//! SigmaOS Stream
//!
//! IPTV and media streaming client for SigmaOS. Parses M3U playlists and
//! plays live TV channels and VOD content via a configurable media backend.
//!
//! # Modules
//!
//! - [`playlist`]: M3U playlist parsing, channel grouping, and VOD catalog.
//! - [`player`]: Media player backend abstraction and playback control.

pub mod playlist;
pub mod player;

/// Initialize the stream subsystem.
///
/// Call once at startup to load configuration, detect available
/// media backends, and prepare the playlist engine.
pub fn init() {
    // TODO: load config, detect media backend, initialize playlist engine
}
