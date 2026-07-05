// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/judicial/sigma_judicial.rs — eCourts Deep Integration
// Implements integration with Indian eCourts system for case management
//
// Features:
//   - Live cause list monitoring
//   - CNR (Case Number Record) lookup
//   - eCourts API integration
//   - e-Stamping integration
//   - Virtual court hearing support
//   - DID-signed pleadings
//   - High Court/Supreme Court e-filing
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Case Information Structures ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseRecord {
    pub cnr: String,  // Case Number Record
    pub case_type: String,
    pub filing_number: String,
    pub filing_date: String,
    pub registration_number: String,
    pub registration_date: String,
    pub court: CourtInfo,
    pub parties: Vec<Party>,
    pub advocates: Vec<Advocate>,
    pub case_status: String,
    pub next_hearing_date: Option<String>,
    pub case_stage: String,
    pub act: String,
    pub offense: Option<String>,
    pub police_station: Option<String>,
    pub fir_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtInfo {
    pub state: String,
    pub district: String,
    pub establishment: String,
    pub court_name: String,
    pub bench: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    pub party_name: String,
    pub party_type: String,  // Petitioner/Respondent
    pub advocate: Option<String>,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Advocate {
    pub advocate_name: String,
    pub enrollment_number: String,
    pub bar_council: String,
}

// ── Cause List ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseListItem {
    pub serial_number: u32,
    pub cnr: String,
    pub case_type: String,
    pub case_number: String,
    pub parties: String,
    pub advocate: String,
    pub hearing_stage: String,
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseList {
    pub date: String,
    pub court: CourtInfo,
    pub items: Vec<CauseListItem>,
}

// ── e-Stamping ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampDutyRequest {
    pub instrument_type: String,
    pub property_value: f64,
    pub property_type: String,
    pub state: String,
    pub district: String,
    pub parties: Vec<Party>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampDutyResponse {
    pub stamp_duty: f64,
    pub registration_fee: f64,
    pub total_amount: f64,
    pub stamp_certificate: String,
    pub transaction_id: String,
}

// ── e-Filing Structures ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfilingRequest {
    pub case_type: String,
    pub jurisdiction: String,
    pub parties: Vec<Party>,
    pub pleadings: String,
    pub documents: Vec<Document>,
    pub did_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub document_type: String,
    pub file_hash: String,
    pub file_name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfilingResponse {
    pub filing_number: String,
    pub acknowledgment: String,
    pub verification_code: String,
    pub status: String,
}

// ── Judicial Client ─────────────────────────────────────────────────────

pub struct JudicialClient {
    base_url: String,
    api_key: String,
    did: Option<String>,
}

impl JudicialClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            did: None,
        }
    }

    pub fn set_did(&mut self, did: String) {
        self.did = Some(did);
    }

    /// Get case details by CNR
    pub fn get_case_by_cnr(&self, cnr: &str) -> Result<CaseRecord, String> {
        // In production: Make HTTP GET request to eCourts API
        // For now: Return mock case record
        Ok(CaseRecord {
            cnr: cnr.to_string(),
            case_type: "Criminal".to_string(),
            filing_number: "FIL2024001234".to_string(),
            filing_date: "2024-01-15".to_string(),
            registration_number: "REG2024000567".to_string(),
            registration_date: "2024-01-20".to_string(),
            court: CourtInfo {
                state: "Maharashtra".to_string(),
                district: "Mumbai".to_string(),
                establishment: "City Civil Court".to_string(),
                court_name: "Court of Additional Sessions Judge".to_string(),
                bench: "Main Bench".to_string(),
            },
            parties: vec![
                Party {
                    party_name: "State of Maharashtra".to_string(),
                    party_type: "Petitioner".to_string(),
                    advocate: Some("Advocate General".to_string()),
                    address: "Mantralaya, Mumbai".to_string(),
                },
                Party {
                    party_name: "John Doe".to_string(),
                    party_type: "Respondent".to_string(),
                    advocate: Some("Defense Counsel".to_string()),
                    address: "123, Andheri East, Mumbai".to_string(),
                },
            ],
            advocates: vec![
                Advocate {
                    advocate_name: "Advocate General".to_string(),
                    enrollment_number: "MH/1234/1990".to_string(),
                    bar_council: "Bar Council of Maharashtra & Goa".to_string(),
                },
            ],
            case_status: "Pending".to_string(),
            next_hearing_date: Some("2024-02-15".to_string()),
            case_stage: "Arguments".to_string(),
            act: "Indian Penal Code".to_string(),
            offense: Some("Section 420".to_string()),
            police_station: Some("Andheri Police Station".to_string()),
            fir_number: Some("FIR/2024/001234".to_string()),
        })
    }

    /// Get cause list for a court on a specific date
    pub fn get_cause_list(&self, court_code: &str, date: &str) -> Result<CauseList, String> {
        // In production: Make HTTP GET request to cause list API
        // For now: Return mock cause list
        Ok(CauseList {
            date: date.to_string(),
            court: CourtInfo {
                state: "Maharashtra".to_string(),
                district: "Mumbai".to_string(),
                establishment: "City Civil Court".to_string(),
                court_name: "Court of Additional Sessions Judge".to_string(),
                bench: "Main Bench".to_string(),
            },
            items: vec![
                CauseListItem {
                    serial_number: 1,
                    cnr: "MHBN202400123456".to_string(),
                    case_type: "Criminal Appeal".to_string(),
                    case_number: "CA/1234/2024".to_string(),
                    parties: "State vs. John Doe".to_string(),
                    advocate: "Adv. A. Sharma".to_string(),
                    hearing_stage: "Arguments".to_string(),
                    purpose: "Final Arguments".to_string(),
                },
                CauseListItem {
                    serial_number: 2,
                    cnr: "MHBN202400123457".to_string(),
                    case_type: "Civil Suit".to_string(),
                    case_number: "CS/5678/2024".to_string(),
                    parties: "ABC Corp vs. XYZ Ltd".to_string(),
                    advocate: "Adv. R. Gupta".to_string(),
                    hearing_stage: "Evidence".to_string(),
                    purpose: "Examination of Witnesses".to_string(),
                },
            ],
        })
    }

    /// Calculate stamp duty
    pub fn calculate_stamp_duty(&self, request: &StampDutyRequest) -> Result<StampDutyResponse, String> {
        // In production: Make HTTP POST request to e-Stamping API
        // For now: Return mock calculation
        let stamp_duty = request.property_value * 0.05; // 5% stamp duty
        let registration_fee = request.property_value * 0.01; // 1% registration fee
        
        Ok(StampDutyResponse {
            stamp_duty,
            registration_fee,
            total_amount: stamp_duty + registration_fee,
            stamp_certificate: format!("STAMP_CERT_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            transaction_id: format!("TXN_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
        })
    }

    /// Submit e-filing
    pub fn submit_efiling(&self, request: &EfilingRequest) -> Result<EfilingResponse, String> {
        // In production: Make HTTP POST request to e-filing API
        // For now: Return mock response
        Ok(EfilingResponse {
            filing_number: format!("EFIL{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            acknowledgment: format!("ACK_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            verification_code: format!("VER{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            status: "Submitted".to_string(),
        })
    }

    /// Search cases by party name
    pub fn search_cases_by_party(&self, party_name: &str) -> Result<Vec<CaseRecord>, String> {
        // In production: Make HTTP GET request to case search API
        // For now: Return mock results
        Ok(vec![
            CaseRecord {
                cnr: "MHBN202400123456".to_string(),
                case_type: "Civil Suit".to_string(),
                filing_number: "FIL2024001234".to_string(),
                filing_date: "2024-01-15".to_string(),
                registration_number: "REG2024000567".to_string(),
                registration_date: "2024-01-20".to_string(),
                court: CourtInfo {
                    state: "Maharashtra".to_string(),
                    district: "Mumbai".to_string(),
                    establishment: "City Civil Court".to_string(),
                    court_name: "Court of Civil Judge".to_string(),
                    bench: "Main Bench".to_string(),
                },
                parties: vec![
                    Party {
                        party_name: party_name.to_string(),
                        party_type: "Petitioner".to_string(),
                        advocate: None,
                        address: "Mumbai".to_string(),
                    },
                ],
                advocates: vec![],
                case_status: "Pending".to_string(),
                next_hearing_date: Some("2024-02-15".to_string()),
                case_stage: "Preliminary Hearing".to_string(),
                act: "Civil Procedure Code".to_string(),
                offense: None,
                police_station: None,
                fir_number: None,
            },
        ])
    }

    /// Get case history
    pub fn get_case_history(&self, cnr: &str) -> Result<Vec<CaseHistoryEntry>, String> {
        // In production: Make HTTP GET request to case history API
        // For now: Return mock history
        Ok(vec![
            CaseHistoryEntry {
                date: "2024-01-20".to_string(),
                stage: "Registration".to_string(),
                remarks: "Case registered".to_string(),
                next_hearing: "2024-02-15".to_string(),
            },
            CaseHistoryEntry {
                date: "2024-02-15".to_string(),
                stage: "Preliminary Hearing".to_string(),
                remarks: "Notice issued to respondent".to_string(),
                next_hearing: "2024-03-15".to_string(),
            },
        ])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseHistoryEntry {
    pub date: String,
    pub stage: String,
    pub remarks: String,
    pub next_hearing: String,
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn judicial_client_create(base_url: *const u8, base_url_len: usize,
                                        api_key: *const u8, api_key_len: usize) -> *mut JudicialClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(JudicialClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn judicial_client_destroy(client: *mut JudicialClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn judicial_get_case_by_cnr(client: *const JudicialClient,
                                          cnr: *const u8, cnr_len: usize,
                                          out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || cnr.is_null() { return -1; }
        let cnr = String::from_utf8_unchecked(
            std::slice::from_raw_parts(cnr, cnr_len));
        match (*client).get_case_by_cnr(&cnr) {
            Ok(case) => {
                let json = serde_json::to_string(&case).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
