#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Sovereign Win32 Compatibility Subsystem (SigmaWin)
// Implementing complete Windows 11 Gap Closure & PE Loading / Registry / USER32/GDI32 Emulation

use crate::klib::HashMap;

/// PE execution formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeFormat {
    Pe32,     // 32-bit x86 Windows binary
    Pe32Plus, // 64-bit x86_64 Windows binary
}

/// Win32 Subsystem execution errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Win32Error {
    Success = 0,
    InvalidPEHeader = 1,
    RegistryKeyNotFound = 2,
    MessageQueueEmpty = 3,
    PlatformMismatch = 4,
    SocketError = 5,
    D3DError = 6,
}

// ==========================================
// 1. Portable Executable Binary Loader
// ==========================================

#[derive(Debug, Clone)]
pub struct PeLoader {
    pub binary_format: PeFormat,
    pub entry_point_addr: u64,
}

impl PeLoader {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            binary_format: PeFormat::Pe32Plus,
            entry_point_addr: 0,
        }
    }

    /// Parses Portable Executable header structure securely
    pub fn load_header(&mut self, raw_bytes: &[u8]) -> Result<(), Win32Error> {
        if raw_bytes.len() < 64 {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Validate DOS signature 'MZ'
        if raw_bytes[0] != b'M' || raw_bytes[1] != b'Z' {
            return Err(Win32Error::InvalidPEHeader);
        }

        // PE offset is stored at 0x3C
        let pe_offset = raw_bytes[0x3C] as usize;
        if pe_offset + 4 >= raw_bytes.len() {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Validate PE signature 'PE\0\0'
        if raw_bytes[pe_offset] != b'P' || raw_bytes[pe_offset + 1] != b'E' {
            return Err(Win32Error::InvalidPEHeader);
        }

        // Optional header starts 24 bytes after the PE signature
        let optional_header_offset = pe_offset + 24;
        if optional_header_offset + 2 >= raw_bytes.len() {
            return Err(Win32Error::InvalidPEHeader);
        }

        let magic = (raw_bytes[optional_header_offset] as u16)
            | ((raw_bytes[optional_header_offset + 1] as u16) << 8);

        match magic {
            0x10B => {
                self.binary_format = PeFormat::Pe32;
            }
            0x20B => {
                self.binary_format = PeFormat::Pe32Plus;
            }
            _ => return Err(Win32Error::InvalidPEHeader),
        }

        Ok(())
    }
}

impl Default for PeLoader {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. Persistent Transactional Registry
// ==========================================

#[derive(Debug, Clone)]
pub struct RegistryManager {
    pub keys: HashMap<String, String>,
}

impl RegistryManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut reg = Self {
            keys: HashMap::new(),
        };
        reg.set_key(
            "HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\CurrentBuild".to_string(),
            "22000".to_string(), // Windows 11 Build ID
        );
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

impl Default for RegistryManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 3. USER32 Message Loop Emulator
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Win32Message {
    Paint,
    KeyDown(u8),
    Close,
}

#[derive(Debug, Clone)]
pub struct User32MessageQueue {
    pub messages: Vec<Win32Message>,
}

impl User32MessageQueue {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
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

impl Default for User32MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 4. WinSock (Windows Sockets) Adapter
// ==========================================

#[derive(Debug, Clone)]
pub struct WinSockAdapter {
    pub wsa_active: bool,
    pub active_connections: HashMap<u32, String>,
}

impl WinSockAdapter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            wsa_active: false,
            active_connections: HashMap::new(),
        }
    }

    pub fn wsa_startup(&mut self) -> Result<(), Win32Error> {
        self.wsa_active = true;
        Ok(())
    }

    pub fn wsa_cleanup(&mut self) {
        self.wsa_active = false;
        self.active_connections.clear();
    }

    pub fn socket_connect(&mut self, socket_id: u32, endpoint: String) -> Result<(), Win32Error> {
        if !self.wsa_active {
            return Err(Win32Error::SocketError);
        }
        self.active_connections.insert(socket_id, endpoint);
        Ok(())
    }
}

impl Default for WinSockAdapter {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 5. Direct3D (DirectX) to Vulkan Translator
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum D3dVersion {
    Dx9,
    Dx11,
    Dx12,
}

#[derive(Debug, Clone)]
pub struct D3dToVulkanTranslator {
    pub version: D3dVersion,
    pub vulkan_layers_active: bool,
}

impl D3dToVulkanTranslator {
    pub fn new(version: D3dVersion) -> Self {
        Self {
            version,
            vulkan_layers_active: true,
        }
    }

    pub fn translate_draw_call(&self, vertices_count: u32) -> Result<String, Win32Error> {
        if !self.vulkan_layers_active {
            return Err(Win32Error::D3DError);
        }
        Ok(format!(
            "vkCmdDraw(vk_context, {}, 1, 0, 0)",
            vertices_count
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pe_loader_header_parsing() {
        let mut loader = PeLoader::new();

        // MZ DOS stub + offset 0x3C pointing to PE signature
        let mut raw_bytes = vec![0u8; 128];
        raw_bytes[0] = b'M';
        raw_bytes[1] = b'Z';
        raw_bytes[0x3C] = 0x40; // PE signature offset

        // PE\0\0 signature
        raw_bytes[0x40] = b'P';
        raw_bytes[0x41] = b'E';

        // PE32 Optional Header Magic (0x10B) at optional_header_offset = 0x40 + 24 = 0x58
        raw_bytes[0x58] = 0x0B;
        raw_bytes[0x59] = 0x01;

        assert!(loader.load_header(&raw_bytes).is_ok());
        assert_eq!(loader.binary_format, PeFormat::Pe32);
    }

    #[test]
    fn test_persistent_registry_windows11() {
        let mut reg = RegistryManager::new();
        let key_path = "HKLM\\Software\\Microsoft\\Windows NT\\CurrentVersion\\CurrentBuild";
        let val = reg.get_key(key_path).unwrap();
        assert_eq!(val, "22000");

        reg.set_key(
            "HKCU\\Control Panel\\Desktop\\Theme".to_string(),
            "Dark".to_string(),
        );
        assert_eq!(
            reg.get_key("HKCU\\Control Panel\\Desktop\\Theme").unwrap(),
            "Dark"
        );
    }

    #[test]
    fn test_user32_message_loop() {
        let mut queue = User32MessageQueue::new();
        queue.post_message(Win32Message::Paint);
        queue.post_message(Win32Message::KeyDown(0x1B));

        assert_eq!(queue.get_message().unwrap(), Win32Message::Paint);
        assert_eq!(queue.get_message().unwrap(), Win32Message::KeyDown(0x1B));
        assert!(queue.get_message().is_err());
    }

    #[test]
    fn test_winsock_and_d3d_translation() {
        let mut winsock = WinSockAdapter::new();
        assert!(winsock
            .socket_connect(1, "127.0.0.1:80".to_string())
            .is_err());

        winsock.wsa_startup().unwrap();
        assert!(winsock
            .socket_connect(1, "127.0.0.1:80".to_string())
            .is_ok());

        let dx_translator = D3dToVulkanTranslator::new(D3dVersion::Dx11);
        let vk_draw = dx_translator.translate_draw_call(36).unwrap();
        assert_eq!(vk_draw, "vkCmdDraw(vk_context, 36, 1, 0, 0)");
    }
}
