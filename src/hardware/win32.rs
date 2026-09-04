// Win32 Emulation Compatibility Layer
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Win32Handle(pub usize);

pub struct PeLoader {
    pub entry_point: usize,
    pub sections: HashMap<String, usize>,
}

impl PeLoader {
    pub fn new() -> Self {
        PeLoader {
            entry_point: 0,
            sections: HashMap::new(),
        }
    }
}

pub struct RegistryManager {
    pub keys: HashMap<String, String>,
}

impl RegistryManager {
    pub fn new() -> Self {
        RegistryManager {
            keys: HashMap::new(),
        }
    }
}

pub struct User32MessageQueue {
    pub messages: Vec<usize>,
}

impl User32MessageQueue {
    pub fn new() -> Self {
        User32MessageQueue {
            messages: Vec::new(),
        }
    }
}

pub struct NonPagedPoolMemory {
    pub base: usize,
    pub size: usize,
}

impl NonPagedPoolMemory {
    pub fn new(size: usize) -> Self {
        NonPagedPoolMemory { base: 0, size }
    }
}
