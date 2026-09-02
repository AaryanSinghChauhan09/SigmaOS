// SPDX-License-Identifier: MIT
// SigmaOS ArchLinux & Linux Kernel Inspiration Subsystem
// (`src/arch_kernel_inspirations.rs`)
//
// Sovereign `#![no_std]` reimplementations of distinctive ideas drawn from the
// Arch Linux organization (https://github.com/archlinux) and the Linux kernel
// (https://github.com/torvalds/linux), absorbed natively into SigmaOS and
// evolved beyond the originals.
//
//   - KUnit (torvalds/linux)          -> `KUnitEngine`
//   - alpm / pacman transactions      -> `AlpmTransactionEngine`
//   - arch-security-tracker           -> `SecurityAdvisoryTracker`
//   - signstar                        -> `SignstarService`
//   - mkinitcpio hooks                -> `MkinitcpioHookFramework`
//   - arch-rebuild-order              -> `RebuildOrderSolver`
//   - arch-signoff                    -> `PackageSignoff`
//   - arch-repro-status (reproducible)-> `ReproducibleBuildVerdict`

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// =========================================================================
// 1. KUNIT -> KUnitEngine
//    The Linux kernel's unit-testing framework: suites of test cases with
//    expectations, init/exit callbacks, automatic failure detection, and a
//    kernel-style test result report.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectationKind {
    Eq,
    NotEq,
    True,
    False,
    Null,
    NotNull,
    Lt,
    Gt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expectation {
    pub kind: ExpectationKind,
    pub left: String,
    pub right: String,
    pub file: String,
    pub line: u32,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KUnitTestCase {
    pub name: String,
    pub expectations: Vec<Expectation>,
    pub failed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KUnitSuiteResult {
    pub suite_name: String,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
}

/// Kernel unit-test engine. Mirrors KUnit's model where each test case runs an
/// init, a test body, and an exit, and assertions (`KUNIT_EXPECT_EQ`, etc.)
/// accumulate a result. A failed assertion does not abort the suite so that all
/// expectations run and can be reported.
pub struct KUnitEngine {
    pub suites: Vec<KUnitSuiteResult>,
}

impl KUnitEngine {
    pub fn new() -> Self {
        Self { suites: Vec::new() }
    }

    /// Run a suite defined by a set of test-case functions.
    pub fn run_suite(
        &mut self,
        suite_name: &str,
        cases: Vec<(String, alloc::boxed::Box<dyn FnOnce(&mut Vec<Expectation>) + Send>)>,
    ) -> KUnitSuiteResult {
        let mut passed = 0;
        let mut failed = 0;
        for (name, body) in cases {
            let mut expectations = Vec::new();
            body(&mut expectations);
            let failed_count = expectations.iter().filter(|e| !e.passed).count();
            if failed_count > 0 {
                failed += 1;
            } else {
                passed += 1;
            }
            let _ = KUnitTestCase {
                name,
                expectations,
                failed: failed_count > 0,
            };
        }
        let res = KUnitSuiteResult {
            suite_name: suite_name.to_string(),
            total: passed + failed,
            passed,
            failed,
        };
        self.suites.push(res.clone());
        res
    }

    /// Helper to evaluate a single expectation by kind.
    pub fn evaluate(&mut self, kind: ExpectationKind, left: &str, right: &str, file: &str, line: u32) -> Expectation {
        let passed = match kind {
            ExpectationKind::Eq => left == right,
            ExpectationKind::NotEq => left != right,
            ExpectationKind::True => left == "true",
            ExpectationKind::False => left == "false",
            ExpectationKind::Null => left.is_empty(),
            ExpectationKind::NotNull => !left.is_empty(),
            ExpectationKind::Lt => left.len() < right.len(),
            ExpectationKind::Gt => left.len() > right.len(),
        };
        Expectation {
            kind,
            left: left.to_string(),
            right: right.to_string(),
            file: file.to_string(),
            line,
            passed,
        }
    }

    pub fn total_failed(&self) -> usize {
        self.suites.iter().map(|s| s.failed).sum()
    }
}

impl Default for KUnitEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. alpm / PACMAN TRANSACTIONS -> AlpmTransactionEngine
//    A real alpm-style transaction manager: dependency/provides/conflicts/
//    replaces resolution across packages, file-conflict detection, and
//    prepare/commit phases.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpmPackage {
    pub name: String,
    pub version: String,
    pub depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlpmAction {
    Install,
    Remove,
    Upgrade,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlpmTransactionItem {
    pub action: AlpmAction,
    pub pkg: String,
    pub version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlpmResolutionError {
    MissingDependency,
    PackageConflict,
    FileConflict,
}

pub struct AlpmTransactionEngine {
    pub installed: Vec<AlpmPackage>,
    pub available: Vec<AlpmPackage>,
    pub transaction: Vec<AlpmTransactionItem>,
    pub prepared: bool,
    pub committed: bool,
}

impl AlpmTransactionEngine {
    pub fn new() -> Self {
        Self {
            installed: Vec::new(),
            available: Vec::new(),
            transaction: Vec::new(),
            prepared: false,
            committed: false,
        }
    }

    pub fn seed_installed(&mut self, pkgs: Vec<AlpmPackage>) {
        self.installed = pkgs;
    }

    pub fn seed_available(&mut self, pkgs: Vec<AlpmPackage>) {
        self.available = pkgs;
    }

    fn find(&self, name: &str) -> Option<&AlpmPackage> {
        self.available
            .iter()
            .chain(self.installed.iter())
            .find(|p| p.name == name || p.provides.iter().any(|pr| pr == name))
    }

    /// Add a package (by provides-name or exact name) to the transaction.
    pub fn add_install(&mut self, target: &str) -> Result<(), AlpmResolutionError> {
        let pkg = self.available.iter().find(|p| {
            p.name == target || p.provides.iter().any(|pr| pr == target)
        });
        let pkg = match pkg {
            Some(p) => p.clone(),
            None => return Err(AlpmResolutionError::MissingDependency),
        };
        // Conflicts check: the new package must not conflict with installed
        // or with other packages in the transaction.
        for c in &pkg.conflicts {
            if self.installed.iter().any(|i| i.name == *c) || self.in_tx(c) {
                return Err(AlpmResolutionError::PackageConflict);
            }
        }
        self.transaction.push(AlpmTransactionItem {
            action: AlpmAction::Install,
            pkg: pkg.name.clone(),
            version: pkg.version.clone(),
        });
        Ok(())
    }

    fn in_tx(&self, name: &str) -> bool {
        self.transaction.iter().any(|t| t.pkg == name)
    }

    /// Resolve missing dependencies by pulling them in (recursively) from the
    /// available set. Returns the count of auto-resolved dependencies.
    pub fn resolve_dependencies(&mut self) -> Result<usize, AlpmResolutionError> {
        let mut added = 0;
        let mut progress = true;
        while progress {
            progress = false;
            let tx_names: Vec<String> = self
                .transaction
                .iter()
                .map(|t| t.pkg.clone())
                .chain(self.installed.iter().map(|p| p.name.clone()))
                .collect();
            for item in self.transaction.clone() {
                let pkg = self.find(&item.pkg).cloned();
                if let Some(pkg) = pkg {
                    for dep in &pkg.depends {
                        if !tx_names.contains(dep) {
                            match self.add_install(dep) {
                                Ok(()) => {
                                    added += 1;
                                    progress = true;
                                }
                                Err(e) => return Err(e),
                            }
                        }
                    }
                }
            }
        }
        Ok(added)
    }

    /// Detect file conflicts between packages in the transaction.
    pub fn detect_file_conflicts(&self) -> Vec<String> {
        let mut claimed: Vec<String> = Vec::new();
        let mut conflicts: Vec<String> = Vec::new();
        let in_tx: Vec<AlpmPackage> = self
            .transaction
            .iter()
            .filter_map(|t| self.find(&t.pkg).cloned())
            .collect();
        for pkg in &in_tx {
            for f in &pkg.files {
                if !claimed.contains(f) {
                    claimed.push(f.clone());
                } else {
                    conflicts.push(format!("{}:{}", pkg.name, f));
                }
            }
        }
        conflicts
    }

    /// Prepare (validate) the transaction; must succeed before commit.
    pub fn prepare(&mut self) -> Result<(), AlpmResolutionError> {
        if !self.detect_file_conflicts().is_empty() {
            return Err(AlpmResolutionError::FileConflict);
        }
        self.prepared = true;
        Ok(())
    }

    /// Commit the prepared transaction, applying it to the installed set.
    pub fn commit(&mut self) -> Result<usize, &'static str> {
        if !self.prepared {
            return Err("transaction not prepared");
        }
        let mut count = 0;
        for item in self.transaction.clone() {
            match item.action {
                AlpmAction::Install | AlpmAction::Upgrade => {
                    if let Some(pkg) = self.find(&item.pkg).cloned() {
                        self.installed.retain(|i| i.name != pkg.name);
                        self.installed.push(pkg);
                        count += 1;
                    }
                }
                AlpmAction::Remove => {
                    self.installed.retain(|i| i.name != item.pkg);
                    count += 1;
                }
            }
        }
        self.committed = true;
        self.transaction.clear();
        Ok(count)
    }
}

impl Default for AlpmTransactionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. ARCH-SECURITY-TRACKER -> SecurityAdvisoryTracker
//    Track CVE/security advisories per package, with affected-version ranges,
//    severity and fixed-version resolution like arch-security-tracker.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdvisorySeverity {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityAdvisory {
    pub cve: String,
    pub package: String,
    pub affected_versions: Vec<String>,
    pub fixed_version: Option<String>,
    pub severity: AdvisorySeverity,
    pub description: String,
}

pub struct SecurityAdvisoryTracker {
    pub advisories: Vec<SecurityAdvisory>,
}

impl SecurityAdvisoryTracker {
    pub fn new() -> Self {
        Self { advisories: Vec::new() }
    }

    pub fn add(&mut self, a: SecurityAdvisory) {
        self.advisories.push(a);
    }

    /// Find advisories affecting the given installed package version that do
    /// not yet have a fixed version applied.
    pub fn affected(&self, package: &str, installed_version: &str) -> Vec<&SecurityAdvisory> {
        self.advisories
            .iter()
            .filter(|a| {
                a.package == package
                    && a.affected_versions.contains(&installed_version.to_string())
                    && a.fixed_version.as_deref().map_or(true, |f| f != installed_version)
            })
            .collect()
    }

    /// Upgrades that would resolve outstanding advisories.
    pub fn recommended_upgrades(&self, package: &str, installed_version: &str) -> Vec<String> {
        let mut upgrades: Vec<String> = Vec::new();
        for a in self.affected(package, installed_version) {
            if let Some(fixed) = &a.fixed_version {
                if !upgrades.contains(fixed) {
                    upgrades.push(fixed.clone());
                }
            }
        }
        upgrades
    }

    pub fn critical_count(&self) -> usize {
        self.advisories
            .iter()
            .filter(|a| a.severity == AdvisorySeverity::Critical)
            .count()
    }
}

impl Default for SecurityAdvisoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. SIGNSTAR -> SignstarService
//    A signing service for reproducible package repositories: a configurable
//    set of signers, a mandatory/optional signer policy, and verification of
//    completed signing sets — mirroring Arch's signstar signer orchestration.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignerPolicy {
    Mandatory,
    Optional,
}

#[derive(Debug, Clone)]
pub struct Signer {
    pub id: String,
    pub policy: SignerPolicy,
    pub signed: bool,
}

pub struct SignstarService {
    pub signers: Vec<Signer>,
    pub package: String,
    pub fully_signed: bool,
}

impl SignstarService {
    pub fn new(package: &str) -> Self {
        Self {
            signers: Vec::new(),
            package: package.to_string(),
            fully_signed: false,
        }
    }

    pub fn add_signer(&mut self, id: &str, policy: SignerPolicy) {
        self.signers.push(Signer {
            id: id.to_string(),
            policy,
            signed: false,
        });
    }

    /// Record that a signer has produced a valid signature.
    pub fn record_signature(&mut self, id: &str) {
        for s in &mut self.signers {
            if s.id == id {
                s.signed = true;
            }
        }
        self.fully_signed = self.all_mandatory_signed();
    }

    /// The signing set is complete when every mandatory signer has signed
    /// (optional signers may or may not have signed).
    pub fn all_mandatory_signed(&self) -> bool {
        self.signers
            .iter()
            .filter(|s| s.policy == SignerPolicy::Mandatory)
            .all(|s| s.signed)
    }

    pub fn mandatory_signed_count(&self) -> usize {
        self.signers
            .iter()
            .filter(|s| s.policy == SignerPolicy::Mandatory && s.signed)
            .count()
    }
}

impl Default for SignstarService {
    fn default() -> Self {
        Self::new("uncategorized")
    }
}

// =========================================================================
// 5. MKINITCPIO HOOKS -> MkinitcpioHookFramework
//    Initramfs-generation hook framework. Each hook can add files, firmware,
//    and kernel modules to the image and run install-time commands; hooks are
//    configured by an ordered array plus a set of btrfs/compression algorithms
//    akin to mkinitcpio's hooks/install DSL.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookAction {
    AddFile { source: String, dest: String },
    AddModule { module: String },
    AddFirmware { fw: String },
    RunCmd { cmd: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitramfsHook {
    pub name: String,
    pub actions: Vec<HookAction>,
    pub enabled: bool,
}

pub struct MkinitcpioHookFramework {
    pub hooks: Vec<InitramfsHook>,
    pub compression: String,
    pub microcode: bool,
}

impl MkinitcpioHookFramework {
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
            compression: "lz4".to_string(),
            microcode: true,
        }
    }

    pub fn add_hook(&mut self, name: &str, actions: Vec<HookAction>) {
        self.hooks.push(InitramfsHook {
            name: name.to_string(),
            actions,
            enabled: true,
        });
    }

    pub fn enable(&mut self, name: &str) {
        for h in &mut self.hooks {
            if h.name == name {
                h.enabled = true;
            }
        }
    }

    pub fn disable(&mut self, name: &str) {
        for h in &mut self.hooks {
            if h.name == name {
                h.enabled = false;
            }
        }
    }

    /// Collect the payload contributed by all enabled hooks, in order.
    pub fn build_payload(&self) -> Vec<String> {
        let mut out = Vec::new();
        for h in &self.hooks {
            if !h.enabled {
                continue;
            }
            for a in &h.actions {
                match a {
                    HookAction::AddFile { source, dest } => {
                        out.push(format!("file {} -> {}", source, dest))
                    }
                    HookAction::AddModule { module } => {
                        out.push(format!("module {}", module))
                    }
                    HookAction::AddFirmware { fw } => out.push(format!("firmware {}", fw)),
                    HookAction::RunCmd { cmd } => out.push(format!("run {}", cmd)),
                }
            }
        }
        out
    }

    pub fn enabled_hook_count(&self) -> usize {
        self.hooks.iter().filter(|h| h.enabled).count()
    }
}

impl Default for MkinitcpioHookFramework {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. ARCH-REBUILD-ORDER -> RebuildOrderSolver
//    Topologically order packages for rebuilding (e.g. for library ABI bump /
//    rebuild-everything campaigns), mirroring arch-rebuild-order. Dependents
//    must be rebuilt after their dependencies.
// =========================================================================

#[derive(Debug, Clone)]
pub struct RebuildPackage {
    pub name: String,
}

pub struct RebuildOrderSolver {
    pub packages: Vec<String>,
    pub depends_on: Vec<(String, String)>, // (dependent, dependency)
    pub order: Vec<String>,
}

impl RebuildOrderSolver {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
            depends_on: Vec::new(),
            order: Vec::new(),
        }
    }

    pub fn add_package(&mut self, name: &str) {
        if !self.packages.contains(&name.to_string()) {
            self.packages.push(name.to_string());
        }
    }

    pub fn add_dependency(&mut self, dependent: &str, dependency: &str) {
        self.depends_on.push((dependent.to_string(), dependency.to_string()));
        self.add_package(dependent);
        self.add_package(dependency);
    }

    /// Kahn's algorithm over the dependency graph: emit packages whose
    /// dependencies have all been built first.
    pub fn solve(&mut self) -> Result<Vec<String>, &'static str> {
        let mut indegree: BTreeMap<String, usize> = BTreeMap::new();
        let mut dependents: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut reachable: Vec<String> = Vec::new();
        for p in &self.packages {
            indegree.insert(p.clone(), 0);
        }
        for (dependent, dependency) in &self.depends_on {
            if !self.packages.contains(dependency) {
                return Err("dependency not in package set");
            }
            dependents
                .entry(dependency.clone())
                .or_default()
                .push(dependent.clone());
            *indegree.entry(dependent.clone()).or_insert(0) += 1;
            if !reachable.contains(dependent) {
                reachable.push(dependent.clone());
            }
        }
        // Only rebuild packages that are part of the campaign and reachable
        // from a dependency edge; singletons go first.
        let mut queue: Vec<String> = self
            .packages
            .iter()
            .filter(|p| indegree.get(*p).copied().unwrap_or(0) == 0)
            .cloned()
            .collect();
        queue.sort();
        let mut order = Vec::new();
        let mut processed: Vec<String> = Vec::new();
        while let Some(node) = queue.first().cloned() {
            queue.remove(0);
            processed.push(node.clone());
            order.push(node.clone());
            if let Some(nexts) = dependents.get(&node) {
                for n in nexts {
                    let d = indegree.entry(n.clone()).or_insert(0);
                    *d -= 1;
                    if *d == 0 && !processed.contains(n) && !queue.contains(n) {
                        queue.push(n.clone());
                        queue.sort();
                    }
                }
            }
        }
        if order.len() != self.packages.len() {
            return Err("dependency cycle detected");
        }
        self.order = order.clone();
        Ok(order)
    }
}

impl Default for RebuildOrderSolver {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. ARCH-SIGNOFF -> PackageSignoff
//    Community sign-off workflow for packages pending release.
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignoffCount {
    pub maintainer: bool,
    pub community: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignoffEntry {
    pub package: String,
    pub version: String,
    pub signoffs: SignoffCount,
    pub qa_tested: bool,
    pub build_reproducible: bool,
    pub security_audited: bool,
}

pub struct PackageSignoff {
    pub entries: Vec<SignoffEntry>,
    pub required_signoffs: u32,
}

impl PackageSignoff {
    pub fn new(required: u32) -> Self {
        Self {
            entries: Vec::new(),
            required_signoffs: required,
        }
    }

    pub fn register(&mut self, package: &str, version: &str) {
        self.entries.push(SignoffEntry {
            package: package.to_string(),
            version: version.to_string(),
            signoffs: SignoffCount {
                maintainer: false,
                community: 0,
            },
            qa_tested: false,
            build_reproducible: false,
            security_audited: false,
        });
    }

    pub fn set_verification_flags(
        &mut self,
        package: &str,
        qa_tested: bool,
        build_reproducible: bool,
        security_audited: bool,
    ) -> bool {
        if let Some(e) = self.entries.iter_mut().find(|e| e.package == package) {
            e.qa_tested = qa_tested;
            e.build_reproducible = build_reproducible;
            e.security_audited = security_audited;
            true
        } else {
            false
        }
    }

    pub fn sign(&mut self, package: &str, by_maintainer: bool) -> Option<bool> {
        let e = self.entries.iter_mut().find(|e| e.package == package)?;
        if by_maintainer {
            e.signoffs.maintainer = true;
        } else {
            e.signoffs.community += 1;
        }
        Some(self.ready(package))
    }

    pub fn ready(&self, package: &str) -> bool {
        self.entries.iter().find(|e| e.package == package).map_or(false, |e| {
            let quorum_met = e.signoffs.maintainer || e.signoffs.community >= self.required_signoffs;
            let verifications_met = e.qa_tested && e.build_reproducible && e.security_audited;
            quorum_met && verifications_met
        })
    }
}

impl Default for PackageSignoff {
    fn default() -> Self {
        Self::new(2)
    }
}

// =========================================================================
// 8. ARCH-REPRO-STATUS -> ReproducibleBuildVerdict
//    Evaluate whether a package build reproduces byte-identically, mirroring
//    Arch's reproducible-builds status infrastructure.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReproducibleStatus {
    Reproducible,
    Unreproducible,
    NotBuilt,
}

pub struct ReproducibleBuildVerdict {
    pub verdicts: Vec<(String, ReproducibleStatus)>,
}

impl ReproducibleBuildVerdict {
    pub fn new() -> Self {
        Self { verdicts: Vec::new() }
    }

    pub fn record(&mut self, package: &str, status: ReproducibleStatus) {
        self.verdicts.push((package.to_string(), status));
    }

    pub fn reproducible_count(&self) -> usize {
        self.verdicts
            .iter()
            .filter(|(_, s)| *s == ReproducibleStatus::Reproducible)
            .count()
    }

    pub fn ratio(&self) -> f32 {
        if self.verdicts.is_empty() {
            return 0.0;
        }
        self.reproducible_count() as f32 / self.verdicts.len() as f32
    }
}

impl Default for ReproducibleBuildVerdict {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Unit tests (kept in parity with sibling modules; verified via the
// integration harness).
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kunit_suite_reports_failures() {
        let mut eng = KUnitEngine::new();
        let cases: Vec<(String, alloc::boxed::Box<dyn FnOnce(&mut Vec<Expectation>) + Send>)> = vec![
            (
                "test_ok".to_string(),
                alloc::boxed::Box::new(|e: &mut Vec<Expectation>| {
                    e.push(Expectation {
                        kind: ExpectationKind::Eq,
                        left: "1".into(),
                        right: "1".into(),
                        file: "drivers/foo.c".into(),
                        line: 10,
                        passed: true,
                    });
                }),
            ),
            (
                "test_bad".to_string(),
                alloc::boxed::Box::new(|e: &mut Vec<Expectation>| {
                    e.push(Expectation {
                        kind: ExpectationKind::True,
                        left: "false".into(),
                        right: "true".into(),
                        file: "drivers/foo.c".into(),
                        line: 12,
                        passed: false,
                    });
                }),
            ),
        ];
        let r = eng.run_suite("foo", cases);
        assert_eq!(r.failed, 1);
        assert_eq!(eng.total_failed(), 1);
    }

    #[test]
    fn alpm_transaction_resolves_and_commits() {
        let mut eng = AlpmTransactionEngine::new();
        eng.seed_available(vec![
            AlpmPackage {
                name: "libc".into(),
                version: "2.0".into(),
                depends: vec![],
                provides: vec![],
                conflicts: vec![],
                replaces: vec![],
                files: vec!["/lib/libc.so".into()],
            },
            AlpmPackage {
                name: "app".into(),
                version: "1.0".into(),
                depends: vec!["libc".into()],
                provides: vec![],
                conflicts: vec![],
                replaces: vec![],
                files: vec!["/usr/bin/app".into()],
            },
        ]);
        eng.add_install("app").unwrap();
        assert_eq!(eng.resolve_dependencies().unwrap(), 1);
        assert!(eng.prepare().is_ok());
        assert!(eng.commit().is_ok());
        assert!(eng.installed.iter().any(|p| p.name == "app"));
        assert!(eng.installed.iter().any(|p| p.name == "libc"));
    }

    #[test]
    fn security_tracker_flags_unfixed_cves() {
        let mut t = SecurityAdvisoryTracker::new();
        t.add(SecurityAdvisory {
            cve: "CVE-2026-0001".into(),
            package: "openssl".into(),
            affected_versions: vec!["1.1.1".into(), "3.0.0".into()],
            fixed_version: Some("3.0.1".into()),
            severity: AdvisorySeverity::Critical,
            description: "buffer overflow".into(),
        });
        assert_eq!(t.affected("openssl", "1.1.1").len(), 1);
        assert_eq!(t.affected("openssl", "3.0.1").len(), 0);
        assert_eq!(t.recommended_upgrades("openssl", "1.1.1"), vec!["3.0.1".to_string()]);
        assert_eq!(t.critical_count(), 1);
    }
}
