// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/irctc/sigma_irctc.rs — IRCTC Deep Integration
// Implements integration with Indian Railways Catering and Tourism Corporation
//
// Features:
//   - PNR status, seat map, running status (NTES real-time)
//   - Tatkal booking (automated queue at 10:00/11:00 AM)
//   - UTS (Unreserved Ticketing System) API for daily commuters
//   - Platform accessibility map (PWD facilities) with sigma-a11y
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── PNR Status ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnrStatus {
    pub pnr: String,
    pub train_number: String,
    pub train_name: String,
    pub doj: String,  // Date of Journey
    pub boarding_point: String,
    pub reservation_upto: String,
    pub class: String,
    pub passengers: Vec<Passenger>,
    pub chart_prepared: bool,
    pub charting_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passenger {
    pub serial_number: u32,
    pub booking_status: String,
    pub current_status: String,
    pub coach: String,
    pub seat_number: String,
}

// ── Seat Map ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatMap {
    pub train_number: String,
    pub coach_number: String,
    pub coach_type: String,
    pub layout: SeatLayout,
    pub available_seats: Vec<Seat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatLayout {
    pub rows: u32,
    pub columns: u32,
    pub aisle_positions: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seat {
    pub seat_number: String,
    pub seat_type: String,  // Window, Middle, Aisle
    pub status: String,  // Available, Booked, RAC, WL
    pub gender: Option<String>,
}

// ── Running Status (NTES) ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningStatus {
    pub train_number: String,
    pub train_name: String,
    pub doj: String,
    pub stations: Vec<StationStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationStatus {
    pub station_code: String,
    pub station_name: String,
    pub scheduled_arrival: String,
    pub scheduled_departure: String,
    pub actual_arrival: Option<String>,
    pub actual_departure: Option<String>,
    pub delay_minutes: i32,
    pub platform: Option<u32>,
    pub distance_km: f64,
}

// ── Tatkal Booking ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TatkalBooking {
    pub booking_id: String,
    pub train_number: String,
    pub class: String,
    pub quota: String,  // Tatkal, Premium Tatkal
    pub booking_date: String,
    pub journey_date: String,
    pub from_station: String,
    pub to_station: String,
    pub passengers: Vec<TatkalPassenger>,
    pub status: String,
    pub confirmation_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TatkalPassenger {
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub berth_preference: String,
    pub status: String,
}

// ── UTS Ticket (Unreserved Ticketing System) ───────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtsTicket {
    pub ticket_id: String,
    pub passenger_name: String,
    pub from_station: String,
    pub to_station: String,
    pub journey_date: String,
    pub class: String,
    pub fare: f64,
    pub distance_km: f64,
    pub valid_until: String,
    pub status: String,
}

// ── Platform Accessibility ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformAccessibility {
    pub station_code: String,
    pub station_name: String,
    pub platform_number: u32,
    pub facilities: Vec<AccessibilityFacility>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityFacility {
    pub facility_type: String,  // Ramp, Elevator, Wheelchair, Restroom, etc.
    pub location: String,
    pub status: String,
}

// ── IRCTC Client ─────────────────────────────────────────────────

pub struct IrctcClient {
    base_url: String,
    api_key: String,
    user_id: Option<String>,
}

impl IrctcClient {
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

    /// Get PNR status
    pub fn get_pnr_status(&self, pnr: &str) -> Result<PnrStatus, String> {
        // In production: Make HTTP GET request to IRCTC API
        // For now: Return mock PNR status
        Ok(PnrStatus {
            pnr: pnr.to_string(),
            train_number: "12301".to_string(),
            train_name: "Rajdhani Express".to_string(),
            doj: "2024-07-10".to_string(),
            boarding_point: "NDLS".to_string(),
            reservation_upto: "BCT".to_string(),
            class: "3A".to_string(),
            passengers: vec![
                Passenger {
                    serial_number: 1,
                    booking_status: "CNF".to_string(),
                    current_status: "CNF/B2/35".to_string(),
                    coach: "B2".to_string(),
                    seat_number: "35".to_string(),
                },
            ],
            chart_prepared: true,
            charting_status: "CHART PREPARED".to_string(),
        })
    }

    /// Get seat map
    pub fn get_seat_map(&self, train_number: &str, coach_number: &str) -> Result<SeatMap, String> {
        // In production: Make HTTP GET request to IRCTC API
        // For now: Return mock seat map
        Ok(SeatMap {
            train_number: train_number.to_string(),
            coach_number: coach_number.to_string(),
            coach_type: "3A".to_string(),
            layout: SeatLayout {
                rows: 8,
                columns: 3,
                aisle_positions: vec![2],
            },
            available_seats: vec![
                Seat {
                    seat_number: "1A".to_string(),
                    seat_type: "Window".to_string(),
                    status: "Booked".to_string(),
                    gender: Some("Male".to_string()),
                },
                Seat {
                    seat_number: "1B".to_string(),
                    seat_type: "Middle".to_string(),
                    status: "Available".to_string(),
                    gender: None,
                },
                Seat {
                    seat_number: "1C".to_string(),
                    seat_type: "Aisle".to_string(),
                    status: "Available".to_string(),
                    gender: None,
                },
            ],
        })
    }

    /// Get running status
    pub fn get_running_status(&self, train_number: &str, doj: &str) -> Result<RunningStatus, String> {
        // In production: Make HTTP GET request to NTES API
        // For now: Return mock running status
        Ok(RunningStatus {
            train_number: train_number.to_string(),
            train_name: "Rajdhani Express".to_string(),
            doj: doj.to_string(),
            stations: vec![
                StationStatus {
                    station_code: "NDLS".to_string(),
                    station_name: "New Delhi".to_string(),
                    scheduled_arrival: "16:55".to_string(),
                    scheduled_departure: "16:55".to_string(),
                    actual_arrival: Some("16:55".to_string()),
                    actual_departure: Some("17:00".to_string()),
                    delay_minutes: 5,
                    platform: Some(16),
                    distance_km: 0.0,
                },
                StationStatus {
                    station_code: "BCT".to_string(),
                    station_name: "Mumbai Central".to_string(),
                    scheduled_arrival: "08:35".to_string(),
                    scheduled_departure: "08:35".to_string(),
                    actual_arrival: None,
                    actual_departure: None,
                    delay_minutes: 0,
                    platform: None,
                    distance_km: 1386.0,
                },
            ],
        })
    }

    /// Book Tatkal ticket
    pub fn book_tatkal(&self, booking: &TatkalBooking) -> Result<String, String> {
        // In production: Make HTTP POST request to IRCTC API at 10:00/11:00 AM
        // For now: Return mock booking ID
        Ok(format!("TATKAL_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Book UTS ticket
    pub fn book_uts_ticket(&self, ticket: &UtsTicket) -> Result<String, String> {
        // In production: Make HTTP POST request to UTS API
        // For now: Return mock ticket ID
        Ok(format!("UTS_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    /// Get platform accessibility
    pub fn get_platform_accessibility(&self, station_code: &str, platform_number: u32) -> Result<PlatformAccessibility, String> {
        // In production: Make HTTP GET request to IRCTC API
        // For now: Return mock accessibility
        Ok(PlatformAccessibility {
            station_code: station_code.to_string(),
            station_name: "New Delhi".to_string(),
            platform_number,
            facilities: vec![
                AccessibilityFacility {
                    facility_type: "Ramp".to_string(),
                    location: "Platform entrance".to_string(),
                    status: "Available".to_string(),
                },
                AccessibilityFacility {
                    facility_type: "Elevator".to_string(),
                    location: "Near booking office".to_string(),
                    status: "Available".to_string(),
                },
                AccessibilityFacility {
                    facility_type: "Wheelchair".to_string(),
                    location: "Platform manager office".to_string(),
                    status: "On Request".to_string(),
                },
            ],
        })
    }

    /// Cancel ticket
    pub fn cancel_ticket(&self, pnr: &str) -> Result<bool, String> {
        // In production: Make HTTP POST request to IRCTC API
        // For now: Return success
        Ok(true)
    }

    /// Get train schedule
    pub fn get_train_schedule(&self, train_number: &str) -> Result<RunningStatus, String> {
        // In production: Make HTTP GET request to IRCTC API
        // For now: Return mock schedule
        Ok(RunningStatus {
            train_number: train_number.to_string(),
            train_name: "Rajdhani Express".to_string(),
            doj: "2024-07-10".to_string(),
            stations: vec![
                StationStatus {
                    station_code: "NDLS".to_string(),
                    station_name: "New Delhi".to_string(),
                    scheduled_arrival: "16:55".to_string(),
                    scheduled_departure: "16:55".to_string(),
                    actual_arrival: None,
                    actual_departure: None,
                    delay_minutes: 0,
                    platform: Some(16),
                    distance_km: 0.0,
                },
            ],
        })
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn irctc_client_create(base_url: *const u8, base_url_len: usize,
                                      api_key: *const u8, api_key_len: usize) -> *mut IrctcClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(IrctcClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn irctc_client_destroy(client: *mut IrctcClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn irctc_get_pnr_status(client: *const IrctcClient,
                                      pnr: *const u8, pnr_len: usize,
                                      out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || pnr.is_null() { return -1; }
        let pnr = String::from_utf8_unchecked(
            std::slice::from_raw_parts(pnr, pnr_len));
        match (*client).get_pnr_status(&pnr) {
            Ok(status) => {
                let json = serde_json::to_string(&status).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
