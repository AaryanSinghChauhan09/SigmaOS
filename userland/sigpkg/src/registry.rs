// sigpkg registry: Sovereign package registry interface

#[derive(Debug, Clone)]
pub struct PackageEntry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub profile: String,
    pub hash: String,
    pub signature: String,
    pub depends: Vec<String>,
}

/// Search the sovereign registry by name/keyword
pub fn search(query: &str) -> Vec<PackageEntry> {
    // In production: query the sovereign registry API / local mirror
    let query = query.to_lowercase();
    known_packages().into_iter()
        .filter(|p| p.name.contains(&query) || p.description.to_lowercase().contains(&query))
        .collect()
}

/// Get detailed info for a specific package
pub fn info(name: &str) -> Option<PackageEntry> {
    known_packages().into_iter().find(|p| p.name == name)
}

/// List installed packages for a profile
pub fn list_installed(profile: &str) -> Vec<PackageEntry> {
    // In production: read /var/lib/sigpkg/installed.db
    if profile == "all" {
        known_packages()
    } else {
        known_packages().into_iter()
            .filter(|p| p.profile == profile || p.profile == "sigma-core")
            .collect()
    }
}

// We need lazy static for runtime Vec construction — use fn instead
fn known_packages() -> Vec<PackageEntry> {
    vec![
        PackageEntry {
            name: "sigma-libc".to_string(),
            version: "1.0.0".to_string(),
            description: "Sovereign libc — zero glibc dependency".to_string(),
            profile: "sigma-core".to_string(),
            hash: "a3f5c2d9e8b1f4a7".to_string(),
            signature: "sig:ed25519:abc123def456".to_string(),
            depends: vec![],
        },
        PackageEntry {
            name: "sigma-sh".to_string(),
            version: "0.2.0".to_string(),
            description: "Sovereign shell with scripting and automation".to_string(),
            profile: "sigma-core".to_string(),
            hash: "c9d2e4f7b3a1c8d5".to_string(),
            signature: "sig:ed25519:ghi789jkl012".to_string(),
            depends: vec!["sigma-libc".to_string()],
        },
        PackageEntry {
            name: "sigma-core-utils".to_string(),
            version: "0.2.0".to_string(),
            description: "Sovereign GNU coreutils replacement (ls, cat, cp, mv, ...)".to_string(),
            profile: "sigma-core".to_string(),
            hash: "b8e1f9a4c7d2e5f3".to_string(),
            signature: "sig:ed25519:mno345pqr678".to_string(),
            depends: vec!["sigma-libc".to_string()],
        },
        PackageEntry {
            name: "sigpkg".to_string(),
            version: "0.2.0".to_string(),
            description: "Sovereign package manager with cryptographic verification".to_string(),
            profile: "sigma-core".to_string(),
            hash: "f1e2d3c4b5a6f7e8".to_string(),
            signature: "sig:ed25519:stu901vwx234".to_string(),
            depends: vec!["sigma-libc".to_string()],
        },
        PackageEntry {
            name: "zenith-compositor".to_string(),
            version: "0.4.0".to_string(),
            description: "Zenith sovereign desktop compositor".to_string(),
            profile: "sigma-desktop".to_string(),
            hash: "d4e5f6a7b8c9d0e1".to_string(),
            signature: "sig:ed25519:yza567bcd890".to_string(),
            depends: vec!["sigma-libc".to_string(), "sigma-gpu-hal".to_string()],
        },
        PackageEntry {
            name: "sigma-browser".to_string(),
            version: "0.1.0".to_string(),
            description: "sigma-browse — sovereign web browser".to_string(),
            profile: "sigma-desktop".to_string(),
            hash: "e5f6a7b8c9d0e1f2".to_string(),
            signature: "sig:ed25519:efg123hij456".to_string(),
            depends: vec!["sigma-libc".to_string(), "sigma-net-tools".to_string()],
        },
        PackageEntry {
            name: "sigma-vcs".to_string(),
            version: "0.1.0".to_string(),
            description: "SigmaVCS — sovereign version control system".to_string(),
            profile: "sigma-research".to_string(),
            hash: "f6a7b8c9d0e1f2a3".to_string(),
            signature: "sig:ed25519:klm789nop012".to_string(),
            depends: vec!["sigma-libc".to_string()],
        },
    ]
}
