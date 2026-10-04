//! Compatibility exports for the single canonical SigmaOS command palette.
//!
//! Keep the implementation in `desktop::launcher` so the desktop menu and
//! tools namespace cannot drift into separate command registries.

pub use crate::desktop::launcher::{
    CommandPalette as OmarchyCommandPalette, LauncherMode, SearchResultItem, SystemActionError,
    SystemActionHandler, SystemActionKind,
};
