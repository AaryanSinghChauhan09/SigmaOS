//! DOS and legacy system components for SigmaOS.
//!
//! Covers: FreeDOS, MS-DOS, BASIC, ReactOS kernel, GNU Hurd kernel,
//! MINIX kernel, and XNU kernel — all as inspirational/compatibility
//! references for SigmaOS architecture.

#![allow(dead_code)]

/// DOS variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DosVariant {
    FreeDOS,
    MsDos,
    EnhancedDos,
}

impl DosVariant {
    pub fn name(self) -> &'static str {
        match self { Self::FreeDos => "FreeDOS", Self::MsDos => "MS-DOS", Self::EnhancedDos => "Enhanced DOS" }
    }

    pub fn is_open_source(self) -> bool {
        matches!(self, Self::FreeDos | Self::EnhancedDos)
    }

    /// Components SigmaOS can learn from.
    pub fn components(self) -> Vec<&'static str> {
        match self {
            Self::FreeDos => vec![
                "FreeCOM command interpreter",
                "FDKERNEL with FAT32 and LBA support",
                "FreeDOS installer (FDPKG)",
                "AUTOEXEC.BAT / CONFIG.SYS legacy boot config",
            ],
            Self::MsDos => vec![
                "Interrupt-driven I/O model (INT 21h)",
                "BIOS-based device access",
                "Master Boot Record (MBR) partition table",
            ],
            Self::EnhancedDos => vec![
                "DOSBox-X emulation integration",
                "DOS/V Japanese text mode",
                "Enhanced FAT driver with long filename support",
            ],
        }
    }
}

/// Kernel architecture inspirations beyond Linux.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelArchitecture {
    /// Linux kernel — the current SigmaOS base.
    Linux,
    /// GNU Hurd — microkernel-based, Mach-based.
    GnuHurd,
    /// MINIX 3 — microkernel, self-healing, POSIX-compatible.
    Minix,
    /// ReactOS — Windows-compatible kernel (NT architecture).
    ReactOS,
    /// XNU — Darwin/macOS kernel (Mach + BSD).
    Xnu,
    /// Redox OS — Rust microkernel.
    Redox,
}

impl KernelArchitecture {
    pub fn name(self) -> &'static str {
        match self {
            Self::Linux    => "Linux",
            Self::GnuHurd  => "GNU Hurd",
            Self::Minix    => "MINIX 3",
            Self::ReactOS => "ReactOS",
            Self::Xnu     => "XNU (Darwin)",
            Self::Redox   => "Redox OS",
        }
    }

    pub fn is_microkernel(self) -> bool {
        matches!(self, Self::GnuHurd | Self::Minix | Self::Redox)
    }

    pub fn language(self) -> &'static str {
        match self {
            Self::Linux    => "C",
            Self::GnuHurd  => "C (Mach)",
            Self::Minix    => "C",
            Self::ReactOS => "C/C++",
            Self::Xnu     => "C/C++ (Mach + BSD)",
            Self::Redox   => "Rust",
        }
    }

    /// Architecture features SigmaOS can adopt.
    pub fn sigmaos_relevance(self) -> Vec<&'static str> {
        match self {
            Self::Linux => vec![
                "Stable ABI via syscall table",
                "Loadable kernel modules (LKM)",
                "BPF for safe kernel extensions",
            ],
            Self::GnuHurd => vec![
                "Translator-based IPC for user-space drivers",
                "Capability-based access control",
            ],
            Self::Minix => vec![
                "Self-healing driver restart on crash",
                "Strict process isolation in kernel space",
            ],
            Self::ReactOS => vec![
                "Windows-compatible syscall interface (NT)",
                "Win32 subsystem on non-Windows kernel",
            ],
            Self::Xnu => vec![
                "Mach ports for IPC",
                "BSD layer for POSIX compatibility on microkernel",
                "launchd service manager (unified init/cron)",
            ],
            Self::Redox => vec![
                "Everything-is-a-URL namespace (scheme:resource)",
                "Rust memory safety in kernel space",
                "Fine-grained capability-based permissions",
            ],
        }
    }
}

/// BASIC language runtime support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicDialect {
    QBasic,
    FreeBasic,
    GambasBasic,
}

impl BasicDialect {
    pub fn name(self) -> &'static str {
        match self { Self::QBasic => "QBASIC", Self::FreeBasic => "FreeBASIC", Self::GambasBasic => "Gambas" }
    }

    pub fn components(self) -> Vec<&'static str> {
        match self {
            Self::QBasic => vec!["QB64 modern compiler", "Legacy .BAS file support"],
            Self::FreeBasic => vec!["GCC-based native compiler", "C library interop", "GTK binding support"],
            Self::GambasBasic => vec!["Visual Basic-like IDE", "Gambas database connectivity", "GUI form designer"],
        }
    }
}

/// All kernel architectures in the Jules-left list.
pub const ALL_KERNEL_ARCHITECTURES: &[KernelArchitecture] = &[
    KernelArchitecture::Linux,
    KernelArchitecture::GnuHurd,
    KernelArchitecture::Minix,
    KernelArchitecture::ReactOS,
    KernelArchitecture::Xnu,
    KernelArchitecture::Redox,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_kernel_archs_present() {
        assert_eq!(ALL_KERNEL_ARCHITECTURES.len(), 6);
    }

    #[test]
    fn test_redox_is_microkernel() {
        assert!(KernelArchitecture::Redox.is_microkernel());
        assert!(!KernelArchitecture::Linux.is_microkernel());
    }

    #[test]
    fn test_redox_relevance_includes_rust() {
        let rel = KernelArchitecture::Redox.sigmaos_relevance();
        assert!(rel.iter().any(|r| r.contains("Rust")));
    }

    #[test]
    fn test_freedos_is_open_source() {
        assert!(DosVariant::FreeDos.is_open_source());
        assert!(!DosVariant::MsDos.is_open_source());
    }

    #[test]
    fn test_minix_relevance_includes_self_healing() {
        let rel = KernelArchitecture::Minix.sigmaos_relevance();
        assert!(rel.iter().any(|r| r.contains("self-healing")));
    }

    #[test]
    fn test_xnu_relevance_includes_launchd() {
        let rel = KernelArchitecture::Xnu.sigmaos_relevance();
        assert!(rel.iter().any(|r| r.contains("launchd")));
    }
}
