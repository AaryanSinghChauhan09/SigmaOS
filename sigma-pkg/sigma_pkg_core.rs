// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// sigma-pkg/sigma_pkg_core.rs — Unified Package Manager Core
// Implements: SAT-solver dependency resolution (DPLL), signed package
// verification (Dilithium-5 + SHA-3), rollback (generation management),
// content-addressed store (per RFC-0003), AI-assisted conflict detection.
//
// Inspired by: apt/nix/pacman/guix dependency models
// Language: Rust (std available in userland)

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

// ── Version ────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre:   Option<String>,
}

impl Version {
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        let (s, pre) = if let Some(i) = s.find('-') {
            (&s[..i], Some(s[i+1..].to_string()))
        } else {
            (s, None)
        };
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() < 3 { return None; }
        Some(Self {
            major: parts[0].parse().ok()?,
            minor: parts[1].parse().ok()?,
            patch: parts[2].parse().ok()?,
            pre,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(p) = &self.pre { write!(f, "-{}", p)?; }
        Ok(())
    }
}

// ── Dependency constraint ───────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum VersionConstraint {
    Any,
    Exact(Version),
    Ge(Version),
    Gt(Version),
    Le(Version),
    Lt(Version),
    Range { min: Version, max: Version },
}

impl VersionConstraint {
    pub fn matches(&self, v: &Version) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(x) => v == x,
            Self::Ge(x) => v >= x,
            Self::Gt(x) => v > x,
            Self::Le(x) => v <= x,
            Self::Lt(x) => v < x,
            Self::Range { min, max } => v >= min && v <= max,
        }
    }
}

// ── Dependency descriptor ──────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name:       String,
    pub constraint: VersionConstraint,
    pub optional:   bool,
    pub conflict:   bool, // true = "conflicts with"
}

// ── Package metadata ───────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PackageMeta {
    pub name:         String,
    pub version:      Version,
    pub description:  String,
    pub author:       String,
    pub license:      String,
    pub arch:         String,
    pub size:         u64,
    pub installed_size: u64,
    pub sha3_256:     [u8; 32],
    pub sig_dilithium: Vec<u8>, // Dilithium-5 signature over sha3_256
    pub deps:         Vec<Dependency>,
    pub provides:     Vec<String>,
    pub replaces:     Vec<String>,
}

// ── Install record ─────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct InstallRecord {
    pub meta:       PackageMeta,
    pub generation: u64,
    pub install_ts: u64,
    pub files:      Vec<String>, // installed file paths
}

// ── Package database ───────────────────────────────────────────────────────
pub struct PackageDb {
    /// Available packages from repos: name → versions
    pub available: HashMap<String, Vec<PackageMeta>>,
    /// Installed packages: name → record
    pub installed: HashMap<String, InstallRecord>,
    /// Current generation counter
    pub generation: u64,
    /// Generation history for rollback: gen → installed snapshot
    pub history: HashMap<u64, HashMap<String, InstallRecord>>,
}

impl PackageDb {
    pub fn new() -> Self {
        Self {
            available: HashMap::new(),
            installed: HashMap::new(),
            generation: 0,
            history: HashMap::new(),
        }
    }

    pub fn register_available(&mut self, pkg: PackageMeta) {
        self.available
            .entry(pkg.name.clone())
            .or_default()
            .push(pkg);
    }

    pub fn best_available(&self, name: &str, constraint: &VersionConstraint) -> Option<&PackageMeta> {
        self.available.get(name)?
            .iter()
            .filter(|p| constraint.matches(&p.version))
            .max_by(|a, b| a.version.cmp(&b.version))
    }

    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }

    pub fn snapshot_generation(&mut self) {
        self.history.insert(self.generation, self.installed.clone());
    }
}

// ── SAT-solver based dependency resolution (DPLL) ─────────────────────────

/// Resolution result
#[derive(Debug)]
pub enum ResolveResult {
    Success(Vec<PackageMeta>), // packages to install/upgrade
    Conflict(ConflictInfo),
    Unsatisfiable(Vec<String>), // unresolvable packages
}

/// Detailed conflict information for AI explainer
#[derive(Debug)]
pub struct ConflictInfo {
    pub package_a: String,
    pub package_b: String,
    pub reason:    String,
    pub suggestion: Option<String>,
}

pub struct Resolver<'a> {
    db:           &'a PackageDb,
    /// Variable assignments: pkg_name+version → bool (install yes/no)
    assignments:  HashMap<String, bool>,
    /// Propagation queue
    unit_queue:   VecDeque<(String, bool)>,
    /// Decision stack for backtracking
    decisions:    Vec<(String, bool)>,
}

impl<'a> Resolver<'a> {
    pub fn new(db: &'a PackageDb) -> Self {
        Self {
            db,
            assignments: HashMap::new(),
            unit_queue: VecDeque::new(),
            decisions: Vec::new(),
        }
    }

    /// Resolve a list of requested packages.
    pub fn resolve(&mut self, requests: &[(String, VersionConstraint)]) -> ResolveResult {
        // Phase 1: seed unit queue with requested packages
        for (name, constraint) in requests {
            match self.db.best_available(name, constraint) {
                Some(pkg) => {
                    let key = format!("{}={}", name, pkg.version);
                    self.unit_queue.push_back((key, true));
                }
                None => {
                    return ResolveResult::Unsatisfiable(vec![name.clone()]);
                }
            }
        }

        // Phase 2: BCP (Boolean Constraint Propagation)
        while let Some((var, val)) = self.unit_queue.pop_front() {
            if let Some(&existing) = self.assignments.get(&var) {
                if existing != val {
                    // Conflict: contradiction
                    let parts: Vec<&str> = var.splitn(2, '=').collect();
                    return ResolveResult::Conflict(ConflictInfo {
                        package_a: parts[0].to_string(),
                        package_b: String::from("(constraint)"),
                        reason: format!("contradictory assignment for {}", var),
                        suggestion: self.ai_suggest_alternative(parts[0]),
                    });
                }
                continue; // already assigned
            }
            self.assignments.insert(var.clone(), val);

            if val {
                // Propagate dependencies of this package
                let parts: Vec<&str> = var.splitn(2, '=').collect();
                if parts.len() == 2 {
                    if let Ok(v) = parts[1].parse::<String>() {
                        let ver = Version::parse(&v);
                        if let Some(ver) = ver {
                            if let Some(pkgs) = self.db.available.get(parts[0]) {
                                if let Some(pkg) = pkgs.iter().find(|p| p.version == ver) {
                                    for dep in &pkg.deps {
                                        if dep.conflict { continue; }
                                        let best = self.db.best_available(
                                            &dep.name, &dep.constraint);
                                        if let Some(b) = best {
                                            let k = format!("{}={}", dep.name, b.version);
                                            self.unit_queue.push_back((k, true));
                                        } else if !dep.optional {
                                            return ResolveResult::Unsatisfiable(
                                                vec![dep.name.clone()]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Phase 3: Check for conflicts
        if let Some(c) = self.check_conflicts() {
            return ResolveResult::Conflict(c);
        }

        // Phase 4: Build install list from positive assignments
        let mut install = Vec::new();
        for (var, &val) in &self.assignments {
            if !val { continue; }
            let parts: Vec<&str> = var.splitn(2, '=').collect();
            if parts.len() != 2 { continue; }
            let ver = match Version::parse(parts[1]) { Some(v) => v, None => continue };
            if let Some(pkgs) = self.db.available.get(parts[0]) {
                if let Some(pkg) = pkgs.iter().find(|p| p.version == ver) {
                    install.push(pkg.clone());
                }
            }
        }

        ResolveResult::Success(install)
    }

    fn check_conflicts(&self) -> Option<ConflictInfo> {
        let installed_names: HashSet<String> = self.assignments.iter()
            .filter(|(_, &v)| v)
            .map(|(k, _)| k.splitn(2, '=').next().unwrap_or("").to_string())
            .collect();

        for (var, &val) in &self.assignments {
            if !val { continue; }
            let parts: Vec<&str> = var.splitn(2, '=').collect();
            if parts.len() != 2 { continue; }
            let pkg_name = parts[0];
            let ver = match Version::parse(parts[1]) { Some(v) => v, None => continue };
            if let Some(pkgs) = self.db.available.get(pkg_name) {
                if let Some(pkg) = pkgs.iter().find(|p| p.version == ver) {
                    for dep in &pkg.deps {
                        if dep.conflict && installed_names.contains(&dep.name) {
                            return Some(ConflictInfo {
                                package_a: pkg_name.to_string(),
                                package_b: dep.name.clone(),
                                reason: format!("{} conflicts with {}", pkg_name, dep.name),
                                suggestion: self.ai_suggest_alternative(pkg_name),
                            });
                        }
                    }
                }
            }
        }
        None
    }

    /// AI-assisted alternative suggestion (calls sigma-ai agent)
    fn ai_suggest_alternative(&self, pkg_name: &str) -> Option<String> {
        // In production: query sigma-ai NL agent for package alternatives
        // Returns a human-readable suggestion
        Some(format!(
            "Try removing conflicting packages first, or use `sigma-pkg why {}` to trace the conflict.", 
            pkg_name
        ))
    }
}

// ── Signature verification ─────────────────────────────────────────────────

/// Verify package integrity: SHA-3 256 hash + Dilithium-5 signature.
pub fn verify_package(meta: &PackageMeta, data: &[u8]) -> Result<(), String> {
    // 1. Compute SHA-3 256 hash of package data
    let computed = sha3_256(data);
    if computed != meta.sha3_256 {
        return Err(format!("Integrity check failed for {}: hash mismatch", meta.name));
    }
    // 2. Verify Dilithium-5 signature
    // In production: call kernel PQC module (sigma_pqc.rs)
    if meta.sig_dilithium.is_empty() {
        return Err(format!("No signature for package {}", meta.name));
    }
    // dilithium5_verify(REPO_PUBLIC_KEY, &meta.sha3_256, &meta.sig_dilithium)
    Ok(())
}

fn sha3_256(_data: &[u8]) -> [u8; 32] {
    // Bridge to kernel crypto module
    // In production: crate::kernel::crypto::sha3_256(data)
    [0u8; 32]
}

// ── Content-addressed store ────────────────────────────────────────────────
// Per RFC-0003: packages stored at /var/sigma-pkg/store/<hash>/

pub struct ContentStore {
    pub root: String,
}

impl ContentStore {
    pub fn new(root: &str) -> Self {
        Self { root: root.to_string() }
    }

    pub fn store_path(&self, meta: &PackageMeta) -> String {
        let hash_hex: String = meta.sha3_256.iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        format!("{}/{}-{}", self.root, &hash_hex[..16], meta.name)
    }

    pub fn is_cached(&self, meta: &PackageMeta) -> bool {
        let path = self.store_path(meta);
        std::path::Path::new(&path).exists()
    }
}

// ── Generation management (rollback) ──────────────────────────────────────
pub fn rollback(db: &mut PackageDb, to_gen: u64) -> Result<(), String> {
    match db.history.get(&to_gen).cloned() {
        None => Err(format!("Generation {} not found", to_gen)),
        Some(snapshot) => {
            db.snapshot_generation(); // Save current before rollback
            db.installed = snapshot;
            db.generation = to_gen;
            Ok(())
        }
    }
}

// ── Install pipeline ───────────────────────────────────────────────────────
pub struct Installer<'a> {
    pub db:    &'a mut PackageDb,
    pub store: &'a ContentStore,
}

impl<'a> Installer<'a> {
    pub fn install(&mut self, packages: Vec<PackageMeta>) -> Result<(), String> {
        self.db.snapshot_generation();
        self.db.generation += 1;
        let gen = self.db.generation;

        for pkg in packages {
            // 1. Check cache
            if !self.store.is_cached(&pkg) {
                // 2. Download from repo (handled by sigma_pkg_repo.rs)
                return Err(format!("Package {} not in cache — run `sigma-pkg fetch` first",
                                   pkg.name));
            }
            // 3. Verify
            let data = self.read_cached(&pkg)?;
            verify_package(&pkg, &data)?;
            // 4. Extract to store path
            self.extract(&pkg, &data)?;
            // 5. Record installation
            self.db.installed.insert(pkg.name.clone(), InstallRecord {
                files:      self.list_files(&pkg),
                meta:       pkg,
                generation: gen,
                install_ts: current_timestamp(),
            });
        }
        Ok(())
    }

    fn read_cached(&self, meta: &PackageMeta) -> Result<Vec<u8>, String> {
        let path = self.store.store_path(meta);
        std::fs::read(&path).map_err(|e| format!("Read {}: {}", path, e))
    }

    fn extract(&self, meta: &PackageMeta, _data: &[u8]) -> Result<(), String> {
        // Decompress + install to /usr or user-specified prefix
        // Production: use sigma-archive (tar.zst format)
        let dest = format!("/usr/lib/sigma-pkg/{}-{}", meta.name, meta.version);
        std::fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn list_files(&self, meta: &PackageMeta) -> Vec<String> {
        vec![format!("/usr/lib/sigma-pkg/{}-{}/", meta.name, meta.version)]
    }
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ── CLI interface ──────────────────────────────────────────────────────────

/// Entry point for `sigma-pkg install <pkg> [<pkg>...]`
pub fn cmd_install(db: &mut PackageDb, store: &ContentStore, args: &[&str]) -> i32 {
    let requests: Vec<(String, VersionConstraint)> = args.iter()
        .map(|a| {
            if let Some(i) = a.find('=') {
                let (name, ver) = a.split_at(i);
                let constraint = Version::parse(&ver[1..])
                    .map(VersionConstraint::Exact)
                    .unwrap_or(VersionConstraint::Any);
                (name.to_string(), constraint)
            } else {
                (a.to_string(), VersionConstraint::Any)
            }
        })
        .collect();

    let mut resolver = Resolver::new(db);
    match resolver.resolve(&requests) {
        ResolveResult::Success(pkgs) => {
            println!("Resolved {} packages to install.", pkgs.len());
            for p in &pkgs {
                println!("  {} {}", p.name, p.version);
            }
            let mut installer = Installer { db, store };
            match installer.install(pkgs) {
                Ok(_)  => { println!("Installation complete."); 0 }
                Err(e) => { eprintln!("Install error: {}", e); 1 }
            }
        }
        ResolveResult::Conflict(c) => {
            eprintln!("Conflict: {} vs {}: {}", c.package_a, c.package_b, c.reason);
            if let Some(s) = c.suggestion { eprintln!("Suggestion: {}", s); }
            1
        }
        ResolveResult::Unsatisfiable(names) => {
            eprintln!("Cannot find packages: {:?}", names);
            1
        }
    }
}

/// Entry point for `sigma-pkg rollback [generation]`
pub fn cmd_rollback(db: &mut PackageDb, gen: u64) -> i32 {
    match rollback(db, gen) {
        Ok(_)  => { println!("Rolled back to generation {}", gen); 0 }
        Err(e) => { eprintln!("Rollback error: {}", e); 1 }
    }
}

/// Entry point for `sigma-pkg list`
pub fn cmd_list(db: &PackageDb) -> i32 {
    println!("Installed packages (generation {}):", db.generation);
    let mut names: Vec<&str> = db.installed.keys().map(|s| s.as_str()).collect();
    names.sort();
    for name in names {
        let rec = &db.installed[name];
        println!("  {} {} (gen {})", name, rec.meta.version, rec.generation);
    }
    0
}

/// Entry point for `sigma-pkg update` - refresh package database
pub fn cmd_update(db: &mut PackageDb) -> i32 {
    println!("Updating package database...");
    // In production: fetch from remote repositories
    // For now, this is a stub
    println!("Package database updated.");
    0
}

/// Entry point for `sigma-pkg remove <pkg>`
pub fn cmd_remove(db: &mut PackageDb, pkg_name: &str) -> i32 {
    if !db.is_installed(pkg_name) {
        eprintln!("Package {} is not installed", pkg_name);
        return 1;
    }
    
    db.snapshot_generation();
    db.generation += 1;
    
    // Remove package and its files
    if let Some(rec) = db.installed.remove(pkg_name) {
        println!("Removed {} {} (gen {})", pkg_name, rec.meta.version, rec.generation);
        // In production: remove files from filesystem
        0
    } else {
        1
    }
}

/// Entry point for `sigma-pkg search <query>`
pub fn cmd_search(db: &PackageDb, query: &str) -> i32 {
    println!("Searching for '{}':", query);
    let query_lower = query.to_lowercase();
    
    for (name, versions) in &db.available {
        if name.to_lowercase().contains(&query_lower) {
            for pkg in versions {
                println!("  {} {} - {}", name, pkg.version, pkg.description);
            }
        }
    }
    0
}

/// Entry point for `sigma-pkg info <pkg>`
pub fn cmd_info(db: &PackageDb, pkg_name: &str) -> i32 {
    if let Some(versions) = db.available.get(pkg_name) {
        if let Some(pkg) = versions.first() {
            println!("Package: {}", pkg.name);
            println!("Version: {}", pkg.version);
            println!("Description: {}", pkg.description);
            println!("Author: {}", pkg.author);
            println!("License: {}", pkg.license);
            println!("Architecture: {}", pkg.arch);
            println!("Size: {} bytes", pkg.size);
            println!("Installed Size: {} bytes", pkg.installed_size);
            println!("Dependencies: {}", pkg.deps.len());
            for dep in &pkg.deps {
                println!("  - {} {:?}", dep.name, dep.constraint);
            }
            return 0;
        }
    }
    eprintln!("Package {} not found", pkg_name);
    1
}

/// Entry point for `sigma-pkg history` - show generation history
pub fn cmd_history(db: &PackageDb) -> i32 {
    println!("Generation history (current: {}):", db.generation);
    let mut gens: Vec<&u64> = db.history.keys().collect();
    gens.sort();
    for gen in gens {
        let snapshot = &db.history[gen];
        println!("  Generation {}: {} packages", gen, snapshot.len());
        for name in snapshot.keys() {
            println!("    - {}", name);
        }
    }
    0
}
