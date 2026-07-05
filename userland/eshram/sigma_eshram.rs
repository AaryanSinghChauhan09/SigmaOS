// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/eshram/sigma_eshram.rs — e-Shram (Unorganised Worker Platform)
// Implements integration with e-Shram portal for unorganised workers
//
// Features:
//   - 300 million unorganised workers. sigma-ultra is perfect for them.
//   - e-Shram profile update via feature phone text mode
//   - PMJJBY/PMSBY/PMSYM scheme linking
//   - Seasonal employment calendar
//   - BoCW cess management for construction employers
//   - Gig worker compliance (Code on Social Security §113)
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── e-Shram Worker Profile ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EshramWorker {
    pub uan: String,  // Universal Account Number
    pub name: String,
    pub father_name: String,
    pub mother_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub mobile: String,
    pub aadhaar: String,
    pub address: Address,
    pub occupation: String,
    pub skill_type: String,
    pub work_type: String,  // Home-based, Self-employed, Street vendor, etc.
    pub registration_date: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: String,
    pub village: String,
    pub district: String,
    pub state: String,
    pub pincode: String,
}

// ── Employment Record ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentRecord {
    pub record_id: String,
    pub uan: String,
    pub employer_name: String,
    pub employer_type: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub wages_per_day: f64,
    pub work_location: String,
    pub is_current: bool,
}

// ── Social Security Scheme ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialSecurityScheme {
    pub scheme_id: String,
    pub uan: String,
    pub scheme_name: String,  // PMJJBY, PMSBY, PMSYM
    pub enrollment_date: String,
    pub premium_amount: f64,
    pub coverage_amount: f64,
    pub status: String,
    pub expiry_date: Option<String>,
}

// ── Seasonal Employment Calendar ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalEmployment {
    pub calendar_id: String,
    pub uan: String,
    pub season: String,
    pub start_date: String,
    pub end_date: String,
    pub expected_earnings: f64,
    pub location: String,
    pub employer: String,
}

// ── BoCW Cess (Building and Other Construction Workers) ─────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BocwCess {
    pub cess_id: String,
    pub employer_id: String,
    pub project_name: String,
    pub project_location: String,
    pub total_workers: u32,
    pub cess_amount: f64,
    pub payment_date: String,
    pub status: String,
}

// ── Gig Worker Compliance ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GigWorkerCompliance {
    pub compliance_id: String,
    pub platform_name: String,
    pub worker_count: u32,
    pub compliance_type: String,
    pub registration_date: String,
    pub status: String,
}

// ── e-Shram Client ─────────────────────────────────────────────────

pub struct EshramClient {
    base_url: String,
    api_key: String,
}

impl EshramClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
        }
    }

    /// Register worker on e-Shram portal
    pub fn register_worker(&self, worker: &EshramWorker) -> Result<String, String> {
        // In production: Make HTTP POST request to e-Shram API
        // For now: Return mock UAN
        Ok(format!("UAN_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get worker profile by UAN
    pub fn get_worker_profile(&self, uan: &str) -> Result<EshramWorker, String> {
        // In production: Make HTTP GET request to e-Shram API
        // For now: Return mock profile
        Ok(EshramWorker {
            uan: uan.to_string(),
            name: "Worker Name".to_string(),
            father_name: "Father Name".to_string(),
            mother_name: "Mother Name".to_string(),
            date_of_birth: "1985-05-15".to_string(),
            gender: "Male".to_string(),
            mobile: "+919876543210".to_string(),
            aadhaar: "1234-5678-9012".to_string(),
            address: Address {
                line1: "123 Village Road".to_string(),
                line2: "".to_string(),
                village: "Sample Village".to_string(),
                district: "District".to_string(),
                state: "State".to_string(),
                pincode: "123456".to_string(),
            },
            occupation: "Construction Worker".to_string(),
            skill_type: "Skilled".to_string(),
            work_type: "Self-employed".to_string(),
            registration_date: "2024-01-15".to_string(),
            status: "Active".to_string(),
        })
    }

    /// Update worker profile
    pub fn update_profile(&self, uan: &str, worker: &EshramWorker) -> Result<bool, String> {
        // In production: Make HTTP POST request to e-Shram API
        // For now: Return success
        Ok(true)
    }

    /// Add employment record
    pub fn add_employment_record(&self, uan: &str, record: &EmploymentRecord) -> Result<String, String> {
        // In production: Make HTTP POST request to e-Shram API
        // For now: Return mock record ID
        Ok(format!("EMP_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get employment history
    pub fn get_employment_history(&self, uan: &str) -> Result<Vec<EmploymentRecord>, String> {
        // In production: Make HTTP GET request to e-Shram API
        // For now: Return mock history
        Ok(vec![
            EmploymentRecord {
                record_id: "EMP001".to_string(),
                uan: uan.to_string(),
                employer_name: "Construction Company A".to_string(),
                employer_type: "Private".to_string(),
                start_date: "2024-01-01".to_string(),
                end_date: Some("2024-03-31".to_string()),
                wages_per_day: 500.0,
                work_location: "Mumbai".to_string(),
                is_current: false,
            },
            EmploymentRecord {
                record_id: "EMP002".to_string(),
                uan: uan.to_string(),
                employer_name: "Construction Company B".to_string(),
                employer_type: "Private".to_string(),
                start_date: "2024-04-01".to_string(),
                end_date: None,
                wages_per_day: 550.0,
                work_location: "Pune".to_string(),
                is_current: true,
            },
        ])
    }

    /// Link social security scheme
    pub fn link_scheme(&self, uan: &str, scheme: &SocialSecurityScheme) -> Result<String, String> {
        // In production: Make HTTP POST request to scheme API
        // For now: Return mock scheme ID
        Ok(format!("SS_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get linked schemes
    pub fn get_linked_schemes(&self, uan: &str) -> Result<Vec<SocialSecurityScheme>, String> {
        // In production: Make HTTP GET request to e-Shram API
        // For now: Return mock schemes
        Ok(vec![
            SocialSecurityScheme {
                scheme_id: "SS001".to_string(),
                uan: uan.to_string(),
                scheme_name: "PMJJBY".to_string(),
                enrollment_date: "2024-01-20".to_string(),
                premium_amount: 330.0,
                coverage_amount: 2000000.0,
                status: "Active".to_string(),
                expiry_date: Some("2025-01-19".to_string()),
            },
            SocialSecurityScheme {
                scheme_id: "SS002".to_string(),
                uan: uan.to_string(),
                scheme_name: "PMSBY".to_string(),
                enrollment_date: "2024-01-20".to_string(),
                premium_amount: 20.0,
                coverage_amount: 200000.0,
                status: "Active".to_string(),
                expiry_date: Some("2025-01-19".to_string()),
            },
        ])
    }

    /// Add seasonal employment
    pub fn add_seasonal_employment(&self, uan: &str, employment: &SeasonalEmployment) -> Result<String, String> {
        // In production: Make HTTP POST request to e-Shram API
        // For now: Return mock calendar ID
        Ok(format!("SE_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get seasonal employment calendar
    pub fn get_seasonal_calendar(&self, uan: &str) -> Result<Vec<SeasonalEmployment>, String> {
        // In production: Make HTTP GET request to e-Shram API
        // For now: Return mock calendar
        Ok(vec![
            SeasonalEmployment {
                calendar_id: "SE001".to_string(),
                uan: uan.to_string(),
                season: "Kharif".to_string(),
                start_date: "2024-06-01".to_string(),
                end_date: "2024-10-31".to_string(),
                expected_earnings: 45000.0,
                location: "Punjab".to_string(),
                employer: "Farm Owner A".to_string(),
            },
        ])
    }

    /// Submit BoCW cess
    pub fn submit_bocw_cess(&self, cess: &BocwCess) -> Result<String, String> {
        // In production: Make HTTP POST request to BoCW API
        // For now: Return mock cess ID
        Ok(format!("BOCW_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Register gig worker compliance
    pub fn register_gig_compliance(&self, compliance: &GigWorkerCompliance) -> Result<String, String> {
        // In production: Make HTTP POST request to gig worker API
        // For now: Return mock compliance ID
        Ok(format!("GIG_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn eshram_client_create(base_url: *const u8, base_url_len: usize,
                                       api_key: *const u8, api_key_len: usize) -> *mut EshramClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(EshramClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn eshram_client_destroy(client: *mut EshramClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn eshram_get_worker_profile(client: *const EshramClient,
                                          uan: *const u8, uan_len: usize,
                                          out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || uan.is_null() { return -1; }
        let uan = String::from_utf8_unchecked(
            std::slice::from_raw_parts(uan, uan_len));
        match (*client).get_worker_profile(&uan) {
            Ok(worker) => {
                let json = serde_json::to_string(&worker).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
