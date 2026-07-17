#![no_std]
#![no_main]

/// OOP-based Governance System for SigmaOS
/// Based on Ideas-999-Structured: Community & Governance Item 836
/// Implements RFC process and community governance

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RFCID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RFCStatus { Draft = 0, Proposed = 1, Discussion = 2, Voting = 3, Accepted = 4, Rejected = 5, Implemented = 6 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GovernanceError { Success = 0, NotFound = 1, InvalidState = 2, AccessDenied = 3 }

pub trait RFC {
    fn id(&self) -> RFCID;
    fn title(&self) -> &[u8];
    fn author(&self) -> &[u8];
    fn status(&self) -> RFCStatus;
    fn set_status(&mut self, status: RFCStatus) -> Result<(), GovernanceError>;
}

#[repr(C)]
pub struct SimpleRFC {
    pub id: RFCID,
    pub title: [u8; 128],
    pub author: [u8; 64],
    pub status: AtomicUsize,
}

impl SimpleRFC {
    pub fn new(id: RFCID, title: &[u8], author: &[u8]) -> Self {
        let mut title_array = [0u8; 128];
        let mut author_array = [0u8; 64];
        let title_len = title.len().min(127);
        let author_len = author.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(title.as_ptr(), title_array.as_mut_ptr(), title_len);
            core::ptr::copy_nonoverlapping(author.as_ptr(), author_array.as_mut_ptr(), author_len);
        }
        SimpleRFC {
            id,
            title: title_array,
            author: author_array,
            status: AtomicUsize::new(RFCStatus::Draft as usize),
        }
    }
}

impl RFC for SimpleRFC {
    fn id(&self) -> RFCID { self.id }
    fn title(&self) -> &[u8] {
        let len = self.title.iter().position(|&b| b == 0).unwrap_or(128);
        &self.title[..len]
    }
    fn author(&self) -> &[u8] {
        let len = self.author.iter().position(|&b| b == 0).unwrap_or(64);
        &self.author[..len]
    }
    fn status(&self) -> RFCStatus { unsafe { core::mem::transmute(self.status.load(Ordering::SeqCst)) } }

    fn set_status(&mut self, status: RFCStatus) -> Result<(), GovernanceError> {
        self.status.store(status as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait RFCRepository {
    fn submit(&mut self, rfc: Box<dyn RFC>) -> Result<RFCID, GovernanceError>;
    fn get(&self, id: RFCID) -> Option<&dyn RFC>;
    fn list_by_status(&self, status: RFCStatus) -> Vec<RFCID>;
    fn list_by_author(&self, author: &[u8]) -> Vec<RFCID>;
}

#[repr(C)]
pub struct SimpleRFCRepository {
    pub rfcs: Vec<Option<Box<dyn RFC>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRFCRepository {
    pub fn new() -> Self {
        SimpleRFCRepository {
            rfcs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RFCRepository for SimpleRFCRepository {
    fn submit(&mut self, rfc: Box<dyn RFC>) -> Result<RFCID, GovernanceError> {
        let id = rfc.id();
        self.rfcs.push(Some(rfc));
        Ok(id)
    }

    fn get(&self, id: RFCID) -> Option<&dyn RFC> {
        for rfc_option in &self.rfcs {
            if let Some(ref rfc) = *rfc_option {
                if rfc.id() == id { return Some(rfc.as_ref()); }
            }
        }
        None
    }

    fn list_by_status(&self, status: RFCStatus) -> Vec<RFCID> {
        let mut ids = Vec::new();
        for rfc_option in &self.rfcs {
            if let Some(ref rfc) = *rfc_option {
                if rfc.status() == status {
                    ids.push(rfc.id());
                }
            }
        }
        ids
    }

    fn list_by_author(&self, author: &[u8]) -> Vec<RFCID> {
        let mut ids = Vec::new();
        for rfc_option in &self.rfcs {
            if let Some(ref rfc) = *rfc_option {
                if rfc.author() == author {
                    ids.push(rfc.id());
                }
            }
        }
        ids
    }
}

pub trait VotingSystem {
    fn cast_vote(&mut self, rfc_id: RFCID, voter: &[u8], vote: bool) -> Result<(), GovernanceError>;
    fn get_vote_count(&self, rfc_id: RFCID) -> (usize, usize);
    fn has_voted(&self, rfc_id: RFCID, voter: &[u8]) -> bool;
}

#[repr(C)]
pub struct SimpleVotingSystem {
    pub votes: Vec<(RFCID, [u8; 64], bool)>,
}

impl SimpleVotingSystem {
    pub fn new() -> Self {
        SimpleVotingSystem {
            votes: Vec::new(),
        }
    }
}

impl VotingSystem for SimpleVotingSystem {
    fn cast_vote(&mut self, rfc_id: RFCID, voter: &[u8], vote: bool) -> Result<(), GovernanceError> {
        let mut voter_array = [0u8; 64];
        let voter_len = voter.len().min(63);
        for i in 0..voter_len {
            voter_array[i] = voter[i];
        }
        self.votes.push((rfc_id, voter_array, vote));
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

    fn has_voted(&self, rfc_id: RFCID, voter: &[u8]) -> bool {
        for &(id, ref v, _) in &self.votes {
            if id == rfc_id {
                let len = v.iter().position(|&b| b == 0).unwrap_or(64);
                if &v[..len] == voter {
                    return true;
                }
            }
        }
        false
    }
}

pub trait ContributorProgram {
    fn register_contributor(&mut self, name: &[u8], email: &[u8]) -> Result<usize, GovernanceError>;
    fn add_contribution(&mut self, contributor_id: usize, contribution: &[u8]) -> Result<(), GovernanceError>;
    fn get_contributions(&self, contributor_id: usize) -> Vec<&[u8]>;
}

#[repr(C)]
pub struct SimpleContributorProgram {
    pub contributors: Vec<([u8; 64], [u8; 128], Vec<[u8; 256]>)>,
    pub next_id: AtomicUsize,
}

impl SimpleContributorProgram {
    pub fn new() -> Self {
        SimpleContributorProgram {
            contributors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
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

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
