// Build Codex Module
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodexCategory {
    Kernel,
    Driver,
    Userland,
}

pub struct CodexEntry {
    pub name: String,
    pub category: CodexCategory,
    pub recipe: String,
}

pub struct BuildCodex {
    pub entries: HashMap<String, CodexEntry>,
}

impl BuildCodex {
    pub fn new() -> Self {
        BuildCodex {
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, entry: CodexEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }
}
