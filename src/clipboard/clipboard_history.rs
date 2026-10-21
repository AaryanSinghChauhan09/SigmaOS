// src/clipboard/clipboard_history.rs
// Unified clipboard history for SigmaOS, inspired by Omarchy 4.0.5
// (`omancy-clipboard-open`, `omancy-clipboard-paste-text`,
// `omancy-clipboard-paste-file`, `omancy-capture-text`).
//
// Design principles (SigmaOS engineering canon):
//   * Single responsibility: manage a bounded clipboard history only.
//   * Clear abstraction & encapsulation: the internal buffer is private.
//   * Command-query separation: commands mutate and report outcome;
//     queries observe without mutating.
//   * Memory safety: `#![no_std]` + `alloc`, bounds-checked, no `unsafe`.
//   * Fail-fast / defensive: validate inputs, never panic on empty history.
//   * Deterministic: sequence-based IDs and logical timestamps (no wall clock),
//     so tests are FAST, ISOLATED, REPEATABLE, SELF-VALIDATING and TIMELY.
//   * Cache-conscious / locality of reference: flat `Vec`, newest entry at
//     index 0 so the most-recently-used data stays in the hottest cache lines.
//   * Zero-copy observation: `history()` borrows the internal buffer.
//   * Composition over inheritance: plain data + free functions, no class
//     hierarchy.

#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Default maximum number of clipboard entries retained.
pub const DEFAULT_CLIPBOARD_CAPACITY: usize = 200;

/// Kind of data held on the clipboard.
///
/// Mirrors Omarchy's `omancy-clipboard-paste-text` and
/// `omancy-clipboard-paste-file` verbs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardContent {
    /// Plain text.
    Text(String),
    /// A file path with an optional MIME type.
    File { path: String, mime: Option<String> },
}

impl fmt::Display for ClipboardContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(t) => write!(f, "Text({})", t),
            Self::File { path, mime } => match mime {
                Some(m) => write!(f, "File({}:{})", path, m),
                None => write!(f, "File({})", path),
            },
        }
    }
}

/// A single clipboard history entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardItem {
    /// Monotonically increasing entry identifier (deterministic).
    pub id: u64,
    /// The clipboard payload.
    pub content: ClipboardContent,
    /// Logical timestamp (sequence number) when the entry was created.
    pub sequence: u64,
    /// Whether the entry is pinned (best-effort exempt from eviction).
    pub pinned: bool,
}

/// Bounded, deterministic clipboard history.
///
/// Every `copy` appends an entry; `paste` restores a previous entry; the
/// history is browsable. Oldest non-pinned entries are evicted once
/// `capacity` is reached, so memory usage is bounded (zero-one-infinity:
/// zero entries is valid, one is typical, infinity is bounded by capacity).
pub struct ClipboardHistory {
    items: Vec<ClipboardItem>,
    next_id: u64,
    sequence: u64,
    capacity: usize,
    evictions: u64,
}

impl ClipboardHistory {
    /// Create a new clipboard history with the given capacity.
    ///
    /// A capacity of `0` is defensively clamped to `1` so the subsystem can
    /// always hold at least the most recent entry instead of failing.
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity.max(1)),
            next_id: 1,
            sequence: 0,
            capacity: capacity.max(1),
            evictions: 0,
        }
    }

    /// Default-capacity history.
    pub fn default_capacity() -> Self {
        Self::new(DEFAULT_CLIPBOARD_CAPACITY)
    }

    /// Copy text onto the clipboard, returning the new entry id.
    ///
    /// Corresponds to Omarchy's `omancy-clipboard-paste-text` source verb.
    pub fn set_text(&mut self, text: String) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.sequence += 1;
        self.push(ClipboardItem {
            id,
            content: ClipboardContent::Text(text),
            sequence: self.sequence,
            pinned: false,
        });
        id
    }

    /// Copy a file reference onto the clipboard, returning the new entry id.
    ///
    /// Corresponds to Omarchy's `omancy-clipboard-paste-file` source verb.
    pub fn set_file(&mut self, path: String, mime: Option<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.sequence += 1;
        self.push(ClipboardItem {
            id,
            content: ClipboardContent::File { path, mime },
            sequence: self.sequence,
            pinned: false,
        });
        id
    }

    /// Return the entry with `id`, or `None` when absent. (Query)
    pub fn get(&self, id: u64) -> Option<&ClipboardItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Paste the text of entry `id`, or `None` when absent or not text.
    /// (Query — returns a clone so callers own the result.)
    pub fn paste_text(&self, id: u64) -> Option<String> {
        match self.get(id)? {
            ClipboardItem {
                content: ClipboardContent::Text(t),
                ..
            } => Some(t.clone()),
            _ => None,
        }
    }

    /// Paste the content of entry `id`, or `None` when absent. (Query)
    pub fn paste_content(&self, id: u64) -> Option<ClipboardContent> {
        self.get(id).map(|item| item.content.clone())
    }

    /// Borrow the history newest-first (zero-copy observation). (Query)
    pub fn history(&self) -> &[ClipboardItem] {
        &self.items
    }

    /// Number of entries currently held. (Query)
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the history is empty. (Query)
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Configured capacity. (Query)
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Remove every entry. (Command)
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Remove the entry with `id`, returning `true` when removed. (Command)
    pub fn remove(&mut self, id: u64) -> bool {
        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    /// Set the pinned state of entry `id`, returning `true` when found.
    /// Pinned entries are best-effort exempt from eviction. (Command)
    pub fn set_pinned(&mut self, id: u64, pinned: bool) -> bool {
        if let Some(item) = self.get_mut(id) {
            item.pinned = pinned;
            true
        } else {
            false
        }
    }

    /// Current number of evictions performed (instrumentation / observability).
    pub fn evictions(&self) -> u64 {
        self.evictions
    }

    // ---- internal ----

    fn push(&mut self, item: ClipboardItem) {
        // Newest first: most-recently-used data at index 0 (cache locality).
        self.items.insert(0, item);
        self.evict_if_over_capacity();
    }

    fn evict_if_over_capacity(&mut self) {
        while self.items.len() > self.capacity {
            // Evict the oldest non-pinned entry (scan from the tail).
            let evict_idx = self
                .items
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, item)| if !item.pinned { Some(i) } else { None });

            match evict_idx {
                Some(i) => {
                    self.items.remove(i);
                    self.evictions += 1;
                }
                // Every entry is pinned: defensively drop the oldest entry
                // anyway so capacity is never exceeded (memory discipline).
                None => {
                    self.items.pop();
                    self.evictions += 1;
                }
            }
        }
    }

    fn get_mut(&mut self, id: u64) -> Option<&mut ClipboardItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }
}

impl Default for ClipboardHistory {
    fn default() -> Self {
        Self::default_capacity()
    }
}

impl fmt::Display for ClipboardHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ClipboardHistory {{ entries={}, capacity={}, evictions={} }}",
            self.items.len(),
            self.capacity,
            self.evictions
        )
    }
}

// ---- Deterministic unit tests (FAST-ISOLATED-REPEATABLE-SELF-VALIDATING-TIMELY) ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_paste_text_round_trip() {
        let mut h = ClipboardHistory::new(10);
        let id = h.set_text("hello sigma".to_string());
        assert_eq!(h.paste_text(id), Some("hello sigma".to_string()));
        assert_eq!(h.len(), 1);
    }

    #[test]
    fn paste_unknown_id_returns_none() {
        let h = ClipboardHistory::new(10);
        assert_eq!(h.paste_text(999), None);
        assert_eq!(h.get(999), None);
    }

    #[test]
    fn paste_wrong_content_type_returns_none() {
        let mut h = ClipboardHistory::new(10);
        let id = h.set_file("/etc/sigma/motd".to_string(), None);
        assert_eq!(h.paste_text(id), None);
        assert!(h.paste_content(id).is_some());
    }

    #[test]
    fn history_is_newest_first() {
        let mut h = ClipboardHistory::new(10);
        let a = h.set_text("a".to_string());
        let b = h.set_text("b".to_string());
        let c = h.set_text("c".to_string());
        let hist: Vec<u64> = h.history().iter().map(|item| item.id).collect();
        assert_eq!(hist, vec![c, b, a]);
    }

    #[test]
    fn capacity_evicts_oldest_non_pinned() {
        let mut h = ClipboardHistory::new(2);
        let first = h.set_text("first".to_string());
        let second = h.set_text("second".to_string());
        assert_eq!(h.len(), 2);
        let third = h.set_text("third".to_string());
        assert_eq!(h.len(), 2);
        assert_eq!(h.paste_text(first), None);   // evicted
        assert_eq!(h.paste_text(second), Some("second".to_string()));
        assert_eq!(h.paste_text(third), Some("third".to_string()));
        assert_eq!(h.evictions(), 1);
    }

    #[test]
    fn pinned_entries_are_best_effort_exempt_from_eviction() {
        let mut h = ClipboardHistory::new(2);
        let pinned = h.set_text("pinned".to_string());
        let other = h.set_text("other".to_string());
        assert!(h.set_pinned(pinned, true));
        let _late = h.set_text("late".to_string()); // should evict `other`, not `pinned`
        assert_eq!(h.paste_text(pinned), Some("pinned".to_string()));
        assert_eq!(h.paste_text(other), None); // evicted
    }

    #[test]
    fn clear_empties_history() {
        let mut h = ClipboardHistory::new(10);
        h.set_text("a".to_string());
        h.clear();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn remove_deletes_entry() {
        let mut h = ClipboardHistory::new(10);
        let id = h.set_text("a".to_string());
        assert!(h.remove(id));
        assert!(!h.remove(id));
        assert!(h.is_empty());
    }

    #[test]
    fn default_capacity_is_sensible() {
        let h = ClipboardHistory::default_capacity();
        assert_eq!(h.capacity(), DEFAULT_CLIPBOARD_CAPACITY);
    }

    #[test]
    fn zero_capacity_is_clamped_instead_of_panicking() {
        let mut h = ClipboardHistory::new(0);
        assert_eq!(h.capacity(), 1);
        let id = h.set_text("only".to_string());
        assert_eq!(h.paste_text(id), Some("only".to_string()));
    }

    #[test]
    fn ids_are_monotonic_and_deterministic() {
        let mut h = ClipboardHistory::new(100);
        let a = h.set_text("a".to_string());
        let b = h.set_text("b".to_string());
        assert!(b > a);
    }
}
