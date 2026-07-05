// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/ayush/sigma_ayush.rs — AYUSH Healthcare Integration
// Implements integration with Indian AYUSH (Ayurveda, Yoga, Naturopathy, Unani, Siddha, Homeopathy) systems
//
// Features:
//   - AYUSH practitioner registry (CCIM/CCH/PCIM&H verification)
//   - Ayurvedic drug formulation database (AFI)
//   - Panchakarma treatment protocol logging
//   - AYUSH hospital NABH accreditation checklist
//   - Yoga therapy protocol management (Y-Break scheme)
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── AYUSH Practitioner Registry ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AyushPractitioner {
    pub practitioner_id: String,
    pub name: String,
    pub system: AyushSystem,  // Ayurveda, Yoga, Naturopathy, Unani, Siddha, Homeopathy
    pub registration_number: String,
    pub council: String,  // CCIM, CCH, PCIM&H, etc.
    pub qualification: String,
    pub college: String,
    pub year_of_registration: u32,
    pub state: String,
    pub district: String,
    pub clinic_address: Address,
    pub specialization: String,
    pub experience_years: u32,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AyushSystem {
    Ayurveda,
    Yoga,
    Naturopathy,
    Unani,
    Siddha,
    Homeopathy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: String,
    pub city: String,
    pub district: String,
    pub state: String,
    pub pincode: String,
}

// ── Ayurvedic Drug Formulation (AFI) ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AyurvedicFormulation {
    pub formulation_id: String,
    pub name: String,
    pub sanskrit_name: String,
    pub category: String,  // Rasayana, Kashaya, Churna, etc.
    pub ingredients: Vec<Ingredient>,
    pub preparation_method: String,
    pub dosage: String,
    pub indications: Vec<String>,
    pub contraindications: Vec<String>,
    pub reference: String,  // Classical text reference
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub botanical_name: String,
    pub part_used: String,
    pub quantity: String,
}

// ── Panchakarma Treatment ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanchakarmaTreatment {
    pub treatment_id: String,
    pub patient_id: String,
    pub practitioner_id: String,
    pub procedure: PanchakarmaProcedure,
    pub start_date: String,
    pub end_date: String,
    pub pre_procedure: Vec<String>,
    pub main_procedure: Vec<String>,
    pub post_procedure: Vec<String>,
    pub diet_recommendations: Vec<String>,
    pub lifestyle_recommendations: Vec<String>,
    pub outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PanchakarmaProcedure {
    Vamana,  // Emesis
    Virechana,  // Purgation
    Basti,  // Enema
    Nasya,  // Nasal administration
    Raktamokshana,  // Bloodletting
}

// ── NABH Accreditation ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NabhAccreditation {
    pub hospital_id: String,
    pub hospital_name: String,
    pub ayush_system: AyushSystem,
    pub accreditation_level: String,  // Entry Level, Progressive Level, Level 1, Level 2
    pub application_date: String,
    pub status: String,
    pub checklist: Vec<AccreditationItem>,
    pub score: f32,
    pub expiry_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccreditationItem {
    pub item_id: String,
    pub category: String,
    pub description: String,
    pub compliant: bool,
    pub evidence: Option<String>,
}

// ── Yoga Therapy Protocol (Y-Break) ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YogaProtocol {
    pub protocol_id: String,
    pub name: String,
    pub duration_minutes: u32,
    pub difficulty: String,  // Beginner, Intermediate, Advanced
    pub asanas: Vec<Asana>,
    pub pranayamas: Vec<Pranayama>,
    pub meditation: Option<Meditation>,
    pub contraindications: Vec<String>,
    pub benefits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asana {
    pub name: String,
    pub sanskrit_name: String,
    pub duration_seconds: u32,
    pub repetitions: u32,
    pub instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pranayama {
    pub name: String,
    pub sanskrit_name: String,
    pub duration_seconds: u32,
    pub rounds: u32,
    pub instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meditation {
    pub name: String,
    pub technique: String,
    pub duration_seconds: u32,
    pub instructions: String,
}

// ── AYUSH Client ─────────────────────────────────────────────────────────

pub struct AyushClient {
    base_url: String,
    api_key: String,
}

impl AyushClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
        }
    }

    /// Verify practitioner registration
    pub fn verify_practitioner(&self, registration_number: &str, council: &str) -> Result<AyushPractitioner, String> {
        // In production: Make HTTP GET request to AYUSH practitioner registry API
        // For now: Return mock practitioner
        Ok(AyushPractitioner {
            practitioner_id: format!("PRA_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            name: "Dr. Ayurveda Sharma".to_string(),
            system: AyushSystem::Ayurveda,
            registration_number: registration_number.to_string(),
            council: council.to_string(),
            qualification: "BAMS, MD (Ayurveda)".to_string(),
            college: "Government Ayurveda College".to_string(),
            year_of_registration: 2010,
            state: "Maharashtra".to_string(),
            district: "Mumbai".to_string(),
            clinic_address: Address {
                line1: "123, Ayurveda Lane".to_string(),
                line2: "Andheri West".to_string(),
                city: "Mumbai".to_string(),
                district: "Mumbai Suburban".to_string(),
                state: "Maharashtra".to_string(),
                pincode: "400058".to_string(),
            },
            specialization: "Panchakarma".to_string(),
            experience_years: 14,
            verified: true,
        })
    }

    /// Search Ayurvedic formulation by name
    pub fn search_formulation(&self, name: &str) -> Result<Vec<AyurvedicFormulation>, String> {
        // In production: Make HTTP GET request to AFI database API
        // For now: Return mock formulations
        Ok(vec![
            AyurvedicFormulation {
                formulation_id: "AFI001".to_string(),
                name: "Chyawanprash".to_string(),
                sanskrit_name: "च्यवनप्राश".to_string(),
                category: "Rasayana".to_string(),
                ingredients: vec![
                    Ingredient {
                        name: "Amalaki".to_string(),
                        botanical_name: "Emblica officinalis".to_string(),
                        part_used: "Fruit".to_string(),
                        quantity: "500 g".to_string(),
                    },
                    Ingredient {
                        name: "Guduchi".to_string(),
                        botanical_name: "Tinospora cordifolia".to_string(),
                        part_used: "Stem".to_string(),
                        quantity: "200 g".to_string(),
                    },
                ],
                preparation_method: "Classical preparation method".to_string(),
                dosage: "12 g twice daily with milk".to_string(),
                indications: vec![
                    "General debility".to_string(),
                    "Respiratory disorders".to_string(),
                    "Digestive disorders".to_string(),
                ],
                contraindications: vec![
                    "Diabetes (use sugar-free variant)".to_string(),
                ],
                reference: "Charaka Samhita, Chikitsa Sthana".to_string(),
            },
        ])
    }

    /// Log Panchakarma treatment
    pub fn log_panchakarma(&self, treatment: &PanchakarmaTreatment) -> Result<String, String> {
        // In production: Make HTTP POST request to treatment logging API
        // For now: Return mock treatment ID
        Ok(format!("PAN_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get NABH accreditation checklist
    pub fn get_nabh_checklist(&self, hospital_id: &str) -> Result<NabhAccreditation, String> {
        // In production: Make HTTP GET request to NABH API
        // For now: Return mock checklist
        Ok(NabhAccreditation {
            hospital_id: hospital_id.to_string(),
            hospital_name: "SigmaOS AYUSH Hospital".to_string(),
            ayush_system: AyushSystem::Ayurveda,
            accreditation_level: "Level 1".to_string(),
            application_date: "2024-01-15".to_string(),
            status: "In Progress".to_string(),
            checklist: vec![
                AccreditationItem {
                    item_id: "ACC001".to_string(),
                    category: "Patient Care".to_string(),
                    description: "Standard Operating Procedures for patient care".to_string(),
                    compliant: true,
                    evidence: Some("SOP Document".to_string()),
                },
                AccreditationItem {
                    item_id: "ACC002".to_string(),
                    category: "Quality Management".to_string(),
                    description: "Quality improvement program".to_string(),
                    compliant: false,
                    evidence: None,
                },
            ],
            score: 75.0,
            expiry_date: None,
        })
    }

    /// Get Yoga protocol (Y-Break)
    pub fn get_yoga_protocol(&self, protocol_id: &str) -> Result<YogaProtocol, String> {
        // In production: Make HTTP GET request to Y-Break API
        // For now: Return mock protocol
        Ok(YogaProtocol {
            protocol_id: protocol_id.to_string(),
            name: "Office Y-Break Protocol".to_string(),
            duration_minutes: 5,
            difficulty: "Beginner".to_string(),
            asanas: vec![
                Asana {
                    name: "Neck Rotation".to_string(),
                    sanskrit_name: "Greeva Sanchalana".to_string(),
                    duration_seconds: 30,
                    repetitions: 5,
                    instructions: "Gently rotate neck clockwise and anticlockwise".to_string(),
                },
                Asana {
                    name: "Shoulder Rotation".to_string(),
                    sanskrit_name: "Skandha Sanchalana".to_string(),
                    duration_seconds: 30,
                    repetitions: 5,
                    instructions: "Rotate shoulders clockwise and anticlockwise".to_string(),
                },
            ],
            pranayamas: vec![
                Pranayama {
                    name: "Deep Breathing".to_string(),
                    sanskrit_name: "Dirgha Shvasa".to_string(),
                    duration_seconds: 60,
                    rounds: 5,
                    instructions: "Inhale deeply through nose, exhale slowly".to_string(),
                },
            ],
            meditation: Some(Meditation {
                name: "Mindfulness".to_string(),
                technique: "Breath awareness".to_string(),
                duration_seconds: 60,
                instructions: "Focus on breath, observe thoughts without judgment".to_string(),
            }),
            contraindications: vec![
                "Severe neck injury".to_string(),
            ],
            benefits: vec![
                "Reduces stress".to_string(),
                "Improves concentration".to_string(),
                "Relieves muscle tension".to_string(),
            ],
        })
    }

    /// Search practitioners by location
    pub fn search_practitioners(&self, state: &str, district: &str, system: AyushSystem) -> Result<Vec<AyushPractitioner>, String> {
        // In production: Make HTTP GET request to practitioner search API
        // For now: Return mock results
        Ok(vec![
            AyushPractitioner {
                practitioner_id: "PRA001".to_string(),
                name: "Dr. Ayurveda Sharma".to_string(),
                system: system.clone(),
                registration_number: "12345".to_string(),
                council: "CCIM".to_string(),
                qualification: "BAMS, MD (Ayurveda)".to_string(),
                college: "Government Ayurveda College".to_string(),
                year_of_registration: 2010,
                state: state.to_string(),
                district: district.to_string(),
                clinic_address: Address {
                    line1: "123, Ayurveda Lane".to_string(),
                    line2: "Andheri West".to_string(),
                    city: "Mumbai".to_string(),
                    district: district.to_string(),
                    state: state.to_string(),
                    pincode: "400058".to_string(),
                },
                specialization: "Panchakarma".to_string(),
                experience_years: 14,
                verified: true,
            },
        ])
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn ayush_client_create(base_url: *const u8, base_url_len: usize,
                                     api_key: *const u8, api_key_len: usize) -> *mut AyushClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(AyushClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn ayush_client_destroy(client: *mut AyushClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn ayush_verify_practitioner(client: *const AyushClient,
                                          reg_no: *const u8, reg_len: usize,
                                          council: *const u8, council_len: usize,
                                          out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || reg_no.is_null() || council.is_null() { return -1; }
        let reg_no = String::from_utf8_unchecked(
            std::slice::from_raw_parts(reg_no, reg_len));
        let council = String::from_utf8_unchecked(
            std::slice::from_raw_parts(council, council_len));
        match (*client).verify_practitioner(&reg_no, &council) {
            Ok(practitioner) => {
                let json = serde_json::to_string(&practitioner).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
