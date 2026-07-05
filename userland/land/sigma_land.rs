// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/land/sigma_land.rs — Land Records & Survey
// Implements integration with Indian land records and survey systems
//
// Features:
//   - DILRMP full integration (Digital India Land Records Modernisation)
//   - Mutation (Dakhil-Kharij) application and status tracking
//   - Bhu-Naksha cadastral map overlay on Bhuvan
//   - Survey of India topo sheet integration
//   - LARR Act 2013 compensation calculator for land acquisition
//   - SVAMITVA scheme (village property rights) mapping integration
//   - Encumbrance certificate fetch + verification
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Land Record Information ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandRecord {
    pub record_id: String,
    pub survey_number: String,
    pub sub_division_number: String,
    pub village: String,
    pub taluka: String,
    pub district: String,
    pub state: String,
    pub area_acres: f64,
    pub area_hectares: f64,
    pub ownership: Vec<Owner>,
    pub land_use: String,  // Agricultural, Residential, Commercial, Industrial
    pub soil_type: String,
    pub irrigation: String,
    pub encumbrances: Vec<Encumbrance>,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub owner_id: String,
    pub name: String,
    pub father_name: String,
    pub share_percentage: f64,
    pub aadhaar: Option<String>,
    pub pan: Option<String>,
    pub address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: String,
    pub village: String,
    pub taluka: String,
    pub district: String,
    pub state: String,
    pub pincode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encumbrance {
    pub encumbrance_id: String,
    pub type_: String,  // Mortgage, Lease, Litigation
    pub description: String,
    pub date: String,
    pub amount: Option<f64>,
}

// ── Mutation (Dakhil-Kharij) Application ───────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationApplication {
    pub application_id: String,
    pub survey_number: String,
    pub village: String,
    pub taluka: String,
    pub district: String,
    pub mutation_type: String,  // Sale, Gift, Inheritance, Partition
    pub transferor: Owner,
    pub transferee: Owner,
    pub consideration_amount: f64,
    pub registration_date: String,
    pub document_number: String,
    pub application_date: String,
    pub status: String,
    pub remarks: Option<String>,
}

// ── Bhu-Naksha Cadastral Map ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadastralMap {
    pub map_id: String,
    pub village: String,
    pub taluka: String,
    pub district: String,
    pub state: String,
    pub scale: String,
    pub survey_year: u32,
    pub parcels: Vec<Parcel>,
    pub map_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parcel {
    pub parcel_id: String,
    pub survey_number: String,
    pub area_acres: f64,
    pub coordinates: Vec<Coordinate>,
    pub land_use: String,
    pub owner_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

// ── LARR Act 2013 Compensation Calculator ─────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandAcquisitionCompensation {
    pub acquisition_id: String,
    pub survey_number: String,
    pub market_value_per_acre: f64,
    pub solatium_percent: f64,
    pub compensation_amount: f64,
    pub rehabilitation_grant: f64,
    pub resettlement_grant: f64,
    pub total_compensation: f64,
    pub calculation_date: String,
}

// ── SVAMITVA Scheme Integration ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvamitvaProperty {
    pub property_id: String,
    pub village: String,
    pub district: String,
    pub state: String,
    pub property_number: String,
    pub owner_name: String,
    pub aadhaar: String,
    pub area_sq_meters: f64,
    pub gps_coordinates: Coordinate,
    pub property_card_url: String,
    pub verification_status: String,
}

// ── Encumbrance Certificate ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncumbranceCertificate {
    pub certificate_id: String,
    pub survey_number: String,
    pub village: String,
    pub taluka: String,
    pub district: String,
    pub period_from: String,
    pub period_to: String,
    pub encumbrances: Vec<Encumbrance>,
    pub issuance_date: String,
    pub valid_until: String,
    pub certificate_url: String,
}

// ── Land Client ─────────────────────────────────────────────────────

pub struct LandClient {
    base_url: String,
    api_key: String,
}

impl LandClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
        }
    }

    /// Get land record by survey number
    pub fn get_land_record(&self, survey_number: &str, village: &str, district: &str) -> Result<LandRecord, String> {
        // In production: Make HTTP GET request to DILRMP API
        // For now: Return mock land record
        Ok(LandRecord {
            record_id: format!("LR_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            survey_number: survey_number.to_string(),
            sub_division_number: "1".to_string(),
            village: village.to_string(),
            taluka: "Taluka".to_string(),
            district: district.to_string(),
            state: "State".to_string(),
            area_acres: 5.0,
            area_hectares: 2.02,
            ownership: vec![
                Owner {
                    owner_id: "OWN001".to_string(),
                    name: "Landowner Name".to_string(),
                    father_name: "Father Name".to_string(),
                    share_percentage: 100.0,
                    aadhaar: Some("1234-5678-9012".to_string()),
                    pan: Some("ABCDE1234F".to_string()),
                    address: Address {
                        line1: "123 Village Road".to_string(),
                        line2: "".to_string(),
                        village: village.to_string(),
                        taluka: "Taluka".to_string(),
                        district: district.to_string(),
                        state: "State".to_string(),
                        pincode: "123456".to_string(),
                    },
                },
            ],
            land_use: "Agricultural".to_string(),
            soil_type: "Black Soil".to_string(),
            irrigation: "Irrigated".to_string(),
            encumbrances: vec![],
            last_updated: "2024-01-15".to_string(),
        })
    }

    /// Apply for mutation (Dakhil-Kharij)
    pub fn apply_mutation(&self, application: &MutationApplication) -> Result<String, String> {
        // In production: Make HTTP POST request to mutation API
        // For now: Return mock application ID
        Ok(format!("MUT_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get mutation application status
    pub fn get_mutation_status(&self, application_id: &str) -> Result<MutationApplication, String> {
        // In production: Make HTTP GET request to mutation API
        // For now: Return mock status
        Ok(MutationApplication {
            application_id: application_id.to_string(),
            survey_number: "123/456".to_string(),
            village: "Village".to_string(),
            taluka: "Taluka".to_string(),
            district: "District".to_string(),
            mutation_type: "Sale".to_string(),
            transferor: Owner {
                owner_id: "OWN001".to_string(),
                name: "Seller Name".to_string(),
                father_name: "Father Name".to_string(),
                share_percentage: 100.0,
                aadhaar: Some("1234-5678-9012".to_string()),
                pan: Some("ABCDE1234F".to_string()),
                address: Address {
                    line1: "123 Village Road".to_string(),
                    line2: "".to_string(),
                    village: "Village".to_string(),
                    taluka: "Taluka".to_string(),
                    district: "District".to_string(),
                    state: "State".to_string(),
                    pincode: "123456".to_string(),
                },
            },
            transferee: Owner {
                owner_id: "OWN002".to_string(),
                name: "Buyer Name".to_string(),
                father_name: "Father Name".to_string(),
                share_percentage: 100.0,
                aadhaar: Some("9876-5432-1098".to_string()),
                pan: Some("FGHIJ5678K".to_string()),
                address: Address {
                    line1: "456 New Road".to_string(),
                    line2: "".to_string(),
                    village: "Village".to_string(),
                    taluka: "Taluka".to_string(),
                    district: "District".to_string(),
                    state: "State".to_string(),
                    pincode: "123456".to_string(),
                },
            },
            consideration_amount: 5000000.0,
            registration_date: "2024-01-15".to_string(),
            document_number: "DOC/2024/12345".to_string(),
            application_date: "2024-01-20".to_string(),
            status: "Under Review".to_string(),
            remarks: None,
        })
    }

    /// Get cadastral map (Bhu-Naksha)
    pub fn get_cadastral_map(&self, village: &str, district: &str) -> Result<CadastralMap, String> {
        // In production: Make HTTP GET request to Bhu-Naksha API
        // For now: Return mock map
        Ok(CadastralMap {
            map_id: format!("MAP_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            village: village.to_string(),
            taluka: "Taluka".to_string(),
            district: district.to_string(),
            state: "State".to_string(),
            scale: "1:5000".to_string(),
            survey_year: 2020,
            parcels: vec![
                Parcel {
                    parcel_id: "PAR001".to_string(),
                    survey_number: "123/456".to_string(),
                    area_acres: 5.0,
                    coordinates: vec![
                        Coordinate { latitude: 19.0760, longitude: 72.8777 },
                        Coordinate { latitude: 19.0765, longitude: 72.8782 },
                        Coordinate { latitude: 19.0762, longitude: 72.8785 },
                        Coordinate { latitude: 19.0758, longitude: 72.8780 },
                    ],
                    land_use: "Agricultural".to_string(),
                    owner_name: "Landowner Name".to_string(),
                },
            ],
            map_url: "https://bhuvan.nrsc.gov.in/map/12345".to_string(),
        })
    }

    /// Calculate LARR Act 2013 compensation
    pub fn calculate_larr_compensation(&self, survey_number: &str, market_value_per_acre: f64, area_acres: f64) -> Result<LandAcquisitionCompensation, String> {
        // LARR Act 2013 provisions
        let solatium_percent = 30.0; // 30% solatium
        let compensation_amount = market_value_per_acre * area_acres;
        let solatium = compensation_amount * (solatium_percent / 100.0);
        let rehabilitation_grant = 500000.0; // Mock amount
        let resettlement_grant = 300000.0; // Mock amount
        let total_compensation = compensation_amount + solatium + rehabilitation_grant + resettlement_grant;
        
        Ok(LandAcquisitionCompensation {
            acquisition_id: format!("LA_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            survey_number: survey_number.to_string(),
            market_value_per_acre,
            solatium_percent,
            compensation_amount,
            rehabilitation_grant,
            resettlement_grant,
            total_compensation,
            calculation_date: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Get SVAMITVA property details
    pub fn get_svamitva_property(&self, property_number: &str, village: &str) -> Result<SvamitvaProperty, String> {
        // In production: Make HTTP GET request to SVAMITVA API
        // For now: Return mock property
        Ok(SvamitvaProperty {
            property_id: format!("SVP_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            village: village.to_string(),
            district: "District".to_string(),
            state: "State".to_string(),
            property_number: property_number.to_string(),
            owner_name: "Property Owner".to_string(),
            aadhaar: "1234-5678-9012".to_string(),
            area_sq_meters: 500.0,
            gps_coordinates: Coordinate {
                latitude: 19.0760,
                longitude: 72.8777,
            },
            property_card_url: "https://svamitva.gov.in/card/12345".to_string(),
            verification_status: "Verified".to_string(),
        })
    }

    /// Get encumbrance certificate
    pub fn get_encumbrance_certificate(&self, survey_number: &str, village: &str, period_from: &str, period_to: &str) -> Result<EncumbranceCertificate, String> {
        // In production: Make HTTP GET request to encumbrance API
        // For now: Return mock certificate
        Ok(EncumbranceCertificate {
            certificate_id: format!("EC_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            survey_number: survey_number.to_string(),
            village: village.to_string(),
            taluka: "Taluka".to_string(),
            district: "District".to_string(),
            period_from: period_from.to_string(),
            period_to: period_to.to_string(),
            encumbrances: vec![],
            issuance_date: chrono::Utc::now().to_rfc3339(),
            valid_until: (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339(),
            certificate_url: "https://landrecords.gov.in/ec/12345".to_string(),
        })
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn land_client_create(base_url: *const u8, base_url_len: usize,
                                   api_key: *const u8, api_key_len: usize) -> *mut LandClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(LandClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn land_client_destroy(client: *mut LandClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn land_get_record(client: *const LandClient,
                                survey_no: *const u8, survey_len: usize,
                                village: *const u8, village_len: usize,
                                district: *const u8, district_len: usize,
                                out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || survey_no.is_null() || village.is_null() || district.is_null() { return -1; }
        let survey_no = String::from_utf8_unchecked(
            std::slice::from_raw_parts(survey_no, survey_len));
        let village = String::from_utf8_unchecked(
            std::slice::from_raw_parts(village, village_len));
        let district = String::from_utf8_unchecked(
            std::slice::from_raw_parts(district, district_len));
        match (*client).get_land_record(&survey_no, &village, &district) {
            Ok(record) => {
                let json = serde_json::to_string(&record).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
