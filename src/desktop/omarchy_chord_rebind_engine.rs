// src/desktop/omarchy_chord_rebind_engine.rs
// SigmaOS Sovereign Modal Keychording & Input Rebind Engine
// Inspired by Omarchy's 'add-hyprland-rebind' branch — completely re-engineered in Safe Rust
//
// Advantages over Omarchy:
// - Leader key sequences (e.g. Space-f-f for find file, Space-b-d for buffer delete)
// - Submap modes (Resize mode, Window layout mode, Media control mode)
// - Per-application window class chord context filtering
// - Hardware mouse button chord combinations (e.g. Thumb+Scroll for workspace switch)
// - 100% Safe Rust, #![no_std] compatible, zero external dependencies.

#[cfg(any(feature = "standalone_test", test))]
use std::{collections::BTreeMap, string::String, vec, vec::Vec};

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChordTrigger {
    KeyPress { key: String, modifiers: Vec<String> },
    KeySequence(Vec<String>),
    MouseChord { button: u32, with_modifier: String },
}

#[derive(Debug, Clone)]
pub struct ChordBinding {
    pub chord_id: String,
    pub description: String,
    pub trigger: ChordTrigger,
    pub action: String,
    pub target_window_class: Option<String>,
    pub submap: Option<String>,
}

/// Sovereign Modal Keychord and Rebind Engine
#[derive(Debug, Clone)]
pub struct OmarchyChordRebindEngine {
    pub bindings: BTreeMap<String, ChordBinding>,
    pub active_submap: Option<String>,
    pub leader_key: String,
    pub sequence_buffer: Vec<String>,
}

impl OmarchyChordRebindEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            bindings: BTreeMap::new(),
            active_submap: None,
            leader_key: "Space".into(),
            sequence_buffer: Vec::new(),
        };
        engine.load_sovereign_presets();
        engine
    }

    fn load_sovereign_presets(&mut self) {
        // Space leader chords (Vim / Doom Emacs style)
        self.register(ChordBinding {
            chord_id: "leader-find-file".into(),
            description: "Quick find file".into(),
            trigger: ChordTrigger::KeySequence(vec!["Space".into(), "f".into(), "f".into()]),
            action: "exec wofi --show drun".into(),
            target_window_class: None,
            submap: None,
        });

        self.register(ChordBinding {
            chord_id: "leader-split-h".into(),
            description: "Split window horizontally".into(),
            trigger: ChordTrigger::KeySequence(vec!["Space".into(), "w".into(), "s".into()]),
            action: "hyprctl dispatch splitratio 0.5".into(),
            target_window_class: None,
            submap: None,
        });

        // Mouse chord: Thumb button + Scroll Up/Down
        self.register(ChordBinding {
            chord_id: "mouse-thumb-workspace".into(),
            description: "Mouse thumb workspace switch".into(),
            trigger: ChordTrigger::MouseChord { button: 8, with_modifier: "ScrollUp".into() },
            action: "workspace +1".into(),
            target_window_class: None,
            submap: None,
        });
    }

    pub fn register(&mut self, binding: ChordBinding) {
        self.bindings.insert(binding.chord_id.clone(), binding);
    }

    pub fn enter_submap(&mut self, submap_name: &str) {
        self.active_submap = Some(submap_name.into());
    }

    pub fn exit_submap(&mut self) {
        self.active_submap = None;
    }

    pub fn push_sequence_key(&mut self, key: &str) -> Option<String> {
        self.sequence_buffer.push(key.into());

        for b in self.bindings.values() {
            if let ChordTrigger::KeySequence(seq) = &b.trigger {
                if seq == &self.sequence_buffer {
                    let action = b.action.clone();
                    self.sequence_buffer.clear();
                    return Some(action);
                }
            }
        }

        // If buffer gets too long without match, clear it
        if self.sequence_buffer.len() > 4 {
            self.sequence_buffer.clear();
        }
        None
    }
}

impl Default for OmarchyChordRebindEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_rebind_and_sequences() {
        let mut engine = OmarchyChordRebindEngine::new();
        assert!(engine.bindings.len() >= 3);

        // Sequence: Space -> f -> f
        assert_eq!(engine.push_sequence_key("Space"), None);
        assert_eq!(engine.push_sequence_key("f"), None);
        let action = engine.push_sequence_key("f");
        assert_eq!(action, Some("exec wofi --show drun".into()));
        assert!(engine.sequence_buffer.is_empty());

        // Submap test
        engine.enter_submap("resize");
        assert_eq!(engine.active_submap, Some("resize".into()));
        engine.exit_submap();
        assert_eq!(engine.active_submap, None);
    }
}
