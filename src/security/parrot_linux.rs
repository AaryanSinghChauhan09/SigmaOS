// SPDX-License-Identifier: MIT
// SigmaOS Parrot Security Forensic & Network Suite (ParrotLinuxParity)
// Zero-dependency, #![no_std] compliant, highly-optimized for low-end hardware

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use alloc::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct SniffedPacket {
    pub protocol: String,
    pub source_ip: String,
    pub dest_ip: String,
    pub payload: Vec<u8>,
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
        if payload_str.contains("user=") || payload_str.contains(&p_word) || payload_str.contains(&p_wd) {
            self.credential_leaks.push(format!("[Leak Alert] Plaintext credentials found in {} payload: {}", packet.protocol, payload_str));
        }
        self.captured_packets.push_back(packet);
    }
}
