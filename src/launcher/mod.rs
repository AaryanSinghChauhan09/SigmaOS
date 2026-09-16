// src/launcher/mod.rs
// App launcher system for SigmaOS

#![no_std]

pub mod app_launcher;

pub use app_launcher::{
    AppLauncher,
    AppEntry,
    SearchResult,
    MatchType,
    Command,
    CommandAction,
    LauncherError,
    init_default_apps,
};
