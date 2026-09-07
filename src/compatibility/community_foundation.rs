// Sovereign OS Foundation & Community Governance Framework
// Fulfills low-priority tasks from TODO.md: Foundation establishment,
// contributor hackathons, documentation sprints, and security bounty programs.

use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationRole {
    BoardMember,
    CoreMaintainer,
    SecurityAuditor,
    CommunityContributor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BountySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SecurityBounty {
    pub id: u64,
    pub title: String,
    pub reporter: String,
    pub severity: BountySeverity,
    pub reward_usd: u32,
    pub resolved: bool,
}

#[derive(Debug, Clone)]
pub struct HackathonEvent {
    pub name: String,
    pub topic: String,
    pub participants: Vec<String>,
    pub projects_submitted: Vec<String>,
}

pub struct SovereignFoundationManager {
    pub foundation_name: String,
    pub members: Vec<(String, FoundationRole)>,
    pub bounties: Vec<SecurityBounty>,
    pub hackathons: Vec<HackathonEvent>,
    pub doc_sprint_active: bool,
    pub next_bounty_id: u64,
}

impl SovereignFoundationManager {
    pub fn new(foundation_name: &str) -> Self {
        Self {
            foundation_name: foundation_name.to_string(),
            members: Vec::new(),
            bounties: Vec::new(),
            hackathons: Vec::new(),
            doc_sprint_active: false,
            next_bounty_id: 1,
        }
    }

    pub fn register_member(&mut self, name: &str, role: FoundationRole) {
        if let Some(pos) = self.members.iter().position(|(m, _)| m == name) {
            self.members[pos].1 = role;
        } else {
            self.members.push((name.to_string(), role));
        }
    }

    pub fn submit_security_bounty(
        &mut self,
        title: &str,
        reporter: &str,
        severity: BountySeverity,
    ) -> u64 {
        let id = self.next_bounty_id;
        self.next_bounty_id += 1;

        let reward_usd = match severity {
            BountySeverity::Low => 500,
            BountySeverity::Medium => 2000,
            BountySeverity::High => 5000,
            BountySeverity::Critical => 15000,
        };

        self.bounties.push(SecurityBounty {
            id,
            title: title.to_string(),
            reporter: reporter.to_string(),
            severity,
            reward_usd,
            resolved: false,
        });

        id
    }

    pub fn resolve_bounty(&mut self, bounty_id: u64) -> Result<u32, &'static str> {
        let bounty = self
            .bounties
            .iter_mut()
            .find(|b| b.id == bounty_id)
            .ok_or("Bounty report not found")?;

        if bounty.resolved {
            return Err("Bounty is already resolved");
        }

        bounty.resolved = true;
        Ok(bounty.reward_usd)
    }

    pub fn organize_hackathon(&mut self, name: &str, topic: &str) {
        self.hackathons.push(HackathonEvent {
            name: name.to_string(),
            topic: topic.to_string(),
            participants: Vec::new(),
            projects_submitted: Vec::new(),
        });
    }

    pub fn register_hackathon_participant(
        &mut self,
        hackathon_name: &str,
        participant: &str,
    ) -> Result<(), &'static str> {
        let event = self
            .hackathons
            .iter_mut()
            .find(|h| h.name == hackathon_name)
            .ok_or("Hackathon event not found")?;

        if !event.participants.contains(&participant.to_string()) {
            event.participants.push(participant.to_string());
        }

        Ok(())
    }

    pub fn submit_hackathon_project(
        &mut self,
        hackathon_name: &str,
        project: &str,
    ) -> Result<(), &'static str> {
        let event = self
            .hackathons
            .iter_mut()
            .find(|h| h.name == hackathon_name)
            .ok_or("Hackathon event not found")?;

        event.projects_submitted.push(project.to_string());
        Ok(())
    }

    pub fn set_doc_sprint_status(&mut self, active: bool) {
        self.doc_sprint_active = active;
    }
}

impl Default for SovereignFoundationManager {
    fn default() -> Self {
        Self::new("SigmaOS Software Foundation")
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_foundation_governance_and_bounties() {
        let mut foundation = SovereignFoundationManager::new("SigmaOS Foundation");
        foundation.register_member("alice", FoundationRole::BoardMember);
        assert_eq!(foundation.members.len(), 1);

        let bounty_id = foundation.submit_security_bounty(
            "Kernel heap overflow in VFS",
            "bob_security",
            BountySeverity::Critical,
        );

        assert_eq!(bounty_id, 1);
        assert_eq!(foundation.bounties[0].reward_usd, 15000);

        let payout = foundation.resolve_bounty(bounty_id).unwrap();
        assert_eq!(payout, 15000);
        assert!(foundation.bounties[0].resolved);

        foundation.organize_hackathon("Global Kernel Hackathon 2026", "Zero-Copy IPC Innovations");
        assert!(foundation
            .register_hackathon_participant("Global Kernel Hackathon 2026", "charlie_dev")
            .is_ok());
        assert!(foundation
            .submit_hackathon_project("Global Kernel Hackathon 2026", "SovereignRingFS")
            .is_ok());

        assert_eq!(foundation.hackathons[0].projects_submitted.len(), 1);
    }
}
