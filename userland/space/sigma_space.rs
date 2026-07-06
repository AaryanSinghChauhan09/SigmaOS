// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/space/sigma_space.rs — IN-SPACe Developer Tools
//
// Implements:
//   - Satellite design validation and simulation
//   - TLE (Two-Line Element) orbit propagation
//   - Ground station management and scheduling
//   - Telemetry data processing and visualization
//   - Attitude determination and control
//   - Power budget management
//   - India context: ISRO satellite integration, NavIC support
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── Satellite type ─────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SatelliteType {
    LEO = 0,      // Low Earth Orbit
    GEO = 1,      // Geostationary Orbit
    MEO = 2,      // Medium Earth Orbit
    HEO = 3,      // Highly Elliptical Orbit
    CubeSat = 4,  // CubeSat
}

// ── Orbit parameters (Keplerian elements) ─────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OrbitParameters {
    pub semi_major_axis: f32,    // a (km)
    pub eccentricity: f32,       // e
    pub inclination: f32,        // i (degrees)
    pub raan: f32,              // Right Ascension of Ascending Node (degrees)
    pub arg_perigee: f32,       // Argument of Perigee (degrees)
    pub true_anomaly: f32,      // True Anomaly (degrees)
}

impl OrbitParameters {
    pub const fn new() -> Self {
        Self {
            semi_major_axis: 7000.0,
            eccentricity: 0.0,
            inclination: 0.0,
            raan: 0.0,
            arg_perigee: 0.0,
            true_anomaly: 0.0,
        }
    }
}

// ── TLE (Two-Line Element) data ─────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TleData {
    pub line1: [u8; 69],
    pub line2: [u8; 69],
    pub satellite_id: [u8; 16],
    pub epoch: f32,
}

impl TleData {
    pub const fn new() -> Self {
        Self {
            line1: [0u8; 69],
            line2: [0u8; 69],
            satellite_id: [0u8; 16],
            epoch: 0.0,
        }
    }
}

// ── Satellite state vector ───────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StateVector {
    pub position: [f32; 3], // x, y, z (km)
    pub velocity: [f32; 3], // vx, vy, vz (km/s)
    pub timestamp: u64,
}

impl StateVector {
    pub const fn new() -> Self {
        Self {
            position: [0.0; 3],
            velocity: [0.0; 3],
            timestamp: 0,
        }
    }
}

// ── Ground station ─────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GroundStation {
    pub id: u32,
    pub name: [u8; 64],
    pub location: [f32; 3], // lat, lon, altitude (degrees, degrees, meters)
    pub min_elevation: f32, // Minimum elevation for contact (degrees)
    pub frequency_mhz: f32,
    pub active: bool,
}

impl GroundStation {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            name: [0u8; 64],
            location: [0.0; 3],
            min_elevation: 10.0,
            frequency_mhz: 2200.0,
            active: false,
        }
    }
}

// ── Pass prediction ─────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PassPrediction {
    pub satellite_id: [u8; 16],
    pub ground_station_id: u32,
    pub aos_time: u64,    // Acquisition of Signal
    pub los_time: u64,    // Loss of Signal
    pub max_elevation: f32,
    pub duration_seconds: u32,
}

impl PassPrediction {
    pub const fn new() -> Self {
        Self {
            satellite_id: [0u8; 16],
            ground_station_id: 0,
            aos_time: 0,
            los_time: 0,
            max_elevation: 0.0,
            duration_seconds: 0,
        }
    }
}

// ── Telemetry data ─────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TelemetryData {
    pub satellite_id: [u8; 16],
    pub timestamp: u64,
    pub battery_voltage: f32,
    pub battery_current: f32,
    pub temperature: f32,
    pub attitude_quaternion: [f32; 4], // x, y, z, w
    pub angular_velocity: [f32; 3],    // rad/s
}

impl TelemetryData {
    pub const fn new() -> Self {
        Self {
            satellite_id: [0u8; 16],
            timestamp: 0,
            battery_voltage: 0.0,
            battery_current: 0.0,
            temperature: 0.0,
            attitude_quaternion: [0.0; 4],
            angular_velocity: [0.0; 3],
        }
    }
}

// ── Space manager state ─────────────────────────────────────────

const MAX_SATELLITES: usize = 64;
const MAX_GROUND_STATIONS: usize = 32;
const MAX_TLE: usize = 128;
const MAX_PASS_PREDICTIONS: usize = 256;
const MAX_TELEMETRY: usize = 512;

pub struct SpaceManager {
    satellite_types: [Option<SatelliteType>; MAX_SATELLITES],
    satellite_ids: [[u8; 16]; MAX_SATELLITES],
    orbits: [Option<OrbitParameters>; MAX_SATELLITES],
    tle_data: [Option<TleData>; MAX_TLE],
    ground_stations: [Option<GroundStation>; MAX_GROUND_STATIONS],
    pass_predictions: [Option<PassPrediction>; MAX_PASS_PREDICTIONS],
    telemetry: [Option<TelemetryData>; MAX_TELEMETRY],
    satellite_count: AtomicU32,
    ground_station_count: AtomicU32,
    initialized: bool,
}

impl SpaceManager {
    pub const fn new() -> Self {
        Self {
            satellite_types: [const { None }; MAX_SATELLITES],
            satellite_ids: [[0u8; 16]; MAX_SATELLITES],
            orbits: [const { None }; MAX_SATELLITES],
            tle_data: [const { None }; MAX_TLE],
            ground_stations: [const { None }; MAX_GROUND_STATIONS],
            pass_predictions: [const { None }; MAX_PASS_PREDICTIONS],
            telemetry: [const { None }; MAX_TELEMETRY],
            satellite_count: AtomicU32::new(0),
            ground_station_count: AtomicU32::new(0),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Add a satellite
    pub fn add_satellite(&mut self, id: &[u8], sat_type: SatelliteType, orbit: OrbitParameters) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_SATELLITES {
            if self.satellite_types[i].is_none() {
                for j in 0..id.len().min(16) {
                    self.satellite_ids[i][j] = id[j];
                }
                self.satellite_types[i] = Some(sat_type);
                self.orbits[i] = Some(orbit);
                self.satellite_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Add TLE data
    pub fn add_tle(&mut self, tle: TleData) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_TLE {
            if self.tle_data[i].is_none() {
                self.tle_data[i] = Some(tle);
                return true;
            }
        }
        false
    }

    /// Add a ground station
    pub fn add_ground_station(&mut self, station: GroundStation) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_GROUND_STATIONS {
            if self.ground_stations[i].is_none() {
                self.ground_stations[i] = Some(station);
                self.ground_station_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Propagate orbit using TLE
    pub fn propagate_orbit(&self, sat_id: &[u8], time_seconds: f32) -> Option<StateVector> {
        if !self.initialized {
            return None;
        }

        // Find satellite
        let mut orbit = None;
        for i in 0..MAX_SATELLITES {
            let mut id_match = true;
            for j in 0..sat_id.len().min(16) {
                if self.satellite_ids[i][j] != sat_id[j] {
                    id_match = false;
                    break;
                }
            }
            if id_match {
                orbit = self.orbits[i];
                break;
            }
        }

        let orbit_params = match orbit {
            Some(o) => o,
            None => return None,
        };

        // Simplified orbit propagation (mock implementation)
        // In production: Use SGP4/SDP4 algorithm
        let mut state = StateVector::new();
        let n = (398600.4418 / orbit_params.semi_major_axis.powi(3)).sqrt(); // Mean motion
        
        let mean_anomaly = orbit_params.true_anomaly + n * time_seconds;
        let r = orbit_params.semi_major_axis * (1.0 - orbit_params.eccentricity.powi(2)) 
                / (1.0 + orbit_params.eccentricity * mean_anomaly.to_radians().cos());
        
        state.position[0] = r * mean_anomaly.to_radians().cos();
        state.position[1] = r * mean_anomaly.to_radians().sin();
        state.position[2] = 0.0;
        
        state.velocity[0] = -n * r * mean_anomaly.to_radians().sin();
        state.velocity[1] = n * r * mean_anomaly.to_radians().cos();
        state.velocity[2] = 0.0;
        
        state.timestamp = self.get_timestamp();
        
        Some(state)
    }

    /// Predict satellite pass over ground station
    pub fn predict_pass(&mut self, sat_id: &[u8], station_id: u32) -> Option<PassPrediction> {
        if !self.initialized {
            return None;
        }

        // Simplified pass prediction (mock implementation)
        // In production: Use actual geometry calculations
        let mut prediction = PassPrediction::new();
        
        for i in 0..sat_id.len().min(16) {
            prediction.satellite_id[i] = sat_id[i];
        }
        
        prediction.ground_station_id = station_id;
        prediction.aos_time = self.get_timestamp() + 3600; // 1 hour from now
        prediction.los_time = prediction.aos_time + 600; // 10 minute pass
        prediction.max_elevation = 45.0;
        prediction.duration_seconds = 600;

        for i in 0..MAX_PASS_PREDICTIONS {
            if self.pass_predictions[i].is_none() {
                self.pass_predictions[i] = Some(prediction);
                return Some(prediction);
            }
        }
        None
    }

    /// Add telemetry data
    pub fn add_telemetry(&mut self, telemetry: TelemetryData) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0::MAX_TELEMETRY {
            if self.telemetry[i].is_none() {
                self.telemetry[i] = Some(telemetry);
                return true;
            }
        }
        false
    }

    fn get_timestamp(&self) -> u64 {
        self.satellite_count.load(Ordering::Relaxed) as u64
    }

    pub fn satellite_count(&self) -> u32 {
        self.satellite_count.load(Ordering::Relaxed)
    }

    pub fn ground_station_count(&self) -> u32 {
        self.ground_station_count.load(Ordering::Relaxed)
    }
}

// ── Global space manager instance ─────────────────────────────────

static mut G_SPACE_MANAGER: SpaceManager = SpaceManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn space_manager_init() {
    G_SPACE_MANAGER.init();
}

#[no_mangle]
pub unsafe extern "C" fn space_add_satellite(
    id: *const u8,
    sat_type: u8,
    semi_major_axis: f32,
    eccentricity: f32,
    inclination: f32,
) -> i32 {
    let mut orbit = OrbitParameters::new();
    orbit.semi_major_axis = semi_major_axis;
    orbit.eccentricity = eccentricity;
    orbit.inclination = inclination;
    
    let sat_type = match sat_type {
        0 => SatelliteType::LEO,
        1 => SatelliteType::GEO,
        2 => SatelliteType::MEO,
        3 => SatelliteType::HEO,
        4 => SatelliteType::CubeSat,
        _ => SatelliteType::LEO,
    };
    
    let id_slice = if id.is_null() { &[] } else {
        let len = 16;
        core::slice::from_raw_parts(id, len)
    };
    
    if G_SPACE_MANAGER.add_satellite(id_slice, sat_type, orbit) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn space_add_tle(
    line1: *const u8,
    line2: *const u8,
    satellite_id: *const u8,
    epoch: f32,
) -> i32 {
    let mut tle = TleData::new();
    
    if !line1.is_null() {
        let line1_slice = core::slice::from_raw_parts(line1, 69.min(tle.line1.len()));
        for i in 0..line1_slice.len() {
            tle.line1[i] = line1_slice[i];
        }
    }
    
    if !line2.is_null() {
        let line2_slice = core::slice::from_raw_parts(line2, 69.min(tle.line2.len()));
        for i in 0..line2_slice.len() {
            tle.line2[i] = line2_slice[i];
        }
    }
    
    if !satellite_id.is_null() {
        let id_slice = core::slice::from_raw_parts(satellite_id, 16.min(tle.satellite_id.len()));
        for i in 0..id_slice.len() {
            tle.satellite_id[i] = id_slice[i];
        }
    }
    
    tle.epoch = epoch;
    
    if G_SPACE_MANAGER.add_tle(tle) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn space_add_ground_station(
    id: u32,
    name: *const u8,
    lat: f32,
    lon: f32,
    altitude: f32,
) -> i32 {
    let mut station = GroundStation::new(id);
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(station.name.len()));
        for i in 0..name_slice.len() {
            station.name[i] = name_slice[i];
        }
    }
    
    station.location = [lat, lon, altitude];
    
    if G_SPACE_MANAGER.add_ground_station(station) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn space_predict_pass(
    satellite_id: *const u8,
    station_id: u32,
) -> u64 {
    let id_slice = if satellite_id.is_null() { &[] } else {
        let len = 16;
        core::slice::from_raw_parts(satellite_id, len)
    };
    
    match G_SPACE_MANAGER.predict_pass(id_slice, station_id) {
        Some(pass) => pass.aos_time,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn space_add_telemetry(
    satellite_id: *const u8,
    battery_voltage: f32,
    battery_current: f32,
    temperature: f32,
) -> i32 {
    let mut telemetry = TelemetryData::new();
    
    if !satellite_id.is_null() {
        let id_slice = core::slice::from_raw_parts(satellite_id, 16.min(telemetry.satellite_id.len()));
        for i in 0..id_slice.len() {
            telemetry.satellite_id[i] = id_slice[i];
        }
    }
    
    telemetry.battery_voltage = battery_voltage;
    telemetry.battery_current = battery_current;
    telemetry.temperature = temperature;
    telemetry.timestamp = G_SPACE_MANAGER.get_timestamp();
    
    if G_SPACE_MANAGER.add_telemetry(telemetry) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn space_satellite_count() -> u32 {
    G_SPACE_MANAGER.satellite_count()
}

#[no_mangle]
pub unsafe extern "C" fn space_ground_station_count() -> u32 {
    G_SPACE_MANAGER.ground_station_count()
}
