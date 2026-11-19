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

SigmaOS provides a turnkey Arch-based live ISO via the archiso profile in
`iso/archiso/`. This uses the stock Arch Linux kernel and standard Arch
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

- ✅ Archiso profile validated successfully
- ✅ Uses stock Arch kernel (no custom kernel stubs)
- ✅ 116 packages configured for functional desktop
- ✅ Defensive installer with safety checks
- ✅ Hyprland configuration with SigmaOS branding added
- ✅ Wallust theming integration
- ✅ Waybar status bar configured
- ✅ Placeholder packages archived (198 moved to pkgs_archive/)
- ✅ 9 essential packages remain (all with real functionality)
- ⏳ Requires Arch host or Docker to build ISO
- ⏳ QEMU boot test pending after build

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
