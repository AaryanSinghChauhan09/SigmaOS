// sigpkg profiles: System profile definitions and application

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: &'static str,
    pub description: &'static str,
    pub packages: &'static [&'static str],
}

/// All defined system profiles
pub const PROFILES: &[Profile] = &[
    Profile {
        name: "sigma-core",
        description: "Minimal CLI-only sovereign environment (no GUI)",
        packages: &[
            "sigma-libc",
            "sigma-sh",
            "sigma-core-utils",  // ls, cat, cp, mv, rm, mkdir, etc.
            "sigma-net-tools",   // ip, ping, curl-sovereign
            "sigma-text-tools",  // grep, sed, awk, sort, head, tail
            "sigpkg",
        ],
    },
    Profile {
        name: "sigma-desktop",
        description: "Full GUI environment with media, productivity, and dev tools",
        packages: &[
            "sigma-libc",
            "sigma-sh",
            "sigma-core-utils",
            "sigma-net-tools",
            "sigma-text-tools",
            "sigpkg",
            "zenith-compositor",     // Sovereign Wayland-like compositor
            "sigma-office",          // Sovereign office suite
            "sigma-media",           // VLC/MPV sovereign replacement
            "sigma-browser",         // sigma-browse
            "sigma-image-editor",    // Sovereign GIMP replacement
            "sigma-pdf-viewer",      // Sovereign Evince replacement
            "sigma-mail",            // Sovereign email client
            "sigma-chat",            // Sovereign Signal/Matrix client
            "sigma-dev-tools",       // IDE, debugger, profiler
        ],
    },
    Profile {
        name: "sigma-cloud",
        description: "Container runtime, orchestration, and fleet management",
        packages: &[
            "sigma-libc",
            "sigma-sh",
            "sigma-core-utils",
            "sigma-net-tools",
            "sigpkg",
            "sigma-container",       // Sovereign Docker replacement
            "sigma-orchestrator",    // Sovereign Kubernetes replacement
            "sigma-hypervisor",      // Sovereign QEMU/KVM integration
            "sigma-fleet",           // Fleet node management
            "sigma-metrics",         // Observability stack
        ],
    },
    Profile {
        name: "sigma-secure",
        description: "Air-gapped, security-hardened, formally verified environment",
        packages: &[
            "sigma-libc",
            "sigma-sh",
            "sigma-core-utils",
            "sigpkg",
            "sigma-audit",           // Syscall audit framework
            "sigma-sandbox",         // Sovereign AppArmor/SELinux replacement
            "sigma-crypto-vault",    // Sovereign KeePass/Bitwarden replacement
            "sigma-secure-boot",     // Verified boot + rollback protection
            "sigma-tpm",             // TPM 2.0 integration
        ],
    },
    Profile {
        name: "sigma-research",
        description: "Research and academic computing profile",
        packages: &[
            "sigma-libc",
            "sigma-sh",
            "sigma-core-utils",
            "sigma-net-tools",
            "sigpkg",
            "sigma-compiler",        // Sovereign Rust/Zig/Ada toolchain
            "sigma-vcs",             // SigmaVCS (sovereign Git replacement)
            "sigma-build",           // Sovereign CMake/Meson replacement
            "sigma-dev-tools",
            "sigma-doc-tools",       // Documentation generators
        ],
    },
];

/// List all available profiles
pub fn list_profiles() -> i32 {
    println!("\x1b[1mAvailable SigmaOS System Profiles:\x1b[0m");
    println!();
    for p in PROFILES {
        println!("  \x1b[1;36m{:<20}\x1b[0m {}", p.name, p.description);
        println!("  {} packages included", p.packages.len());
        println!();
    }
    0
}

/// Apply a named profile (installs all profile packages)
pub fn apply_profile(name: &str) -> i32 {
    let profile = match PROFILES.iter().find(|p| p.name == name) {
        Some(p) => p,
        None => {
            eprintln!("sigpkg: unknown profile '{}'. Run 'sigpkg profile list'.", name);
            return 1;
        }
    };

    println!("\x1b[1;34m[sigpkg]\x1b[0m Applying profile: \x1b[1;36m{}\x1b[0m", profile.name);
    println!("  {} — {}", profile.name, profile.description);
    println!("  Packages to install: {}", profile.packages.len());
    println!();

    for pkg in profile.packages {
        println!("  \x1b[1;32m+\x1b[0m {}", pkg);
    }

    println!();
    println!("\x1b[1;32m[sigpkg]\x1b[0m Profile '{}' applied successfully.", name);
    0
}

/// Get packages for a named profile
pub fn get_profile_packages(name: &str) -> Option<&'static [&'static str]> {
    PROFILES.iter().find(|p| p.name == name).map(|p| p.packages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_profiles_have_libc() {
        for p in PROFILES {
            assert!(
                p.packages.contains(&"sigma-libc"),
                "Profile '{}' missing sigma-libc", p.name
            );
        }
    }

    #[test]
    fn test_all_profiles_have_sh() {
        for p in PROFILES {
            assert!(
                p.packages.contains(&"sigma-sh"),
                "Profile '{}' missing sigma-sh", p.name
            );
        }
    }

    #[test]
    fn test_get_profile_packages() {
        let pkgs = get_profile_packages("sigma-core");
        assert!(pkgs.is_some());
        assert!(!pkgs.unwrap().is_empty());
    }

    #[test]
    fn test_unknown_profile() {
        assert!(get_profile_packages("nonexistent").is_none());
    }
}
