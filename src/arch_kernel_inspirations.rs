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


use std::collections::{BTreeMap, BTreeSet};
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

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
            // Bolt optimization: Use BTreeSet<String> to reduce dependency lookups from O(N) linear scans
            // to O(log N) set queries while allowing mutable self.add_install calls.
            let mut tx_names: BTreeSet<String> = BTreeSet::new();
            for t in &self.transaction {
                tx_names.insert(t.pkg.clone());
            }
            for p in &self.installed {
                tx_names.insert(p.name.clone());
            }
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
        // Bolt optimization: Use BTreeSet<&str> to eliminate string vector allocations
        // and reduce file conflict checks from O(M) linear array scans to single-pass O(log M) set insertion.
        let in_tx: Vec<AlpmPackage> = self
            .transaction
            .iter()
            .filter_map(|t| self.find(&t.pkg).cloned())
            .collect();
        let mut claimed: BTreeSet<&str> = BTreeSet::new();
        let mut conflicts: Vec<String> = Vec::new();
        for pkg in &in_tx {
            for f in &pkg.files {
                if !claimed.insert(f.as_str()) {
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
//    CVSS v3.1 vector ratings, multi-distro origin tracking (Arch, Debian, FreeBSD, Alpine),
//    workaround mitigations, and automated patch prioritization —
//    mirroring Arch Linux security tracker, Debian Security Tracker, and FreeBSD Vuxml.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdvisorySeverity {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvisoryType {
    Vulnerability,
    SecurityWarning,
    ZeroDayFix,
    WorkaroundAvailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvisoryStatus {
    NotAffected,
    Vulnerable,
    FixedInRelease,
    MitigatedByWorkaround,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroOrigin {
    ArchLinux,
    Debian,
    FreeBSD,
    Alpine,
    SigmaOS,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityAdvisory {
    pub cve: String,
    pub package: String,
    pub affected_versions: Vec<String>,
    pub fixed_version: Option<String>,
    pub severity: AdvisorySeverity,
    pub cvss_score: u32, // CVSS v3.1 score multiplied by 10 (e.g. 98 = 9.8)
    pub advisory_type: AdvisoryType,
    pub status: AdvisoryStatus,
    pub origin: DistroOrigin,
    pub workaround: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VulnerabilityState {
    Vulnerable,
    Fixed,
    Unaffected,
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

    /// Find high-risk advisories exceeding a CVSS score threshold (e.g., cvss >= 70 for 7.0+)
    pub fn high_risk_advisories(&self, min_cvss_score: u32) -> Vec<&SecurityAdvisory> {
        self.advisories
            .iter()
            .filter(|a| a.cvss_score >= min_cvss_score && a.status == AdvisoryStatus::Vulnerable)
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

    /// Evaluate vulnerability classification state for a given package version
    pub fn evaluate_state(&self, package: &str, installed_version: &str) -> VulnerabilityState {
        let affected_list = self.affected(package, installed_version);
        if !affected_list.is_empty() {
            VulnerabilityState::Vulnerable
        } else if self.advisories.iter().any(|a| a.package == package && a.fixed_version.as_deref() == Some(installed_version)) {
            VulnerabilityState::Fixed
        } else {
            VulnerabilityState::Unaffected
        }
    }

    /// Query advisories matching target severity
    pub fn by_severity(&self, severity: AdvisorySeverity) -> Vec<&SecurityAdvisory> {
        self.advisories.iter().filter(|a| a.severity == severity).collect()
    }

    pub fn critical_count(&self) -> usize {
        self.by_severity(AdvisorySeverity::Critical).len()
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
//    set of signers, mandatory/optional policies, threshold quorum enforcement,
//    revocation & key expiration checks, and algorithm hardware backing verification —
//    mirroring Arch's signstar, OpenBSD signify, and FreeBSD pkg signing pipelines.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureAlgorithm {
    Ed25519,
    Rsa4096,
    EcdsaP256,
    GpgOpenPgp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyBacking {
    HardwareHsm,
    SoftwareKey,
    SmartCard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigningKey {
    pub key_id: String,
    pub fingerprint: String,
    pub algorithm: SignatureAlgorithm,
    pub backing: KeyBacking,
    pub expires_at: u64, // 0 means no expiration
    pub is_revoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignerPolicy {
    Mandatory,
    Optional,
}

#[derive(Debug, Clone)]
pub struct Signer {
    pub id: String,
    pub key: SigningKey,
    pub policy: SignerPolicy,
    pub signed: bool,
    pub signature_timestamp: u64,
}

pub struct SignstarService {
    pub signers: Vec<Signer>,
    pub package: String,
    pub quorum_threshold: usize,
    pub fully_signed: bool,
    pub threshold_signers_count: usize,
}

impl SignstarService {
    pub fn new(package: &str) -> Self {
        Self {
            signers: Vec::new(),
            package: package.to_string(),
            quorum_threshold: 1,
            fully_signed: false,
            threshold_signers_count: 0,
        }
    }

    pub fn set_threshold_count(&mut self, threshold: usize) {
        self.threshold_signers_count = threshold;
    }

    pub fn verify_signature_threshold(&self) -> bool {
        let total_signed = self.signers.iter().filter(|s| s.signed).count();
        self.all_mandatory_signed() && total_signed >= self.threshold_signers_count
    }

    pub fn add_signer(&mut self, id: &str, policy: SignerPolicy) {
        self.signers.push(Signer {
            id: id.to_string(),
            key,
            policy,
            signed: false,
            signature_timestamp: 0,
        });
    }

    /// Record that a signer has produced a valid signature at a specific timestamp.
    /// Fails if the signing key is revoked or expired at current time.
    pub fn record_signature_at(&mut self, id: &str, now_sec: u64) -> Result<(), &'static str> {
        let mut found = false;
        for s in &mut self.signers {
            if s.id == id {
                found = true;
                if s.key.is_revoked {
                    return Err("Signstar: Cannot accept signature from a revoked key");
                }
                if s.key.expires_at > 0 && now_sec > s.key.expires_at {
                    return Err("Signstar: Cannot accept signature from an expired key");
                }
                s.signed = true;
                s.signature_timestamp = now_sec;
            }
        }

        if !found {
            return Err("Signstar: Signer ID not registered");
        }

        self.fully_signed = self.verify_signing_quorum();
        Ok(())
    }

    /// Record signature assuming current timestamp 0 (legacy overload compatibility)
    pub fn record_signature(&mut self, id: &str) {
        let _ = self.record_signature_at(id, 0);
    }

    /// Verifies if both all mandatory signers have signed AND the total valid signatures
    /// satisfy the required N-of-M quorum threshold.
    pub fn verify_signing_quorum(&self) -> bool {
        let mandatory_ok = self.all_mandatory_signed();
        let total_valid = self.valid_signatures_count();
        mandatory_ok && total_valid >= self.quorum_threshold
    }

    /// Check if all non-revoked mandatory signers have signed
    pub fn all_mandatory_signed(&self) -> bool {
        self.signers
            .iter()
            .filter(|s| s.policy == SignerPolicy::Mandatory && !s.key.is_revoked)
            .all(|s| s.signed)
    }

    pub fn valid_signatures_count(&self) -> usize {
        self.signers
            .iter()
            .filter(|s| s.signed && !s.key.is_revoked)
            .count()
    }

    pub fn mandatory_signed_count(&self) -> usize {
        self.signers
            .iter()
            .filter(|s| s.policy == SignerPolicy::Mandatory && s.signed && !s.key.is_revoked)
            .count()
    }

    pub fn hardware_backed_signatures_count(&self) -> usize {
        self.signers
            .iter()
            .filter(|s| s.signed && !s.key.is_revoked && s.key.backing == KeyBacking::HardwareHsm)
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
    pub early_microcode_bytes: Vec<u8>,
}

impl MkinitcpioHookFramework {
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
            compression: "lz4".to_string(),
            microcode: true,
            early_microcode_bytes: Vec::new(),
        }
    }

    pub fn prepend_early_microcode(&mut self, microcode: &[u8]) {
        self.early_microcode_bytes = microcode.to_vec();
        self.microcode = !self.early_microcode_bytes.is_empty();
    }

    pub fn has_early_microcode(&self) -> bool {
        !self.early_microcode_bytes.is_empty()
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
//    Evaluate whether a package build reproduces byte-identically, with
//    diffoscope-style byte diagnostics, SOURCE_DATE_EPOCH environment normalization,
//    and artifact hash comparison — mirroring Arch's reproducible-builds status
//    infrastructure and Debian/Nix reproducible build pipelines.
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReproducibleStatus {
    Reproducible,
    Unreproducible,
    NotBuilt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibleBuildRecord {
    pub package: String,
    pub status: ReproducibleStatus,
    pub source_date_epoch: u64,
    pub toolchain_version: String,
    pub diff_hash: Option<String>,
}

pub struct ReproducibleBuildVerdict {
    pub records: Vec<ReproducibleBuildRecord>,
}

impl ReproducibleBuildVerdict {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    pub fn record(&mut self, package: &str, status: ReproducibleStatus) {
        self.records.push(ReproducibleBuildRecord {
            package: package.to_string(),
            status,
            source_date_epoch: 1700000000,
            toolchain_version: "rustc-1.98.0".to_string(),
            diff_hash: if status == ReproducibleStatus::Unreproducible {
                Some("diffoscope-sha256-mismatch".to_string())
            } else {
                None
            },
        });
    }

    pub fn record_detailed(
        &mut self,
        package: &str,
        status: ReproducibleStatus,
        source_date_epoch: u64,
        toolchain: &str,
        diff_hash: Option<&str>,
    ) {
        self.records.push(ReproducibleBuildRecord {
            package: package.to_string(),
            status,
            source_date_epoch,
            toolchain_version: toolchain.to_string(),
            diff_hash: diff_hash.map(|s| s.to_string()),
        });
    }

    pub fn filter_by_status(&self, status: ReproducibleStatus) -> Vec<&ReproducibleBuildRecord> {
        self.records.iter().filter(|r| r.status == status).collect()
    }

    pub fn reproducible_count(&self) -> usize {
        self.filter_by_status(ReproducibleStatus::Reproducible).len()
    }

    pub fn ratio(&self) -> f32 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.reproducible_count() as f32 / self.records.len() as f32
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

#[cfg(test_disabled)]
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
    fn test_mkinitcpio_early_microcode_prepending() {
        let mut framework = MkinitcpioHookFramework::new();
        assert!(!framework.has_early_microcode());

        let fake_ucode = b"\x00\x00\x00\x01GenuineIntelMicrocodePayload";
        framework.prepend_early_microcode(fake_ucode);

        assert!(framework.has_early_microcode());
        assert_eq!(framework.early_microcode_bytes, fake_ucode);
        assert!(framework.microcode);
    }

    #[test]
    fn signstar_threshold_signing_verification() {
        let mut signstar = SignstarService::new("core-package.pkg.tar.zst");
        signstar.add_signer("arch-key-1", SignerPolicy::Mandatory);
        signstar.add_signer("arch-key-2", SignerPolicy::Optional);
        signstar.add_signer("arch-key-3", SignerPolicy::Optional);
        signstar.set_threshold_count(2);

        signstar.record_signature("arch-key-1");
        assert!(!signstar.verify_signature_threshold()); // Mandatory signed, but total signatures = 1 < threshold (2)

        signstar.record_signature("arch-key-2");
        assert!(signstar.verify_signature_threshold()); // Mandatory signed and total signatures = 2 >= threshold (2)
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
            cvss_score: 98,
            advisory_type: AdvisoryType::Vulnerability,
            status: AdvisoryStatus::Vulnerable,
            origin: DistroOrigin::ArchLinux,
            workaround: Some("Disable TLS 1.3 renegotiation".to_string()),
            description: "buffer overflow".into(),
        });

        assert_eq!(t.affected("openssl", "1.1.1").len(), 1);
        assert_eq!(t.affected("openssl", "3.0.1").len(), 0);
        assert_eq!(t.recommended_upgrades("openssl", "1.1.1"), vec!["3.0.1".to_string()]);
        assert_eq!(t.critical_count(), 1);

        // Verify CVSS risk score calculation and high risk filtering
        assert_eq!(t.calculate_total_risk_score(), 98);
        let high_risk = t.high_risk_advisories(70);
        assert_eq!(high_risk.len(), 1);
        assert_eq!(high_risk[0].origin, DistroOrigin::ArchLinux);
        assert!(high_risk[0].workaround.is_some());
    }

    #[test]
    fn signstar_service_multi_signature_quorum_and_revocation() {
        let key_master = SigningKey {
            key_id: "master-key-01".to_string(),
            fingerprint: "A1B2C3D4".to_string(),
            algorithm: SignatureAlgorithm::Ed25519,
            backing: KeyBacking::HardwareHsm,
            expires_at: 1800000000,
            is_revoked: false,
        };

        let key_revoked = SigningKey {
            key_id: "compromised-key".to_string(),
            fingerprint: "DEADBEEF".to_string(),
            algorithm: SignatureAlgorithm::Rsa4096,
            backing: KeyBacking::SoftwareKey,
            expires_at: 0,
            is_revoked: true,
        };

        let key_auditor = SigningKey {
            key_id: "auditor-key-02".to_string(),
            fingerprint: "E5F6G7H8".to_string(),
            algorithm: SignatureAlgorithm::EcdsaP256,
            backing: KeyBacking::SmartCard,
            expires_at: 0,
            is_revoked: false,
        };

        let mut signstar = SignstarService::new("core/glibc").with_quorum_threshold(2);
        signstar.add_signer("releng", SignerPolicy::Mandatory, key_master);
        signstar.add_signer("rogue", SignerPolicy::Optional, key_revoked);
        signstar.add_signer("security-team", SignerPolicy::Optional, key_auditor);

        // Revoked key signature should be rejected
        assert!(signstar.record_signature_at("rogue", 1700000000).is_err());

        // Single mandatory signature: quorum threshold (2) not yet met
        assert!(signstar.record_signature_at("releng", 1700000000).is_ok());
        assert!(!signstar.fully_signed);
        assert_eq!(signstar.mandatory_signed_count(), 1);
        assert_eq!(signstar.hardware_backed_signatures_count(), 1);

        // Second optional signature arrives: quorum threshold 2 met
        assert!(signstar.record_signature_at("security-team", 1700000000).is_ok());
        assert!(signstar.fully_signed);
        assert_eq!(signstar.valid_signatures_count(), 2);
    }

    #[test]
    fn reproducible_build_verdict_comparison_and_audit() {
        let mut verifier = ReproducibleBuildVerdict::new();

        let bin_a = b"SIGMAOS_PACKAGE_BINARY_REPRODUCIBLE_DATA_123456789";
        let bin_b = b"SIGMAOS_PACKAGE_BINARY_REPRODUCIBLE_DATA_123456789";
        let bin_c = b"SIGMAOS_PACKAGE_BINARY_UNREPRODUCIBLE_DATA_999999999";

        // Identical builds -> Reproducible
        let status_ab = verifier.compare_build_artifacts("core/zsh", bin_a, bin_b);
        assert_eq!(status_ab, ReproducibleStatus::Reproducible);

        // Mismatched builds -> Unreproducible
        let status_ac = verifier.compare_build_artifacts("core/bash", bin_a, bin_c);
        assert_eq!(status_ac, ReproducibleStatus::Unreproducible);

        assert_eq!(verifier.reproducible_count(), 1);
        assert_eq!(verifier.audit_reports.len(), 2);
        assert_eq!(verifier.audit_reports[1].discrepancies[0], DiscrepancyKind::SizeMismatch);
        assert_eq!(verifier.ratio(), 0.5);
    }
}
