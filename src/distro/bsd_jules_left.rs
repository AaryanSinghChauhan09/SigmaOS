//! BSD distribution components missing from the Jules-left roadmap.
//!
//! Covers: GhostBSD, NomadBSD, MidnightBSD, HardenedBSD, LibertyBSD,
//! FuryBSD, DesktopBSD, KarmaBSD, and StarBSD.

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

/// BSD distribution variant identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BsdVariant {
    GhostBSD,
    NomadBSD,
    MidnightBSD,
    HardenedBSD,
    LibertyBSD,
    FuryBSD,
    DesktopBSD,
    KarmaBSD,
    StarBSD,
}

impl BsdVariant {
    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            Self::GhostBSD    => "GhostBSD",
            Self::NomadBSD    => "NomadBSD",
            Self::MidnightBSD  => "MidnightBSD",
            Self::HardenedBSD  => "HardenedBSD",
            Self::LibertyBSD   => "LibertyBSD",
            Self::FuryBSD      => "FuryBSD",
            Self::DesktopBSD   => "DesktopBSD",
            Self::KarmaBSD     => "KarmaBSD",
            Self::StarBSD      => "StarBSD",
        }
    }

    /// Underlying FreeBSD base version this distro is built on.
    pub fn freebsd_base(self) -> &'static str {
        match self {
            Self::GhostBSD   | Self::MidnightBSD | Self::DesktopBSD => "FreeBSD 13.x",
            Self::NomadBSD   | Self::FuryBSD     | Self::StarBSD    => "FreeBSD 14.x",
            Self::HardenedBSD | Self::LibertyBSD                     => "FreeBSD 14.x (hardened)",
            Self::KarmaBSD                                          => "FreeBSD 13.x (GhostBSD fork)",
        }
    }

    /// Desktop environment this distro ships with.
    pub fn desktop_environment(self) -> &'static str {
        match self {
            Self::GhostBSD | Self::DesktopBSD | Self::KarmaBSD => "MATE",
            Self::NomadBSD   => "XFCE (persistent live)",
            Self::MidnightBSD => "MATE (GNUstep optional)",
            Self::HardenedBSD => "headless / server-first",
            Self::LibertyBSD => "headless / secure desktop",
            Self::FuryBSD    => "XFCE",
            Self::StarBSD    => "XFCE (live-only)",
        }
    }

    /// Package manager used.
    pub fn package_manager(self) -> &'static str {
        match self {
            Self::HardenedBSD => "pkg (HardenedBSD repo)",
            _                 => "pkg (FreeBSD ports)",
        }
    }

    /// Primary use case / target audience.
    pub fn use_case(self) -> &'static str {
        match self {
            Self::GhostBSD   => "Desktop users transitioning from Linux/Windows",
            Self::NomadBSD   => "Persistent live USB for nomadic computing",
            Self::MidnightBSD => "Desktop users who like GNUstep/NeXT aesthetics",
            Self::HardenedBSD => "Security-hardened server and firewall",
            Self::LibertyBSD  => "Privacy-focused, FSF-endorsed free software only",
            Self::FuryBSD     => "Fast live desktop with instant-on boot",
            Self::DesktopBSD  => "User-friendly desktop (legacy project)",
            Self::KarmaBSD    => "GhostBSD-derived alternative desktop",
            Self::StarBSD     => "Lightweight live rescue desktop",
        }
    }

    /// Components SigmaOS can learn from or integrate.
    pub fn inspirational_components(self) -> Vec<&'static str> {
        match self {
            Self::GhostBSD => vec!["GhostBSD installer (Calamares-based)",
                "OpenRC service manager on FreeBSD", "automount via devd"],
            Self::NomadBSD => vec!["Persistent live USB with ZFS snapshots",
                "NTFS write support via ntfs-3g", "Encrypted home on USB"],
            Self::MidnightBSD => vec!["GNUstep desktop integration",
                "mports package system", "Libdispatch support"],
            Self::HardenedBSD => vec!["ASLR (PaX-style) on FreeBSD",
                "Wrapped libc for hardening", "branch-specific security patches",
                "Kernel address sanitizer (KASAN)"],
            Self::LibertyBSD => vec!["FSF-endorsed blob-free kernel config",
                "Removing non-free firmware loading"],
            Self::FuryBSD => vec!["Live boot optimization",
                "Slab allocator tuning for desktop responsiveness"],
            Self::DesktopBSD => vec!["PC-BSD-style GUI package manager",
                "warden jail manager (legacy)"],
            Self::KarmaBSD => vec!["GhostBSD fork improvements",
                "Boot environment management"],
            Self::StarBSD => vec!["Lightweight rescue tools",
                "ZFS pool import/repair from live media"],
        }
    }
}

/// All BSD variants in the Jules-left list.
pub const ALL_BSD_VARIANTS: &[BsdVariant] = &[
    BsdVariant::GhostBSD,
    BsdVariant::NomadBSD,
    BsdVariant::MidnightBSD,
    BsdVariant::HardenedBSD,
    BsdVariant::LibertyBSD,
    BsdVariant::FuryBSD,
    BsdVariant::DesktopBSD,
    BsdVariant::KarmaBSD,
    BsdVariant::StarBSD,
];

/// SigmaOS integration plan for a BSD variant.
#[derive(Debug, Clone)]
pub struct BsdIntegrationPlan {
    pub variant: BsdVariant,
    pub components_to_adapt: Vec<String>,
    pub target_sigmaos_module: String,
}

impl BsdIntegrationPlan {
    pub fn for_variant(variant: BsdVariant) -> Self {
        let comps: Vec<String> = variant.inspirational_components()
            .iter().map(|s| s.to_string()).collect();
        let target = match variant {
            BsdVariant::GhostBSD | BsdVariant::KarmaBSD => "sigmaos-desktop-installer",
            BsdVariant::NomadBSD | BsdVariant::FuryBSD | BsdVariant::StarBSD => "sigmaos-live-session",
            BsdVariant::MidnightBSD => "sigmaos-desktop-environment",
            BsdVariant::HardenedBSD => "sigmaos-security",
            BsdVariant::LibertyBSD => "sigmaos-firmware-policy",
            BsdVariant::DesktopBSD => "sigmaos-packages",
        }.to_string();
        Self { variant, components_to_adapt: comps, target_sigmaos_module: target }
    }
}

/// Generate integration plans for all BSD variants.
pub fn all_bsd_integration_plans() -> Vec<BsdIntegrationPlan> {
    ALL_BSD_VARIANTS.iter().map(|&v| BsdIntegrationPlan::for_variant(v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_nine_variants_present() {
        assert_eq!(ALL_BSD_VARIANTS.len(), 9);
    }

    #[test]
    fn test_ghostbsd_desktop_is_mate() {
        assert_eq!(BsdVariant::GhostBSD.desktop_environment(), "MATE");
    }

    #[test]
    fn test_hardenedbsd_use_case_security() {
        assert!(BsdVariant::HardenedBSD.use_case().contains("Security"));
    }

    #[test]
    fn test_nomadbsd_use_case_persistent_usb() {
        assert!(BsdVariant::NomadBSD.use_case().contains("Persistent"));
    }

    #[test]
    fn test_integration_plan_for_hardenedbsd_targets_security() {
        let plan = BsdIntegrationPlan::for_variant(BsdVariant::HardenedBSD);
        assert_eq!(plan.target_sigmaos_module, "sigmaos-security");
        assert!(!plan.components_to_adapt.is_empty());
    }

    #[test]
    fn test_all_integration_plans_generated() {
        let plans = all_bsd_integration_plans();
        assert_eq!(plans.len(), 9);
    }
}
