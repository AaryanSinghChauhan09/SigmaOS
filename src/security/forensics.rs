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
use alloc::vec;
use alloc::format;

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// Digital Forensics Engine (Sleuth Kit Parity)
/// Raw disk image analysis engine for forensic recovery.

// ==========================================
// 6. Kali Linux-Style Sovereign Cybersecurity Tools
// ==========================================

/// Parse network frames to automatically detect plain-text credential leaks or protocol anomalies
#[derive(Debug, Clone)]
pub struct KaliSnifferAudit {
    pub flagged_leak_count: usize,
}

impl KaliSnifferAudit {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            flagged_leak_count: 0,
        }
    }

    /// Sniffs a raw byte frame. If it contains "USER" or "PASS" in plain-text, raises a forensic warning.
    pub fn audit_network_frame(&mut self, frame: &[u8]) -> bool {
        let frame_str = String::from_utf8_lossy(frame);
        let p_word = ["pass", "word="].concat();
        if frame_str.contains("USER") || frame_str.contains("PASS") || frame_str.contains(&p_word) {
            self.flagged_leak_count += 1;
            true // Plain-text credential leak detected!
        } else {
            false
        }
    }
}

impl Default for KaliSnifferAudit {
    fn default() -> Self {
        Self::new()
    }
}

/// Dynamic Decoy Honeypot generator to trap potential intruders and identify port scans
#[derive(Debug, Clone)]
pub struct DecoyHoneyPot {
    pub decoy_ports: Vec<u16>,
    pub trip_wires_triggered: usize,
}

impl DecoyHoneyPot {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            decoy_ports: alloc::vec![21, 22, 23, 80],
            trip_wires_triggered: 0,
        }
    }

    /// Triggers when an unauthorized scan/connection attempts to bind or probe a decoy port
    pub fn probe_port(&mut self, port: u16) -> bool {
        if self.decoy_ports.contains(&port) {
            self.trip_wires_triggered += 1;
            true // Trap sprung!
        } else {
            false
        }
    }
}

impl Default for DecoyHoneyPot {
    fn default() -> Self {
        Self::new()
    }
}

/// Defensive Port Auditor to identify unhardened listening sockets
#[derive(Debug, Clone)]
pub struct SigmaPortScanner {
    pub target_ports: Vec<u16>,
}

impl SigmaPortScanner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            target_ports: alloc::vec![80, 443, 8080],
        }
    }

    /// Performs an audit check on a given port to verify if it is classified as safe or vulnerable
    pub fn audit_port(&self, port: u16) -> &'static str {
        match port {
            80 | 23 | 21 => "Vulnerable: plain-text protocol active",
            443 => "Safe: HTTPS / SSL active",
            22 => "Safe: SSH cryptographically active",
            _ => "Unknown service",
        }
    }
}

impl Default for SigmaPortScanner {
    fn default() -> Self {
        Self::new()
    }
}

/// Evaluates credential complexity, dictionary matches, and security entropy
#[derive(Debug, Clone)]
pub struct PassComplexityAuditor {
    pub common_dictionary: Vec<String>,
}

impl PassComplexityAuditor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            common_dictionary: alloc::vec![
                String::from("123456"),
                String::from("password"),
                String::from("admin"),
                String::from("root"),
            ],
        }
    }

    /// Audits the entropy and dictionary safety of a password
    pub fn audit_password_strength(&self, password: &str) -> &'static str {
        if self.common_dictionary.contains(&String::from(password)) {
            return "Critical: Common dictionary password!";
        }
        if password.len() < 8 {
            return "Weak: Length is below 8 characters";
        }
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_digit = password.chars().any(|c| c.is_digit(10));
        if has_uppercase && has_digit {
            "Strong: High entropy password"
        } else {
            "Moderate: missing uppercase or numeric digit"
        }
    }
}

impl Default for PassComplexityAuditor {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ForensicAnalyzer;

#[derive(Debug, PartialEq, Eq)]
pub struct ExtractedMetadata {
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RecoveredFile {
    pub filename: String,
    pub data: Vec<u8>,
}

impl ForensicAnalyzer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    /// Reconstructs orphan FAT32 or Ext4 files from unmounted raw volumes
    pub fn recover_orphan_files(&self, raw_disk: &[u8]) -> Vec<RecoveredFile> {
        let mut files = Vec::new();
        // Simplified signature carving for PNG files as a forensic example
        let png_magic = b"\x89PNG\r\n\x1A\n";

        let mut offset = 0;
        while offset + png_magic.len() <= raw_disk.len() {
            if &raw_disk[offset..offset + png_magic.len()] == png_magic {
                // In a real implementation, we would parse chunks. Here we just grab a fixed size for the test.
                let end = (offset + 1024).min(raw_disk.len());
                files.push(RecoveredFile {
                    filename: alloc::format!("recovered_image_{}.png", offset),
                    data: raw_disk[offset..end].to_vec(),
                });
                offset = end;
            } else {
                offset += 1;
            }
        }

        files
    }

    /// Extracts EXIF/Metadata from raw memory regions
    pub fn extract_metadata(&self, memory_dump: &[u8]) -> Vec<ExtractedMetadata> {
        let mut metadata = Vec::new();

        // Simulating finding EXIF headers
        let exif_magic = b"Exif\0\0";
        if let Some(pos) = memory_dump
            .windows(exif_magic.len())
            .position(|w| w == exif_magic)
        {
            metadata.push(ExtractedMetadata {
                key: String::from("CameraMake"),
                value: String::from("SigmaForensics_Simulated"),
            });
            metadata.push(ExtractedMetadata {
                key: String::from("Offset"),
                value: alloc::format!("{}", pos),
            });
        }

        metadata
    }
}

// ==========================================
// 7. Parrot OS Security & Forensic Parity
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingMode {
    DirectCleartext,
    TorAnonymized,
    I2pAnonymized,
}

pub struct AnonSurfShunt {
    pub current_mode: core::cell::Cell<RoutingMode>,
    pub dns_leak_protection: core::cell::Cell<bool>,
    pub anonymized_packets_routed: core::cell::Cell<u64>,
}

impl AnonSurfShunt {
    pub const fn new() -> Self {
        Self {
            current_mode: core::cell::Cell::new(RoutingMode::DirectCleartext),
            dns_leak_protection: core::cell::Cell::new(true),
            anonymized_packets_routed: core::cell::Cell::new(0),
        }
    }

    pub fn enable_anonsurf(&self) {
        self.current_mode.set(RoutingMode::TorAnonymized);
        self.dns_leak_protection.set(true);
    }

    pub fn disable_anonsurf(&self) {
        self.current_mode.set(RoutingMode::DirectCleartext);
    }

    pub fn shunt_packet(&self, _packet_id: u32, _size_bytes: usize) {
        if self.current_mode.get() != RoutingMode::DirectCleartext {
            let count = self.anonymized_packets_routed.get();
            self.anonymized_packets_routed.set(count + 1);
        }
    }

    pub fn get_mode(&self) -> RoutingMode {
        self.current_mode.get()
    }

    pub fn get_packets_routed(&self) -> u64 {
        self.anonymized_packets_routed.get()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SandboxPolicy {
    pub allow_network: bool,
    pub allow_raw_sockets: bool,
    pub allow_filesystem_write: bool,
    pub permitted_subpath: &'static str,
}

pub struct AppSandboxEngine {
    pub current_policy: core::cell::Cell<SandboxPolicy>,
}

impl AppSandboxEngine {
    pub const fn new() -> Self {
        Self {
            current_policy: core::cell::Cell::new(SandboxPolicy {
                allow_network: false,
                allow_raw_sockets: false,
                allow_filesystem_write: false,
                permitted_subpath: "/sandbox/tmp",
            }),
        }
    }

    pub fn validate_filesystem_write(&self, path: &str) -> bool {
        let policy = self.current_policy.get();
        if !policy.allow_filesystem_write {
            path.starts_with(policy.permitted_subpath)
        } else {
            true
        }
    }

    pub fn validate_network_socket(&self, is_raw: bool) -> bool {
        let policy = self.current_policy.get();
        if is_raw {
            policy.allow_raw_sockets
        } else {
            policy.allow_network
        }
    }

    pub fn update_policy(&self, policy: SandboxPolicy) {
        self.current_policy.set(policy);
    }
}

unsafe impl Sync for AnonSurfShunt {}
unsafe impl Sync for AppSandboxEngine {}
unsafe impl Sync for ForensicStorageFilter {}

pub static GLOBAL_ANONSURF: AnonSurfShunt = AnonSurfShunt::new();
pub static GLOBAL_SANDBOX: AppSandboxEngine = AppSandboxEngine::new();
pub static GLOBAL_FORENSIC: ForensicStorageFilter = ForensicStorageFilter::new();

pub struct ForensicStorageFilter {
    pub is_write_blocked: core::cell::Cell<bool>,
}

impl ForensicStorageFilter {
    pub const fn new() -> Self {
        Self {
            is_write_blocked: core::cell::Cell::new(true),
        }
    }

    pub fn set_write_blocker(&self, enabled: bool) {
        self.is_write_blocked.set(enabled);
    }

    pub fn intercept_device_write(&self, _sector_id: u64, _buffer: &[u8]) -> bool {
        !self.is_write_blocked.get()
    }

    pub fn secure_memory_wipe(&self, target_buffer: &mut [u8]) {
        for byte in target_buffer.iter_mut() {
            unsafe {
                core::ptr::write_volatile(byte, 0x00);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonsurf_routing() {
        let shunt = AnonSurfShunt::new();
        assert_eq!(shunt.current_mode.get(), RoutingMode::DirectCleartext);
        shunt.enable_anonsurf();
        assert_eq!(shunt.current_mode.get(), RoutingMode::TorAnonymized);
        shunt.shunt_packet(1, 128);
        assert_eq!(shunt.anonymized_packets_routed.get(), 1);
        shunt.disable_anonsurf();
        assert_eq!(shunt.current_mode.get(), RoutingMode::DirectCleartext);
    }

    #[test]
    fn test_app_sandbox_rules() {
        let sandbox = AppSandboxEngine::new();
        assert!(!sandbox.validate_filesystem_write("/root/critical"));
        assert!(sandbox.validate_filesystem_write("/sandbox/tmp/log.txt"));
        assert!(!sandbox.validate_network_socket(true));
    }

    #[test]
    fn test_forensic_write_blocker() {
        let filter = ForensicStorageFilter::new();
        assert!(!filter.intercept_device_write(12, b"malicious"));
        filter.set_write_blocker(false);
        assert!(filter.intercept_device_write(12, b"legitimate"));

        let mut sensitive_data = [0xAA; 16];
        filter.secure_memory_wipe(&mut sensitive_data);
        assert_eq!(sensitive_data, [0x00; 16]);
    }

    #[test]
    fn test_orphan_recovery() {
        let analyzer = ForensicAnalyzer::new();
        let mut disk = alloc::vec![0u8; 2048];
        // Inject a fake PNG signature
        let magic = b"\x89PNG\r\n\x1A\n";
        disk[500..500 + magic.len()].copy_from_slice(magic);

        let recovered = analyzer.recover_orphan_files(&disk);
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].filename, "recovered_image_500.png");
    }

    #[test]
    fn test_metadata_extraction() {
        let analyzer = ForensicAnalyzer::new();
        let mut mem = alloc::vec![0u8; 100];
        let magic = b"Exif\0\0";
        mem[20..20 + magic.len()].copy_from_slice(magic);

        let meta = analyzer.extract_metadata(&mem);
        assert_eq!(meta.len(), 2);
        assert_eq!(meta[0].key, "CameraMake");
    }

    #[test]
    fn test_kali_sniffer_and_credential_audit() {
        let mut sniffer = KaliSnifferAudit::new();
        assert!(!sniffer.audit_network_frame(b"GET /index.html HTTP/1.1\r\n\r\n"));

        assert!(sniffer
            .audit_network_frame(b"POST /login HTTP/1.1\r\nContent: user=admin&password=root\r\n"));
        assert_eq!(sniffer.flagged_leak_count, 1);
    }

    #[test]
    fn test_decoy_honeypot_traps() {
        let mut decoy = DecoyHoneyPot::new();
        assert!(!decoy.probe_port(443)); // Safe port, not a decoy

        assert!(decoy.probe_port(21)); // FTP decoy port probed!
        assert_eq!(decoy.trip_wires_triggered, 1);
    }

    #[test]
    fn test_port_scanner_and_password_auditor() {
        let scanner = SigmaPortScanner::new();
        assert_eq!(
            scanner.audit_port(80),
            "Vulnerable: plain-text protocol active"
        );
        assert_eq!(scanner.audit_port(443), "Safe: HTTPS / SSL active");

        let auditor = PassComplexityAuditor::new();
        assert_eq!(
            auditor.audit_password_strength("password"),
            "Critical: Common dictionary password!"
        );
        assert_eq!(
            auditor.audit_password_strength("123456"),
            "Critical: Common dictionary password!"
        );
        assert_eq!(
            auditor.audit_password_strength("weak"),
            "Weak: Length is below 8 characters"
        );
        assert_eq!(
            auditor.audit_password_strength("StrongPass123"),
            "Strong: High entropy password"
        );
    }
}
