// SigmaOS Safe Win32 Compatibility Subsystem (SigmaWin)
// Designed to parse, load, and manage legacy Win32 binaries securely on the sovereign transaction bus

use std::collections::HashMap;

/// Win32 processing error states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Win32Error {
    Success = 0,
    InvalidPEHeader = 1,
    RegistryKeyNotFound = 2,
    MessageQueueEmpty = 3,
    PlatformMismatch = 4,
}

/// Win32 HANDLE abstraction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Win32Handle(pub u64);

/// Supported PE formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeFormat {
    Pe32,     // 32-bit x86
    Pe32Plus, // 64-bit x86_64
}

// ==========================================
// 1. Concrete PE Binary Loader Parser
// ==========================================

pub struct PeLoader {
    pub binary_format: PeFormat,
    pub entry_point_addr: u64,
}

impl Default for PeLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PeLoader {
    pub fn new() -> Self {
        PeLoader {
            binary_format: PeFormat::Pe32Plus,
            entry_point_addr: 0,
        }
    }

    /// Parses raw binary bytes to load Windows PE headers securely
    pub fn load_header(&mut self, raw_bytes: &[u8]) -> Result<(), Win32Error> {
        if raw_bytes.len() < 64 {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Validate DOS 'MZ' signature
        if raw_bytes[0] != b'M' || raw_bytes[1] != b'Z' {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Offset to PE Header is at address 0x3C
        let pe_offset = raw_bytes[0x3C] as usize;
        if pe_offset + 4 >= raw_bytes.len() {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Validate PE signature 'PE\0\0'
        if raw_bytes[pe_offset] != b'P' || raw_bytes[pe_offset + 1] != b'E' {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Parse Magic field to determine 32-bit vs 64-bit
        // Magic offset relative to PE header offset is usually 24 (Optional Header starts at 24)
        let optional_header_offset = pe_offset + 24;
        if optional_header_offset + 2 >= raw_bytes.len() {
            return Err(Win32Error::InvalidPEHeader);
        }

        let magic = (raw_bytes[optional_header_offset] as u16)
            | ((raw_bytes[optional_header_offset + 1] as u16) << 8);
        match magic {
            0x10B => {
                self.binary_format = PeFormat::Pe32; // x86 32-bit magic
            }
            0x20B => {
                self.binary_format = PeFormat::Pe32Plus; // x86_64 64-bit magic
            }
            _ => return Err(Win32Error::InvalidPEHeader),
        }

        Ok(())
    }
}

// ==========================================
// 2. Structured Registry Subsystem
// ==========================================

pub struct RegistryManager {
    pub keys: HashMap<String, String>,
}

impl Default for RegistryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryManager {
    pub fn new() -> Self {
        let mut reg = RegistryManager {
            keys: HashMap::new(),
        };
        // Seed default registry settings
        reg.set_key(
            "HKLM\\Software\\SigmaWin\\Version".to_string(),
            "1.0.0-LTS".to_string(),
        );
        reg
    }

    pub fn set_key(&mut self, path: String, value: String) {
        self.keys.insert(path, value);
    }

    pub fn get_key(&self, path: &str) -> Option<&String> {
        self.keys.get(path)
    }
}

// ==========================================
// 3. USER32 Message Loop Compositor
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Win32Message {
    Paint,
    KeyDown(u8),
    Close,
}

pub struct User32MessageQueue {
    pub messages: Vec<Win32Message>,
}

impl Default for User32MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl User32MessageQueue {
    pub fn new() -> Self {
        User32MessageQueue {
            messages: Vec::new(),
        }
    }

    pub fn post_message(&mut self, msg: Win32Message) {
        self.messages.push(msg);
    }

    pub fn get_message(&mut self) -> Result<Win32Message, Win32Error> {
        if self.messages.is_empty() {
            return Err(Win32Error::MessageQueueEmpty);
        }
        Ok(self.messages.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pe_loader_invalid_bytes() {
        let mut loader = PeLoader::new();
        let bytes = [0u8; 10];
        assert_eq!(loader.load_header(&bytes), Err(Win32Error::InvalidPEHeader));
    }

    #[test]
    fn test_pe_loader_valid_mock() {
        let mut bytes = vec![0u8; 256];
        bytes[0] = b'M';
        bytes[1] = b'Z';
        bytes[0x3C] = 64; // PE header offset
        bytes[64] = b'P';
        bytes[65] = b'E';
        // Optional header starts at 64 + 24 = 88. Magic value for PE32+ (0x20B)
        bytes[88] = 0x0B;
        bytes[89] = 0x02;

        let mut loader = PeLoader::new();
        assert_eq!(loader.load_header(&bytes), Ok(()));
        assert_eq!(loader.binary_format, PeFormat::Pe32Plus);
    }

    #[test]
    fn test_registry_manager() {
        let mut manager = RegistryManager::new();
        assert_eq!(
            manager
                .get_key("HKLM\\Software\\SigmaWin\\Version")
                .unwrap(),
            "1.0.0-LTS"
        );
        manager.set_key(
            "HKCU\\Software\\Theme".to_string(),
            "Glassmorphism".to_string(),
        );
        assert_eq!(
            manager.get_key("HKCU\\Software\\Theme").unwrap(),
            "Glassmorphism"
        );
    }

    #[test]
    fn test_message_queue() {
        let mut queue = User32MessageQueue::new();
        assert_eq!(queue.get_message(), Err(Win32Error::MessageQueueEmpty));
        queue.post_message(Win32Message::Paint);
        queue.post_message(Win32Message::Close);
        assert_eq!(queue.get_message(), Ok(Win32Message::Paint));
        assert_eq!(queue.get_message(), Ok(Win32Message::Close));
    }
}
// SigmaOS Safe Win32 Compatibility Subsystem (SigmaWin)
// Designed to parse, load, and manage legacy Win32 binaries securely on the sovereign transaction bus


/// Win32 processing error states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Win32Error {
    Success = 0,
    InvalidPEHeader = 1,
    RegistryKeyNotFound = 2,
    MessageQueueEmpty = 3,
    PlatformMismatch = 4,
}

/// Win32 HANDLE abstraction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Win32Handle(pub u64);

/// Supported PE formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeFormat {
    Pe32,    // 32-bit x86
    Pe32Plus, // 64-bit x86_64
}

// ==========================================
// 1. Concrete PE Binary Loader Parser
// ==========================================

pub struct PeLoader {
    pub binary_format: PeFormat,
    pub entry_point_addr: u64,
}

impl PeLoader {
    pub fn new() -> Self {
        PeLoader {
            binary_format: PeFormat::Pe32Plus,
            entry_point_addr: 0,
        }
    }

    /// Parses raw binary bytes to load Windows PE headers securely
    pub fn load_header(&mut self, raw_bytes: &[u8]) -> Result<(), Win32Error> {
        if raw_bytes.len() < 64 {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Validate DOS 'MZ' signature
        if raw_bytes[0] != b'M' || raw_bytes[1] != b'Z' {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Offset to PE Header is at address 0x3C
        let pe_offset = raw_bytes[0x3C] as usize;
        if pe_offset + 4 >= raw_bytes.len() {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Validate PE signature 'PE\0\0'
        if raw_bytes[pe_offset] != b'P' || raw_bytes[pe_offset + 1] != b'E' {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Parse Magic field to determine 32-bit vs 64-bit
        // Magic offset relative to PE header offset is usually 24 (Optional Header starts at 24)
        let optional_header_offset = pe_offset + 24;
        if optional_header_offset + 2 >= raw_bytes.len() {
            return Err(Win32Error::InvalidPEHeader);
        }

        let magic = (raw_bytes[optional_header_offset] as u16) | ((raw_bytes[optional_header_offset + 1] as u16) << 8);
        match magic {
            0x10B => {
                self.binary_format = PeFormat::Pe32; // x86 32-bit magic
            }
            0x20B => {
                self.binary_format = PeFormat::Pe32Plus; // x86_64 64-bit magic
            }
            _ => return Err(Win32Error::InvalidPEHeader),
        }

        Ok(())
    }
}

// ==========================================
// 2. Structured Registry Subsystem
// ==========================================

pub struct RegistryManager {
    pub keys: HashMap<String, String>,
}

impl RegistryManager {
    pub fn new() -> Self {
        let mut reg = RegistryManager { keys: HashMap::new() };
        // Seed default registry settings
        reg.set_key("HKLM\\Software\\SigmaWin\\Version".to_string(), "1.0.0-LTS".to_string());
        reg
    }

    pub fn set_key(&mut self, path: String, value: String) {
        self.keys.insert(path, value);
    }

    pub fn get_key(&self, path: &str) -> Option<&String> {
        self.keys.get(path)
    }
}

// ==========================================
// 3. USER32 Message Loop Compositor
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Win32Message {
    Paint,
    KeyDown(u8),
    Close,
}

pub struct User32MessageQueue {
    pub messages: Vec<Win32Message>,
}

impl User32MessageQueue {
    pub fn new() -> Self {
        User32MessageQueue { messages: Vec::new() }
    }

    pub fn post_message(&mut self, msg: Win32Message) {
        self.messages.push(msg);
    }

    pub fn get_message(&mut self) -> Result<Win32Message, Win32Error> {
        if self.messages.is_empty() {
            return Err(Win32Error::MessageQueueEmpty);
        }
        Ok(self.messages.remove(0))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pe_loader_invalid_bytes() {
        let mut loader = PeLoader::new();
        let bytes = [0u8; 10];
        assert_eq!(loader.load_header(&bytes), Err(Win32Error::InvalidPEHeader));
    }

    #[test]
    fn test_pe_loader_valid_mock() {
        let mut bytes = vec![0u8; 256];
        bytes[0] = b'M';
        bytes[1] = b'Z';
        bytes[0x3C] = 64; // PE header offset
        bytes[64] = b'P';
        bytes[65] = b'E';
        // Optional header starts at 64 + 24 = 88. Magic value for PE32+ (0x20B)
        bytes[88] = 0x0B;
        bytes[89] = 0x02;

        let mut loader = PeLoader::new();
        assert_eq!(loader.load_header(&bytes), Ok(()));
        assert_eq!(loader.binary_format, PeFormat::Pe32Plus);
    }

    #[test]
    fn test_registry_manager() {
        let mut manager = RegistryManager::new();
        assert_eq!(manager.get_key("HKLM\\Software\\SigmaWin\\Version").unwrap(), "1.0.0-LTS");
        manager.set_key("HKCU\\Software\\Theme".to_string(), "Glassmorphism".to_string());
        assert_eq!(manager.get_key("HKCU\\Software\\Theme").unwrap(), "Glassmorphism");
    }

    #[test]
    fn test_message_queue() {
        let mut queue = User32MessageQueue::new();
        assert_eq!(queue.get_message(), Err(Win32Error::MessageQueueEmpty));
        queue.post_message(Win32Message::Paint);
        queue.post_message(Win32Message::Close);
        assert_eq!(queue.get_message(), Ok(Win32Message::Paint));
        assert_eq!(queue.get_message(), Ok(Win32Message::Close));
    }
}
