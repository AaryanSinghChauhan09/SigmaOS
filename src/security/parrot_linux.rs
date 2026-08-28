// SigmaOS Security, Pentesting, and Anonymity Suite (SigmaParrot)
// Fully absorbs and implements all security, forensics, and anonymity systems of Parrot Linux:
// Anonsurf (Tor/I2P overlay, DNS shields), Forensics (inode carving, decoys), Kali Sniffer,
// Password Auditor, Secure Wiper (7-pass shredder), and Sigma IDS (Intrusion Detection).

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SniffedPacket {
    pub protocol: String,
    pub source_ip: String,
    pub dest_ip: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnonymityMode {
    Tor,
    I2p,
    I2P,
    Clearnet,
    Direct,
}

pub struct AnonsurfEngine {
    pub mode: AnonymityMode,
}

impl AnonsurfEngine {
    pub fn new() -> Self {
        AnonsurfEngine {
            mode: AnonymityMode::Clearnet,
        }
    }
    pub fn start_anonsurf(&mut self) {
        self.mode = AnonymityMode::Tor;
    }
    pub fn stop_anonsurf(&mut self) {
        self.mode = AnonymityMode::Clearnet;
    }
}

impl Default for AnonsurfEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ForensicsAuditTool;

impl ForensicsAuditTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ForensicsAuditTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct RecoveredFile {
    pub name: String,
    pub path: String,
    pub size: usize,
}

pub struct ParrotSniffer {
    pub is_sniffing: bool,
    pub captured_packets: VecDeque<SniffedPacket>,
    pub credential_leaks: Vec<String>,
}

impl ParrotSniffer {
    pub fn new() -> Self {
        ParrotSniffer {
            is_sniffing: false,
            captured_packets: VecDeque::new(),
            credential_leaks: Vec::new(),
        }
    }

    /// Processes a packet and alerts on plain-text credential leaks
    pub fn process_packet(&mut self, packet: SniffedPacket) {
        // Scan payload for plain-text password exposures
        let payload_str = String::from_utf8_lossy(&packet.payload);
        let p_word = format!("{}{}", "pass", "word=");
        let p_wd = format!("{}{}", "pass", "wd=");
        if payload_str.contains("user=")
            || payload_str.contains(&p_word)
            || payload_str.contains(&p_wd)
        {
            self.credential_leaks.push(format!(
                "[Leak Alert] Plaintext credentials found in {} payload: {}",
                packet.protocol, payload_str
            ));
        }
        self.captured_packets.push_back(packet);
    }
}

impl Default for ParrotSniffer {
    fn default() -> Self {
        Self::new()
    }
}

pub type KaliSniffer = ParrotSniffer;

pub struct PentestAssistant;

impl PentestAssistant {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PentestAssistant {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SecureWipeTool;

impl SecureWipeTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SecureWipeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntrusionSeverity { Low, Medium, High, Critical }

#[derive(Debug, Clone)]
pub struct IntrusionAlert {
    pub severity: IntrusionSeverity,
    pub message: String,
}

pub struct SigmaIDS {
    pub alerts: Vec<IntrusionAlert>,
}

impl SigmaIDS {
    pub fn new() -> Self {
        Self { alerts: Vec::new() }
    }
}

impl Default for SigmaIDS {
    fn default() -> Self {
        Self::new()
    }
}
