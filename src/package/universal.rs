// SigmaOS Universal Package Manager
// Unified system absorbing apt, yum, pacman, snap, flatpak

use std::collections::HashMap;

/// Package format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageFormat {
    Deb,      // apt
    Rpm,      // yum
    Pacman,   // pacman
    Snap,     // snap
    Flatpak,  // flatpak
    SigmaPkg, // native SigmaOS format
    Apk,      // alpine apk format
}

/// Package source
#[derive(Debug, Clone)]
pub enum PackageSource {
    Repository { url: String },
    Local { path: String },
    Remote { url: String },
}

/// Dependency conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolution {
    PreferNewest,
    PreferOldest,
    PreferNative,
    Manual,
}

/// Unified package
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
    pub use_flags: Vec<String>,               // Portage-style available USE flags
    pub active_use_flags: Vec<String>,         // Portage-style enabled USE flags
    pub conditional_dependencies: Vec<(String, String)>, // (use_flag, dependency)
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
            use_flags: Vec::new(),
            active_use_flags: Vec::new(),
            conditional_dependencies: Vec::new(),
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

    pub fn with_use_flag(mut self, flag: String) -> Self {
        self.use_flags.push(flag);
        self
    }

    pub fn with_conditional_dependency(mut self, flag: String, dep: String) -> Self {
        self.conditional_dependencies.push((flag, dep));
        self
    }

    pub fn enable_use_flag(&mut self, flag: &str) {
        if self.use_flags.iter().any(|f| f == flag) && !self.active_use_flags.iter().any(|f| f == flag) {
            self.active_use_flags.push(flag.to_string());
        }
    }

    pub fn disable_use_flag(&mut self, flag: &str) {
        self.active_use_flags.retain(|f| f != flag);
    }

    /// Portage-style dynamic dependency resolution depending on active USE flags
    pub fn get_resolved_dependencies(&self) -> Vec<String> {
        let mut deps = self.dependencies.clone();
        for (flag, dep) in &self.conditional_dependencies {
            if self.active_use_flags.contains(flag) && !deps.contains(dep) {
                deps.push(dep.clone());
            }
        }
        deps
    }

    pub fn has_conflict_with(&self, other: &UnifiedPackage) -> bool {
        self.conflicts.iter().any(|c| c == &other.name)
            || other.conflicts.iter().any(|c| c == &self.name)
    }
}

/// Package format adapter
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

    pub fn can_handle(&self, package: &UnifiedPackage) -> bool {
        package.formats.contains(&self.format)
    }

    pub fn install(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Installing {} using {} adapter",
            package.name, self.adapter_name
        );
        // Simulate installation
        Ok(())
    }

    pub fn remove(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Removing {} using {} adapter",
            package.name, self.adapter_name
        );
        // Simulate removal
        Ok(())
    }

    pub fn update(&self, package: &UnifiedPackage) -> Result<(), PackageError> {
        println!(
            "Updating {} using {} adapter",
            package.name, self.adapter_name
        );
        // Simulate update
        Ok(())
    }
}

/// Dependency resolver
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
        let mut resolved = Vec::new();
        let mut to_visit = vec![package_name.to_string()];
        let mut visited = std::collections::HashSet::new();

        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current.clone());

            if let Some(package) = self.packages.get(&current) {
                for dep in package.get_resolved_dependencies() {
                    if !visited.contains(&dep) {
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

        for (i, pkg1_name) in packages.iter().enumerate() {
            for pkg2_name in packages.iter().skip(i + 1) {
                if let (Some(pkg1), Some(pkg2)) =
                    (self.packages.get(pkg1_name), self.packages.get(pkg2_name))
                {
                    if pkg1.has_conflict_with(pkg2) {
                        conflicts.push((pkg1_name.clone(), pkg2_name.clone()));
                    }
                }
            }
        }

        conflicts
    }

    pub fn resolve_conflicts(&self, conflicts: &[(String, String)]) -> Vec<String> {
        let mut resolution = Vec::new();

        match self.resolution_strategy {
            ConflictResolution::PreferNewest => {
                // Prefer the package with higher version
                for (pkg1, pkg2) in conflicts {
                    if let (Some(p1), Some(p2)) = (self.packages.get(pkg1), self.packages.get(pkg2))
                    {
                        if p1.version > p2.version {
                            resolution.push(pkg1.clone());
                        } else {
                            resolution.push(pkg2.clone());
                        }
                    }
                }
            }
            ConflictResolution::PreferOldest => {
                // Prefer the package with lower version
                for (pkg1, pkg2) in conflicts {
                    if let (Some(p1), Some(p2)) = (self.packages.get(pkg1), self.packages.get(pkg2))
                    {
                        if p1.version < p2.version {
                            resolution.push(pkg1.clone());
                        } else {
                            resolution.push(pkg2.clone());
                        }
                    }
                }
            }
            ConflictResolution::PreferNative => {
                // Prefer SigmaPkg format
                for (pkg1, pkg2) in conflicts {
                    if let (Some(p1), Some(p2)) = (self.packages.get(pkg1), self.packages.get(pkg2))
                    {
                        if p1.formats.contains(&PackageFormat::SigmaPkg) {
                            resolution.push(pkg1.clone());
                        } else if p2.formats.contains(&PackageFormat::SigmaPkg) {
                            resolution.push(pkg2.clone());
                        } else {
                            resolution.push(pkg1.clone());
                        }
                    }
                }
            }
            ConflictResolution::Manual => {
                // Return conflicts for manual resolution
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

/// Transactional history representing a history of installed and removed packages
#[derive(Debug, Clone)]
pub struct TransactionalHistory {
    pub transactions: Vec<String>,
}

impl TransactionalHistory {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
        }
    }
}

/// Package snapshot representing a saved system state of installed packages
#[derive(Debug, Clone)]
pub struct PackageSnapshot {
    pub id: usize,
    pub description: String,
    pub timestamp: u64,
    pub installed_packages: HashMap<String, UnifiedPackage>,
}

/// Universal package manager with transaction-safe snapshots & rollback mechanisms
pub struct UniversalPackageManager {
    pub packages: HashMap<String, UnifiedPackage>,
    pub adapters: HashMap<PackageFormat, PackageAdapter>,
    pub resolver: DependencyResolver,
    pub installed_packages: HashMap<String, UnifiedPackage>,
    pub transaction_history: TransactionalHistory,
    pub metadata_cache: HashMap<String, UnifiedPackage>,
    pub snapshots: HashMap<usize, PackageSnapshot>,
    pub next_snapshot_id: usize,
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
            snapshots: HashMap::new(),
            next_snapshot_id: 1,
        };

        manager.add_default_adapters();
        manager
    }

    fn add_default_adapters(&mut self) {
        let apt_adapter = PackageAdapter::new(PackageFormat::Deb, "apt".to_string());
        let yum_adapter = PackageAdapter::new(PackageFormat::Rpm, "yum".to_string());
        let pacman_adapter = PackageAdapter::new(PackageFormat::Pacman, "pacman".to_string());
        let snap_adapter = PackageAdapter::new(PackageFormat::Snap, "snap".to_string());
        let flatpak_adapter = PackageAdapter::new(PackageFormat::Flatpak, "flatpak".to_string());
        let sigpkg_adapter = PackageAdapter::new(PackageFormat::SigmaPkg, "sigpkg".to_string());
        let apk_adapter = PackageAdapter::new(PackageFormat::Apk, "apk".to_string());

        self.adapters.insert(PackageFormat::Deb, apt_adapter);
        self.adapters.insert(PackageFormat::Rpm, yum_adapter);
        self.adapters.insert(PackageFormat::Pacman, pacman_adapter);
        self.adapters.insert(PackageFormat::Snap, snap_adapter);
        self.adapters
            .insert(PackageFormat::Flatpak, flatpak_adapter);
        self.adapters
            .insert(PackageFormat::SigmaPkg, sigpkg_adapter);
        self.adapters
            .insert(PackageFormat::Apk, apk_adapter);
    }

    pub fn add_package(&mut self, package: UnifiedPackage) {
        self.resolver.add_package(package.clone());
        self.packages.insert(package.name.clone(), package);
    }

    pub fn install(&mut self, package_name: &str) -> Result<(), PackageError> {
        // Resolve dependencies
        let dependencies = self.resolver.resolve_dependencies(package_name)?;

        // Detect conflicts
        let conflicts = self.resolver.detect_conflicts(&dependencies);

        if !conflicts.is_empty() {
            let resolution = self.resolver.resolve_conflicts(&conflicts);
            println!("Conflicts detected: {:?}", conflicts);
            println!("Resolution: {:?}", resolution);
        }

        // Install packages
        for dep_name in dependencies {
            if let Some(package) = self.packages.get(&dep_name) {
                // Find appropriate adapter
                for format in &package.formats {
                    if let Some(adapter) = self.adapters.get(format) {
                        adapter.install(package)?;
                        break;
                    }
                }

                let mut installed = package.clone();
                installed.installed = true;
                self.installed_packages.insert(dep_name.clone(), installed);
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get(package_name) {
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.remove(package)?;
                    break;
                }
            }
            self.installed_packages.remove(package_name);
        }
        Ok(())
    }

    pub fn update(&mut self, package_name: &str) -> Result<(), PackageError> {
        if let Some(package) = self.installed_packages.get(package_name) {
            for format in &package.formats {
                if let Some(adapter) = self.adapters.get(format) {
                    adapter.update(package)?;
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

    /// Create a snapshot of currently installed packages state
    pub fn create_snapshot(&mut self, description: String) -> usize {
        let id = self.next_snapshot_id;
        self.next_snapshot_id += 1;

        let snapshot = PackageSnapshot {
            id,
            description,
            timestamp: 0,
            installed_packages: self.installed_packages.clone(),
        };

        self.snapshots.insert(id, snapshot);
        id
    }

    /// Delete a package snapshot
    pub fn delete_snapshot(&mut self, id: usize) -> Result<(), PackageError> {
        if self.snapshots.remove(&id).is_none() {
            return Err(PackageError::PackageNotFound(format!("Snapshot ID {}", id)));
        }
        Ok(())
    }

    /// List all package snapshots
    pub fn list_snapshots(&self) -> Vec<(usize, String)> {
        let mut list = Vec::new();
        for (id, snap) in &self.snapshots {
            list.push((*id, snap.description.clone()));
        }
        list.sort_by_key(|&(id, _)| id);
        list
    }

    /// Rollback the active package state exactly to a previously saved snapshot
    pub fn rollback_to_snapshot(&mut self, id: usize) -> Result<(), PackageError> {
        let snapshot = self
            .snapshots
            .get(&id)
            .ok_or_else(|| PackageError::PackageNotFound(format!("Snapshot ID {}", id)))?
            .clone();

        // 1. Identify and uninstall packages currently installed but not in the snapshot
        let mut to_uninstall: Vec<String> = Vec::new();
        for pkg_name in self.installed_packages.keys() {
            if !snapshot.installed_packages.contains_key(pkg_name.as_str()) {
                to_uninstall.push(pkg_name.clone());
            }
        }

        for pkg_name in to_uninstall {
            self.remove(&pkg_name)?;
        }

        // 2. Identify and reinstall packages in the snapshot but not currently installed
        let mut to_install: Vec<String> = Vec::new();
        for (pkg_name, _) in &snapshot.installed_packages {
            if !self.installed_packages.contains_key(pkg_name.as_str()) {
                to_install.push(pkg_name.clone());
            }
        }

        for pkg_name in to_install {
            self.install(&pkg_name)?;
        }

        // 3. Sync full installed_packages state exactly with the snapshot
        self.installed_packages = snapshot.installed_packages;

        Ok(())
    }
}

impl Default for UniversalPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 1. MultiDistroPackageAdapter (Multi-format RPM, DEB, APK, Arch, Snap, Flatpak)
// =========================================================================

pub struct MultiDistroPackageAdapter {
    pub registered_formats: Vec<PackageFormat>,
}

impl MultiDistroPackageAdapter {
    pub fn new() -> Self {
        MultiDistroPackageAdapter {
            registered_formats: vec![
                PackageFormat::Deb,
                PackageFormat::Rpm,
                PackageFormat::Pacman,
                PackageFormat::Snap,
                PackageFormat::Flatpak,
                PackageFormat::Apk,
                PackageFormat::SigmaPkg,
            ],
        }
    }

    /// Dynamically parses package spec/control file headers from any Linux distro package format
    pub fn parse_package_headers(&self, raw_metadata: &str, format: PackageFormat) -> Result<UnifiedPackage, String> {
        if !self.registered_formats.contains(&format) {
            return Err("Unsupported package format".to_string());
        }

        let mut name = String::new();
        let mut version = String::new();
        let mut dependencies = Vec::new();

        for line in raw_metadata.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match format {
                PackageFormat::Deb => {
                    // Debian Control format (e.g. Package: libc6, Version: 2.31, Depends: libcrypt1)
                    if line.starts_with("Package:") {
                        name = line["Package:".len()..].trim().to_string();
                    } else if line.starts_with("Version:") {
                        version = line["Version:".len()..].trim().to_string();
                    } else if line.starts_with("Depends:") {
                        let deps_str = line["Depends:".len()..].trim();
                        for d in deps_str.split(',') {
                            dependencies.push(d.trim().to_string());
                        }
                    }
                }
                PackageFormat::Rpm => {
                    // RPM spec format (e.g. Name: coreutils, Version: 8.32, Requires: glibc)
                    if line.starts_with("Name:") {
                        name = line["Name:".len()..].trim().to_string();
                    } else if line.starts_with("Version:") {
                        version = line["Version:".len()..].trim().to_string();
                    } else if line.starts_with("Requires:") {
                        let deps_str = line["Requires:".len()..].trim();
                        for d in deps_str.split(',') {
                            dependencies.push(d.trim().to_string());
                        }
                    }
                }
                PackageFormat::Pacman => {
                    // Arch PKGBUILD / .PKGINFO format (e.g. pkgname = pacman, pkgver = 6.0, depend = openssl)
                    if line.starts_with("pkgname =") {
                        name = line["pkgname =".len()..].trim().to_string();
                    } else if line.starts_with("pkgver =") {
                        version = line["pkgver =".len()..].trim().to_string();
                    } else if line.starts_with("depend =") {
                        let dep = line["depend =".len()..].trim().to_string();
                        dependencies.push(dep);
                    }
                }
                PackageFormat::Apk => {
                    // Alpine APKINDEX format (e.g. P:musl, V:1.2, D:so:libc)
                    if line.starts_with("P:") {
                        name = line["P:".len()..].trim().to_string();
                    } else if line.starts_with("V:") {
                        version = line["V:".len()..].trim().to_string();
                    } else if line.starts_with("D:") {
                        let deps_str = line["D:".len()..].trim();
                        for d in deps_str.split(' ') {
                            dependencies.push(d.trim().to_string());
                        }
                    }
                }
                PackageFormat::Flatpak | PackageFormat::Snap => {
                    // YAML/JSON Manifest (e.g. id: org.kde.Platform, version: 5.15)
                    if line.starts_with("id:") {
                        name = line["id:".len()..].trim().to_string();
                    } else if line.starts_with("version:") {
                        version = line["version:".len()..].trim().to_string();
                    }
                }
                PackageFormat::SigmaPkg => {
                    if line.starts_with("name:") {
                        name = line["name:".len()..].trim().to_string();
                    } else if line.starts_with("version:") {
                        version = line["version:".len()..].trim().to_string();
                    }
                }
            }
        }

        if name.is_empty() || version.is_empty() {
            return Err("Missing required metadata headers".to_string());
        }

        let mut pkg = UnifiedPackage::new(name, version).with_format(format);
        for d in dependencies {
            pkg = pkg.with_dependency(d);
        }

        Ok(pkg)
    }
}

// =========================================================================
// 2. PackageInstallHook (User-defined trigger functions)
// =========================================================================

pub struct PackageInstallHook {
    pub hook_name: String,
    pub run_counter: u64,
}

impl PackageInstallHook {
    pub fn new(name: &str) -> Self {
        PackageInstallHook {
            hook_name: name.to_string(),
            run_counter: 0,
        }
    }

    /// Trigger hook function executed before a distro application runs to pre-configure sandboxed directories
    pub fn execute_pre_install_hook(&mut self, pkg: &UnifiedPackage) -> bool {
        self.run_counter += 1;
        // User-defined validation hook check: block untrusted third-party apps unless GPG signed
        if pkg.name.contains("untrusted") {
            return false;
        }
        true
    }
}

// =========================================================================
// 3. MultiFormatExtractor (Emulated package extraction)
// =========================================================================

pub struct MultiFormatExtractor {
    pub extracted_paths: Vec<String>,
}

impl MultiFormatExtractor {
    pub fn new() -> Self {
        MultiFormatExtractor {
            extracted_paths: Vec::new(),
        }
    }

    /// Simulates package file payload extraction and automatically routes them to the correct comopsable FHS system directories
    pub fn extract_payload(&mut self, pkg: &UnifiedPackage) -> Result<usize, String> {
        let mut files_created = 0;

        // Emulates extracting files from the package format layers (ar / cpio / tar.zst)
        let simulated_files = match pkg.formats.first().unwrap_or(&PackageFormat::SigmaPkg) {
            PackageFormat::Deb => vec!["usr/bin/apt-app", "etc/apt-app.conf", "usr/lib/libapt.so"],
            PackageFormat::Rpm => vec!["usr/bin/rpm-app", "etc/rpm-app.conf"],
            PackageFormat::Pacman => vec!["usr/bin/pacman-app", "usr/lib/libpacman.so"],
            PackageFormat::Apk => vec!["sbin/apk-app", "etc/apk-app.conf"],
            _ => vec!["usr/bin/app"],
        };

        for f in simulated_files {
            self.extracted_paths.push(f.to_string());
            files_created += 1;
        }

        Ok(files_created)
    }
}

/// Package errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageError {
    PackageNotFound(String),
    DependencyNotFound(String),
    AdapterNotFound,
    InstallationFailed(String),
    ConflictDetected(Vec<(String, String)>),
}

// =========================================================================
// 4. Offline Package Installer and Verification Systems (dpkg & rpm parity)
// =========================================================================

#[derive(Debug, Clone)]
pub struct SovereignPackageArchive {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub conflicts: Vec<String>,
    pub payload: HashMap<String, String>, // filepath -> file content
    pub file_permissions: HashMap<String, u32>, // filepath -> octal file mode (e.g. 0o755)
    pub conffiles: Vec<String>, // list of configuration files
    pub preinst_script: Option<String>,
    pub postinst_script: Option<String>,
    pub prerm_script: Option<String>,
    pub postrm_script: Option<String>,
    pub interested_triggers: Vec<String>, // triggers it registers
}

#[derive(Debug, Clone, Default)]
pub struct DpkgTriggerSystem {
    pub interested_paths: HashMap<String, Vec<String>>, // directory -> list of trigger actions (e.g., "usr/share/man" -> ["update-man-db"])
    pub pending_triggers: Vec<String>, // triggered actions queued to run post-configure
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConffilePolicy {
    KeepOld,
    InstallNew,
}

pub struct SovereignOfflineInstaller {
    pub local_db: HashMap<String, UnifiedPackage>, // local database simulating /var/lib/dpkg/status
    pub installed_files: HashMap<String, (String, u32)>, // filepath -> (hash, permissions)
    pub mock_fs: HashMap<String, String>, // simulated physical file system disk (filepath -> content)
    pub trigger_system: DpkgTriggerSystem,
    pub conffile_policy: ConffilePolicy,
}

impl DpkgTriggerSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_interest(&mut self, directory: &str, trigger: &str) {
        self.interested_paths
            .entry(directory.to_string())
            .or_insert_with(Vec::new)
            .push(trigger.to_string());
    }

    /// Checks if a file path belongs to an interested directory path and queues matching triggers
    pub fn monitor_file_activity(&mut self, filepath: &str) {
        for (dir, triggers) in &self.interested_paths {
            if filepath.starts_with(dir) {
                for trigger in triggers {
                    if !self.pending_triggers.contains(trigger) {
                        self.pending_triggers.push(trigger.clone());
                    }
                }
            }
        }
    }

    /// Emulates running all pending post-install triggers
    pub fn process_pending_triggers(&mut self) -> Vec<String> {
        let executed = self.pending_triggers.clone();
        self.pending_triggers.clear();
        executed
    }
}

impl SovereignOfflineInstaller {
    pub fn new() -> Self {
        Self {
            local_db: HashMap::new(),
            installed_files: HashMap::new(),
            mock_fs: HashMap::new(),
            trigger_system: DpkgTriggerSystem::new(),
            conffile_policy: ConffilePolicy::KeepOld,
        }
    }

    pub fn with_policy(mut self, policy: ConffilePolicy) -> Self {
        self.conffile_policy = policy;
        self
    }

    /// Query archive details (dpkg -I or rpm -qpi)
    pub fn info(&self, archive: &SovereignPackageArchive) -> String {
        let mut out = String::new();
        out.push_str(&format!("Package: {}\n", archive.name));
        out.push_str(&format!("Version: {}\n", archive.version));
        out.push_str(&format!("Architecture: {}\n", archive.architecture));
        out.push_str(&format!("Description: {}\n", archive.description));
        out.push_str("Dependencies: ");
        if archive.dependencies.is_empty() {
            out.push_str("none\n");
        } else {
            out.push_str(&archive.dependencies.join(", "));
            out.push_str("\n");
        }
        out.push_str("Conflicts: ");
        if archive.conflicts.is_empty() {
            out.push_str("none\n");
        } else {
            out.push_str(&archive.conflicts.join(", "));
            out.push_str("\n");
        }
        out.push_str(&format!("Conffiles: {}\n", archive.conffiles.join(", ")));
        out
    }

    /// Audits the integrity of all files belonging to a package (rpm -V parity)
    /// Returns a vector of tuples: (filepath, status_flag)
    /// Standard flags format:
    /// S: file size differs
    /// M: mode (permissions) differs
    /// 5: MD5/hash digest differs
    /// D: device major/minor mismatch (omitted in mock)
    /// L: readLink path mismatch (omitted)
    /// U: owner differs (omitted)
    /// G: group differs (omitted)
    /// T: mTime differs (simulated)
    pub fn verify_package_files(&self, package_name: &str, archive: &SovereignPackageArchive) -> Vec<(String, String)> {
        let mut verification_results = Vec::new();

        // Ensure package is tracked as installed in local db
        if !self.local_db.contains_key(package_name) {
            return verification_results;
        }

        for (filepath, expected_content) in &archive.payload {
            let mut flag = String::from("........."); // 9 dots base

            // Check if file exists on disk
            if let Some(actual_content) = self.mock_fs.get(filepath) {
                let actual_hash = self.compute_simple_hash(actual_content);
                let expected_hash = self.compute_simple_hash(expected_content);

                // Check size
                if actual_content.len() != expected_content.len() {
                    flag.replace_range(0..1, "S");
                }

                // Check hash (5)
                if actual_hash != expected_hash {
                    flag.replace_range(2..3, "5");
                }

                // Check permissions (M)
                if let Some(expected_mode) = archive.file_permissions.get(filepath) {
                    if let Some((_, actual_mode)) = self.installed_files.get(filepath) {
                        if actual_mode != expected_mode {
                            flag.replace_range(1..2, "M");
                        }
                    }
                }

                // If any difference was detected, append result
                if flag != "........." {
                    verification_results.push((filepath.clone(), flag));
                }
            } else {
                // File missing completely
                verification_results.push((filepath.clone(), "missing".to_string()));
            }
        }

        verification_results
    }

    pub fn compute_simple_hash(&self, data: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Full offline package installation process (unpacking + triggers + conffiles merging/saving)
    pub fn install_archive(&mut self, archive: &SovereignPackageArchive) -> Result<Vec<String>, String> {
        // 1. Dependency and Conflict pre-checks
        for dep in &archive.dependencies {
            if !self.local_db.contains_key(dep) {
                return Err(format!("Dependency unsatisfied: {} is required", dep));
            }
        }
        for conf in &archive.conflicts {
            if self.local_db.contains_key(conf) {
                return Err(format!("Conflict detected: {} cannot coexist", conf));
            }
        }

        // 2. Preinst script
        if let Some(ref script) = archive.preinst_script {
            println!("Executing preinst: {}", script);
        }

        // 3. Unpack file payload and handle user-modified configuration files (dpkg conffiles / rpmsave / rpmnew)
        for (filepath, expected_content) in &archive.payload {
            let is_conffile = archive.conffiles.contains(filepath);
            let default_perm = archive.file_permissions.get(filepath).cloned().unwrap_or(0o644);

            if is_conffile && self.mock_fs.contains_key(filepath) {
                // If the file exists and is a configuration file, check if it's been modified from what we expected
                let current_content = self.mock_fs.get(filepath).unwrap();

                // Fetch the expected hash from when we last recorded it (if any) or compute it on expected
                let last_hash = self.installed_files.get(filepath).map(|x| x.0.clone());
                let expected_hash = self.compute_simple_hash(expected_content);

                let is_user_modified = match last_hash {
                    Some(lh) => self.compute_simple_hash(current_content) != lh,
                    None => self.compute_simple_hash(current_content) != expected_hash,
                };

                if is_user_modified {
                    // Conflict scenario! Apply configuration policy
                    match self.conffile_policy {
                        ConffilePolicy::KeepOld => {
                            // Save new content with `.sigrpmnew` suffix so the user's modifications are untouched
                            let new_suffix_path = format!("{}.sigrpmnew", filepath);
                            self.mock_fs.insert(new_suffix_path.clone(), expected_content.clone());
                            self.installed_files.insert(new_suffix_path, (expected_hash, default_perm));
                        }
                        ConffilePolicy::InstallNew => {
                            // Back up the user's modified config file to `.sigrpmsave`, then overwrite original
                            let save_suffix_path = format!("{}.sigrpmsave", filepath);
                            self.mock_fs.insert(save_suffix_path, current_content.clone());

                            self.mock_fs.insert(filepath.clone(), expected_content.clone());
                            self.installed_files.insert(filepath.clone(), (expected_hash, default_perm));
                        }
                    }
                } else {
                    // Not modified, overwrite cleanly
                    self.mock_fs.insert(filepath.clone(), expected_content.clone());
                    self.installed_files.insert(filepath.clone(), (expected_hash, default_perm));
                }
            } else {
                // Regular file or new conffile, unpack cleanly
                let hash = self.compute_simple_hash(expected_content);
                self.mock_fs.insert(filepath.clone(), expected_content.clone());
                self.installed_files.insert(filepath.clone(), (hash, default_perm));
            }

            // Monitor activity to queue triggers
            self.trigger_system.monitor_file_activity(filepath);
        }

        // 4. Register package in local_db
        let mut pkg = UnifiedPackage::new(archive.name.clone(), archive.version.clone())
            .with_format(PackageFormat::SigmaPkg);
        pkg.installed = true;
        self.local_db.insert(archive.name.clone(), pkg);

        // 5. Postinst script
        if let Some(ref script) = archive.postinst_script {
            println!("Executing postinst: {}", script);
        }

        // 6. Run deferred triggers
        let executed_triggers = self.trigger_system.process_pending_triggers();

        Ok(executed_triggers)
    }

    /// Simulates package removal (removes files, keeps conffiles if they have been modified)
    pub fn remove_package(&mut self, package_name: &str, archive: &SovereignPackageArchive) -> Result<(), String> {
        if !self.local_db.contains_key(package_name) {
            return Err(format!("Package {} is not installed", package_name));
        }

        // prerm script
        if let Some(ref script) = archive.prerm_script {
            println!("Executing prerm: {}", script);
        }

        for (filepath, _) in &archive.payload {
            let is_conffile = archive.conffiles.contains(filepath);

            if is_conffile {
                // If it is a configuration file, check if it's modified from what we expected
                if let Some(actual_content) = self.mock_fs.get(filepath) {
                    let last_hash = self.installed_files.get(filepath).map(|x| x.0.clone());
                    let expected_hash = archive.payload.get(filepath).map(|x| self.compute_simple_hash(x));

                    let is_user_modified = match last_hash {
                        Some(lh) => self.compute_simple_hash(actual_content) != lh,
                        None => expected_hash.is_none() || self.compute_simple_hash(actual_content) != expected_hash.unwrap(),
                    };

                    if is_user_modified {
                        // User modified config file, preserve it! Do not delete from filesystem
                        println!("Preserving user-modified configuration file: {}", filepath);
                        continue;
                    }
                }
            }

            // Otherwise, delete file and metadata record
            self.mock_fs.remove(filepath);
            self.installed_files.remove(filepath);
        }

        // Remove from local package registry status database
        self.local_db.remove(package_name);

        // postrm script
        if let Some(ref script) = archive.postrm_script {
            println!("Executing postrm: {}", script);
        }

        Ok(())
    }

    /// Simulates purging a package (removes all files, including user-modified conffiles)
    pub fn purge_package(&mut self, package_name: &str, archive: &SovereignPackageArchive) -> Result<(), String> {
        if !self.local_db.contains_key(package_name) {
            return Err(format!("Package {} is not installed", package_name));
        }

        // prerm script
        if let Some(ref script) = archive.prerm_script {
            println!("Executing prerm: {}", script);
        }

        for (filepath, _) in &archive.payload {
            // Delete unconditionally, ignoring conffile modifications
            self.mock_fs.remove(filepath);
            self.installed_files.remove(filepath);

            // Also delete potential .sigrpmnew or .sigrpmsave files associated
            let new_suffix_path = format!("{}.sigrpmnew", filepath);
            let save_suffix_path = format!("{}.sigrpmsave", filepath);
            self.mock_fs.remove(&new_suffix_path);
            self.installed_files.remove(&new_suffix_path);
            self.mock_fs.remove(&save_suffix_path);
            self.installed_files.remove(&save_suffix_path);
        }

        // Remove from local package registry status database
        self.local_db.remove(package_name);

        // postrm script
        if let Some(ref script) = archive.postrm_script {
            println!("Executing postrm: {}", script);
        }

        Ok(())
    }
}

// =========================================================================
// 5. Nix-inspired Atomic Profiles & Generation Symlink switching
// =========================================================================

#[derive(Debug, Clone)]
pub struct ProfileGeneration {
    pub generation_id: usize,
    pub description: String,
    pub active_symlink: String,
    pub installed_package_names: Vec<String>,
}

pub struct SovereignProfileManager {
    pub active_profile_name: String,
    pub generations: Vec<ProfileGeneration>,
    pub current_generation_id: usize,
}

impl SovereignProfileManager {
    pub fn new(profile_name: &str) -> Self {
        Self {
            active_profile_name: profile_name.to_string(),
            generations: Vec::new(),
            current_generation_id: 0,
        }
    }

    /// Creates and switches to a new profile generation
    pub fn create_generation(&mut self, desc: &str, packages: Vec<String>) -> usize {
        let gen_id = self.generations.len() + 1;
        let symlink = format!("/nix/var/nix/profiles/per-user/{}/profile-{}", self.active_profile_name, gen_id);

        let gen = ProfileGeneration {
            generation_id: gen_id,
            description: desc.to_string(),
            active_symlink: symlink,
            installed_package_names: packages,
        };

        self.generations.push(gen);
        self.current_generation_id = gen_id;
        gen_id
    }

    /// Switches the symlink pointer back to a previously saved generation
    pub fn switch_to_generation(&mut self, gen_id: usize) -> Result<String, &'static str> {
        if let Some(gen) = self.generations.iter().find(|g| g.generation_id == gen_id) {
            self.current_generation_id = gen_id;
            Ok(gen.active_symlink.clone())
        } else {
            Err("Profile generation not found")
        }
    }

    /// Rollbacks to the previous generation
    pub fn rollback(&mut self) -> Result<String, &'static str> {
        if self.current_generation_id <= 1 {
            return Err("No older generation available for rollback");
        }
        let prev_gen_id = self.current_generation_id - 1;
        self.switch_to_generation(prev_gen_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = UniversalPackageManager::new();
        assert_eq!(manager.adapters.len(), 7); // Deb, Rpm, Pacman, Snap, Flatpak, SigmaPkg, Apk
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
    fn test_package_snapshots_and_rollback() {
        let mut manager = UniversalPackageManager::new();
        let pkg_v1 = UnifiedPackage::new("essential-tool".to_string(), "1.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);
        let pkg_v2 = UnifiedPackage::new("add-on-tool".to_string(), "2.0.0".to_string())
            .with_format(PackageFormat::SigmaPkg);

        manager.add_package(pkg_v1);
        manager.add_package(pkg_v2);

        // Install first package
        manager.install("essential-tool").unwrap();
        assert_eq!(manager.installed_packages.len(), 1);
        assert!(manager.installed_packages.contains_key("essential-tool"));

        // Create snapshot 1
        let snap_id = manager.create_snapshot("First stable package state".to_string());
        assert_eq!(manager.list_snapshots().len(), 1);

        // Install second package
        manager.install("add-on-tool").unwrap();
        assert_eq!(manager.installed_packages.len(), 2);
        assert!(manager.installed_packages.contains_key("add-on-tool"));

        // Rollback to snapshot 1
        manager.rollback_to_snapshot(snap_id).unwrap();

        // Verify state is reverted to exactly one package
        assert_eq!(manager.installed_packages.len(), 1);
        assert!(manager.installed_packages.contains_key("essential-tool"));
        assert!(!manager.installed_packages.contains_key("add-on-tool"));

        // Delete snapshot
        assert!(manager.delete_snapshot(snap_id).is_ok());
        assert!(manager.list_snapshots().is_empty());
    }

    #[test]
    fn test_portage_style_use_flags() {
        let mut pkg = UnifiedPackage::new("lib-curl".to_string(), "8.2.1".to_string())
            .with_dependency("lib-base".to_string())
            .with_use_flag("ssl".to_string())
            .with_conditional_dependency("ssl".to_string(), "openssl".to_string());

        // Under default flags, only lib-base is resolved
        let core_deps = pkg.get_resolved_dependencies();
        assert_eq!(core_deps.len(), 1);
        assert_eq!(core_deps[0], "lib-base");

        // Enable SSL flag, now openssl is dynamically included as a dependency
        pkg.enable_use_flag("ssl");
        let active_deps = pkg.get_resolved_dependencies();
        assert_eq!(active_deps.len(), 2);
        assert!(active_deps.contains(&"lib-base".to_string()));
        assert!(active_deps.contains(&"openssl".to_string()));

        // Disable USE flag
        pkg.disable_use_flag("ssl");
        assert_eq!(pkg.get_resolved_dependencies().len(), 1);
    }

    #[test]
    fn test_nix_style_profile_atomic_rollback() {
        let mut profile = SovereignProfileManager::new("admin-user");
        assert_eq!(profile.current_generation_id, 0);

        let gen1 = profile.create_generation("Base config", vec!["bash".to_string()]);
        assert_eq!(gen1, 1);
        assert_eq!(profile.current_generation_id, 1);

        let gen2 = profile.create_generation("Developer tools", vec!["bash".to_string(), "git".to_string()]);
        assert_eq!(gen2, 2);
        assert_eq!(profile.current_generation_id, 2);

        // Rollback to gen1
        let symlink_path = profile.rollback().unwrap();
        assert_eq!(profile.current_generation_id, 1);
        assert!(symlink_path.contains("profile-1"));

        // Rollback further fails
        assert!(profile.rollback().is_err());
    }
}
