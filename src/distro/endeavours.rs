//! EndeavourOS Linux distribution components.
//!
//! EndeavourOS is an Arch Linux derivative focused on a clean, beginner-friendly
//! installer while remaining terminal-centric and close to upstream Arch.

#![allow(dead_code)]

/// EndeavourOS release identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndeavourRelease {
    Cassini,
    Galileo,
}

impl EndeavourRelease {
    pub fn name(self) -> &'static str {
        match self { Self::Cassini => "Cassini", Self::Galileo => "Galileo" }
    }
}

/// Desktop environments EndeavourOS ships.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndeavourDesktop {
    Xfce,
    Kde,
    Gnome,
    Budgie,
    Cinnamon,
    Mate,
    Lxqt,
    Sway,
    I3,
    Bspwm,
    Hyprland,
    NoneNetinstall,
}

impl EndeavourDesktop {
    pub fn is_tiling(self) -> bool {
        matches!(self, Self::Sway | Self::I3 | Self::Bspwm | Self::Hyprland)
    }
}

/// EndeavourOS components that SigmaOS can learn from.
pub fn endeavours_components() -> Vec<&'static str> {
    vec![
        "Calamares installer with Arch base bootstrap",
        "eos-installer terminal mode for headless installs",
        "welcome app with community-driven post-install tips",
        "AUR helper integration (yay) preconfigured",
        "inxi system info tool on first boot",
        "reflector for fastest Arch mirror selection",
        "pacman mirrors managed by reflector-simple",
        "eos-update script for clean full-system upgrade",
        "arch-chroot recovery from live ISO",
        "boot manager: systemd-boot (not GRUB) by default",
        "BTRFS snapshots via snapper on root subvolume",
        "NVIDIA driver detection at install time",
        "community editions for each desktop environment",
        "netinstall ISO for minimal base + desktop choice at boot",
    ]
}

/// Integration plan mapping EndeavourOS features to SigmaOS components.
pub fn endeavours_integration_map() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Calamares installer",          "sigmaos-installer"),
        ("welcome app",                  "sigmaos-welcome-screen"),
        ("yay / AUR helper",             "sigmaos-packages"),
        ("reflector mirror selection",   "sigmaos-mirror"),
        ("eos-update script",            "sigmaos-update"),
        ("systemd-boot manager",         "sigmaos-bootloader"),
        ("snapper BTRFS snapshots",      "sigmaos-timeshift"),
        ("NVIDIA driver detection",      "sigmaos-drivers"),
        ("netinstall ISO",              "sigmaos-ISO"),
        ("community editions",          "sigmaos-desktop-environment"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiling_wms_detected() {
        assert!(EndeavourDesktop::Sway.is_tiling());
        assert!(EndeavourDesktop::Hyprland.is_tiling());
        assert!(!EndeavourDesktop::Xfce.is_tiling());
    }

    #[test]
    fn test_components_nonempty() {
        assert!(!endeavours_components().is_empty());
        assert!(endeavours_components().iter().any(|c| c.contains("Calamares")));
    }

    #[test]
    fn test_integration_map_has_installer() {
        let map = endeavours_integration_map();
        assert!(map.iter().any(|(c, t)| c.contains("Calamares") && t.contains("installer")));
    }
}
