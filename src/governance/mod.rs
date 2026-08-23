// SigmaOS Governance & Transparency enhancements
// Foundation Model, Democratic Voting, and Release Roadmaps

pub mod okr;
pub mod rfc;
pub mod strategic_vision;

pub use okr::{
    MilestoneCategory, OkrError, OkrTracker, StrategicMilestone, StrategicOkrEvaluator,
};
pub use rfc::{
    GovernanceError, RFCRepository, RFCStatus, SimpleRFC, SimpleRFCRepository, SimpleVotingSystem,
    VotingSystem, RFC, RFCID,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Foundation committee member profile
#[derive(Debug, Clone)]
pub struct FoundationMember {
    pub name: String,
    pub role: String, // e.g. "Board Director", "Technical Lead", "Community Council"
    pub voting_power: usize,
}

/// SigmaOS Foundation Governance model
pub struct FoundationModel {
    pub members: Vec<FoundationMember>,
    pub treasury_usd: f64,
    pub foundation_name: String,
}

impl FoundationModel {
    pub fn new(foundation_name: String) -> Self {
        Self {
            members: Vec::new(),
            treasury_usd: 0.0,
            foundation_name,
        }
    }

    pub fn appoint_member(&mut self, name: String, role: String, power: usize) {
        let member = FoundationMember {
            name,
            role,
            voting_power: power,
        };
        self.members.push(member);
    }

    pub fn grant_funds(&mut self, amount: f64) {
        self.treasury_usd += amount;
    }

    pub fn allocate_funds(&mut self, amount: f64) -> bool {
        if amount <= self.treasury_usd {
            self.treasury_usd -= amount;
            true
        } else {
            false
        }
    }
}

impl Default for FoundationModel {
    fn default() -> Self {
        Self::new("SigmaOS Foundation".to_string())
    }
}

/// Roadmap release type / stability profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseType {
    Alpha,
    Beta,
    Lts,
    Rolling,
}

/// A milestone in the public/transparent roadmap
#[derive(Debug, Clone)]
pub struct RoadmapMilestone {
    pub version: String,
    pub release_type: ReleaseType,
    pub target_date: String,
    pub description: String,
    pub complete: bool,
}

/// Public release cycles scheduler
pub struct TransparentRoadmap {
    pub milestones: Vec<RoadmapMilestone>,
}

impl TransparentRoadmap {
    pub fn new() -> Self {
        Self {
            milestones: Vec::new(),
        }
    }

    pub fn publish_milestone(
        &mut self,
        version: String,
        r_type: ReleaseType,
        date: String,
        desc: String,
    ) {
        let milestone = RoadmapMilestone {
            version,
            release_type: r_type,
            target_date: date,
            description: desc,
            complete: false,
        };
        self.milestones.push(milestone);
    }

    pub fn mark_milestone_completed(&mut self, version: &str) -> bool {
        if let Some(m) = self.milestones.iter_mut().find(|m| m.version == version) {
            m.complete = true;
            true
        } else {
            false
        }
    }
}

impl Default for TransparentRoadmap {
    fn default() -> Self {
        Self::new()
    }
}

/// Democratic decision proposal
pub struct DemocraticProposal {
    pub id: usize,
    pub description: String,
    pub yes_votes: AtomicUsize,
    pub no_votes: AtomicUsize,
    pub quorum_required: usize,
    pub active: bool,
}

/// Democratic Community Voting system
pub struct DemocraticVoting {
    pub proposals: HashMap<usize, DemocraticProposal>,
    pub next_id: usize,
}

impl DemocraticVoting {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn submit_proposal(&mut self, description: String, quorum: usize) -> usize {
        let id = self.next_id;
        let proposal = DemocraticProposal {
            id,
            description,
            yes_votes: AtomicUsize::new(0),
            no_votes: AtomicUsize::new(0),
            quorum_required: quorum,
            active: true,
        };
        self.proposals.insert(id, proposal);
        self.next_id += 1;
        id
    }

    pub fn cast_vote(&self, proposal_id: usize, vote_yes: bool) -> bool {
        if let Some(prop) = self.proposals.get(&proposal_id) {
            if prop.active {
                if vote_yes {
                    prop.yes_votes.fetch_add(1, Ordering::SeqCst);
                } else {
                    prop.no_votes.fetch_add(1, Ordering::SeqCst);
                }
                return true;
            }
        }
        false
    }

    pub fn evaluate_proposal(&mut self, proposal_id: usize) -> Option<bool> {
        if let Some(prop) = self.proposals.get_mut(&proposal_id) {
            let yes = prop.yes_votes.load(Ordering::SeqCst);
            let no = prop.no_votes.load(Ordering::SeqCst);
            let total = yes + no;

            if total >= prop.quorum_required {
                prop.active = false;
                return Some(yes > no);
            }
        }
        None
    }
}

impl Default for DemocraticVoting {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foundation_model() {
        let mut foundation = FoundationModel::new("SigmaOS Europe Foundation".to_string());
        foundation.appoint_member("Niklas".to_string(), "Technical Lead".to_string(), 2);
        assert_eq!(foundation.members.len(), 1);
        assert_eq!(foundation.members[0].name, "Niklas");

        foundation.grant_funds(50000.0);
        assert!(foundation.allocate_funds(20000.0));
        assert_eq!(foundation.treasury_usd, 30000.0);
        assert!(!foundation.allocate_funds(40000.0));
    }

    #[test]
    fn test_transparent_roadmap() {
        let mut roadmap = TransparentRoadmap::new();
        roadmap.publish_milestone(
            "v1.0.0-LTS".to_string(),
            ReleaseType::Lts,
            "2025-12-31".to_string(),
            "Production ready Long-Term Support release".to_string(),
        );

        assert_eq!(roadmap.milestones.len(), 1);
        assert!(!roadmap.milestones[0].complete);

        assert!(roadmap.mark_milestone_completed("v1.0.0-LTS"));
        assert!(roadmap.milestones[0].complete);
        assert!(!roadmap.mark_milestone_completed("v2.0.0-LTS"));
    }

    #[test]
    fn test_democratic_voting() {
        let mut sys = DemocraticVoting::new();
        let id = sys.submit_proposal("Appoint community mediator".to_string(), 3);

        assert!(sys.cast_vote(id, true));
        assert!(sys.cast_vote(id, true));
        assert!(sys.cast_vote(id, false));

        // Quorum is 3. Total is 3. Yes (2) > No (1) => Pass!
        let outcome = sys.evaluate_proposal(id);
        assert_eq!(outcome, Some(true));

        let proposal = sys.proposals.get(&id).unwrap();
        assert!(!proposal.active); // Proposal has closed.
    }
}
