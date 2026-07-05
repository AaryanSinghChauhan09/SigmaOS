// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/elections/sigma_elections.rs — Voter Services
// Implements integration with Indian Election Commission services
//
// Features:
//   - Electoral Roll search (Voter Helpline 1950 API)
//   - EPIC (Voter ID) application (Form 6) and status
//   - Booth location finder with NavIC routing
//   - Candidate affidavit viewer (ADR database)
//   - EVM mock voting simulator
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Voter Information ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterInfo {
    pub epic_number: String,  // Electors Photo Identity Card
    pub name: String,
    pub father_name: String,
    pub age: u32,
    pub gender: String,
    pub address: Address,
    pub polling_station: PollingStation,
    pub assembly_constituency: String,
    pub parliamentary_constituency: String,
    pub district: String,
    pub state: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub house_number: String,
    pub street: String,
    pub locality: String,
    pub village: String,
    pub tehsil: String,
    pub district: String,
    pub state: String,
    pub pincode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollingStation {
    pub station_id: String,
    pub name: String,
    pub address: String,
    pub building: String,
    pub latitude: f64,
    pub longitude: f64,
    pub serial_number_range: String,
}

// ── EPIC Application (Form 6) ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpicApplication {
    pub application_id: String,
    pub application_type: String,  // Form 6, Form 7, Form 8, Form 8A
    pub applicant_name: String,
    pub father_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub address: Address,
    pub documents: Vec<Document>,
    pub application_date: String,
    pub status: String,
    pub epic_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub document_type: String,
    pub document_number: String,
    pub file_hash: String,
}

// ── Candidate Information ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub candidate_id: String,
    pub name: String,
    pub party: String,
    pub party_symbol: String,
    pub constituency: String,
    pub election_type: String,  // Assembly/Parliamentary
    pub affidavit_url: String,
    pub criminal_cases: u32,
    pub assets: f64,
    pub liabilities: f64,
    pub education: String,
    pub profession: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affidavit {
    pub candidate_id: String,
    pub election_year: u32,
    pub constituency: String,
    pub criminal_cases: Vec<CriminalCase>,
    pub assets: AssetDeclaration,
    pub liabilities: LiabilityDeclaration,
    pub income: IncomeDeclaration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriminalCase {
    pub case_number: String,
    pub court: String,
    pub offense: String,
    pub year: u32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDeclaration {
    pub movable_assets: f64,
    pub immovable_assets: f64,
    pub total_assets: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiabilityDeclaration {
    pub total_liabilities: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeDeclaration {
    pub self_income: f64,
    pub spouse_income: f64,
    pub total_income: f64,
}

// ── Election Information ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Election {
    pub election_id: String,
    pub name: String,
    pub election_type: String,
    pub state: String,
    pub date: String,
    pub constituencies: Vec<String>,
    pub total_voters: u64,
    pub total_candidates: u32,
}

// ── Elections Client ─────────────────────────────────────────────────────

pub struct ElectionsClient {
    base_url: String,
    api_key: String,
}

impl ElectionsClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
        }
    }

    /// Search electoral roll by EPIC number
    pub fn search_by_epic(&self, epic_number: &str) -> Result<VoterInfo, String> {
        // In production: Make HTTP GET request to Voter Helpline API
        // For now: Return mock voter info
        Ok(VoterInfo {
            epic_number: epic_number.to_string(),
            name: "Voter Name".to_string(),
            father_name: "Father Name".to_string(),
            age: 35,
            gender: "Male".to_string(),
            address: Address {
                house_number: "123".to_string(),
                street: "Main Street".to_string(),
                locality: "Ward 5".to_string(),
                village: "City".to_string(),
                tehsil: "City".to_string(),
                district: "District".to_string(),
                state: "State".to_string(),
                pincode: "123456".to_string(),
            },
            polling_station: PollingStation {
                station_id: "PS001".to_string(),
                name: "Government School".to_string(),
                address: "123 School Road".to_string(),
                building: "School Building".to_string(),
                latitude: 19.0760,
                longitude: 72.8777,
                serial_number_range: "100-200".to_string(),
            },
            assembly_constituency: "Constituency 1".to_string(),
            parliamentary_constituency: "PC 1".to_string(),
            district: "District".to_string(),
            state: "State".to_string(),
            photo_url: Some("https://example.com/photo.jpg".to_string()),
        })
    }

    /// Search electoral roll by name and address
    pub fn search_by_name(&self, name: &str, father_name: &str, district: &str) -> Result<Vec<VoterInfo>, String> {
        // In production: Make HTTP GET request to search API
        // For now: Return mock results
        Ok(vec![
            VoterInfo {
                epic_number: "ABC1234567".to_string(),
                name: name.to_string(),
                father_name: father_name.to_string(),
                age: 35,
                gender: "Male".to_string(),
                address: Address {
                    house_number: "123".to_string(),
                    street: "Main Street".to_string(),
                    locality: "Ward 5".to_string(),
                    village: "City".to_string(),
                    tehsil: "City".to_string(),
                    district: district.to_string(),
                    state: "State".to_string(),
                    pincode: "123456".to_string(),
                },
                polling_station: PollingStation {
                    station_id: "PS001".to_string(),
                    name: "Government School".to_string(),
                    address: "123 School Road".to_string(),
                    building: "School Building".to_string(),
                    latitude: 19.0760,
                    longitude: 72.8777,
                    serial_number_range: "100-200".to_string(),
                },
                assembly_constituency: "Constituency 1".to_string(),
                parliamentary_constituency: "PC 1".to_string(),
                district: district.to_string(),
                state: "State".to_string(),
                photo_url: None,
            },
        ])
    }

    /// Submit EPIC application (Form 6)
    pub fn submit_epic_application(&self, application: &EpicApplication) -> Result<String, String> {
        // In production: Make HTTP POST request to application API
        // For now: Return mock application ID
        Ok(format!("APP_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get application status
    pub fn get_application_status(&self, application_id: &str) -> Result<EpicApplication, String> {
        // In production: Make HTTP GET request to status API
        // For now: Return mock status
        Ok(EpicApplication {
            application_id: application_id.to_string(),
            application_type: "Form 6".to_string(),
            applicant_name: "Applicant Name".to_string(),
            father_name: "Father Name".to_string(),
            date_of_birth: "1990-01-01".to_string(),
            gender: "Male".to_string(),
            address: Address {
                house_number: "123".to_string(),
                street: "Main Street".to_string(),
                locality: "Ward 5".to_string(),
                village: "City".to_string(),
                tehsil: "City".to_string(),
                district: "District".to_string(),
                state: "State".to_string(),
                pincode: "123456".to_string(),
            },
            documents: vec![],
            application_date: "2024-01-15".to_string(),
            status: "Approved".to_string(),
            epic_number: Some("ABC1234567".to_string()),
        })
    }

    /// Get booth location
    pub fn get_booth_location(&self, epic_number: &str) -> Result<PollingStation, String> {
        // In production: Make HTTP GET request to booth location API
        // For now: Return mock location
        Ok(PollingStation {
            station_id: "PS001".to_string(),
            name: "Government School".to_string(),
            address: "123 School Road".to_string(),
            building: "School Building".to_string(),
            latitude: 19.0760,
            longitude: 72.8777,
            serial_number_range: "100-200".to_string(),
        })
    }

    /// Get candidates for constituency
    pub fn get_candidates(&self, constituency: &str, election_type: &str) -> Result<Vec<Candidate>, String> {
        // In production: Make HTTP GET request to candidates API
        // For now: Return mock candidates
        Ok(vec![
            Candidate {
                candidate_id: "CAND001".to_string(),
                name: "Candidate 1".to_string(),
                party: "Party A".to_string(),
                party_symbol: "Symbol A".to_string(),
                constituency: constituency.to_string(),
                election_type: election_type.to_string(),
                affidavit_url: "https://example.com/affidavit1.pdf".to_string(),
                criminal_cases: 0,
                assets: 1000000.0,
                liabilities: 100000.0,
                education: "Graduate".to_string(),
                profession: "Business".to_string(),
            },
            Candidate {
                candidate_id: "CAND002".to_string(),
                name: "Candidate 2".to_string(),
                party: "Party B".to_string(),
                party_symbol: "Symbol B".to_string(),
                constituency: constituency.to_string(),
                election_type: election_type.to_string(),
                affidavit_url: "https://example.com/affidavit2.pdf".to_string(),
                criminal_cases: 2,
                assets: 5000000.0,
                liabilities: 500000.0,
                education: "Post Graduate".to_string(),
                profession: "Lawyer".to_string(),
            },
        ])
    }

    /// Get candidate affidavit
    pub fn get_affidavit(&self, candidate_id: &str) -> Result<Affidavit, String> {
        // In production: Make HTTP GET request to ADR API
        // For now: Return mock affidavit
        Ok(Affidavit {
            candidate_id: candidate_id.to_string(),
            election_year: 2024,
            constituency: "Constituency 1".to_string(),
            criminal_cases: vec![
                CriminalCase {
                    case_number: "CC/2024/001".to_string(),
                    court: "District Court".to_string(),
                    offense: "IPC Section 420".to_string(),
                    year: 2020,
                    status: "Pending".to_string(),
                },
            ],
            assets: AssetDeclaration {
                movable_assets: 3000000.0,
                immovable_assets: 2000000.0,
                total_assets: 5000000.0,
            },
            liabilities: LiabilityDeclaration {
                total_liabilities: 500000.0,
            },
            income: IncomeDeclaration {
                self_income: 1000000.0,
                spouse_income: 500000.0,
                total_income: 1500000.0,
            },
        })
    }

    /// Get upcoming elections
    pub fn get_upcoming_elections(&self, state: &str) -> Result<Vec<Election>, String> {
        // In production: Make HTTP GET request to elections API
        // For now: Return mock elections
        Ok(vec![
            Election {
                election_id: "ELEC001".to_string(),
                name: "State Assembly Election".to_string(),
                election_type: "Assembly".to_string(),
                state: state.to_string(),
                date: "2024-04-15".to_string(),
                constituencies: vec!["Constituency 1".to_string(), "Constituency 2".to_string()],
                total_voters: 10000000,
                total_candidates: 500,
            },
        ])
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn elections_client_create(base_url: *const u8, base_url_len: usize,
                                         api_key: *const u8, api_key_len: usize) -> *mut ElectionsClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(ElectionsClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn elections_client_destroy(client: *mut ElectionsClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn elections_search_by_epic(client: *const ElectionsClient,
                                          epic: *const u8, epic_len: usize,
                                          out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || epic.is_null() { return -1; }
        let epic = String::from_utf8_unchecked(
            std::slice::from_raw_parts(epic, epic_len));
        match (*client).search_by_epic(&epic) {
            Ok(voter) => {
                let json = serde_json::to_string(&voter).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
