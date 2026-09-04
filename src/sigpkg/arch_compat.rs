// Arch Linux ALPM & Gentoo Ebuild Parity Subsystem
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

pub struct PortPackage {
    pub name: String,
    pub version: Version,
    pub dependencies: Vec<String>,
}

pub struct ArchPkgbuild {
    pub pkgname: String,
    pub pkgver: String,
    pub depends: Vec<String>,
}

impl ArchPkgbuild {
    pub fn parse(content: &str) -> Option<Self> {
        let mut pkgname = String::new();
        let mut pkgver = String::new();
        let mut depends = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("pkgname=") {
                pkgname = line.trim_start_matches("pkgname=").trim_matches('"').trim_matches('\'').to_string();
            } else if line.starts_with("pkgver=") {
                pkgver = line.trim_start_matches("pkgver=").trim_matches('"').trim_matches('\'').to_string();
            } else if line.starts_with("depends=(") {
                let deps_str = line.trim_start_matches("depends=(").trim_end_matches(')');
                for dep in deps_str.split_whitespace() {
                    depends.push(dep.trim_matches('"').trim_matches('\'').to_string());
                }
            }
        }

        if !pkgname.is_empty() {
            Some(ArchPkgbuild { pkgname, pkgver, depends })
        } else {
            None
        }
    }
}

pub struct PortageEbuildCompiler {
    pub use_flags: HashMap<String, bool>,
}

impl PortageEbuildCompiler {
    pub fn new() -> Self {
        PortageEbuildCompiler {
            use_flags: HashMap::new(),
        }
    }
}

impl Default for PortageEbuildCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_pkgbuild_parser() {
        let pkgbuild_str = r#"
            pkgname='neofetch-sovereign'
            pkgver='7.1.0'
            depends=('bash' 'pciutils')
        "#;

        let pkg = ArchPkgbuild::parse(pkgbuild_str).unwrap();
        assert_eq!(pkg.pkgname, "neofetch-sovereign");
        assert_eq!(pkg.pkgver, "7.1.0");
        assert_eq!(pkg.depends, vec!["bash", "pciutils"]);
    }

    #[test]
    fn test_portage_ebuild_compiler() {
        let mut compiler = PortageEbuildCompiler::new();
        compiler.use_flags.insert("x86".to_string(), true);
        assert!(compiler.use_flags.get("x86").copied().unwrap_or(false));
    }
}
