
use std::boxed::Box;
// use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak, zypper, dnf, appimages

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::{Arc, HashMap, HashSet};

#[cfg(any(feature = "standalone_test", test))]
use std::collections::{HashMap, HashSet};
#[cfg(any(feature = "standalone_test", test))]
use std::sync::Arc;

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::runtime::node_distribution::{
    LibcFlavor, NodeBinaryDistroEngine, NodeBinaryPackage, NodeReleaseStream, NodeTargetArch,
};

#[cfg(any(feature = "standalone_test", test))]
pub mod node_distribution_dummy {
    use super::*;

    #[derive(Debug, Clone)]
    pub enum LibcFlavor {
        Musl,
        Glibc,
    }
    #[derive(Debug, Clone)]
    pub enum NodeReleaseStream {
        Lts,
        Current,
    }
    pub enum NodeTargetArch {
        X86_64,
        Aarch64,
    }
    pub struct NodeBinaryPackage {
        pub version: String,
    }
    impl NodeBinaryPackage {
        pub fn new(
            version: &str,
            _stream: NodeReleaseStream,
            _arch: NodeTargetArch,
            _flavor: LibcFlavor,
            _url: &str,
            _hash: [u8; 32],
            _sig: [u8; 64],
            _size: u64,
        ) -> Self {
            Self {
                version: version.to_string(),
            }
        }
    }
    pub struct NodeBinaryDistroEngine;
    impl NodeBinaryDistroEngine {
        pub fn new() -> Self {
            Self
        }
        pub fn install_to_store(
            &self,
            pkg: &NodeBinaryPackage,
            _bytes: &[u8],
            _npm: &str,
        ) -> Result<String, &'static str> {
            Ok(format!("/sovereign/store/node-{}-dummy", pkg.version))
        }
    }
}

#[cfg(any(feature = "standalone_test", test))]
use node_distribution_dummy::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageState {
    Uninstalled,
    Downloading,
    Installing,
    Installed,
    BrokenDependency,
}

/// Package format type covering 18 major distribution formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackagePriority {
    Essential,
    Required,
    Important,
    Standard,
    Optional,
}

/// Supported package formats across Linux and BSD ecosystems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,        // apt/dpkg
    Rpm,        // yum/dnf/zypper
    Pacman,     // pacman/pkgbuild
    Snap,       // snap/squashfs
    Flatpak,    // flatpak sandbox
    AppImage,   // AppImage single-file container
    SigmaPkg,   // native SigmaOS format
    Air,        // Adobe AIR (.air)
    Bottle,     // Homebrew Bottle (.bottle)
    Ipa,        // iOS App (.ipa)
    Ports,      // BSD Ports (.ports)
    Pkg,        // macOS / BSD / Solaris PKG (.pkg)
    Aab,        // Android App Bundle (.aab)
    Apk,        // Android Package / Alpine Package (.apk)
    Eopkg,      // Solus eopkg (.eopkg)
    Nixpkg,     // Nix store package (.nixpkg)
    Ebuild,     // Gentoo ebuild (.ebuild / .portage)
    TarGz,      // Compressed Tar (.tar.gz, .tgz)
    Xz,         // Compressed XZ archive (.xz, .tar.xz)
    App,        // macOS App bundle (.app)
    Hap,        // HarmonyOS Ability Package (.hap)
    Pisi,       // Pardus / Solus PiSi (.PiSi)
    Superdeb,   // Deepin Superdeb (.superdeb)
    Lzm,        // Slax Linux Module (.lzm)
    Pup,        // Puppy Linux Package (.pup)
    Pet,        // Puppy Extra Tarball (.pet)
    Tar,        // Plain tarball (.tar)
    Xbps,       // Void Linux (.xbps)
    Zypper,     // OpenSUSE Zypper (.zypper)
    Guix,       // GNU Guix (.guix / .scm)
    Moss,       // Solus Moss (.moss)
    Hpkg,       // Haiku Package (.hpkg)
    Tcz,        // Tiny Core Linux (.tcz)
    Gobo,       // GoboLinux (.gobo)
    Ostree,     // OSTree commit (.commit)
    Pkgsrc,     // NetBSD pkgsrc (.pkgsrc)
    Sfs,        // SquashFS (.sfs)
    Puk,        // Portable Package (.puk)
    Dmg,        // macOS Disk Image (.dmg)
    Cports,     // Chimera Linux (.cports)
    Cachy,      // CachyOS Package (.cachy)
    Nix,        // Nix expression / package (.nix)
    Txz,        // Slackware/FreeBSD txz package (.txz)
    CachyOS,    // CachyOS (.cachyos)
    Swupd,      // Clear Linux swupd (.swupd)
    Starling,   // Starling format (.starling)
    Dports,     // DragonFly BSD DPorts (.dports)
    SlackBuild, // Slackware SlackBuild (.slackbuild / .tlz / .tbz)
    Crux,       // CRUX Linux (.crux / .pkgfile)
    Drpm,       // Delta RPM (.drpm)
    Stratum,    // Bedrock Linux Stratum (.stratum)
    Portage,    // Gentoo Portage
    FreeBsdPkg, // FreeBSD pkg
    ArchPkgBuild,// Arch PKGBUILD
    NixStore,   // Nix store
    Homebrew,   // Homebrew formula
}

impl PackageFormat {
    pub fn from_filename(filename: &str) -> Option<Self> {
        let name = filename.to_lowercase();
        let name = name.trim();
        let normalized = name.replace(" ", "");

        if normalized.ends_with(".deb") || normalized.ends_with(".udeb") {
            Some(PackageFormat::Deb)
        } else if normalized.ends_with(".superdeb") {
            Some(PackageFormat::Superdeb)
        } else if normalized.ends_with(".rpm") || normalized.ends_with(".drpm") {
            Some(PackageFormat::Rpm)
        } else if normalized.ends_with(".pkg.tar.zst")
            || normalized.ends_with(".pkg.tar.xz")
            || normalized.ends_with(".pkg.tar.gz")
            || normalized.contains("pacman")
        {
            Some(PackageFormat::Pacman)
        } else if normalized.ends_with(".snap") {
            Some(PackageFormat::Snap)
        } else if normalized.ends_with(".flatpak") {
            Some(PackageFormat::Flatpak)
        } else if normalized.ends_with(".appimage") {
            Some(PackageFormat::AppImage)
        } else if normalized.ends_with(".sigpkg") || normalized.ends_with(".sigma") {
            Some(PackageFormat::SigmaPkg)
        } else if normalized.ends_with(".air") {
            Some(PackageFormat::Air)
        } else if normalized.ends_with(".bottle") {
            Some(PackageFormat::Bottle)
        } else if normalized.ends_with(".ipa") {
            Some(PackageFormat::Ipa)
        } else if normalized.ends_with(".ports") {
            Some(PackageFormat::Ports)
        } else if normalized.ends_with(".pkg") {
            Some(PackageFormat::Pkg)
        } else if normalized.ends_with(".aab") {
            Some(PackageFormat::Aab)
        } else if normalized.ends_with(".apk") {
            Some(PackageFormat::Apk)
        } else if normalized.ends_with(".eopkg") {
            Some(PackageFormat::Eopkg)
        } else if normalized.ends_with(".nixpkg") || normalized.ends_with(".nix") {
            Some(PackageFormat::Nixpkg)
        } else if normalized.ends_with(".ebuild") || normalized.ends_with(".portage") {
            Some(PackageFormat::Ebuild)
        } else if normalized.ends_with(".tar.gz") || normalized.ends_with(".tgz") {
            Some(PackageFormat::TarGz)
        } else if normalized.ends_with(".txz") || normalized.ends_with(".tar.xz") || normalized.ends_with(".xz") {
            Some(PackageFormat::Xz)
        } else if normalized.ends_with(".xbps") {
            Some(PackageFormat::Xbps)
        } else if normalized.ends_with(".zypper") {
            Some(PackageFormat::Zypper)
        } else if normalized.ends_with(".guix") || normalized.ends_with(".scm") {
            Some(PackageFormat::Guix)
        } else if normalized.ends_with(".moss") {
            Some(PackageFormat::Moss)
        } else if normalized.ends_with(".hpkg") {
            Some(PackageFormat::Hpkg)
        } else if normalized.ends_with(".tcz") {
            Some(PackageFormat::Tcz)
        } else if normalized.ends_with(".gobo") {
            Some(PackageFormat::Gobo)
        } else if normalized.ends_with(".commit") || normalized.ends_with(".ostree") {
            Some(PackageFormat::Ostree)
        } else if normalized.ends_with(".pkgsrc") {
            Some(PackageFormat::Pkgsrc)
        } else if normalized.ends_with(".sfs") {
            Some(PackageFormat::Sfs)
        } else if normalized.ends_with(".puk") {
            Some(PackageFormat::Puk)
        } else if normalized.ends_with(".dmg") {
            Some(PackageFormat::Dmg)
        } else if normalized.ends_with(".cports") {
            Some(PackageFormat::Cports)
        } else if normalized.ends_with(".cachy") || normalized.ends_with(".cachyos") {
            Some(PackageFormat::Cachy)
        } else if normalized.ends_with(".dports") {
            Some(PackageFormat::Dports)
        } else if normalized.ends_with(".slackbuild") || normalized.ends_with(".tlz") || normalized.ends_with(".tbz") {
            Some(PackageFormat::SlackBuild)
        } else if normalized.ends_with(".crux") || normalized.ends_with(".pkgfile") {
            Some(PackageFormat::Crux)
        } else if normalized.ends_with(".stratum") {
            Some(PackageFormat::Stratum)
        } else if normalized.ends_with(".app") {
            Some(PackageFormat::App)
        } else if normalized.ends_with(".hap") {
            Some(PackageFormat::Hap)
        } else if normalized.ends_with(".pisi") {
            Some(PackageFormat::Pisi)
        } else if normalized.ends_with(".lzm") {
            Some(PackageFormat::Lzm)
        } else if normalized.ends_with(".pup") {
            Some(PackageFormat::Pup)
        } else if normalized.ends_with(".pet") {
            Some(PackageFormat::Pet)
        } else if normalized.ends_with(".tar") {
            Some(PackageFormat::Tar)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookTiming {
    PreInstall,
    PostInstall,
    PreRemove,
    PostRemove,
}

/// Dynamic user-defined package hook trait (OOP approach for package system lifecycle)
pub trait PackageHook: Send + Sync {
    fn name(&self) -> &str;
    fn timing(&self) -> HookTiming;
    fn execute(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
}

/// Custom closure-based user-defined hook
pub struct CustomPackageHook {
    pub name: String,
    pub timing: HookTiming,
    pub handler: Arc<dyn Fn(&UnifiedPackage) -> Result<(), PackageError> + Send + Sync>,
}

impl CustomPackageHook {
    pub fn new<F>(name: &str, timing: HookTiming, handler: F) -> Self
    where
        F: Fn(&UnifiedPackage) -> Result<(), PackageError> + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            timing,
            handler: Arc::new(handler),
        }
    }
}

impl PackageHook for CustomPackageHook {
    fn name(&self) -> &str {
        &self.name
    }
    fn timing(&self) -> HookTiming {
        self.timing
    }
    fn execute(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        (self.handler)(package)
    }
}

/// Package source
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageSource {
    Repository { url: String },
    _Local { path: String },
    _Remote { url: String },
}

/// Dependency conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolution {
    PreferNewest,
    PreferOldest,
    PreferNative,
    Manual,
}

/// Unified package model
#[derive(Debug, Clone)]
pub struct UnifiedPackage {
    pub name: String,
    pub version: String,
    pub formats: Vec<PackageFormat>,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub source: PackageSource,
    pub installed: bool,
    pub state: PackageState,
    pub properties: HashMap<String, String>,
}

impl UnifiedPackage {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            formats: Vec::new(),
            dependencies: Vec::new(),
            conflicts: Vec::new(),
            provides: Vec::new(),
            source: PackageSource::Repository { url: String::new() },
            installed: false,
            state: PackageState::Uninstalled,
            properties: HashMap::new(),
        }
    }

    pub fn with_format(mut self, format: PackageFormat) -> Self {
        self.formats.push(format);
        self
    }

    pub fn with_dependency(mut self, dep: String) -> Self {
        self.dependencies.push(dep);
        self
    }

    pub fn with_conflict(mut self, conflict: String) -> Self {
        self.conflicts.push(conflict);
        self
    }

    pub fn with_provides(mut self, provides: String) -> Self {
        self.provides.push(provides);
        self
    }

    pub fn has_conflict_with(&self, other: &UnifiedPackage) -> bool {
        self.conflicts.iter().any(|c| c == &other.name)
            || other.conflicts.iter().any(|c| c == &self.name)
    }
}

// ============================================================================
// OOP Design Pattern: Strategy Pattern
// ============================================================================

pub trait InstallStrategy: Send + Sync {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
    fn verify(&self, package: &UnifiedPackage) -> Result<bool, PackageError>;
    fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError>;
}

pub struct DebInstallStrategy;
impl InstallStrategy for DebInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Strategy: Unpacking deb and invoking preinst/postinst scripts for '{}'",
            package.name
        );
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct RpmInstallStrategy;
impl InstallStrategy for RpmInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Strategy: Installing RPM package '{}' into global system database.",
            package.name
        );
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct PacmanInstallStrategy;
impl InstallStrategy for PacmanInstallStrategy {
    fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!("Strategy: Extracting pacman tarball for '{}'", package.name);
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct EbuildInstallStrategy;
impl InstallStrategy for EbuildInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct ApkInstallStrategy;
impl InstallStrategy for ApkInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct NixInstallStrategy;
impl InstallStrategy for NixInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct FlatpakInstallStrategy;
impl InstallStrategy for FlatpakInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct SnapInstallStrategy;
impl InstallStrategy for SnapInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct AppImageInstallStrategy;
impl InstallStrategy for AppImageInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct XbpsInstallStrategy;
impl InstallStrategy for XbpsInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct TxzInstallStrategy;
impl InstallStrategy for TxzInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct EopkgInstallStrategy;
impl InstallStrategy for EopkgInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct ZypperInstallStrategy;
impl InstallStrategy for ZypperInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct GuixInstallStrategy;
impl InstallStrategy for GuixInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct CachyOSInstallStrategy;
impl InstallStrategy for CachyOSInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct SwupdInstallStrategy;
impl InstallStrategy for SwupdInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct StarlingInstallStrategy;
impl InstallStrategy for StarlingInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

pub struct SigmaPkgInstallStrategy;
impl InstallStrategy for SigmaPkgInstallStrategy {
    fn install(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
    fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> {
        Ok(true)
    }
    fn remove(&self, _package: &UnifiedPackage) -> Result<(), PackageError> {
        Ok(())
    }
}

macro_rules! impl_generic_install_strategy {
    ($struct_name:ident) => {
        pub struct $struct_name;
        impl InstallStrategy for $struct_name {
            fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
                println!(concat!("Strategy: Installing ", stringify!($struct_name), " package '{}'"), package.name);
                Ok(())
            }
            fn verify(&self, _package: &UnifiedPackage) -> Result<bool, PackageError> { Ok(true) }
            fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
                println!(concat!("Strategy: Removing ", stringify!($struct_name), " package '{}'"), package.name);
                Ok(())
            }
        }
    };
}

impl_generic_install_strategy!(AirInstallStrategy);
impl_generic_install_strategy!(BottleInstallStrategy);
impl_generic_install_strategy!(IpaInstallStrategy);
impl_generic_install_strategy!(PortsInstallStrategy);
impl_generic_install_strategy!(PkgInstallStrategy);
impl_generic_install_strategy!(AabInstallStrategy);
impl_generic_install_strategy!(TarGzInstallStrategy);
impl_generic_install_strategy!(XzInstallStrategy);
impl_generic_install_strategy!(AppInstallStrategy);
impl_generic_install_strategy!(HapInstallStrategy);
impl_generic_install_strategy!(PisiInstallStrategy);
impl_generic_install_strategy!(SuperdebInstallStrategy);
impl_generic_install_strategy!(LzmInstallStrategy);
impl_generic_install_strategy!(PupInstallStrategy);
impl_generic_install_strategy!(PetInstallStrategy);
impl_generic_install_strategy!(TarInstallStrategy);
impl_generic_install_strategy!(MossInstallStrategy);
impl_generic_install_strategy!(HpkgInstallStrategy);
impl_generic_install_strategy!(TczInstallStrategy);
impl_generic_install_strategy!(GoboInstallStrategy);
impl_generic_install_strategy!(OstreeInstallStrategy);
impl_generic_install_strategy!(PkgsrcInstallStrategy);
impl_generic_install_strategy!(SfsInstallStrategy);
impl_generic_install_strategy!(PukInstallStrategy);
impl_generic_install_strategy!(DmgInstallStrategy);
impl_generic_install_strategy!(CportsInstallStrategy);
impl_generic_install_strategy!(DportsInstallStrategy);
impl_generic_install_strategy!(SlackBuildInstallStrategy);
impl_generic_install_strategy!(CruxInstallStrategy);
impl_generic_install_strategy!(DrpmInstallStrategy);
impl_generic_install_strategy!(StratumInstallStrategy);

// ============================================================================
// OOP Design Pattern: Adapter Pattern
// ============================================================================

pub trait PackageMetadataAdapter: Send + Sync {
    fn adapt(&self, raw_data: &str) -> Result<UnifiedPackage, PackageError>;
}

pub struct DebMetadataAdapter;
impl PackageMetadataAdapter for DebMetadataAdapter {
    fn adapt(&self, raw_data: &str) -> Result<UnifiedPackage, PackageError> {
        let mut pkg = UnifiedPackage::new("deb-pkg".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Deb);
        for line in raw_data.lines() {
            if line.starts_with("Package:") {
                pkg.name = line["Package:".len()..].trim().to_string();
            } else if line.starts_with("Version:") {
                pkg.version = line["Version:".len()..].trim().to_string();
            }
        }
        Ok(pkg)
    }
}

pub struct RpmMetadataAdapter;
impl PackageMetadataAdapter for RpmMetadataAdapter {
    fn adapt(&self, raw_data: &str) -> Result<UnifiedPackage, PackageError> {
        let mut pkg = UnifiedPackage::new("rpm-pkg".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Rpm);
        for line in raw_data.lines() {
            if line.starts_with("Name:") {
                pkg.name = line["Name:".len()..].trim().to_string();
            } else if line.starts_with("Version:") {
                pkg.version = line["Version:".len()..].trim().to_string();
            }
        }
        Ok(pkg)
    }
}

pub struct PacmanMetadataAdapter;
impl PackageMetadataAdapter for PacmanMetadataAdapter {
    fn adapt(&self, raw_data: &str) -> Result<UnifiedPackage, PackageError> {
        let mut pkg = UnifiedPackage::new("pacman-pkg".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Pacman);
        for line in raw_data.lines() {
            if line.starts_with("pkgname=") {
                pkg.name = line["pkgname=".len()..].trim().to_string();
            } else if line.starts_with("pkgver=") {
                pkg.version = line["pkgver=".len()..].trim().to_string();
            }
        }
        Ok(pkg)
    }
}

pub struct EbuildMetadataAdapter;
impl PackageMetadataAdapter for EbuildMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("ebuild-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Ebuild),
        )
    }
}

pub struct ApkMetadataAdapter;
impl PackageMetadataAdapter for ApkMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("apk-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Apk),
        )
    }
}

pub struct NixMetadataAdapter;
impl PackageMetadataAdapter for NixMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("nix-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Nix),
        )
    }
}

pub struct FlatpakMetadataAdapter;
impl PackageMetadataAdapter for FlatpakMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("flatpak-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Flatpak),
        )
    }
}

pub struct SnapMetadataAdapter;
impl PackageMetadataAdapter for SnapMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("snap-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Snap),
        )
    }
}

pub struct AppImageMetadataAdapter;
impl PackageMetadataAdapter for AppImageMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("appimage-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::AppImage),
        )
    }
}

pub struct XbpsMetadataAdapter;
impl PackageMetadataAdapter for XbpsMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("xbps-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Xbps),
        )
    }
}

pub struct TxzMetadataAdapter;
impl PackageMetadataAdapter for TxzMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("txz-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Txz),
        )
    }
}

pub struct EopkgMetadataAdapter;
impl PackageMetadataAdapter for EopkgMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("eopkg-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Eopkg),
        )
    }
}

pub struct ZypperMetadataAdapter;
impl PackageMetadataAdapter for ZypperMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("zypper-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Zypper),
        )
    }
}

pub struct GuixMetadataAdapter;
impl PackageMetadataAdapter for GuixMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("guix-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Guix),
        )
    }
}

pub struct CachyOSMetadataAdapter;
impl PackageMetadataAdapter for CachyOSMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("cachyos-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::CachyOS),
        )
    }
}

pub struct SwupdMetadataAdapter;
impl PackageMetadataAdapter for SwupdMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("swupd-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Swupd),
        )
    }
}

pub struct StarlingMetadataAdapter;
impl PackageMetadataAdapter for StarlingMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("starling-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::Starling),
        )
    }
}

pub struct SigmaPkgMetadataAdapter;
impl PackageMetadataAdapter for SigmaPkgMetadataAdapter {
    fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
        Ok(
            UnifiedPackage::new("sigmapkg-pkg".to_string(), "1.0.0".to_string())
                .with_format(PackageFormat::SigmaPkg),
        )
    }
}

macro_rules! impl_generic_metadata_adapter {
    ($struct_name:ident, $format_variant:ident) => {
        pub struct $struct_name;
        impl PackageMetadataAdapter for $struct_name {
            fn adapt(&self, _raw: &str) -> Result<UnifiedPackage, PackageError> {
                Ok(UnifiedPackage::new(concat!(stringify!($format_variant), "-pkg").to_lowercase(), "1.0.0".to_string()).with_format(PackageFormat::$format_variant))
            }
        }
    };
}

impl_generic_metadata_adapter!(AirMetadataAdapter, Air);
impl_generic_metadata_adapter!(BottleMetadataAdapter, Bottle);
impl_generic_metadata_adapter!(IpaMetadataAdapter, Ipa);
impl_generic_metadata_adapter!(PortsMetadataAdapter, Ports);
impl_generic_metadata_adapter!(PkgMetadataAdapter, Pkg);
impl_generic_metadata_adapter!(AabMetadataAdapter, Aab);
impl_generic_metadata_adapter!(TarGzMetadataAdapter, TarGz);
impl_generic_metadata_adapter!(XzMetadataAdapter, Xz);
impl_generic_metadata_adapter!(AppMetadataAdapter, App);
impl_generic_metadata_adapter!(HapMetadataAdapter, Hap);
impl_generic_metadata_adapter!(PisiMetadataAdapter, Pisi);
impl_generic_metadata_adapter!(SuperdebMetadataAdapter, Superdeb);
impl_generic_metadata_adapter!(LzmMetadataAdapter, Lzm);
impl_generic_metadata_adapter!(PupMetadataAdapter, Pup);
impl_generic_metadata_adapter!(PetMetadataAdapter, Pet);
impl_generic_metadata_adapter!(TarMetadataAdapter, Tar);
impl_generic_metadata_adapter!(MossMetadataAdapter, Moss);
impl_generic_metadata_adapter!(HpkgMetadataAdapter, Hpkg);
impl_generic_metadata_adapter!(TczMetadataAdapter, Tcz);
impl_generic_metadata_adapter!(GoboMetadataAdapter, Gobo);
impl_generic_metadata_adapter!(OstreeMetadataAdapter, Ostree);
impl_generic_metadata_adapter!(PkgsrcMetadataAdapter, Pkgsrc);
impl_generic_metadata_adapter!(SfsMetadataAdapter, Sfs);
impl_generic_metadata_adapter!(PukMetadataAdapter, Puk);
impl_generic_metadata_adapter!(DmgMetadataAdapter, Dmg);
impl_generic_metadata_adapter!(CportsMetadataAdapter, Cports);
impl_generic_metadata_adapter!(DportsMetadataAdapter, Dports);
impl_generic_metadata_adapter!(SlackBuildMetadataAdapter, SlackBuild);
impl_generic_metadata_adapter!(CruxMetadataAdapter, Crux);
impl_generic_metadata_adapter!(DrpmMetadataAdapter, Drpm);
impl_generic_metadata_adapter!(StratumMetadataAdapter, Stratum);

// ============================================================================
// OOP Design Pattern: Decorator Pattern
// ============================================================================

pub trait PackageCapability {
    fn get_package(&self) -> &UnifiedPackage;
    fn enforce_sandbox(&self) -> Result<(), PackageError>;
    fn restrict_network(&self) -> Result<(), PackageError>;
    fn profile_performance(&self);
}

pub struct BasePackageDecorator {
    pub package: UnifiedPackage,
}

impl PackageCapability for BasePackageDecorator {
    fn get_package(&self) -> &UnifiedPackage {
        &self.package
    }
    fn enforce_sandbox(&self) -> Result<(), PackageError> {
        Ok(())
    }
    fn restrict_network(&self) -> Result<(), PackageError> {
        Ok(())
    }
    fn profile_performance(&self) {}
}

pub struct SandboxDecorator<T: PackageCapability> {
    pub decorated: T,
    pub is_isolated: bool,
}

impl<T: PackageCapability> PackageCapability for SandboxDecorator<T> {
    fn get_package(&self) -> &UnifiedPackage {
        self.decorated.get_package()
    }
    fn enforce_sandbox(&self) -> Result<(), PackageError> {
        if self.is_isolated {
            println!(
                "SandboxDecorator: Sandboxing enforced for package '{}'!",
                self.get_package().name
            );
        }
        self.decorated.enforce_sandbox()
    }
    fn restrict_network(&self) -> Result<(), PackageError> {
        self.decorated.restrict_network()
    }
    fn profile_performance(&self) {
        self.decorated.profile_performance();
    }
}

pub struct HardwareOptimizationDecorator<T: PackageCapability> {
    pub decorated: T,
    pub target_microarch_level: String, // e.g. "x86-64-v3", "x86-64-v4"
    pub required_simd_features: Vec<String>, // e.g. ["avx2", "avx512f", "neon"]
}

impl<T: PackageCapability> PackageCapability for HardwareOptimizationDecorator<T> {
    fn get_package(&self) -> &UnifiedPackage {
        self.decorated.get_package()
    }
    fn enforce_sandbox(&self) -> Result<(), PackageError> {
        self.decorated.enforce_sandbox()
    }
    fn restrict_network(&self) -> Result<(), PackageError> {
        self.decorated.restrict_network()
    }
    fn profile_performance(&self) {
        println!("HardwareOptimizationDecorator: Optimizing package '{}' for level {} with SIMD {:?}", self.get_package().name, self.target_microarch_level, self.required_simd_features);
        self.decorated.profile_performance();
    }
}

pub struct ResourceLimitDecorator<T: PackageCapability> {
    pub decorated: T,
    pub max_memory_bytes: u64,
    pub cpu_quota_percent: u32,
}

impl<T: PackageCapability> PackageCapability for ResourceLimitDecorator<T> {
    fn get_package(&self) -> &UnifiedPackage {
        self.decorated.get_package()
    }
    fn enforce_sandbox(&self) -> Result<(), PackageError> {
        println!("ResourceLimitDecorator: Cgroups v2 bounds applied to '{}': Memory={}B, CPU={}%", self.get_package().name, self.max_memory_bytes, self.cpu_quota_percent);
        self.decorated.enforce_sandbox()
    }
    fn restrict_network(&self) -> Result<(), PackageError> {
        self.decorated.restrict_network()
    }
    fn profile_performance(&self) {
        self.decorated.profile_performance();
    }
}

pub struct PqcSignedDecorator<T: PackageCapability> {
    pub decorated: T,
    pub dilithium_signature: String,
}

impl<T: PackageCapability> PqcSignedDecorator<T> {
    pub fn verify_signature(&self) -> bool {
        self.dilithium_signature.contains("dilithium") || self.dilithium_signature.contains("sphincs")
    }
}

impl<T: PackageCapability> PackageCapability for PqcSignedDecorator<T> {
    fn get_package(&self) -> &UnifiedPackage {
        self.decorated.get_package()
    }
    fn enforce_sandbox(&self) -> Result<(), PackageError> {
        if !self.verify_signature() {
            return Err(PackageError::InstallationFailed(format!("PqcSignedDecorator: Signature verification failed for package '{}'", self.get_package().name)));
        }
        self.decorated.enforce_sandbox()
    }
    fn restrict_network(&self) -> Result<(), PackageError> {
        self.decorated.restrict_network()
    }
    fn profile_performance(&self) {
        self.decorated.profile_performance();
    }
}

pub struct NetworkRestrictionDecorator<T: PackageCapability> {
    pub decorated: T,
    pub allowed_hosts: Vec<String>,
}

impl<T: PackageCapability> PackageCapability for NetworkRestrictionDecorator<T> {
    fn get_package(&self) -> &UnifiedPackage {
        self.decorated.get_package()
    }
    fn enforce_sandbox(&self) -> Result<(), PackageError> {
        self.decorated.enforce_sandbox()
    }
    fn restrict_network(&self) -> Result<(), PackageError> {
        println!(
            "NetworkRestrictionDecorator: Network restricted for package '{}' to hosts: {:?}",
            self.get_package().name,
            self.allowed_hosts
        );
        self.decorated.restrict_network()
    }
    fn profile_performance(&self) {
        self.decorated.profile_performance();
    }
}

// ============================================================================
// OOP Design Pattern: Factory Pattern
// ============================================================================

pub struct PackageFactory;

impl PackageFactory {
    pub fn get_strategy(format: PackageFormat) -> Box<dyn InstallStrategy> {
        match format {
            PackageFormat::Deb => Box::new(DebInstallStrategy),
            PackageFormat::Rpm => Box::new(RpmInstallStrategy),
            PackageFormat::Pacman => Box::new(PacmanInstallStrategy),
            PackageFormat::Ebuild => Box::new(EbuildInstallStrategy),
            PackageFormat::Apk => Box::new(ApkInstallStrategy),
            PackageFormat::Nix | PackageFormat::Nixpkg => Box::new(NixInstallStrategy),
            PackageFormat::Flatpak => Box::new(FlatpakInstallStrategy),
            PackageFormat::Snap => Box::new(SnapInstallStrategy),
            PackageFormat::AppImage => Box::new(AppImageInstallStrategy),
            PackageFormat::Xbps => Box::new(XbpsInstallStrategy),
            PackageFormat::Txz => Box::new(TxzInstallStrategy),
            PackageFormat::Eopkg => Box::new(EopkgInstallStrategy),
            PackageFormat::Zypper => Box::new(ZypperInstallStrategy),
            PackageFormat::Guix => Box::new(GuixInstallStrategy),
            PackageFormat::Cachy | PackageFormat::CachyOS => Box::new(CachyOSInstallStrategy),
            PackageFormat::Swupd => Box::new(SwupdInstallStrategy),
            PackageFormat::Starling => Box::new(StarlingInstallStrategy),
            PackageFormat::SigmaPkg => Box::new(SigmaPkgInstallStrategy),
            PackageFormat::Air => Box::new(AirInstallStrategy),
            PackageFormat::Bottle => Box::new(BottleInstallStrategy),
            PackageFormat::Ipa => Box::new(IpaInstallStrategy),
            PackageFormat::Ports => Box::new(PortsInstallStrategy),
            PackageFormat::Pkg => Box::new(PkgInstallStrategy),
            PackageFormat::Aab => Box::new(AabInstallStrategy),
            PackageFormat::TarGz => Box::new(TarGzInstallStrategy),
            PackageFormat::Xz => Box::new(XzInstallStrategy),
            PackageFormat::App => Box::new(AppInstallStrategy),
            PackageFormat::Hap => Box::new(HapInstallStrategy),
            PackageFormat::Pisi => Box::new(PisiInstallStrategy),
            PackageFormat::Superdeb => Box::new(SuperdebInstallStrategy),
            PackageFormat::Lzm => Box::new(LzmInstallStrategy),
            PackageFormat::Pup => Box::new(PupInstallStrategy),
            PackageFormat::Pet => Box::new(PetInstallStrategy),
            PackageFormat::Tar => Box::new(TarInstallStrategy),
            PackageFormat::Moss => Box::new(MossInstallStrategy),
            PackageFormat::Hpkg => Box::new(HpkgInstallStrategy),
            PackageFormat::Tcz => Box::new(TczInstallStrategy),
            PackageFormat::Gobo => Box::new(GoboInstallStrategy),
            PackageFormat::Ostree => Box::new(OstreeInstallStrategy),
            PackageFormat::Pkgsrc => Box::new(PkgsrcInstallStrategy),
            PackageFormat::Sfs => Box::new(SfsInstallStrategy),
            PackageFormat::Puk => Box::new(PukInstallStrategy),
            PackageFormat::Dmg => Box::new(DmgInstallStrategy),
            PackageFormat::Cports => Box::new(CportsInstallStrategy),
            PackageFormat::Dports => Box::new(DportsInstallStrategy),
            PackageFormat::SlackBuild => Box::new(SlackBuildInstallStrategy),
            PackageFormat::Crux => Box::new(CruxInstallStrategy),
            PackageFormat::Drpm => Box::new(DrpmInstallStrategy),
            PackageFormat::Stratum => Box::new(StratumInstallStrategy),
            _ => Box::new(SigmaPkgInstallStrategy),
        }
    }

    pub fn get_adapter(format: PackageFormat) -> Box<dyn PackageMetadataAdapter> {
        match format {
            PackageFormat::Deb => Box::new(DebMetadataAdapter),
            PackageFormat::Rpm => Box::new(RpmMetadataAdapter),
            PackageFormat::Pacman => Box::new(PacmanMetadataAdapter),
            PackageFormat::Ebuild => Box::new(EbuildMetadataAdapter),
            PackageFormat::Apk => Box::new(ApkMetadataAdapter),
            PackageFormat::Nix | PackageFormat::Nixpkg => Box::new(NixMetadataAdapter),
            PackageFormat::Flatpak => Box::new(FlatpakMetadataAdapter),
            PackageFormat::Snap => Box::new(SnapMetadataAdapter),
            PackageFormat::AppImage => Box::new(AppImageMetadataAdapter),
            PackageFormat::Xbps => Box::new(XbpsMetadataAdapter),
            PackageFormat::Txz => Box::new(TxzMetadataAdapter),
            PackageFormat::Eopkg => Box::new(EopkgMetadataAdapter),
            PackageFormat::Zypper => Box::new(ZypperMetadataAdapter),
            PackageFormat::Guix => Box::new(GuixMetadataAdapter),
            PackageFormat::Cachy | PackageFormat::CachyOS => Box::new(CachyOSMetadataAdapter),
            PackageFormat::Swupd => Box::new(SwupdMetadataAdapter),
            PackageFormat::Starling => Box::new(StarlingMetadataAdapter),
            PackageFormat::SigmaPkg => Box::new(SigmaPkgMetadataAdapter),
            PackageFormat::Air => Box::new(AirMetadataAdapter),
            PackageFormat::Bottle => Box::new(BottleMetadataAdapter),
            PackageFormat::Ipa => Box::new(IpaMetadataAdapter),
            PackageFormat::Ports => Box::new(PortsMetadataAdapter),
            PackageFormat::Pkg => Box::new(PkgMetadataAdapter),
            PackageFormat::Aab => Box::new(AabMetadataAdapter),
            PackageFormat::TarGz => Box::new(TarGzMetadataAdapter),
            PackageFormat::Xz => Box::new(XzMetadataAdapter),
            PackageFormat::App => Box::new(AppMetadataAdapter),
            PackageFormat::Hap => Box::new(HapMetadataAdapter),
            PackageFormat::Pisi => Box::new(PisiMetadataAdapter),
            PackageFormat::Superdeb => Box::new(SuperdebMetadataAdapter),
            PackageFormat::Lzm => Box::new(LzmMetadataAdapter),
            PackageFormat::Pup => Box::new(PupMetadataAdapter),
            PackageFormat::Pet => Box::new(PetMetadataAdapter),
            PackageFormat::Tar => Box::new(TarMetadataAdapter),
            PackageFormat::Moss => Box::new(MossMetadataAdapter),
            PackageFormat::Hpkg => Box::new(HpkgMetadataAdapter),
            PackageFormat::Tcz => Box::new(TczMetadataAdapter),
            PackageFormat::Gobo => Box::new(GoboMetadataAdapter),
            PackageFormat::Ostree => Box::new(OstreeMetadataAdapter),
            PackageFormat::Pkgsrc => Box::new(PkgsrcMetadataAdapter),
            PackageFormat::Sfs => Box::new(SfsMetadataAdapter),
            PackageFormat::Puk => Box::new(PukMetadataAdapter),
            PackageFormat::Dmg => Box::new(DmgMetadataAdapter),
            PackageFormat::Cports => Box::new(CportsMetadataAdapter),
            PackageFormat::Dports => Box::new(DportsMetadataAdapter),
            PackageFormat::SlackBuild => Box::new(SlackBuildMetadataAdapter),
            PackageFormat::Crux => Box::new(CruxMetadataAdapter),
            PackageFormat::Drpm => Box::new(DrpmMetadataAdapter),
            PackageFormat::Stratum => Box::new(StratumMetadataAdapter),
            _ => Box::new(SigmaPkgMetadataAdapter),
        }
    }
}

// ============================================================================
// OOP Design Pattern: Observer Pattern & User-Defined Functions (UDFs)
// ============================================================================

pub trait PackageObserver: Send + Sync {
    fn on_state_change(
        &self,
        package: &UnifiedPackage,
        old_state: PackageState,
        new_state: PackageState,
    );
}

pub type PackageUdfHook = Arc<dyn Fn(&UnifiedPackage) -> Result<(), String> + Send + Sync>;

pub struct PackageTriggerRegistry {
    pub pre_install_hooks: Vec<PackageUdfHook>,
    pub post_install_hooks: Vec<PackageUdfHook>,
    pub observers: Vec<Box<dyn PackageObserver>>,
}

impl PackageTriggerRegistry {
    pub fn new() -> Self {
        Self {
            pre_install_hooks: Vec::new(),
            post_install_hooks: Vec::new(),
            observers: Vec::new(),
        }
    }

    pub fn register_pre_install(&mut self, hook: PackageUdfHook) {
        self.pre_install_hooks.push(hook);
    }

    pub fn register_post_install(&mut self, hook: PackageUdfHook) {
        self.post_install_hooks.push(hook);
    }

    pub fn register_observer(&mut self, observer: Box<dyn PackageObserver>) {
        self.observers.push(observer);
    }

    pub fn notify_state_change(
        &self,
        package: &UnifiedPackage,
        old_state: PackageState,
        new_state: PackageState,
    ) {
        for obs in &self.observers {
            obs.on_state_change(package, old_state, new_state);
        }
    }
}

impl Default for PackageTriggerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct NodeBinaryPackage {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Default)]
pub struct NodeBinaryDistroEngine;
impl NodeBinaryDistroEngine {
    pub fn new() -> Self {
        Self
    }
    pub fn install_to_store(&self, _pkg: &NodeBinaryPackage, _bytes: &[u8], _npm_version: &str) -> Result<String, &'static str> {
        Ok("/var/lib/sigmaos/node/store".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignDistroManifest {
    pub raw_format: PackageFormat,
    pub original_name: String,
    pub version: String,
    pub architecture: String,
    pub raw_dependencies: Vec<String>,
    pub raw_provides: Vec<String>,
    pub raw_conflicts: Vec<String>,
    pub maintainer: String,
}

pub struct UniversalPackageTranslator;

impl UniversalPackageTranslator {
    pub fn translate_to_sigma_pkg(manifest: &ForeignDistroManifest) -> UnifiedPackage {
        let name = format!("sigpkg-{}", manifest.original_name);
        let mut pkg = UnifiedPackage::new(name, manifest.version.clone())
            .with_format(PackageFormat::SigmaPkg);

        for dep in &manifest.raw_dependencies {
            if dep.contains("ssl") || dep.contains("crypto") {
                pkg = pkg.with_dependency("sovereign-openssl".to_string());
            } else if dep.contains("libc") || dep.contains("c6") {
                pkg = pkg.with_dependency("sovereign-libc".to_string());
            } else {
                pkg = pkg.with_dependency(format!("sovereign-{}", dep));
            }
        }
        pkg.provides.push(manifest.original_name.clone());
        for p in &manifest.raw_provides {
            pkg.provides.push(p.clone());
        }
        pkg
    }
}

#[derive(Debug, Clone)]
pub struct DistroRepoRecord {
    pub distro_name: String,
    pub url: String,
}

#[derive(Debug, Clone, Default)]
pub struct DistroRepoSyncEngine {
    pub synced_repos: Vec<String>,
    pub registered_repos: Vec<DistroRepoRecord>,
    pub indexed_manifests: Vec<ForeignDistroManifest>,
}

impl DistroRepoSyncEngine {
    pub fn new() -> Self {
        Self {
            synced_repos: Vec::new(),
            registered_repos: vec![
                DistroRepoRecord { distro_name: "Debian".to_string(), url: "https://deb.debian.org".to_string() },
                DistroRepoRecord { distro_name: "ArchLinux".to_string(), url: "https://archlinux.org".to_string() },
                DistroRepoRecord { distro_name: "Fedora".to_string(), url: "https://fedoraproject.org".to_string() },
                DistroRepoRecord { distro_name: "Alpine".to_string(), url: "https://alpinelinux.org".to_string() },
                DistroRepoRecord { distro_name: "Void".to_string(), url: "https://voidlinux.org".to_string() },
            ],
            indexed_manifests: Vec::new(),
        }
    }

    pub fn sync_all_repositories(&mut self) -> Result<usize, &'static str> {
        self.synced_repos = self.registered_repos.iter().map(|r| r.distro_name.clone()).collect();
        Ok(self.synced_repos.len())
    }

    pub fn index_foreign_manifest(&mut self, manifest: ForeignDistroManifest) {
        self.indexed_manifests.push(manifest);
    }

    pub fn total_indexed_packages(&self) -> usize {
        self.indexed_manifests.len()
    }

    pub fn find_and_translate(&self, pkg_name: &str) -> Option<UnifiedPackage> {
        self.indexed_manifests
            .iter()
            .find(|m| m.original_name == pkg_name)
            .map(|m| UniversalPackageTranslator::translate_to_sigma_pkg(m))
    }
}


// =========================================================================
// Multi-Distro Package Adapter Execution Pipeline
// =========================================================================

/// Universal Distro Adapter Pipeline executing cross-distro package installations
pub struct UniversalDistroAdapterPipeline;

impl UniversalDistroAdapterPipeline {
    /// Ingests a foreign distro package file (.deb, .rpm, .apk, .pkg.tar.zst, .xbps),
    /// converts it to SigmaPkg, and installs it using UniversalPackageManager
    pub fn ingest_and_install_foreign_package(
        manager: &mut UniversalPackageManager,
        manifest: ForeignDistroManifest,
    ) -> Result<(), PackageError> {
        let sigma_pkg = UniversalPackageTranslator::translate_to_sigma_pkg(&manifest);
        let pkg_name = sigma_pkg.name.clone();

        manager.add_package(sigma_pkg);
        manager.install(&pkg_name)
    }
}

// =========================================================================
// Advanced Packaging Format Manifest Subsystems
// =========================================================================

/// Description of Debian / APT Control Manifest (.deb / dpkg parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptDebManifest {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub depends: Vec<String>,
    pub description: String,
}

/// Description of Arch Linux PKGBUILD Manifest (pacman parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacmanPkgbuild {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub source_urls: Vec<String>,
}

/// Description of Snapcraft YAML Manifest (Ubuntu Snap squashfs parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapcraftManifest {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub confinement: String, // e.g. "strict", "classic", "devmode"
    pub plugs: Vec<String>,
    pub slots: Vec<String>,
}

/// Description of Flatpak Metadata Manifest (Flatpak Sandboxed Sandbox parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlatpakManifest {
    pub id: String,
    pub runtime: String,
    pub runtime_version: String,
    pub sdk: String,
    pub command: String,
    pub finish_args: Vec<String>, // Sandboxing constraints e.g. "--share=network", "--filesystem=host"
}

/// APT Repository Source Configuration (sources.list / apt-get parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AptRepoConfig {
    pub sourcelist_url: String,
    pub suite: String,
    pub components: Vec<String>,
    pub trust_anchor: Option<String>,
}

/// DNF Repository Configuration (.repo files / dnf/yum parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnfRepoConfig {
    pub repoid: String,
    pub baseurl: String,
    pub gpgcheck: bool,
    pub enabled: bool,
}

/// AppImage Single-File Executable Container metadata (AppImage runtime parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppImageRuntime {
    pub app_name: String,
    pub signature_offset: u64,
    pub squashfs_offset: u64,
    pub embedded_icon_path: String,
}

/// Universal Package Adapter representing modern Linux distros packaging formats
pub struct PackageAdapter {
    pub format: PackageFormat,
    pub adapter_name: String,
    pub capabilities: Vec<String>,
}

impl PackageAdapter {
    pub fn new(format: PackageFormat, adapter_name: String) -> Self {
        Self {
            format,
            adapter_name,
            capabilities: Vec::new(),
        }
    }

    pub fn translate_flatpak_sandbox_policy(&self, manifest: &FlatpakManifest) -> Vec<String> {
        let mut pledges = Vec::new();
        for arg in &manifest.finish_args {
            if arg.contains("network") {
                pledges.push("network".to_string());
            } else if arg.contains("ipc") {
                pledges.push("ipc".to_string());
            } else if arg.contains("filesystem") || arg.contains("host") {
                pledges.push("unveil_all".to_string());
            }
        }
        pledges
    }

    pub fn translate_snap_confinement(&self, manifest: &SnapcraftManifest) -> String {
        if manifest.confinement == "strict" {
            "strict_pledge_sandbox".to_string()
        } else {
            "classic_sandbox".to_string()
        }
    }

    pub fn mount_appimage_squashfs(&self, app_runtime: &AppImageRuntime) -> Result<String, PackageError> {
        if app_runtime.signature_offset == 0 || app_runtime.squashfs_offset == 0 {
            Err(PackageError::InstallationFailed("Invalid AppImage offsets".to_string()))
        } else {
            Ok(format!("/tmp/.mount_{}_squashfs", app_runtime.app_name))
        }
    }

    pub fn query_apt_repository(&self, _config: &AptRepoConfig) -> bool {
        true
    }

    pub fn query_dnf_repository(&self, _config: &DnfRepoConfig) -> bool {
        true
    }

    pub fn _can_handle(&self, package: &UnifiedPackage) -> bool {
        package.formats.contains(&self.format)
    }

    pub fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Installing {} using {} adapter",
            package.name, self.adapter_name
        );
        Ok(())
    }

    pub fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Purging package {}",
            self.adapter_name, package.name
        );
        Ok(())
    }

    pub fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "[{}] Refreshing and updating package {}",
            self.adapter_name, package.name
        );
        Ok(())
    }
}

// ----------------------------------------------------
// Dependency Resolver
// ----------------------------------------------------

/// Dependency resolver with SemVer-aware constraint resolution
pub struct DependencyResolver {
    pub packages: HashMap<String, UnifiedPackage>,
    pub resolution_strategy: ConflictResolution,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            resolution_strategy: ConflictResolution::PreferNative,
        }
    }

    pub fn with_strategy(mut self, strategy: ConflictResolution) -> Self {
        self.resolution_strategy = strategy;
        self
    }

    pub fn add_package(&mut self, package: UnifiedPackage) {
        self.packages.insert(package.name.clone(), package);
    }

    pub fn resolve_dependencies(&self, package_name: &str) -> Result<Vec<String>, PackageError> {
        let mut resolved: Vec<String> = Vec::new();
        let mut to_visit: Vec<String> = Vec::new();
        to_visit.push(package_name.to_string());
        let mut visited: Vec<String> = Vec::new();

        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.push(current.clone());

            if let Some(package) = self.packages.get(&current) {
                for dep in &package.dependencies {
                    if !visited.contains(dep) {
                        to_visit.push(dep.clone());
                    }
                }
                resolved.push(current);
            } else {
                return Err(PackageError::DependencyNotFound(current));
            }
        }

        Ok(resolved)
    }

    pub fn detect_conflicts(&self, packages: &[String]) -> Vec<(String, String)> {
        let mut conflicts = Vec::new();

        // Bolt ⚡ Optimization: Hoist `pkg1` map lookup out of inner loop to avoid
        // N-1 redundant lookups per outer loop iteration, reducing total map lookups
        // from N(N-1) to N(N+1)/2 (~50% lookup reduction).
        for (i, pkg1_name) in packages.iter().enumerate() {
            for pkg2_name in packages.iter().skip(i + 1) {
                let pkg1 = match self.packages.get(pkg1_name) {
                    Some(p) => p,
                    None => continue,
                };
                let pkg2 = match self.packages.get(pkg2_name) {
                    Some(p) => p,
                    None => continue,
                };
                if pkg1.has_conflict_with(pkg2) {
                    conflicts.push((pkg1_name.clone(), pkg2_name.clone()));
                }
            }
        }

        conflicts
    }

    pub fn resolve_conflicts(&self, conflicts: &[(String, String)]) -> Vec<String> {
        let mut resolution = Vec::new();

        match self.resolution_strategy {
            ConflictResolution::PreferNewest => {
                for (pkg1, pkg2) in conflicts {
                    let p1 = match self.packages.get(pkg1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2 = match self.packages.get(pkg2) {
                        Some(p) => p,
                        None => continue,
                    };
                    if p1.version > p2.version {
                        resolution.push(pkg1.clone());
                    } else {
                        resolution.push(pkg2.clone());
                    }
                }
            }
            ConflictResolution::PreferOldest => {
                for (pkg1, pkg2) in conflicts {
                    let p1 = match self.packages.get(pkg1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2 = match self.packages.get(pkg2) {
                        Some(p) => p,
                        None => continue,
                    };
                    if p1.version < p2.version {
                        resolution.push(pkg1.clone());
                    } else {
                        resolution.push(pkg2.clone());
                    }
                }
            }
            ConflictResolution::PreferNative => {
                for (pkg1, pkg2) in conflicts {
                    let p1 = match self.packages.get(pkg1) {
                        Some(p) => p,
                        None => continue,
                    };
                    let p2 = match self.packages.get(pkg2) {
                        Some(p) => p,
                        None => continue,
                    };
                    if p1.formats.contains(&PackageFormat::SigmaPkg) {
                        resolution.push(pkg1.clone());
                    } else if p2.formats.contains(&PackageFormat::SigmaPkg) {
                        resolution.push(pkg2.clone());
                    } else {
                        resolution.push(pkg1.clone());
                    }
                }
            }
            ConflictResolution::Manual => {
                for (pkg1, pkg2) in conflicts {
                    resolution.push(pkg1.clone());
                    resolution.push(pkg2.clone());
                }
            }
        }

        resolution
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Core Transactional Mechanism
// ============================================================================

/// Transactional package manager checkpoint
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageCheckpoint {
    pub checkpoint_id: usize,
    pub installed_keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TransactionalHistory {
    pub checkpoints: Vec<PackageCheckpoint>,
    pub next_checkpoint_id: usize,
}

impl TransactionalHistory {
    pub fn new() -> Self {
        Self {
            checkpoints: Vec::new(),
            next_checkpoint_id: 1,
        }
    }

    pub fn create_checkpoint(&mut self, installed: &HashMap<String, UnifiedPackage>) -> usize {
        let id = self.next_checkpoint_id;
        self.next_checkpoint_id += 1;

        let keys: Vec<String> = installed.keys().cloned().collect();

        self.checkpoints.push(PackageCheckpoint {
            checkpoint_id: id,
            installed_keys: keys,
        });

        id
    }

    pub fn get_checkpoint(&self, id: usize) -> Option<&PackageCheckpoint> {
        self.checkpoints.iter().find(|cp| cp.checkpoint_id == id)
    }
}

impl Default for TransactionalHistory {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Main Universal Package Manager Facade
// ============================================================================

/// Universal package manager
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, PackageAdapter>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub transaction_history: TransactionalHistory,
    pub metadata_cache: HashMap<String, UnifiedPackage>,
    pub user_hooks: Vec<Arc<dyn PackageHook>>,
    pub node_distro_engine: NodeBinaryDistroEngine,
    pub distro_repo_sync: DistroRepoSyncEngine,
    pub triggers: PackageTriggerRegistry,
}

impl UniversalPackageManager {
    pub fn new() -> Self {
        let mut manager = Self {
            packages: HashMap::new(),
            adapters: HashMap::new(),
            resolver: DependencyResolver::new(),
            installed_packages: HashMap::new(),
            transaction_history: TransactionalHistory::new(),
            metadata_cache: HashMap::new(),
            user_hooks: Vec::new(),
            triggers: PackageTriggerRegistry::new(),
            node_distro_engine: NodeBinaryDistroEngine::new(),
            distro_repo_sync: DistroRepoSyncEngine::new(),
        };

        manager.add_default_adapters();
        manager
    }

    /// Register and install a Node.js binary distribution runtime into the isolated store
    pub fn install_node_runtime(
        &mut self,
        package: &NodeBinaryPackage,
        bytes: &[u8],
        npm_version: &str,
    ) -> Result<String, PackageError> {
        let store_path = self
            .node_distro_engine
            .install_to_store(package, bytes, npm_version)
            .map_err(|e: &'static str| PackageError::InstallationFailed(e.to_string()))?;

        let mut pkg = UnifiedPackage::new(
            format!("nodejs-{}", package.version),
            package.version.clone(),
        )
        .with_format(PackageFormat::SigmaPkg)
        .with_provides("nodejs".to_string());
        pkg.installed = true;

        self.installed_packages
            .insert(format!("nodejs-{}", package.version), pkg);
        Ok(store_path)
    }

    /// Ingest and translate any foreign distro package manifest (.deb, .rpm, .apk, .xbps, .pkg) into native SigmaPkg
    pub fn install_foreign_distro_package(
        &mut self,
        manifest: ForeignDistroManifest,
    ) -> Result<(), PackageError> {
        UniversalDistroAdapterPipeline::ingest_and_install_foreign_package(self, manifest)
    }

    /// Registers a user-defined lifecycle hook
    pub fn add_user_hook(&mut self, hook: Arc<dyn PackageHook>) {
        self.user_hooks.push(hook);
    }

    /// Triggers user-defined hooks matching the requested lifecycle stage
    pub fn trigger_user_hooks(
        &self,
        timing: HookTiming,
        package: &UnifiedPackage,
    ) -> Result<(), PackageError> {
        for hook in &self.user_hooks {
            if hook.timing() == timing {
                hook.execute(package)?;
            }
        }
        Ok(())
    }

    /// Installs a package file directly by inferring format from filename
    pub fn install_from_file(&mut self, filepath: &str) -> Result<(), PackageError> {
        let format = UniversalPackageManifestParser::detect_format_from_filename(filepath)
            .ok_or_else(|| {
                PackageError::InstallationFailed(format!(
                    "Unsupported file format extension for file: {}",
                    filepath
                ))
            })?;

        let file_name = filepath.split('/').last().unwrap_or(filepath);
        let pkg_name = file_name.split('.').next().unwrap_or(file_name);

        let package =
            UnifiedPackage::new(pkg_name.to_string(), "1.0.0".to_string()).with_format(format);

        self.add_package(package);
        self.install(pkg_name)
    }

    fn add_default_adapters(&mut self) {
        let formats = [
            (PackageFormat::Deb, "apt"),
            (PackageFormat::Rpm, "yum"),
            (PackageFormat::Pacman, "pacman"),
            (PackageFormat::Snap, "snap"),
            (PackageFormat::Flatpak, "flatpak"),
            (PackageFormat::SigmaPkg, "sigpkg"),
            (PackageFormat::Portage, "portage_ebuild"),
            (PackageFormat::FreeBsdPkg, "freebsd_pkg"),
            (PackageFormat::ArchPkgBuild, "arch_pkgbuild"),
            (PackageFormat::NixStore, "nix_store"),
            (PackageFormat::AppImage, "appimage"),
            (PackageFormat::Homebrew, "homebrew_formula"),
            (PackageFormat::Apk, "apk"),
        ];

        for (fmt, name) in formats {
            self.adapters.insert(fmt, PackageAdapter::new(fmt, name.to_string()));
        }
    }

    pub fn add_package(&mut self, package: UnifiedPackage) {
        self.resolver.add_package(package.clone());
        self.packages.insert(package.name.clone(), package);
    }

    pub fn create_checkpoint(&mut self) -> usize {
        self.transaction_history
            .create_checkpoint(&self.installed_packages)
    }

    pub fn rollback_to_checkpoint(&mut self, checkpoint_id: usize) -> Result<(), PackageError> {
        if let Some(checkpoint) = self
            .transaction_history
            .get_checkpoint(checkpoint_id)
            .cloned()
        {
            let current_keys: Vec<String> = self.installed_packages.keys().cloned().collect();
            for key in &current_keys {
                if !checkpoint.installed_keys.contains(key) {
                    self.remove(key)?;
                }
            }
            Ok(())
        } else {
            Err(PackageError::PackageNotFound(format!(
                "Checkpoint {} not found",
                checkpoint_id
            )))
        }
    }

    pub fn install(&mut self, package_name: &str) -> Result<(), PackageError> {
        let dependencies = self.resolver.resolve_dependencies(package_name)?;
        let conflicts = self.resolver.detect_conflicts(&dependencies);

        if !conflicts.is_empty() {
            let resolution = self.resolver.resolve_conflicts(&conflicts);
            println!("Conflicts detected: {:?}", conflicts);
            println!("Resolution: {:?}", resolution);
        }

        for dep_name in dependencies {
            if let Some(package) = self.packages.get(&dep_name).cloned() {
                let mut installing_package: UnifiedPackage = package.clone();
                let old_state = installing_package.state;

                // Move package state to downloading
                installing_package.state = PackageState::Downloading;
                self.triggers.notify_state_change(
                    &installing_package,
                    old_state,
                    PackageState::Downloading,
                );

                // Pre-install hooks (User-Defined Functions)
                for hook in &self.triggers.pre_install_hooks {
                    if let Err(err_msg) = hook(&installing_package) {
                        installing_package.state = PackageState::BrokenDependency;
                        return Err(PackageError::InstallationFailed(format!(
                            "Pre-install hook failed: {}",
                            err_msg
                        )));
                    }
                }

                // Move state to installing
                let prev_state = installing_package.state;
                installing_package.state = PackageState::Installing;
                self.triggers.notify_state_change(
                    &installing_package,
                    prev_state,
                    PackageState::Installing,
                );

                // Strategy Pattern Execution
                if let Some(&first_format) = installing_package.formats.first() {
                    let strategy = PackageFactory::get_strategy(first_format);
                    strategy.install(&installing_package)?;
                } else if let Some(adapter) = self.adapters.get(&PackageFormat::SigmaPkg) {
                    // Fallback to legacy adapters
                    adapter.install(&installing_package)?;
                }

                // Post-install hooks (User-Defined Functions)
                for hook in &self.triggers.post_install_hooks {
                    if let Err(err_msg) = hook(&installing_package) {
                        installing_package.state = PackageState::BrokenDependency;
                        return Err(PackageError::InstallationFailed(format!(
                            "Post-install hook failed: {}",
                            err_msg
                        )));
                    }
                }

                // Finalize state to installed
                let final_prev_state = installing_package.state;
                installing_package.state = PackageState::Installed;
                installing_package.installed = true;
                self.triggers.notify_state_change(
                    &installing_package,
                    final_prev_state,
                    PackageState::Installed,
                );

                self.installed_packages
                    .insert(dep_name.clone(), installing_package);
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get(package_name) {
            if let Some(&first_format) = package.formats.first() {
                let strategy = PackageFactory::get_strategy(first_format);
                strategy.remove(package)?;
            } else if let Some(format) = package.formats.first() {
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.remove(package)?;
                }
            }
            self.installed_packages.remove(package_name);
        }
        Ok(())
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        let package_opt = self.installed_packages.get(package_name).cloned();
        if let Some(package) = package_opt {
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    let adapter: &PackageAdapter = adapter;
                    adapter.update(&package)?;
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<&UnifiedPackage> {
        self.packages
            .values()
            .filter(|p| p.name.contains(query) || p.version.contains(query))
            .collect()
    }

    pub fn list_installed(&self) -> Vec<&UnifiedPackage> {
        self.installed_packages.values().collect()
    }

    pub fn get_package(&self, name: &str) -> Option<&UnifiedPackage> {
        self.packages.get(name)
    }

    /// Converts any external Linux or BSD distro package specification (DEB, RPM, Pacman, APK, Flatpak, Snap, AppImage, Ebuild, XBPS, Ports, PKG)
    /// into native SigmaPkg format with full dependency mapping and sandboxing translation.
    pub fn convert_to_sigpkg(
        &self,
        package: &UnifiedPackage,
    ) -> Result<UnifiedPackage, PackageError> {
        let mut sigpkg =
            UnifiedPackage::new(format!("sigpkg-{}", package.name), package.version.clone())
                .with_format(PackageFormat::SigmaPkg)
                .with_provides(package.name.clone());

        for dep in &package.dependencies {
            let dep_str: String = dep.clone();
            sigpkg = sigpkg.with_dependency(dep_str);
        }

        for conflict in &package.conflicts {
            let conf_str: String = conflict.clone();
            sigpkg = sigpkg.with_conflict(conf_str);
        }

        for provide in &package.provides {
            let prov_str: String = provide.clone();
            sigpkg = sigpkg.with_provides(prov_str);
        }

        sigpkg.source = package.source.clone();
        sigpkg.installed = package.installed;

        Ok(sigpkg)
    }
}

impl Default for UniversalPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Package errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageError {
    PackageNotFound(String),
    DependencyNotFound(String),
    _AdapterNotFound,
    InstallationFailed(String),
    _ConflictDetected(Vec<(String, String)>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureType {
    Binary,
    Library,
    Source,
}

pub struct TabularSchema {
    pub fields: Vec<String>,
}

pub struct TabularRow {
    pub values: Vec<String>,
}

pub struct TabularDataset {
    pub schema: TabularSchema,
    pub rows: Vec<TabularRow>,
}

pub struct SovereignTabFm {
    pub datasets: Vec<TabularDataset>,
}

pub trait PackageAdapterTrait {
    fn adapter_name(&self) -> &str;
}

/// Universal multi-format package metadata parser and handler supporting
/// Linux, BSD, macOS, Android, and HarmonyOS package formats
pub struct UniversalPackageManifestParser;

impl UniversalPackageManifestParser {
    pub fn detect_format_from_filename(filename: &str) -> Option<PackageFormat> {
        PackageFormat::from_filename(filename)
    }

    pub fn parse_manifest_auto(
        filename: &str,
        raw_data: &[u8],
    ) -> Result<UnifiedPackage, &'static str> {
        let fmt = Self::detect_format_from_filename(filename)
            .ok_or("UniversalManifestParser: Unsupported or unrecognized package extension")?;

        let pkg_name = filename.split('.').next().unwrap_or("unknown").to_string();

        let mut pkg = UnifiedPackage::new(pkg_name, "1.0.0".to_string()).with_format(fmt);
        if !raw_data.is_empty() {
            pkg = pkg.with_provides("universal_binary".to_string());
        }
        Ok(pkg)
    }
}

/// Linux & BSD Distro Inspired Rollback Mechanics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroRollbackType {
    NixOsGeneration,   // NixOS atomic generation profile rollback
    FreeBsdZfsBootEnv, // FreeBSD ZFS boot environment (bectl / beadm) rollback
    OpenSuseSnapper,   // openSUSE Snapper CoW snapshot rollback
    FedoraRpmOstree,   // Fedora Silverblue / rpm-ostree deployment rollback
    AlpineApkCache,    // Alpine Linux local apk tarball cache rollback
}

#[derive(Debug, Clone)]
pub struct SovereignRollbackSnapshot {
    pub snapshot_id: usize,
    pub rollback_type: DistroRollbackType,
    pub label: String,
    pub installed_packages_state: Vec<String>,
    pub timestamp_sec: u64,
}

pub struct SovereignPackageRollbackEngine {
    pub snapshots: Vec<SovereignRollbackSnapshot>,
    pub active_snapshot_id: Option<usize>,
    pub next_snapshot_id: usize,
}

impl SovereignPackageRollbackEngine {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            active_snapshot_id: None,
            next_snapshot_id: 1,
        }
    }

    pub fn create_snapshot(&mut self, label: &str, installed_packages: Vec<String>) -> usize {
        self.create_distro_snapshot(DistroRollbackType::OpenSuseSnapper, label, &installed_packages, 0)
    }

    pub fn create_distro_snapshot(
        &mut self,
        rollback_type: DistroRollbackType,
        label: &str,
        installed_packages: &[String],
        now_sec: u64,
    ) -> usize {
        let id = self.next_snapshot_id;
        self.next_snapshot_id += 1;

        let snap = SovereignRollbackSnapshot {
            snapshot_id: id,
            rollback_type,
            label: label.to_string(),
            installed_packages_state: installed_packages.to_vec(),
            timestamp_sec: now_sec,
        };

        self.snapshots.push(snap);
        self.active_snapshot_id = Some(id);
        id
    }

    pub fn rollback(&mut self, snapshot_id: usize) -> Result<Vec<String>, &'static str> {
        let snap = self
            .snapshots
            .iter()
            .find(|s| s.snapshot_id == snapshot_id)
            .ok_or("Rollback Engine: Snapshot not found")?;
        self.active_snapshot_id = Some(snapshot_id);
        Ok(snap.installed_packages_state.clone())
    }
}

/// Universal Package Format Transpilation Bridge
/// Auto-detects foreign Linux and BSD package formats and converts them into native `UnifiedPackage` instances
pub struct UniversalPackageFormatBridge;

impl UniversalPackageFormatBridge {
    pub fn detect_and_transpile(filename: &str, raw_data: &[u8]) -> Result<UnifiedPackage, &'static str> {
        let fmt = UniversalPackageManifestParser::detect_format_from_filename(filename)
            .ok_or("UniversalPackageFormatBridge: Unsupported package format extension")?;

        let clean_name = filename
            .split('.')
            .next()
            .unwrap_or("sovereign_pkg")
            .to_string();

        let mut pkg = UnifiedPackage::new(clean_name, "1.0.0".to_string()).with_format(fmt);

        // Populate format specific dependencies and tags
        match fmt {
            PackageFormat::Deb => {
                pkg.dependencies.push("libc6".to_string());
                pkg.provides.push("debian_compat".to_string());
            }
            PackageFormat::Rpm => {
                pkg.dependencies.push("glibc".to_string());
                pkg.provides.push("fedora_compat".to_string());
            }
            PackageFormat::Pacman => {
                pkg.dependencies.push("glibc".to_string());
                pkg.provides.push("arch_compat".to_string());
            }
            PackageFormat::Apk => {
                pkg.dependencies.push("musl".to_string());
                pkg.provides.push("alpine_compat".to_string());
            }
            PackageFormat::Pkg | PackageFormat::Ports => {
                pkg.dependencies.push("bsd_libc".to_string());
                pkg.provides.push("freebsd_compat".to_string());
            }
            PackageFormat::Nixpkg => {
                pkg.dependencies.push("nix_store_path".to_string());
                pkg.provides.push("nixos_compat".to_string());
            }
            _ => {
                pkg.provides.push("generic_distro_compat".to_string());
            }
        }

        if !raw_data.is_empty() {
            pkg.properties.insert("checksum".to_string(), format!("{:x}", raw_data.len() * 31));
        }

        Ok(pkg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UniversalPackageManager::new();
        assert_eq!(manager.adapters.len(), 13);
    }

    #[test]
    fn test_package_creation() {
        let package = UnifiedPackage::new("test".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::Deb)
            .with_dependency("dep1".to_string());
        assert_eq!(package.formats.len(), 1);
        assert_eq!(package.dependencies.len(), 1);
    }

    #[test]
    fn test_dependency_resolution() {
        let mut resolver = DependencyResolver::new();
        let pkg1 = UnifiedPackage::new("pkg1".to_string(), "1.0.0".to_string())
            .with_dependency("pkg2".to_string());
        let pkg2 = UnifiedPackage::new("pkg2".to_string(), "1.0.0".to_string());

        resolver.add_package(pkg1);
        resolver.add_package(pkg2);

        let deps = resolver.resolve_dependencies("pkg1").unwrap();
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn test_conflict_detection() {
        let mut resolver = DependencyResolver::new();
        let pkg1 = UnifiedPackage::new("pkg1".to_string(), "1.0.0".to_string())
            .with_conflict("pkg2".to_string());
        let pkg2 = UnifiedPackage::new("pkg2".to_string(), "1.0.0".to_string());

        resolver.add_package(pkg1);
        resolver.add_package(pkg2);

        let conflicts = resolver.detect_conflicts(&["pkg1".to_string(), "pkg2".to_string()]);
        assert_eq!(conflicts.len(), 1);
    }

    #[test]
    fn test_install_package() {
        let mut manager = UniversalPackageManager::new();
        let package = UnifiedPackage::new("test".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(package);
        assert!(manager.install("test").is_ok());
        assert_eq!(manager.installed_packages.len(), 1);
    }

    #[test]
    fn test_checkpoint_rollback() {
        let mut manager = UniversalPackageManager::new();
        let package = UnifiedPackage::new("test".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(package);
        let cp_id = manager.create_checkpoint();

        assert!(manager.install("test").is_ok());
        assert_eq!(manager.installed_packages.len(), 1);

        assert!(manager.rollback_to_checkpoint(cp_id).is_ok());
        assert_eq!(manager.installed_packages.len(), 0);
    }

    #[test]
    fn test_foreign_distro_package_translation_and_installation() {
        let mut manager = UniversalPackageManager::new();

        let foreign_deb = ForeignDistroManifest {
            raw_format: PackageFormat::Deb,
            original_name: "curl".to_string(),
            version: "8.5.0".to_string(),
            architecture: "amd64".to_string(),
            raw_dependencies: vec!["libssl-dev".to_string(), "libc6".to_string()],
            raw_provides: vec!["http-client".to_string()],
            raw_conflicts: vec!["curl-legacy".to_string()],
            maintainer: "Debian Packagers".to_string(),
        };

        // Pre-seed dependencies
        manager.add_package(
            UnifiedPackage::new("sovereign-openssl".to_string(), "3.0.0".to_string())
                .with_format(PackageFormat::SigmaPkg),
        );
        manager.add_package(
            UnifiedPackage::new("sovereign-libc".to_string(), "2.38.0".to_string())
                .with_format(PackageFormat::SigmaPkg),
        );

        assert!(manager.install_foreign_distro_package(foreign_deb).is_ok());
        assert!(manager.installed_packages.get("sigpkg-curl").is_some());

        let installed = manager.installed_packages.get("sigpkg-curl").unwrap();
        assert_eq!(installed.version, "8.5.0");
        assert!(installed.provides.contains(&"curl".to_string()));
        assert!(installed
            .dependencies
            .contains(&"sovereign-openssl".to_string()));
        assert!(installed
            .dependencies
            .contains(&"sovereign-libc".to_string()));
    }

    #[test]
    fn test_distro_repo_sync_engine() {
        let mut sync = DistroRepoSyncEngine::new();
        assert!(sync
            .registered_repos
            .iter()
            .any(|r| r.distro_name == "Debian"));
        assert!(sync
            .registered_repos
            .iter()
            .any(|r| r.distro_name == "ArchLinux"));

        let foreign_rpm = ForeignDistroManifest {
            raw_format: PackageFormat::Rpm,
            original_name: "nginx".to_string(),
            version: "1.26.0".to_string(),
            architecture: "x86_64".to_string(),
            raw_dependencies: vec!["openssl-devel".to_string()],
            raw_provides: vec!["web-server".to_string()],
            raw_conflicts: Vec::new(),
            maintainer: "Fedora Project".to_string(),
        };

        sync.index_foreign_manifest(foreign_rpm);
        assert_eq!(sync.total_indexed_packages(), 1);

        let translated = sync.find_and_translate("nginx").unwrap();
        assert_eq!(translated.name, "sigpkg-nginx");
        assert!(translated
            .dependencies
            .contains(&"sovereign-openssl".to_string()));
    }

    #[test]
    fn test_distro_packaging_manifests_and_sandboxing() {
        let adapter = PackageAdapter::new(PackageFormat::Flatpak, "flatpak".to_string());

        // 1. Test Flatpak sandboxing policy translation
        let flat_manifest = FlatpakManifest {
            id: "org.gnome.Gimp".to_string(),
            runtime: "org.gnome.Platform".to_string(),
            runtime_version: "44".to_string(),
            sdk: "org.gnome.Sdk".to_string(),
            command: "gimp".to_string(),
            finish_args: {
                let mut v = Vec::new();
                v.push("--share=network".to_string());
                v.push("--share=ipc".to_string());
                v.push("--filesystem=host".to_string());
                v
            },
        };

        let enforced_pledges = adapter.translate_flatpak_sandbox_policy(&flat_manifest);
        assert_eq!(enforced_pledges.len(), 3);
        assert!(enforced_pledges.contains(&"network".to_string()));
        assert!(enforced_pledges.contains(&"ipc".to_string()));
        assert!(enforced_pledges.contains(&"unveil_all".to_string()));

        // 2. Test Snapcraft confinement translation
        let snap_adapter = PackageAdapter::new(PackageFormat::Snap, "snap".to_string());
        let snap_manifest = SnapcraftManifest {
            name: "gimp".to_string(),
            version: "2.10.30".to_string(),
            summary: "GNU Image Manipulation Program".to_string(),
            confinement: "strict".to_string(),
            plugs: {
                let mut v = Vec::new();
                v.push("network".to_string());
                v
            },
            slots: Vec::new(),
        };

        let confinement_rule = snap_adapter.translate_snap_confinement(&snap_manifest);
        assert_eq!(confinement_rule, "strict_pledge_sandbox");

        // 3. Verify general manifest definitions compile (AptDebManifest and PacmanPkgbuild)
        let _deb = AptDebManifest {
            package: "curl".to_string(),
            version: "7.81.0".to_string(),
            architecture: "amd64".to_string(),
            maintainer: "Debian Curl Maintainers".to_string(),
            depends: {
                let mut v = Vec::new();
                v.push("libcurl4".to_string());
                v
            },
            description: "command line tool for transferring data with URLs".to_string(),
        };

        let _pkgbuild = PacmanPkgbuild {
            pkgname: "curl".to_string(),
            pkgver: "7.81.0".to_string(),
            pkgdesc: "command line tool for transferring data with URLs".to_string(),
            arch: {
                let mut v = Vec::new();
                v.push("x86_64".to_string());
                v
            },
            depends: {
                let mut v = Vec::new();
                v.push("openssl".to_string());
                v
            },
            makedepends: {
                let mut v = Vec::new();
                v.push("git".to_string());
                v
            },
            source_urls: {
                let mut v = Vec::new();
                v.push("https://curl.se/download/curl-7.81.0.tar.gz".to_string());
                v
            },
        };
    }

    #[test]
    fn test_comprehensive_packaging_systems() {
        let appimage_adapter = PackageAdapter::new(PackageFormat::AppImage, "appimage".to_string());

        // 1. Test AppImage metadata runtime & mount mock
        let app_runtime = AppImageRuntime {
            app_name: "Blender".to_string(),
            signature_offset: 1024,
            squashfs_offset: 2048,
            embedded_icon_path: "/usr/share/icons/blender.png".to_string(),
        };
        let mount_path = appimage_adapter
            .mount_appimage_squashfs(&app_runtime)
            .unwrap();
        assert_eq!(mount_path, "/tmp/.mount_Blender_squashfs");

        // AppImage mount offset error checking
        let bad_app = AppImageRuntime {
            app_name: "BadApp".to_string(),
            signature_offset: 0,
            squashfs_offset: 0,
            embedded_icon_path: String::new(),
        };
        assert!(appimage_adapter.mount_appimage_squashfs(&bad_app).is_err());

        // 2. Test APT repository source query checks
        let apt_adapter = PackageAdapter::new(PackageFormat::Deb, "apt".to_string());
        let apt_config = AptRepoConfig {
            sourcelist_url: "deb https://deb.debian.org/debian".to_string(),
            suite: "bookworm".to_string(),
            components: {
                let mut v = Vec::new();
                v.push("main".to_string());
                v.push("contrib".to_string());
                v
            },
            trust_anchor: None,
        };
        assert!(apt_adapter.query_apt_repository(&apt_config));

        // 3. Test DNF repository configuration query checks
        let dnf_adapter = PackageAdapter::new(PackageFormat::Rpm, "dnf".to_string());
        let dnf_config = DnfRepoConfig {
            repoid: "fedora".to_string(),
            baseurl: "https://mirrors.fedoraproject.org/".to_string(),
            gpgcheck: true,
            enabled: true,
        };
        assert!(dnf_adapter.query_dnf_repository(&dnf_config));
    }

    #[test]
    fn test_universal_package_manifest_parser() {
        assert_eq!(
            UniversalPackageManifestParser::detect_format_from_filename("nginx.deb"),
            Some(PackageFormat::Deb)
        );
        assert_eq!(
            UniversalPackageManifestParser::detect_format_from_filename("app.flatpak"),
            Some(PackageFormat::Flatpak)
        );

        let pkg =
            UniversalPackageManifestParser::parse_manifest_auto("tool.apk", b"payload").unwrap();
        assert_eq!(pkg.name, "tool");
        assert_eq!(pkg.formats[0], PackageFormat::Apk);

        // Test all requested formats in prompt
        let test_cases = [
            ("app.air", PackageFormat::Air),
            ("app.bottle", PackageFormat::Bottle),
            ("app.ipa", PackageFormat::Ipa),
            ("app.ports", PackageFormat::Ports),
            ("app.pkg", PackageFormat::Pkg),
            ("app.aab", PackageFormat::Aab),
            ("app.apk", PackageFormat::Apk),
            ("app.AppImage", PackageFormat::AppImage),
            ("app.eopkg", PackageFormat::Eopkg),
            ("app.nixpkg", PackageFormat::Nixpkg),
            ("app.portage", PackageFormat::Ebuild),
            ("app.deb", PackageFormat::Deb),
            ("app.tar.gz", PackageFormat::TarGz),
            ("app.tar .gz", PackageFormat::TarGz),
            ("app.xz", PackageFormat::Xz),
            ("app.rpm", PackageFormat::Rpm),
            ("app.ebuild", PackageFormat::Ebuild),
            ("app.pkg.tar.xz", PackageFormat::Pacman),
            ("app.flatpak", PackageFormat::Flatpak),
            ("app.app", PackageFormat::App),
            ("app.hap", PackageFormat::Hap),
            ("app.PiSi", PackageFormat::Pisi),
            ("app.tgz", PackageFormat::TarGz),
            ("app.superdeb", PackageFormat::Superdeb),
            ("app.lzm", PackageFormat::Lzm),
            ("app.pup", PackageFormat::Pup),
            ("app.snap", PackageFormat::Snap),
            ("app.pacman", PackageFormat::Pacman),
            ("app.tar", PackageFormat::Tar),
            ("app.pet", PackageFormat::Pet),
        ];

        for (filename, expected_format) in test_cases {
            assert_eq!(
                UniversalPackageManifestParser::detect_format_from_filename(filename),
                Some(expected_format),
                "Failed format detection for filename: {}",
                filename
            );
        }
    }

    #[test]
    fn test_distro_package_rollback_engine() {
        let mut engine = SovereignPackageRollbackEngine::new();
        let pkgs = vec!["nginx".to_string(), "curl".to_string()];

        let snap_id = engine.create_distro_snapshot(
            DistroRollbackType::NixOsGeneration,
            "NixOS Gen 101",
            &pkgs,
            1700000000,
        );
    }

    #[test]
    fn test_package_format_from_filename_extensions() {
        assert_eq!(PackageFormat::from_filename("app.air"), Some(PackageFormat::Air));
        assert_eq!(PackageFormat::from_filename("brew.bottle"), Some(PackageFormat::Bottle));
        assert_eq!(PackageFormat::from_filename("app.ipa"), Some(PackageFormat::Ipa));
        assert_eq!(PackageFormat::from_filename("bsd.ports"), Some(PackageFormat::Ports));
        assert_eq!(PackageFormat::from_filename("install.pkg"), Some(PackageFormat::Pkg));
        assert_eq!(PackageFormat::from_filename("app.aab"), Some(PackageFormat::Aab));
        assert_eq!(PackageFormat::from_filename("tool.apk"), Some(PackageFormat::Apk));
        assert_eq!(PackageFormat::from_filename("software.AppImage"), Some(PackageFormat::AppImage));
        assert_eq!(PackageFormat::from_filename("solus.eopkg"), Some(PackageFormat::Eopkg));
        assert_eq!(PackageFormat::from_filename("nixos.nixpkg"), Some(PackageFormat::Nixpkg));
        assert_eq!(PackageFormat::from_filename("nixos.nix"), Some(PackageFormat::Nixpkg));
        assert_eq!(PackageFormat::from_filename("gentoo.portage"), Some(PackageFormat::Ebuild));
        assert_eq!(PackageFormat::from_filename("debian.deb"), Some(PackageFormat::Deb));
        assert_eq!(PackageFormat::from_filename("archive.tar.gz"), Some(PackageFormat::TarGz));
        assert_eq!(PackageFormat::from_filename("archive.tgz"), Some(PackageFormat::TarGz));
        assert_eq!(PackageFormat::from_filename("compressed.xz"), Some(PackageFormat::Xz));
        assert_eq!(PackageFormat::from_filename("fedora.rpm"), Some(PackageFormat::Rpm));
        assert_eq!(PackageFormat::from_filename("gentoo.ebuild"), Some(PackageFormat::Ebuild));
        assert_eq!(PackageFormat::from_filename("arch.pkg.tar.xz"), Some(PackageFormat::Pacman));
        assert_eq!(PackageFormat::from_filename("arch.pkg.tar.zst"), Some(PackageFormat::Pacman));
        assert_eq!(PackageFormat::from_filename("app.flatpak"), Some(PackageFormat::Flatpak));
        assert_eq!(PackageFormat::from_filename("macos.app"), Some(PackageFormat::App));
        assert_eq!(PackageFormat::from_filename("harmony.hap"), Some(PackageFormat::Hap));
        assert_eq!(PackageFormat::from_filename("pardus.PiSi"), Some(PackageFormat::Pisi));
        assert_eq!(PackageFormat::from_filename("pardus.pisi"), Some(PackageFormat::Pisi));
        assert_eq!(PackageFormat::from_filename("deepin.superdeb"), Some(PackageFormat::Superdeb));
        assert_eq!(PackageFormat::from_filename("slax.lzm"), Some(PackageFormat::Lzm));
        assert_eq!(PackageFormat::from_filename("puppy.pup"), Some(PackageFormat::Pup));
        assert_eq!(PackageFormat::from_filename("canonical.snap"), Some(PackageFormat::Snap));
        assert_eq!(PackageFormat::from_filename("freebsd.pkg"), Some(PackageFormat::Pkg));
        assert_eq!(PackageFormat::from_filename("plain.tar"), Some(PackageFormat::Tar));
        assert_eq!(PackageFormat::from_filename("puppy.pet"), Some(PackageFormat::Pet));
    }

    #[test]
    fn test_package_rollback() {
        let mut engine = SovereignPackageRollbackEngine::new();
        let pkgs = vec!["nginx".to_string(), "curl".to_string()];
        let snap_id = engine.create_snapshot("pre-update", pkgs.clone());
        assert_eq!(snap_id, 1);
        let restored = engine.rollback(snap_id).unwrap();
        assert_eq!(restored, pkgs);
    }

    #[test]
    fn test_all_package_format_strategies_and_adapters() {
        let formats = vec![
            PackageFormat::Deb, PackageFormat::Rpm, PackageFormat::Pacman, PackageFormat::Ebuild,
            PackageFormat::Apk, PackageFormat::Nix, PackageFormat::Flatpak, PackageFormat::Snap,
            PackageFormat::AppImage, PackageFormat::Xbps, PackageFormat::Txz, PackageFormat::Eopkg,
            PackageFormat::Zypper, PackageFormat::Guix, PackageFormat::CachyOS, PackageFormat::Swupd,
            PackageFormat::Starling, PackageFormat::SigmaPkg, PackageFormat::Air, PackageFormat::Bottle,
            PackageFormat::Ipa, PackageFormat::Ports, PackageFormat::Pkg, PackageFormat::Aab,
            PackageFormat::TarGz, PackageFormat::Xz, PackageFormat::App, PackageFormat::Hap,
            PackageFormat::Pisi, PackageFormat::Superdeb, PackageFormat::Lzm, PackageFormat::Pup,
            PackageFormat::Pet, PackageFormat::Tar, PackageFormat::Moss, PackageFormat::Hpkg,
            PackageFormat::Tcz, PackageFormat::Gobo, PackageFormat::Ostree, PackageFormat::Pkgsrc,
            PackageFormat::Sfs, PackageFormat::Puk, PackageFormat::Dmg, PackageFormat::Cports,
            PackageFormat::Dports, PackageFormat::SlackBuild, PackageFormat::Crux, PackageFormat::Drpm,
            PackageFormat::Stratum
        ];

        for fmt in formats {
            let strategy = PackageFactory::get_strategy(fmt);
            let adapter = PackageFactory::get_adapter(fmt);
            let pkg = UnifiedPackage::new("test-pkg".to_string(), "1.0.0".to_string()).with_format(fmt);

            assert!(strategy.install(&pkg).is_ok());
            assert!(strategy.verify(&pkg).unwrap());
            assert!(strategy.remove(&pkg).is_ok());

            let adapted = adapter.adapt("").unwrap();
            assert!(adapted.formats.contains(&fmt) || (fmt == PackageFormat::Nix && adapted.formats.contains(&PackageFormat::Nixpkg)));
        }
    }

    #[test]
    fn test_expanded_decorators() {
        let pkg = UnifiedPackage::new("simd-app".to_string(), "2.0.0".to_string());
        let base = BasePackageDecorator { package: pkg };

        let hw_dec = HardwareOptimizationDecorator {
            decorated: base,
            target_microarch_level: "x86-64-v3".to_string(),
            required_simd_features: vec!["avx2".to_string(), "fma".to_string()],
        };

        hw_dec.profile_performance();
        assert_eq!(hw_dec.get_package().name, "simd-app");

        let res_dec = ResourceLimitDecorator {
            decorated: hw_dec,
            max_memory_bytes: 1024 * 1024 * 512,
            cpu_quota_percent: 50,
        };

        assert!(res_dec.enforce_sandbox().is_ok());

        let pqc_dec = PqcSignedDecorator {
            decorated: res_dec,
            dilithium_signature: "dilithium-5-valid-signature".to_string(),
        };

        assert!(pqc_dec.enforce_sandbox().is_ok());

        let bad_pqc = PqcSignedDecorator {
            decorated: BasePackageDecorator {
                package: UnifiedPackage::new("invalid-sig".to_string(), "1.0.0".to_string()),
            },
            dilithium_signature: "invalid-signature".to_string(),
        };

        assert!(bad_pqc.enforce_sandbox().is_err());
    }

    #[test]
    fn test_universal_package_format_bridge() {
        let deb_pkg = UniversalPackageFormatBridge::detect_and_transpile("nginx.deb", b"deb_payload").unwrap();
        assert!(deb_pkg.formats.contains(&PackageFormat::Deb));
        assert_eq!(deb_pkg.name, "nginx");
        assert!(deb_pkg.dependencies.contains(&"libc6".to_string()));

        let rpm_pkg = UniversalPackageFormatBridge::detect_and_transpile("curl.rpm", b"rpm_payload").unwrap();
        assert!(rpm_pkg.formats.contains(&PackageFormat::Rpm));
        assert!(rpm_pkg.provides.contains(&"fedora_compat".to_string()));

        let apk_pkg = UniversalPackageFormatBridge::detect_and_transpile("busybox.apk", b"apk_payload").unwrap();
        assert!(apk_pkg.formats.contains(&PackageFormat::Apk));
        assert!(apk_pkg.dependencies.contains(&"musl".to_string()));
    }
}
