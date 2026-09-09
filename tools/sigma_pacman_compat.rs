// SPDX-License-Identifier: MIT
// SigmaOS Arch Linux Pacman & AUR Helper Compatibility Utility (sigma_pacman_compat)
// Clean-room representation of Arch Linux's core package utility suite (pacman, yay, paru)

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ArchPackageRecord {
    pub name: String,
    pub version: String,
    pub repository: String, // "core", "extra", "community", or "aur"
    pub dependencies: Vec<String>,
    pub description: String,
}

pub struct PacmanSystem {
    pub sync_database: HashMap<String, ArchPackageRecord>,
    pub aur_database: HashMap<String, ArchPackageRecord>,
    pub installed_packages: HashMap<String, ArchPackageRecord>,
    pub package_owners: HashMap<String, String>, // file path -> pkgname mapping for -Qo
    pub db_synced: bool,
    pub db_locked: bool,
}

impl PacmanSystem {
    pub fn new() -> Self {
        let mut sys = Self {
            sync_database: HashMap::new(),
            aur_database: HashMap::new(),
            installed_packages: HashMap::new(),
            package_owners: HashMap::new(),
            db_synced: false,
            db_locked: false,
        };
        sys.load_default_arch_repos();
        sys
    }

    /// Acquires pacman database lock (/var/lib/pacman/db.lck)
    pub fn lock_db(&mut self) -> Result<(), &'static str> {
        if self.db_locked {
            return Err("pacman Error: Database is locked by another process (/var/lib/pacman/db.lck)");
        }
        self.db_locked = true;
        Ok(())
    }

    /// Releases pacman database lock
    pub fn unlock_db(&mut self) {
        self.db_locked = false;
    }

    /// Preloads standard Arch Linux core/extra/community and AUR repository indices
    fn load_default_arch_repos(&mut self) {
        // Official Repositories
        self.sync_database.insert(
            "linux".to_string(),
            ArchPackageRecord {
                name: "linux".to_string(),
                version: "6.5.9.arch1-1".to_string(),
                repository: "core".to_string(),
                dependencies: Vec::new(),
                description: "The Linux kernel and modules".to_string(),
            },
        );

        self.sync_database.insert(
            "glibc".to_string(),
            ArchPackageRecord {
                name: "glibc".to_string(),
                version: "2.38-7".to_string(),
                repository: "core".to_string(),
                dependencies: Vec::new(),
                description: "GNU C Library".to_string(),
            },
        );

        self.sync_database.insert(
            "neovim".to_string(),
            ArchPackageRecord {
                name: "neovim".to_string(),
                version: "0.9.4-1".to_string(),
                repository: "extra".to_string(),
                dependencies: vec!["glibc".to_string(), "libuv".to_string()],
                description: "Vim-fork focused on extensibility and usability".to_string(),
            },
        );

        self.sync_database.insert(
            "libuv".to_string(),
            ArchPackageRecord {
                name: "libuv".to_string(),
                version: "1.46.0-1".to_string(),
                repository: "extra".to_string(),
                dependencies: vec!["glibc".to_string()],
                description: "Multi-platform support library with a focus on asynchronous I/O".to_string(),
            },
        );

        // Arch User Repository (AUR)
        self.aur_database.insert(
            "google-chrome".to_string(),
            ArchPackageRecord {
                name: "google-chrome".to_string(),
                version: "119.0.6045.105-1".to_string(),
                repository: "aur".to_string(),
                dependencies: vec!["glibc".to_string()],
                description: "The popular web browser from Google (AUR PKGBUILD)".to_string(),
            },
        );
    }

    /// Simulates 'pacman -Sy' or 'pacman -Syu' database sync
    pub fn pacman_sync_db(&mut self) -> Result<usize, &'static str> {
        println!(":: Synchronizing package databases...");
        println!(" downloading core.db...");
        println!(" downloading extra.db...");
        println!(" downloading community.db...");
        self.db_synced = true;
        Ok(self.sync_database.len())
    }

    /// Simulates 'pacman -Ss <query>' search
    pub fn pacman_search(&self, query: &str) -> Vec<ArchPackageRecord> {
        let mut results = Vec::new();
        for pkg in self.sync_database.values() {
            if pkg.name.contains(query) || pkg.description.contains(query) {
                results.push(pkg.clone());
            }
        }
        for pkg in self.aur_database.values() {
            if pkg.name.contains(query) || pkg.description.contains(query) {
                results.push(pkg.clone());
            }
        }
        results.sort_by(|a, b| a.name.cmp(&b.name));
        results
    }

    /// Simulates 'pacman -S <pkg>' installation from official repos
    pub fn pacman_install(&mut self, package_name: &str) -> Result<Vec<String>, &'static str> {
        if !self.db_synced {
            return Err("pacman Error: Databases not synchronized (run pacman -Sy).");
        }

        if !self.sync_database.contains_key(package_name) {
            return Err("pacman Error: Target not found in official repositories.");
        }

        let mut install_order = Vec::new();
        let mut visited = HashMap::new();

        self.resolve_deps(package_name, &mut install_order, &mut visited)?;

        println!("resolving dependencies...");
        println!("looking for conflicting packages...");
        println!("Packages ({}) {}", install_order.len(), install_order.join(" "));

        for pkg_name in &install_order {
            if let Some(record) = self.sync_database.get(pkg_name) {
                self.installed_packages.insert(pkg_name.clone(), record.clone());
                println!(":: Processing package changes...");
                println!("({}) installing {}...", pkg_name, record.version);
            }
        }

        Ok(install_order)
    }

    /// Helper dependency resolver
    fn resolve_deps(
        &self,
        name: &str,
        order: &mut Vec<String>,
        visited: &mut HashMap<String, bool>,
    ) -> Result<(), &'static str> {
        if let Some(&in_progress) = visited.get(name) {
            if in_progress {
                return Err("pacman Error: Circular dependency detected.");
            }
            return Ok(());
        }

        if self.installed_packages.contains_key(name) {
            return Ok(());
        }

        visited.insert(name.to_string(), true);

        if let Some(record) = self.sync_database.get(name) {
            for dep in &record.dependencies {
                self.resolve_deps(dep, order, visited)?;
            }
        }

        visited.insert(name.to_string(), false);
        if !order.contains(&name.to_string()) {
            order.push(name.to_string());
        }

        Ok(())
    }

    /// Simulates 'yay -S <aur_pkg>' or 'paru -S <aur_pkg>' AUR helper execution
    pub fn yay_aur_install(&mut self, aur_package: &str) -> Result<(), &'static str> {
        if !self.aur_database.contains_key(aur_package) {
            return Err("yay Error: Package not found in AUR database.");
        }

        let record = self.aur_database.get(aur_package).unwrap().clone();
        println!(":: Fetching PKGBUILD from AUR for {}...", aur_package);
        println!(":: Parsing PKGBUILD recipe... OK");
        println!(":: Compiling {} in sandboxed Ring 3 workspace...", aur_package);
        self.installed_packages.insert(aur_package.to_string(), record);
        println!(":: Package {} installed successfully via yay helper.", aur_package);
        Ok(())
    }

    /// Simulates 'pacman -R <pkg>' package removal
    pub fn pacman_remove(&mut self, package_name: &str) -> Result<(), &'static str> {
        if self.db_locked {
            return Err("pacman Error: Database is locked by another process (/var/lib/pacman/db.lck)");
        }
        if !self.installed_packages.contains_key(package_name) {
            return Err("pacman Error: Target not found in installed database.");
        }

        self.installed_packages.remove(package_name);
        println!("checking dependencies...");
        println!(":: Removing {}...", package_name);
        Ok(())
    }

    /// Simulates 'pacman -Q' list installed packages
    pub fn pacman_query_installed(&self) -> Vec<ArchPackageRecord> {
        let mut pkgs: Vec<ArchPackageRecord> = self.installed_packages.values().cloned().collect();
        pkgs.sort_by(|a, b| a.name.cmp(&b.name));
        pkgs
    }

    /// Simulates 'pacman -Qe' list explicitly installed packages
    pub fn pacman_query_explicit(&self) -> Vec<ArchPackageRecord> {
        // In our simulation, non-dependency installed packages are treated as explicit
        let mut explicit = Vec::new();
        for pkg in self.installed_packages.values() {
            let is_dep = self.installed_packages.values().any(|other| other.dependencies.contains(&pkg.name));
            if !is_dep {
                explicit.push(pkg.clone());
            }
        }
        explicit.sort_by(|a, b| a.name.cmp(&b.name));
        explicit
    }

    /// Simulates 'pacman -Qdt' list unneeded orphan packages
    pub fn pacman_query_orphans(&self) -> Vec<ArchPackageRecord> {
        let mut orphans = Vec::new();
        for pkg in self.installed_packages.values() {
            // An orphan is a package that was installed as a dependency, but no installed package depends on it
            let is_depended_on = self.installed_packages.values().any(|other| other.dependencies.contains(&pkg.name));
            let is_explicit_target = self.sync_database.get(&pkg.name).map_or(true, |p| p.dependencies.is_empty());
            if !is_depended_on && !is_explicit_target {
                orphans.push(pkg.clone());
            }
        }
        orphans.sort_by(|a, b| a.name.cmp(&b.name));
        orphans
    }

    /// Simulates 'pacman -Qo <file>' file ownership query
    pub fn pacman_query_owner(&self, filepath: &str) -> Result<String, &'static str> {
        if let Some(owner) = self.package_owners.get(filepath) {
            Ok(format!("{} is owned by {} {}", filepath, owner, self.installed_packages.get(owner).map_or("unknown", |p| &p.version)))
        } else {
            Err("pacman Error: No package owns specified file.")
        }
    }
}

/// Represents an Arch package mirror entry for Reflector ranking
#[derive(Debug, Clone)]
pub struct ArchMirror {
    pub url: String,
    pub country: String,
    pub ping_ms: u32,
    pub rate_kbps: u32,
}

/// Reflector Arch Linux mirror ranking generator
#[derive(Debug, Clone, Default)]
pub struct ReflectorMirrorEngine {
    pub mirrors: Vec<ArchMirror>,
}

impl ReflectorMirrorEngine {
    pub fn new() -> Self {
        Self {
            mirrors: vec![
                ArchMirror { url: "https://mirror.rackspace.com/archlinux/$repo/os/$arch".to_string(), country: "US".to_string(), ping_ms: 22, rate_kbps: 15000 },
                ArchMirror { url: "https://archlinux.honkgong.info/$repo/os/$arch".to_string(), country: "DE".to_string(), ping_ms: 110, rate_kbps: 8000 },
                ArchMirror { url: "https://kernel.org/archlinux/$repo/os/$arch".to_string(), country: "US".to_string(), ping_ms: 15, rate_kbps: 25000 },
            ],
        }
    }

    /// Ranks mirrors by download rate (fastest first) and generates /etc/pacman.d/mirrorlist
    pub fn generate_ranked_mirrorlist(&self, country_filter: Option<&str>, limit: usize) -> String {
        let mut filtered: Vec<ArchMirror> = self.mirrors.iter()
            .filter(|m| country_filter.map_or(true, |c| m.country.eq_ignore_ascii_case(c)))
            .cloned()
            .collect();

        filtered.sort_by(|a, b| b.rate_kbps.cmp(&a.rate_kbps));
        filtered.truncate(limit);

        let mut out = String::from("## Arch Linux mirrorlist ranked by Reflector\n");
        for m in filtered {
            out.push_str(&format!("## {}, Speed: {} KB/s, Ping: {} ms\nServer = {}\n\n", m.country, m.rate_kbps, m.ping_ms, m.url));
        }
        out
    }
}

/// Arch Linux `arch-chroot` VFS isolation and mount manager
#[derive(Debug, Clone)]
pub struct ArchChrootEnvironment {
    pub target_root: String,
    pub mounted_vfs: Vec<String>,
}

impl ArchChrootEnvironment {
    pub fn new(target_root: &str) -> Self {
        Self {
            target_root: target_root.to_string(),
            mounted_vfs: Vec::new(),
        }
    }

    pub fn mount_virtual_filesystems(&mut self) -> Vec<String> {
        let mounts = vec![
            format!("{}/proc", self.target_root),
            format!("{}/sys", self.target_root),
            format!("{}/dev", self.target_root),
            format!("{}/dev/pts", self.target_root),
            format!("{}/run", self.target_root),
        ];
        self.mounted_vfs = mounts.clone();
        mounts
    }

    pub fn generate_chroot_command(&self, cmd: &str) -> String {
        format!("chroot {} /bin/bash -c \"{}\"", self.target_root, cmd)
    }
}

/// Arch Linux `pacman-key` GPG keyring and Web of Trust manager
#[derive(Debug, Clone)]
pub struct PacmanKeyringManager {
    pub keyring_dir: String,
    pub master_keys: Vec<String>,
    pub trusted_fingerprints: Vec<String>,
}

impl PacmanKeyringManager {
    pub fn new() -> Self {
        Self {
            keyring_dir: "/etc/pacman.d/gnupg".to_string(),
            master_keys: vec!["archlinux".to_string()],
            trusted_fingerprints: Vec::new(),
        }
    }

    pub fn init_keyring(&mut self) -> Result<(), &'static str> {
        self.trusted_fingerprints.push("4A8B569A89012345".to_string());
        Ok(())
    }

    pub fn populate_arch_keyring(&mut self) -> usize {
        let keys = vec![
            "3B94A80E50A477C71F41250B09F89B57748652A5", // Pierre Schmitz
            "E253630B257A589C92063209A3D0D2B828DA2D42", // Levente Polyak
        ];
        for k in keys {
            self.trusted_fingerprints.push(k.to_string());
        }
        self.trusted_fingerprints.len()
    }

    pub fn verify_package_signature(&self, _pkg_path: &str) -> bool {
        !self.trusted_fingerprints.is_empty()
    }
}

/// Arch Linux `checkupdates` safe upgrade checking engine
#[derive(Debug, Clone, Default)]
pub struct CheckupdatesEngine;

impl CheckupdatesEngine {
    pub fn check_pending_upgrades(sys: &PacmanSystem) -> Vec<(String, String, String)> {
        let mut pending = Vec::new();
        for (name, installed_pkg) in &sys.installed_packages {
            if let Some(sync_pkg) = sys.sync_database.get(name) {
                if sync_pkg.version != installed_pkg.version {
                    pending.push((name.clone(), installed_pkg.version.clone(), sync_pkg.version.clone()));
                }
            }
        }
        pending
    }
}

/// Arch Linux `updpkgsums` PKGBUILD checksum updater
#[derive(Debug, Clone, Default)]
pub struct UpdpkgsumsEngine;

impl UpdpkgsumsEngine {
    pub fn compute_sha256_sums(sources: &[String]) -> Vec<String> {
        sources
            .iter()
            .map(|src| {
                let mut hash: u64 = 5381;
                for b in src.bytes() {
                    hash = hash.wrapping_mul(33).wrapping_add(b as u64);
                }
                format!("{:064x}", hash)
            })
            .collect()
    }

    pub fn update_recipe_checksums(recipe: &mut PkgbuildRecipe) {
        recipe.sha256sums = Self::compute_sha256_sums(&recipe.source);
    }
}

/// Arch Linux `makepkg` package compiler producing `.pkg.tar.zst` artifacts
#[derive(Debug, Clone)]
pub struct MakepkgExecutor {
    pub recipe: PkgbuildRecipe,
    pub clean_build: bool,
}

impl MakepkgExecutor {
    pub fn new(recipe: PkgbuildRecipe) -> Self {
        Self {
            recipe,
            clean_build: true,
        }
    }

    pub fn build_package_tarball(&self) -> Result<String, &'static str> {
        let warnings = NamcapLinter::lint_pkgbuild(&self.recipe);
        if warnings.iter().any(|w| w.starts_with("E:")) {
            return Err("makepkg failed: PKGBUILD contains critical errors");
        }
        let pkg_filename = format!(
            "{}-{}-{}-{}.pkg.tar.zst",
            self.recipe.pkgname,
            self.recipe.pkgver,
            self.recipe.pkgrel,
            self.recipe.arch.first().cloned().unwrap_or_else(|| "x86_64".to_string())
        );
        Ok(pkg_filename)
    }
}

/// Represents a filesystem mount point for Arch 'genfstab' generator
#[derive(Debug, Clone)]
pub struct ArchMountPoint {
    pub device: String,
    pub mount_point: String,
    pub fstype: String,
    pub options: String,
    pub dump: u32,
    pub pass: u32,
    pub uuid: String,
}

/// Generates /etc/fstab content with UUID tags (Arch Linux 'genfstab -U' parity)
pub fn generate_genfstab(mounts: &[ArchMountPoint]) -> String {
    let mut out = String::from("# /etc/fstab: static file system information.\n");
    out.push_str("# Generated by SigmaOS genfstab Arch Linux compatibility engine\n#\n");
    out.push_str("# <file system>                           <dir>       <type>  <options>       <dump>  <pass>\n");

    for m in mounts {
        let fs_spec = if !m.uuid.is_empty() {
            format!("UUID={}", m.uuid)
        } else {
            m.device.clone()
        };
        out.push_str(&format!(
            "{:<42} {:<11} {:<7} {:<15} {}      {}\n",
            fs_spec, m.mount_point, m.fstype, m.options, m.dump, m.pass
        ));
    }
    out
}

/// Generates Arch Linux /etc/mkinitcpio.conf initramfs builder configuration
pub fn generate_mkinitcpio_conf(hooks: &[&str], modules: &[&str]) -> String {
    let mut out = String::from("# /etc/mkinitcpio.conf - SigmaOS Arch Linux Parity\n");
    out.push_str(&format!("MODULES=({})\n", modules.join(" ")));
    out.push_str("BINARIES=()\n");
    out.push_str("FILES=()\n");
    out.push_str(&format!("HOOKS=({})\n", hooks.join(" ")));
    out
}

/// Generates Arch Linux mkinitcpio preset file (/etc/mkinitcpio.d/linux.preset)
pub fn generate_mkinitcpio_preset(preset_name: &str, kernel: &str, initramfs: &str) -> String {
    let mut out = String::from("# mkinitcpio preset file for SigmaOS Arch Linux kernel\n");
    out.push_str(&format!("# Preset: {}\n", preset_name));
    out.push_str(&format!("ALL_kver=\"{}\"\n", kernel));
    out.push_str("PRESETS=('default' 'fallback')\n\n");
    out.push_str(&format!("default_config=\"/etc/mkinitcpio.conf\"\n"));
    out.push_str(&format!("default_image=\"{}\"\n\n", initramfs));
    out.push_str(&format!("fallback_config=\"/etc/mkinitcpio.conf\"\n"));
    out.push_str(&format!("fallback_image=\"{}-fallback.img\"\n", initramfs.trim_end_matches(".img")));
    out
}

/// Represents Arch Linux system localization and identity configuration files
#[derive(Debug, Clone)]
pub struct ArchSystemIdentity {
    pub hostname: String,
    pub lang: String,
    pub keymap: String,
    pub font: String,
    pub enabled_locales: Vec<String>,
}

/// Represents parsed PKGBUILD recipe metadata for ABS & makepkg execution
#[derive(Debug, Clone, Default)]
pub struct PkgbuildRecipe {
    pub pkgname: String,
    pub pkgver: String,
    pub pkgrel: String,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub source: Vec<String>,
    pub sha256sums: Vec<String>,
}

impl PkgbuildRecipe {
    /// Parses a PKGBUILD string into a PkgbuildRecipe struct
    pub fn parse(contents: &str) -> Self {
        let mut recipe = PkgbuildRecipe::default();
        for line in contents.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }
            if let Some((key, val)) = trimmed.split_once('=') {
                let key = key.trim();
                let clean_val = val.trim().trim_matches('\'').trim_matches('"').trim_matches('(').trim_matches(')');
                match key {
                    "pkgname" => recipe.pkgname = clean_val.to_string(),
                    "pkgver" => recipe.pkgver = clean_val.to_string(),
                    "pkgrel" => recipe.pkgrel = clean_val.to_string(),
                    "pkgdesc" => recipe.pkgdesc = clean_val.to_string(),
                    "url" => recipe.url = clean_val.to_string(),
                    "arch" => recipe.arch = clean_val.split_whitespace().map(|s| s.trim_matches('\'').trim_matches('"').to_string()).collect(),
                    "license" => recipe.license = clean_val.split_whitespace().map(|s| s.trim_matches('\'').trim_matches('"').to_string()).collect(),
                    "depends" => recipe.depends = clean_val.split_whitespace().map(|s| s.trim_matches('\'').trim_matches('"').to_string()).collect(),
                    "makedepends" => recipe.makedepends = clean_val.split_whitespace().map(|s| s.trim_matches('\'').trim_matches('"').to_string()).collect(),
                    "source" => recipe.source = clean_val.split_whitespace().map(|s| s.trim_matches('\'').trim_matches('"').to_string()).collect(),
                    "sha256sums" => recipe.sha256sums = clean_val.split_whitespace().map(|s| s.trim_matches('\'').trim_matches('"').to_string()).collect(),
                    _ => {}
                }
            }
        }
        recipe
    }

    /// Generates Arch Linux `.SRCINFO` metadata from PKGBUILD recipe
    pub fn generate_srcinfo(&self) -> String {
        let mut out = String::from("pkgbase = ") + &self.pkgname + "\n";
        out.push_str(&format!("\tpkgdesc = {}\n", self.pkgdesc));
        out.push_str(&format!("\tpkgver = {}\n", self.pkgver));
        out.push_str(&format!("\tpkgrel = {}\n", self.pkgrel));
        if !self.url.is_empty() {
            out.push_str(&format!("\turl = {}\n", self.url));
        }
        for a in &self.arch {
            out.push_str(&format!("\tarch = {}\n", a));
        }
        for l in &self.license {
            out.push_str(&format!("\tlicense = {}\n", l));
        }
        for d in &self.depends {
            out.push_str(&format!("\tdepends = {}\n", d));
        }
        for md in &self.makedepends {
            out.push_str(&format!("\tmakedepends = {}\n", md));
        }
        for s in &self.source {
            out.push_str(&format!("\tsource = {}\n", s));
        }
        for sum in &self.sha256sums {
            out.push_str(&format!("\tsha256sums = {}\n", sum));
        }
        out.push_str(&format!("\npkgname = {}\n", self.pkgname));
        out
    }
}

/// Namcap PKGBUILD Linter checking for required Arch packaging conventions
#[derive(Debug, Clone, Default)]
pub struct NamcapLinter;

impl NamcapLinter {
    pub fn lint_pkgbuild(recipe: &PkgbuildRecipe) -> Vec<String> {
        let mut warnings = Vec::new();
        if recipe.pkgname.is_empty() {
            warnings.push("E: PKGBUILD missing 'pkgname'".to_string());
        }
        if recipe.pkgver.is_empty() {
            warnings.push("E: PKGBUILD missing 'pkgver'".to_string());
        }
        if recipe.arch.is_empty() {
            warnings.push("W: PKGBUILD missing 'arch' architecture array".to_string());
        }
        if recipe.license.is_empty() {
            warnings.push("W: PKGBUILD missing 'license' tag".to_string());
        }
        if recipe.pkgdesc.is_empty() {
            warnings.push("W: PKGBUILD description 'pkgdesc' is empty".to_string());
        }
        warnings
    }
}

impl ArchSystemIdentity {
    pub fn new(hostname: &str, lang: &str, keymap: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            lang: lang.to_string(),
            keymap: keymap.to_string(),
            font: "eisfnt".to_string(),
            enabled_locales: vec!["en_US.UTF-8 UTF-8".to_string()],
        }
    }

    pub fn generate_hostname_file(&self) -> String {
        format!("{}\n", self.hostname)
    }

    pub fn generate_locale_conf(&self) -> String {
        format!("LANG={}\n", self.lang)
    }

    pub fn generate_vconsole_conf(&self) -> String {
        format!("KEYMAP={}\nFONT={}\n", self.keymap, self.font)
    }

    pub fn generate_locale_gen(&self) -> String {
        let mut out = String::from("# /etc/locale.gen generated by Arch System Identity\n");
        for loc in &self.enabled_locales {
            out.push_str(&format!("{}\n", loc));
        }
        out
    }
}

impl Default for PacmanSystem {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("Σ SIGMAOS ARCH LINUX PACMAN & YAY PARITY SUITE");
    let mut sys = PacmanSystem::new();
    sys.pacman_sync_db().unwrap();
    sys.pacman_install("neovim").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacman_sync_and_search() {
        let mut sys = PacmanSystem::new();
        assert!(!sys.db_synced);

        let count = sys.pacman_sync_db().unwrap();
        assert_eq!(count, 4);
        assert!(sys.db_synced);

        let search_res = sys.pacman_search("kernel");
        assert_eq!(search_res.len(), 1);
        assert_eq!(search_res[0].name, "linux");
    }

    #[test]
    fn test_pacman_install_and_remove() {
        let mut sys = PacmanSystem::new();
        assert!(sys.pacman_install("neovim").is_err()); // Needs sync first

        sys.pacman_sync_db().unwrap();
        let order = sys.pacman_install("neovim").unwrap();

        // Dependencies: neovim -> [glibc, libuv] (libuv -> glibc)
        assert_eq!(order, vec!["glibc".to_string(), "libuv".to_string(), "neovim".to_string()]);
        assert!(sys.installed_packages.contains_key("neovim"));

        // Removal
        assert!(sys.pacman_remove("neovim").is_ok());
        assert!(!sys.installed_packages.contains_key("neovim"));
    }

    #[test]
    fn test_yay_aur_helper() {
        let mut sys = PacmanSystem::new();
        assert!(sys.yay_aur_install("google-chrome").is_ok());
        assert!(sys.installed_packages.contains_key("google-chrome"));
        assert_eq!(sys.installed_packages.get("google-chrome").unwrap().repository, "aur");
    }

    #[test]
    fn test_genfstab_and_mkinitcpio() {
        let mounts = vec![
            ArchMountPoint {
                device: "/dev/sda2".to_string(),
                mount_point: "/".to_string(),
                fstype: "ext4".to_string(),
                options: "rw,relatime".to_string(),
                dump: 0,
                pass: 1,
                uuid: "abc-123".to_string(),
            },
        ];
        let fstab = generate_genfstab(&mounts);
        assert!(fstab.contains("UUID=abc-123"));
        assert!(fstab.contains("ext4"));

        let conf = generate_mkinitcpio_conf(&["base", "udev", "autodetect", "block", "filesystems"], &["ext4"]);
        assert!(conf.contains("HOOKS=(base udev autodetect block filesystems)"));
        assert!(conf.contains("MODULES=(ext4)"));

        let preset = generate_mkinitcpio_preset("linux", "6.5.9-arch1", "/boot/initramfs-linux.img");
        assert!(preset.contains("ALL_kver=\"6.5.9-arch1\""));
        assert!(preset.contains("default_image=\"/boot/initramfs-linux.img\""));
    }

    #[test]
    fn test_arch_system_identity() {
        let identity = ArchSystemIdentity::new("sigma-arch", "en_US.UTF-8", "us");
        assert_eq!(identity.generate_hostname_file(), "sigma-arch\n");
        assert_eq!(identity.generate_locale_conf(), "LANG=en_US.UTF-8\n");
        assert_eq!(identity.generate_vconsole_conf(), "KEYMAP=us\nFONT=eisfnt\n");
        assert!(identity.generate_locale_gen().contains("en_US.UTF-8 UTF-8"));
    }

    #[test]
    fn test_pkgbuild_parser_srcinfo_and_namcap() {
        let pkgbuild = r#"
pkgname=ripgrep
pkgver=13.0.0
pkgrel=1
pkgdesc="A search tool that combines the usability of ag with the raw speed of grep"
arch=('x86_64')
url="https://github.com/BurntSushi/ripgrep"
license=('MIT' 'Unlicense')
depends=('pcre2')
"#;
        let recipe = PkgbuildRecipe::parse(pkgbuild);
        assert_eq!(recipe.pkgname, "ripgrep");
        assert_eq!(recipe.pkgver, "13.0.0");
        assert_eq!(recipe.depends, vec!["pcre2"]);

        let srcinfo = recipe.generate_srcinfo();
        assert!(srcinfo.contains("pkgbase = ripgrep"));
        assert!(srcinfo.contains("depends = pcre2"));

        let warnings = NamcapLinter::lint_pkgbuild(&recipe);
        assert!(warnings.is_empty(), "Expected no namcap lint warnings, got: {:?}", warnings);
    }

    #[test]
    fn test_pacman_queries_and_locking() {
        let mut sys = PacmanSystem::new();
        sys.pacman_sync_db().unwrap();
        sys.pacman_install("neovim").unwrap();

        let installed = sys.pacman_query_installed();
        assert_eq!(installed.len(), 3); // neovim, glibc, libuv

        let explicit = sys.pacman_query_explicit();
        assert_eq!(explicit.len(), 1);
        assert_eq!(explicit[0].name, "neovim");

        sys.package_owners.insert("/usr/bin/nvim".to_string(), "neovim".to_string());
        let owner_res = sys.pacman_query_owner("/usr/bin/nvim").unwrap();
        assert!(owner_res.contains("is owned by neovim"));

        // Lock test
        assert!(sys.lock_db().is_ok());
        assert!(sys.lock_db().is_err()); // Already locked
        assert!(sys.pacman_remove("neovim").is_err()); // Cannot remove when locked
        sys.unlock_db();
        assert!(sys.pacman_remove("neovim").is_ok());
    }

    #[test]
    fn test_reflector_mirrorlist() {
        let reflector = ReflectorMirrorEngine::new();
        let mirrorlist = reflector.generate_ranked_mirrorlist(Some("US"), 2);
        assert!(mirrorlist.contains("kernel.org"));
        assert!(mirrorlist.contains("rackspace.com"));
        assert!(!mirrorlist.contains("honkgong.info")); // DE filtered out
    }

    #[test]
    fn test_arch_chroot_and_pacman_keyring() {
        let mut chroot = ArchChrootEnvironment::new("/mnt");
        let vfs = chroot.mount_virtual_filesystems();
        assert_eq!(vfs.len(), 5);
        assert!(vfs.contains(&"/mnt/proc".to_string()));
        let cmd = chroot.generate_chroot_command("pacman -Syu");
        assert_eq!(cmd, "chroot /mnt /bin/bash -c \"pacman -Syu\"");

        let mut keyring = PacmanKeyringManager::new();
        assert!(keyring.init_keyring().is_ok());
        let total_keys = keyring.populate_arch_keyring();
        assert_eq!(total_keys, 3);
        assert!(keyring.verify_package_signature("/var/cache/pacman/pkg/linux.tar.zst"));
    }

    #[test]
    fn test_checkupdates_updpkgsums_and_makepkg() {
        let mut sys = PacmanSystem::new();
        sys.installed_packages.insert(
            "linux".to_string(),
            ArchPackageRecord {
                name: "linux".to_string(),
                version: "6.5.0-1".to_string(),
                repository: "core".to_string(),
                dependencies: vec![],
                description: "Outdated Linux kernel".to_string(),
            },
        );
        sys.sync_database.insert(
            "linux".to_string(),
            ArchPackageRecord {
                name: "linux".to_string(),
                version: "6.5.9-1".to_string(),
                repository: "core".to_string(),
                dependencies: vec![],
                description: "Updated Linux kernel".to_string(),
            },
        );

        let pending = CheckupdatesEngine::check_pending_upgrades(&sys);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0], ("linux".to_string(), "6.5.0-1".to_string(), "6.5.9-1".to_string()));

        let mut recipe = PkgbuildRecipe {
            pkgname: "htop".to_string(),
            pkgver: "3.2.2".to_string(),
            pkgrel: "1".to_string(),
            pkgdesc: "Interactive process viewer".to_string(),
            arch: vec!["x86_64".to_string()],
            url: "https://htop.dev".to_string(),
            license: vec!["GPL2".to_string()],
            depends: vec!["ncurses".to_string()],
            makedepends: vec![],
            source: vec!["https://github.com/htop-dev/htop/archive/3.2.2.tar.gz".to_string()],
            sha256sums: vec![],
        };

        UpdpkgsumsEngine::update_recipe_checksums(&mut recipe);
        assert_eq!(recipe.sha256sums.len(), 1);
        assert_eq!(recipe.sha256sums[0].len(), 64);

        let executor = MakepkgExecutor::new(recipe);
        let tarball = executor.build_package_tarball().unwrap();
        assert_eq!(tarball, "htop-3.2.2-1-x86_64.pkg.tar.zst");
    }
}
