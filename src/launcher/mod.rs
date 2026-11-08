// src/launcher/mod.rs
// App launcher system for SigmaOS

pub mod app_launcher;

pub use app_launcher::{
    init_default_apps, AppEntry, AppLauncher, Command, CommandAction, LauncherError, MatchType,
    SearchResult,
};
