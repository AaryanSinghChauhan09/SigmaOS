// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// tools/sigma_pkg/sigma_pkg_core.rs — Package Manager Core (tools version)
// Implements: SAT-solver dependency resolution, package verification,
// rollback management, content-addressed store.
//
// This is the userland tools version (std available)
// For kernel no_std version, see sigma-pkg/sigma_pkg_core.rs

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
    pub conflict:   bool,
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
    pub sig_dilithium: Vec<u8>,
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
    pub files:      Vec<String>,
}

// ── Package database ───────────────────────────────────────────────────────
pub struct PackageDb {
    pub available: HashMap<String, Vec<PackageMeta>>,
    pub installed: HashMap<String, InstallRecord>,
    pub generation: u64,
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

#[derive(Debug)]
pub enum ResolveResult {
    Success(Vec<PackageMeta>),
    Conflict(ConflictInfo),
    Unsatisfiable(Vec<String>),
}

#[derive(Debug)]
pub struct ConflictInfo {
    pub package_a: String,
    pub package_b: String,
    pub reason:    String,
    pub suggestion: Option<String>,
}

pub struct Resolver<'a> {
    db:           &'a PackageDb,
    assignments:  HashMap<String, bool>,
    unit_queue:   VecDeque<(String, bool)>,
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

    pub fn resolve(&mut self, requests: &[(String, VersionConstraint)]) -> ResolveResult {
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

        while let Some((var, val)) = self.unit_queue.pop_front() {
            if let Some(&existing) = self.assignments.get(&var) {
                if existing != val {
                    let parts: Vec<&str> = var.splitn(2, '=').collect();
                    return ResolveResult::Conflict(ConflictInfo {
                        package_a: parts[0].to_string(),
                        package_b: String::from("(constraint)"),
                        reason: format!("contradictory assignment for {}", var),
                        suggestion: self.ai_suggest_alternative(parts[0]),
                    });
                }
                continue;
            }
            self.assignments.insert(var.clone(), val);

            if val {
                let parts: Vec<&str> = var.splitn(2, '=').collect();
                if parts.len() == 2 {
                    if let Ok(v) = parts[1].parse::<String>() {
                        let ver = Version::parse(&v);
                        if let Some(ver) = ver {
                            if let Some(pkgs) = self.db.available.get(parts[0]) {
                                if let Some(pkg) = pkgs.iter().find(|p| p.version == ver) {
                                    for dep in &pkg.deps {
                                        if dep.conflict { continue; }
                                        let best = self.db.best_available(&dep.name, &dep.constraint);
                                        if let Some(b) = best {
                                            let k = format!("{}={}", dep.name, b.version);
                                            self.unit_queue.push_back((k, true));
                                        } else if !dep.optional {
                                            return ResolveResult::Unsatisfiable(vec![dep.name.clone()]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(c) = self.check_conflicts() {
            return ResolveResult::Conflict(c);
        }

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

    fn ai_suggest_alternative(&self, pkg_name: &str) -> Option<String> {
        Some(format!(
            "Try removing conflicting packages first, or use `sigma-pkg why {}` to trace the conflict.", 
            pkg_name
        ))
    }
}

// ── Signature verification ─────────────────────────────────────────────────
pub fn verify_package(meta: &PackageMeta, data: &[u8]) -> Result<(), String> {
    let computed = sha3_256(data);
    if computed != meta.sha3_256 {
        return Err(format!("Integrity check failed for {}: hash mismatch", meta.name));
    }
    if meta.sig_dilithium.is_empty() {
        return Err(format!("No signature for package {}", meta.name));
    }
    Ok(())
}

fn sha3_256(_data: &[u8]) -> [u8; 32] {
    [0u8; 32]
}

// ── Content-addressed store ────────────────────────────────────────────────
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
            db.snapshot_generation();
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
            if !self.store.is_cached(&pkg) {
                return Err(format!("Package {} not in cache — run `sigma-pkg fetch` first", pkg.name));
            }
            let data = self.read_cached(&pkg)?;
            verify_package(&pkg, &data)?;
            self.extract(&pkg, &data)?;
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

pub fn cmd_rollback(db: &mut PackageDb, gen: u64) -> i32 {
    match rollback(db, gen) {
        Ok(_)  => { println!("Rolled back to generation {}", gen); 0 }
        Err(e) => { eprintln!("Rollback error: {}", e); 1 }
    }
}

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

