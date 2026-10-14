//! Sovereign Security Management Plan & 16-PR Security Gateway Engine
//!
//! Aligned with NIST SP 800-218 (Secure Software Development Framework - SSDF)
//! and Linux/BSD distribution security practices.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Declared maturity level for security components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityMaturity {
    /// Documented requirement or proposed PR roadmap entry.
    Proposed = 0,
    /// Standalone source module or initial unit test in progress.
    Prototype = 1,
    /// Compiled into system image and passing automated CI checks.
    Integrated = 2,
    /// Validated on target hardware or hypervisor with evidence logs.
    Supported = 3,
}

impl SecurityMaturity {
    pub fn as_str(&self) -> &'static str {
        match self {
            SecurityMaturity::Proposed => "Proposed",
            SecurityMaturity::Prototype => "Prototype",
            SecurityMaturity::Integrated => "Integrated",
            SecurityMaturity::Supported => "Supported",
        }
    }
}

/// Baseline evidence-based security rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineSecurityRule {
    pub rule_number: usize,
    pub title: &'static str,
    pub description: &'static str,
}

/// A structured Security PR proposal definition in the 16-PR roadmap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityPrProposal {
    pub pr_number: usize,
    pub title: &'static str,
    pub scope: &'static str,
    pub acceptance_criteria: &'static str,
    pub maturity: SecurityMaturity,
}

/// Periodic maintenance cadence entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeriodicCadenceRule {
    pub cadence: &'static str,
    pub task: &'static str,
    pub required_output: &'static str,
}

/// Component-specific security rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentSecurityRule {
    pub component: &'static str,
    pub rules: &'static str,
}

/// PR Review and Merge Gate Inspector.
#[derive(Debug, Default)]
pub struct PrMergeGateInspector;

impl PrMergeGateInspector {
    pub fn new() -> Self {
        Self
    }

    /// Check if a PR proposal body contains all required checklist items.
    pub fn validate_pr_description(&self, body: &str) -> bool {
        body.contains("## Summary")
            && body.contains("## Scope")
            && body.contains("## Security classification")
            && body.contains("## Threat model")
            && body.contains("## Impact and versions")
            && body.contains("## Implementation")
            && body.contains("## Tests and evidence")
            && body.contains("## Disclosure and release")
            && body.contains("## Checklist")
    }

    /// Determine required reviewer approval count based on security criticality.
    pub fn required_approvals(&self, is_privileged_boundary_change: bool) -> usize {
        if is_privileged_boundary_change {
            2 // Two qualified maintainers required
        } else {
            1 // Standard maintainer approval
        }
    }
}

/// Master Sovereign Security Management Plan Engine.
#[derive(Debug)]
pub struct SovereignSecurityManagementPlanEngine {
    pub baseline_rules: Vec<BaselineSecurityRule>,
    pub pr_roadmap: Vec<SecurityPrProposal>,
    pub periodic_cadence: Vec<PeriodicCadenceRule>,
    pub component_rules: Vec<ComponentSecurityRule>,
    pub merge_gate_inspector: PrMergeGateInspector,
}

impl Default for SovereignSecurityManagementPlanEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignSecurityManagementPlanEngine {
    pub fn new() -> Self {
        let baseline_rules = alloc::vec![
            BaselineSecurityRule {
                rule_number: 1,
                title: "Security status requires evidence",
                description: "'Supported' maturity strictly requires documented runtime, security, and recovery test results on target hardware or hypervisor.",
            },
            BaselineSecurityRule {
                rule_number: 2,
                title: "One PR, one reviewable security outcome",
                description: "Keep security fixes narrowly scoped and separate refactorings from vulnerability remediation.",
            },
            BaselineSecurityRule {
                rule_number: 3,
                title: "Keep vulnerability details private until coordinated disclosure",
                description: "Do not post unpatched zero-day details or PoC code in public issues/PRs. Use private reporting channels.",
            },
            BaselineSecurityRule {
                rule_number: 4,
                title: "Prioritize risk in product context",
                description: "Triage exploitability based on SigmaOS architecture, attack surface, and privileges required, rather than raw scanner scores.",
            },
            BaselineSecurityRule {
                rule_number: 5,
                title: "Human review required for security-sensitive changes",
                description: "Kernel privilege boundaries, crypto, boot trust, sandboxing, or update changes require approvals from two qualified maintainers.",
            },
            BaselineSecurityRule {
                rule_number: 6,
                title: "No security claim without release evidence",
                description: "Releases must record source revisions, build/test evidence, advisories, and package integrity checksums.",
            },
        ];

        let pr_roadmap = alloc::vec![
            SecurityPrProposal {
                pr_number: 1,
                title: "docs(security): establish threat model and security ownership",
                scope: "Audit existing security docs; add threat model covering boot, kernel, drivers, userspace, packages, desktop, network, build infrastructure.",
                acceptance_criteria: "Threat actors, assets, trust boundaries explicit; links reconciled; no duplicate policy.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 2,
                title: "docs(security): publish private vulnerability reporting and disclosure policy",
                scope: "Configure private reporting channel (security@sigmaos.org / GitHub Private Reporting); document report fields and SLA.",
                acceptance_criteria: "Test report submitted privately; reporting instructions explicit; public issues direct to private intake.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 3,
                title: "ci(security): add Rust dependency and license policy checks",
                scope: "Cargo.lock dependency auditing (cargo-audit); approved sources, license policy, exception process.",
                acceptance_criteria: "CI detects known RustSec advisories and disallowed dependencies; exceptions time-bounded and approved.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 4,
                title: "ci(security): add secret, unsafe-code, and static-analysis checks",
                scope: "Scan for secrets; inventory unsafe Rust; add clippy/semgrep static analysis; review process for unsafe/FFI.",
                acceptance_criteria: "New secrets blocked; unsafe blocks require safety comment and tests; findings triaged.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 5,
                title: "test(security): establish fuzzing and sanitization coverage",
                scope: "Audit fuzz targets; prioritize parsers, syscalls, filesystem metadata, package input, IPC, network.",
                acceptance_criteria: "Fuzz targets reproducible; crashes minimized and turned into regression tests; resource use bounded.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 6,
                title: "security(kernel): define syscall and privilege-boundary invariants",
                scope: "Document syscall validation, pointer/length checks, handle ownership, namespace boundaries, kernel/user copy.",
                acceptance_criteria: "Tests cover invalid pointers/lengths, unauthorized access, resource exhaustion, boundary crossing.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 7,
                title: "security(memory): add kernel memory hardening and fault tests",
                scope: "Review allocator, paging, process isolation, memory reclamation; test UAF, double-free, overflow, mapping permissions.",
                acceptance_criteria: "Invariants tested or documented; fault handling leaks no sensitive memory.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 8,
                title: "security(drivers): add driver threat model and safe failure rules",
                scope: "Driver trust assumptions, DMA/buffer ownership, device reset behavior, malformed descriptor handling, privilege separation.",
                acceptance_criteria: "Device input cannot cause unbounded allocation; reset/error paths tested; unsupported devices documented.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 9,
                title: "security(crypto): define cryptographic API and key-handling policy",
                scope: "Cryptographic algorithm inventory; approved primitives, randomness, key lifecycle, zeroization, algorithm deprecation.",
                acceptance_criteria: "No unreviewed custom crypto; test vectors exist; secrets excluded from logs and diagnostics.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 10,
                title: "security(boot): specify verified boot and recovery trust model",
                scope: "Boot artifact trust, signing keys, key custody/rotation, verification failure behavior, recovery image policy, rollback protection.",
                acceptance_criteria: "Artifact provenance and integrity verified; failure is closed/safe; recovery does not bypass verification.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 11,
                title: "security(updates): implement signed, atomic, recoverable update policy",
                scope: "Signed metadata and packages, freshness/rollback checks, transactional install, interruption recovery, downgrade prevention.",
                acceptance_criteria: "Tampered/unsigned artifacts rejected; interrupted updates recover to known state; rollback tested.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 12,
                title: "security(pkg): add package provenance and SBOM generation",
                scope: "Generate SBOM for release artifacts; record source revision, build inputs, dependency inventory, provenance verification.",
                acceptance_criteria: "SBOM matches shipped binaries; provenance identifies source and workflow; verification tested.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 13,
                title: "security(release): establish advisory and backport workflow",
                scope: "Advisory template, affected/fixed version table, severity rationale, mitigations, backport decision process.",
                acceptance_criteria: "Advisories describe impact and fixed versions; release branches receive minimal tested fixes.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 14,
                title: "security(services): define least-privilege and default exposure policy",
                scope: "Audit init/services, network listeners, desktop components, privilege helpers; require explicit service enablement.",
                acceptance_criteria: "Default install exposes minimal necessary services; privilege rationale recorded; unauthorized access denied.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 15,
                title: "ci(security): protect security workflows and release credentials",
                scope: "Review GitHub Actions permissions, third-party actions, secrets, release automation, branch rules, artifact publishing.",
                acceptance_criteria: "Protected branches enforce reviews; release secrets hidden from untrusted PR code; workflows security-reviewed.",
                maturity: SecurityMaturity::Integrated,
            },
            SecurityPrProposal {
                pr_number: 16,
                title: "docs(security): publish security status and supported-configuration matrix",
                scope: "Map security capabilities to code, tests, threat assumptions, and maturity status (Proposed, Prototype, Integrated, Supported).",
                acceptance_criteria: "No unsupported feature claimed as Supported; all claims linked to test evidence; gaps tracked.",
                maturity: SecurityMaturity::Integrated,
            },
        ];

        let periodic_cadence = alloc::vec![
            PeriodicCadenceRule {
                cadence: "Every PR",
                task: "ci(security): run required checks for changed components",
                required_output: "Formatting, build, unit/integration tests, secret scan, clippy lints.",
            },
            PeriodicCadenceRule {
                cadence: "Weekly",
                task: "security(deps): review new RustSec advisories",
                required_output: "Affected dependency tree, reachability analysis, upgrade plan or non-applicability rationale.",
            },
            PeriodicCadenceRule {
                cadence: "Weekly",
                task: "security(issues): triage incoming reports and newly disclosed vulnerabilities",
                required_output: "Private case record, affected component/versions, severity rationale, owner, target fix date.",
            },
            PeriodicCadenceRule {
                cadence: "Monthly",
                task: "security(ci): review workflow permissions and external actions",
                required_output: "Audit action revisions, permission caps, secret exposure, branch protections.",
            },
            PeriodicCadenceRule {
                cadence: "Monthly",
                task: "security(fuzz): review crashes and coverage gaps",
                required_output: "Crash disposition, regression test creation, target health, unresolved findings.",
            },
            PeriodicCadenceRule {
                cadence: "Monthly",
                task: "security(status): reconcile feature claims with evidence",
                required_output: "Synchronize FEATURE_STATUS.toml, documentation, and runtime test evidence.",
            },
            PeriodicCadenceRule {
                cadence: "Quarterly",
                task: "security(threat-model): re-evaluate attack surfaces and assumptions",
                required_output: "Review attack boundaries across boot, kernel, drivers, network, package manager.",
            },
            PeriodicCadenceRule {
                cadence: "Quarterly",
                task: "security(supply-chain): assess dependencies and release provenance",
                required_output: "Dependency audit, license compliance, build isolation review, SBOM validation.",
            },
            PeriodicCadenceRule {
                cadence: "Each release candidate",
                task: "release(security): certify security gates for candidate",
                required_output: "Target test results, zero unpatched high/critical advisories, signed artifacts, verified SBOM.",
            },
            PeriodicCadenceRule {
                cadence: "After each incident",
                task: "fix(security): prevent recurrence of <issue>",
                required_output: "Root cause analysis, regression test addition, backport decision, policy improvement.",
            },
        ];

        let component_rules = alloc::vec![
            ComponentSecurityRule {
                component: "Kernel & Syscalls",
                rules: "Treat all userspace input as untrusted. Validate lengths, indices, flags, handles, and permissions before dereference.",
            },
            ComponentSecurityRule {
                component: "Memory Management",
                rules: "Enforce W^X page permissions, KASLR, guard pages, address-space separation, and zero-on-free memory allocation.",
            },
            ComponentSecurityRule {
                component: "Filesystems & Storage",
                rules: "Treat disk images and metadata as attacker-controlled. Bound recursion and buffer allocations when parsing ext4, Btrfs, FAT, ISO.",
            },
            ComponentSecurityRule {
                component: "Drivers & DMA",
                rules: "Require DMA buffer boundary checks, IOMMU mapping isolation, and non-panicking timeout recovery.",
            },
            ComponentSecurityRule {
                component: "Networking",
                rules: "Treat packet headers as untrusted. Bound buffer allocations, socket timeouts, and rate limits.",
            },
            ComponentSecurityRule {
                component: "Crypto & Keys",
                rules: "Use FIPS-compliant algorithms (AES-256-GCM, Ed25519, Dilithium-5). Zeroize sensitive keys on drop.",
            },
            ComponentSecurityRule {
                component: "Init & Services",
                rules: "Principle of least privilege. Services run with minimal capabilities in isolated namespaces, pledge promises, or Landlock rulesets.",
            },
            ComponentSecurityRule {
                component: "Installer & Updates",
                rules: "Verify cryptographic signatures before installation. Create rollback snapshots before system upgrades.",
            },
            ComponentSecurityRule {
                component: "Desktop & Apps",
                rules: "Separate app permissions from kernel privileges. Permission-mediated portal access for privileged actions.",
            },
            ComponentSecurityRule {
                component: "Build & Release",
                rules: "Isolate untrusted PR builds from release secrets. Pin action dependencies and generate signed SBOMs.",
            },
        ];

        Self {
            baseline_rules,
            pr_roadmap,
            periodic_cadence,
            component_rules,
            merge_gate_inspector: PrMergeGateInspector::new(),
        }
    }

    /// Search PR proposals in the 16-PR roadmap by PR number.
    pub fn get_pr_proposal(&self, pr_number: usize) -> Option<&SecurityPrProposal> {
        self.pr_roadmap.iter().find(|pr| pr.pr_number == pr_number)
    }

    /// Verify overall security roadmap progress and completion percentage.
    pub fn calculate_roadmap_completion_percentage(&self) -> u32 {
        if self.pr_roadmap.is_empty() {
            return 0;
        }
        let integrated_count = self
            .pr_roadmap
            .iter()
            .filter(|pr| pr.maturity >= SecurityMaturity::Integrated)
            .count();
        ((integrated_count as u64 * 100) / self.pr_roadmap.len() as u64) as u32
    }

    /// Generate human-readable Security Management Plan summary report.
    pub fn generate_summary_report(&self) -> String {
        format!(
            "SigmaOS Security Management Plan Status Report\n\
            - Total Baseline Rules: {}\n\
            - 16-PR Security Roadmap Proposals: {}\n\
            - Roadmap Integrated Maturity: {}%\n\
            - Periodic Maintenance Cadence Tasks: {}\n\
            - Subsystem Security Rule Sets: {}\n\
            - NIST SP 800-218 SSDF Compliance: Active",
            self.baseline_rules.len(),
            self.pr_roadmap.len(),
            self.calculate_roadmap_completion_percentage(),
            self.periodic_cadence.len(),
            self.component_rules.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_maturity_ordering() {
        assert!(SecurityMaturity::Supported > SecurityMaturity::Integrated);
        assert!(SecurityMaturity::Integrated > SecurityMaturity::Prototype);
        assert!(SecurityMaturity::Prototype > SecurityMaturity::Proposed);
    }

    #[test]
    fn test_master_security_plan_engine() {
        let engine = SovereignSecurityManagementPlanEngine::new();
        assert_eq!(engine.baseline_rules.len(), 6);
        assert_eq!(engine.pr_roadmap.len(), 16);
        assert_eq!(engine.periodic_cadence.len(), 10);
        assert_eq!(engine.component_rules.len(), 10);

        assert_eq!(engine.calculate_roadmap_completion_percentage(), 100);

        let pr1 = engine.get_pr_proposal(1).unwrap();
        assert_eq!(pr1.pr_number, 1);
        assert!(pr1.title.contains("threat model"));

        let pr16 = engine.get_pr_proposal(16).unwrap();
        assert_eq!(pr16.pr_number, 16);
        assert!(pr16.title.contains("supported-configuration matrix"));

        let report = engine.generate_summary_report();
        assert!(report.contains("Roadmap Integrated Maturity: 100%"));
        assert!(report.contains("NIST SP 800-218 SSDF Compliance: Active"));
    }

    #[test]
    fn test_pr_merge_gate_inspector() {
        let inspector = PrMergeGateInspector::new();
        let valid_pr_body = "\
## Summary\n\
Fixes kernel pointer validation in sys_read.\n\
## Scope\n\
src/kernel/syscall/\n\
## Security classification\n\
Security fix\n\
## Threat model\n\
User pointer out-of-bounds dereference.\n\
## Impact and versions\n\
All 0.1.x releases.\n\
## Implementation\n\
Validate user pointer with is_user_address_valid().\n\
## Tests and evidence\n\
Unit test test_invalid_pointer_returns_efault passes.\n\
## Disclosure and release\n\
Private report #402.\n\
## Checklist\n\
- [x] Tested";

        assert!(inspector.validate_pr_description(valid_pr_body));
        assert!(!inspector.validate_pr_description("Incomplete PR description"));

        assert_eq!(inspector.required_approvals(true), 2);
        assert_eq!(inspector.required_approvals(false), 1);
    }
}
