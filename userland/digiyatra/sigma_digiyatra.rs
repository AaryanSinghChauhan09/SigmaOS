// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/digiyatra/sigma_digiyatra.rs — DigiYatra (Biometric Air/Rail Travel)
// Implements integration with DigiYatra biometric travel system
//
// Features:
//   - Face-based boarding at airports (BCAS system)
//   - sigma-auth face enrollment → DigiYatra token (local processing, only token sent)
//   - Rail: IRCTC biometric boarding extension
//   - Fully voluntary — can link/unlink from sigma-datasov vault
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── DigiYatra Profile ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigiYatraProfile {
    pub profile_id: String,
    pub user_id: String,
    pub name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub aadhaar: String,
    pub phone: String,
    pub email: String,
    pub face_id: String,
    pub digiyatra_token: String,
    pub enrollment_date: String,
    pub status: String,
}

// ── Travel Document ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelDocument {
    pub document_id: String,
    pub profile_id: String,
    pub document_type: String,  // Passport, Aadhaar, Driving License
    pub document_number: String,
    pub expiry_date: String,
    pub issuer: String,
    pub is_primary: bool,
}

// ── Booking ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub booking_id: String,
    pub profile_id: String,
    pub travel_type: TravelType,
    pub pnr: String,
    pub flight_number: Option<String>,
    pub train_number: Option<String>,
    pub origin: String,
    pub destination: String,
    pub departure_date: String,
    pub departure_time: String,
    pub seat_number: String,
    pub gate: Option<String>,
    pub terminal: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TravelType {
    Flight,
    Train,
}

// ── Biometric Verification ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricVerification {
    pub verification_id: String,
    pub booking_id: String,
    pub profile_id: String,
    pub verification_type: String,  // Face, Fingerprint, Iris
    pub verification_point: String,  // Check-in, Security, Boarding
    pub timestamp: String,
    pub success: bool,
    pub confidence_score: f32,
    pub location: String,
}

// ── DigiYatra Client ─────────────────────────────────────────────────

pub struct DigiYatraClient {
    base_url: String,
    api_key: String,
    user_id: Option<String>,
}

impl DigiYatraClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            user_id: None,
        }
    }

    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = Some(user_id);
    }

    /// Enroll face for DigiYatra
    pub fn enroll_face(&self, user_id: &str, face_image_data: &[u8]) -> Result<DigiYatraProfile, String> {
        // In production: Process face image locally, generate face template, create DigiYatra token
        // For now: Return mock profile
        Ok(DigiYatraProfile {
            profile_id: format!("DYP_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            user_id: user_id.to_string(),
            name: "Traveler Name".to_string(),
            date_of_birth: "1990-01-01".to_string(),
            gender: "Male".to_string(),
            aadhaar: "1234-5678-9012".to_string(),
            phone: "+919876543210".to_string(),
            email: "traveler@example.com".to_string(),
            face_id: format!("FACE_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            digiyatra_token: format!("TOKEN_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            enrollment_date: chrono::Utc::now().to_rfc3339(),
            status: "Active".to_string(),
        })
    }

    /// Add travel document
    pub fn add_travel_document(&self, profile_id: &str, document: &TravelDocument) -> Result<String, String> {
        // In production: Make HTTP POST request to DigiYatra API
        // For now: Return mock document ID
        Ok(format!("DOC_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Link booking to DigiYatra profile
    pub fn link_booking(&self, profile_id: &str, pnr: &str, travel_type: TravelType) -> Result<Booking, String> {
        // In production: Make HTTP POST request to DigiYatra API
        // For now: Return mock booking
        Ok(Booking {
            booking_id: format!("BK_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            profile_id: profile_id.to_string(),
            travel_type,
            pnr: pnr.to_string(),
            flight_number: Some("SG123".to_string()),
            train_number: None,
            origin: "DEL".to_string(),
            destination: "BOM".to_string(),
            departure_date: "2024-07-10".to_string(),
            departure_time: "10:00".to_string(),
            seat_number: "12A".to_string(),
            gate: Some("A12".to_string()),
            terminal: Some("T3".to_string()),
            status: "Confirmed".to_string(),
        })
    }

    /// Get profile details
    pub fn get_profile(&self, profile_id: &str) -> Result<DigiYatraProfile, String> {
        // In production: Make HTTP GET request to DigiYatra API
        // For now: Return mock profile
        Ok(DigiYatraProfile {
            profile_id: profile_id.to_string(),
            user_id: "USR001".to_string(),
            name: "Traveler Name".to_string(),
            date_of_birth: "1990-01-01".to_string(),
            gender: "Male".to_string(),
            aadhaar: "1234-5678-9012".to_string(),
            phone: "+919876543210".to_string(),
            email: "traveler@example.com".to_string(),
            face_id: "FACE_12345".to_string(),
            digiyatra_token: "TOKEN_67890".to_string(),
            enrollment_date: "2024-01-15".to_string(),
            status: "Active".to_string(),
        })
    }

    /// Verify biometric at checkpoint
    pub fn verify_biometric(&self, profile_id: &str, booking_id: &str, verification_point: &str) -> Result<BiometricVerification, String> {
        // In production: Process biometric verification locally, send only token to server
        // For now: Return mock verification
        Ok(BiometricVerification {
            verification_id: format!("BV_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            booking_id: booking_id.to_string(),
            profile_id: profile_id.to_string(),
            verification_type: "Face".to_string(),
            verification_point: verification_point.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            success: true,
            confidence_score: 0.98,
            location: "Delhi Airport T3".to_string(),
        })
    }

    /// Get booking details
    pub fn get_booking(&self, booking_id: &str) -> Result<Booking, String> {
        // In production: Make HTTP GET request to DigiYatra API
        // For now: Return mock booking
        Ok(Booking {
            booking_id: booking_id.to_string(),
            profile_id: "DYP001".to_string(),
            travel_type: TravelType::Flight,
            pnr: "ABC123".to_string(),
            flight_number: Some("SG123".to_string()),
            train_number: None,
            origin: "DEL".to_string(),
            destination: "BOM".to_string(),
            departure_date: "2024-07-10".to_string(),
            departure_time: "10:00".to_string(),
            seat_number: "12A".to_string(),
            gate: Some("A12".to_string()),
            terminal: Some("T3".to_string()),
            status: "Confirmed".to_string(),
        })
    }

    /// Unlink DigiYatra profile (voluntary unlink)
    pub fn unlink_profile(&self, profile_id: &str) -> Result<bool, String> {
        // In production: Make HTTP POST request to DigiYatra API
        // For now: Return success
        Ok(true)
    }

    /// Get travel history
    pub fn get_travel_history(&self, profile_id: &str, start_date: &str, end_date: &str) -> Result<Vec<Booking>, String> {
        // In production: Make HTTP GET request to DigiYatra API
        // For now: Return mock history
        Ok(vec![
            Booking {
                booking_id: "BK001".to_string(),
                profile_id: profile_id.to_string(),
                travel_type: TravelType::Flight,
                pnr: "ABC123".to_string(),
                flight_number: Some("SG123".to_string()),
                train_number: None,
                origin: "DEL".to_string(),
                destination: "BOM".to_string(),
                departure_date: "2024-07-01".to_string(),
                departure_time: "10:00".to_string(),
                seat_number: "12A".to_string(),
                gate: Some("A12".to_string()),
                terminal: Some("T3".to_string()),
                status: "Completed".to_string(),
            },
        ])
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn digiyatra_client_create(base_url: *const u8, base_url_len: usize,
                                         api_key: *const u8, api_key_len: usize) -> *mut DigiYatraClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(DigiYatraClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn digiyatra_client_destroy(client: *mut DigiYatraClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn digiyatra_enroll_face(client: *const DigiYatraClient,
                                      user_id: *const u8, user_len: usize,
                                      face_data: *const u8, face_len: usize,
                                      out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || user_id.is_null() || face_data.is_null() { return -1; }
        let user_id = String::from_utf8_unchecked(
            std::slice::from_raw_parts(user_id, user_len));
        let face_data = std::slice::from_raw_parts(face_data, face_len);
        match (*client).enroll_face(&user_id, face_data) {
            Ok(profile) => {
                let json = serde_json::to_string(&profile).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
