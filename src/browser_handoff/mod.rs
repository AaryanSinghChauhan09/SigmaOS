// src/browser_handoff/mod.rs
// Browser URL handoff and default-browser management for SigmaOS.
// Inspired by Omarchy 4.0.5 (`omancy-default-browser`,
// `omancy-cmd-browser-handoff`, `omancy-chromium-copy-url-host`,
// `omancy-chromium-ytdlp-host`).

pub mod browser_handoff;

pub use browser_handoff::{
    BrowserHandoff, DefaultBrowser, HandoffError, HandoffOutcome, HandoffResult,
};
