// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/census/sigma_census.rs — Population Survey Tool
// Implements integration with Indian census and survey systems
//
// Features:
//   - Offline-capable for census enumerators (sigma-ultra + forms)
//   - DID-linked household identity (replaces paper slips)
//   - Real-time coverage dashboard (which areas enumerated vs. pending)
//   - NPR (National Population Register) data entry
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Household Record ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseholdRecord {
    pub household_id: String,
    pub household_number: String,
    pub did: String,  // Decentralized Identity
    pub head_of_household: String,
    pub address: Address,
    pub members: Vec<HouseholdMember>,
    pub dwelling_type: String,
    pub ownership_status: String,
    pub amenities: Vec<String>,
    pub enumerator_id: String,
    pub enumeration_date: String,
    pub status: String,
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
pub struct HouseholdMember {
    pub member_id: String,
    pub name: String,
    pub relationship_to_head: String,
    pub gender: String,
    pub date_of_birth: String,
    pub age: u32,
    pub marital_status: String,
    pub education: String,
    pub occupation: String,
    pub aadhaar: Option<String>,
    pub voter_id: Option<String>,
}

// ── Census Enumerator ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CensusEnumerator {
    pub enumerator_id: String,
    pub name: String,
    pub aadhaar: String,
    pub mobile: String,
    pub assigned_area: String,
    pub district: String,
    pub state: String,
    pub target_households: u32,
    pub completed_households: u32,
    pub status: String,
}

// ── Coverage Dashboard ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageDashboard {
    pub dashboard_id: String,
    pub district: String,
    pub state: String,
    pub total_households: u32,
    pub enumerated_households: u32,
    pub pending_households: u32,
    pub coverage_percentage: f64,
    pub area_breakdown: Vec<AreaCoverage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaCoverage {
    pub area_name: String,
    pub total_households: u32,
    pub enumerated_households: u32,
    pub pending_households: u32,
    pub coverage_percentage: f64,
}

// ── NPR Data Entry ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NprEntry {
    pub entry_id: String,
    pub household_id: String,
    pub member_id: String,
    pub npr_number: String,
    pub name: String,
    pub father_name: String,
    pub mother_name: String,
    pub spouse_name: Option<String>,
    pub date_of_birth: String,
    pub gender: String,
    pub marital_status: String,
    pub education: String,
    pub occupation: String,
    pub aadhaar: String,
    pub mobile: String,
    pub address: Address,
    pub photo: String,
    pub status: String,
}

// ── Census Client ─────────────────────────────────────────────────

pub struct CensusClient {
    base_url: String,
    api_key: String,
    enumerator_id: Option<String>,
}

impl CensusClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            enumerator_id: None,
        }
    }

    pub fn set_enumerator_id(&mut self, enumerator_id: String) {
        self.enumerator_id = Some(enumerator_id);
    }

    /// Create household record
    pub fn create_household(&self, household: &HouseholdRecord) -> Result<String, String> {
        // In production: Make HTTP POST request to Census API (or store offline)
        // For now: Return mock household ID
        Ok(format!("HH_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get household record
    pub fn get_household(&self, household_id: &str) -> Result<HouseholdRecord, String> {
        // In production: Make HTTP GET request to Census API (or load from offline storage)
        // For now: Return mock household
        Ok(HouseholdRecord {
            household_id: household_id.to_string(),
            household_number: "HH/2024/12345".to_string(),
            did: "did:sigmaos:1234567890abcdef".to_string(),
            head_of_household: "Head of Household".to_string(),
            address: Address {
                house_number: "123".to_string(),
                street: "Main Street".to_string(),
                locality: "Ward 5".to_string(),
                village: "Sample Village".to_string(),
                tehsil: "Tehsil".to_string(),
                district: "District".to_string(),
                state: "State".to_string(),
                pincode: "123456".to_string(),
            },
            members: vec![
                HouseholdMember {
                    member_id: "MEM001".to_string(),
                    name: "Head of Household".to_string(),
                    relationship_to_head: "Self".to_string(),
                    gender: "Male".to_string(),
                    date_of_birth: "1970-01-15".to_string(),
                    age: 54,
                    marital_status: "Married".to_string(),
                    education: "Graduate".to_string(),
                    occupation: "Farmer".to_string(),
                    aadhaar: Some("1234-5678-9012".to_string()),
                    voter_id: Some("ABC1234567".to_string()),
                },
                HouseholdMember {
                    member_id: "MEM002".to_string(),
                    name: "Spouse Name".to_string(),
                    relationship_to_head: "Spouse".to_string(),
                    gender: "Female".to_string(),
                    date_of_birth: "1975-05-20".to_string(),
                    age: 49,
                    marital_status: "Married".to_string(),
                    education: "Higher Secondary".to_string(),
                    occupation: "Homemaker".to_string(),
                    aadhaar: Some("2345-6789-0123".to_string()),
                    voter_id: Some("BCD2345678".to_string()),
                },
            ],
            dwelling_type: "Pucca".to_string(),
            ownership_status: "Owned".to_string(),
            amenities: vec![
                "Electricity".to_string(),
                "Water Supply".to_string(),
                "Toilet".to_string(),
                "Kitchen".to_string(),
            ],
            enumerator_id: "ENUM001".to_string(),
            enumeration_date: chrono::Utc::now().to_rfc3339(),
            status: "Completed".to_string(),
        })
    }

    /// Update household record
    pub fn update_household(&self, household_id: &str, household: &HouseholdRecord) -> Result<bool, String> {
        // In production: Make HTTP PUT request to Census API (or update offline storage)
        // For now: Return success
        Ok(true)
    }

    /// Get enumerator details
    pub fn get_enumerator(&self, enumerator_id: &str) -> Result<CensusEnumerator, String> {
        // In production: Make HTTP GET request to Census API
        // For now: Return mock enumerator
        Ok(CensusEnumerator {
            enumerator_id: enumerator_id.to_string(),
            name: "Enumerator Name".to_string(),
            aadhaar: "1234-5678-9012".to_string(),
            mobile: "+919876543210".to_string(),
            assigned_area: "Ward 5".to_string(),
            district: "District".to_string(),
            state: "State".to_string(),
            target_households: 500,
            completed_households: 350,
            status: "Active".to_string(),
        })
    }

    /// Get coverage dashboard
    pub fn get_coverage_dashboard(&self, district: &str) -> Result<CoverageDashboard, String> {
        // In production: Make HTTP GET request to Census API
        // For now: Return mock dashboard
        Ok(CoverageDashboard {
            dashboard_id: format!("DASH_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            district: district.to_string(),
            state: "State".to_string(),
            total_households: 100000,
            enumerated_households: 75000,
            pending_households: 25000,
            coverage_percentage: 75.0,
            area_breakdown: vec![
                AreaCoverage {
                    area_name: "Ward 1".to_string(),
                    total_households: 20000,
                    enumerated_households: 18000,
                    pending_households: 2000,
                    coverage_percentage: 90.0,
                },
                AreaCoverage {
                    area_name: "Ward 2".to_string(),
                    total_households: 25000,
                    enumerated_households: 20000,
                    pending_households: 5000,
                    coverage_percentage: 80.0,
                },
            ],
        })
    }

    /// Submit NPR entry
    pub fn submit_npr_entry(&self, entry: &NprEntry) -> Result<String, String> {
        // In production: Make HTTP POST request to NPR API
        // For now: Return mock entry ID
        Ok(format!("NPR_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get NPR entry
    pub fn get_npr_entry(&self, entry_id: &str) -> Result<NprEntry, String> {
        // In production: Make HTTP GET request to NPR API
        // For now: Return mock entry
        Ok(NprEntry {
            entry_id: entry_id.to_string(),
            household_id: "HH001".to_string(),
            member_id: "MEM001".to_string(),
            npr_number: "NPR1234567890".to_string(),
            name: "Person Name".to_string(),
            father_name: "Father Name".to_string(),
            mother_name: "Mother Name".to_string(),
            spouse_name: Some("Spouse Name".to_string()),
            date_of_birth: "1970-01-15".to_string(),
            gender: "Male".to_string(),
            marital_status: "Married".to_string(),
            education: "Graduate".to_string(),
            occupation: "Farmer".to_string(),
            aadhaar: "1234-5678-9012".to_string(),
            mobile: "+919876543210".to_string(),
            address: Address {
                house_number: "123".to_string(),
                street: "Main Street".to_string(),
                locality: "Ward 5".to_string(),
                village: "Sample Village".to_string(),
                tehsil: "Tehsil".to_string(),
                district: "District".to_string(),
                state: "State".to_string(),
                pincode: "123456".to_string(),
            },
            photo: "https://example.com/photo.jpg".to_string(),
            status: "Verified".to_string(),
        })
    }

    /// Sync offline data to server
    pub fn sync_offline_data(&self, enumerator_id: &str) -> Result<u32, String> {
        // In production: Upload all pending offline records to server
        // For now: Return mock sync count
        Ok(50)
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn census_client_create(base_url: *const u8, base_url_len: usize,
                                     api_key: *const u8, api_key_len: usize) -> *mut CensusClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(CensusClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn census_client_destroy(client: *mut CensusClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn census_create_household(client: *const CensusClient,
                                        household_json: *const u8, h_len: usize,
                                        out: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || household_json.is_null() { return -1; }
        let household_str = String::from_utf8_unchecked(
            std::slice::from_raw_parts(household_json, h_len));
        let household: HouseholdRecord = match serde_json::from_str(&household_str) {
            Ok(h) => h,
            Err(_) => return -1,
        };
        match (*client).create_household(&household) {
            Ok(household_id) => {
                let bytes = household_id.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
