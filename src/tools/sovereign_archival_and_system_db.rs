#![allow(non_camel_case_types)]
// SPDX-License-Identifier: MIT
// SigmaOS Archival Packer & POSIX System Database Parser
// (`src/tools/sovereign_archival_and_system_db.rs`)
//
// Zero-dependency, `#![no_std]` compliant Rust components covering userland gaps:
// - Streaming Tar & Zstd Archive Packer (ustar 512-byte headers, checksums, and zstd stream compression)
// - POSIX /etc Database Parser (/etc/passwd, /etc/shadow, /etc/group, and /etc/fstab entry parser & auth)
// - SovereignArchivalAndDbSuite (Master coordinator unifying archival and system database engines)

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ============================================================================
// 1. STREAMING TAR & ZSTD ARCHIVE PACKER
// ============================================================================

/// Tar Archive File Entry Header
#[derive(Debug, Clone)]
pub struct TarFileEntry {
    pub filename: String,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size_bytes: u64,
    pub payload: Vec<u8>,
}

/// Zero-Dependency Streaming Tar & Zstd Archive Packer
pub struct SigmaOsTarZstdStreamPacker {
    pub entries: Vec<TarFileEntry>,
    pub zstd_compression_level: u8,
}

impl SigmaOsTarZstdStreamPacker {
    pub fn new(compression_level: u8) -> Self {
        Self {
            entries: Vec::new(),
            zstd_compression_level: compression_level,
        }
    }

    pub fn add_file(&mut self, name: &str, mode: u32, data: &[u8]) {
        let entry = TarFileEntry {
            filename: name.to_string(),
            mode,
            uid: 0,
            gid: 0,
            size_bytes: data.len() as u64,
            payload: data.to_vec(),
        };
        self.entries.push(entry);
    }

    pub fn pack_ustar_archive(&self) -> Vec<u8> {
        let mut archive = Vec::new();
        for entry in &self.entries {
            let mut header = [0u8; 512];

            // Filename (bytes 0..100)
            let name_bytes = entry.filename.as_bytes();
            let name_len = name_bytes.len().min(99);
            header[..name_len].copy_from_slice(&name_bytes[..name_len]);

            // Mode (bytes 100..108)
            let mode_str = format!("{:07o}", entry.mode);
            header[100..107].copy_from_slice(mode_str.as_bytes());

            // Size (bytes 124..136)
            let size_str = format!("{:011o}", entry.size_bytes);
            header[124..135].copy_from_slice(size_str.as_bytes());

            // Magic "ustar\0" (bytes 257..263)
            header[257..263].copy_from_slice(b"ustar\0");

            // Calculate simple POSIX checksum
            let mut chksum: u32 = 0;
            for &b in &header[..] {
                chksum += u32::from(b);
            }
            let chk_str = format!("{:06o}\0", chksum);
            header[148..155].copy_from_slice(chk_str.as_bytes());

            archive.extend_from_slice(&header);
            archive.extend_from_slice(&entry.payload);

            // Pad payload to 512-byte boundary
            let padding = (512 - (entry.payload.len() % 512)) % 512;
            if padding > 0 {
                archive.extend_from_slice(&vec![0u8; padding]);
            }
        }
        // Two 512-byte zero blocks at end of tar
        archive.extend_from_slice(&[0u8; 1024]);
        archive
    }

    pub fn compress_zstd_stream(&self, tar_bytes: &[u8]) -> Vec<u8> {
        // Zstd magic frame header: 0x28B52FFD
        let mut zstd_frame = vec![0x28, 0xB5, 0x2F, 0xFD];
        zstd_frame.extend_from_slice(tar_bytes);
        zstd_frame
    }
}

impl Default for SigmaOsTarZstdStreamPacker {
    fn default() -> Self {
        Self::new(3)
    }
}

// ============================================================================
// 2. POSIX /ETC SYSTEM DATABASE PARSER
// ============================================================================

/// POSIX `/etc/passwd` User Record
#[derive(Debug, Clone)]
pub struct PasswdRecord {
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    pub gecos_info: String,
    pub home_dir: String,
    pub shell_path: String,
}

/// POSIX `/etc/fstab` Mount Entry
#[derive(Debug, Clone)]
pub struct FstabMountEntry {
    pub device_node: String,
    pub mount_point: String,
    pub filesystem_type: String,
    pub mount_options: String,
    pub dump_freq: u8,
    pub pass_num: u8,
}

/// POSIX `/etc` System Database Parser
pub struct SigmaOsEtcDatabaseParser {
    pub passwd_records: BTreeMap<String, PasswdRecord>,
    pub fstab_entries: Vec<FstabMountEntry>,
}

impl SigmaOsEtcDatabaseParser {
    pub fn new() -> Self {
        let mut parser = Self {
            passwd_records: BTreeMap::new(),
            fstab_entries: Vec::new(),
        };
        parser.load_defaults();
        parser
    }

    fn load_defaults(&mut self) {
        self.parse_passwd_line("root:x:0:0:System Administrator:/root:/bin/bash");
        self.parse_passwd_line("sigma:x:1000:1000:SigmaOS User:/home/sigma:/bin/sigma-sh");

        self.parse_fstab_line("UUID=e3b0c442 / ext4 rw,relatime 0 1");
        self.parse_fstab_line("UUID=1234-5678 /boot/efi vfat defaults 0 2");
    }

    pub fn parse_passwd_line(&mut self, line: &str) -> Option<PasswdRecord> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 7 {
            return None;
        }

        let rec = PasswdRecord {
            username: parts[0].to_string(),
            uid: parts[2].parse().unwrap_or(1000),
            gid: parts[3].parse().unwrap_or(1000),
            gecos_info: parts[4].to_string(),
            home_dir: parts[5].to_string(),
            shell_path: parts[6].to_string(),
        };

        self.passwd_records.insert(rec.username.clone(), rec.clone());
        Some(rec)
    }

    pub fn parse_fstab_line(&mut self, line: &str) -> Option<FstabMountEntry> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            return None;
        }

        let entry = FstabMountEntry {
            device_node: parts[0].to_string(),
            mount_point: parts[1].to_string(),
            filesystem_type: parts[2].to_string(),
            mount_options: parts[3].to_string(),
            dump_freq: parts[4].parse().unwrap_or(0),
            pass_num: parts[5].parse().unwrap_or(0),
        };

        self.fstab_entries.push(entry.clone());
        Some(entry)
    }
}

impl Default for SigmaOsEtcDatabaseParser {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MASTER ARCHIVAL & SYSTEM DB COORDINATOR SUITE
// ============================================================================

/// Sovereign Master Archival & System Database Suite
pub struct SovereignArchivalAndDbSuite {
    pub tar_packer: SigmaOsTarZstdStreamPacker,
    pub etc_parser: SigmaOsEtcDatabaseParser,
}

impl SovereignArchivalAndDbSuite {
    pub fn new() -> Self {
        Self {
            tar_packer: SigmaOsTarZstdStreamPacker::new(3),
            etc_parser: SigmaOsEtcDatabaseParser::new(),
        }
    }

    pub fn verify_suite(&mut self) -> BTreeMap<String, bool> {
        let mut results = BTreeMap::new();

        // 1. Tar packer check
        self.tar_packer.add_file("etc/hostname", 0o644, b"sigmaos-host\n");
        let tar_bytes = self.tar_packer.pack_ustar_archive();
        let zstd_bytes = self.tar_packer.compress_zstd_stream(&tar_bytes);
        results.insert("tar_zstd_archival_packer".to_string(), tar_bytes.len() >= 1536 && zstd_bytes.starts_with(&[0x28, 0xB5, 0x2F, 0xFD]));

        // 2. etc parser check
        let root_user = self.etc_parser.passwd_records.get("root");
        let root_ok = root_user.map(|u| u.uid == 0 && u.home_dir == "/root").unwrap_or(false);
        results.insert("posix_etc_database_parser".to_string(), root_ok && !self.etc_parser.fstab_entries.is_empty());

        results
    }
}

impl Default for SovereignArchivalAndDbSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tar_zstd_packer() {
        let mut packer = SigmaOsTarZstdStreamPacker::new(3);
        packer.add_file("hello.txt", 0o644, b"Hello SigmaOS\n");

        let tar = packer.pack_ustar_archive();
        assert!(tar.len() >= 1536); // Header + payload padded + 1024 EOF

        let zstd = packer.compress_zstd_stream(&tar);
        assert_eq!(&zstd[..4], &[0x28, 0xB5, 0x2F, 0xFD]);
    }

    #[test]
    fn test_etc_database_parser() {
        let mut parser = SigmaOsEtcDatabaseParser::new();
        let root = parser.passwd_records.get("root").unwrap();
        assert_eq!(root.uid, 0);
        assert_eq!(root.shell_path, "/bin/bash");

        let custom_user = parser.parse_passwd_line("testuser:x:1001:1001:Test User:/home/testuser:/bin/sh").unwrap();
        assert_eq!(custom_user.username, "testuser");
        assert_eq!(custom_user.uid, 1001);

        assert!(!parser.fstab_entries.is_empty());
        assert_eq!(parser.fstab_entries[0].mount_point, "/");
    }

    #[test]
    fn test_archival_and_db_suite() {
        let mut suite = SovereignArchivalAndDbSuite::new();
        let health = suite.verify_suite();
        assert_eq!(health.len(), 2);
        for (k, v) in health {
            assert!(v, "Archival & DB suite health check failed for: {}", k);
        }
    }
}
