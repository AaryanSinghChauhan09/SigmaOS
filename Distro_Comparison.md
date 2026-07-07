# SigmaOS vs Advanced Distros

## Decision Criteria Table

| Criteria | Arch | Fedora | Kali | SigmaOS (Target) |
| :--- | :--- | :--- | :--- | :--- |
| **Release model** | Rolling | Rapid / Fedora releases | Rolling (pentest) | Hybrid: stable LTS + optional rolling research |
| **Package system** | Pacman + AUR | DNF / RPM | APT (Debian base) | sigpkg: signed, atomic, reproducible |
| **Security posture** | User-managed | SELinux by default | Offensive security tools | Default least-privilege, microVMs, TPM attestation |
| **Desktop/UX** | Minimal; user builds | GNOME polished | Lightweight | Zenith Wayland shell + accessibility |
| **Reproducibility** | Source-oriented (AUR) | Binary focus | Tooling for pentest | Nix/Guix-inspired reproducible builds |

## Missing Gaps to Fix

| Area | Missing in SigmaOS | Example from advanced distros |
| :--- | :--- | :--- |
| **Kernel & drivers** | No LTS kernel branch; limited upstream drivers | Arch/Fedora track kernels and upstream drivers closely. |
| **Package/update system** | No signed, atomic package format or repo mirrors | Fedora/Arch have robust package managers (dnf/pacman). |
| **Installer & UX** | No polished graphical installer or Wayland compositor | Fedora/Ubuntu ship polished installers and Wayland sessions. |
| **Security & sandboxing** | No TPM attestation, microVMs, or default least-privilege | Kali/Qubes show specialized security models. |
| **Reproducible builds** | No deterministic build farm or SBOMs | NixOS/Guix provide reproducible, declarative builds. |
