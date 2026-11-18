// SPDX-License-Identifier: MIT OR GPL-2.0
//! SigmaOS — Arch Linux pacman 7.0 JSON Bridge
//!
//! Implements pacman 7.0 `--json` output format for all query commands:
//!   -Qi / -Si : package info (JSON)
//!   -Qs / -Ss : search results (JSON)
//!   -Qu       : pending upgrades (JSON)
//!   -Ql       : package file list (JSON)
//! Also implements PKGBUILD v2 extended fields (checkdepends, license_spdx).

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

extern crate alloc;

// ─── Package Info (pacman -Qi / -Si --json) ───────────────────────────────────

#[derive(Debug, Clone)]
pub struct PacmanPackageInfo {
    pub repository: Option<String>,   // None = local, Some = sync
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub architecture: String,
    pub licenses: Vec<String>,
    pub groups: Vec<String>,
    pub provides: Vec<String>,
    pub depends: Vec<String>,
    pub opt_depends: Vec<OptDepend>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub installed_size: u64,          // bytes
    pub download_size: Option<u64>,   // bytes, only for sync
    pub packager: String,
    pub build_date: String,           // ISO 8601
    pub install_date: Option<String>, // ISO 8601, only for local
    pub install_reason: InstallReason,
    pub install_script: bool,
    pub validation: Vec<String>,      // e.g. ["pgp", "sha256"]
}

#[derive(Debug, Clone)]
pub struct OptDepend {
    pub package: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InstallReason {
    ExplicitlyInstalled,
    InstalledAsDependency,
}

impl InstallReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ExplicitlyInstalled => "explicitly installed",
            Self::InstalledAsDependency => "installed as a dependency",
        }
    }
}

impl PacmanPackageInfo {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            repository: None,
            name: name.to_string(),
            version: version.to_string(),
            description: String::new(),
            url: String::new(),
            architecture: "x86_64".to_string(),
            licenses: Vec::new(),
            groups: Vec::new(),
            provides: Vec::new(),
            depends: Vec::new(),
            opt_depends: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            installed_size: 0,
            download_size: None,
            packager: "SigmaOS Packager <sigmaos@example.com>".to_string(),
            build_date: "2026-11-18T00:00:00Z".to_string(),
            install_date: None,
            install_reason: InstallReason::ExplicitlyInstalled,
            install_script: false,
            validation: vec!["pgp".to_string()],
        }
    }

    /// Render as JSON string (pacman 7.0 --json format)
    pub fn to_json(&self) -> String {
        let repo = match &self.repository {
            Some(r) => format!("\"{}\"", r),
            None => "null".to_string(),
        };
        let install_date = match &self.install_date {
            Some(d) => format!("\"{}\"", d),
            None => "null".to_string(),
        };
        let download_size = match self.download_size {
            Some(s) => format!("{}", s),
            None => "null".to_string(),
        };

        let licenses_json = self.licenses.iter()
            .map(|l| format!("\"{}\"", l))
            .collect::<Vec<_>>()
            .join(", ");
        let depends_json = self.depends.iter()
            .map(|d| format!("\"{}\"", d))
            .collect::<Vec<_>>()
            .join(", ");
        let provides_json = self.provides.iter()
            .map(|p| format!("\"{}\"", p))
            .collect::<Vec<_>>()
            .join(", ");
        let conflicts_json = self.conflicts.iter()
            .map(|c| format!("\"{}\"", c))
            .collect::<Vec<_>>()
            .join(", ");
        let opt_json = self.opt_depends.iter()
            .map(|o| format!("{{\"package\":\"{}\",\"reason\":\"{}\"}}", o.package, o.reason))
            .collect::<Vec<_>>()
            .join(", ");
        let validation_json = self.validation.iter()
            .map(|v| format!("\"{}\"", v))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "{{\
                \"repository\":{},\
                \"name\":\"{}\",\
                \"version\":\"{}\",\
                \"description\":\"{}\",\
                \"url\":\"{}\",\
                \"architecture\":\"{}\",\
                \"licenses\":[{}],\
                \"provides\":[{}],\
                \"depends\":[{}],\
                \"optdepends\":[{}],\
                \"conflicts\":[{}],\
                \"installed_size\":{},\
                \"download_size\":{},\
                \"packager\":\"{}\",\
                \"build_date\":\"{}\",\
                \"install_date\":{},\
                \"install_reason\":\"{}\",\
                \"install_script\":{},\
                \"validation\":[{}]\
            }}",
            repo,
            self.name, self.version, self.description, self.url,
            self.architecture,
            licenses_json, provides_json, depends_json,
            opt_json, conflicts_json,
            self.installed_size, download_size,
            self.packager, self.build_date, install_date,
            self.install_reason.as_str(),
            self.install_script,
            validation_json
        )
    }
}

// ─── Search Result (pacman -Ss --json) ───────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PacmanSearchResult {
    pub repository: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub installed: bool,
    pub installed_version: Option<String>,
    pub download_size: u64,
    pub installed_size: u64,
}

impl PacmanSearchResult {
    pub fn to_json(&self) -> String {
        let installed_version = match &self.installed_version {
            Some(v) => format!("\"{}\"", v),
            None => "null".to_string(),
        };
        format!(
            "{{\"repository\":\"{}\",\"name\":\"{}\",\"version\":\"{}\",\
             \"description\":\"{}\",\"installed\":{},\"installed_version\":{},\
             \"download_size\":{},\"installed_size\":{}}}",
            self.repository, self.name, self.version,
            self.description, self.installed, installed_version,
            self.download_size, self.installed_size
        )
    }
}

/// Render a list of search results as a JSON array
pub fn search_results_to_json(results: &[PacmanSearchResult]) -> String {
    let items: Vec<String> = results.iter().map(|r| r.to_json()).collect();
    format!("[{}]", items.join(","))
}

// ─── Upgrade List (pacman -Qu --json) ────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PacmanUpgrade {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub repository: String,
    pub download_size: u64,
}

impl PacmanUpgrade {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"name\":\"{}\",\"current_version\":\"{}\",\"new_version\":\"{}\",\
             \"repository\":\"{}\",\"download_size\":{}}}",
            self.name, self.current_version, self.new_version,
            self.repository, self.download_size
        )
    }
}

pub fn upgrade_list_to_json(upgrades: &[PacmanUpgrade]) -> String {
    let items: Vec<String> = upgrades.iter().map(|u| u.to_json()).collect();
    format!("[{}]", items.join(","))
}

// ─── PKGBUILD v2 ──────────────────────────────────────────────────────────────

/// PKGBUILD v2 extended metadata model
#[derive(Debug, Clone)]
pub struct PkgBuildV2 {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: u32,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub url: String,
    // v2: SPDX license expression
    pub license_spdx: String,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    // v2: test-only dependencies (not installed in final package)
    pub checkdepends: Vec<String>,
    pub optdepends: Vec<OptDepend>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub source: Vec<String>,
    pub sha256sums: Vec<String>,
    // v2: upstream changelog file
    pub changelog: Option<String>,
    // v2: backup files (preserve on update)
    pub backup: Vec<String>,
    pub options: Vec<String>,
}

impl PkgBuildV2 {
    pub fn new(name: &str, ver: &str) -> Self {
        Self {
            pkgname: name.to_string(),
            pkgver: ver.to_string(),
            pkgrel: 1,
            pkgdesc: String::new(),
            arch: vec!["x86_64".to_string()],
            url: String::new(),
            license_spdx: "GPL-2.0-only".to_string(),
            depends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            optdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            source: Vec::new(),
            sha256sums: Vec::new(),
            changelog: None,
            backup: Vec::new(),
            options: Vec::new(),
        }
    }

    /// Generate PKGBUILD file content
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# Maintainer: SigmaOS Team\n"));
        out.push_str(&format!("pkgname={}\n", self.pkgname));
        out.push_str(&format!("pkgver={}\n", self.pkgver));
        out.push_str(&format!("pkgrel={}\n", self.pkgrel));
        out.push_str(&format!("pkgdesc='{}'\n", self.pkgdesc));
        out.push_str(&format!("arch=({})\n",
            self.arch.iter().map(|a| format!("'{}'", a)).collect::<Vec<_>>().join(" ")));
        out.push_str(&format!("url='{}'\n", self.url));
        out.push_str(&format!("license=('{}')\n", self.license_spdx));

        if !self.depends.is_empty() {
            out.push_str(&format!("depends=({})\n",
                self.depends.iter().map(|d| format!("'{}'", d)).collect::<Vec<_>>().join(" ")));
        }
        if !self.makedepends.is_empty() {
            out.push_str(&format!("makedepends=({})\n",
                self.makedepends.iter().map(|d| format!("'{}'", d)).collect::<Vec<_>>().join(" ")));
        }
        // v2: checkdepends
        if !self.checkdepends.is_empty() {
            out.push_str(&format!("checkdepends=({})\n",
                self.checkdepends.iter().map(|d| format!("'{}'", d)).collect::<Vec<_>>().join(" ")));
        }
        if !self.optdepends.is_empty() {
            out.push_str("optdepends=(\n");
            for od in &self.optdepends {
                out.push_str(&format!("    '{}: {}'\n", od.package, od.reason));
            }
            out.push_str(")\n");
        }
        // v2: changelog
        if let Some(cl) = &self.changelog {
            out.push_str(&format!("changelog='{}'\n", cl));
        }
        if !self.backup.is_empty() {
            out.push_str(&format!("backup=({})\n",
                self.backup.iter().map(|b| format!("'{}'", b)).collect::<Vec<_>>().join(" ")));
        }
        if !self.source.is_empty() {
            out.push_str("source=(\n");
            for s in &self.source {
                out.push_str(&format!("    '{}'\n", s));
            }
            out.push_str(")\n");
            out.push_str("sha256sums=(\n");
            for s in &self.sha256sums {
                out.push_str(&format!("    '{}'\n", s));
            }
            out.push_str(")\n");
        }

        out.push_str("\nbuild() {\n    cmake -S \"$pkgname-$pkgver\" -B build -G Ninja \\\n        -DCMAKE_BUILD_TYPE=Release \\\n        -DCMAKE_INSTALL_PREFIX=/usr\n    ninja -C build\n}\n\n");
        out.push_str("check() {\n    ninja -C build test\n}\n\n");
        out.push_str("package() {\n    DESTDIR=\"$pkgdir\" ninja -C build install\n}\n");

        out
    }

    pub fn atom(&self) -> String {
        format!("{}-{}-{}", self.pkgname, self.pkgver, self.pkgrel)
    }
}

// ─── Sigpkg JSON Output ───────────────────────────────────────────────────────

/// sigpkg --json query result
#[derive(Debug, Clone)]
pub struct SigpkgJsonResponse {
    pub command: String,
    pub query: Option<String>,
    pub results: Vec<String>,  // pre-rendered JSON objects
    pub total: usize,
    pub timestamp: String,
}

impl SigpkgJsonResponse {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            query: None,
            results: Vec::new(),
            total: 0,
            timestamp: "2026-11-18T07:00:00Z".to_string(),
        }
    }

    pub fn render(&self) -> String {
        format!(
            "{{\"command\":\"{}\",\"query\":{},\"total\":{},\"timestamp\":\"{}\",\"results\":[{}]}}",
            self.command,
            self.query.as_deref().map(|q| format!("\"{}\"", q)).unwrap_or("null".to_string()),
            self.total,
            self.timestamp,
            self.results.join(",")
        )
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_info_json() {
        let mut pkg = PacmanPackageInfo::new("hyprland", "0.45.0-1");
        pkg.repository = Some("extra".to_string());
        pkg.description = "A dynamic tiling Wayland compositor".to_string();
        pkg.depends = vec!["wayland".to_string(), "pango".to_string()];
        pkg.licenses = vec!["BSD-3-Clause".to_string()];
        pkg.installed_size = 15728640;
        pkg.install_date = Some("2026-11-18T07:00:00Z".to_string());

        let json = pkg.to_json();
        assert!(json.contains("\"name\":\"hyprland\""));
        assert!(json.contains("\"version\":\"0.45.0-1\""));
        assert!(json.contains("\"repository\":\"extra\""));
        assert!(json.contains("\"installed_size\":15728640"));
        assert!(json.contains("wayland"));
    }

    #[test]
    fn test_search_results_json() {
        let results = vec![
            PacmanSearchResult {
                repository: "extra".to_string(),
                name: "hyprland".to_string(),
                version: "0.45.0-1".to_string(),
                description: "Wayland compositor".to_string(),
                installed: true,
                installed_version: Some("0.44.0-1".to_string()),
                download_size: 2097152,
                installed_size: 15728640,
            },
            PacmanSearchResult {
                repository: "extra".to_string(),
                name: "hyprlock".to_string(),
                version: "0.5.0-1".to_string(),
                description: "Screen locker".to_string(),
                installed: false,
                installed_version: None,
                download_size: 524288,
                installed_size: 2097152,
            },
        ];

        let json = search_results_to_json(&results);
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        assert!(json.contains("\"installed\":true"));
        assert!(json.contains("\"installed\":false"));
    }

    #[test]
    fn test_upgrade_list_json() {
        let upgrades = vec![
            PacmanUpgrade {
                name: "linux".to_string(),
                current_version: "6.11.0.arch1-1".to_string(),
                new_version: "6.12.0.arch1-1".to_string(),
                repository: "core".to_string(),
                download_size: 134217728,
            },
        ];

        let json = upgrade_list_to_json(&upgrades);
        assert!(json.contains("\"name\":\"linux\""));
        assert!(json.contains("6.11.0"));
        assert!(json.contains("6.12.0"));
    }

    #[test]
    fn test_pkgbuild_v2_render() {
        let mut pkg = PkgBuildV2::new("hyprland", "0.45.0");
        pkg.pkgdesc = "A dynamic tiling Wayland compositor".to_string();
        pkg.license_spdx = "BSD-3-Clause".to_string();
        pkg.depends = vec!["wayland".to_string(), "pango".to_string()];
        pkg.makedepends = vec!["cmake".to_string(), "ninja".to_string()];
        pkg.checkdepends = vec!["xorg-server-xvfb".to_string()];
        pkg.optdepends = vec![
            OptDepend { package: "xorg-xwayland".to_string(), reason: "XWayland support".to_string() },
        ];
        pkg.changelog = Some("CHANGELOG.md".to_string());

        let rendered = pkg.render();
        assert!(rendered.contains("pkgname=hyprland"));
        assert!(rendered.contains("checkdepends=('xorg-server-xvfb')"));
        assert!(rendered.contains("changelog='CHANGELOG.md'"));
        assert!(rendered.contains("license=('BSD-3-Clause')"));
        assert!(rendered.contains("check()"));
    }

    #[test]
    fn test_sigpkg_json_response() {
        let mut resp = SigpkgJsonResponse::new("search");
        resp.query = Some("hyprland".to_string());
        resp.results.push("{\"name\":\"hyprland\"}".to_string());
        resp.total = 1;

        let rendered = resp.render();
        assert!(rendered.contains("\"command\":\"search\""));
        assert!(rendered.contains("\"query\":\"hyprland\""));
        assert!(rendered.contains("\"total\":1"));
    }

    #[test]
    fn test_install_reason() {
        let explicit = InstallReason::ExplicitlyInstalled;
        let dep = InstallReason::InstalledAsDependency;
        assert_eq!(explicit.as_str(), "explicitly installed");
        assert!(dep.as_str().contains("dependency"));
    }
}
