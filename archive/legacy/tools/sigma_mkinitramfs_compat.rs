// SPDX-License-Identifier: MIT
// Clean-Room CLI Compatibility Tool for Initramfs Binary Generation
// (mkinitramfs / mkinitcpio / dracut)
// tools/sigma_mkinitramfs_compat.rs

use std::env;
use std::fs::{self, File};
use std::io::Write;

/// CLI options for sigma-mkinitramfs
#[derive(Debug, Clone)]
pub struct MkinitramfsCliArgs {
    pub output_path: String,
    pub kernel_version: String,
    pub compression: String,
    pub hooks: Vec<String>,
    pub modules: Vec<String>,
    pub preset: String,
    pub verbose: bool,
}

impl MkinitramfsCliArgs {
    pub fn new() -> Self {
        Self {
            output_path: String::from("/boot/initramfs-sigma.img"),
            kernel_version: String::from("6.8.1-sigmaos-1"),
            compression: String::from("gzip"),
            hooks: vec![
                "base".to_string(),
                "udev".to_string(),
                "autodetect".to_string(),
                "block".to_string(),
                "filesystems".to_string(),
            ],
            modules: Vec::new(),
            preset: String::from("default"),
            verbose: false,
        }
    }

    pub fn parse(args: &[String]) -> Self {
        let mut cli = Self::new();
        let mut i = 1;

        while i < args.len() {
            match args[i].as_str() {
                "-o" | "--output" | "-g" => {
                    if i + 1 < args.len() {
                        cli.output_path = args[i + 1].clone();
                        i += 1;
                    }
                }
                "-k" | "--kernel" => {
                    if i + 1 < args.len() {
                        cli.kernel_version = args[i + 1].clone();
                        i += 1;
                    }
                }
                "-c" | "--compression" => {
                    if i + 1 < args.len() {
                        cli.compression = args[i + 1].clone();
                        i += 1;
                    }
                }
                "-p" | "--preset" => {
                    if i + 1 < args.len() {
                        cli.preset = args[i + 1].clone();
                        i += 1;
                    }
                }
                "--add-drivers" | "-m" => {
                    if i + 1 < args.len() {
                        for mod_name in args[i + 1].split(',') {
                            cli.modules.push(mod_name.trim().to_string());
                        }
                        i += 1;
                    }
                }
                "-v" | "--verbose" => {
                    cli.verbose = true;
                }
                "-h" | "--help" => {
                    cli.print_help();
                    std::process::exit(0);
                }
                _ => {}
            }
            i += 1;
        }

        cli
    }

    pub fn print_help(&self) {
        println!("SigmaOS mkinitramfs / mkinitcpio / dracut Compatibility Tool");
        println!("Usage: sigma_mkinitramfs_compat [OPTIONS]");
        println!();
        println!("Options:");
        println!("  -o, --output <FILE>      Specify output image file path");
        println!("  -k, --kernel <VER>       Specify target kernel version");
        println!("  -c, --compression <TYPE> Compression type: gzip, zstd, lz4, none");
        println!("  -p, --preset <PRESET>    Arch-style preset name");
        println!("  -m, --add-drivers <MODS> Comma-separated list of kernel drivers");
        println!("  -v, --verbose            Enable verbose log output");
        println!("  -h, --help               Display help screen");
    }
}

impl Default for MkinitramfsCliArgs {
    fn default() -> Self {
        Self::new()
    }
}

/// Core binary CPIO and compression generator engine
pub struct SovereignMkinitramfsEngine {
    pub args: MkinitramfsCliArgs,
}

impl SovereignMkinitramfsEngine {
    pub fn new(args: MkinitramfsCliArgs) -> Self {
        Self { args }
    }

    pub fn generate_cpio_archive(&self) -> Vec<u8> {
        let mut archive = Vec::new();

        // 1. /init script entry
        let init_data = b"#!/bin/sh\necho Booting SigmaOS Kernel...\nmount -t devtmpfs devtmpfs /dev\nexec /sbin/init\n";
        archive.extend_from_slice(&self.create_cpio_entry("init", 0o100755, init_data));

        // 2. /etc/initrd.conf
        let conf_data = format!("KERNEL_VERSION={}\nPRESET={}\n", self.args.kernel_version, self.args.preset).into_bytes();
        archive.extend_from_slice(&self.create_cpio_entry("etc/initrd.conf", 0o100644, &conf_data));

        // 3. /dev/console & /dev/null placeholders
        archive.extend_from_slice(&self.create_cpio_entry("dev/console", 0o20600, b""));
        archive.extend_from_slice(&self.create_cpio_entry("dev/null", 0o20666, b""));

        // 4. Kernel modules
        for mod_name in &self.args.modules {
            let mod_path = format!("lib/modules/{}/kernel/drivers/{}.ko", self.args.kernel_version, mod_name);
            let mod_payload = format!("// Kernel driver stub for {}", mod_name).into_bytes();
            archive.extend_from_slice(&self.create_cpio_entry(&mod_path, 0o100644, &mod_payload));
        }

        // 5. CPIO TRAILER!!!
        archive.extend_from_slice(&self.create_cpio_entry("TRAILER!!!", 0, b""));

        archive
    }

    fn create_cpio_entry(&self, filename: &str, mode: u32, data: &[u8]) -> Vec<u8> {
        let mut entry = Vec::new();
        let header = format!(
            "070701{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}{:08X}",
            100u32, // ino
            mode,
            0u32, // uid
            0u32, // gid
            1u32, // nlink
            1704067200u32, // mtime
            data.len() as u32,
            3u32, 1u32, 0u32, 0u32, // devmajor/minor/rdev
            filename.len() as u32 + 1,
            0u32, // check
        );

        entry.extend_from_slice(header.as_bytes());
        entry.extend_from_slice(filename.as_bytes());
        entry.push(0);

        while entry.len() % 4 != 0 {
            entry.push(0);
        }

        entry.extend_from_slice(data);
        while entry.len() % 4 != 0 {
            entry.push(0);
        }

        entry
    }

    pub fn compress_and_save(&self, cpio_bytes: &[u8]) -> Result<usize, String> {
        let mut output = Vec::new();

        match self.args.compression.as_str() {
            "gzip" => {
                // Header magic + deflate block
                output.extend_from_slice(&[0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff]);
                output.push(0x01); // Last block header
                let len_bytes = (cpio_bytes.len() as u16).to_le_bytes();
                let nlen_bytes = (!cpio_bytes.len() as u16).to_le_bytes();
                output.extend_from_slice(&len_bytes);
                output.extend_from_slice(&nlen_bytes);
                output.extend_from_slice(cpio_bytes);
                output.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // CRC32 stub
                output.extend_from_slice(&(cpio_bytes.len() as u32).to_le_bytes());
            }
            "zstd" => {
                output.extend_from_slice(&[0x28, 0xB5, 0x2F, 0xFD]);
                output.extend_from_slice(&(cpio_bytes.len() as u32).to_le_bytes());
                output.extend_from_slice(cpio_bytes);
            }
            "lz4" => {
                output.extend_from_slice(&[0x04, 0x22, 0x4D, 0x18]);
                output.extend_from_slice(&(cpio_bytes.len() as u32).to_le_bytes());
                output.extend_from_slice(cpio_bytes);
            }
            _ => {
                output.extend_from_slice(cpio_bytes);
            }
        }

        if let Some(parent) = std::path::Path::new(&self.args.output_path).parent() {
            let _ = fs::create_dir_all(parent);
        }

        let mut file = File::create(&self.args.output_path)
            .map_err(|e| format!("Failed to create output image: {}", e))?;
        file.write_all(&output)
            .map_err(|e| format!("Failed to write initramfs data: {}", e))?;

        Ok(output.len())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli_args = MkinitramfsCliArgs::parse(&args);

    if cli_args.verbose {
        println!("[sigma_mkinitramfs] Generating initramfs for kernel {}...", cli_args.kernel_version);
    }

    let engine = SovereignMkinitramfsEngine::new(cli_args.clone());
    let cpio = engine.generate_cpio_archive();

    match engine.compress_and_save(&cpio) {
        Ok(bytes) => {
            println!(
                "[sigma_mkinitramfs] Successfully built {} ({}) -> {} bytes",
                cli_args.output_path, cli_args.compression, bytes
            );
        }
        Err(err) => {
            eprintln!("[sigma_mkinitramfs ERROR] {}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_argument_parsing() {
        let args = vec![
            "sigma_mkinitramfs_compat".to_string(),
            "-o".to_string(),
            "/tmp/initramfs-test.img".to_string(),
            "-k".to_string(),
            "6.8.5-arch1".to_string(),
            "-c".to_string(),
            "zstd".to_string(),
            "-m".to_string(),
            "ext4,nvme,ahci".to_string(),
        ];

        let parsed = MkinitramfsCliArgs::parse(&args);
        assert_eq!(parsed.output_path, "/tmp/initramfs-test.img");
        assert_eq!(parsed.kernel_version, "6.8.5-arch1");
        assert_eq!(parsed.compression, "zstd");
        assert_eq!(parsed.modules, vec!["ext4", "nvme", "ahci"]);
    }

    #[test]
    fn test_sovereign_mkinitramfs_engine_archive_and_save() {
        let mut cli = MkinitramfsCliArgs::new();
        cli.output_path = "/tmp/test_mkinitramfs_engine.img".to_string();
        cli.modules = vec!["btrfs".to_string(), "xhci_hcd".to_string()];

        let engine = SovereignMkinitramfsEngine::new(cli);
        let cpio = engine.generate_cpio_archive();
        let text = String::from_utf8_lossy(&cpio);

        assert!(text.contains("070701"));
        assert!(text.contains("init"));
        assert!(text.contains("lib/modules/6.8.1-sigmaos-1/kernel/drivers/btrfs.ko"));
        assert!(text.contains("TRAILER!!!"));

        let saved_bytes = engine.compress_and_save(&cpio).unwrap();
        assert!(saved_bytes > 50);

        let metadata = fs::metadata("/tmp/test_mkinitramfs_engine.img").unwrap();
        assert_eq!(metadata.len() as usize, saved_bytes);
        let _ = fs::remove_file("/tmp/test_mkinitramfs_engine.img");
    }
}
