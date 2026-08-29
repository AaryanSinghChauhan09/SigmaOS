use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS Community Infrastructure
// Mentorship onboarding, structured bug tracking, and funding sustainability model

use crate::klib::HashMap;

/// Contributor onboarding stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnboardingStage {
    Registered,
    ActiveMentorship,
    TaskAssigned,
    FullyOnboarded,
}

/// Contributor profile
#[derive(Debug, Clone)]
pub struct ContributorProfile {
    pub name: String,
    pub email: String,
    pub stage: OnboardingStage,
    pub mentor: Option<String>,
    pub completed_tasks: Vec<String>,
    pub skills: Vec<String>,
}

impl ContributorProfile {
    pub fn new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            stage: OnboardingStage::Registered,
            mentor: None,
            completed_tasks: Vec::new(),
            skills: Vec::new(),
        }
    }
}

/// Mentorship program coordinator
pub struct MentorshipProgram {
    pub contributors: HashMap<String, ContributorProfile>,
    pub active_mentors: Vec<String>,
}

impl MentorshipProgram {
    pub fn new() -> Self {
        Self {
            contributors: HashMap::new(),
            active_mentors: Vec::new(),
        }
    }

    pub fn register_contributor(&mut self, name: String, email: String) {
        let profile = ContributorProfile::new(name.clone(), email);
        self.contributors.insert(name, profile);
    }

    pub fn add_mentor(&mut self, mentor_name: String) {
        if !self.active_mentors.contains(&mentor_name) {
            self.active_mentors.push(mentor_name);
        }
    }

    pub fn assign_mentor(&mut self, contributor_name: &str, mentor_name: &str) -> bool {
        if !self.active_mentors.contains(&mentor_name.to_string()) {
            return false;
        }

        if let Some(profile) = self.contributors.get_mut(contributor_name) {
            profile.mentor = Some(mentor_name.to_string());
            profile.stage = OnboardingStage::ActiveMentorship;
            true
        } else {
            false
        }
    }

    pub fn complete_task(&mut self, contributor_name: &str, task: String) -> bool {
        if let Some(profile) = self.contributors.get_mut(contributor_name) {
            profile.completed_tasks.push(task);
            if profile.completed_tasks.len() >= 3
                && profile.stage != OnboardingStage::FullyOnboarded
            {
                profile.stage = OnboardingStage::FullyOnboarded;
            }
            true
        } else {
            false
        }
    }
}

impl Default for MentorshipProgram {
    fn default() -> Self {
        Self::new()
    }
}

/// Bug severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BugSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Bug tracking states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueStatus {
    Reported,
    Triaged,
    Investigating,
    Resolved,
    Closed,
}

/// Structured issue report
#[derive(Debug, Clone)]
pub struct CommunityIssue {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub severity: BugSeverity,
    pub status: IssueStatus,
    pub assigned_to: Option<String>,
    pub reproduction_steps: String,
    pub tags: Vec<String>,
}

impl CommunityIssue {
    pub fn new(id: u32, title: String, description: String, severity: BugSeverity) -> Self {
        Self {
            id,
            title,
            description,
            severity,
            status: IssueStatus::Reported,
            assigned_to: None,
            reproduction_steps: String::new(),
            tags: Vec::new(),
        }
    }
}

/// Bug Tracking & Triage System
pub struct BugTracker {
    pub issues: HashMap<u32, CommunityIssue>,
    pub next_id: u32,
}

impl BugTracker {
    pub fn new() -> Self {
        Self {
            issues: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn file_issue(&mut self, title: String, description: String, severity: BugSeverity) -> u32 {
        let id = self.next_id;
        let issue = CommunityIssue::new(id, title, description, severity);
        self.issues.insert(id, issue);
        self.next_id += 1;
        id
    }

    pub fn triage_issue(&mut self, id: u32, severity: BugSeverity, tags: Vec<String>) -> bool {
        if let Some(issue) = self.issues.get_mut(&id) {
            issue.severity = severity;
            issue.tags = tags;
            issue.status = IssueStatus::Triaged;
            true
        } else {
            false
        }
    }

    pub fn assign_issue(&mut self, id: u32, developer: String) -> bool {
        if let Some(issue) = self.issues.get_mut(&id) {
            issue.assigned_to = Some(developer);
            issue.status = IssueStatus::Investigating;
            true
        } else {
            false
        }
    }

    pub fn update_status(&mut self, id: u32, status: IssueStatus) -> bool {
        if let Some(issue) = self.issues.get_mut(&id) {
            issue.status = status;
            true
        } else {
            false
        }
    }
}

impl Default for BugTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Corporate backer or sponsor profile
#[derive(Debug, Clone)]
pub struct Sponsor {
    pub name: String,
    pub tier: String, // e.g. Platinum, Gold, Individual
    pub contribution_amount: f64,
}

/// Funding & Sustainability Model
pub struct FundingSustainability {
    pub total_funds: f64,
    pub sponsors: Vec<Sponsor>,
    pub resource_allocations: HashMap<String, f64>,
}

impl FundingSustainability {
    pub fn new() -> Self {
        Self {
            total_funds: 0.0,
            sponsors: Vec::new(),
            resource_allocations: HashMap::new(),
        }
    }

    pub fn add_donation(&mut self, sponsor_name: String, tier: String, amount: f64) {
        let sponsor = Sponsor {
            name: sponsor_name,
            tier,
            contribution_amount: amount,
        };
        self.sponsors.push(sponsor);
        self.total_funds += amount;
    }

    pub fn allocate_budget(&mut self, sector: String, amount: f64) -> bool {
        if amount <= self.total_funds {
            self.total_funds -= amount;
            let current = self.resource_allocations.entry(sector).or_insert(0.0);
            *current += amount;
            true
        } else {
            false
        }
    }

    pub fn get_allocated_budget(&self, sector: &str) -> f64 {
        self.resource_allocations
            .get(sector)
            .copied()
            .unwrap_or(0.0)
    }
}

impl Default for FundingSustainability {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mentorship_program() {
        let mut program = MentorshipProgram::new();
        program.register_contributor("Aarav".to_string(), "aarav@sigmaos.org".to_string());
        program.add_mentor("Rohan".to_string());

        assert!(program.assign_mentor("Aarav", "Rohan"));
        let aarav = program.contributors.get("Aarav").unwrap();
        assert_eq!(aarav.stage, OnboardingStage::ActiveMentorship);
        assert_eq!(aarav.mentor, Some("Rohan".to_string()));

        // Complete tasks to onboard
        assert!(program.complete_task("Aarav", "Fix typo".to_string()));
        assert!(program.complete_task("Aarav", "Write test".to_string()));
        assert!(program.complete_task("Aarav", "Implement feature".to_string()));

        let aarav_after = program.contributors.get("Aarav").unwrap();
        assert_eq!(aarav_after.stage, OnboardingStage::FullyOnboarded);
    }

    #[test]
    fn test_bug_tracker() {
        let mut tracker = BugTracker::new();
        let id = tracker.file_issue(
            "Kernel panic on boot".to_string(),
            "Syscall routing failure".to_string(),
            BugSeverity::Critical,
        );

        assert_eq!(id, 1);
        let issue = tracker.issues.get(&id).unwrap();
        assert_eq!(issue.status, IssueStatus::Reported);

        assert!(tracker.triage_issue(id, BugSeverity::Critical, vec!["kernel".to_string()]));
        assert_eq!(
            tracker.issues.get(&id).unwrap().status,
            IssueStatus::Triaged
        );

        assert!(tracker.assign_issue(id, "Ananya".to_string()));
        assert_eq!(
            tracker.issues.get(&id).unwrap().status,
            IssueStatus::Investigating
        );
        assert_eq!(
            tracker.issues.get(&id).unwrap().assigned_to,
            Some("Ananya".to_string())
        );
    }

    #[test]
    fn test_funding_sustainability() {
        let mut fs = FundingSustainability::new();
        fs.add_donation("TechCorp".to_string(), "Platinum".to_string(), 100000.0);
        assert_eq!(fs.total_funds, 100000.0);

        assert!(fs.allocate_budget("Kernel Core".to_string(), 50000.0));
        assert_eq!(fs.get_allocated_budget("Kernel Core"), 50000.0);
        assert_eq!(fs.total_funds, 50000.0);

        assert!(!fs.allocate_budget("UI Design".to_string(), 60000.0));
    }
}
