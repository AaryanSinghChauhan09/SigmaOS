// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/cowin/sigma_cowin.rs — COWIN / U-WIN Immunisation
// Implements integration with CoWIN and U-WIN immunisation systems
//
// Features:
//   - Universal Immunisation Programme records in sigma-health/ABHA
//   - School entry health records (RTE + NHM)
//   - AEFI (Adverse Event Following Immunisation) reporting to CDSCO
//   - Pregnancy + child health tracking (JSSK/PMMVY)
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Immunisation Record ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunisationRecord {
    pub record_id: String,
    pub beneficiary_id: String,
    pub beneficiary_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub aadhaar: String,
    pub mobile: String,
    pub vaccinations: Vec<Vaccination>,
    pub abha_linked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vaccination {
    pub vaccination_id: String,
    pub vaccine_name: String,
    pub vaccine_type: String,  // BCG, OPV, DPT, Hepatitis B, etc.
    pub dose_number: u32,
    pub administration_date: String,
    pub administered_by: String,
    pub facility: String,
    pub batch_number: String,
    pub next_due_date: Option<String>,
    pub adverse_event: Option<String>,
}

// ── School Entry Health Record ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchoolHealthRecord {
    pub record_id: String,
    pub student_id: String,
    pub student_name: String,
    pub school_name: String,
    pub class: String,
    pub academic_year: String,
    pub immunisation_status: String,
    pub required_vaccinations: Vec<String>,
    pub completed_vaccinations: Vec<String>,
    pub pending_vaccinations: Vec<String>,
    pub health_checkup_date: Option<String>,
    pub rte_eligible: bool,
}

// ── AEFI Report (Adverse Event Following Immunisation) ─────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AefiReport {
    pub report_id: String,
    pub vaccination_id: String,
    pub beneficiary_id: String,
    pub event_date: String,
    pub event_type: String,  // Mild, Moderate, Severe
    pub symptoms: Vec<String>,
    pub treatment_given: String,
    pub outcome: String,
    pub reported_by: String,
    pub reported_date: String,
    pub status: String,
}

// ── Pregnancy Health Tracking ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PregnancyTracking {
    pub tracking_id: String,
    pub mother_id: String,
    pub mother_name: String,
    pub aadhaar: String,
    pub lmp_date: String,  // Last Menstrual Period
    pub edd_date: String,  // Expected Delivery Date
    pub anc_visits: Vec<AncVisit>,
    pub vaccinations: Vec<Vaccination>,
    pub jssk_registered: bool,
    pub pmmvy_beneficiary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncVisit {
    pub visit_id: String,
    pub visit_date: String,
    pub gestational_age_weeks: u32,
    pub blood_pressure: String,
    pub hemoglobin: f64,
    pub weight_kg: f64,
    pub fetal_heart_rate: Option<u32>,
    pub findings: String,
    pub next_visit_date: String,
}

// ── Child Health Tracking ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildHealthTracking {
    pub tracking_id: String,
    pub child_id: String,
    pub child_name: String,
    pub date_of_birth: String,
    pub mother_id: String,
    pub growth_monitoring: Vec<GrowthRecord>,
    pub developmental_milestones: Vec<DevelopmentalMilestone>,
    pub vaccinations: Vec<Vaccination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthRecord {
    pub record_id: String,
    pub date: String,
    pub age_months: u32,
    pub weight_kg: f64,
    pub height_cm: f64,
    pub head_circumference_cm: Option<f64>,
    pub nutritional_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentalMilestone {
    pub milestone_id: String,
    pub milestone_type: String,  // Motor, Language, Social, Cognitive
    pub description: String,
    pub expected_age_months: u32,
    pub achieved_age_months: u32,
    pub achieved: bool,
}

// ── CoWIN Client ─────────────────────────────────────────────────

pub struct CowinClient {
    base_url: String,
    api_key: String,
}

impl CowinClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
        }
    }

    /// Get immunisation record by beneficiary ID
    pub fn get_immunisation_record(&self, beneficiary_id: &str) -> Result<ImmunisationRecord, String> {
        // In production: Make HTTP GET request to CoWIN API
        // For now: Return mock record
        Ok(ImmunisationRecord {
            record_id: format!("IMM_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            beneficiary_id: beneficiary_id.to_string(),
            beneficiary_name: "Beneficiary Name".to_string(),
            date_of_birth: "2020-01-15".to_string(),
            gender: "Male".to_string(),
            aadhaar: "1234-5678-9012".to_string(),
            mobile: "+919876543210".to_string(),
            vaccinations: vec![
                Vaccination {
                    vaccination_id: "VAC001".to_string(),
                    vaccine_name: "BCG".to_string(),
                    vaccine_type: "BCG".to_string(),
                    dose_number: 1,
                    administration_date: "2020-02-15".to_string(),
                    administered_by: "Dr. Sharma".to_string(),
                    facility: "PHC Center".to_string(),
                    batch_number: "BATCH12345".to_string(),
                    next_due_date: Some("2020-03-15".to_string()),
                    adverse_event: None,
                },
                Vaccination {
                    vaccination_id: "VAC002".to_string(),
                    vaccine_name: "OPV".to_string(),
                    vaccine_type: "OPV".to_string(),
                    dose_number: 1,
                    administration_date: "2020-03-15".to_string(),
                    administered_by: "Dr. Sharma".to_string(),
                    facility: "PHC Center".to_string(),
                    batch_number: "BATCH12346".to_string(),
                    next_due_date: Some("2020-04-15".to_string()),
                    adverse_event: None,
                },
            ],
            abha_linked: true,
        })
    }

    /// Register vaccination
    pub fn register_vaccination(&self, beneficiary_id: &str, vaccination: &Vaccination) -> Result<String, String> {
        // In production: Make HTTP POST request to CoWIN API
        // For now: Return mock vaccination ID
        Ok(format!("VAC_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get school health record
    pub fn get_school_health_record(&self, student_id: &str) -> Result<SchoolHealthRecord, String> {
        // In production: Make HTTP GET request to U-WIN API
        // For now: Return mock record
        Ok(SchoolHealthRecord {
            record_id: format!("SCH_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            student_id: student_id.to_string(),
            student_name: "Student Name".to_string(),
            school_name: "Government School".to_string(),
            class: "1st Standard".to_string(),
            academic_year: "2024-25".to_string(),
            immunisation_status: "Complete".to_string(),
            required_vaccinations: vec![
                "BCG".to_string(),
                "OPV".to_string(),
                "DPT".to_string(),
                "Hepatitis B".to_string(),
                "MMR".to_string(),
            ],
            completed_vaccinations: vec![
                "BCG".to_string(),
                "OPV".to_string(),
                "DPT".to_string(),
                "Hepatitis B".to_string(),
                "MMR".to_string(),
            ],
            pending_vaccinations: vec![],
            health_checkup_date: Some("2024-06-15".to_string()),
            rte_eligible: true,
        })
    }

    /// Report AEFI
    pub fn report_aefi(&self, report: &AefiReport) -> Result<String, String> {
        // In production: Make HTTP POST request to CDSCO API
        // For now: Return mock report ID
        Ok(format!("AEFI_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get pregnancy tracking
    pub fn get_pregnancy_tracking(&self, mother_id: &str) -> Result<PregnancyTracking, String> {
        // In production: Make HTTP GET request to JSSK API
        // For now: Return mock tracking
        Ok(PregnancyTracking {
            tracking_id: format!("PREG_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            mother_id: mother_id.to_string(),
            mother_name: "Mother Name".to_string(),
            aadhaar: "1234-5678-9012".to_string(),
            lmp_date: "2024-01-15".to_string(),
            edd_date: "2024-10-22".to_string(),
            anc_visits: vec![
                AncVisit {
                    visit_id: "ANC001".to_string(),
                    visit_date: "2024-03-15".to_string(),
                    gestational_age_weeks: 8,
                    blood_pressure: "120/80".to_string(),
                    hemoglobin: 12.5,
                    weight_kg: 55.0,
                    fetal_heart_rate: Some(160),
                    findings: "Normal".to_string(),
                    next_visit_date: "2024-05-15".to_string(),
                },
            ],
            vaccinations: vec![
                Vaccination {
                    vaccination_id: "VAC_TT1".to_string(),
                    vaccine_name: "Tetanus Toxoid".to_string(),
                    vaccine_type: "TT".to_string(),
                    dose_number: 1,
                    administration_date: "2024-02-15".to_string(),
                    administered_by: "Dr. Sharma".to_string(),
                    facility: "PHC Center".to_string(),
                    batch_number: "BATCH_TT123".to_string(),
                    next_due_date: Some("2024-08-15".to_string()),
                    adverse_event: None,
                },
            ],
            jssk_registered: true,
            pmmvy_beneficiary: true,
        })
    }

    /// Get child health tracking
    pub fn get_child_health_tracking(&self, child_id: &str) -> Result<ChildHealthTracking, String> {
        // In production: Make HTTP GET request to child health API
        // For now: Return mock tracking
        Ok(ChildHealthTracking {
            tracking_id: format!("CHILD_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            child_id: child_id.to_string(),
            child_name: "Child Name".to_string(),
            date_of_birth: "2024-01-15".to_string(),
            mother_id: "MOTHER001".to_string(),
            growth_monitoring: vec![
                GrowthRecord {
                    record_id: "GR001".to_string(),
                    date: "2024-04-15".to_string(),
                    age_months: 3,
                    weight_kg: 6.5,
                    height_cm: 62.0,
                    head_circumference_cm: Some(40.5),
                    nutritional_status: "Normal".to_string(),
                },
            ],
            developmental_milestones: vec![
                DevelopmentalMilestone {
                    milestone_id: "DM001".to_string(),
                    milestone_type: "Motor".to_string(),
                    description: "Holds head steady".to_string(),
                    expected_age_months: 3,
                    achieved_age_months: 3,
                    achieved: true,
                },
            ],
            vaccinations: vec![
                Vaccination {
                    vaccination_id: "VAC_BCG".to_string(),
                    vaccine_name: "BCG".to_string(),
                    vaccine_type: "BCG".to_string(),
                    dose_number: 1,
                    administration_date: "2024-02-15".to_string(),
                    administered_by: "Dr. Sharma".to_string(),
                    facility: "PHC Center".to_string(),
                    batch_number: "BATCH12345".to_string(),
                    next_due_date: Some("2024-03-15".to_string()),
                    adverse_event: None,
                },
            ],
        })
    }

    /// Link ABHA to immunisation record
    pub fn link_abha(&self, beneficiary_id: &str, abha_number: &str) -> Result<bool, String> {
        // In production: Make HTTP POST request to CoWIN API
        // For now: Return success
        Ok(true)
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn cowin_client_create(base_url: *const u8, base_url_len: usize,
                                     api_key: *const u8, api_key_len: usize) -> *mut CowinClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(CowinClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn cowin_client_destroy(client: *mut CowinClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn cowin_get_record(client: *const CowinClient,
                                  beneficiary_id: *const u8, ben_len: usize,
                                  out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || beneficiary_id.is_null() { return -1; }
        let beneficiary_id = String::from_utf8_unchecked(
            std::slice::from_raw_parts(beneficiary_id, ben_len));
        match (*client).get_immunisation_record(&beneficiary_id) {
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
