//! Keybinding dispatch for SigmaOS-window-manager.

use std::collections::HashMap;
use crate::{Action, Result, WmError};

/// Modifier key bitmask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifiers(pub u32);

impl Modifiers {
    pub const NONE: Self = Self(0);
    pub const SHIFT: Self = Self(1 << 0);
    pub const CTRL: Self = Self(1 << 1);
    pub const ALT: Self = Self(1 << 2);
    pub const SUPER: Self = Self(1 << 3);

    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

/// A (modifiers, keysym) pair used as a binding key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Keybinding {
    pub modifiers: Modifiers,
    /// xkbcommon keysym value.
    pub keysym: u32,
}

impl Keybinding {
    pub fn new(modifiers: Modifiers, keysym: u32) -> Self {
        Self { modifiers, keysym }
    }
}

/// Maps keybindings to WM actions.
#[derive(Debug, Default)]
pub struct KeybindingTable {
    bindings: HashMap<Keybinding, Action>,
}

impl KeybindingTable {
    pub fn new() -> Self { Self::default() }

    /// Register a keybinding. Returns an error if it already exists.
    pub fn register(&mut self, binding: Keybinding, action: Action) -> Result<()> {
        if self.bindings.contains_key(&binding) {
            return Err(WmError::KeybindingConflict(format!(
                "keysym {:x} with mods {:x} already bound",
                binding.keysym, binding.modifiers.0
            )));
        }
        self.bindings.insert(binding, action);
        Ok(())
    }

    /// Override a binding without checking for conflicts.
    pub fn bind(&mut self, binding: Keybinding, action: Action) {
        self.bindings.insert(binding, action);
    }

    /// Dispatch a key event. Returns the matching action if any.
    pub fn dispatch(&self, binding: &Keybinding) -> Option<&Action> {
        self.bindings.get(binding)
    }

    pub fn remove(&mut self, binding: &Keybinding) {
        self.bindings.remove(binding);
    }

    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}
