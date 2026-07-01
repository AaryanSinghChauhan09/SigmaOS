// sigpkg resolver: DAG-based dependency resolution with SemVer constraint solving

/// A resolved package entry
#[derive(Debug, Clone)]
pub struct ResolvedPackage {
    pub name: String,
    pub version: String,
    pub hash: String,
    pub signature: String,
}

/// Resolve a package and all its transitive dependencies (topologically sorted)
pub fn resolve(package: &str) -> Result<Vec<ResolvedPackage>, String> {
    // In a real implementation this would:
    // 1. Fetch package manifest from sovereign registry
    // 2. Parse SemVer constraints
    // 3. Build DAG and detect cycles/conflicts
    // 4. Return topological sort (deps first)

    let mut resolved = Vec::new();
    let deps = fetch_deps(package)?;

    for dep in deps {
        resolved.push(ResolvedPackage {
            name: dep.0,
            version: dep.1,
            hash: dep.2,
            signature: dep.3,
        });
    }

    Ok(resolved)
}

/// Check for dependency conflicts between two sets of packages
pub fn check_conflicts(pkgs_a: &[ResolvedPackage], pkgs_b: &[ResolvedPackage]) -> Vec<String> {
    let mut conflicts = Vec::new();
    for a in pkgs_a {
        for b in pkgs_b {
            if a.name == b.name && a.version != b.version {
                conflicts.push(format!(
                    "Conflict: {} requires {} but {} requires {}",
                    a.name, a.version, b.name, b.version
                ));
            }
        }
    }
    conflicts
}

/// Compare two SemVer strings: returns Ordering
pub fn semver_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> (u64, u64, u64) {
        let parts: Vec<u64> = s.split('.')
            .map(|x| x.parse().unwrap_or(0))
            .collect();
        (
            parts.get(0).copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    };
    parse(a).cmp(&parse(b))
}

// Simulated registry fetch (replace with real HTTP + TOML parse)
fn fetch_deps(package: &str) -> Result<Vec<(String, String, String, String)>, String> {
    // Returns: (name, version, sha256_hash, ed25519_signature)
    match package {
        "sigma-core-utils" => Ok(vec![
            ("sigma-libc".to_string(), "1.0.0".to_string(),
             "a3f5c2d...".to_string(), "sig:ed25519:abc123...".to_string()),
            ("sigma-core-utils".to_string(), "0.2.0".to_string(),
             "b8e1f9a...".to_string(), "sig:ed25519:def456...".to_string()),
        ]),
        "sigma-sh" => Ok(vec![
            ("sigma-libc".to_string(), "1.0.0".to_string(),
             "a3f5c2d...".to_string(), "sig:ed25519:abc123...".to_string()),
            ("sigma-sh".to_string(), "0.2.0".to_string(),
             "c9d2e4f...".to_string(), "sig:ed25519:ghi789...".to_string()),
        ]),
        _ => Ok(vec![
            (package.to_string(), "0.1.0".to_string(),
             "deadbeef...".to_string(), "sig:ed25519:unknown...".to_string()),
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_cmp() {
        assert_eq!(semver_cmp("1.0.0", "1.0.1"), std::cmp::Ordering::Less);
        assert_eq!(semver_cmp("2.0.0", "1.9.9"), std::cmp::Ordering::Greater);
        assert_eq!(semver_cmp("1.2.3", "1.2.3"), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_resolve_known() {
        let r = resolve("sigma-sh");
        assert!(r.is_ok());
        let pkgs = r.unwrap();
        assert!(!pkgs.is_empty());
        assert!(pkgs.iter().any(|p| p.name == "sigma-sh"));
    }

    #[test]
    fn test_no_conflicts() {
        let a = resolve("sigma-sh").unwrap();
        let b = resolve("sigma-sh").unwrap();
        let conflicts = check_conflicts(&a, &b);
        assert!(conflicts.is_empty());
    }
}
