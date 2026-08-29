extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS RFC & Proposal Governance System
// Standard compliance based on Ideas-999-Structured: Community & Governance

use core::sync::atomic::{AtomicUsize, Ordering};

pub type RFCID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RFCStatus {
    Draft = 0,
    Proposed = 1,
    Discussion = 2,
    Voting = 3,
    Accepted = 4,
    Rejected = 5,
    Implemented = 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceError {
    Success = 0,
    NotFound = 1,
    InvalidState = 2,
    AccessDenied = 3,
}

pub trait RFC {
    fn id(&self) -> RFCID;
    fn title(&self) -> &str;
    fn author(&self) -> &str;
    fn status(&self) -> RFCStatus;
    fn set_status(&self, status: RFCStatus) -> Result<(), GovernanceError>;
}

pub struct SimpleRFC {
    pub id: RFCID,
    pub title: String,
    pub author: String,
    pub status: AtomicUsize,
}

impl SimpleRFC {
    pub fn new(id: RFCID, title: String, author: String) -> Self {
        Self {
            id,
            title,
            author,
            status: AtomicUsize::new(RFCStatus::Draft as usize),
        }
    }
}

impl RFC for SimpleRFC {
    fn id(&self) -> RFCID {
        self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn author(&self) -> &str {
        &self.author
    }

    fn status(&self) -> RFCStatus {
        match self.status.load(Ordering::SeqCst) {
            0 => RFCStatus::Draft,
            1 => RFCStatus::Proposed,
            2 => RFCStatus::Discussion,
            3 => RFCStatus::Voting,
            4 => RFCStatus::Accepted,
            5 => RFCStatus::Rejected,
            _ => RFCStatus::Implemented,
        }
    }

    fn set_status(&self, status: RFCStatus) -> Result<(), GovernanceError> {
        self.status.store(status as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait RFCRepository {
    fn submit(&mut self, rfc: Box<dyn RFC>) -> Result<RFCID, GovernanceError>;
    fn get(&self, id: RFCID) -> Option<&dyn RFC>;
    fn list_by_status(&self, status: RFCStatus) -> Vec<RFCID>;
    fn list_by_author(&self, author: &str) -> Vec<RFCID>;
}

pub struct SimpleRFCRepository {
    pub rfcs: Vec<Box<dyn RFC>>,
    pub next_id: AtomicUsize,
}

impl SimpleRFCRepository {
    pub fn new() -> Self {
        Self {
            rfcs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RFCRepository for SimpleRFCRepository {
    fn submit(&mut self, rfc: Box<dyn RFC>) -> Result<RFCID, GovernanceError> {
        let id = rfc.id();
        self.rfcs.push(rfc);
        Ok(id)
    }

    fn get(&self, id: RFCID) -> Option<&dyn RFC> {
        self.rfcs.iter().map(|r| r.as_ref()).find(|r| r.id() == id)
    }

    fn list_by_status(&self, status: RFCStatus) -> Vec<RFCID> {
        self.rfcs
            .iter()
            .filter(|r| r.status() == status)
            .map(|r| r.id())
            .collect()
    }

    fn list_by_author(&self, author: &str) -> Vec<RFCID> {
        self.rfcs
            .iter()
            .filter(|r| r.author() == author)
            .map(|r| r.id())
            .collect()
    }
}

impl Default for SimpleRFCRepository {
    fn default() -> Self {
        Self::new()
    }
}

pub trait VotingSystem {
    fn cast_vote(
        &mut self,
        rfc_id: RFCID,
        voter: String,
        vote: bool,
    ) -> Result<(), GovernanceError>;
    fn get_vote_count(&self, rfc_id: RFCID) -> (usize, usize);
    fn has_voted(&self, rfc_id: RFCID, voter: &str) -> bool;
}

pub struct SimpleVotingSystem {
    pub votes: Vec<(RFCID, String, bool)>,
}

impl SimpleVotingSystem {
    pub fn new() -> Self {
        Self { votes: Vec::new() }
    }
}

impl VotingSystem for SimpleVotingSystem {
    fn cast_vote(
        &mut self,
        rfc_id: RFCID,
        voter: String,
        vote: bool,
    ) -> Result<(), GovernanceError> {
        if self.has_voted(rfc_id, &voter) {
            return Err(GovernanceError::AccessDenied);
        }
        self.votes.push((rfc_id, voter, vote));
        Ok(())
    }

    fn get_vote_count(&self, rfc_id: RFCID) -> (usize, usize) {
        let mut for_votes = 0;
        let mut against_votes = 0;
        for &(id, _, vote) in &self.votes {
            if id == rfc_id {
                if vote {
                    for_votes += 1;
                } else {
                    against_votes += 1;
                }
            }
        }
        (for_votes, against_votes)
    }

    fn has_voted(&self, rfc_id: RFCID, voter: &str) -> bool {
        self.votes
            .iter()
            .any(|&(id, ref v, _)| id == rfc_id && v == voter)
    }
}

impl Default for SimpleVotingSystem {
    fn default() -> Self {
        Self::new()
    }
}
