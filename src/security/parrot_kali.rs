// SigmaOS Parrot Security & Kali Parity Engine
// Zero-dependency, #![no_std] compliant, zero-allocation
// Extends SigmaOS security structures with AnonSurf routing, AppSandbox policy, Forensic storage filters,
// MAC spoofer (macchanger), Packet sniffer analyzer (wireshark/tcpdump), and Hash credential auditor (john/hashcat)

use core::cell::Cell;

// ==========================================
// 1. AnonSurf Routing Shunt
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingMode {
    DirectCleartext,
    TorAnonymized,
    I2pAnonymized,
}

pub struct AnonSurfShunt {
    pub current_mode: Cell<RoutingMode>,
    pub dns_leak_protection: Cell<bool>,
    pub anonymized_packets_routed: Cell<u64>,
}

impl AnonSurfShunt {
    pub const fn new() -> Self {
        Self {
            current_mode: Cell::new(RoutingMode::DirectCleartext),
            dns_leak_protection: Cell::new(true),
            anonymized_packets_routed: Cell::new(0),
        }
    }

    /// Transitions the primary network interfaces into an encrypted Tor routing mode
    pub fn enable_anonsurf(&self) {
        self.current_mode.set(RoutingMode::TorAnonymized);
        self.dns_leak_protection.set(true);
    }

    /// Disables anonymized redirection
    pub fn disable_anonsurf(&self) {
        self.current_mode.set(RoutingMode::DirectCleartext);
    }

    /// Simulates interception and routing of packets through virtual Tor nodes
    pub fn shunt_packet(&self, _packet_id: u32, _size_bytes: usize) {
        if self.current_mode.get() != RoutingMode::DirectCleartext {
            let count = self.anonymized_packets_routed.get();
            self.anonymized_packets_routed.set(count + 1);
        }
    }
}

impl Default for AnonSurfShunt {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. AppSandbox Policy Engine (Firejail-Parity)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandboxPolicy {
    pub allow_network: bool,
    pub allow_raw_sockets: bool,
    pub allow_filesystem_write: bool,
    pub permitted_subpath: &'static str,
}

pub struct AppSandboxEngine {
    pub current_policy: Cell<SandboxPolicy>,
}

impl AppSandboxEngine {
    pub const fn new() -> Self {
        Self {
            current_policy: Cell::new(SandboxPolicy {
                allow_network: false,
                allow_raw_sockets: false,
                allow_filesystem_write: false,
                permitted_subpath: "/sandbox/tmp",
            }),
        }
    }

    /// Enforces the strict security context before launching a third-party process
    pub fn validate_filesystem_write(&self, path: &str) -> bool {
        let policy = self.current_policy.get();
        if !policy.allow_filesystem_write {
            // Check if within permitted directory path
            path.starts_with(policy.permitted_subpath)
        } else {
            true
        }
    }

    /// Verifies socket creation requests
    pub fn validate_network_socket(&self, is_raw: bool) -> bool {
        let policy = self.current_policy.get();
        if is_raw && !policy.allow_raw_sockets {
            false
        } else if !is_raw && !policy.allow_network {
            false
        } else {
            true
        }
    }
}

impl Default for AppSandboxEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 3. Forensic Write-Blocker Filter & Memory Wiper
// ==========================================

pub struct ForensicStorageFilter {
    pub is_write_blocked: Cell<bool>,
}

impl ForensicStorageFilter {
    pub const fn new() -> Self {
        Self {
            is_write_blocked: Cell::new(true), // Enabled by default to protect evidence
        }
    }

    /// Set write blocker toggle safely
    pub fn set_write_blocker(&self, enabled: bool) {
        self.is_write_blocked.set(enabled);
    }

    /// Intercepts device operations, granting read-only capabilities and blocking all writes
    pub fn intercept_device_write(&self, _sector_id: u64, _buffer: &[u8]) -> bool {
        !self.is_write_blocked.get()
    }

    /// Zeroes out secure regions of volatile memory to protect keys against hardware cold-boot analysis
    pub fn secure_memory_wipe(&self, target_buffer: &mut [u8]) {
        for byte in target_buffer.iter_mut() {
            // Write volatile zero states safely
            unsafe {
                core::ptr::write_volatile(byte, 0x00);
            }
        }
    }
}

impl Default for ForensicStorageFilter {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 4. MacChanger - OUI MAC Spoofing & Anonymizer
// ==========================================

pub struct MacChanger {
    pub current_mac: Cell<[u8; 6]>,
}

impl MacChanger {
    pub const fn new() -> Self {
        Self {
            current_mac: Cell::new([0x00, 0x0C, 0x29, 0xAB, 0xCD, 0xEF]), // Default VMware OUI
        }
    }

    /// Spoofs the MAC address to a randomized but format-compliant MAC address
    /// with an official OUI (Organizationally Unique Identifier).
    pub fn spoof_random_mac(&self, seed: u64) {
        let mut new_mac = [0u8; 6];
        let oui_choice = (seed % 3) as usize;
        let oui = match oui_choice {
            0 => [0x00, 0x0C, 0x29], // VMware
            1 => [0x00, 0x16, 0x3E], // Xen
            _ => [0x52, 0x54, 0x00], // QEMU
        };
        new_mac[0..3].copy_from_slice(&oui);

        let mut lcg = seed;
        for i in 3..6 {
            lcg = lcg.wrapping_mul(6364136223846793005).wrapping_add(1);
            new_mac[i] = (lcg >> 24) as u8;
        }

        new_mac[0] &= 0xFE; // Clear multicast bit
        new_mac[0] |= 0x02; // Set locally administered bit
        self.current_mac.set(new_mac);
    }
}

impl Default for MacChanger {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 5. KaliPacketSniffer - TCPDUMP/Wireshark Analyser
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketAnomaly {
    None,
    SynFlood,
    UnencryptedSensitives,
    SuspiciousPortAccess,
}

pub struct KaliPacketSniffer;

impl KaliPacketSniffer {
    pub const fn new() -> Self {
        Self
    }

    /// Parses raw network headers to inspect for security anomalies
    pub fn analyze_packet(&self, packet_bytes: &[u8]) -> PacketAnomaly {
        if packet_bytes.len() < 34 {
            return PacketAnomaly::None;
        }

        // IPv4 Header: Protocol sits at offset 9
        let protocol = packet_bytes[9];
        if protocol == 6 {
            // TCP
            let dest_port = ((packet_bytes[22] as u16) << 8) | (packet_bytes[23] as u16);
            let tcp_flags = packet_bytes[33];

            // TCP SYN flag is bit 0x02
            let is_syn = (tcp_flags & 0x02) != 0;
            let is_ack = (tcp_flags & 0x10) != 0;

            if is_syn && !is_ack && (dest_port == 22 || dest_port == 23 || dest_port == 445) {
                return PacketAnomaly::SuspiciousPortAccess;
            }

            // SYN-Flood heuristic (lots of SYN packets targeting HTTP)
            if is_syn && !is_ack && dest_port == 80 {
                return PacketAnomaly::SynFlood;
            }

            // Unencrypted payload check (e.g. USER/PASS keywords in FTP port 21, Telnet port 23)
            if dest_port == 80 || dest_port == 21 || dest_port == 23 {
                let payload = &packet_bytes[34..];
                // Check if subslice matches USER or PASS
                let mut found_sensitive = false;
                if payload.len() >= 5 {
                    for i in 0..=(payload.len() - 5) {
                        let word = &payload[i..i + 5];
                        if word == b"USER " || word == b"PASS " {
                            found_sensitive = true;
                            break;
                        }
                    }
                }
                if found_sensitive {
                    return PacketAnomaly::UnencryptedSensitives;
                }
            }
        }
        PacketAnomaly::None
    }
}

impl Default for KaliPacketSniffer {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 6. HashAuditor - john/hashcat-style weak credentials audits
// ==========================================

pub struct HashAuditor;

impl HashAuditor {
    pub const fn new() -> Self {
        Self
    }

    /// Simulates credentials auditing against wordlist using u32 hashes
    pub fn audit_weak_credentials(
        &self,
        password_hash: u32,
        wordlist: &[&'static str],
    ) -> Option<&'static str> {
        for &word in wordlist {
            // FNV-1a hash calculation
            let mut h = 2166136261u32;
            for byte in word.bytes() {
                h ^= byte as u32;
                h = h.wrapping_mul(16777619);
            }
            if h == password_hash {
                return Some(word);
            }
        }
        None
    }
}

impl Default for HashAuditor {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for AnonSurfShunt {}
unsafe impl Sync for AppSandboxEngine {}
unsafe impl Sync for ForensicStorageFilter {}
unsafe impl Sync for MacChanger {}
unsafe impl Sync for KaliPacketSniffer {}
unsafe impl Sync for HashAuditor {}

// ==========================================
// Global Static Security Orchestrators
// ==========================================

pub static GLOBAL_ANONSURF: AnonSurfShunt = AnonSurfShunt::new();
pub static GLOBAL_SANDBOX: AppSandboxEngine = AppSandboxEngine::new();
pub static GLOBAL_FORENSIC: ForensicStorageFilter = ForensicStorageFilter::new();
pub static GLOBAL_MACCHANGER: MacChanger = MacChanger::new();
pub static GLOBAL_SNIFFER: KaliPacketSniffer = KaliPacketSniffer::new();
pub static GLOBAL_AUDITOR: HashAuditor = HashAuditor::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonsurf_shunt() {
        let shunt = AnonSurfShunt::new();
        assert_eq!(shunt.current_mode.get(), RoutingMode::DirectCleartext);
        assert!(shunt.dns_leak_protection.get());

        shunt.enable_anonsurf();
        assert_eq!(shunt.current_mode.get(), RoutingMode::TorAnonymized);

        assert_eq!(shunt.anonymized_packets_routed.get(), 0);
        shunt.shunt_packet(101, 1024);
        assert_eq!(shunt.anonymized_packets_routed.get(), 1);

        shunt.disable_anonsurf();
        assert_eq!(shunt.current_mode.get(), RoutingMode::DirectCleartext);
    }

    #[test]
    fn test_app_sandbox_engine() {
        let engine = AppSandboxEngine::new();
        let default_policy = engine.current_policy.get();
        assert_eq!(default_policy.permitted_subpath, "/sandbox/tmp");

        assert!(engine.validate_filesystem_write("/sandbox/tmp/log.txt"));
        assert!(!engine.validate_filesystem_write("/etc/shadow"));

        assert!(!engine.validate_network_socket(false));
        assert!(!engine.validate_network_socket(true));

        engine.current_policy.set(SandboxPolicy {
            allow_network: true,
            allow_raw_sockets: false,
            allow_filesystem_write: false,
            permitted_subpath: "/sandbox/tmp",
        });
        assert!(engine.validate_network_socket(false));
        assert!(!engine.validate_network_socket(true));
    }

    #[test]
    fn test_forensic_storage_filter_and_wiper() {
        let filter = ForensicStorageFilter::new();
        assert!(filter.is_write_blocked.get());

        assert!(!filter.intercept_device_write(12, b"compromised data"));

        filter.set_write_blocker(false);
        assert!(!filter.is_write_blocked.get());
        assert!(filter.intercept_device_write(12, b"authorized forensics write"));

        let mut sensitive_data = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
        filter.secure_memory_wipe(&mut sensitive_data);
        assert_eq!(sensitive_data, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_mac_changer() {
        let changer = MacChanger::new();
        assert_eq!(
            changer.current_mac.get(),
            [0x00, 0x0C, 0x29, 0xAB, 0xCD, 0xEF]
        );

        changer.spoof_random_mac(42);
        let mac1 = changer.current_mac.get();
        assert_ne!(mac1, [0x00, 0x0C, 0x29, 0xAB, 0xCD, 0xEF]);
        assert_eq!(mac1[0] & 0x01, 0); // Is unicast
        assert_eq!(mac1[0] & 0x02, 2); // Is locally administered

        changer.spoof_random_mac(1337);
        let mac2 = changer.current_mac.get();
        assert_ne!(mac1, mac2);
    }

    #[test]
    fn test_kali_packet_sniffer() {
        let sniffer = KaliPacketSniffer::new();

        // Standard cleartext HTTP TCP packet with sensitive payload
        let mut packet = [0u8; 40];
        packet[9] = 6; // Protocol: TCP
        packet[22] = 0;
        packet[23] = 80; // Dest port: 80
        packet[33] = 0x10; // ACK flag

        assert_eq!(sniffer.analyze_packet(&packet), PacketAnomaly::None);

        // Inject "USER " in payload
        packet[34..39].copy_from_slice(b"USER ");
        assert_eq!(
            sniffer.analyze_packet(&packet),
            PacketAnomaly::UnencryptedSensitives
        );

        // SYN flood signature
        let mut syn_packet = [0u8; 35];
        syn_packet[9] = 6;
        syn_packet[22] = 0;
        syn_packet[23] = 80;
        syn_packet[33] = 0x02; // SYN flag
        assert_eq!(sniffer.analyze_packet(&syn_packet), PacketAnomaly::SynFlood);
    }

    #[test]
    fn test_hash_auditor() {
        let auditor = HashAuditor::new();
        let wordlist = ["admin", "root", "password", "123456"];

        // FNV-1a hash of "password" is 910909208u32
        let vulnerable_hash = 910909208u32;
        assert_eq!(
            auditor.audit_weak_credentials(vulnerable_hash, &wordlist),
            Some("password")
        );

        // Non-weak hash
        assert_eq!(auditor.audit_weak_credentials(11111111, &wordlist), None);
    }
}
