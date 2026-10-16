# Arch Install Scripts Agent

## Role
The **Arch Install Scripts Agent** (`SigmaArchInstallScriptsAgent`) orchestrates system bootstrap, declarative filesystem table generation, and namespace-isolated chroot environments inspired by Arch Linux's `arch-install-scripts` (`pacstrap`, `genfstab`, `arch-chroot`) (https://github.com/archlinux/arch-install-scripts).

## Mission & Purpose
- Provide clean, deterministic system installation and provisioning into target mountpoints (`pacstrap`).
- Automatically detect, parse, and format `/etc/fstab` entries using persistent UUID or filesystem LABEL attributes (`genfstab`).
- Establish isolated chroot environments with automatic bind mounts (`/proc`, `/sys`, `/dev`, `/run`, efivars) and guarantee clean unmounting upon exit (`arch-chroot`).

## Source Implementation
- Core Engine: `src/installer/arch_install_scripts.rs`
- Installer Module: `src/installer/mod.rs`

## Public API & Invariant Contract
```rust
pub struct PacstrapConfig {
    pub target_directory: String,
    pub base_packages: Vec<String>,
    pub copy_mirrorlist: bool,
    pub use_host_cache: bool,
}

pub struct SovereignArchInstallScripts {
    pub fn pacstrap(config: PacstrapConfig) -> Result<BootstrapResult, &'static str>;
    pub fn genfstab(mounts: &[MountEntry], mode: FstabIdMode) -> String;
    pub fn prepare_chroot_bind_mounts(target_dir: &str) -> Vec<String>;
}
```

## Repeatable AI Execution Checklist
- [ ] Test standalone with `rustc --test src/installer/arch_install_scripts.rs --edition 2021`.
- [ ] Verify `genfstab` generates standard POSIX format entries with valid UUID strings.
- [ ] Ensure all 7 critical pseudo-filesystem paths are included in chroot bind mount lists.
- [ ] Enforce clean error handling if target mount paths are non-existent.
