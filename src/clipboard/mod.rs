// src/clipboard/mod.rs
// Unified clipboard subsystem for SigmaOS.
// Inspired by Omarchy 4.0.5 (`omancy-clipboard-open`,
// `omancy-clipboard-paste-text`, `omancy-clipboard-paste-file`,
// `omancy-capture-text`).

pub mod clipboard_history;

pub use clipboard_history::{
    ClipboardContent, ClipboardHistory, ClipboardItem, DEFAULT_CLIPBOARD_CAPACITY,
};
