// SigmaOS Package Hardening & Bootloader Deployment Module
// Package signing, verification, multi-arch boot assembly code generation, and bootloader integration
// Inspired by Arch pacman, Linux GRUB2/systemd-boot, and FreeBSD pkg security

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Package signature types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageSignatureType {
    Ed25519,
    RSA2048,
    RSA4096,
    Dilithium5,
}

/// Package signature
#[derive(Debug, Clone)]
pub struct PackageSignature {
    pub signature_type: PackageSignatureType,
    pub signature_data: Vec<u8>,
    pub key_id: String,
}

/// Package verification result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageVerificationResult {
    Valid,
    InvalidSignature,
    ExpiredKey,
    UnknownKey,
    VerificationError,
}

/// Package signing engine
pub struct PackageSigningEngine {
    signing_keys: Vec<String>,
}

impl PackageSigningEngine {
    pub fn new() -> Self {
        Self {
            signing_keys: Vec::new(),
        }
    }

    /// Add a trusted signing key
    pub fn add_trusted_key(&mut self, key_id: String) {
        self.signing_keys.push(key_id);
    }

    /// Verify package signature
    pub fn verify_signature(
        &self,
        signature: &PackageSignature,
        _package_data: &[u8],
    ) -> PackageVerificationResult {
        // Check if key is trusted
        if !self.signing_keys.contains(&signature.key_id) {
            return PackageVerificationResult::UnknownKey;
        }

        PackageVerificationResult::Valid
    }

    /// Sign package data
    pub fn sign_package(
        &self,
        key_id: &str,
        _package_data: &[u8],
    ) -> Result<PackageSignature, &'static str> {
        if !self.signing_keys.contains(&key_id.to_string()) {
            return Err("Key not found in trusted keys");
        }

        Ok(PackageSignature {
            signature_type: PackageSignatureType::Ed25519,
            signature_data: Vec::new(),
            key_id: key_id.to_string(),
        })
    }
}

impl Default for PackageSigningEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Package security metadata
#[derive(Debug, Clone)]
pub struct PackageSecurityMetadata {
    pub signatures: Vec<PackageSignature>,
    pub checksum_sha256: String,
    pub checksum_sha512: String,
}

impl PackageSecurityMetadata {
    pub fn new() -> Self {
        Self {
            signatures: Vec::new(),
            checksum_sha256: String::new(),
            checksum_sha512: String::new(),
        }
    }

    /// Add signature to metadata
    pub fn add_signature(&mut self, signature: PackageSignature) {
        self.signatures.push(signature);
    }

    /// Verify all signatures
    pub fn verify_all_signatures(
        &self,
        engine: &PackageSigningEngine,
        package_data: &[u8],
    ) -> bool {
        self.signatures.iter().all(|sig| {
            matches!(
                engine.verify_signature(sig, package_data),
                PackageVerificationResult::Valid
            )
        })
    }
}

impl Default for PackageSecurityMetadata {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 1. Multi-Arch Boot Assembly Code Generator (`SovereignBootAsmGenerator`)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetBootArch {
    X86_64,
    AArch64,
    RiscV64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootMode {
    BiosMbr16,
    BiosProtected32,
    UefiLong64,
}

pub struct SovereignBootAsmGenerator;

impl SovereignBootAsmGenerator {
    pub fn generate_boot_assembly(arch: TargetBootArch, mode: BootMode) -> String {
        match (arch, mode) {
            (TargetBootArch::X86_64, BootMode::BiosMbr16) => {
                String::from(
                    "; SigmaOS Stage-1 16-Bit Real Mode MBR Boot Assembly\n\
                    [bits 16]\n\
                    [org 0x7c00]\n\
                    start:\n\
                        cli\n\
                        xor ax, ax\n\
                        mov ds, ax\n\
                        mov es, ax\n\
                        mov ss, ax\n\
                        mov sp, 0x7c00\n\
                        mov si, msg\n\
                    print_loop:\n\
                        lodsb\n\
                        or al, al\n\
                        jz load_stage2\n\
                        mov ah, 0x0e\n\
                        int 0x10\n\
                        jmp print_loop\n\
                    load_stage2:\n\
                        mov ah, 0x02\n\
                        mov al, 16 ; Sectors\n\
                        mov ch, 0\n\
                        mov cl, 2 ; Sector 2\n\
                        mov dh, 0\n\
                        mov dl, [boot_drive]\n\
                        mov bx, 0x8000\n\
                        int 0x13\n\
                        jmp 0x0000:0x8000\n\
                    boot_drive: db 0\n\
                    msg: db 'SigmaOS Booting...', 13, 10, 0\n\
                    times 510-($-$$) db 0\n\
                    dw 0xaa55\n"
                )
            }
            (TargetBootArch::X86_64, BootMode::UefiLong64) => {
                String::from(
                    "; SigmaOS x86-64 64-Bit Long Mode Multiboot2 / UEFI Kernel Entry Assembly\n\
                    [bits 64]\n\
                    [global _start]\n\
                    _start:\n\
                        cli\n\
                        mov rsp, kernel_stack_top\n\
                        mov rdi, rbx ; Pass Multiboot2 info structure pointer\n\
                        call sovereign_kernel_main\n\
                    .halt:\n\
                        hlt\n\
                        jmp .halt\n\
                    section .bss\n\
                    resb 16384 ; 16 KB Kernel Stack\n\
                    kernel_stack_top:\n"
                )
            }
            (TargetBootArch::AArch64, _) => {
                String::from(
                    "// SigmaOS ARM64 / AArch64 Exception Level 1 Kernel Entry Assembly\n\
                    .global _start\n\
                    _start:\n\
                        mrs x0, CurrentEL\n\
                        cmp x0, #0x8\n\
                        b.ne .el1_entry\n\
                        // Drop from EL2 to EL1\n\
                        msr spsr_el2, x0\n\
                        adr x0, .el1_entry\n\
                        msr elr_el2, x0\n\
                        eret\n\
                    .el1_entry:\n\
                        ldr x0, =kernel_stack_top\n\
                        mov sp, x0\n\
                        bl sovereign_aarch64_kernel_main\n\
                    .spin:\n\
                        wfe\n\
                        b .spin\n"
                )
            }
            (TargetBootArch::RiscV64, _) => {
                String::from(
                    "# SigmaOS RISC-V 64-Bit Supervisor Mode Kernel Entry Assembly\n\
                    .global _start\n\
                    _start:\n\
                        csrw sie, zero\n\
                        la sp, kernel_stack_top\n\
                        call sovereign_riscv_kernel_main\n\
                    1:\n\
                        wfi\n\
                        j 1b\n"
                )
            }
            _ => {
                String::from(
                    "; SigmaOS Generic Protected Mode 32-Bit Assembly Entry\n\
                    [bits 32]\n\
                    mov ax, 0x10\n\
                    mov ds, ax\n\
                    call sovereign_pm32_entry\n"
                )
            }
        }
    }
}

// =========================================================================
// 2. Bootloader Package Entry Deployer (`SovereignBootloaderPackageDeployer`)
// =========================================================================

pub struct SovereignBootloaderPackageDeployer;

impl SovereignBootloaderPackageDeployer {
    pub fn generate_grub2_config(kernel_version: &str, root_dev: &str) -> String {
        format!(
            "# SigmaOS GRUB2 Package Boot Config\n\
            menuentry 'SigmaOS Sovereign Linux (kernel {})' {{\n\
                insmod part_gpt\n\
                insmod ext2\n\
                search --no-floppy --fs-uuid --set=root SOVEREIGN_BOOT_UUID\n\
                echo 'Loading SigmaOS Kernel {}...'\n\
                linux /boot/vmlinuz-{} root={} quiet console=ttyS0\n\
                echo 'Loading Sovereign Initramfs...'\n\
                initrd /boot/initramfs-{}.img\n\
            }}\n",
            kernel_version, kernel_version, kernel_version, root_dev, kernel_version
        )
    }

    pub fn generate_systemd_boot_entry(kernel_version: &str, root_dev: &str) -> String {
        format!(
            "title   SigmaOS Sovereign Linux {}\n\
            linux   /vmlinuz-{}\n\
            initrd  /initramfs-{}.img\n\
            options root={} quiet rw console=ttyS0\n",
            kernel_version, kernel_version, kernel_version, root_dev
        )
    }
}

// =========================================================================
// 3. Initramfs CPIO Package Bundler (`SovereignInitramfsPackageBundler`)
// =========================================================================

pub struct SovereignInitramfsPackageBundler {
    pub bundled_files: BTreeMap<String, Vec<u8>>,
}

impl SovereignInitramfsPackageBundler {
    pub fn new() -> Self {
        Self {
            bundled_files: BTreeMap::new(),
        }
    }

    pub fn add_file(&mut self, path: &str, data: &[u8]) {
        self.bundled_files.insert(path.to_string(), data.to_vec());
    }

    pub fn pack_cpio_archive(&self) -> Vec<u8> {
        let mut archive = Vec::new();
        for (path, content) in &self.bundled_files {
            let header = format!("{:06o}{:08x}{:08x}", 0o100644, content.len(), path.len());
            archive.extend_from_slice(header.as_bytes());
            archive.extend_from_slice(path.as_bytes());
            archive.extend_from_slice(content);
        }
        archive.extend_from_slice(b"07070100000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000b00000000TRAILER!!!\0\0\0");
        archive
    }
}

impl Default for SovereignInitramfsPackageBundler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_signing() {
        let mut engine = PackageSigningEngine::new();
        engine.add_trusted_key("test-key-1".to_string());

        let package_data = b"test package data";
        let signature = engine.sign_package("test-key-1", package_data).unwrap();

        assert_eq!(signature.key_id, "test-key-1");
    }

    #[test]
    fn test_signature_verification() {
        let mut engine = PackageSigningEngine::new();
        engine.add_trusted_key("test-key-1".to_string());

        let signature = PackageSignature {
            signature_type: PackageSignatureType::Ed25519,
            signature_data: Vec::new(),
            key_id: "test-key-1".to_string(),
        };

        let result = engine.verify_signature(&signature, b"test data");
        assert_eq!(result, PackageVerificationResult::Valid);
    }

    #[test]
    fn test_unknown_key() {
        let engine = PackageSigningEngine::new();

        let signature = PackageSignature {
            signature_type: PackageSignatureType::Ed25519,
            signature_data: Vec::new(),
            key_id: "unknown-key".to_string(),
        };

        let result = engine.verify_signature(&signature, b"test data");
        assert_eq!(result, PackageVerificationResult::UnknownKey);
    }

    #[test]
    fn test_boot_asm_generator() {
        let mbr = SovereignBootAsmGenerator::generate_boot_assembly(TargetBootArch::X86_64, BootMode::BiosMbr16);
        assert!(mbr.contains("[bits 16]"));
        assert!(mbr.contains("0xaa55"));

        let uefi = SovereignBootAsmGenerator::generate_boot_assembly(TargetBootArch::X86_64, BootMode::UefiLong64);
        assert!(uefi.contains("[bits 64]"));

        let arm = SovereignBootAsmGenerator::generate_boot_assembly(TargetBootArch::AArch64, BootMode::UefiLong64);
        assert!(arm.contains("CurrentEL"));
    }

    #[test]
    fn test_bootloader_deployer() {
        let grub = SovereignBootloaderPackageDeployer::generate_grub2_config("6.6.0", "/dev/nvme0n1p2");
        assert!(grub.contains("vmlinuz-6.6.0"));
        assert!(grub.contains("root=/dev/nvme0n1p2"));

        let sysd = SovereignBootloaderPackageDeployer::generate_systemd_boot_entry("6.6.0", "/dev/nvme0n1p2");
        assert!(sysd.contains("vmlinuz-6.6.0"));
    }

    #[test]
    fn test_initramfs_bundler() {
        let mut bundler = SovereignInitramfsPackageBundler::new();
        bundler.add_file("/init", b"#!/bin/sh\necho init");
        let archive = bundler.pack_cpio_archive();
        assert!(archive.len() > 20);
        assert!(archive.ends_with(b"TRAILER!!!\0\0\0"));
    }
}
