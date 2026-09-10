// SPDX-License-Identifier: MIT
// SigmaOS It's FOSS Inspired Tools Module (`src/tools/itsfoss_innovations.rs`)
// Zero-dependency `#![no_std]` high-performance Rust tools inspired by popular
// software featured on It's FOSS (Fastfetch, Stacer, Ventoy, Tldr, FastDownloader).

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// =========================================================================
// 1. SOVEREIGN FASTFETCH ENGINE (Inspired by Fastfetch / Neofetch / Hyfetch)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemSpecs {
    pub os_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub uptime_seconds: u64,
    pub shell: String,
    pub desktop_environment: String,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
}

pub struct SovereignFastfetchEngine {
    pub specs: SystemSpecs,
}

impl SovereignFastfetchEngine {
    pub fn new(specs: SystemSpecs) -> Self {
        Self { specs }
    }

    pub fn render_ansi_spec_card(&self) -> String {
        let mut card = String::new();
        card.push_str("\x1b[1;36m       .---.       \x1b[1;37m");
        card.push_str(&format!("{}@{}\n", self.specs.host_name, self.specs.os_name));

        card.push_str("\x1b[1;36m      /     \\      \x1b[0m-------------------------\n");
        card.push_str(&format!(
            "\x1b[1;36m     |  (\x1b[1;33mo\x1b[1;36m)  |     \x1b[1;32mOS:\x1b[0m {}\n",
            self.specs.os_name
        ));
        card.push_str(&format!(
            "\x1b[1;36m      \\     /      \x1b[1;32mKernel:\x1b[0m {}\n",
            self.specs.kernel_version
        ));
        card.push_str(&format!(
            "\x1b[1;36m       '---'       \x1b[1;32mUptime:\x1b[0m {} mins\n",
            self.specs.uptime_seconds / 60
        ));
        card.push_str(&format!(
            "                   \x1b[1;32mShell:\x1b[0m {}\n",
            self.specs.shell
        ));
        card.push_str(&format!(
            "                   \x1b[1;32mDE:\x1b[0m {}\n",
            self.specs.desktop_environment
        ));
        card.push_str(&format!(
            "                   \x1b[1;32mMemory:\x1b[0m {} MiB / {} MiB\n",
            self.specs.memory_used_mb, self.specs.memory_total_mb
        ));

        card
    }
}

// =========================================================================
// 2. SOVEREIGN STACER CLEANER ENGINE (Inspired by Stacer & BleachBit)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanableCategory {
    pub name: String,
    pub path: String,
    pub reclaimable_bytes: u64,
}

pub struct SovereignStacerCleanerEngine {
    pub categories: Vec<CleanableCategory>,
}

impl SovereignStacerCleanerEngine {
    pub fn new() -> Self {
        Self {
            categories: vec![
                CleanableCategory {
                    name: "Package Cache".to_string(),
                    path: "/var/cache/pkg".to_string(),
                    reclaimable_bytes: 1024 * 1024 * 350, // 350 MB
                },
                CleanableCategory {
                    name: "Crash Logs".to_string(),
                    path: "/var/log/crash".to_string(),
                    reclaimable_bytes: 1024 * 1024 * 85, // 85 MB
                },
                CleanableCategory {
                    name: "Application Caches".to_string(),
                    path: "/home/user/.cache".to_string(),
                    reclaimable_bytes: 1024 * 1024 * 1200, // 1.2 GB
                },
            ],
        }
    }

    pub fn scan_total_junk_bytes(&self) -> u64 {
        self.categories.iter().map(|c| c.reclaimable_bytes).sum()
    }

    pub fn execute_clean(&mut self) -> u64 {
        let total = self.scan_total_junk_bytes();
        for c in &mut self.categories {
            c.reclaimable_bytes = 0;
        }
        total
    }
}

impl Default for SovereignStacerCleanerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. SOVEREIGN VENTOY MULTIBOOT ENGINE (Inspired by Ventoy & Etcher)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VentoyIsoEntry {
    pub name: String,
    pub file_size_bytes: u64,
    pub boot_mode: String, // "UEFI", "Legacy", "Dual"
    pub persistence_file: Option<String>,
}

pub struct SovereignVentoyMultibootEngine {
    pub target_device: String,
    pub partition_scheme: String, // "GPT" / "MBR"
    pub registered_isos: Vec<VentoyIsoEntry>,
}

impl SovereignVentoyMultibootEngine {
    pub fn new(device: &str) -> Self {
        Self {
            target_device: device.to_string(),
            partition_scheme: "GPT".to_string(),
            registered_isos: Vec::new(),
        }
    }

    pub fn add_iso_image(&mut self, iso: VentoyIsoEntry) {
        self.registered_isos.push(iso);
    }

    pub fn generate_grub_multiboot_cfg(&self) -> String {
        let mut cfg = String::from("# Sovereign Ventoy Multiboot GRUB Config\n");
        cfg.push_str("set timeout=10\n");

        for (idx, iso) in self.registered_isos.iter().enumerate() {
            cfg.push_str(&format!(
                "menuentry '{} [{}]' {{\n",
                iso.name, iso.boot_mode
            ));
            cfg.push_str(&format!("    set isofile=\"/ISO/{}\"\n", iso.name));
            cfg.push_str("    loopback loop $isofile\n");
            cfg.push_str("    linux (loop)/boot/vmlinuz boot=live iso-scan/filename=$isofile\n");
            cfg.push_str("    initrd (loop)/boot/initrd.img\n");
            cfg.push_str("}\n\n");
            let _ = idx;
        }

        cfg
    }
}

// =========================================================================
// 4. SOVEREIGN TLDR CHEAT SHEET ENGINE (Inspired by tldr & cheat.sh)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandExample {
    pub description: String,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TldrPage {
    pub command_name: String,
    pub summary: String,
    pub examples: Vec<CommandExample>,
}

pub struct SovereignTldrCheatSheetEngine {
    pub database: Vec<TldrPage>,
}

impl SovereignTldrCheatSheetEngine {
    pub fn new() -> Self {
        Self {
            database: vec![
                TldrPage {
                    command_name: "tar".to_string(),
                    summary: "Archiving utility".to_string(),
                    examples: vec![
                        CommandExample {
                            description: "Create an archive from files:".to_string(),
                            command: "tar -cvf archive.tar file1 file2".to_string(),
                        },
                        CommandExample {
                            description: "Extract an archive to target directory:".to_string(),
                            command: "tar -xvf archive.tar -C /target".to_string(),
                        },
                    ],
                },
                TldrPage {
                    command_name: "git".to_string(),
                    summary: "Version control system".to_string(),
                    examples: vec![CommandExample {
                        description: "Clone a repository with depth 1:".to_string(),
                        command: "git clone --depth 1 <url>".to_string(),
                    }],
                },
            ],
        }
    }

    pub fn query_cheat_sheet(&self, cmd: &str) -> Option<&TldrPage> {
        self.database.iter().find(|p| p.command_name == cmd)
    }

    pub fn render_page(&self, cmd: &str) -> Option<String> {
        let page = self.query_cheat_sheet(cmd)?;
        let mut out = format!("\x1b[1;32m# {}\x1b[0m\n", page.command_name);
        out.push_str(&format!("> {}\n\n", page.summary));

        for ex in &page.examples {
            out.push_str(&format!("- {}\n", ex.description));
            out.push_str(&format!("  \x1b[1;36m{}\x1b[0m\n\n", ex.command));
        }
        Some(out)
    }
}

impl Default for SovereignTldrCheatSheetEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. SOVEREIGN FAST DOWNLOADER ENGINE (Inspired by Axel / Aria2 / Gdown)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadChunk {
    pub chunk_id: usize,
    pub start_byte: u64,
    pub end_byte: u64,
    pub downloaded_bytes: u64,
    pub completed: bool,
}

pub struct SovereignFastDownloaderEngine {
    pub url: String,
    pub total_size_bytes: u64,
    pub chunks: Vec<DownloadChunk>,
}

impl SovereignFastDownloaderEngine {
    pub fn new(url: &str, total_size: u64, num_connections: usize) -> Self {
        let num = num_connections.max(1);
        let chunk_size = total_size / (num as u64);
        let mut chunks = Vec::new();

        for i in 0..num {
            let start = i as u64 * chunk_size;
            let end = if i == num - 1 {
                total_size - 1
            } else {
                (i as u64 + 1) * chunk_size - 1
            };

            chunks.push(DownloadChunk {
                chunk_id: i + 1,
                start_byte: start,
                end_byte: end,
                downloaded_bytes: 0,
                completed: false,
            });
        }

        Self {
            url: url.to_string(),
            total_size_bytes: total_size,
            chunks,
        }
    }

    pub fn simulate_chunk_progress(&mut self, chunk_id: usize, bytes: u64) -> bool {
        if let Some(chunk) = self.chunks.iter_mut().find(|c| c.chunk_id == chunk_id) {
            chunk.downloaded_bytes = (chunk.downloaded_bytes + bytes).min(chunk.end_byte - chunk.start_byte + 1);
            if chunk.downloaded_bytes == (chunk.end_byte - chunk.start_byte + 1) {
                chunk.completed = true;
            }
            true
        } else {
            false
        }
    }

    pub fn get_overall_progress_pct(&self) -> f64 {
        if self.total_size_bytes == 0 {
            return 100.0;
        }
        let total_downloaded: u64 = self.chunks.iter().map(|c| c.downloaded_bytes).sum();
        (total_downloaded as f64 / self.total_size_bytes as f64) * 100.0
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fastfetch_engine() {
        let specs = SystemSpecs {
            os_name: "SigmaOS".to_string(),
            host_name: "sovereign-node".to_string(),
            kernel_version: "1.0.0-pqc".to_string(),
            uptime_seconds: 3600,
            shell: "sigma-shell".to_string(),
            desktop_environment: "Zenith-Desktop".to_string(),
            memory_used_mb: 4096,
            memory_total_mb: 16384,
        };
        let fetch = SovereignFastfetchEngine::new(specs);
        let card = fetch.render_ansi_spec_card();

        assert!(card.contains("sovereign-node@SigmaOS"));
        assert!(card.contains("1.0.0-pqc"));
        assert!(card.contains("4096 MiB / 16384 MiB"));
    }

    #[test]
    fn test_stacer_cleaner_engine() {
        let mut cleaner = SovereignStacerCleanerEngine::new();
        let junk_bytes = cleaner.scan_total_junk_bytes();
        assert!(junk_bytes > 0);

        let cleaned = cleaner.execute_clean();
        assert_eq!(cleaned, junk_bytes);
        assert_eq!(cleaner.scan_total_junk_bytes(), 0);
    }

    #[test]
    fn test_ventoy_multiboot_engine() {
        let mut ventoy = SovereignVentoyMultibootEngine::new("/dev/sdb");
        ventoy.add_iso_image(VentoyIsoEntry {
            name: "SigmaOS-Live-1.0.iso".to_string(),
            file_size_bytes: 2 * 1024 * 1024 * 1024,
            boot_mode: "Dual".to_string(),
            persistence_file: Some("/ISO/persistence.dat".to_string()),
        });

        let grub_cfg = ventoy.generate_grub_multiboot_cfg();
        assert!(grub_cfg.contains("SigmaOS-Live-1.0.iso"));
        assert!(grub_cfg.contains("loopback loop $isofile"));
    }

    #[test]
    fn test_tldr_cheat_sheet_engine() {
        let tldr = SovereignTldrCheatSheetEngine::new();
        let page = tldr.query_cheat_sheet("tar");
        assert!(page.is_some());
        assert_eq!(page.unwrap().command_name, "tar");

        let rendered = tldr.render_page("tar").unwrap();
        assert!(rendered.contains("Archiving utility"));
        assert!(rendered.contains("tar -cvf archive.tar"));

        assert!(tldr.render_page("nonexistent").is_none());
    }

    #[test]
    fn test_fast_downloader_engine() {
        let mut dl = SovereignFastDownloaderEngine::new("https://releases.sigmaos.org/iso/latest.iso", 1000, 4);
        assert_eq!(dl.chunks.len(), 4);
        assert_eq!(dl.get_overall_progress_pct(), 0.0);

        assert!(dl.simulate_chunk_progress(1, 250));
        assert_eq!(dl.get_overall_progress_pct(), 25.0);

        assert!(dl.simulate_chunk_progress(2, 250));
        assert!(dl.simulate_chunk_progress(3, 250));
        assert!(dl.simulate_chunk_progress(4, 250));
        assert_eq!(dl.get_overall_progress_pct(), 100.0);
    }
}
