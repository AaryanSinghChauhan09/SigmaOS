// SigmaOS RFC & Proposal Governance System
// Standard compliance based on Ideas-999-Structured: Community & Governance

use std::sync::atomic::{AtomicUsize, Ordering};

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

impl ContributorProgram for SimpleContributorProgram {
    fn register_contributor(&mut self, name: &[u8], email: &[u8]) -> Result<usize, GovernanceError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut name_array = [0u8; 64];
        let mut email_array = [0u8; 128];
        let name_len = name.len().min(63);
        let email_len = email.len().min(127);
        for i in 0..name_len { name_array[i] = name[i]; }
        for i in 0..email_len { email_array[i] = email[i]; }
        self.contributors.push((name_array, email_array, Vec::new()));
        Ok(id)
    }

    fn add_contribution(&mut self, contributor_id: usize, contribution: &[u8]) -> Result<(), GovernanceError> {
        if contributor_id > 0 && contributor_id <= self.contributors.len() {
            let mut contrib_array = [0u8; 256];
            let contrib_len = contribution.len().min(255);
            for i in 0..contrib_len { contrib_array[i] = contribution[i]; }
            self.contributors[contributor_id - 1].2.push(contrib_array);
            Ok(())
        } else {
            Err(GovernanceError::NotFound)
        }
    }

    fn get_contributions(&self, contributor_id: usize) -> Vec<&[u8]> {
        if contributor_id > 0 && contributor_id <= self.contributors.len() {
            let mut contributions = Vec::new();
            for contrib in &self.contributors[contributor_id - 1].2 {
                let len = contrib.iter().position(|&b| b == 0).unwrap_or(256);
                contributions.push(&contrib[..len]);
            }
            contributions
        } else {
            Vec::new()
        }
    }
}

pub trait CommunityGovernance {
    fn propose_rfc(&mut self, title: &[u8], author: &[u8]) -> Result<RFCID, GovernanceError>;
    fn vote_on_rfc(&mut self, rfc_id: RFCID, voter: &[u8], vote: bool) -> Result<(), GovernanceError>;
    fn finalize_rfc(&mut self, rfc_id: RFCID) -> Result<(), GovernanceError>;
}

#[repr(C)]
pub struct SimpleCommunityGovernance {
    pub repository: SimpleRFCRepository,
    pub voting: SimpleVotingSystem,
}

impl SimpleCommunityGovernance {
    pub fn new() -> Self {
        SimpleCommunityGovernance {
            repository: SimpleRFCRepository::new(),
            voting: SimpleVotingSystem::new(),
        }
    }
}

impl CommunityGovernance for SimpleCommunityGovernance {
    fn propose_rfc(&mut self, title: &[u8], author: &[u8]) -> Result<RFCID, GovernanceError> {
        let id = self.repository.next_id.fetch_add(1, Ordering::SeqCst);
        let rfc = SimpleRFC::new(id, title, author);
        self.repository.submit(Box::new(rfc))
    }

    fn vote_on_rfc(&mut self, rfc_id: RFCID, voter: &[u8], vote: bool) -> Result<(), GovernanceError> {
        self.voting.cast_vote(rfc_id, voter, vote)
    }

    fn finalize_rfc(&mut self, rfc_id: RFCID) -> Result<(), GovernanceError> {
        let (for_votes, against_votes) = self.voting.get_vote_count(rfc_id);

        if for_votes > against_votes {
            if let Some(rfc) = self.repository.get(rfc_id) {
                let mut rfc_status = RFCStatus::Accepted;
                return rfc.set_status(rfc_status);
            }
        }

        Err(GovernanceError::InvalidState)
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
