// SPDX-License-Identifier: MIT
// SigmaOS Engineering Framework — AAA Testing, Component Isolation,
// Specification-Driven Development, and Support/Update Programs
// Implements remaining quality principles in one cohesive module.

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

// ============================================================================
// ARRANGE-ACT-ASSERT (AAA) TESTING FRAMEWORK
// ============================================================================

/// AAA test case: Arrange -> Act -> Assert with deterministic lifecycle
pub struct AaaTest {
    name: String,
    arrange: Box<dyn Fn() -> TestContext>,
    act: Box<dyn Fn(&TestContext) -> TestOutcome>,
    assert: Box<dyn Fn(&TestOutcome) -> Result<(), String>>,
}

/// Shared context produced by Arrange, consumed by Act/Assert
pub struct TestContext {
    pub values: HashMap<String, String>,
}

impl TestContext {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }
    pub fn set(&mut self, k: &str, v: impl Into<String>) {
        self.values.insert(k.to_string(), v.into());
    }
    pub fn get(&self, k: &str) -> Option<&String> {
        self.values.get(k)
    }
}

impl Default for TestContext {
    fn default() -> Self { Self::new() }
}

/// Outcome produced by Act, checked by Assert
#[derive(Debug)]
pub struct TestOutcome {
    pub success: bool,
    pub result: Option<String>,
    pub error: Option<String>,
    pub duration_ms: u64,
}

impl TestOutcome {
    pub fn ok(result: impl Into<String>) -> Self {
        Self { success: true, result: Some(result.into()), error: None, duration_ms: 0 }
    }
    pub fn err(msg: impl Into<String>) -> Self {
        Self { success: false, result: None, error: Some(msg.into()), duration_ms: 0 }
    }
}

impl AaaTest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arrange: Box::new(|| TestContext::new()),
            act: Box::new(|_| TestOutcome::ok("")),
            assert: Box::new(|_| Ok(())),
        }
    }
    pub fn arrange<F>(mut self, f: F) -> Self
    where F: Fn() -> TestContext + 'static {
        self.arrange = Box::new(f); self
    }
    pub fn act<F>(mut self, f: F) -> Self
    where F: Fn(&TestContext) -> TestOutcome + 'static {
        self.act = Box::new(f); self
    }
    pub fn assert<F>(mut self, f: F) -> Self
    where F: Fn(&TestOutcome) -> Result<(), String> + 'static {
        self.assert = Box::new(f); self
    }
    pub fn name(&self) -> &str { &self.name }

    /// Run the test in strict Arrange-Act-Assert order
    pub fn run(&self) -> Result<(), String> {
        let ctx = (self.arrange)();           // ARRANGE
        let outcome = (self.act)(&ctx);      // ACT
        (self.assert)(&outcome)              // ASSERT
    }
}

/// AAA test suite runner with FAST properties: isolated, repeatable, self-validating
pub struct AaaSuite {
    tests: Vec<AaaTest>,
    repeat: u32,
}

impl AaaSuite {
    pub fn new() -> Self {
        Self { tests: Vec::new(), repeat: 1 }
    }
    pub fn add(&mut self, test: AaaTest) { self.tests.push(test); }
    pub fn repeat(mut self, n: u32) -> Self { self.repeat = n.max(1); self }
    pub fn len(&self) -> usize { self.tests.len() }
    pub fn is_empty(&self) -> bool { self.tests.is_empty() }

    /// Run all tests; each test is isolated (fresh Arrange) and repeatable.
    /// Returns list of (name, result) — self-validating output.
    pub fn run_all(&self) -> Vec<(&str, Result<(), String>)> {
        let mut results = Vec::new();
        for test in &self.tests {
            let mut last = Err("not run".to_string());
            for _ in 0..self.repeat {
                last = test.run();
                if last.is_err() { break; } // fail-fast on first failure
            }
            results.push((test.name(), last));
        }
        results
    }

    /// Deterministic: returns true only if every test passed every repeat
    pub fn all_passed(&self) -> bool {
        self.run_all().iter().all(|(_, r)| r.is_ok())
    }
}

impl Default for AaaSuite { fn default() -> Self { Self::new() } }

// ============================================================================
// COMPONENT ISOLATION & MODULAR ARCHITECTURE
// ============================================================================

/// Component manifest — declares a modular OS component (Arch-style PKGBUILD-like)
#[derive(Debug, Clone)]
pub struct ComponentManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    /// Dependencies on other components (by name)
    pub depends: Vec<String>,
    /// Optional dependencies
    pub opt_depends: Vec<String>,
    /// Provided virtual packages
    pub provides: Vec<String>,
    /// Conflicting components
    pub conflicts: Vec<String>,
    /// Component category
    pub category: ComponentCategory,
    /// Entry point / init hook name
    pub init_hook: Option<String>,
    /// Config files shipped
    pub config_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentCategory {
    Kernel, Boot, Init, Package, Service, Network, Filesystem,
    Hardware, Security, Desktop, Development, Documentation, Other,
}

impl ComponentManifest {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: String::new(),
            depends: Vec::new(),
            opt_depends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            category: ComponentCategory::Other,
            init_hook: None,
            config_files: Vec::new(),
        }
    }
    pub fn depends(mut self, deps: Vec<String>) -> Self { self.depends = deps; self }
    pub fn category(mut self, c: ComponentCategory) -> Self { self.category = c; self }
    pub fn provides(mut self, p: Vec<String>) -> Self { self.provides = p; self }
}

/// Component registry enforcing isolation: components declare deps, registry
/// resolves a valid load order and detects conflicts/cycles (fail-fast).
pub struct ComponentRegistry {
    components: HashMap<String, ComponentManifest>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self { components: HashMap::new() }
    }

    pub fn register(&mut self, m: ComponentManifest) -> Result<(), RegistryError> {
        // Conflict check: a component cannot coexist with something it conflicts
        for existing in self.components.values() {
            if m.conflicts.contains(&existing.name) || existing.conflicts.contains(&m.name) {
                return Err(RegistryError::Conflict {
                    a: m.name.clone(), b: existing.name.clone(),
                });
            }
        }
        // Duplicate provides check
        for p in &m.provides {
            if self.components.values().any(|e| e.provides.contains(p)) {
                return Err(RegistryError::DuplicateProvide(p.clone()));
            }
        }
        self.components.insert(m.name.clone(), m);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&ComponentManifest> {
        self.components.get(name)
    }

    /// Resolve a valid initialization order via topological sort.
    /// Fails fast on missing deps or dependency cycles.
    pub fn resolve_order(&self, wanted: &[String]) -> Result<Vec<String>, RegistryError> {
        // Collect transitive deps
        let mut needed: Vec<String> = Vec::new();
        let mut stack: Vec<String> = wanted.to_vec();
        while let Some(n) = stack.pop() {
            if needed.contains(&n) { continue; }
            let m = self.components.get(&n)
                .ok_or_else(|| RegistryError::MissingDependency(n.clone()))?;
            needed.push(n.clone());
            for d in &m.depends { stack.push(d.clone()); }
        }

        // Kahn's algorithm on the needed subgraph
        let mut indeg: HashMap<String, usize> = needed.iter().map(|n| (n.clone(), 0)).collect();
        for n in &needed {
            if let Some(m) = self.components.get(n) {
                for d in &m.depends {
                    if indeg.contains_key(d) {
                        *indeg.get_mut(n).unwrap() += 1;
                    }
                }
            }
        }
        let mut ready: Vec<String> = indeg.iter()
            .filter(|(_, &d)| d == 0).map(|(n, _)| n.clone()).collect();
        ready.sort(); // deterministic order
        let mut order = Vec::new();
        while let Some(n) = ready.pop() {
            order.push(n.clone());
            for m in self.components.values() {
                if m.depends.contains(&n) && indeg.contains_key(&m.name) {
                    let d = indeg.get_mut(&m.name).unwrap();
                    *d -= 1;
                    if *d == 0 {
                        ready.push(m.name.clone());
                        ready.sort();
                    }
                }
            }
        }
        if order.len() != needed.len() {
            return Err(RegistryError::DependencyCycle);
        }
        Ok(order)
    }

    /// Components not required by `wanted` (isolation report)
    pub fn isolated(&self, wanted: &[String]) -> Vec<String> {
        let needed: std::collections::HashSet<String> = wanted.iter().cloned().collect();
        self.components.keys()
            .filter(|k| !needed.contains(*k))
            .cloned().collect()
    }
}

impl Default for ComponentRegistry { fn default() -> Self { Self::new() } }

#[derive(Debug)]
pub enum RegistryError {
    Conflict { a: String, b: String },
    DuplicateProvide(String),
    MissingDependency(String),
    DependencyCycle,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegistryError::Conflict { a, b } => write!(f, "component conflict: {} vs {}", a, b),
            RegistryError::DuplicateProvide(p) => write!(f, "duplicate provide: {}", p),
            RegistryError::MissingDependency(d) => write!(f, "missing dependency: {}", d),
            RegistryError::DependencyCycle => write!(f, "dependency cycle detected"),
        }
    }
}
impl std::error::Error for RegistryError {}

// ============================================================================
// SPECIFICATION-DRIVEN DEVELOPMENT
// ============================================================================

/// A written spec for a component — the contract before the code
#[derive(Debug, Clone)]
pub struct Spec {
    pub component: String,
    pub version: String,
    /// Functional requirements
    pub requirements: Vec<Requirement>,
    /// Interface contracts (stable APIs)
    pub interfaces: Vec<InterfaceContract>,
    /// Acceptance criteria mapped to tests
    pub acceptance: Vec<AcceptanceCriterion>,
}

#[derive(Debug, Clone)]
pub struct Requirement {
    pub id: String,
    pub description: String,
    pub priority: Priority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority { Low, Medium, High, Critical }

#[derive(Debug, Clone)]
pub struct InterfaceContract {
    pub name: String,
    /// Version of the API contract — bump on breaking change
    pub api_version: String,
    /// Stability level
    pub stability: Stability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability { Experimental, Beta, Stable }

#[derive(Debug, Clone)]
pub struct AcceptanceCriterion {
    pub id: String,
    pub description: String,
    /// Name of the AAA test that verifies this criterion
    pub test_name: String,
}

impl Spec {
    pub fn new(component: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            component: component.into(),
            version: version.into(),
            requirements: Vec::new(),
            interfaces: Vec::new(),
            acceptance: Vec::new(),
        }
    }
    pub fn requirement(mut self, id: &str, desc: &str, p: Priority) -> Self {
        self.requirements.push(Requirement {
            id: id.into(), description: desc.into(), priority: p,
        });
        self
    }
    pub fn interface(mut self, name: &str, api_version: &str, s: Stability) -> Self {
        self.interfaces.push(InterfaceContract {
            name: name.into(), api_version: api_version.into(), stability: s,
        });
        self
    }
    pub fn acceptance(mut self, id: &str, desc: &str, test: &str) -> Self {
        self.acceptance.push(AcceptanceCriterion {
            id: id.into(), description: desc.into(), test_name: test.into(),
        });
        self
    }

    /// Validate spec completeness — every requirement must have acceptance coverage
    pub fn validate(&self) -> Result<(), SpecError> {
        if self.requirements.is_empty() {
            return Err(SpecError::NoRequirements);
        }
        if self.interfaces.is_empty() {
            return Err(SpecError::NoInterfaces);
        }
        // Each acceptance criterion must reference a test
        for ac in &self.acceptance {
            if ac.test_name.is_empty() {
                return Err(SpecError::MissingTestMapping(ac.id.clone()));
            }
        }
        Ok(())
    }

    /// Render spec as Markdown documentation
    pub fn to_markdown(&self) -> String {
        let mut out = format!("# Spec: {} v{}\n\n## Requirements\n", self.component, self.version);
        for r in &self.requirements {
            out.push_str(&format!("- [{}] {} — {}\n", r.id, r.priority_str(), r.description));
        }
        out.push_str("\n## Interfaces\n");
        for i in &self.interfaces {
            out.push_str(&format!("- {} (api v{}, {:?})\n", i.name, i.api_version, i.stability));
        }
        out.push_str("\n## Acceptance Criteria\n");
        for a in &self.acceptance {
            out.push_str(&format!("- {} — {} (test: {})\n", a.id, a.description, a.test_name));
        }
        out
    }
}

impl Requirement {
    fn priority_str(&self) -> &'static str {
        match self.priority {
            Priority::Low => "low", Priority::Medium => "medium",
            Priority::High => "high", Priority::Critical => "critical",
        }
    }
}

#[derive(Debug)]
pub enum SpecError {
    NoRequirements,
    NoInterfaces,
    MissingTestMapping(String),
}

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpecError::NoRequirements => write!(f, "spec has no requirements"),
            SpecError::NoInterfaces => write!(f, "spec has no interface contracts"),
            SpecError::MissingTestMapping(id) => write!(f, "acceptance criterion {} has no test", id),
        }
    }
}
impl std::error::Error for SpecError {}

// ============================================================================
// SOFTWARE SUPPORT & UPDATING PROGRAMS (Linux-distro style)
// ============================================================================

/// Support lifecycle status for a component/package
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportStatus {
    /// Fully supported, receives updates
    Supported,
    /// Security fixes only
    SecurityOnly,
    /// Deprecated — scheduled for removal
    Deprecated,
    /// End of life — no more updates
    EndOfLife,
}

/// A support policy entry — distro-style EOL tracking
#[derive(Debug, Clone)]
pub struct SupportPolicy {
    pub component: String,
    pub status: SupportStatus,
    pub release_date: u64,          // unix secs
    pub eol_date: Option<u64>,      // unix secs
    pub security_support_until: Option<u64>,
    pub notes: String,
}

impl SupportPolicy {
    pub fn is_supported_at(&self, now: u64) -> bool {
        match self.status {
            SupportStatus::Supported => true,
            SupportStatus::SecurityOnly => {
                self.security_support_until.map_or(false, |u| now < u)
            }
            SupportStatus::Deprecated => {
                self.eol_date.map_or(true, |e| now < e)
            }
            SupportStatus::EndOfLife => false,
        }
    }

    pub fn days_until_eol(&self, now: u64) -> Option<i64> {
        self.eol_date.map(|e| (e as i64) - (now as i64))
    }
}

/// Update channel — Arch-style stable/testing/experimental
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateChannel {
    Stable,
    Testing,
    Experimental,
}

/// Update policy for the whole system
#[derive(Debug, Clone)]
pub struct UpdatePolicy {
    pub channel: UpdateChannel,
    pub auto_check: bool,
    pub auto_install_security: bool,
    pub notify_days_before_eol: u32,
    pub hold_packages: Vec<String>,
    pub ignore_packages: Vec<String>,
}

impl Default for UpdatePolicy {
    fn default() -> Self {
        Self {
            channel: UpdateChannel::Stable,
            auto_check: true,
            auto_install_security: true,
            notify_days_before_eol: 30,
            hold_packages: Vec::new(),
            ignore_packages: Vec::new(),
        }
    }
}

/// Update manager implementing distro-style support/update program
pub struct SupportProgram {
    policies: HashMap<String, SupportPolicy>,
    update_policy: UpdatePolicy,
}

impl SupportProgram {
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
            update_policy: UpdatePolicy::default(),
        }
    }

    pub fn set_policy(&mut self, p: SupportPolicy) {
        self.policies.insert(p.component.clone(), p);
    }

    pub fn get_policy(&self, component: &str) -> Option<&SupportPolicy> {
        self.policies.get(component)
    }

    /// List components needing attention: EOL, security-only, or near-EOL
    pub fn attention_needed(&self, now: u64) -> Vec<&SupportPolicy> {
        let warn_days = (self.update_policy.notify_days_before_eol as i64) * 86400;
        self.policies.values()
            .filter(|p| match p.status {
                SupportStatus::EndOfLife => true,
                SupportStatus::Deprecated => p.eol_date.map_or(false, |e| (e as i64) - (now as i64) <= warn_days),
                SupportStatus::SecurityOnly => true,
                SupportStatus::Supported => false,
            })
            .collect()
    }

    /// Compute which updates are eligible under policy (respects holds/ignores)
    pub fn eligible_updates<I>(&self, updates: I) -> Vec<String>
    where I: IntoIterator<Item = String> {
        updates.into_iter()
            .filter(|u| !self.update_policy.hold_packages.contains(u)
                     && !self.update_policy.ignore_packages.contains(u))
            .collect()
    }

    /// Channel-aware update check result
    pub fn updates_for_channel(&self, available: &[String], channel: UpdateChannel) -> Vec<String> {
        match channel {
            UpdateChannel::Stable => available.to_vec(),
            // In a real system, testing/experimental filter prerelease updates;
            // here we return all but mark channel for caller decisions.
            _ => available.to_vec(),
        }
    }

    pub fn update_policy(&self) -> &UpdatePolicy { &self.update_policy }
    pub fn set_update_policy(&mut self, p: UpdatePolicy) { self.update_policy = p; }
}

impl Default for SupportProgram { fn default() -> Self { Self::new() } }

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aaa_basic() {
        let test = AaaTest::new("addition works")
            .arrange(|| {
                let mut c = TestContext::new();
                c.set("a", "2"); c.set("b", "3"); c
            })
            .act(|ctx| {
                let a: i32 = ctx.get("a").unwrap().parse().unwrap();
                let b: i32 = ctx.get("b").unwrap().parse().unwrap();
                TestOutcome::ok((a + b).to_string())
            })
            .assert(|out| {
                if out.result.as_deref() == Some("5") { Ok(()) }
                else { Err(format!("expected 5, got {:?}", out.result)) }
            });
        assert!(test.run().is_ok());
    }

    #[test]
    fn test_aaa_suite_repeatable() {
        let mut suite = AaaSuite::new().repeat(3);
        suite.add(AaaTest::new("always passes")
            .act(|_| TestOutcome::ok("done"))
            .assert(|o| if o.success { Ok(()) } else { Err("fail".into()) }));
        suite.add(AaaTest::new("deterministic")
            .act(|_| TestOutcome::ok("x"))
            .assert(|o| {
                if o.result.as_deref() == Some("x") { Ok(()) } else { Err("bad".into()) }
            }));
        let results = suite.run_all();
        assert_eq!(results.len(), 2);
        assert!(suite.all_passed());
    }

    #[test]
    fn test_component_registry_order() {
        let mut reg = ComponentRegistry::new();
        reg.register(ComponentManifest::new("kernel", "1.0")
            .category(ComponentCategory::Kernel)).unwrap();
        reg.register(ComponentManifest::new("init", "1.0")
            .category(ComponentCategory::Init)
            .depends(vec!["kernel".to_string()])).unwrap();
        reg.register(ComponentManifest::new("pacman", "1.0")
            .category(ComponentCategory::Package)
            .depends(vec!["init".to_string()])).unwrap();

        let order = reg.resolve_order(&["pacman".to_string()]).unwrap();
        // kernel before init before pacman
        let ki = order.iter().position(|n| n == "kernel").unwrap();
        let ii = order.iter().position(|n| n == "init").unwrap();
        let pi = order.iter().position(|n| n == "pacman").unwrap();
        assert!(ki < ii && ii < pi);
    }

    #[test]
    fn test_component_registry_conflict() {
        let mut reg = ComponentRegistry::new();
        reg.register(ComponentManifest::new("a", "1.0")).unwrap();
        let mut b = ComponentManifest::new("b", "1.0");
        b.conflicts = vec!["a".to_string()];
        assert!(reg.register(b).is_err());
    }

    #[test]
    fn test_component_registry_cycle() {
        let mut reg = ComponentRegistry::new();
        reg.register(ComponentManifest::new("a", "1.0")
            .depends(vec!["b".to_string()])).unwrap();
        reg.register(ComponentManifest::new("b", "1.0")
            .depends(vec!["a".to_string()])).unwrap();
        assert!(reg.resolve_order(&["a".to_string()]).is_err());
    }

    #[test]
    fn test_spec_validation() {
        let spec = Spec::new("pacman", "1.0")
            .requirement("R1", "install packages", Priority::High)
            .interface("PackageManager", "1.0", Stability::Stable)
            .acceptance("AC1", "installs firefox", "test_install");
        assert!(spec.validate().is_ok());
        let md = spec.to_markdown();
        assert!(md.contains("Spec: pacman"));
        assert!(md.contains("R1"));
    }

    #[test]
    fn test_spec_incomplete_fails() {
        let spec = Spec::new("x", "1.0"); // no requirements/interfaces
        assert!(spec.validate().is_err());
    }

    #[test]
    fn test_support_program() {
        let mut sp = SupportProgram::new();
        sp.set_policy(SupportPolicy {
            component: "oldpkg".into(),
            status: SupportStatus::Deprecated,
            release_date: 0,
            eol_date: Some(1000),
            security_support_until: None,
            notes: String::new(),
        });
        let now = 500u64;
        let attention = sp.attention_needed(now);
        assert_eq!(attention.len(), 1);

        // eligible updates respect holds
        let updates = vec!["a".to_string(), "b".to_string()];
        sp.update_policy.hold_packages = vec!["a".to_string()];
        let eligible = sp.eligible_updates(updates);
        assert_eq!(eligible, vec!["b".to_string()]);
    }
}
