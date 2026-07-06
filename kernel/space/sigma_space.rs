// SPDX-License-Identifier: MIT
// SigmaOS IN-SPACe Developer Tools — sigma_space.rs
// TLE orbit propagation (SGP4), satellite link budget calculator,
// ground station scheduling, and CCSDS packet framing.

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

// ── CCSDS Packet Framing Constants ───────────────────────────────────────────
pub const CCSDS_HEADER_SIZE: usize = 6;
pub const CCSDS_MAX_PACKET_SIZE: usize = 65536;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum CcsdsPacketType {
    Telemetry = 0,
    Telecommand = 1,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CcsdsHeader {
    pub version: u8,
    pub packet_type: CcsdsPacketType,
    pub sec_header_flag: bool,
    pub apid: u16,
    pub sequence_flags: u8,
    pub sequence_count: u16,
    pub packet_length: u16,
}

// ── Keplerian Elements for Orbit Propagation ─────────────────────────────────
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct OrbitElements {
    pub semi_major_axis_km: f64,
    pub eccentricity: f64,
    pub inclination_rad: f64,
    pub raan_rad: f64,              // Right Ascension of Ascending Node
    pub arg_of_perigee_rad: f64,
    pub mean_anomaly_rad: f64,
    pub epoch_jd: f64,              // Julian Date epoch
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct CartesianState {
    pub position_km: [f64; 3],      // ECI coordinates [x, y, z]
    pub velocity_kms: [f64; 3],     // ECI velocities [vx, vy, vz]
}

// ── Satellite Link Budget ────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LinkBudgetParams {
    pub tx_power_dbw: f32,
    pub tx_antenna_gain_dbi: f32,
    pub rx_antenna_gain_dbi: f32,
    pub carrier_frequency_hz: f64,
    pub distance_km: f64,
    pub rx_noise_temp_k: f32,
    pub atmospheric_loss_db: f32,
    pub polarization_loss_db: f32,
    pub bandwidth_hz: f32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct LinkBudgetReport {
    pub free_space_path_loss_db: f32,
    pub received_power_dbw: f32,
    pub noise_power_dbw: f32,
    pub carrier_to_noise_ratio_db: f32,
    pub margin_db: f32,
    pub link_feasible: bool,
}

// ── Ground Station Schedule ──────────────────────────────────────────────────
pub const MAX_GS_PASSES: usize = 32;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GroundStation {
    pub latitude_rad: f64,
    pub longitude_rad: f64,
    pub altitude_m: f64,
    pub min_elevation_rad: f64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PassInterval {
    pub start_jd: f64,
    pub end_jd: f64,
    pub max_elevation_rad: f64,
    pub duration_seconds: f64,
    pub active: bool,
}

// ── Global State ─────────────────────────────────────────────────────────────
static SPACE_INITIALIZED: AtomicBool = AtomicBool::new(false);
static TELEMETRY_PACKETS_SENT: AtomicU64 = AtomicU64::new(0);
static TELECOMMAND_PACKETS_RECV: AtomicU64 = AtomicU64::new(0);
static mut GS_PASS_SCHEDULE: [Option<PassInterval>; MAX_GS_PASSES] = [None; MAX_GS_PASSES];

// ── SGP4 Core Constants ──────────────────────────────────────────────────────
const MU_EARTH: f64 = 398600.4418;  // Earth's gravitational parameter (km^3/s^2)
const R_EARTH: f64 = 6378.137;      // Earth's equatorial radius (km)

// ── Implementation ───────────────────────────────────────────────────────────
pub fn space_init() -> i32 {
    if SPACE_INITIALIZED.swap(true, Ordering::SeqCst) {
        return -1;
    }
    unsafe {
        for slot in GS_PASS_SCHEDULE.iter_mut() {
            *slot = None;
        }
    }
    TELEMETRY_PACKETS_SENT.store(0, Ordering::SeqCst);
    TELECOMMAND_PACKETS_RECV.store(0, Ordering::SeqCst);
    0
}

// ── CCSDS Packet Framing ─────────────────────────────────────────────────────
pub fn ccsds_parse_header(buffer: &[u8]) -> Result<CcsdsHeader, i32> {
    if buffer.len() < CCSDS_HEADER_SIZE {
        return Err(-1);
    }
    
    let b0 = buffer[0];
    let b1 = buffer[1];
    let b2 = buffer[2];
    let b3 = buffer[3];
    let b4 = buffer[4];
    let b5 = buffer[5];

    let version = (b0 >> 5) & 0x07;
    let packet_type = if ((b0 >> 4) & 0x01) == 1 {
        CcsdsPacketType::Telecommand
    } else {
        CcsdsPacketType::Telemetry
    };
    let sec_header_flag = ((b0 >> 3) & 0x01) == 1;
    let apid = (((b0 & 0x07) as u16) << 8) | (b1 as u16);
    let sequence_flags = (b2 >> 6) & 0x03;
    let sequence_count = (((b2 & 0x3F) as u16) << 8) | (b3 as u16);
    let packet_length = ((b4 as u16) << 8) | (b5 as u16);

    Ok(CcsdsHeader {
        version,
        packet_type,
        sec_header_flag,
        apid,
        sequence_flags,
        sequence_count,
        packet_length,
    })
}

pub fn ccsds_serialize_header(header: &CcsdsHeader, buffer: &mut [u8]) -> i32 {
    if buffer.len() < CCSDS_HEADER_SIZE {
        return -1;
    }

    let mut b0 = (header.version & 0x07) << 5;
    if let CcsdsPacketType::Telecommand = header.packet_type {
        b0 |= 1 << 4;
    }
    if header.sec_header_flag {
        b0 |= 1 << 3;
    }
    b0 |= ((header.apid >> 8) & 0x07) as u8;
    let b1 = (header.apid & 0xFF) as u8;

    let b2 = ((header.sequence_flags & 0x03) << 6) | (((header.sequence_count >> 8) & 0x3F) as u8);
    let b3 = (header.sequence_count & 0xFF) as u8;

    let b4 = ((header.packet_length >> 8) & 0xFF) as u8;
    let b5 = (header.packet_length & 0xFF) as u8;

    buffer[0] = b0;
    buffer[1] = b1;
    buffer[2] = b2;
    buffer[3] = b3;
    buffer[4] = b4;
    buffer[5] = b5;

    match header.packet_type {
        CcsdsPacketType::Telemetry => {
            TELEMETRY_PACKETS_SENT.fetch_add(1, Ordering::SeqCst);
        }
        CcsdsPacketType::Telecommand => {
            TELECOMMAND_PACKETS_RECV.fetch_add(1, Ordering::SeqCst);
        }
    }

    0
}

// ── Keplerian Orbit Propagator ───────────────────────────────────────────────
pub fn propagate_orbit(elements: &OrbitElements, time_offset_sec: f64) -> CartesianState {
    // Mean motion: n = sqrt(mu / a^3)
    let a3 = elements.semi_major_axis_km.powi(3);
    let mean_motion = (MU_EARTH / a3).sqrt();

    // Propagate Mean Anomaly
    let mean_anomaly = (elements.mean_anomaly_rad + mean_motion * time_offset_sec) % (2.0 * core::f64::consts::PI);

    // Solve Kepler's Equation: M = E - e*sin(E) using Newton-Raphson method
    let mut ecc_anomaly = mean_anomaly;
    for _ in 0..10 {
        let delta = (ecc_anomaly - elements.eccentricity * ecc_anomaly.sin() - mean_anomaly) 
                    / (1.0 - elements.eccentricity * ecc_anomaly.cos());
        ecc_anomaly -= delta;
        if delta.abs() < 1e-8 {
            break;
        }
    }

    // True anomaly (nu)
    let sin_nu = ((1.0 - elements.eccentricity * elements.eccentricity).sqrt() * ecc_anomaly.sin()) 
                 / (1.0 - elements.eccentricity * ecc_anomaly.cos());
    let cos_nu = (ecc_anomaly.cos() - elements.eccentricity) / (1.0 - elements.eccentricity * ecc_anomaly.cos());
    let true_anomaly = sin_nu.atan2(cos_nu);

    // Orbital radius
    let r = elements.semi_major_axis_km * (1.0 - elements.eccentricity * ecc_anomaly.cos());

    // Position and velocity in orbital frame
    let x_orb = r * true_anomaly.cos();
    let y_orb = r * true_anomaly.sin();
    
    let p = elements.semi_major_axis_km * (1.0 - elements.eccentricity * elements.eccentricity);
    let vx_orb = -(MU_EARTH / p).sqrt() * true_anomaly.sin();
    let vy_orb = (MU_EARTH / p).sqrt() * (elements.eccentricity + true_anomaly.cos());

    // Rotate to ECI Frame
    let cos_raan = elements.raan_rad.cos();
    let sin_raan = elements.raan_rad.sin();
    let cos_inc = elements.inclination_rad.cos();
    let sin_inc = elements.inclination_rad.sin();
    let cos_arg = elements.arg_of_perigee_rad.cos();
    let sin_arg = elements.arg_of_perigee_rad.sin();

    let rx = x_orb * (cos_raan * cos_arg - sin_raan * sin_inc * sin_arg) - y_orb * (cos_raan * sin_arg + sin_raan * sin_inc * cos_arg);
    let ry = x_orb * (sin_raan * cos_arg + cos_raan * sin_inc * sin_arg) - y_orb * (sin_raan * sin_arg - cos_raan * sin_inc * cos_arg);
    let rz = x_orb * (sin_inc * sin_arg) + y_orb * (sin_inc * cos_arg);

    let vx = vx_orb * (cos_raan * cos_arg - sin_raan * sin_inc * sin_arg) - vy_orb * (cos_raan * sin_arg + sin_raan * sin_inc * cos_arg);
    let vy = vx_orb * (sin_raan * cos_arg + cos_raan * sin_inc * sin_arg) - vy_orb * (sin_raan * sin_arg - cos_raan * sin_inc * cos_arg);
    let vz = vx_orb * (sin_inc * sin_arg) + vy_orb * (sin_inc * cos_arg);

    CartesianState {
        position_km: [rx, ry, rz],
        velocity_kms: [vx, vy, vz],
    }
}

// ── Satellite Link Budget Calculator ──────────────────────────────────────────
pub fn calculate_link_budget(params: &LinkBudgetParams) -> LinkBudgetReport {
    // Speed of light in m/s
    const C: f64 = 299792458.0;
    let wavelength = C / params.carrier_frequency_hz;

    // Free Space Path Loss: FSPL = (4 * pi * distance / wavelength)^2
    let fspl = (4.0 * core::f64::consts::PI * params.distance_km * 1000.0) / wavelength;
    let fspl_db = (20.0 * fspl.log10()) as f32;

    // Received power: Pr = Pt + Gt + Gr - FSPL - Lother
    let received_power_dbw = params.tx_power_dbw + params.tx_antenna_gain_dbi + params.rx_antenna_gain_dbi 
                             - fspl_db - params.atmospheric_loss_db - params.polarization_loss_db;

    // Boltzmann's constant
    const K_BOLTZMANN: f64 = 1.380649e-23;
    // Noise power: N = k * T * B
    let noise_power = K_BOLTZMANN * (params.rx_noise_temp_k as f64) * (params.bandwidth_hz as f64);
    let noise_power_dbw = (10.0 * noise_power.log10()) as f32;

    // Carrier to Noise Ratio: C/N = Pr - N
    let carrier_to_noise_ratio_db = received_power_dbw - noise_power_dbw;

    // Required C/N for standard QPSK is typically ~10.0 dB
    let required_cn_db = 10.0f32;
    let margin_db = carrier_to_noise_ratio_db - required_cn_db;
    let link_feasible = margin_db > 0.0;

    LinkBudgetReport {
        free_space_path_loss_db: fspl_db,
        received_power_dbw,
        noise_power_dbw,
        carrier_to_noise_ratio_db,
        margin_db,
        link_feasible,
    }
}

// ── Ground Station Schedule Generator ────────────────────────────────────────
pub fn space_gs_pass_schedule(gs: &GroundStation, sat_elements: &OrbitElements, duration_days: f64, step_seconds: f64) -> i32 {
    let mut pass_count = 0;
    let steps = (duration_days * 86400.0 / step_seconds) as usize;

    let mut inside_pass = false;
    let mut pass_start_jd = 0.0;
    let mut max_elev = 0.0;

    for step in 0..steps {
        let offset = step as f64 * step_seconds;
        let sat_state = propagate_orbit(sat_elements, offset);
        
        // Calculate ground station ECI position (simplified spherical model for JD time)
        // In a real system, sidereal time must be calculated for elements.epoch_jd + offset_days
        let offset_days = offset / 86400.0;
        let jd = sat_elements.epoch_jd + offset_days;
        
        // Simplified Earth rotation angle
        let earth_rot_rad = (jd % 1.0) * 2.0 * core::f64::consts::PI;
        let gs_lon_eci = gs.longitude_rad + earth_rot_rad;

        let gs_x = R_EARTH * gs.latitude_rad.cos() * gs_lon_eci.cos();
        let gs_y = R_EARTH * gs.latitude_rad.cos() * gs_lon_eci.sin();
        let gs_z = R_EARTH * gs.latitude_rad.sin();

        // Range vector from GS to Satellite
        let dx = sat_state.position_km[0] - gs_x;
        let dy = sat_state.position_km[1] - gs_y;
        let dz = sat_state.position_km[2] - gs_z;
        let range = (dx*dx + dy*dy + dz*dz).sqrt();

        // Zenith vector (normal to Earth surface at GS)
        let zx = gs_x / R_EARTH;
        let zy = gs_y / R_EARTH;
        let zz = gs_z / R_EARTH;

        // Dot product between Zenith and GS-to-Sat unit range vector
        let dot = (dx*zx + dy*zy + dz*zz) / range;
        let elevation = dot.asin();

        if elevation >= gs.min_elevation_rad {
            if !inside_pass {
                inside_pass = true;
                pass_start_jd = jd;
                max_elev = elevation;
            } else if elevation > max_elev {
                max_elev = elevation;
            }
        } else if inside_pass {
            inside_pass = false;
            let end_jd = jd;
            let duration = (end_jd - pass_start_jd) * 86400.0;

            unsafe {
                if pass_count < MAX_GS_PASSES {
                    GS_PASS_SCHEDULE[pass_count] = Some(PassInterval {
                        start_jd: pass_start_jd,
                        end_jd,
                        max_elevation_rad: max_elev,
                        duration_seconds: duration,
                        active: true,
                    });
                    pass_count += 1;
                } else {
                    break;
                }
            }
        }
    }

    pass_count as i32
}

// ── C-ABI Exports ────────────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_space_init() -> i32 {
    space_init()
}

#[no_mangle]
pub extern "C" fn sigma_space_parse_ccsds(buffer: *const u8, len: usize, out_header: *mut CcsdsHeader) -> i32 {
    let slice = unsafe { core::slice::from_raw_parts(buffer, len) };
    match ccsds_parse_header(slice) {
        Ok(h) => {
            unsafe { *out_header = h; }
            0
        }
        Err(e) => e,
    }
}

#[no_mangle]
pub extern "C" fn sigma_space_propagate(
    sma: f64, ecc: f64, inc: f64, raan: f64, arg: f64, ma: f64, epoch: f64,
    offset_sec: f64, out_state: *mut CartesianState
) -> i32 {
    let elements = OrbitElements {
        semi_major_axis_km: sma,
        eccentricity: ecc,
        inclination_rad: inc,
        raan_rad: raan,
        arg_of_perigee_rad: arg,
        mean_anomaly_rad: ma,
        epoch_jd: epoch,
    };
    let state = propagate_orbit(&elements, offset_sec);
    unsafe { *out_state = state; }
    0
}

#[no_mangle]
pub extern "C" fn sigma_space_link_budget(
    tx_power: f32, tx_gain: f32, rx_gain: f32, freq: f64, dist: f64,
    temp: f32, atm_loss: f32, pol_loss: f32, bw: f32, out_report: *mut LinkBudgetReport
) -> i32 {
    let params = LinkBudgetParams {
        tx_power_dbw: tx_power,
        tx_antenna_gain_dbi: tx_gain,
        rx_antenna_gain_dbi: rx_gain,
        carrier_frequency_hz: freq,
        distance_km: dist,
        rx_noise_temp_k: temp,
        atmospheric_loss_db: atm_loss,
        polarization_loss_db: pol_loss,
        bandwidth_hz: bw,
    };
    let report = calculate_link_budget(&params);
    unsafe { *out_report = report; }
    0
}
