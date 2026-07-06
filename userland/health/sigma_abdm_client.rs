// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/health/sigma_abdm_client.rs — ABDM FHIR API Client
// Implements client for Ayushman Bharat Digital Mission (ABDM) FHIR API
//
// Features:
//   - FHIR R4 resource handling
//   - Health ID (ABHA) authentication
//   - Patient data retrieval
//   - Consent management
//   - HIP (Health Information Provider) integration
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── ABDM FHIR Resources (R4) ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient {
    pub id: String,
    pub name: Vec<HumanName>,
    pub birth_date: String,
    pub gender: String,
    pub telecom: Vec<ContactPoint>,
    pub address: Vec<Address>,
    pub identifier: Vec<Identifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanName {
    pub use_: Option<String>,
    pub family: String,
    pub given: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactPoint {
    pub system: String,
    pub value: String,
    pub use_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub use_: Option<String>,
    pub line: Vec<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub system: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: String,
    pub status: String,
    pub code: CodeableConcept,
    pub subject: Reference,
    pub effective_date_time: Option<String>,
    pub value_quantity: Option<Quantity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeableConcept {
    pub coding: Vec<Coding>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coding {
    pub system: String,
    pub code: String,
    pub display: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub system: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub reference: String,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consent {
    pub id: String,
    pub status: String,
    pub scope: CodeableConcept,
    pub category: Vec<CodeableConcept>,
    pub patient: Reference,
    pub provision: Vec<Provision>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provision {
    pub period: Option<Period>,
    pub action: Vec<CodeableConcept>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    pub start: String,
    pub end: Option<String>,
}

// ── ABDM Authentication ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbhaAuthRequest {
    pub auth_mode: String,
    pub health_id: Option<String>,
    pub mobile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbhaAuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbhaProfile {
    pub health_id: String,
    pub health_id_number: String,
    pub name: String,
    pub gender: String,
    pub date_of_birth: String,
    pub address: String,
    pub state: String,
    pub district: String,
}

// ── ABDM Client ─────────────────────────────────────────────────────────

pub struct AbdmClient {
    base_url: String,
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
}

impl AbdmClient {
    pub fn new(base_url: String, client_id: String, client_secret: String) -> Self {
        Self {
            base_url,
            client_id,
            client_secret,
            access_token: None,
        }
    }

    /// Authenticate with ABDM gateway
    pub fn authenticate(&mut self) -> Result<AbhaAuthResponse, String> {
        // In production: Make HTTP request to ABDM auth endpoint
        // For now: Return mock response
        let response = AbhaAuthResponse {
            access_token: format!("mock_token_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
            refresh_token: "mock_refresh_token".to_string(),
            expires_in: 3600,
        };
        self.access_token = Some(response.access_token.clone());
        Ok(response)
    }

    /// Get patient by ABHA ID
    pub fn get_patient(&self, abha_id: &str) -> Result<Patient, String> {
        let token = self.access_token.as_ref()
            .ok_or("Not authenticated")?;
        
        // In production: Make HTTP GET request to /Patient/{id}
        // For now: Return mock patient
        Ok(Patient {
            id: abha_id.to_string(),
            name: vec![HumanName {
                use_: Some("official".to_string()),
                family: "Doe".to_string(),
                given: vec!["John".to_string()],
            }],
            birth_date: "1990-01-01".to_string(),
            gender: "male".to_string(),
            telecom: vec![ContactPoint {
                system: "phone".to_string(),
                value: "+919876543210".to_string(),
                use_: Some("mobile".to_string()),
            }],
            address: vec![Address {
                use_: Some("home".to_string()),
                line: vec!["123 Main Street".to_string()],
                city: "Mumbai".to_string(),
                state: "Maharashtra".to_string(),
                postal_code: "400001".to_string(),
                country: "India".to_string(),
            }],
            identifier: vec![Identifier {
                system: "https://abdm.gov.in/abha".to_string(),
                value: abha_id.to_string(),
            }],
        })
    }

    /// Get observations for a patient
    pub fn get_observations(&self, patient_id: &str) -> Result<Vec<Observation>, String> {
        let _token = self.access_token.as_ref()
            .ok_or("Not authenticated")?;
        
        // In production: Make HTTP GET request to /Observation?patient={id}
        // For now: Return mock observations
        Ok(vec![
            Observation {
                id: "obs1".to_string(),
                status: "final".to_string(),
                code: CodeableConcept {
                    coding: vec![Coding {
                        system: "http://loinc.org".to_string(),
                        code: "8480-6".to_string(),
                        display: "Systolic blood pressure".to_string(),
                    }],
                    text: "Blood Pressure".to_string(),
                },
                subject: Reference {
                    reference: format!("Patient/{}", patient_id),
                    display: None,
                },
                effective_date_time: Some("2024-01-15T10:00:00Z".to_string()),
                value_quantity: Some(Quantity {
                    value: 120.0,
                    unit: "mmHg".to_string(),
                    system: Some("http://unitsofmeasure.org".to_string()),
                    code: Some("mm[Hg]".to_string()),
                }),
            },
        ])
    }

    /// Create consent request
    pub fn create_consent(&self, patient_id: &str, hip_id: &str, purpose: &str) -> Result<Consent, String> {
        let _token = self.access_token.as_ref()
            .ok_or("Not authenticated")?;
        
        // In production: Make HTTP POST请求 to /Consent
        // For now: Return mock consent
        Ok(Consent {
            id: format!("consent_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            status: "active".to_string(),
            scope: CodeableConcept {
                coding: vec![Coding {
                    system: "http://abdm.gov.in/consent-scope".to_string(),
                    code: "patient-data".to_string(),
                    display: "Patient Data Access".to_string(),
                }],
                text: purpose.to_string(),
            },
            category: vec![CodeableConcept {
                coding: vec![Coding {
                    system: "http://abdm.gov.in/consent-category".to_string(),
                    code: "health-data".to_string(),
                    display: "Health Data".to_string(),
                }],
                text: "Health Information".to_string(),
            }],
            patient: Reference {
                reference: format!("Patient/{}", patient_id),
                display: None,
            },
            provision: vec![Provision {
                period: Some(Period {
                    start: "2024-01-01T00:00:00Z".to_string(),
                    end: Some("2024-12-31T23:59:59Z".to_string()),
                }),
                action: vec![CodeableConcept {
                    coding: vec![Coding {
                        system: "http://abdm.gov.in/consent-action".to_string(),
                        code: "view".to_string(),
                        display: "View".to_string(),
                    }],
                    text: "View Data".to_string(),
                }],
            }],
        })
    }

    /// Get ABHA profile
    pub fn get_abha_profile(&self, health_id: &str) -> Result<AbhaProfile, String> {
        let _token = self.access_token.as_ref()
            .ok_or("Not authenticated")?;
        
        // In production: Make HTTP GET request to ABHA profile endpoint
        // For now: Return mock profile
        Ok(AbhaProfile {
            health_id: health_id.to_string(),
            health_id_number: "91-1234-5678-9012".to_string(),
            name: "John Doe".to_string(),
            gender: "Male".to_string(),
            date_of_birth: "1990-01-01".to_string(),
            address: "123 Main Street, Mumbai".to_string(),
            state: "Maharashtra".to_string(),
            district: "Mumbai City".to_string(),
        })
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn abdm_client_create(base_url: *const u8, base_url_len: usize,
                                     client_id: *const u8, client_id_len: usize,
                                     client_secret: *const u8, client_secret_len: usize) -> *mut AbdmClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let client_id = String::from_utf8_unchecked(
            std::slice::from_raw_parts(client_id, client_id_len));
        let client_secret = String::from_utf8_unchecked(
            std::slice::from_raw_parts(client_secret, client_secret_len));
        Box::into_raw(Box::new(AbdmClient::new(base_url, client_id, client_secret)))
    }
}

#[no_mangle]
pub extern "C" fn abdm_client_destroy(client: *mut AbdmClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn abdm_authenticate(client: *mut AbdmClient) -> i32 {
    unsafe {
        if client.is_null() { return -1; }
        match (*client).authenticate() {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn abdm_get_patient(client: *const AbdmClient, abha_id: *const u8, abha_id_len: usize,
                                   out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || abha_id.is_null() { return -1; }
        let abha_id = String::from_utf8_unchecked(
            std::slice::from_raw_parts(abha_id, abha_id_len));
        match (*client).get_patient(&abha_id) {
            Ok(patient) => {
                let json = serde_json::to_string(&patient).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
