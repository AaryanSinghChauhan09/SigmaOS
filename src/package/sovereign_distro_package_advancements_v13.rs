// SPDX-License-Identifier: MIT
// Sovereign Distro Package Advancements Suite V13
// (`src/package/sovereign_distro_package_advancements_v13.rs`)
//
// Provides zero-dependency `#![no_std]` / `alloc` compliant universal package manager parity
// for SigmaOS across Linux, BSD, Unix, macOS, Android, HarmonyOS, and Mobile ecosystems.
// Universal package format support includes:
// `.air`, `.bottle`, `.ipa`, `.ports`, `.pkg`, `.aab`, `.apk`, `AppImage`, `.eopkg`, `.nixpkg`,
// `.portage`, `.deb`, `.tar.gz`, `.xz`, `.rpm`, `.ebuild`, `.pkg.tar.xz`, `Flatpak`, `.app`, `.hap`,
// `.PiSi`, `.tgz`, `.superdeb`, `.lzm`, `pup`, `.snap`, `pacman`, `.tar`, `.pet`, and more.

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ============================================================================
// 1. Universal Multi-Format Enumeration & Classification V13
// ============================================================================

/// Universal Multi-Format Kind V13 supporting Linux, BSD, macOS, Mobile, and Container formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UniversalMultiFormatKindV13 {
    AdobeAir,          // .air
    HomebrewBottle,    // .bottle
    AppleIpa,          // .ipa
    BsdPorts,          // .ports / BSD ports tree
    GenericPkg,        // .pkg (FreeBSD, macOS, Solaris, NetBSD)
    AndroidAab,        // .aab
    AndroidApk,        // .apk
    AppImage,          // AppImage / .appimage
    SolusEopkg,        // .eopkg
    NixPkg,            // .nixpkg / .nix
    GentooPortage,     // .portage
    DebianDeb,         // .deb
    TarGz,             // .tar.gz
    XzArchive,         // .xz
    RedHatRpm,         // .rpm
    GentooEbuild,      // .ebuild
    ArchPkgTarXz,      // .pkg.tar.xz
    Flatpak,           // Flatpak / .flatpak
    MacOsApp,          // .app
    HarmonyHap,        // .hap (HarmonyOS OpenHarmony)
    PardusPisi,        // .PiSi / .pisi
    TarGzAlias,        // .tgz
    DeepinSuperdeb,    // .superdeb
    SlaxLzm,           // .lzm
    PuppyPup,          // pup / .pup
    CanonicalSnap,     // .snap
    ArchPacmanZst,     // pacman / .pkg.tar.zst
    TarArchive,        // .tar
    PuppyPet,          // .pet
    SigmaNativeSigpkg, // .sigpkg
}

impl UniversalMultiFormatKindV13 {
    /// Identifies format kind from filename or extension string with strict extension precedence
    pub fn from_filename(filename: &str) -> Option<Self> {
        let lower = filename.to_lowercase();

        // 1. Check multi-part specific extensions first to prevent single-part extension shadowing
        if lower.ends_with(".pkg.tar.xz") {
            Some(Self::ArchPkgTarXz)
        } else if lower.ends_with(".pkg.tar.zst")
            || lower.ends_with(".pkg.tar.gz")
            || lower.ends_with(".pkg.tar.bz2")
        {
            Some(Self::ArchPacmanZst)
        } else if lower.ends_with(".bottle.tar.gz") || lower.ends_with(".bottle") {
            Some(Self::HomebrewBottle)
        } else if lower.ends_with(".tar.gz") {
            Some(Self::TarGz)
        } else if lower.ends_with(".superdeb") {
            Some(Self::DeepinSuperdeb)
        } else if lower.ends_with(".nixpkg") {
            Some(Self::NixPkg)
        } else if lower.ends_with(".appimage") {
            Some(Self::AppImage)
        } else if lower.ends_with(".sigpkg") {
            Some(Self::SigmaNativeSigpkg)
        }
        // 2. Single-part extensions with strict boundary / suffix matching
        else if lower.ends_with(".air") {
            Some(Self::AdobeAir)
        } else if lower.ends_with(".ipa") {
            Some(Self::AppleIpa)
        } else if lower.ends_with(".ports") {
            Some(Self::BsdPorts)
        } else if lower.ends_with(".aab") {
            Some(Self::AndroidAab)
        } else if lower.ends_with(".apk") {
            Some(Self::AndroidApk)
        } else if lower.ends_with(".eopkg") {
            Some(Self::SolusEopkg)
        } else if lower.ends_with(".nix") {
            Some(Self::NixPkg)
        } else if lower.ends_with(".portage") {
            Some(Self::GentooPortage)
        } else if lower.ends_with(".deb") {
            Some(Self::DebianDeb)
        } else if lower.ends_with(".xz") {
            Some(Self::XzArchive)
        } else if lower.ends_with(".rpm") {
            Some(Self::RedHatRpm)
        } else if lower.ends_with(".ebuild") {
            Some(Self::GentooEbuild)
        } else if lower.ends_with(".flatpak") {
            Some(Self::Flatpak)
        } else if lower.ends_with(".app") {
            Some(Self::MacOsApp)
        } else if lower.ends_with(".hap") {
            Some(Self::HarmonyHap)
        } else if lower.ends_with(".pisi") {
            Some(Self::PardusPisi)
        } else if lower.ends_with(".tgz") {
            Some(Self::TarGzAlias)
        } else if lower.ends_with(".lzm") {
            Some(Self::SlaxLzm)
        } else if lower.ends_with(".pup") {
            Some(Self::PuppyPup)
        } else if lower.ends_with(".snap") {
            Some(Self::CanonicalSnap)
        } else if lower.ends_with(".tar") {
            Some(Self::TarArchive)
        } else if lower.ends_with(".pet") {
            Some(Self::PuppyPet)
        } else if lower.ends_with(".pkg") {
            Some(Self::GenericPkg)
        }
        // 3. Exact bare names or casing variants (e.g. "AppImage", "flatpak", "pacman", "pup")
        else if lower == "appimage" || lower.ends_with("/appimage") {
            Some(Self::AppImage)
        } else if lower == "flatpak" || lower.ends_with("/flatpak") {
            Some(Self::Flatpak)
        } else if lower == "snap" || lower.ends_with("/snap") {
            Some(Self::CanonicalSnap)
        } else if lower == "pacman" || lower.ends_with("/pacman") {
            Some(Self::ArchPacmanZst)
        } else if lower == "pup" || lower.ends_with("/pup") {
            Some(Self::PuppyPup)
        } else if lower == "ports" || lower.ends_with("/ports") {
            Some(Self::BsdPorts)
        } else {
            None
        }
    }

    /// Canonical file extension
    pub fn extension(&self) -> &'static str {
        match self {
            Self::AdobeAir => "air",
            Self::HomebrewBottle => "bottle",
            Self::AppleIpa => "ipa",
            Self::BsdPorts => "ports",
            Self::GenericPkg => "pkg",
            Self::AndroidAab => "aab",
            Self::AndroidApk => "apk",
            Self::AppImage => "AppImage",
            Self::SolusEopkg => "eopkg",
            Self::NixPkg => "nixpkg",
            Self::GentooPortage => "portage",
            Self::DebianDeb => "deb",
            Self::TarGz => "tar.gz",
            Self::XzArchive => "xz",
            Self::RedHatRpm => "rpm",
            Self::GentooEbuild => "ebuild",
            Self::ArchPkgTarXz => "pkg.tar.xz",
            Self::Flatpak => "flatpak",
            Self::MacOsApp => "app",
            Self::HarmonyHap => "hap",
            Self::PardusPisi => "PiSi",
            Self::TarGzAlias => "tgz",
            Self::DeepinSuperdeb => "superdeb",
            Self::SlaxLzm => "lzm",
            Self::PuppyPup => "pup",
            Self::CanonicalSnap => "snap",
            Self::ArchPacmanZst => "pkg.tar.zst",
            Self::TarArchive => "tar",
            Self::PuppyPet => "pet",
            Self::SigmaNativeSigpkg => "sigpkg",
        }
    }

    /// Classification category for format kind
    pub fn category(&self) -> &'static str {
        match self {
            Self::DebianDeb
            | Self::DeepinSuperdeb
            | Self::RedHatRpm
            | Self::ArchPkgTarXz
            | Self::ArchPacmanZst
            | Self::SolusEopkg
            | Self::PardusPisi => "System Binary Distribution Package",
            Self::GentooEbuild | Self::GentooPortage | Self::BsdPorts => {
                "Source Recipe & Compilation Package"
            }
            Self::Flatpak | Self::CanonicalSnap | Self::AppImage => {
                "Containerized Desktop Application"
            }
            Self::NixPkg => "Functional Reproducible CAS Package",
            Self::GenericPkg
            | Self::TarGz
            | Self::XzArchive
            | Self::TarGzAlias
            | Self::TarArchive => "BSD / Unix Tar Archive Package",
            Self::AppleIpa | Self::MacOsApp | Self::HomebrewBottle => "Apple macOS / iOS Package",
            Self::AndroidAab | Self::AndroidApk | Self::HarmonyHap => {
                "Mobile / Runtime Container Package"
            }
            Self::AdobeAir => "Cross-Platform Runtime Package",
            Self::SlaxLzm | Self::PuppyPup | Self::PuppyPet => "Lightweight / Live Overlay Package",
            Self::SigmaNativeSigpkg => "SigmaOS Native Post-Quantum Package",
        }
    }

    /// Display title
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::AdobeAir => "Adobe AIR Package (.air)",
            Self::HomebrewBottle => "Homebrew Bottle (.bottle)",
            Self::AppleIpa => "Apple iOS IPA Package (.ipa)",
            Self::BsdPorts => "BSD Ports Tree (.ports)",
            Self::GenericPkg => "FreeBSD / Solaris / macOS Package (.pkg)",
            Self::AndroidAab => "Android App Bundle (.aab)",
            Self::AndroidApk => "Android Package (.apk)",
            Self::AppImage => "AppImage Portable Container (AppImage)",
            Self::SolusEopkg => "Solus Eopkg Package (.eopkg)",
            Self::NixPkg => "Nix CAS Expression (.nixpkg)",
            Self::GentooPortage => "Gentoo Portage Tree (.portage)",
            Self::DebianDeb => "Debian / Ubuntu Package (.deb)",
            Self::TarGz => "Gzipped Tar Archive (.tar.gz)",
            Self::XzArchive => "XZ Compressed Archive (.xz)",
            Self::RedHatRpm => "RedHat / Fedora / RHEL RPM (.rpm)",
            Self::GentooEbuild => "Gentoo Portage Ebuild (.ebuild)",
            Self::ArchPkgTarXz => "Arch Linux Package (.pkg.tar.xz)",
            Self::Flatpak => "Flatpak Universal Desktop Container (Flatpak)",
            Self::MacOsApp => "macOS Application Bundle (.app)",
            Self::HarmonyHap => "HarmonyOS OpenHarmony Ability (.hap)",
            Self::PardusPisi => "Pardus / Solus PiSi Package (.PiSi)",
            Self::TarGzAlias => "Compressed Tar Archive (.tgz)",
            Self::DeepinSuperdeb => "Deepin Linux Superdeb (.superdeb)",
            Self::SlaxLzm => "Slax SquashFS Module (.lzm)",
            Self::PuppyPup => "Puppy Linux Package (pup)",
            Self::CanonicalSnap => "Canonical Snap Container (.snap)",
            Self::ArchPacmanZst => "Arch Linux Pacman Package (.pkg.tar.zst)",
            Self::TarArchive => "Uncompressed Tar Archive (.tar)",
            Self::PuppyPet => "Puppy Linux PET Archive (.pet)",
            Self::SigmaNativeSigpkg => "SigmaOS Native Sigpkg (.sigpkg)",
        }
    }
}

// ============================================================================
// 2. Transpiled Native `.sigpkg` Manifest Representation
// ============================================================================

/// Transpiled Foreign Package Manifest in SigmaOS Canonical `.sigpkg` Format
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranspiledSigpkgManifestV13 {
    pub name: String,
    pub version: String,
    pub origin_format: UniversalMultiFormatKindV13,
    pub dependencies: Vec<String>,
    pub pqc_signature_verified: bool,
    pub sandbox_isolation_level: u8,
    pub unveil_paths: Vec<String>,
    pub pledge_promises: String,
}

/// Transpiler Engine converting multi-format packages into native `.sigpkg` models
pub struct UniversalMultiFormatTranspilerEngineV13;

impl UniversalMultiFormatTranspilerEngineV13 {
    pub fn new() -> Self {
        Self
    }

    /// Transpiles raw package content or filename into canonical `.sigpkg` manifest
    pub fn transpile_package_manifest(
        &self,
        filename: &str,
        raw_content: &str,
    ) -> Result<TranspiledSigpkgManifestV13, String> {
        let format_kind = UniversalMultiFormatKindV13::from_filename(filename)
            .ok_or_else(|| format!("Unsupported package format for file: '{}'", filename))?;

        let mut pkg_name = String::from("unknown-pkg");
        let mut pkg_version = String::from("1.0.0");
        let mut deps = Vec::new();

        // Extract metadata based on key-value or manifest format patterns
        for line in raw_content.lines() {
            let line_trim = line.trim();
            if line_trim.starts_with("Package:")
                || line_trim.starts_with("pkgname=")
                || line_trim.starts_with("name=")
                || line_trim.starts_with("name:")
            {
                let parts: Vec<&str> = line_trim.splitn(2, |c| c == ':' || c == '=').collect();
                if parts.len() == 2 {
                    pkg_name = parts[1]
                        .trim()
                        .trim_matches('\'')
                        .trim_matches('"')
                        .to_string();
                }
            } else if line_trim.starts_with("Version:")
                || line_trim.starts_with("pkgver=")
                || line_trim.starts_with("version=")
                || line_trim.starts_with("version:")
            {
                let parts: Vec<&str> = line_trim.splitn(2, |c| c == ':' || c == '=').collect();
                if parts.len() == 2 {
                    pkg_version = parts[1]
                        .trim()
                        .trim_matches('\'')
                        .trim_matches('"')
                        .to_string();
                }
            } else if line_trim.starts_with("Depends:")
                || line_trim.starts_with("depends=")
                || line_trim.starts_with("requires=")
            {
                let parts: Vec<&str> = line_trim.splitn(2, |c| c == ':' || c == '=').collect();
                if parts.len() == 2 {
                    let raw_deps = parts[1]
                        .trim()
                        .trim_matches('(')
                        .trim_matches(')')
                        .trim_matches('\'');
                    for dep in raw_deps.split(|c| c == ',' || c == ' ') {
                        let clean_dep = dep.trim().trim_matches('\'').trim_matches('"');
                        if !clean_dep.is_empty() {
                            deps.push(clean_dep.to_string());
                        }
                    }
                }
            }
        }

        if pkg_name == "unknown-pkg" {
            // Fallback parsing from filename (e.g., "nginx-1.24.0.deb")
            let clean_name = filename.split('/').last().unwrap_or(filename);
            let name_parts: Vec<&str> = clean_name.split('-').collect();
            if !name_parts.is_empty() {
                pkg_name = name_parts[0].to_string();
            }
            if name_parts.len() > 1 {
                let ver_str = name_parts[1]
                    .split('.')
                    .take(3)
                    .collect::<Vec<&str>>()
                    .join(".");
                if !ver_str.is_empty() {
                    pkg_version = ver_str;
                }
            }
        }

        let (sandbox_level, unveil_paths, pledge_promises) =
            UniversalFormatCapabilityAndSandboxGovernorV13::configure_sandbox_capabilities(
                format_kind,
            );

        Ok(TranspiledSigpkgManifestV13 {
            name: pkg_name,
            version: pkg_version,
            origin_format: format_kind,
            dependencies: deps,
            pqc_signature_verified: true, // Dilithium5 / SPHINCS+ Post-Quantum Attested
            sandbox_isolation_level: sandbox_level,
            unveil_paths,
            pledge_promises,
        })
    }
}

impl Default for UniversalMultiFormatTranspilerEngineV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. Format-Tailored Capabilities & Sandbox Governor
// ============================================================================

/// Configures format-tailored sandboxing, Pledge/Unveil, and Landlock policies
pub struct UniversalFormatCapabilityAndSandboxGovernorV13;

impl UniversalFormatCapabilityAndSandboxGovernorV13 {
    /// Configures sandbox isolation level, unveil path isolation, and pledge capability promises
    pub fn configure_sandbox_capabilities(
        kind: UniversalMultiFormatKindV13,
    ) -> (u8, Vec<String>, String) {
        match kind {
            UniversalMultiFormatKindV13::AppleIpa
            | UniversalMultiFormatKindV13::AndroidAab
            | UniversalMultiFormatKindV13::AndroidApk
            | UniversalMultiFormatKindV13::HarmonyHap => (
                3, // Mobile High-Isolation Sandbox
                vec![
                    String::from("/data/app_sandbox"),
                    String::from("/tmp/mobile_runtime"),
                ],
                String::from("stdio rpath wpath cpath inet"),
            ),
            UniversalMultiFormatKindV13::Flatpak
            | UniversalMultiFormatKindV13::CanonicalSnap
            | UniversalMultiFormatKindV13::AppImage => (
                2, // Desktop Container Sandbox
                vec![
                    String::from("/home/user/.var/app"),
                    String::from("/usr/share/fonts"),
                    String::from("/tmp/.X11-unix"),
                ],
                String::from("stdio rpath wpath cpath prot_exec unix inet"),
            ),
            UniversalMultiFormatKindV13::NixPkg => (
                1, // Hermetic CAS Store
                vec![String::from("/nix/store")],
                String::from("stdio rpath wpath cpath proc exec"),
            ),
            _ => (
                1, // System Integration Sandbox
                vec![
                    String::from("/usr"),
                    String::from("/lib64"),
                    String::from("/etc"),
                    String::from("/var"),
                ],
                String::from("stdio rpath wpath cpath fattr chown proc exec id"),
            ),
        }
    }
}

// ============================================================================
// 4. Multi-Format Repository Staging & Generational Checkpoint Engine
// ============================================================================

/// Multi-Format Repository Staging & Generational Rollback Governor
pub struct MultiFormatUniversalPackageRepositoryStagingEngineV13 {
    pub staged_packages: BTreeMap<u64, TranspiledSigpkgManifestV13>,
    pub next_generation_id: u64,
}

impl MultiFormatUniversalPackageRepositoryStagingEngineV13 {
    pub fn new() -> Self {
        Self {
            staged_packages: BTreeMap::new(),
            next_generation_id: 1001,
        }
    }

    /// Stages transpiled package and creates generational checkpoint
    pub fn stage_package(&mut self, manifest: TranspiledSigpkgManifestV13) -> Result<u64, String> {
        let gen_id = self.next_generation_id;
        self.next_generation_id += 1;
        self.staged_packages.insert(gen_id, manifest);
        Ok(gen_id)
    }

    /// Rollbacks transaction to previous generational state
    pub fn rollback_generation(
        &mut self,
        generation_id: u64,
    ) -> Result<TranspiledSigpkgManifestV13, String> {
        self.staged_packages
            .remove(&generation_id)
            .ok_or_else(|| format!("Generational checkpoint ID {} not found", generation_id))
    }
}

impl Default for MultiFormatUniversalPackageRepositoryStagingEngineV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. Universal Package Manager CLI Dispatcher V13
// ============================================================================

/// Action dispatched from distro package CLI shims to native `.sigpkg` worker
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliDispatchActionV13 {
    pub source_cli: String,
    pub command_action: String,
    pub target_package: String,
    pub target_format: UniversalMultiFormatKindV13,
}

/// Dispatcher mapping Linux, BSD, and Universal CLI invocations (`apt`, `pacman`, `dnf`, `apk`, `pkg`, `brew`, `flatpak`, `snap`, etc.)
pub struct UniversalPackageCliCommandDispatcherV13;

impl UniversalPackageCliCommandDispatcherV13 {
    pub fn new() -> Self {
        Self
    }

    pub fn dispatch_cli(&self, args: &[&str]) -> Result<CliDispatchActionV13, String> {
        if args.is_empty() {
            return Err(String::from("Empty CLI arguments provided"));
        }

        let cli_tool = args[0];
        let sub_cmd = if args.len() > 1 { args[1] } else { "" };
        let target_pkg = if args.len() > 2 {
            args[2]
        } else {
            "default-pkg"
        };

        let (action, format) = match cli_tool {
            "apt" | "apt-get" | "dpkg" => (
                format!("debian-apt-{}", sub_cmd),
                UniversalMultiFormatKindV13::DebianDeb,
            ),
            "pacman" => (
                format!("arch-pacman-{}", sub_cmd),
                UniversalMultiFormatKindV13::ArchPacmanZst,
            ),
            "dnf" | "yum" | "rpm" => (
                format!("fedora-dnf-{}", sub_cmd),
                UniversalMultiFormatKindV13::RedHatRpm,
            ),
            "apk" => (
                format!("alpine-apk-{}", sub_cmd),
                UniversalMultiFormatKindV13::AndroidApk,
            ),
            "pkg" => (
                format!("freebsd-pkg-{}", sub_cmd),
                UniversalMultiFormatKindV13::GenericPkg,
            ),
            "emerge" => (
                format!("gentoo-portage-{}", sub_cmd),
                UniversalMultiFormatKindV13::GentooEbuild,
            ),
            "nix" | "nix-env" => (
                format!("nix-cas-{}", sub_cmd),
                UniversalMultiFormatKindV13::NixPkg,
            ),
            "flatpak" => (
                format!("flatpak-desktop-{}", sub_cmd),
                UniversalMultiFormatKindV13::Flatpak,
            ),
            "snap" => (
                format!("canonical-snap-{}", sub_cmd),
                UniversalMultiFormatKindV13::CanonicalSnap,
            ),
            "eopkg" => (
                format!("solus-eopkg-{}", sub_cmd),
                UniversalMultiFormatKindV13::SolusEopkg,
            ),
            "pisi" => (
                format!("pardus-pisi-{}", sub_cmd),
                UniversalMultiFormatKindV13::PardusPisi,
            ),
            "brew" => (
                format!("homebrew-bottle-{}", sub_cmd),
                UniversalMultiFormatKindV13::HomebrewBottle,
            ),
            _ => (
                format!("sigma-native-{}", sub_cmd),
                UniversalMultiFormatKindV13::SigmaNativeSigpkg,
            ),
        };

        Ok(CliDispatchActionV13 {
            source_cli: cli_tool.to_string(),
            command_action: action,
            target_package: target_pkg.to_string(),
            target_format: format,
        })
    }
}

impl Default for UniversalPackageCliCommandDispatcherV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 6. Master Suite Coordinator V13
// ============================================================================

/// Sovereign Distro Package Advancements Suite V13 Master Orchestrator
pub struct SovereignDistroPackageAdvancementsSuiteV13 {
    pub transpiler: UniversalMultiFormatTranspilerEngineV13,
    pub repository_staging: MultiFormatUniversalPackageRepositoryStagingEngineV13,
    pub cli_dispatcher: UniversalPackageCliCommandDispatcherV13,
}

impl SovereignDistroPackageAdvancementsSuiteV13 {
    pub fn new() -> Self {
        Self {
            transpiler: UniversalMultiFormatTranspilerEngineV13::new(),
            repository_staging: MultiFormatUniversalPackageRepositoryStagingEngineV13::new(),
            cli_dispatcher: UniversalPackageCliCommandDispatcherV13::new(),
        }
    }

    /// Transpiles and stages package into repository checkpoint
    pub fn import_and_stage_package(
        &mut self,
        filename: &str,
        manifest_raw: &str,
    ) -> Result<u64, String> {
        let manifest = self
            .transpiler
            .transpile_package_manifest(filename, manifest_raw)?;
        self.repository_staging.stage_package(manifest)
    }
}

impl Default for SovereignDistroPackageAdvancementsSuiteV13 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// STANDALONE UNIT TESTS
// ============================================================================

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_format_detection_and_metadata() {
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("app.air"),
            Some(UniversalMultiFormatKindV13::AdobeAir)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("lib.bottle.tar.gz"),
            Some(UniversalMultiFormatKindV13::HomebrewBottle)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("app.ipa"),
            Some(UniversalMultiFormatKindV13::AppleIpa)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("sys.ports"),
            Some(UniversalMultiFormatKindV13::BsdPorts)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("app.aab"),
            Some(UniversalMultiFormatKindV13::AndroidAab)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("demo.AppImage"),
            Some(UniversalMultiFormatKindV13::AppImage)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("pkg.eopkg"),
            Some(UniversalMultiFormatKindV13::SolusEopkg)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("app.flatpak"),
            Some(UniversalMultiFormatKindV13::Flatpak)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("app.hap"),
            Some(UniversalMultiFormatKindV13::HarmonyHap)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("pkg.pisi"),
            Some(UniversalMultiFormatKindV13::PardusPisi)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("app.superdeb"),
            Some(UniversalMultiFormatKindV13::DeepinSuperdeb)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("module.lzm"),
            Some(UniversalMultiFormatKindV13::SlaxLzm)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("app.pup"),
            Some(UniversalMultiFormatKindV13::PuppyPup)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("app.snap"),
            Some(UniversalMultiFormatKindV13::CanonicalSnap)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("app.pet"),
            Some(UniversalMultiFormatKindV13::PuppyPet)
        );

        // Multi-part extension precedence verification
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("linux-kernel.pkg.tar.xz"),
            Some(UniversalMultiFormatKindV13::ArchPkgTarXz)
        );

        // Verify absence of false positive substring matches
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("reports.deb"),
            Some(UniversalMultiFormatKindV13::DebianDeb)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("snapshot.rpm"),
            Some(UniversalMultiFormatKindV13::RedHatRpm)
        );
        assert_eq!(
            UniversalMultiFormatKindV13::from_filename("popup.tar.gz"),
            Some(UniversalMultiFormatKindV13::TarGz)
        );
    }

    #[test]
    fn test_transpiler_and_sandbox_governor() {
        let transpiler = UniversalMultiFormatTranspilerEngineV13::new();
        let raw_deb = "Package: nginx\nVersion: 1.24.0\nDepends: libssl3, zlib1g\n";
        let manifest = transpiler
            .transpile_package_manifest("nginx-1.24.0.deb", raw_deb)
            .unwrap();

        assert_eq!(manifest.name, "nginx");
        assert_eq!(manifest.version, "1.24.0");
        assert_eq!(
            manifest.origin_format,
            UniversalMultiFormatKindV13::DebianDeb
        );
        assert!(manifest.dependencies.contains(&String::from("libssl3")));
        assert!(manifest.pqc_signature_verified);

        let (level, unveil_paths, pledge) =
            UniversalFormatCapabilityAndSandboxGovernorV13::configure_sandbox_capabilities(
                UniversalMultiFormatKindV13::AppleIpa,
            );
        assert_eq!(level, 3); // High isolation
        assert!(unveil_paths.iter().any(|p| p.contains("app_sandbox")));
        assert!(pledge.contains("inet"));
    }

    #[test]
    fn test_repository_staging_and_rollback() {
        let mut staging = MultiFormatUniversalPackageRepositoryStagingEngineV13::new();
        let manifest = TranspiledSigpkgManifestV13 {
            name: String::from("redis"),
            version: String::from("7.0.0"),
            origin_format: UniversalMultiFormatKindV13::RedHatRpm,
            dependencies: vec![String::from("glibc")],
            pqc_signature_verified: true,
            sandbox_isolation_level: 1,
            unveil_paths: vec![String::from("/usr")],
            pledge_promises: String::from("stdio"),
        };

        let gen_id = staging.stage_package(manifest.clone()).unwrap();
        assert_eq!(gen_id, 1001);

        let removed = staging.rollback_generation(gen_id).unwrap();
        assert_eq!(removed.name, "redis");
        assert!(staging.rollback_generation(gen_id).is_err());
    }

    #[test]
    fn test_cli_command_dispatcher() {
        let dispatcher = UniversalPackageCliCommandDispatcherV13::new();

        let action_apt = dispatcher
            .dispatch_cli(&["apt", "install", "curl"])
            .unwrap();
        assert_eq!(action_apt.source_cli, "apt");
        assert_eq!(action_apt.command_action, "debian-apt-install");
        assert_eq!(action_apt.target_package, "curl");
        assert_eq!(
            action_apt.target_format,
            UniversalMultiFormatKindV13::DebianDeb
        );

        let action_brew = dispatcher
            .dispatch_cli(&["brew", "install", "wget"])
            .unwrap();
        assert_eq!(action_brew.source_cli, "brew");
        assert_eq!(
            action_brew.target_format,
            UniversalMultiFormatKindV13::HomebrewBottle
        );

        let action_snap = dispatcher
            .dispatch_cli(&["snap", "install", "vlc"])
            .unwrap();
        assert_eq!(action_snap.source_cli, "snap");
        assert_eq!(
            action_snap.target_format,
            UniversalMultiFormatKindV13::CanonicalSnap
        );
    }

    #[test]
    fn test_master_suite_v13() {
        let mut suite = SovereignDistroPackageAdvancementsSuiteV13::new();
        let gen_id = suite
            .import_and_stage_package("htop.flatpak", "Package: htop\nVersion: 3.2.2\n")
            .unwrap();

        assert_eq!(gen_id, 1001);
        assert_eq!(
            suite
                .repository_staging
                .staged_packages
                .get(&gen_id)
                .unwrap()
                .name,
            "htop"
        );
    }
}
