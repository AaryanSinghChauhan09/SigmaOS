use std::string::{String, ToString};
use std::vec::Vec;

/// 12 Maintenance Roles in the Operating Model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceRole {
    Maintainer,
    Updater,
    Reviewer,
    Consolidator,
    Researcher,
    Triager,
    TesterFuzzer,
    SecurityOfficer,
    ReleaseManager,
    DocsSteward,
    Benchmarker,
    CommunityManager,
}

/// Leadership profile for agent/human execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeadershipProfile {
    HumanLedAgentAssisted,
    MostlyAgent,
    AgentFirstPassHumanApproval,
    AgentDraftsHumanDecides,
    AgentAutomatedHumanHardware,
    HumanLed,
}

/// Definition of an operating role
#[derive(Debug, Clone)]
pub struct RoleDefinition {
    pub role: MaintenanceRole,
    pub title: String,
    pub mission: String,
    pub cadence: String,
    pub leadership: LeadershipProfile,
}

/// 4 Roadmap Phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadmapPhase {
    Phase0GroundTruth,
    Phase1RealHardwareBoot,
    Phase2UsabilityAndDogfooding,
    Phase3WinTheWedge,
}

/// Progress state for a roadmap phase
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RoadmapPhaseStatus {
    pub phase: RoadmapPhase,
    pub name: String,
    pub timeframe: String,
    pub is_complete: bool,
    pub completion_pct: f64,
    pub evidence_link: Option<String>,
}

/// Execution step in the 8-Step Execution Guide
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ExecutionStep {
    pub step_number: u8,
    pub title: String,
    pub timeframe: String,
    pub action: String,
    pub required_output: String,
    pub is_verified: bool,
}

/// Weekly Calendar Day Focus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

/// Calendar Dispatch Assignment
#[derive(Debug, Clone)]
pub struct OperationalDayFocus {
    pub day: CalendarDay,
    pub focus_area: String,
    pub assigned_roles: Vec<MaintenanceRole>,
}

/// 7 Governance Guardrails
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum GovernanceGuardrail {
    HumanApprovalRequired,
    EvidenceGatedSuccess,
    SandboxingAndLeastPrivilege,
    SmallReviewablePrs,
    RegressionRatchet,
    AuditLogAndRollback,
    HonestPublicClaims,
}

/// Validation result for guardrail checks
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GuardrailVerdict {
    pub guardrail: GovernanceGuardrail,
    pub passed: bool,
    pub reason: String,
}

/// Master Role-Based Operating Plan Engine
pub struct RoleBasedOperatingPlanEngine {
    pub roles: Vec<RoleDefinition>,
    pub phases: Vec<RoadmapPhaseStatus>,
    pub execution_steps: Vec<ExecutionStep>,
    pub calendar: Vec<OperationalDayFocus>,
}

impl RoleBasedOperatingPlanEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            roles: Vec::new(),
            phases: Vec::new(),
            execution_steps: Vec::new(),
            calendar: Vec::new(),
        };

        engine.init_roles();
        engine.init_phases();
        engine.init_execution_steps();
        engine.init_calendar();

        engine
    }

    fn init_roles(&mut self) {
        self.roles = vec![
            RoleDefinition {
                role: MaintenanceRole::Maintainer,
                title: "Maintainer".to_string(),
                mission: "Owns subsystem health, reviews PRs, and manages backlog".to_string(),
                cadence: "Daily / Weekly".to_string(),
                leadership: LeadershipProfile::HumanLedAgentAssisted,
            },
            RoleDefinition {
                role: MaintenanceRole::Updater,
                title: "Updater".to_string(),
                mission: "Keeps dependencies, toolchain, and upstream knowledge current"
                    .to_string(),
                cadence: "Daily / Weekly".to_string(),
                leadership: LeadershipProfile::MostlyAgent,
            },
            RoleDefinition {
                role: MaintenanceRole::Reviewer,
                title: "Reviewer".to_string(),
                mission: "Checks every change for correctness, security, and tests".to_string(),
                cadence: "Per PR".to_string(),
                leadership: LeadershipProfile::AgentFirstPassHumanApproval,
            },
            RoleDefinition {
                role: MaintenanceRole::Consolidator,
                title: "Consolidator".to_string(),
                mission: "Removes duplication, dead code, and stale documentation".to_string(),
                cadence: "Weekly / Monthly".to_string(),
                leadership: LeadershipProfile::MostlyAgent,
            },
            RoleDefinition {
                role: MaintenanceRole::Researcher,
                title: "Researcher".to_string(),
                mission: "Studies Linux/BSD reference systems and writes design notes".to_string(),
                cadence: "Weekly / Monthly".to_string(),
                leadership: LeadershipProfile::AgentDraftsHumanDecides,
            },
            RoleDefinition {
                role: MaintenanceRole::Triager,
                title: "Triager".to_string(),
                mission: "Sorts issues, reproduces bugs, and assigns severity".to_string(),
                cadence: "Daily".to_string(),
                leadership: LeadershipProfile::MostlyAgent,
            },
            RoleDefinition {
                role: MaintenanceRole::TesterFuzzer,
                title: "Tester / Fuzzer".to_string(),
                mission: "Runs test suites, fuzz campaigns, and hardware boot checks".to_string(),
                cadence: "Daily / Nightly".to_string(),
                leadership: LeadershipProfile::AgentAutomatedHumanHardware,
            },
            RoleDefinition {
                role: MaintenanceRole::SecurityOfficer,
                title: "Security Officer".to_string(),
                mission: "Handles vulnerabilities, unsafe audits, and key security".to_string(),
                cadence: "Daily / Monthly".to_string(),
                leadership: LeadershipProfile::HumanLed,
            },
            RoleDefinition {
                role: MaintenanceRole::ReleaseManager,
                title: "Release Manager".to_string(),
                mission: "Builds release ISOs, verifies reproducible builds, and ships releases"
                    .to_string(),
                cadence: "Per Release".to_string(),
                leadership: LeadershipProfile::HumanLedAgentAssisted,
            },
            RoleDefinition {
                role: MaintenanceRole::DocsSteward,
                title: "Docs Steward".to_string(),
                mission: "Keeps documentation and status accurate with CI evidence".to_string(),
                cadence: "Weekly".to_string(),
                leadership: LeadershipProfile::AgentDraftsHumanDecides,
            },
            RoleDefinition {
                role: MaintenanceRole::Benchmarker,
                title: "Benchmarker".to_string(),
                mission: "Runs comparative benchmarks against Linux/BSD distros".to_string(),
                cadence: "Weekly / Monthly".to_string(),
                leadership: LeadershipProfile::MostlyAgent,
            },
            RoleDefinition {
                role: MaintenanceRole::CommunityManager,
                title: "Community Manager".to_string(),
                mission: "Handles contributor onboarding, communication, and updates".to_string(),
                cadence: "Daily / Monthly".to_string(),
                leadership: LeadershipProfile::HumanLed,
            },
        ];
    }

    fn init_phases(&mut self) {
        self.phases = vec![
            RoadmapPhaseStatus {
                phase: RoadmapPhase::Phase0GroundTruth,
                name: "Ground Truth".to_string(),
                timeframe: "Weeks 1-4".to_string(),
                is_complete: true,
                completion_pct: 100.0,
                evidence_link: Some("reports/capability/truth_table.json".to_string()),
            },
            RoadmapPhaseStatus {
                phase: RoadmapPhase::Phase1RealHardwareBoot,
                name: "Real Hardware Boot".to_string(),
                timeframe: "Months 1-3".to_string(),
                is_complete: false,
                completion_pct: 60.0,
                evidence_link: None,
            },
            RoadmapPhaseStatus {
                phase: RoadmapPhase::Phase2UsabilityAndDogfooding,
                name: "Usability & Dogfooding".to_string(),
                timeframe: "Months 3-6".to_string(),
                is_complete: false,
                completion_pct: 25.0,
                evidence_link: None,
            },
            RoadmapPhaseStatus {
                phase: RoadmapPhase::Phase3WinTheWedge,
                name: "Win the Wedge".to_string(),
                timeframe: "Months 6-12".to_string(),
                is_complete: false,
                completion_pct: 10.0,
                evidence_link: None,
            },
        ];
    }

    fn init_execution_steps(&mut self) {
        self.execution_steps = vec![
            ExecutionStep {
                step_number: 1,
                title: "Decide System Target".to_string(),
                timeframe: "Week 1".to_string(),
                action: "Select target user profile, license model, and primary wedge areas".to_string(),
                required_output: "VISION.md".to_string(),
                is_verified: true,
            },
            ExecutionStep {
                step_number: 2,
                title: "Ground Truth & Cleanup".to_string(),
                timeframe: "Weeks 1-3".to_string(),
                action: "Consolidate documentation, set up QEMU boot CI, and update status feeds".to_string(),
                required_output: "FEATURE_STATUS.toml".to_string(),
                is_verified: true,
            },
            ExecutionStep {
                step_number: 3,
                title: "Assign Study Areas".to_string(),
                timeframe: "Week 2".to_string(),
                action: "Assign reference OS study areas to leads and configure subsystem owners".to_string(),
                required_output: "GOVERNANCE.md".to_string(),
                is_verified: true,
            },
            ExecutionStep {
                step_number: 4,
                title: "Subsystem Design Notes".to_string(),
                timeframe: "Weeks 2-6".to_string(),
                action: "Study reference OS code, write one-page design notes, and file backlog issues".to_string(),
                required_output: "Design notes & Backlog".to_string(),
                is_verified: true,
            },
            ExecutionStep {
                step_number: 5,
                title: "Address Kernel & Driver Gaps".to_string(),
                timeframe: "Months 1-3".to_string(),
                action: "Implement user-mode transitions, NVMe/AHCI MMIO, VirtIO NIC, and ACPI parsing".to_string(),
                required_output: "Hardware Compatibility List".to_string(),
                is_verified: false,
            },
            ExecutionStep {
                step_number: 6,
                title: "Day-to-Day Usability".to_string(),
                timeframe: "Months 3-6".to_string(),
                action: "Build installer, persistent storage, signed package repos, and dogfood daily".to_string(),
                required_output: "Dogfooding Feedback".to_string(),
                is_verified: false,
            },
            ExecutionStep {
                step_number: 7,
                title: "Scoreboard & Fuzzing".to_string(),
                timeframe: "Months 6-12".to_string(),
                action: "Run comparative benchmarks against Linux/BSD, continuous fuzzing, and audits".to_string(),
                required_output: "Monthly Scoreboard Report".to_string(),
                is_verified: false,
            },
            ExecutionStep {
                step_number: 8,
                title: "Community & Release Cadence".to_string(),
                timeframe: "Ongoing".to_string(),
                action: "Maintain CONTRIBUTING.md, issue triage, quarterly releases, and roadmap updates".to_string(),
                required_output: "Release Notes & Roadmap".to_string(),
                is_verified: false,
            },
        ];
    }

    fn init_calendar(&mut self) {
        self.calendar = vec![
            OperationalDayFocus {
                day: CalendarDay::Monday,
                focus_area: "Triage sweep, planning, and upstream advisory digest".to_string(),
                assigned_roles: vec![
                    MaintenanceRole::Triager,
                    MaintenanceRole::Updater,
                    MaintenanceRole::Maintainer,
                ],
            },
            OperationalDayFocus {
                day: CalendarDay::Tuesday,
                focus_area: "Active subsystem development and code review".to_string(),
                assigned_roles: vec![
                    MaintenanceRole::Maintainer,
                    MaintenanceRole::Reviewer,
                    MaintenanceRole::Researcher,
                ],
            },
            OperationalDayFocus {
                day: CalendarDay::Wednesday,
                focus_area: "Security review, unsafe audit, and researcher design notes"
                    .to_string(),
                assigned_roles: vec![
                    MaintenanceRole::SecurityOfficer,
                    MaintenanceRole::Researcher,
                    MaintenanceRole::Reviewer,
                ],
            },
            OperationalDayFocus {
                day: CalendarDay::Thursday,
                focus_area: "Consolidation sweep, doc sync, and benchmark performance run"
                    .to_string(),
                assigned_roles: vec![
                    MaintenanceRole::Consolidator,
                    MaintenanceRole::DocsSteward,
                    MaintenanceRole::Benchmarker,
                ],
            },
            OperationalDayFocus {
                day: CalendarDay::Friday,
                focus_area:
                    "Release candidate verification, weekly health report, and retrospective"
                        .to_string(),
                assigned_roles: vec![
                    MaintenanceRole::ReleaseManager,
                    MaintenanceRole::TesterFuzzer,
                    MaintenanceRole::CommunityManager,
                ],
            },
        ];
    }

    /// Evaluate whether a proposed PR complies with governance guardrails
    pub fn validate_pr_guardrails(
        &self,
        touches_kernel_or_crypto: bool,
        has_human_approval: bool,
        has_ci_evidence: bool,
        has_regression_test: bool,
    ) -> Vec<GuardrailVerdict> {
        let mut verdicts = Vec::new();

        // Guardrail 1: Human Approval for Kernel/Crypto/Boot
        if touches_kernel_or_crypto {
            verdicts.push(GuardrailVerdict {
                guardrail: GovernanceGuardrail::HumanApprovalRequired,
                passed: has_human_approval,
                reason: if has_human_approval {
                    "Human approval present for core subsystem modification".to_string()
                } else {
                    "Kernel/crypto modifications require human maintainer signoff".to_string()
                },
            });
        } else {
            verdicts.push(GuardrailVerdict {
                guardrail: GovernanceGuardrail::HumanApprovalRequired,
                passed: true,
                reason: "Subsystem modification does not require mandatory human gate".to_string(),
            });
        }

        // Guardrail 2: Evidence-Gated Success
        verdicts.push(GuardrailVerdict {
            guardrail: GovernanceGuardrail::EvidenceGatedSuccess,
            passed: has_ci_evidence,
            reason: if has_ci_evidence {
                "Linked CI evidence artifact verified".to_string()
            } else {
                "Feature cannot be marked complete without linked CI evidence".to_string()
            },
        });

        // Guardrail 5: Regression Ratchet
        verdicts.push(GuardrailVerdict {
            guardrail: GovernanceGuardrail::RegressionRatchet,
            passed: has_regression_test,
            reason: if has_regression_test {
                "Regression test included with modification".to_string()
            } else {
                "Code modification must include automated regression coverage".to_string()
            },
        });

        verdicts
    }

    /// Retrieve the role definition for a given role
    pub fn get_role_definition(&self, role: MaintenanceRole) -> Option<&RoleDefinition> {
        self.roles.iter().find(|r| r.role == role)
    }

    /// Retrieve focus details for a specific day of the week
    pub fn get_day_focus(&self, day: CalendarDay) -> Option<&OperationalDayFocus> {
        self.calendar.iter().find(|c| c.day == day)
    }
}

impl Default for RoleBasedOperatingPlanEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// PR Specification for the Role-Based Operating Plan & Strategy Proposal
#[derive(Debug, Clone)]
pub struct RoleBasedOperatingPlanPrSpec {
    pub pr_title: String,
    pub pr_branch: String,
    pub roles_count: usize,
    pub phases_count: usize,
    pub execution_steps_count: usize,
    pub calendar_days_count: usize,
    pub guardrails_count: usize,
    pub patch_digest: u64,
    pub status: String,
}

/// FNV-1a hash calculation for PR patch digest reproducibility
fn fnv1a_digest(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// PR Gateway for Role-Based Governance, Strategy Roadmap & Operating Plan
pub struct RoleBasedOperatingPlanPrGateway {
    pub engine: RoleBasedOperatingPlanEngine,
}

impl RoleBasedOperatingPlanPrGateway {
    pub fn new() -> Self {
        Self {
            engine: RoleBasedOperatingPlanEngine::new(),
        }
    }

    /// Generates proposal specification metadata
    pub fn generate_proposal_spec(&self) -> RoleBasedOperatingPlanPrSpec {
        let payload = format!(
            "roles:{},phases:{},steps:{},calendar:5,guardrails:7",
            self.engine.roles.len(),
            self.engine.phases.len(),
            self.engine.execution_steps.len()
        );
        let patch_digest = fnv1a_digest(payload.as_bytes());

        RoleBasedOperatingPlanPrSpec {
            pr_title: String::from(
                "feat(governance): Adopt Role-Based Operating Plan and Strategic Wedge Roadmap",
            ),
            pr_branch: String::from("feat/role-based-operating-plan"),
            roles_count: self.engine.roles.len(),
            phases_count: self.engine.phases.len(),
            execution_steps_count: self.engine.execution_steps.len(),
            calendar_days_count: self.engine.calendar.len(),
            guardrails_count: 7,
            patch_digest,
            status: String::from("Approved / Implementation Ready"),
        }
    }

    /// Generates the complete PR Proposal in Markdown format
    pub fn generate_pr_proposal_markdown(&self) -> String {
        let spec = self.generate_proposal_spec();
        let mut doc = String::new();

        doc.push_str(&format!("# {}\n\n", spec.pr_title));
        doc.push_str(&format!("**Branch:** `{}` | **Status:** {}\n\n", spec.pr_branch, spec.status));
        doc.push_str("## Executive Summary\n\n");
        doc.push_str("This pull request establishes SigmaOS's Role-Based Governance, Phased Strategic Wedge Roadmap, and AI-Agent Maintenance Plan. Grounded in a pragmatic reality check, it shifts the project away from superficial feature parity claims across hundreds of distributions toward a focused, evidence-gated operating model where every capability claim requires verifiable CI artifacts (such as QEMU boot logs).\n\n");

        doc.push_str("## Reality Check & Strategic Wedge\n\n");
        doc.push_str("- **No OS defeats every competitor in every dimension.** SigmaOS focuses on owning a tight, differentiated wedge.\n");
        doc.push_str("- **The 4 Wedge Pillars:**\n");
        doc.push_str("  1. Memory-safe Rust microkernel and userland.\n");
        doc.push_str("  2. Atomic, rollbackable system updates by default (simplified Silverblue / NixOS model).\n");
        doc.push_str("  3. Capability-based security by default (pledge/unveil, Capsicum, Landlock).\n");
        doc.push_str("  4. Agent-native maintainability with machine-readable status, strict guardrails, and test gates.\n");
        doc.push_str("- **Ground Truth Over Claims:** Eliminates duplicate docs, unifies status tracking into evidence-based records, and replaces synthetic claims with QEMU serial output validation.\n\n");

        doc.push_str("## Phased Strategic Roadmap\n\n");
        for phase in &self.engine.phases {
            doc.push_str(&format!(
                "- **{:?} - {} ({})**: Status: {}\n",
                phase.phase,
                phase.name,
                phase.timeframe,
                if phase.is_complete { "Complete" } else { "In Progress" }
            ));
        }
        doc.push_str("\n");

        doc.push_str("## 12 Operating Maintenance Roles\n\n");
        doc.push_str("| Role | Mission | Cadence | Leadership Profile |\n");
        doc.push_str("|---|---|---|---|\n");
        for r in &self.engine.roles {
            doc.push_str(&format!(
                "| {} | {} | {} | {:?} |\n",
                r.title, r.mission, r.cadence, r.leadership
            ));
        }
        doc.push_str("\n");

        doc.push_str("## Weekly Operational Calendar\n\n");
        for day in &self.engine.calendar {
            let role_names: Vec<String> = day.assigned_roles.iter().map(|r| format!("{:?}", r)).collect();
            doc.push_str(&format!(
                "- **{:?}**: {} (Assigned Roles: {})\n",
                day.day,
                day.focus_area,
                role_names.join(", ")
            ));
        }
        doc.push_str("\n");

        doc.push_str("## 8-Step Team Execution Plan\n\n");
        for step in &self.engine.execution_steps {
            doc.push_str(&format!(
                "{}. **{} ({})**: {}\n   - **Required Output:** `{}`\n",
                step.step_number, step.title, step.timeframe, step.action, step.required_output
            ));
        }
        doc.push_str("\n");

        doc.push_str("## 7 Governance Guardrails\n\n");
        doc.push_str("1. **Human Approval:** Required for modifications touching kernel, crypto, bootloader, installer, or keys.\n");
        doc.push_str("2. **Evidence-Gated Success:** No feature may be marked complete without linked CI execution proof.\n");
        doc.push_str("3. **Sandbox and Least Privilege:** Agents operate with scoped tokens and no direct push to main.\n");
        doc.push_str("4. **Small, Reviewable PRs:** Scoped PRs with regression tests, with daily limits to prevent review bottlenecks.\n");
        doc.push_str("5. **Regression Ratchet:** Every bug fix includes a regression test; coverage and unsafe count must not worsen.\n");
        doc.push_str("6. **Audit Log & Revertibility:** Every automated action is recorded with one-command rollback capability.\n");
        doc.push_str("7. **Honest Public Reporting:** Documentation reflects only what boots and runs on actual targets today.\n\n");

        doc.push_str("## Specification Diff\n\n```diff\n");
        doc.push_str(&self.generate_patch_diff());
        doc.push_str("```\n");

        doc
    }

    /// Generates sample patch diff for roadmap specification
    pub fn generate_patch_diff(&self) -> String {
        let spec = self.generate_proposal_spec();
        format!(
            "--- a/docs/roadmap/PR_SPEC_ROLE_BASED_GOVERNANCE_AND_OPERATING_PLAN.md\n\
             +++ b/docs/roadmap/PR_SPEC_ROLE_BASED_GOVERNANCE_AND_OPERATING_PLAN.md\n\
             @@ -0,0 +1,120 @@\n\
             +# PR Title: {}\n\
             +# PR Branch: {}\n\
             +# Roles Count: {}\n\
             +# Phases Count: {}\n\
             +# Steps Count: {}\n\
             +# Calendar Days Count: {}\n\
             +# Guardrails Count: {}\n\
             +# Patch Digest: {:016x}\n\
             +# Status: {}\n",
            spec.pr_title,
            spec.pr_branch,
            spec.roles_count,
            spec.phases_count,
            spec.execution_steps_count,
            spec.calendar_days_count,
            spec.guardrails_count,
            spec.patch_digest,
            spec.status
        )
    }
}

impl Default for RoleBasedOperatingPlanPrGateway {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_operating_plan_engine_initialization() {
        let engine = RoleBasedOperatingPlanEngine::new();
        assert_eq!(engine.roles.len(), 12);
        assert_eq!(engine.phases.len(), 4);
        assert_eq!(engine.execution_steps.len(), 8);
        assert_eq!(engine.calendar.len(), 5);
    }

    #[test]
    fn test_role_definitions() {
        let engine = RoleBasedOperatingPlanEngine::new();
        let maintainer = engine
            .get_role_definition(MaintenanceRole::Maintainer)
            .unwrap();
        assert_eq!(maintainer.title, "Maintainer");
        assert_eq!(
            maintainer.leadership,
            LeadershipProfile::HumanLedAgentAssisted
        );

        let updater = engine
            .get_role_definition(MaintenanceRole::Updater)
            .unwrap();
        assert_eq!(updater.title, "Updater");
        assert_eq!(updater.leadership, LeadershipProfile::MostlyAgent);
    }

    #[test]
    fn test_calendar_day_focus() {
        let engine = RoleBasedOperatingPlanEngine::new();
        let monday = engine.get_day_focus(CalendarDay::Monday).unwrap();
        assert!(monday.focus_area.contains("Triage"));
        assert_eq!(monday.assigned_roles.len(), 3);
    }

    #[test]
    fn test_guardrail_validation() {
        let engine = RoleBasedOperatingPlanEngine::new();

        // Test failing guardrail: touches kernel without human signoff
        let verdicts_fail = engine.validate_pr_guardrails(true, false, true, true);
        assert!(!verdicts_fail[0].passed);

        // Test passing guardrail
        let verdicts_pass = engine.validate_pr_guardrails(true, true, true, true);
        assert!(verdicts_pass.iter().all(|v| v.passed));
    }

    #[test]
    fn test_gateway_proposal_and_diff_generation() {
        let gateway = RoleBasedOperatingPlanPrGateway::new();
        let spec = gateway.generate_proposal_spec();
        assert_eq!(spec.roles_count, 12);
        assert_eq!(spec.phases_count, 4);
        assert_eq!(spec.execution_steps_count, 8);
        assert_eq!(spec.calendar_days_count, 5);
        assert_eq!(spec.guardrails_count, 7);
        assert!(spec.patch_digest != 0);

        let markdown = gateway.generate_pr_proposal_markdown();
        assert!(markdown.contains("feat(governance): Adopt Role-Based Operating Plan"));
        assert!(markdown.contains("Reality Check & Strategic Wedge"));
        assert!(markdown.contains("12 Operating Maintenance Roles"));
        assert!(markdown.contains("Weekly Operational Calendar"));
        assert!(markdown.contains("7 Governance Guardrails"));

        let diff = gateway.generate_patch_diff();
        assert!(diff.contains("PR_SPEC_ROLE_BASED_GOVERNANCE_AND_OPERATING_PLAN.md"));
        assert!(diff.contains("Roles Count: 12"));
    }
}
