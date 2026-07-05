// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/prison/sigma_prison.rs — Correctional Facility Management
// Implements integration with Indian prison management systems (ePrisons/ICJS)
//
// Features:
//   - ePrisons (ICJS) system integration
//   - BNSS undertrial time limit tracker (prevents illegal detention)
//   - Bail compliance monitoring
//   - Prisoner rehabilitation programme management
//   - Under-trial review compliance (Arnesh Kumar judgment checklist)
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Prisoner Information ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prisoner {
    pub prisoner_id: String,
    pub name: String,
    pub father_name: String,
    pub age: u32,
    pub gender: String,
    pub category: String,  // Under-trial, Convicted
    pub admission_date: String,
    pub prison: Prison,
    pub offense: String,
    pub case_number: String,
    pub court: String,
    pub sentence: Option<Sentence>,
    pub bail_details: Option<BailDetails>,
    pub rehabilitation: Option<Rehabilitation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prison {
    pub prison_id: String,
    pub name: String,
    pub state: String,
    pub district: String,
    pub capacity: u32,
    pub current_population: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentence {
    pub sentence_type: String,  // Life imprisonment, Term imprisonment
    pub years: u32,
    pub months: u32,
    pub start_date: String,
    pub end_date: String,
    pub remission_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BailDetails {
    pub bail_type: String,  // Regular bail, Anticipatory bail, Parole
    pub granted_date: String,
    pub conditions: Vec<String>,
    pub surety_amount: f64,
    pub expiry_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rehabilitation {
    pub programme_id: String,
    pub programme_name: String,
    pub start_date: String,
    pub completion_date: Option<String>,
    pub status: String,
    pub skills: Vec<String>,
}

// ── BNSS Undertrial Time Limit Tracker ─────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndertrialTracker {
    pub prisoner_id: String,
    pub case_number: String,
    pub offense_type: String,  // Bailable, Non-bailable
    pub arrest_date: String,
    pub detention_days: u32,
    pub statutory_limit_days: u32,
    pub days_remaining: i32,
    pub status: String,  // Within limit, Approaching limit, Exceeded limit
    pub urgent_action_required: bool,
}

// ── Bail Compliance Monitoring ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BailCompliance {
    pub prisoner_id: String,
    pub bail_conditions: Vec<BailCondition>,
    pub compliance_status: String,
    pub violations: Vec<Violation>,
    pub last_check_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BailCondition {
    pub condition_id: String,
    pub description: String,
    pub compliance: bool,
    pub last_verified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub violation_id: String,
    pub condition_id: String,
    pub description: String,
    pub date: String,
    pub severity: String,
}

// ── Arnesh Kumar Judgment Checklist ───────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArneshKumarChecklist {
    pub case_number: String,
    pub prisoner_id: String,
    pub offense: String,
    pub maximum_punishment_years: u32,
    pub detention_duration_days: u32,
    pub checklist_items: Vec<ChecklistItem>,
    pub overall_compliance: bool,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub item_id: String,
    pub description: String,
    pub compliant: bool,
    pub remarks: String,
}

// ── Prison Client ─────────────────────────────────────────────────────

pub struct PrisonClient {
    base_url: String,
    api_key: String,
}

impl PrisonClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
        }
    }

    /// Get prisoner information
    pub fn get_prisoner(&self, prisoner_id: &str) -> Result<Prisoner, String> {
        // In production: Make HTTP GET request to ePrisons API
        // For now: Return mock prisoner data
        Ok(Prisoner {
            prisoner_id: prisoner_id.to_string(),
            name: "Prisoner Name".to_string(),
            father_name: "Father Name".to_string(),
            age: 35,
            gender: "Male".to_string(),
            category: "Under-trial".to_string(),
            admission_date: "2024-01-15".to_string(),
            prison: Prison {
                prison_id: "PRN001".to_string(),
                name: "Central Prison".to_string(),
                state: "Maharashtra".to_string(),
                district: "Mumbai".to_string(),
                capacity: 2000,
                current_population: 1800,
            },
            offense: "IPC Section 420".to_string(),
            case_number: "CC/2024/001234".to_string(),
            court: "Sessions Court".to_string(),
            sentence: None,
            bail_details: None,
            rehabilitation: Some(Rehabilitation {
                programme_id: "REHAB001".to_string(),
                programme_name: "Vocational Training".to_string(),
                start_date: "2024-02-01".to_string(),
                completion_date: None,
                status: "In Progress".to_string(),
                skills: vec![
                    "Carpentry".to_string(),
                    "Welding".to_string(),
                ],
            }),
        })
    }

    /// Track undertrial detention time
    pub fn track_undertrial(&self, prisoner_id: &str) -> Result<UndertrialTracker, String> {
        // In production: Make HTTP GET request to ICJS API
        // For now: Return mock tracker data
        let detention_days = 180;
        let statutory_limit = 365; // For offenses punishable up to 7 years
        let days_remaining = (statutory_limit as i32) - (detention_days as i32);
        
        Ok(UndertrialTracker {
            prisoner_id: prisoner_id.to_string(),
            case_number: "CC/2024/001234".to_string(),
            offense_type: "Non-bailable".to_string(),
            arrest_date: "2024-01-15".to_string(),
            detention_days,
            statutory_limit_days: statutory_limit,
            days_remaining,
            status: if days_remaining > 90 { "Within limit".to_string() } 
                    else if days_remaining > 0 { "Approaching limit".to_string() }
                    else { "Exceeded limit".to_string() },
            urgent_action_required: days_remaining < 30,
        })
    }

    /// Monitor bail compliance
    pub fn monitor_bail_compliance(&self, prisoner_id: &str) -> Result<BailCompliance, String> {
        // In production: Make HTTP GET request to bail monitoring API
        // For now: Return mock compliance data
        Ok(BailCompliance {
            prisoner_id: prisoner_id.to_string(),
            bail_conditions: vec![
                BailCondition {
                    condition_id: "BC001".to_string(),
                    description: "Report to police station every Monday".to_string(),
                    compliance: true,
                    last_verified: "2024-07-15".to_string(),
                },
                BailCondition {
                    condition_id: "BC002".to_string(),
                    description: "Do not leave city limits".to_string(),
                    compliance: true,
                    last_verified: "2024-07-15".to_string(),
                },
            ],
            compliance_status: "Compliant".to_string(),
            violations: vec![],
            last_check_date: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Run Arnesh Kumar judgment checklist
    pub fn run_arnesh_kumar_checklist(&self, case_number: &str, prisoner_id: &str) -> Result<ArneshKumarChecklist, String> {
        // In production: Make HTTP GET request to case details API
        // For now: Return mock checklist
        Ok(ArneshKumarChecklist {
            case_number: case_number.to_string(),
            prisoner_id: prisoner_id.to_string(),
            offense: "IPC Section 420".to_string(),
            maximum_punishment_years: 7,
            detention_duration_days: 180,
            checklist_items: vec![
                ChecklistItem {
                    item_id: "AK001".to_string(),
                    description: "Offense punishable with imprisonment up to 7 years".to_string(),
                    compliant: true,
                    remarks: "Section 436 CrPC applies".to_string(),
                },
                ChecklistItem {
                    item_id: "AK002".to_string(),
                    description: "Detention within statutory limit".to_string(),
                    compliant: true,
                    remarks: "180 days detained, limit is 365 days".to_string(),
                },
                ChecklistItem {
                    item_id: "AK003".to_string(),
                    description: "First-time offender".to_string(),
                    compliant: true,
                    remarks: "No prior criminal record".to_string(),
                },
            ],
            overall_compliance: true,
            recommendation: "Consider bail application under Section 436 CrPC".to_string(),
        })
    }

    /// Get rehabilitation programmes
    pub fn get_rehabilitation_programmes(&self, prison_id: &str) -> Result<Vec<Rehabilitation>> {
        // In production: Make HTTP GET request to rehabilitation API
        // For now: Return mock programmes
        Ok(vec![
            Rehabilitation {
                programme_id: "REHAB001".to_string(),
                programme_name: "Vocational Training".to_string(),
                start_date: "2024-01-01".to_string(),
                completion_date: None,
                status: "Active".to_string(),
                skills: vec![
                    "Carpentry".to_string(),
                    "Welding".to_string(),
                    "Electrical work".to_string(),
                ],
            },
            Rehabilitation {
                programme_id: "REHAB002".to_string(),
                programme_name: "Literacy Programme".to_string(),
                start_date: "2024-01-01".to_string(),
                completion_date: None,
                status: "Active".to_string(),
                skills: vec![
                    "Basic literacy".to_string(),
                    "Numeracy".to_string(),
                ],
            },
        ])
    }

    /// Update prisoner rehabilitation status
    pub fn update_rehabilitation(&self, prisoner_id: &str, programme_id: &str, status: &str) -> Result<bool, String> {
        // In production: Make HTTP POST request to rehabilitation API
        // For now: Return success
        Ok(true)
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn prison_client_create(base_url: *const u8, base_url_len: usize,
                                     api_key: *const u8, api_key_len: usize) -> *mut PrisonClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(PrisonClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn prison_client_destroy(client: *mut PrisonClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn prison_track_undertrial(client: *const PrisonClient,
                                        prisoner_id: *const u8, pid_len: usize,
                                        out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || prisoner_id.is_null() { return -1; }
        let prisoner_id = String::from_utf8_unchecked(
            std::slice::from_raw_parts(prisoner_id, pid_len));
        match (*client).track_undertrial(&prisoner_id) {
            Ok(tracker) => {
                let json = serde_json::to_string(&tracker).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
