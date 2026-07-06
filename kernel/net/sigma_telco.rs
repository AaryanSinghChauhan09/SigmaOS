// SPDX-License-Identifier: MIT
// SigmaOS 5G/6G Network OS — sigma_telco.rs
// O-RAN Alliance xApp integration, TRAI QoS monitoring,
// network slicing controller, and CU/DU split management.
//
// Implements the sigma-telco subsystem for sovereign telecom infrastructure.

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

// ── O-RAN Interface IDs ──────────────────────────────────────────────────────
pub const ORAN_E2_INTERFACE:  u8 = 0x01; // E2 interface (near-RT RIC ↔ E2 Node)
pub const ORAN_A1_INTERFACE:  u8 = 0x02; // A1 interface (non-RT RIC → near-RT RIC)
pub const ORAN_O1_INTERFACE:  u8 = 0x03; // O1 interface (SMO ↔ O-RAN NFs)
pub const ORAN_O2_INTERFACE:  u8 = 0x04; // O2 interface (SMO → O-Cloud)
pub const ORAN_FH_INTERFACE:  u8 = 0x05; // Fronthaul (O-DU ↔ O-RU)

// ── Network Slice Types (3GPP TS 23.501) ─────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SliceType {
    EMBB,       // Enhanced Mobile Broadband
    URLLC,      // Ultra-Reliable Low-Latency Communications
    MMTC,       // Massive Machine-Type Communications
    V2X,        // Vehicle-to-Everything
    Custom(u8), // Operator-defined
}

// ── QoS Flow Identifiers (3GPP QFI) ─────────────────────────────────────────
pub const QFI_CONVERSATIONAL_VOICE: u8 = 1;
pub const QFI_CONVERSATIONAL_VIDEO: u8 = 2;
pub const QFI_REAL_TIME_GAMING:     u8 = 3;
pub const QFI_NON_CONV_VIDEO:       u8 = 4;
pub const QFI_MISSION_CRITICAL:     u8 = 65;
pub const QFI_V2X:                  u8 = 79;
pub const QFI_BEST_EFFORT:          u8 = 9;

// ── TRAI QoS Parameters ─────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TraiQosMetrics {
    pub download_speed_kbps: u32,
    pub upload_speed_kbps:   u32,
    pub latency_ms:          u16,
    pub jitter_ms:           u16,
    pub packet_loss_ppm:     u16, // parts per million
    pub call_drop_rate_ppm:  u16,
    pub signal_strength_dbm: i16,
    pub operator_id:         u16,
    pub circle_id:           u8,  // TRAI telecom circle (1–22)
    pub technology:          RadioTech,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RadioTech {
    LTE4G,
    NR5GSA,      // 5G Standalone
    NR5GNSA,     // 5G Non-Standalone
    NR6G,        // 6G (future)
    WiFi6E,
    Satellite,   // LEO satellite backhaul
}

// ── CU/DU Split Architecture ─────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CuDuSplit {
    Split2,  // PDCP ↔ RLC (most common for 5G)
    Split6,  // MAC ↔ PHY-high
    Split7,  // PHY-high ↔ PHY-low (eCPRI fronthaul)
    Split8,  // PHY ↔ RF (full centralization)
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CentralUnit {
    pub cu_id:        u32,
    pub split:        CuDuSplit,
    pub max_ues:      u32,
    pub active_ues:   AtomicU32,
    pub amf_ip:       [u8; 4],   // AMF (Access & Mobility Mgmt) endpoint
    pub upf_ip:       [u8; 4],   // UPF (User Plane Function) endpoint
    pub plmn:         [u8; 6],   // PLMN identity (MCC+MNC)
    pub enabled:      AtomicBool,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DistributedUnit {
    pub du_id:        u32,
    pub cu_id:        u32,      // parent CU
    pub cell_id:      u32,      // NR Cell Identity
    pub pci:          u16,      // Physical Cell ID
    pub arfcn:        u32,      // Absolute Radio Frequency Channel Number
    pub bandwidth_mhz: u8,
    pub mimo_layers:  u8,       // 2/4/8/16 MIMO
    pub subcarrier_spacing_khz: u16, // 15/30/60/120/240
    pub tx_power_dbm: i16,
    pub enabled:      AtomicBool,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct RadioUnit {
    pub ru_id:        u32,
    pub du_id:        u32,
    pub antenna_ports: u8,
    pub beamforming:  bool,
    pub ecpri_rate_gbps: u8,    // eCPRI fronthaul rate
    pub gps_lat:      i32,      // latitude × 1e6
    pub gps_lon:      i32,      // longitude × 1e6
    pub enabled:      AtomicBool,
}

// ── Network Slice Descriptor ─────────────────────────────────────────────────
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct NetworkSlice {
    pub snssai_sst:    u8,       // Slice/Service Type
    pub snssai_sd:     [u8; 3],  // Slice Differentiator
    pub slice_type:    SliceType,
    pub max_ues:       u32,
    pub guaranteed_br_kbps: u32, // Guaranteed Bit Rate
    pub max_br_kbps:   u32,     // Maximum Bit Rate
    pub latency_budget_ms: u16,
    pub reliability_percent: u8, // 99, 99.9, 99.99 etc. (×10 → 990, 999)
    pub isolation:     SliceIsolation,
    pub active:        AtomicBool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SliceIsolation {
    Shared,           // Shared RAN resources
    Dedicated,        // Dedicated PRBs
    HardIsolated,     // Separate DU instances
}

// ── xApp Framework ───────────────────────────────────────────────────────────
pub const MAX_XAPPS: usize = 64;
pub const XAPP_NAME_LEN: usize = 32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum XappState {
    Registered,
    Subscribing,
    Active,
    Suspended,
    Failed,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct XappDescriptor {
    pub name:        [u8; XAPP_NAME_LEN],
    pub xapp_id:     u32,
    pub state:       XappState,
    pub e2_sub_id:   u32,        // E2 subscription ID
    pub ran_func_id: u16,        // RAN function ID being monitored
    pub report_period_ms: u32,   // KPI report interval
    pub action:      XappAction,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum XappAction {
    Report,     // Receive KPI reports only
    Insert,     // Can inject RAN control messages
    Policy,     // Can set RAN policies via A1
}

// ── Global State ─────────────────────────────────────────────────────────────
const MAX_CUS:    usize = 8;
const MAX_DUS:    usize = 32;
const MAX_RUS:    usize = 128;
const MAX_SLICES: usize = 16;

static mut CU_TABLE:    [Option<CentralUnit>; MAX_CUS]     = [None; MAX_CUS];
static mut DU_TABLE:    [Option<DistributedUnit>; MAX_DUS]  = [None; MAX_DUS];
static mut RU_TABLE:    [Option<RadioUnit>; MAX_RUS]        = [None; MAX_RUS];
static mut SLICE_TABLE: [Option<NetworkSlice>; MAX_SLICES]  = [None; MAX_SLICES];
static mut XAPP_TABLE:  [Option<XappDescriptor>; MAX_XAPPS] = [None; MAX_XAPPS];

static TELCO_INITIALIZED: AtomicBool = AtomicBool::new(false);
static TOTAL_UES:         AtomicU32  = AtomicU32::new(0);
static TOTAL_SLICES:      AtomicU32  = AtomicU32::new(0);
static TRAI_VIOLATIONS:   AtomicU32  = AtomicU32::new(0);
static XAPP_COUNT:        AtomicU32  = AtomicU32::new(0);
static HANDOVER_COUNT:    AtomicU64  = AtomicU64::new(0);

// ── Initialization ───────────────────────────────────────────────────────────
pub fn telco_init() -> i32 {
    if TELCO_INITIALIZED.swap(true, Ordering::SeqCst) {
        return -1; // already initialized
    }

    // Initialize tables
    unsafe {
        for slot in CU_TABLE.iter_mut()    { *slot = None; }
        for slot in DU_TABLE.iter_mut()    { *slot = None; }
        for slot in RU_TABLE.iter_mut()    { *slot = None; }
        for slot in SLICE_TABLE.iter_mut() { *slot = None; }
        for slot in XAPP_TABLE.iter_mut()  { *slot = None; }
    }

    TOTAL_UES.store(0, Ordering::SeqCst);
    TOTAL_SLICES.store(0, Ordering::SeqCst);
    TRAI_VIOLATIONS.store(0, Ordering::SeqCst);
    XAPP_COUNT.store(0, Ordering::SeqCst);
    HANDOVER_COUNT.store(0, Ordering::SeqCst);

    0
}

// ── CU Management ────────────────────────────────────────────────────────────
pub fn cu_register(cu_id: u32, split: CuDuSplit, max_ues: u32,
                   amf_ip: [u8; 4], upf_ip: [u8; 4], plmn: [u8; 6]) -> i32 {
    unsafe {
        for slot in CU_TABLE.iter_mut() {
            if slot.is_none() {
                *slot = Some(CentralUnit {
                    cu_id, split, max_ues,
                    active_ues: AtomicU32::new(0),
                    amf_ip, upf_ip, plmn,
                    enabled: AtomicBool::new(true),
                });
                return 0;
            }
        }
    }
    -1 // table full
}

pub fn cu_attach_ue(cu_id: u32) -> i32 {
    unsafe {
        for slot in CU_TABLE.iter() {
            if let Some(cu) = slot {
                if cu.cu_id == cu_id && cu.enabled.load(Ordering::Relaxed) {
                    let prev = cu.active_ues.fetch_add(1, Ordering::SeqCst);
                    if prev >= cu.max_ues {
                        cu.active_ues.fetch_sub(1, Ordering::SeqCst);
                        return -1; // capacity exceeded
                    }
                    TOTAL_UES.fetch_add(1, Ordering::SeqCst);
                    return 0;
                }
            }
        }
    }
    -2 // CU not found
}

pub fn cu_detach_ue(cu_id: u32) -> i32 {
    unsafe {
        for slot in CU_TABLE.iter() {
            if let Some(cu) = slot {
                if cu.cu_id == cu_id {
                    let prev = cu.active_ues.fetch_sub(1, Ordering::SeqCst);
                    if prev == 0 {
                        cu.active_ues.store(0, Ordering::SeqCst);
                        return -1;
                    }
                    TOTAL_UES.fetch_sub(1, Ordering::SeqCst);
                    return 0;
                }
            }
        }
    }
    -2
}

// ── Network Slice Management ─────────────────────────────────────────────────
pub fn slice_create(sst: u8, sd: [u8; 3], slice_type: SliceType,
                    max_ues: u32, guaranteed_br: u32, max_br: u32,
                    latency_ms: u16, isolation: SliceIsolation) -> i32 {
    unsafe {
        for slot in SLICE_TABLE.iter_mut() {
            if slot.is_none() {
                *slot = Some(NetworkSlice {
                    snssai_sst: sst,
                    snssai_sd: sd,
                    slice_type,
                    max_ues,
                    guaranteed_br_kbps: guaranteed_br,
                    max_br_kbps: max_br,
                    latency_budget_ms: latency_ms,
                    reliability_percent: 99,
                    isolation,
                    active: AtomicBool::new(true),
                });
                TOTAL_SLICES.fetch_add(1, Ordering::SeqCst);
                return 0;
            }
        }
    }
    -1
}

pub fn slice_deactivate(sst: u8, sd: [u8; 3]) -> i32 {
    unsafe {
        for slot in SLICE_TABLE.iter() {
            if let Some(slice) = slot {
                if slice.snssai_sst == sst && slice.snssai_sd == sd {
                    slice.active.store(false, Ordering::SeqCst);
                    return 0;
                }
            }
        }
    }
    -1
}

// ── xApp Registration ────────────────────────────────────────────────────────
pub fn xapp_register(name: &[u8], ran_func_id: u16, action: XappAction,
                     report_period_ms: u32) -> i32 {
    let xapp_id = XAPP_COUNT.fetch_add(1, Ordering::SeqCst);
    if xapp_id as usize >= MAX_XAPPS {
        XAPP_COUNT.fetch_sub(1, Ordering::SeqCst);
        return -1;
    }

    let mut name_buf = [0u8; XAPP_NAME_LEN];
    let copy_len = if name.len() < XAPP_NAME_LEN { name.len() } else { XAPP_NAME_LEN };
    name_buf[..copy_len].copy_from_slice(&name[..copy_len]);

    unsafe {
        XAPP_TABLE[xapp_id as usize] = Some(XappDescriptor {
            name: name_buf,
            xapp_id,
            state: XappState::Registered,
            e2_sub_id: 0,
            ran_func_id,
            report_period_ms,
            action,
        });
    }

    xapp_id as i32
}

pub fn xapp_subscribe(xapp_id: u32, e2_sub_id: u32) -> i32 {
    if xapp_id as usize >= MAX_XAPPS { return -1; }
    unsafe {
        if let Some(ref mut xapp) = XAPP_TABLE[xapp_id as usize] {
            xapp.e2_sub_id = e2_sub_id;
            xapp.state = XappState::Active;
            return 0;
        }
    }
    -1
}

// ── TRAI QoS Enforcement ─────────────────────────────────────────────────────
/// Check if QoS metrics meet TRAI minimum standards.
/// Returns 0 if compliant, bitmap of violations otherwise.
pub fn trai_check_compliance(metrics: &TraiQosMetrics) -> u32 {
    let mut violations: u32 = 0;

    // TRAI minimum download speed: 2 Mbps for 4G
    let min_dl = match metrics.technology {
        RadioTech::LTE4G    => 2_000,
        RadioTech::NR5GSA   => 20_000,
        RadioTech::NR5GNSA  => 10_000,
        _                   => 1_000,
    };
    if metrics.download_speed_kbps < min_dl { violations |= 0x01; }

    // TRAI latency threshold
    let max_latency = match metrics.technology {
        RadioTech::LTE4G    => 100,
        RadioTech::NR5GSA   => 20,
        RadioTech::NR5GNSA  => 50,
        _                   => 200,
    };
    if metrics.latency_ms > max_latency { violations |= 0x02; }

    // Packet loss < 1% (10,000 ppm)
    if metrics.packet_loss_ppm > 10_000 { violations |= 0x04; }

    // Call drop rate < 2% (20,000 ppm)
    if metrics.call_drop_rate_ppm > 20_000 { violations |= 0x08; }

    if violations != 0 {
        TRAI_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
    }

    violations
}

// ── Handover Management ──────────────────────────────────────────────────────
pub fn handover_execute(ue_id: u32, source_du: u32, target_du: u32,
                        _measurement_report: &[u8]) -> i32 {
    // Verify source and target DUs exist
    let mut source_found = false;
    let mut target_found = false;
    unsafe {
        for slot in DU_TABLE.iter() {
            if let Some(du) = slot {
                if du.du_id == source_du && du.enabled.load(Ordering::Relaxed) {
                    source_found = true;
                }
                if du.du_id == target_du && du.enabled.load(Ordering::Relaxed) {
                    target_found = true;
                }
            }
        }
    }
    if !source_found || !target_found { return -1; }

    // In a real implementation, this would:
    // 1. Send Handover Request to target DU via XnAP
    // 2. Allocate resources on target cell
    // 3. Send RRC Reconfiguration to UE
    // 4. Receive Handover Complete from UE
    // 5. Release resources on source cell
    let _ = ue_id; // used in handover context tracking

    HANDOVER_COUNT.fetch_add(1, Ordering::SeqCst);
    0
}

// ── DU Registration ──────────────────────────────────────────────────────────
pub fn du_register(du_id: u32, cu_id: u32, cell_id: u32, pci: u16,
                   arfcn: u32, bandwidth_mhz: u8, mimo_layers: u8,
                   scs_khz: u16, tx_power: i16) -> i32 {
    unsafe {
        for slot in DU_TABLE.iter_mut() {
            if slot.is_none() {
                *slot = Some(DistributedUnit {
                    du_id, cu_id, cell_id, pci, arfcn,
                    bandwidth_mhz, mimo_layers,
                    subcarrier_spacing_khz: scs_khz,
                    tx_power_dbm: tx_power,
                    enabled: AtomicBool::new(true),
                });
                return 0;
            }
        }
    }
    -1
}

// ── RU Registration ──────────────────────────────────────────────────────────
pub fn ru_register(ru_id: u32, du_id: u32, antenna_ports: u8,
                   beamforming: bool, ecpri_gbps: u8,
                   lat: i32, lon: i32) -> i32 {
    unsafe {
        for slot in RU_TABLE.iter_mut() {
            if slot.is_none() {
                *slot = Some(RadioUnit {
                    ru_id, du_id, antenna_ports, beamforming,
                    ecpri_rate_gbps: ecpri_gbps,
                    gps_lat: lat, gps_lon: lon,
                    enabled: AtomicBool::new(true),
                });
                return 0;
            }
        }
    }
    -1
}

// ── Statistics ───────────────────────────────────────────────────────────────
pub fn telco_stats_total_ues() -> u32 { TOTAL_UES.load(Ordering::Relaxed) }
pub fn telco_stats_total_slices() -> u32 { TOTAL_SLICES.load(Ordering::Relaxed) }
pub fn telco_stats_trai_violations() -> u32 { TRAI_VIOLATIONS.load(Ordering::Relaxed) }
pub fn telco_stats_handovers() -> u64 { HANDOVER_COUNT.load(Ordering::Relaxed) }
pub fn telco_stats_xapps() -> u32 { XAPP_COUNT.load(Ordering::Relaxed) }

// ── C-ABI Exports ────────────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_telco_init() -> i32 { telco_init() }

#[no_mangle]
pub extern "C" fn sigma_telco_cu_register(
    cu_id: u32, split: u8, max_ues: u32,
    amf_ip: *const u8, upf_ip: *const u8, plmn: *const u8
) -> i32 {
    let split = match split {
        2 => CuDuSplit::Split2,
        6 => CuDuSplit::Split6,
        7 => CuDuSplit::Split7,
        8 => CuDuSplit::Split8,
        _ => return -1,
    };
    let mut amf = [0u8; 4];
    let mut upf = [0u8; 4];
    let mut pl  = [0u8; 6];
    unsafe {
        for i in 0..4 { amf[i] = *amf_ip.add(i); upf[i] = *upf_ip.add(i); }
        for i in 0..6 { pl[i] = *plmn.add(i); }
    }
    cu_register(cu_id, split, max_ues, amf, upf, pl)
}

#[no_mangle]
pub extern "C" fn sigma_telco_slice_create(
    sst: u8, sd0: u8, sd1: u8, sd2: u8, slice_type: u8,
    max_ues: u32, guaranteed_br: u32, max_br: u32,
    latency_ms: u16, isolation: u8
) -> i32 {
    let st = match slice_type {
        0 => SliceType::EMBB,
        1 => SliceType::URLLC,
        2 => SliceType::MMTC,
        3 => SliceType::V2X,
        n => SliceType::Custom(n),
    };
    let iso = match isolation {
        0 => SliceIsolation::Shared,
        1 => SliceIsolation::Dedicated,
        2 => SliceIsolation::HardIsolated,
        _ => return -1,
    };
    slice_create(sst, [sd0, sd1, sd2], st, max_ues, guaranteed_br, max_br, latency_ms, iso)
}

#[no_mangle]
pub extern "C" fn sigma_telco_xapp_register(
    name: *const u8, name_len: usize, ran_func_id: u16,
    action: u8, report_period_ms: u32
) -> i32 {
    let act = match action {
        0 => XappAction::Report,
        1 => XappAction::Insert,
        2 => XappAction::Policy,
        _ => return -1,
    };
    let name_slice = unsafe { core::slice::from_raw_parts(name, name_len) };
    xapp_register(name_slice, ran_func_id, act, report_period_ms)
}

#[no_mangle]
pub extern "C" fn sigma_telco_handover(
    ue_id: u32, source_du: u32, target_du: u32,
    report: *const u8, report_len: usize
) -> i32 {
    let rep = unsafe { core::slice::from_raw_parts(report, report_len) };
    handover_execute(ue_id, source_du, target_du, rep)
}

#[no_mangle]
pub extern "C" fn sigma_telco_stats_ues() -> u32 { telco_stats_total_ues() }

#[no_mangle]
pub extern "C" fn sigma_telco_stats_violations() -> u32 { telco_stats_trai_violations() }
