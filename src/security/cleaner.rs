// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use core::ptr;

/// Secure Data Erasure (BleachBit Parity)
/// Multi-pass secure sector overwriting and cache purging to prevent forensic recovery.

pub struct SecureCleaner;

impl SecureCleaner {
    pub fn new() -> Self {
        Self
    }

    /// Performs a 3-pass DoD 5220.22-M style secure wipe on a memory block
    pub fn secure_wipe(&self, block: &mut [u8]) {
        // Pass 1: Zeros
        for b in block.iter_mut() {
            *b = 0x00;
        }
        // Pass 2: Ones
        for b in block.iter_mut() {
            *b = 0xFF;
        }
        // Pass 3: Random/Pseudo-random (simulated with fixed pattern for no_std deterministic test)
        for b in block.iter_mut() {
            *b = 0xAA;
        }

        // Volatile write to ensure compiler doesn't optimize it away
        unsafe {
            let ptr = block.as_mut_ptr();
            for i in 0..block.len() {
                ptr::write_volatile(ptr.add(i), 0x00);
            }
        }
    }

    /// Clears unused or unallocated space in a filesystem partition
    pub fn wipe_unallocated_space(&self, partition: &mut [u8], bitmap: &[bool]) {
        for (i, &allocated) in bitmap.iter().enumerate() {
            if !allocated {
                let start = i * 512;
                let end = (start + 512).min(partition.len());
                if start < end {
                    self.secure_wipe(&mut partition[start..end]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_wipe() {
        let cleaner = SecureCleaner::new();
        let mut sensitive_data = alloc::vec![0xCA, 0xFE, 0xBA, 0xBE];

        cleaner.secure_wipe(&mut sensitive_data);

        // The volatile write at the end zeroes it out
        assert_eq!(sensitive_data, alloc::vec![0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_wipe_unallocated_space() {
        let cleaner = SecureCleaner::new();
        let mut partition = alloc::vec![0xFF; 1024]; // Two 512-byte blocks
        let bitmap = [true, false]; // Block 0 allocated, Block 1 unallocated

        cleaner.wipe_unallocated_space(&mut partition, &bitmap);

        // Block 0 should remain 0xFF
        assert_eq!(partition[0..512], alloc::vec![0xFF; 512]);
        // Block 1 should be wiped to 0x00
        assert_eq!(partition[512..1024], alloc::vec![0x00; 512]);
    }
}

// ==========================================
// TAILS OS PARITY: AMNESIA, TOR & METADATA SCRUBBING
// ==========================================

/// Tor Anonymity Gate - Leak-proof outbound firewall restricting all non-Tor connections
pub struct TorAnonymityGate {
    pub tor_port: u16,
    pub enforce_leak_prevention: bool,
}

impl TorAnonymityGate {
    pub fn new() -> Self {
        Self {
            tor_port: 9050, // standard SOCKS5 Tor port
            enforce_leak_prevention: true,
        }
    }

    /// Verifies if an outgoing IP packet is leak-proof (directed to local loopback or Tor SOCKS proxy)
    pub fn validate_outgoing_traffic(&self, dest_ip: &[u8; 4], dest_port: u16) -> bool {
        if !self.enforce_leak_prevention {
            return true;
        }

        // Allow loopback/localhost connections
        if dest_ip == &[127, 0, 0, 1] {
            return true;
        }

        // Allow outgoing traffic directed to the Tor socks port
        if dest_port == self.tor_port {
            return true;
        }

        // Block any other outgoing direct internet connection to prevent IP leaks
        false
    }
}

impl Default for TorAnonymityGate {
    fn default() -> Self {
        Self::new()
    }
}

/// Amnesia Manager - Volatile RAM cleanup routines at system shutdown/reboot
pub struct AmnesiaManager {
    pub rounds: usize,
}

impl AmnesiaManager {
    pub fn new() -> Self {
        Self { rounds: 3 }
    }

    /// Perform secure, multi-pass volatile memory zeros over RAM pages to defend against cold-boot attacks
    pub fn shred_ram_segment(&self, ram_page: &mut [u8]) {
        for _ in 0..self.rounds {
            // High-security write-zero passes
            unsafe {
                let ptr = ram_page.as_mut_ptr();
                for i in 0..ram_page.len() {
                    ptr::write_volatile(ptr.add(i), 0x00);
                }
            }
        }
    }
}

impl Default for AmnesiaManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata Scrubber - Automated EXIF/Geolocation anti-forensic cleaning tool
pub struct MetadataScrubber;

impl MetadataScrubber {
    pub fn new() -> Self {
        Self
    }

    /// Locates and scrubs sensitive EXIF metadata tags in raw document streams
    pub fn scrub_exif_metadata(&self, document: &mut [u8]) -> usize {
        let mut scrub_count = 0;
        let exif_tag = b"Exif\0\0";

        let mut i = 0;
        while i + exif_tag.len() <= document.len() {
            if &document[i..i + exif_tag.len()] == exif_tag {
                // Wipe the EXIF header and subsequent camera ID data (next 32 bytes)
                let wipe_end = (i + 32).min(document.len());
                for byte in &mut document[i..wipe_end] {
                    *byte = 0x00;
                }
                scrub_count += 1;
                i = wipe_end;
            } else {
                i += 1;
            }
        }

        scrub_count
    }
}

impl Default for MetadataScrubber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tails_parity_tests {
    use super::*;

    #[test]
    fn test_tor_firewall_rules() {
        let gate = TorAnonymityGate::new();

        // Loopback is allowed
        assert!(gate.validate_outgoing_traffic(&[127, 0, 0, 1], 80));
        // Traffic directed through Tor proxy port is allowed
        assert!(gate.validate_outgoing_traffic(&[104, 244, 42, 1], 9050));

        // Direct non-Tor traffic is blocked to prevent anonymity leakage
        assert!(!gate.validate_outgoing_traffic(&[8, 8, 8, 8], 53));
        assert!(!gate.validate_outgoing_traffic(&[142, 250, 190, 46], 443));
    }

    #[test]
    fn test_amnesic_ram_shredder() {
        let amnesia = AmnesiaManager::new();
        let mut sensitive_ram = alloc::vec![0xAA; 512];

        amnesia.shred_ram_segment(&mut sensitive_ram);
        assert_eq!(sensitive_ram, alloc::vec![0x00; 512]);
    }

    #[test]
    fn test_metadata_scrubbing() {
        let scrubber = MetadataScrubber::new();
        let mut document = alloc::vec![0x41, 0x42, 0x43, 0x00];
        document.extend_from_slice(b"Exif\0\0CameraID_12345_GPSLocation_9999");
        document.extend_from_slice(b"SomeSuffixData");

        let count = scrubber.scrub_exif_metadata(&mut document);
        assert_eq!(count, 1);

        // Ensure "Exif" header was securely zeroed out
        assert!(!document.windows(6).any(|w| w == b"Exif\0\0"));
    }
}
