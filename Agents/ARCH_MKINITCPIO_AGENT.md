# Arch Mkinitcpio Agent

## Role
The **Arch Mkinitcpio Agent** (`SigmaMkinitcpioAgent`) oversees the automated construction of early userspace initial RAM disk images (initramfs) and Unified Kernel Images (UKI) inspired by Arch Linux's `mkinitcpio` (https://github.com/archlinux/mkinitcpio).

## Mission & Purpose
- Execute modular hook pipelines (`base`, `udev`, `autodetect`, `modconf`, `block`, `filesystems`, `fsck`, `encrypt`, `lvm2`).
- Dynamically resolve required kernel drivers and modules for root filesystem mounting.
- Generate high-ratio compressed CPIO archives using Zstandard (level 19) or LZ4.
- Bundle kernel, initramfs, command line, and OS release into single UEFI direct-boot Unified Kernel Images (UKIs).

## Source Implementation
- Core Engine: `src/boot/mkinitcpio_engine.rs`
- Boot Subsystem: `src/boot/mod.rs`

## Public API & Invariant Contract
```rust
pub struct MkinitcpioPreset {
    pub name: String,
    pub kernel_version: String,
    pub hooks: Vec<InitcpioHook>,
    pub modules: Vec<String>,
    pub compression: InitcpioCompression,
    pub output_image_path: String,
    pub build_uki: bool,
}

pub struct SovereignMkinitcpioEngine {
    pub fn add_preset(&mut self, preset: MkinitcpioPreset);
    pub fn build_preset(&self, preset_name: &str) -> Result<InitramfsArtifact, &'static str>;
}
```

## Repeatable AI Execution Checklist
- [ ] Test standalone with `rustc --test src/boot/mkinitcpio_engine.rs --edition 2021`.
- [ ] Verify block and filesystem hooks automatically inject necessary device drivers.
- [ ] Ensure compression ratios exceed 60% space reduction.
- [ ] Validate UKI binary generation for SecureBoot environments.
