// SPDX-License-Identifier: MIT
// SigmaOS Cross-Distribution Unifier Engine (sigma_distro_unifier)
// Unifies packaging, dependency mapping, and init systems across:
// - Arch Linux / Omarchy (ALPM, PKGBUILD)
// - Debian / Ubuntu (DPKG, APT, control)
// - Gentoo Linux (Portage, ebuild, USE flags, OpenRC)
// - Fedora / RHEL (RPM, DNF, spec)
// - Alpine Linux (APK, APKBUILD, musl)
// - NixOS (Nix expressions, functional closure)

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistroFamily {
    Arch,
    Debian,
    Gentoo,
    Fedora,
    Alpine,
    Nixos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitSystem {
    Systemd,
    OpenRC,
    Runit,
    SysVinit,
}

#[derive(Debug, Clone)]
pub struct UnifiedPackageSpec {
    pub name: String,
    pub version: String,
    pub release: u32,
    pub summary: String,
    pub license: String,
    pub url: String,
    pub generic_dependencies: Vec<String>,
}

pub struct DistroDependencyMapper {
    // generic_name -> (DistroFamily -> native_name)
    mappings: HashMap<String, HashMap<DistroFamily, String>>,
}

impl DistroDependencyMapper {
    pub fn standard() -> Self {
        let mut map = HashMap::new();

        let mut add = |generic: &str, arch: &str, deb: &str, gentoo: &str, fedora: &str, alpine: &str, nix: &str| {
            let mut d = HashMap::new();
            d.insert(DistroFamily::Arch, arch.to_string());
            d.insert(DistroFamily::Debian, deb.to_string());
            d.insert(DistroFamily::Gentoo, gentoo.to_string());
            d.insert(DistroFamily::Fedora, fedora.to_string());
            d.insert(DistroFamily::Alpine, alpine.to_string());
            d.insert(DistroFamily::Nixos, nix.to_string());
            map.insert(generic.to_string(), d);
        };

        add("openssl", "openssl", "libssl-dev", "dev-libs/openssl", "openssl-devel", "openssl-dev", "openssl");
        add("zlib", "zlib", "zlib1g-dev", "sys-libs/zlib", "zlib-devel", "zlib-dev", "zlib");
        add("sqlite", "sqlite", "libsqlite3-dev", "dev-db/sqlite", "sqlite-devel", "sqlite-dev", "sqlite");
        add("wayland", "wayland", "libwayland-dev", "dev-libs/wayland", "wayland-devel", "wayland-dev", "wayland");
        add("pipewire", "pipewire", "libpipewire-0.3-dev", "media-video/pipewire", "pipewire-devel", "pipewire-dev", "pipewire");

        Self { mappings: map }
    }

    pub fn translate(&self, generic_dep: &str, distro: DistroFamily) -> String {
        if let Some(target_map) = self.mappings.get(generic_dep) {
            if let Some(native) = target_map.get(&distro) {
                return native.clone();
            }
        }
        generic_dep.to_string()
    }
}

pub struct GentooUseFlagResolver {
    active_flags: HashSet<String>,
}

impl GentooUseFlagResolver {
    pub fn new(flags: &[&str]) -> Self {
        let mut s = HashSet::new();
        for &f in flags {
            s.insert(f.to_string());
        }
        Self { active_flags: s }
    }

    pub fn is_active(&self, flag: &str) -> bool {
        self.active_flags.contains(flag)
    }

    pub fn resolve_conditional_dep(&self, condition: &str, package: &str) -> Option<String> {
        if self.is_active(condition) {
            Some(package.to_string())
        } else {
            None
        }
    }
}

pub struct DistroRecipeRenderer;

impl DistroRecipeRenderer {
    pub fn render_pkgbuild(spec: &UnifiedPackageSpec, mapper: &DistroDependencyMapper) -> String {
        let deps: Vec<String> = spec.generic_dependencies
            .iter()
            .map(|d| format!("'{}'", mapper.translate(d, DistroFamily::Arch)))
            .collect();

        format!(
            "# Maintainer: SigmaOS Project <sigmaos@example.com>\n\
             pkgname={}\n\
             pkgver={}\n\
             pkgrel={}\n\
             pkgdesc=\"{}\"\n\
             arch=('x86_64')\n\
             url=\"{}\"\n\
             license=('{}')\n\
             depends=({})\n\
             package() {{\n\
               install -d \"${{pkgdir}}/usr/bin\"\n\
             }}\n",
            spec.name, spec.version, spec.release, spec.summary, spec.url, spec.license, deps.join(" ")
        )
    }

    pub fn render_debian_control(spec: &UnifiedPackageSpec, mapper: &DistroDependencyMapper) -> String {
        let deps: Vec<String> = spec.generic_dependencies
            .iter()
            .map(|d| mapper.translate(d, DistroFamily::Debian))
            .collect();

        format!(
            "Source: {}\n\
             Section: utils\n\
             Priority: optional\n\
             Maintainer: SigmaOS Team <team@sigmaos.org>\n\
             Standards-Version: 4.6.2\n\
             Homepage: {}\n\n\
             Package: {}\n\
             Architecture: any\n\
             Depends: ${{shlibs:Depends}}, ${{misc:Depends}}, {}\n\
             Description: {}\n",
            spec.name, spec.url, spec.name, deps.join(", "), spec.summary
        )
    }

    pub fn render_gentoo_ebuild(spec: &UnifiedPackageSpec, mapper: &DistroDependencyMapper) -> String {
        let rdepend: Vec<String> = spec.generic_dependencies
            .iter()
            .map(|d| mapper.translate(d, DistroFamily::Gentoo))
            .collect();

        format!(
            "# Copyright 2026 SigmaOS Authors\n\
             # Distributed under the terms of the GNU General Public License v3\n\n\
             EAPI=8\n\
             DESCRIPTION=\"{}\"\n\
             HOMEPAGE=\"{}\"\n\
             LICENSE=\"{}\"\n\
             SLOT=\"0\"\n\
             KEYWORDS=\"~amd64 ~arm64\"\n\
             IUSE=\"wayland pipewire lto\"\n\
             RDEPEND=\"{}\"\n",
            spec.summary, spec.url, spec.license, rdepend.join("\n\t")
        )
    }

    pub fn render_fedora_spec(spec: &UnifiedPackageSpec, mapper: &DistroDependencyMapper) -> String {
        let requires: Vec<String> = spec.generic_dependencies
            .iter()
            .map(|d| format!("Requires: {}", mapper.translate(d, DistroFamily::Fedora)))
            .collect();

        format!(
            "Name:           {}\n\
             Version:        {}\n\
             Release:        {}%{{?dist}}\n\
             Summary:        {}\n\
             License:        {}\n\
             URL:            {}\n\
             {}\n\n\
             %description\n\
             {}\n",
            spec.name, spec.version, spec.release, spec.summary, spec.license, spec.url, requires.join("\n"), spec.summary
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_distro_dependency_translation_aaa() {
        // Arrange
        let mapper = DistroDependencyMapper::standard();

        // Act
        let arch_ssl = mapper.translate("openssl", DistroFamily::Arch);
        let deb_ssl = mapper.translate("openssl", DistroFamily::Debian);
        let gentoo_ssl = mapper.translate("openssl", DistroFamily::Gentoo);
        let fedora_ssl = mapper.translate("openssl", DistroFamily::Fedora);
        let alpine_ssl = mapper.translate("openssl", DistroFamily::Alpine);

        // Assert
        assert_eq!(arch_ssl, "openssl");
        assert_eq!(deb_ssl, "libssl-dev");
        assert_eq!(gentoo_ssl, "dev-libs/openssl");
        assert_eq!(fedora_ssl, "openssl-devel");
        assert_eq!(alpine_ssl, "openssl-dev");
    }

    #[test]
    fn test_gentoo_useflag_resolution_aaa() {
        // Arrange
        let resolver = GentooUseFlagResolver::new(&["wayland", "lto"]);

        // Act
        let wayland_dep = resolver.resolve_conditional_dep("wayland", "gui-libs/wlroots");
        let x11_dep = resolver.resolve_conditional_dep("X", "x11-base/xorg-server");

        // Assert
        assert_eq!(wayland_dep, Some("gui-libs/wlroots".to_string()));
        assert_eq!(x11_dep, None);
    }

    #[test]
    fn test_arch_and_debian_recipe_render_aaa() {
        // Arrange
        let mapper = DistroDependencyMapper::standard();
        let spec = UnifiedPackageSpec {
            name: "sigmaos-desktop".to_string(),
            version: "1.0.0".to_string(),
            release: 1,
            summary: "Sovereign desktop environment".to_string(),
            license: "GPL-3.0-or-later".to_string(),
            url: "https://sigmaos.org".to_string(),
            generic_dependencies: vec!["openssl".to_string(), "wayland".to_string()],
        };

        // Act
        let pkgbuild = DistroRecipeRenderer::render_pkgbuild(&spec, &mapper);
        let control = DistroRecipeRenderer::render_debian_control(&spec, &mapper);

        // Assert
        assert!(pkgbuild.contains("pkgname=sigmaos-desktop"));
        assert!(pkgbuild.contains("'openssl' 'wayland'"));
        assert!(control.contains("Package: sigmaos-desktop"));
        assert!(control.contains("libssl-dev, libwayland-dev"));
    }
}
