// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak, gentoo ebuilds, freebsd pkgs, appimages, and nix store hashes.

use std::collections::HashMap;

/// Package format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,      // apt (Debian/Ubuntu)
    Rpm,      // yum/dnf (RHEL/Fedora)
    Pacman,   // pacman (Arch Linux)
    Snap,     // snap (Ubuntu Sandboxed)
    Flatpak,  // flatpak (Desktop Sandboxed)
    SigmaPkg, // native SigmaOS format
    // Advanced Open-Source Packaging Formats:
    Portage,      // Gentoo Portage (ebuild source recipes)
    FreeBsdPkg,   // FreeBSD pkg (txz binaries)
    ArchPkgBuild, // Arch PKGBUILD (source compile scripts)
    NixStore,     // Nix package manager (content-addressed store hashes)
    AppImage,     // AppImage (self-contained portable binaries)
    Homebrew,     // Homebrew (ruby formulas)