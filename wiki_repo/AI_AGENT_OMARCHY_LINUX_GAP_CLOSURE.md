# 🏔️ AI Agent Omarchy Linux Gap Closure Protocol for SigmaOS

This document specifies the operational protocols and architectural gap closure directives for **AI Agents in Omarchy Linux Gap Closure** (`Agent-Omarchy`) within the SigmaOS ecosystem. Omarchy Linux represents cutting-edge Arch-based rolling releases, Hyprland dynamic tiling window management, AUR packaging, and reproducible containerized build workflows.

---

## 🏛️ 1. Rolling Release & Package Management Parity

`Agent-Omarchy` ensures 100% operational parity with Arch/Omarchy Linux's pacman and AUR ecosystems:

```
┌─────────────────────────────────────────────────────────────┐
│             Agent-Omarchy Package Parity Engine             │
└─────────────────────────────────────────────────────────────┘
         │                          │                         │
         ▼                          ▼                         ▼
┌──────────────────┐      ┌──────────────────┐      ┌──────────────────┐
│ Pacman Sync & AUR│      │ Pacman-Contrib   │      │ Makepkg Chroot   │
│ • PKGBUILD Parser│      │ • Paccache Clean │      │ • Isolated Build │
│ • AUR Diff Audit │      │ • RankMirrors    │      │ • Clean Rootfs   │
└──────────────────┘      └──────────────────┘      └──────────────────┘
```

### 🔹 Core Packaging Components
1. **Pacman Sync & AUR Helper (`src/sigpkg/aur_helper.rs`)**:
   - Parses Arch PKGBUILD manifests, validates `optdepends`, `provides`, and `conflicts`, and performs PKGBUILD diff security auditing prior to execution.
2. **Pacman-Contrib Tooling (`src/sigpkg/arch_pacman_engine.rs`)**:
   - Implements `PacmanContribEngine` providing `paccache_clean`, `rankmirrors`, `updpkgsums`, `checkupdates`, and `finddeps` parity.
3. **Makepkg Clean Chroot Builder**:
   - Compiles AUR and official packages inside isolated, non-contaminated chroot build roots (`MockChrootBuilder`), enforcing OpenBSD `pledge`/`unveil` sandbox boundaries.

---

## 🎨 2. Hyprland & Wayland Tiling Compositor Parity

Omarchy Linux is renowned for fluid Hyprland Wayland tiling workflows. `Agent-Omarchy` brings native Wayland dynamic tiling parity to the Zenith Desktop Environment:

- **Dynamic BSP & Master-Stack Window Tiling**:
  - Implements Hyprland-style dynamic Binary Space Partitioning (`DynamicBSP`) and Lumina/BSD master-stack window layouts in `ZenithCompositor`.
- **Bezier Animation Curves & Blur Effects**:
  - Renders GPU-accelerated window transitions, rounded corners, and Gaussian blur overlays using atomic display planes in `GpuDriver`.
- **Wayland Layer-Shell Desklets**:
  - Renders transparent, grid-snapped desktop widgets and status bars (`FedoraDeskletWidgetEngine`).

---

## 🛡️ 3. Reproducible Build & Sigstore Verification

To eliminate supply chain tampering, `Agent-Omarchy` enforces reproducible build verification:

1. **SPDX & CycloneDX SBOM Generation**:
   - Automatically generates SPDX 2.3 Software Bill of Materials (SBOM) manifests for all compiled package archives.
2. **Cosign & Sigstore PKGBUILD Signatures**:
   - Verifies cryptographic signatures on PKGBUILD scripts and binary tarballs (`.pkg.tar.zst`) before installation.
3. **Bit-for-Bit Deterministic Compilation**:
   - Enforces reproducible build environments (`ReproducibleBuildContext`), stripping non-deterministic timestamps and build paths.

---

## 📊 4. Omarchy Gap Closure Scorecard

`Agent-Omarchy` measures and reports gap closure metrics continuously over the system message bus:

| Metric | Target | Enforced By |
|---|---|---|
| **AUR / PKGBUILD Compatibility** | 100% Manifest Parsing | `PacmanSyncManager` & `AurParser` |
| **Pacman-Contrib Parity** | 5/5 Utility Commands | `PacmanContribEngine` |
| **Wayland Tiling Responsiveness** | 144 FPS Fluidity | `ZenithCompositor` |
| **Reproducible Build Verification** | 100% Bit-for-Bit Hash Match | SPDX / Cosign Sigstore Verifier |

---

This protocol ensures that SigmaOS absorbs the performance, desktop fluidity, and rolling-release agility of Omarchy Linux while securing enterprise stability and zero-dependency microkernel sovereignty.
