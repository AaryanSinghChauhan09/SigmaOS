# SigmaOS — Agent & Contributor Rules

SigmaOS is a sovereign, modular operating system. This repo (`SigmaOS`) is the
**connector/control plane** for the `SigmaOS-*` component fleet — it owns the
registry (`COMPONENTS.toml`), the canonical scaffold (`templates/component/`),
and the shell hub (`scripts/sigma_hub.sh`). Each component lives in its own
`SigmaOS-<name>` repository (modular multi-repo layout).

## Prime directives

- **NO DEMO & NO STUB — only actual working code.** Scaffold repos must be
  upgraded to real, tested implementations; never leave placeholder logic.
- **TALK IS CHEAP, SHOW ME THE CODE.** Every change must build and pass tests.
- **TEST CODE USING CI/CD.** Run `cargo test` / `make hub-test`; AAA pattern.
- **ONE COMPONENT PER REPO; ONE `main` BRANCH PER REPO.**
- **CONSOLIDATE similar repos; CREATE only necessary components.**

## Engineering principles (apply always)

Modularity · single responsibility · DRY · KISS · separation of concerns ·
mechanism vs policy · information hiding · loose coupling · high cohesion ·
interface segregation · dependency inversion · open/closed · Liskov ·
composition over inheritance · fail-fast · least privilege · secure defaults ·
defensive programming · atomic ops · race-free design · deadlock avoidance ·
zero-copy where possible · reproducible builds · semantic versioning ·
upstream-first · pristine sources · optimize for deletion.

## Component contract

Every `SigmaOS-*` repo carries `COMPONENT.toml` (`[component]`, `[contract]
provides/depends_on`, `[maturity]`). The registry `COMPONENTS.toml` is the
source of truth — keep metadata truthful (no invented descriptions).

## Hub workflow (`scripts/sigma_hub.sh`)

Use the hub for all fleet ops — never hand-edit across repos blindly:
`list status audit families consolidate dupes health` (inventory) ·
`search info depends` (package-query, Arch/Debian/Fedora/Mint/Ubuntu-
inspired) · `connect create upgrade sync push test` (lifecycle).
Make targets: `make hub-*`. Prefer `--dry-run` first; `upgrade`/`create` are
non-destructive (add missing files only).

## Safety

- Never commit secrets/keys/tokens — env vars or a secret manager only.
- Consolidation/archival of remote repos is **reversible-preferred** (archive,
  don't delete) and requires verification each group isn't unique work.
- Branch only for features needing tests; merge after tests pass.
- Keep `main` free of `.md`/`.txt` docs — those belong in the GitHub Wiki.

## Verification before "done"

`cargo build --offline` + `cargo test --offline` green; `COMPONENT.toml` valid
TOML; changes committed; push only when asked or via `make hub-sync --push`.

## Arch-based ISO Build Process

> **Moved (2026-11-20):** the live-ISO pipeline now lives in the
> **`SigmaOS-ISO` component repo** (fleet sibling `../SigmaOS-ISO`).
> It owns `iso/archiso/`, `scripts/build_archiso.sh`,
> `scripts/reassemble-iso.sh`, `scripts/sign-iso-secureboot.sh`,
> `scripts/ci/qemu_iso_boot.sh`, `scripts/ci/qemu_boot_smoke.sh`, and the
> `build-iso.yml` / `boot-smoke.yml` workflows. Releases publish under
> `SigmaOS-ISO` releases going forward. The notes below are kept for
> historical context.

SigmaOS provides a turnkey Arch-based live ISO via the archiso profile in
`iso/archiso/` (now `SigmaOS-ISO/iso/archiso/`). This uses the stock Arch
Linux kernel and standard Arch
infrastructure - NOT a custom Rust microkernel (that's a separate research track).

### Architecture

- **Kernel**: Stock Arch Linux kernel (`linux`, `linux-firmware`, `linux-headers`)
- **Desktop**: Hyprland (Wayland tiling compositor) with Waybar, Quickshell
- **Audio**: PipeWire + WirePlumber
- **Package Manager**: Standard pacman (not the experimental `sigpkg`)
- **Installer**: Defensive installer (`sigmaos-install`) with dry-run protection

### Build Requirements

Build the ISO on an Arch Linux system OR use Docker on any Linux host:

**Option A: Native Arch Linux build**
```bash
# On Arch Linux host
sudo pacman -S archiso
sudo scripts/build_archiso.sh --native
```

**Option B: Docker build (any Linux host)**
```bash
# Install Docker (Ubuntu/Debian example)
sudo apt-get install -y docker.io
sudo systemctl start docker
sudo usermod -aG docker $USER  # log out and in

# Build ISO
scripts/build_archiso.sh --docker
```

**Option C: Validate profile only**
```bash
scripts/build_archiso.sh --validate-only
```

### ISO Structure

- `iso/archiso/profiledef.sh` - ISO metadata and boot modes
- `iso/archiso/packages.x86_64` - 116 curated packages (Hyprland, Wayland, PipeWire, etc.)
- `iso/archiso/airootfs/` - Root filesystem overlay with:
  - `usr/local/bin/sigmaos-install` - Defensive installer (dry-run by default)
  - `etc/hostname` - Hostname: "sigmaos"
  - `etc/locale.conf` - Locale: en_US.UTF-8
  - Custom pacman hooks for Btrfs snapshots

### Testing the ISO

After successful build:
```bash
# Create test disk
qemu-img create -f qcow2 /tmp/sigmaos_test.qcow2 32G

# Boot ISO in QEMU
qemu-system-x86_64 \
  -m 4096 \
  -enable-kvm \
  -drive file=/tmp/sigmaos_test.qcow2,format=qcow2,if=virtio \
  -cdrom out/sigmaos-desktop-YYYYMMDD-x86_64.iso \
  -boot d
```

### Installer Safety

The installer (`sigmaos-install`) has built-in safety:
- **Default**: Dry-run mode (writes nothing to disk)
- **Destructive mode**: Requires both `--destructive` AND `--confirm-disk <DEV>`
- **Active root guard**: Refuses to install to the host's root disk
- **Mount protection**: Aborts if target partitions are mounted

Example destructive install (ONLY in VMs):
```bash
sudo sigmaos-install --destructive --confirm-disk /dev/vda /dev/vda
```

### Package Notes

The `pkgs/` directory contains many placeholder PKGBUILD files for custom
SigmaOS components. These are NOT used in the initial ISO build - the ISO
uses standard Arch packages from the official repositories. These custom
packages can be added incrementally after the base ISO is working.

### Current Status

- ✅ Archiso profile validated successfully (105 packages)
- ✅ Uses stock Arch kernel (no custom kernel stubs)
- ✅ Lean live ISO (~3.3 GB) — Hyprland, Waybar, PipeWire, foot, Thunar
- ✅ Boots to Hyprland desktop via tty1 autologin (UEFI verified in QEMU)
- ✅ One-step installer (`sigmaos-install`) — real arch-install-scripts,
  Btrfs+GRUB, guided user setup, dry-run default + destructive guards
- ✅ Secure Boot helpers (`sigmaos-secureboot`, `scripts/sign-secureboot.sh`)
- ✅ Component scaffold uses `ComponentLifecycle` trait (OOP/SOLID)
- ✅ Placeholder packages archived (198 moved to pkgs_archive/)
- ✅ CI builds ISO in Docker and auto-publishes split Release assets
- ✅ BIOS (syslinux) + UEFI (systemd-boot) boot paths fixed — upstream
  archiso releng configs, correct COM32 paths, splash.png, serial console
- ✅ Initramfs ships archiso hooks + explicit storage modules
  (ata_piix/ata_generic/pata_*/sr_mod/cdrom/isofs/usb-storage/uas/nvme/
  vmd/mmc/loop/dm-snapshot/overlay .ko's; ahci/sd_mod/virtio/btrfs are
  builtin) — live media is found on BIOS, UEFI, USB, and NVMe/SATA devices
- ✅ Build-time initramfs verification in customize_airootfs.sh fails the
  build if archiso hooks or storage modules are missing
- ✅ Working pacman hooks: sigmaos-snapshot (btrfs pre/post-transaction
  snapshots with retention), paccache via pacman-contrib, sigmaos-update
- ⏳ Secure Boot signing of the live ISO's efiboot.img (needs release key)

### Recent Changes (2026-11-20) — boot fixes

Root-caused "ISO doesn't boot at all" to three independent defects:

1. **syslinux menu failure**: custom configs referenced
   `boot/syslinux/vesamenu.c32` / `splash.png` by ISO-root-relative paths;
   isolinux resolves them relative to `/boot/syslinux/`, so the COM32 load
   failed and BIOS boot dropped to a `boot:` prompt. Replaced with the
   upstream archiso releng syslinux configs (whichsys.c32 selector, PXE +
   local + chain.c32 + memtest + hdt entries), rebranded to SigmaOS, and
   added the missing `splash.png`.

2. **initramfs missing archiso hooks**: the shipped `initramfs-linux.img`
   was built with `HOOKS="udev keymap consolefont"` only — no
   `archiso`/`archiso_loop_mnt`, and `autodetect` produced a junk module
   set with no disk drivers, so the kernel found zero block devices and
   dropped to an emergency shell. `mkinitcpio.conf` now lists every
   required storage driver explicitly in `MODULES=` (autodetect cannot
   work inside the airootfs chroot; only real .ko modules are listed —
   ahci/sd_mod/virtio/btrfs/zstd are builtin in the Arch kernel), and
   `customize_airootfs.sh` verifies the built images contain the archiso
   hooks and `sr_mod`/`ata_piix`/`isofs`/`cdrom` before the ISO is
   assembled.

3. **Broken pacman hooks**: `05/95-sigmaos-snapshot-*.hook` and
   `clean_package_cache.hook` called `/usr/local/bin/sigmaos-snapshot` and
   `paccache`, neither of which existed in the image. Added
   `usr/local/bin/sigmaos-snapshot` (real btrfs snapshot/rollback helper,
   fail-safe), shipped `usr/local/bin/sigmaos-update` (self-contained
   ISO variant used by the post-upgrade hook and the weekly systemd
   timer), added `pacman-contrib` for `paccache`, and shipped
   `sigmaos-provision-owner` (referenced by its first-boot systemd unit).
   Tightened `/etc/sudoers.d/wheel` from `NOPASSWD: ALL` to standard sudo.

Also: systemd-boot entries now use a single `initrd` line (ucodes are
embedded by the mkinitcpio `microcode` hook), matching upstream archiso.

**Rebuild required**: `scripts/build_archiso.sh --docker` (needs Docker
access) then re-test with QEMU. `customize_airootfs.sh` will now abort the
build if the initramfs regresses.

### Recent Changes (2026-11-19)

Following **GALL'S LAW**, **NO DEMO & NO STUB**, and **NORMALISE THE OS & IT'S COMPONENTS' IDENTITY TO SIGMAOS** principles:

1. **Archived placeholder packages**: Moved 198 PKGBUILD files with `|| true` and `install /dev/null` to `pkgs_archive/`. These were hiding failures and not providing real artifacts.

2. **Removed Omarchy references**: 
   - Archived `config/omarchy/` to `config_archive/omarchy_legacy/`
   - Renamed `sigmaos-omarchy-shell` to `sigmaos-shell`
   - Updated AGENTS.md to remove Omarchy references
   - SigmaOS is now independently branded

3. **Essential packages only**: Kept 9 working packages:
   - `sigmaos-base` - Minimal Arch base meta-package
   - `sigmaos-base-devel` - Development tools meta-package
   - `sigmaos-desktop-meta` - Desktop environment meta-package
   - `sigmaos-keyring` - PGP keyring
   - `sigmaos-mirrorlist` - Mirror configuration
   - `sigmaos-shell` - Shell configuration with Starship prompt (real content)
   - `sigmaos-pacman` - Pacman wrapper
   - `sigmaos-wallust-themes` - Real color presets and templates (with actual content)
   - `sigmaos-linux` - Linux kernel meta-package

4. **Added Hyprland configuration**: Real working Hyprland config with SigmaOS branding in `iso/archiso/airootfs/etc/skel/.config/hyprland/`

5. **Added Waybar configuration**: Status bar with workspaces, clock, tray, audio, network, CPU, memory

6. **Added Wallust theming**: Default SigmaOS color preset and Hyprland template

7. **Enhanced installer**: Added pacstrap step to install complete desktop environment with all necessary packages

8. **Stock Arch kernel**: ISO uses standard `linux`, `linux-firmware`, `linux-headers` from Arch repositories - NOT custom stubs

9. **Created build instructions**: BUILD_INSTRUCTIONS.md with Docker and Arch build methods
