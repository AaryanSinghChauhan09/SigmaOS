# SigmaOS vs Other Distributions

> An honest, technical comparison of SigmaOS against major Linux distributions. This is not a marketing document — it acknowledges where SigmaOS falls short and what it aims to do better.

---

## Comparison Matrix

| Feature | SigmaOS | Linux Mint | Omarchy | Arch Linux | Ubuntu | Fedora |
|---------|---------|-----------|---------|-----------|--------|--------|
| Primary language | Rust/C | C/Python | Shell/Ruby | C | C/Python | C/Python |
| Desktop | Custom (Wayland) | Cinnamon (X11) | Hyprland | User choice | GNOME | GNOME |
| Display protocol | Wayland-first | X11 (Wayland optional) | Wayland | Both | Wayland | Wayland |
| Package format | Native + Flatpak | .deb (Ubuntu base) | pacman | pacman | .deb | .rpm |
| Atomic updates | Yes (btrfs) | No | No | No | Partial | Yes (rpm-ostree) |
| AI integration | Native (first-class) | None | None | None | None | None |
| Secure Boot | Yes (signed shim) | Yes | No | Manual | Yes | Yes |
| Telemetry | Zero | Minimal opt-in | None | None | Optional | Optional |
| Release model | LTS + rolling channel | LTS | Rolling | Rolling | LTS + non-LTS | Biannual |
| Target user | Power user + AI user | Casual desktop user | Developer/Vim user | Advanced user | General user | Developer |
| Memory usage (idle) | Target < 512MB | ~700MB | ~300MB | ~150MB | ~900MB | ~900MB |
| Multi-repo architecture | Yes (149 repos) | No | No | No | No | No |
| Reproducible builds | Goal (in progress) | No | No | Partial | No | Partial |
| FOSS-only default | Yes | Mostly | Yes | No (AUR) | No (multiverse) | Yes (nonfree repo) |

---

## SigmaOS vs Linux Mint

### What Linux Mint Does Well

Linux Mint is the gold standard for "Linux that just works" for desktop users. It excels at:

- **User experience polish**: Cinnamon is one of the most Windows-like, approachable Linux desktops. Transitions from Windows are smooth.
- **Stability**: Long-term Ubuntu LTS base with conservative update policy means broken systems are rare.
- **Hardware compatibility**: Excellent out-of-the-box support inherited from Ubuntu's massive hardware database.
- **Community and documentation**: 15+ years of forums, wiki articles, and tutorials.
- **Update manager maturity**: Color-coded update levels, rollback warnings, excellent UX for updates.
- **Driver management**: `mintdrivers` makes proprietary driver installation trivial.

### Where Linux Mint Falls Short (for SigmaOS's Goals)

- **X11-first**: Cinnamon is still primarily X11. Wayland support is experimental. SigmaOS is Wayland-native from day one.
- **No atomicity**: Updates are not atomic. A partial update can leave the system in a broken state.
- **No AI integration**: AI is not part of Linux Mint's vision.
- **Monolithic Cinnamon**: The desktop is a tightly coupled monolith, harder to swap components.
- **No Rust**: The entire stack is C/Python. Memory safety issues exist in the desktop layer.
- **Update signing**: Package signing is inherited from Debian/Ubuntu, not a first-party design.
- **No sovereign update server**: Mint relies on Ubuntu's infrastructure.

### What SigmaOS Borrows from Linux Mint

- **Installer UX philosophy**: The installer should be as simple as Linux Mint's — clean, linear, no surprises.
- **Update manager UX**: The update presentation concept (showing what changes and what it affects) is inspired by mintupdate.
- **"Works for normal people" mindset**: Mint proved that a curated desktop can reach non-technical users. SigmaOS aims for the same.
- **Software manager concept**: sigma-store is conceptually inspired by Linux Mint's Software Manager.

### SigmaOS Advantages Over Linux Mint

- Wayland-native compositor with better security boundaries
- Atomic updates with btrfs rollback
- AI assistant as a first-class system component
- Memory-safe system layer (Rust)
- Modular, replaceable components (not monolithic Cinnamon)
- Fully signed update chain (package + repo + ISO)

---

## SigmaOS vs Omarchy

### What Omarchy Does Well

Omarchy (by DHH) is a highly opinionated Arch-based setup for developers who want a curated Hyprland experience. It excels at:

- **Developer focus**: Configured for Rails/Ruby developers but extensible for any workflow.
- **Wayland-native**: Hyprland is a capable, beautiful Wayland compositor.
- **Simplicity**: It's not trying to be everything — it's one person's ideal setup, open for others.
- **Terminal-centric**: Excellent terminal tooling (Ghostty, Neovim, tmux).
- **Minimal**: Very little bloat — only what the author uses.

### Where Omarchy Falls Short (for SigmaOS's Goals)

- **Not a distro**: Omarchy is a personal configuration script on top of Arch. It has no installer, no package management of its own, no update guarantees.
- **No stability contract**: Rolling Arch under the hood means breaking changes can arrive at any time.
- **Not for general users**: Requires comfort with command line. No graphical installer.
- **No AI integration**: AI features are not part of Omarchy.
- **No security hardening**: Inherits whatever Arch provides. No custom seccomp profiles, Landlock, etc.
- **Single maintainer**: Omarchy is one person's configuration. SigmaOS is a collaborative multi-contributor project.
- **No documentation governance**: No principles, no architecture document, no contribution model.

### What SigmaOS Borrows from Omarchy

- **Opinionated defaults**: Like Omarchy, SigmaOS makes strong defaults rather than offering 20 choices.
- **Wayland-first**: Omarchy's commitment to Wayland-native tools validates SigmaOS's approach.
- **Terminal-quality**: Omarchy's emphasis on terminal quality (fast, feature-rich) is shared by SigmaOS.
- **"One right way" philosophy**: Rather than being all things to all people, SigmaOS makes deliberate, justified choices.

### SigmaOS Advantages Over Omarchy

- Full distribution with installer, package management, update system
- Stability guarantees and LTS release cadence
- Security hardening (seccomp, Landlock, signed updates)
- Multi-contributor governance and community
- AI-native design throughout the system
- General-user-accessible (not just developer audiences)

---

## SigmaOS vs Arch Linux

### What Arch Does Well

- **Rolling release**: Always current packages.
- **AUR**: Largest community package repository on any Linux distro.
- **Minimal base**: You build exactly what you want.
- **Documentation**: The Arch Wiki is the best Linux documentation resource in existence.
- **systemd mastery**: Arch's systemd integration is second to none.
- **User control**: Total control — no decisions made for you.

### Where Arch Falls Short (for SigmaOS's Goals)

- **No atomic updates**: `pacman -Syu` on a bad day can break the system.
- **No installer (official)**: The official installer is `archinstall` — functional but not polished for general users.
- **No stability guarantees**: Rolling release means no LTS, no 2-year support window.
- **No AI integration**.
- **Complexity**: Arch requires significant expertise to maintain correctly.
- **No curated experience**: Arch is a toolkit, not a product.

### What SigmaOS Borrows from Arch

- **Minimal philosophy**: Arch's "only what you need" approach influences sigma-kernel's minimal Kconfig.
- **pacman concepts**: sigma-pkg's transaction model is conceptually inspired by pacman's atomic transaction design.
- **Documentation culture**: SigmaOS aspires to Arch-wiki-quality documentation.

### SigmaOS Advantages Over Arch

- Polished installer for all users
- Atomic updates with rollback
- Curated, opinionated defaults (no decision fatigue)
- AI-native features
- Memory-safe system components

---

## SigmaOS vs Ubuntu

### What Ubuntu Does Well

- **Hardware support**: Ubuntu's hardware enablement stack (HWE) is the most comprehensive available.
- **Enterprise ecosystem**: Snap store, landscape, commercial support, .deb ecosystem.
- **GNOME integration**: Best-in-class GNOME implementation.
- **Global community**: Largest Linux community by raw user numbers.
- **Certification**: Ubuntu is certified on more laptops and servers than any other distro.

### Where Ubuntu Falls Short (for SigmaOS's Goals)

- **Snap controversies**: Snap packages have performance and privacy concerns. Many users disable Snap immediately.
- **Optional telemetry**: Ubuntu collects system data by default (opt-out, but still a concern for sovereignty).
- **Non-atomic updates**: APT updates are not atomic.
- **Heavy footprint**: Stock Ubuntu uses ~900MB RAM idle.
- **Canonical control**: Key decisions are made by Canonical, not the community.
- **No Rust in system layer**: Memory safety vulnerabilities in system daemons are a real concern.

### What SigmaOS Borrows from Ubuntu

- **Installer polish inspiration**: Ubuntu's Ubiquity/new installer is a UX reference.
- **Hardware enablement concept**: SigmaOS's Hardware Validation phase is modeled on Ubuntu's HWE philosophy.
- **LTS model**: SigmaOS's 2-year LTS commitment mirrors Ubuntu's.

### SigmaOS Advantages Over Ubuntu

- Zero telemetry, no opt-out required
- Atomic updates with rollback
- Rust-based system components (memory safety)
- Wayland-native (no X11 legacy overhead)
- AI-native design
- Lighter footprint target (< 512MB idle)

---

## SigmaOS vs Fedora

### What Fedora Does Well

- **Cutting edge**: Fedora ships the latest kernel, GNOME, and toolchain — often 6 months ahead of other distros.
- **rpm-ostree (Silverblue/Kinoite)**: Fedora has the best atomic desktop implementation currently available.
- **SELinux**: Fedora's SELinux implementation is the most mature in the consumer Linux space.
- **Developer tools**: Toolbox/Distrobox integration, Fedora is a developer's distro.
- **GNOME upstream**: Fedora is GNOME's primary proving ground.
- **Reproducible builds progress**: Fedora has invested in reproducible RPM builds.

### Where Fedora Falls Short (for SigmaOS's Goals)

- **Short support window**: Fedora releases are only supported for ~13 months (two releases).
- **No AI integration**: AI is not a first-class Fedora concern.
- **GNOME-only (standard)**: Fedora prioritizes GNOME; other DEs are spins with less polish.
- **No Rust in core**: Despite being a developer distro, Fedora's core system tools are still C/Python.
- **Red Hat dependency**: Fedora is upstream of RHEL; corporate interests influence decisions.

### What SigmaOS Borrows from Fedora

- **Atomic updates**: Fedora Silverblue's rpm-ostree approach validates atomic desktop updates. SigmaOS uses btrfs snapshots to achieve similar atomicity.
- **SELinux/seccomp philosophy**: Fedora's security-first approach is a reference for SigmaOS's security layer.
- **Reproducible builds investment**: SigmaOS targets reproducible builds, borrowing concepts from Fedora's work.
- **Toolbox/Distrobox concept**: sigma-distrobox is inspired by Fedora's Toolbox.

### SigmaOS Advantages Over Fedora

- Longer support window (2-year LTS)
- AI-native design
- Rust-based system layer
- Custom compositor (not GNOME-tied)
- Modular component architecture

---

## Honest Gap Analysis: What SigmaOS Still Needs to Match

| Capability | Best-in-class | SigmaOS Status |
|------------|--------------|----------------|
| Hardware support depth | Ubuntu | Far behind — Phase 8 |
| Documentation quality | Arch Wiki | Far behind — ongoing |
| Package ecosystem size | Ubuntu (.deb AUR) | Far behind — needs years |
| Installer maturity | Ubuntu/Linux Mint | Phase 3 — not started |
| Security hardening | Fedora (SELinux) | Phase 6 — planned |
| Atomic updates | Fedora Silverblue | Phase 6 — planned |
| Developer tooling | Fedora | Phase 5 — planned |
| Community size | Ubuntu | Nascent |
| Certified hardware | Ubuntu | None yet |

SigmaOS is **not** trying to replace Ubuntu for general use today. It is building toward a specific niche: users who want sovereignty, AI-native tooling, and a memory-safe system layer — and are willing to accept early-adopter rough edges.

---

## Summary: SigmaOS's Unique Position

SigmaOS is the only Linux distribution that combines:

1. **Wayland-native** compositor (not a retrofit)
2. **AI-native** design (not bolted on)
3. **Rust-based** system layer (memory safety throughout)
4. **Atomic updates** with automatic rollback
5. **Zero telemetry** and no forced cloud
6. **Modular, replaceable** architecture (149 independent components)
7. **Principle-driven** development (90+ named principles)

No other distribution does all seven. That is SigmaOS's reason to exist.
