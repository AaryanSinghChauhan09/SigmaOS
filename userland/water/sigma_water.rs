// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/water/sigma_water.rs — Water Resource Management
// Implements integration with Indian water resource management systems
//
// Features:
//   - CWC (Central Water Commission) data integration
//   - Jal Jeevan Mission sensor data (water quality + flow per village)
//   - WRIS (Water Resources Information System) API
//   - Irrigation scheduling: weather + soil moisture + ET0 crop coefficient
//   - CGWB groundwater level monitoring
//   - Flood early warning system
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── CWC (Central Water Commission) Data ─────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CwcData {
    pub station_id: String,
    pub station_name: String,
    pub river: String,
    pub state: String,
    pub basin: String,
    pub location: Location,
    pub water_level: WaterLevel,
    pub flow_data: FlowData,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub district: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterLevel {
    pub current_level_m: f64,
    pub warning_level_m: f64,
    pub danger_level_m: f64,
    pub highest_flood_level_m: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowData {
    pub discharge_cusecs: f64,
    pub velocity_mps: f64,
}

// ── Jal Jeevan Mission Sensor Data ─────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JjmSensorData {
    pub village_id: String,
    pub village_name: String,
    pub district: String,
    pub state: String,
    pub scheme_id: String,
    pub sensor_type: SensorType,
    pub water_quality: WaterQuality,
    pub flow_data: FlowMeasurement,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensorType {
    WaterQuality,
    FlowMeter,
    LevelSensor,
    PressureSensor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterQuality {
    pub ph: f64,
    pub turbidity_ntu: f64,
    pub tds_mg_l: f64,
    pub chlorine_mg_l: f64,
    pub fluoride_mg_l: f64,
    pub nitrate_mg_l: f64,
    pub arsenic_mg_l: f64,
    pub iron_mg_l: f64,
    pub compliance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowMeasurement {
    pub flow_rate_lpm: f64,
    pub total_volume_liters: f64,
    pub pressure_bar: f64,
}

// ── WRIS (Water Resources Information System) ─────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrisProject {
    pub project_id: String,
    pub project_name: String,
    pub project_type: String,  // Dam, Barrage, Canal, Reservoir, etc.
    pub river: String,
    pub basin: String,
    pub state: String,
    pub district: String,
    pub capacity_mcm: f64,
    pub live_storage_mcm: f64,
    pub inflow_cusecs: f64,
    pub outflow_cusecs: f64,
    pub purpose: Vec<String>,
    pub status: String,
}

// ── Irrigation Scheduling ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrrigationSchedule {
    pub schedule_id: String,
    pub field_id: String,
    pub crop: String,
    pub area_hectares: f64,
    pub growth_stage: String,
    pub et0_mm: f64,
    pub kc: f64,  // Crop coefficient
    pub et_crop_mm: f64,
    pub soil_moisture_percent: f64,
    pub recommended_irrigation_mm: f64,
    pub irrigation_date: String,
    pub duration_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature_c: f64,
    pub humidity_percent: f64,
    pub wind_speed_kmph: f64,
    pub solar_radiation_mj_m2: f64,
    pub rainfall_mm: f64,
}

// ── CGWB Groundwater Level ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundwaterLevel {
    pub well_id: String,
    pub location: Location,
    pub pre_monsoon_level_m: f64,
    pub post_monsoon_level_m: f64,
    pub water_table_depth_m: f64,
    pub trend: String,  // Rising, Falling, Stable
    pub quality: GroundwaterQuality,
    pub measurement_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundwaterQuality {
    pub ec_ds_m: f64,
    pub ph: f64,
    pub fluoride_mg_l: f64,
    pub nitrate_mg_l: f64,
    pub arsenic_mg_l: f64,
}

// ── Flood Early Warning ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloodWarning {
    pub warning_id: String,
    pub river: String,
    pub location: String,
    pub warning_level: WarningLevel,
    pub current_level_m: f64,
    pub warning_level_m: f64,
    pub danger_level_m: f64,
    pub forecast_peak_m: f64,
    pub expected_peak_time: String,
    pub affected_areas: Vec<String>,
    pub evacuation_required: bool,
    pub issued_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarningLevel {
    Normal,
    Alert,
    Warning,
    Danger,
}

// ── Water Client ─────────────────────────────────────────────────────

pub struct WaterClient {
    base_url: String,
    api_key: String,
}

impl WaterClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
        }
    }

    /// Get CWC water level data
    pub fn get_cwc_data(&self, station_id: &str) -> Result<CwcData, String> {
        // In production: Make HTTP GET request to CWC API
        // For now: Return mock data
        Ok(CwcData {
            station_id: station_id.to_string(),
            station_name: "Yamuna at Delhi".to_string(),
            river: "Yamuna".to_string(),
            state: "Delhi".to_string(),
            basin: "Ganga Basin".to_string(),
            location: Location {
                latitude: 28.6139,
                longitude: 77.2090,
                district: "Delhi".to_string(),
            },
            water_level: WaterLevel {
                current_level_m: 205.5,
                warning_level_m: 206.0,
                danger_level_m: 207.0,
                highest_flood_level_m: 207.5,
            },
            flow_data: FlowData {
                discharge_cusecs: 15000.0,
                velocity_mps: 2.5,
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Get Jal Jeevan Mission sensor data
    pub fn get_jjm_sensor_data(&self, village_id: &str) -> Result<JjmSensorData, String> {
        // In production: Make HTTP GET request to JJM API
        // For now: Return mock data
        Ok(JjmSensorData {
            village_id: village_id.to_string(),
            village_name: "Sample Village".to_string(),
            district: "District".to_string(),
            state: "State".to_string(),
            scheme_id: "JJM001".to_string(),
            sensor_type: SensorType::WaterQuality,
            water_quality: WaterQuality {
                ph: 7.2,
                turbidity_ntu: 2.5,
                tds_mg_l: 350.0,
                chlorine_mg_l: 0.5,
                fluoride_mg_l: 0.8,
                nitrate_mg_l: 25.0,
                arsenic_mg_l: 0.01,
                iron_mg_l: 0.3,
                compliance: true,
            },
            flow_data: FlowMeasurement {
                flow_rate_lpm: 50.0,
                total_volume_liters: 100000.0,
                pressure_bar: 2.5,
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Get WRIS project information
    pub fn get_wris_project(&self, project_id: &str) -> Result<WrisProject, String> {
        // In production: Make HTTP GET request to WRIS API
        // For now: Return mock data
        Ok(WrisProject {
            project_id: project_id.to_string(),
            project_name: "Sample Dam".to_string(),
            project_type: "Dam".to_string(),
            river: "River".to_string(),
            basin: "Basin".to_string(),
            state: "State".to_string(),
            district: "District".to_string(),
            capacity_mcm: 1000.0,
            live_storage_mcm: 750.0,
            inflow_cusecs: 5000.0,
            outflow_cusecs: 3000.0,
            purpose: vec![
                "Irrigation".to_string(),
                "Hydropower".to_string(),
                "Flood Control".to_string(),
            ],
            status: "Active".to_string(),
        })
    }

    /// Calculate irrigation schedule
    pub fn calculate_irrigation(&self, field_id: &str, crop: &str, weather: &WeatherData, soil_moisture: f64) -> Result<IrrigationSchedule, String> {
        // In production: Calculate based on crop coefficient, ET0, soil moisture
        // For now: Return mock schedule
        let kc = match crop {
            "Wheat" => 1.15,
            "Rice" => 1.20,
            "Maize" => 1.10,
            "Cotton" => 1.05,
            _ => 1.0,
        };
        
        let et0 = (weather.temperature_c * 0.1) + (weather.solar_radiation_mj_m2 * 0.05);
        let et_crop = et0 * kc;
        let recommended_irrigation = if soil_moisture < 30.0 { et_crop * 1.5 } else { 0.0 };
        
        Ok(IrrigationSchedule {
            schedule_id: format!("IRR_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            field_id: field_id.to_string(),
            crop: crop.to_string(),
            area_hectares: 5.0,
            growth_stage: "Mid-season".to_string(),
            et0_mm: et0,
            kc,
            et_crop_mm: et_crop,
            soil_moisture_percent: soil_moisture,
            recommended_irrigation_mm: recommended_irrigation,
            irrigation_date: chrono::Utc::now().to_rfc3339(),
            duration_hours: if recommended_irrigation > 0.0 { 4.0 } else { 0.0 },
        })
    }

    /// Get groundwater level data
    pub fn get_groundwater_level(&self, well_id: &str) -> Result<GroundwaterLevel, String> {
        // In production: Make HTTP GET request to CGWB API
        // For now: Return mock data
        Ok(GroundwaterLevel {
            well_id: well_id.to_string(),
            location: Location {
                latitude: 28.6139,
                longitude: 77.2090,
                district: "District".to_string(),
            },
            pre_monsoon_level_m: 15.0,
            post_monsoon_level_m: 12.0,
            water_table_depth_m: 12.0,
            trend: "Rising".to_string(),
            quality: GroundwaterQuality {
                ec_ds_m: 500.0,
                ph: 7.5,
                fluoride_mg_l: 0.9,
                nitrate_mg_l: 30.0,
                arsenic_mg_l: 0.01,
            },
            measurement_date: "2024-01-15".to_string(),
        })
    }

    /// Get flood warning
    pub fn get_flood_warning(&self, river: &str, location: &str) -> Result<FloodWarning, String> {
        // In production: Make HTTP GET request to flood warning API
        // For now: Return mock warning
        Ok(FloodWarning {
            warning_id: format!("FW_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()),
            river: river.to_string(),
            location: location.to_string(),
            warning_level: WarningLevel::Alert,
            current_level_m: 205.5,
            warning_level_m: 206.0,
            danger_level_m: 207.0,
            forecast_peak_m: 206.5,
            expected_peak_time: "2024-07-20T18:00:00Z".to_string(),
            affected_areas: vec![
                "Area 1".to_string(),
                "Area 2".to_string(),
            ],
            evacuation_required: false,
            issued_at: chrono::Utc::now().to_rfc3339(),
        })
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn water_client_create(base_url: *const u8, base_url_len: usize,
                                    api_key: *const u8, api_key_len: usize) -> *mut WaterClient {
    unsafe {
        let base_url = String::from_utf8_unchecked(
            std::slice::from_raw_parts(base_url, base_url_len));
        let api_key = String::from_utf8_unchecked(
            std::slice::from_raw_parts(api_key, api_key_len));
        Box::into_raw(Box::new(WaterClient::new(base_url, api_key)))
    }
}

#[no_mangle]
pub extern "C" fn water_client_destroy(client: *mut WaterClient) {
    unsafe {
        if !client.is_null() {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
pub extern "C" fn water_get_cwc_data(client: *const WaterClient,
                                    station_id: *const u8, station_len: usize,
                                    out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if client.is_null() || station_id.is_null() { return -1; }
        let station_id = String::from_utf8_unchecked(
            std::slice::from_raw_parts(station_id, station_len));
        match (*client).get_cwc_data(&station_id) {
            Ok(data) => {
                let json = serde_json::to_string(&data).unwrap_or_default();
                let bytes = json.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), out_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
