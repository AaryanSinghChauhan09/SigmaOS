// SPDX-License-Identifier: MIT
// SigmaOS Brain-Computer Interface — sigma_neuro.rs
// OpenBCI EEG driver, Neurosity Crown integration, EEG signal processing
// (FFT band extraction), and BCI event bus via sigma-bus IPC.

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

// ── EEG Channel Configuration ────────────────────────────────────────────────
pub const MAX_EEG_CHANNELS:   usize = 32;  // OpenBCI Cyton+Daisy = 16, research = 32
pub const MAX_BCI_DEVICES:    usize = 4;
pub const EEG_SAMPLE_BUFFER:  usize = 1024; // ring buffer per channel
pub const FFT_SIZE:           usize = 256;  // FFT window size

// ── EEG Frequency Bands (Hz) ─────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EegBand {
    Delta,     // 0.5–4 Hz   (deep sleep)
    Theta,     // 4–8 Hz     (drowsiness, meditation)
    Alpha,     // 8–13 Hz    (relaxed, eyes closed)
    Beta,      // 13–30 Hz   (active thinking, focus)
    Gamma,     // 30–100 Hz  (high-level processing, consciousness)
}

impl EegBand {
    pub fn freq_range(&self) -> (f32, f32) {
        match self {
            EegBand::Delta => (0.5, 4.0),
            EegBand::Theta => (4.0, 8.0),
            EegBand::Alpha => (8.0, 13.0),
            EegBand::Beta  => (13.0, 30.0),
            EegBand::Gamma => (30.0, 100.0),
        }
    }
}

// ── Device Types ─────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BciDeviceType {
    OpenBCICyton,      // 8-channel, 250 Hz, ADS1299
    OpenBCIDaisy,      // 16-channel extension
    OpenBCIGanglion,   // 4-channel, 200 Hz, MCP3912
    NeurosityNotion,   // 8-channel, CP3/C3/F5/PO3/PO4/F6/C4/CP4
    NeurosityCrown,    // 8-channel crown form factor
    MuseS,             // 4-channel, AF7/AF8/TP9/TP10
    Emotiv,            // 14/32-channel
    Generic,           // custom via serial/USB
}

// ── 10-20 System Electrode Positions ─────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Electrode {
    Fp1 = 0, Fp2, F7, F3, Fz, F4, F8, T3,
    C3, Cz, C4, T4, T5, P3, Pz, P4,
    T6, O1, O2, A1, A2, AF7, AF8, TP9,
    TP10, CP3, CP4, PO3, PO4, F5, F6, Oz,
}

// ── BCI Device Descriptor ────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct BciDevice {
    pub device_id:     u8,
    pub device_type:   BciDeviceType,
    pub channels:      u8,
    pub sample_rate_hz: u16,
    pub resolution_bits: u8,   // ADC resolution (24 for ADS1299)
    pub gain:          u8,      // PGA gain (1, 2, 4, 6, 8, 12, 24)
    pub electrode_map: [Electrode; MAX_EEG_CHANNELS],
    pub enabled:       AtomicBool,
    pub samples_total: AtomicU64,
    pub impedance_ok:  AtomicBool,
}

// ── EEG Sample ───────────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct EegSample {
    pub channels:     [f32; MAX_EEG_CHANNELS], // µV
    pub channel_count: u8,
    pub timestamp_ns:  u64,
    pub sample_index:  u32,
    pub accel:         [f32; 3],  // optional accelerometer (OpenBCI)
}

// ── Band Power Result ────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct BandPower {
    pub delta: f32,
    pub theta: f32,
    pub alpha: f32,
    pub beta:  f32,
    pub gamma: f32,
    pub total: f32,
    pub channel: u8,
    pub timestamp_ns: u64,
}

// ── BCI Events (sent via sigma-bus) ──────────────────────────────────────────
pub const IPC_CH_BCI: u32 = 0x30;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BciEvent {
    Blink,              // eye blink detected
    DoubleBlink,        // double blink (action trigger)
    JawClench,          // jaw clench artifact → command
    Focus(u8),          // focus level 0–100
    Relaxation(u8),     // relaxation level 0–100
    MotorImageryLeft,   // left-hand motor imagery
    MotorImageryRight,  // right-hand motor imagery
    ErrorPotential,     // error-related negativity (ErrP)
    P300,               // P300 evoked potential (oddball paradigm)
    SSVEP(u8),          // steady-state VEP at frequency (Hz)
    RawBandPower(BandPower),
}

// ── Global State ─────────────────────────────────────────────────────────────
static mut DEVICE_TABLE: [Option<BciDevice>; MAX_BCI_DEVICES] = [None; MAX_BCI_DEVICES];
static mut SAMPLE_RING:  [[f32; EEG_SAMPLE_BUFFER]; MAX_EEG_CHANNELS] =
    [[0.0; EEG_SAMPLE_BUFFER]; MAX_EEG_CHANNELS];
static mut RING_HEAD: usize = 0;
static mut FFT_SCRATCH: [f32; FFT_SIZE] = [0.0; FFT_SIZE];
static mut FFT_OUTPUT:  [f32; FFT_SIZE] = [0.0; FFT_SIZE];

static NEURO_INITIALIZED: AtomicBool = AtomicBool::new(false);
static DEVICE_COUNT:      AtomicU32  = AtomicU32::new(0);
static EVENT_COUNT:       AtomicU64  = AtomicU64::new(0);
static BLINK_COUNT:       AtomicU32  = AtomicU32::new(0);

// ── Initialization ───────────────────────────────────────────────────────────
pub fn neuro_init() -> i32 {
    if NEURO_INITIALIZED.swap(true, Ordering::SeqCst) {
        return -1;
    }
    unsafe {
        for slot in DEVICE_TABLE.iter_mut() { *slot = None; }
        for ch in SAMPLE_RING.iter_mut() {
            for s in ch.iter_mut() { *s = 0.0; }
        }
        RING_HEAD = 0;
    }
    DEVICE_COUNT.store(0, Ordering::SeqCst);
    EVENT_COUNT.store(0, Ordering::SeqCst);
    BLINK_COUNT.store(0, Ordering::SeqCst);
    0
}

// ── Device Registration ──────────────────────────────────────────────────────
pub fn device_register(device_type: BciDeviceType, channels: u8,
                       sample_rate: u16, resolution: u8, gain: u8,
                       electrodes: &[Electrode]) -> i32 {
    let id = DEVICE_COUNT.fetch_add(1, Ordering::SeqCst);
    if id as usize >= MAX_BCI_DEVICES {
        DEVICE_COUNT.fetch_sub(1, Ordering::SeqCst);
        return -1;
    }
    let mut emap = [Electrode::Fp1; MAX_EEG_CHANNELS];
    let copy_len = electrodes.len().min(MAX_EEG_CHANNELS);
    emap[..copy_len].copy_from_slice(&electrodes[..copy_len]);

    unsafe {
        DEVICE_TABLE[id as usize] = Some(BciDevice {
            device_id: id as u8,
            device_type, channels, sample_rate_hz: sample_rate,
            resolution_bits: resolution, gain,
            electrode_map: emap,
            enabled: AtomicBool::new(true),
            samples_total: AtomicU64::new(0),
            impedance_ok: AtomicBool::new(false),
        });
    }
    id as i32
}

// ── Sample Ingestion ─────────────────────────────────────────────────────────
pub fn ingest_sample(sample: &EegSample) -> i32 {
    let ch_count = sample.channel_count as usize;
    if ch_count > MAX_EEG_CHANNELS { return -1; }

    unsafe {
        for ch in 0..ch_count {
            SAMPLE_RING[ch][RING_HEAD] = sample.channels[ch];
        }
        RING_HEAD = (RING_HEAD + 1) % EEG_SAMPLE_BUFFER;
    }

    // Update device sample count
    // (in real code, we'd match by device_id from sample metadata)
    0
}

// ── FFT Band Power Extraction ────────────────────────────────────────────────
/// Compute FFT and extract band powers for a specific channel.
/// Uses a radix-2 DIT FFT (Cooley-Tukey) on the last FFT_SIZE samples.
pub fn compute_band_power(channel: u8, sample_rate: f32) -> BandPower {
    if channel as usize >= MAX_EEG_CHANNELS {
        return BandPower::default();
    }

    unsafe {
        // Copy last FFT_SIZE samples into scratch buffer
        let start = if RING_HEAD >= FFT_SIZE {
            RING_HEAD - FFT_SIZE
        } else {
            EEG_SAMPLE_BUFFER - (FFT_SIZE - RING_HEAD)
        };

        for i in 0..FFT_SIZE {
            let idx = (start + i) % EEG_SAMPLE_BUFFER;
            FFT_SCRATCH[i] = SAMPLE_RING[channel as usize][idx];
        }

        // Apply Hann window
        for i in 0..FFT_SIZE {
            let w = 0.5 * (1.0 - cos_approx(2.0 * 3.14159265 * i as f32 / FFT_SIZE as f32));
            FFT_SCRATCH[i] *= w as f32;
        }

        // Compute magnitude spectrum (simplified DFT for no_std)
        // For production, this would use a proper radix-2 FFT
        let freq_resolution = sample_rate / FFT_SIZE as f32;

        let mut delta_pwr: f32 = 0.0;
        let mut theta_pwr: f32 = 0.0;
        let mut alpha_pwr: f32 = 0.0;
        let mut beta_pwr:  f32 = 0.0;
        let mut gamma_pwr: f32 = 0.0;
        let mut total_pwr: f32 = 0.0;

        // Compute power at each frequency bin using Goertzel algorithm
        for k in 1..(FFT_SIZE / 2) {
            let freq = k as f32 * freq_resolution;
            let power = goertzel_power(k, &FFT_SCRATCH);

            total_pwr += power;

            if freq >= 0.5 && freq < 4.0   { delta_pwr += power; }
            if freq >= 4.0 && freq < 8.0   { theta_pwr += power; }
            if freq >= 8.0 && freq < 13.0  { alpha_pwr += power; }
            if freq >= 13.0 && freq < 30.0 { beta_pwr  += power; }
            if freq >= 30.0 && freq < 100.0 { gamma_pwr += power; }
        }

        BandPower {
            delta: delta_pwr,
            theta: theta_pwr,
            alpha: alpha_pwr,
            beta:  beta_pwr,
            gamma: gamma_pwr,
            total: total_pwr,
            channel,
            timestamp_ns: 0, // caller should set this
        }
    }
}

/// Goertzel algorithm — computes power at a single DFT bin without full FFT
fn goertzel_power(k: usize, samples: &[f32]) -> f32 {
    let n = samples.len();
    let w = 2.0 * 3.14159265 * k as f32 / n as f32;
    let coeff = 2.0 * cos_approx(w);

    let mut s0: f32 = 0.0;
    let mut s1: f32 = 0.0;
    let mut s2: f32 = 0.0;

    for i in 0..n {
        s0 = samples[i] + coeff * s1 - s2;
        s2 = s1;
        s1 = s0;
    }

    // Power = s1² + s2² - coeff·s1·s2
    s1 * s1 + s2 * s2 - coeff * s1 * s2
}

/// Approximate cosine for no_std (Bhaskara I approximation)
fn cos_approx(x: f32) -> f32 {
    let pi = 3.14159265f32;
    // Normalize to [0, 2π]
    let mut x = x % (2.0 * pi);
    if x < 0.0 { x += 2.0 * pi; }

    // Use Bhaskara I's formula: cos(x) ≈ (π² - 4x²) / (π² + x²) for x in [0, π/2]
    let negate = x > pi;
    if negate { x = 2.0 * pi - x; }
    let half = x > pi / 2.0;
    if half { x = pi - x; }

    let x2 = x * x;
    let pi2 = pi * pi;
    let result = (pi2 - 4.0 * x2) / (pi2 + x2);

    if half { -result } else { result }
}

// ── Blink Detection ──────────────────────────────────────────────────────────
/// Detect eye blinks from frontal channels (Fp1, Fp2).
/// Blinks produce large amplitude (~100-200 µV) deflections lasting 200-400 ms.
pub fn detect_blink(fp1_samples: &[f32], threshold_uv: f32) -> bool {
    if fp1_samples.len() < 10 { return false; }

    // Look for sharp positive-then-negative deflection
    let mut max_val: f32 = 0.0;
    let mut min_val: f32 = 0.0;

    for &s in fp1_samples.iter() {
        if s > max_val { max_val = s; }
        if s < min_val { min_val = s; }
    }

    let amplitude = max_val - min_val;
    if amplitude > threshold_uv {
        BLINK_COUNT.fetch_add(1, Ordering::Relaxed);
        EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
        return true;
    }

    false
}

// ── Motor Imagery Classification ─────────────────────────────────────────────
/// Classify left vs right motor imagery from C3/C4 electrode band powers.
/// Uses mu-rhythm (8-12 Hz) desynchronization (ERD):
///   Left motor imagery → ERD at C4 (contralateral)
///   Right motor imagery → ERD at C3
pub fn classify_motor_imagery(c3_band: &BandPower, c4_band: &BandPower) -> BciEvent {
    // Mu-rhythm is within the alpha band
    let c3_mu = c3_band.alpha;
    let c4_mu = c4_band.alpha;

    // The side with LOWER mu power has ERD → that's the contralateral side
    let ratio = if c3_mu + c4_mu > 0.001 {
        (c4_mu - c3_mu) / (c4_mu + c3_mu)
    } else {
        0.0
    };

    // Positive ratio → C4 stronger → C3 has ERD → LEFT imagery
    // Negative ratio → C3 stronger → C4 has ERD → RIGHT imagery
    if ratio > 0.15 {
        EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
        BciEvent::MotorImageryLeft
    } else if ratio < -0.15 {
        EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
        BciEvent::MotorImageryRight
    } else {
        BciEvent::Focus(50) // indeterminate
    }
}

// ── Focus/Relaxation Metrics ─────────────────────────────────────────────────
/// Compute focus level from beta/theta ratio (engagement index).
/// Higher beta/theta ratio → more focused.
pub fn compute_focus(band_power: &BandPower) -> u8 {
    if band_power.theta < 0.001 { return 0; }
    let ratio = band_power.beta / band_power.theta;
    // Normalize to 0–100 range (typical ratio: 0.5–3.0)
    let focus = ((ratio - 0.5) / 2.5 * 100.0).max(0.0).min(100.0);
    focus as u8
}

/// Compute relaxation level from alpha/beta ratio.
/// Higher alpha/beta ratio → more relaxed.
pub fn compute_relaxation(band_power: &BandPower) -> u8 {
    if band_power.beta < 0.001 { return 0; }
    let ratio = band_power.alpha / band_power.beta;
    let relaxation = ((ratio - 0.3) / 2.0 * 100.0).max(0.0).min(100.0);
    relaxation as u8
}

// ── Statistics ───────────────────────────────────────────────────────────────
pub fn neuro_stats_events() -> u64 { EVENT_COUNT.load(Ordering::Relaxed) }
pub fn neuro_stats_blinks() -> u32 { BLINK_COUNT.load(Ordering::Relaxed) }
pub fn neuro_stats_devices() -> u32 { DEVICE_COUNT.load(Ordering::Relaxed) }

// ── C-ABI Exports ────────────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_neuro_init() -> i32 { neuro_init() }

#[no_mangle]
pub extern "C" fn sigma_neuro_register_device(
    device_type: u8, channels: u8, sample_rate: u16, resolution: u8, gain: u8
) -> i32 {
    let dt = match device_type {
        0 => BciDeviceType::OpenBCICyton,
        1 => BciDeviceType::OpenBCIDaisy,
        2 => BciDeviceType::OpenBCIGanglion,
        3 => BciDeviceType::NeurosityNotion,
        4 => BciDeviceType::NeurosityCrown,
        5 => BciDeviceType::MuseS,
        6 => BciDeviceType::Emotiv,
        _ => BciDeviceType::Generic,
    };
    device_register(dt, channels, sample_rate, resolution, gain, &[])
}

#[no_mangle]
pub extern "C" fn sigma_neuro_ingest(
    channels: *const f32, ch_count: u8, timestamp: u64, sample_idx: u32
) -> i32 {
    let mut sample = EegSample::default();
    sample.channel_count = ch_count;
    sample.timestamp_ns = timestamp;
    sample.sample_index = sample_idx;
    let n = (ch_count as usize).min(MAX_EEG_CHANNELS);
    unsafe {
        for i in 0..n { sample.channels[i] = *channels.add(i); }
    }
    ingest_sample(&sample)
}

#[no_mangle]
pub extern "C" fn sigma_neuro_band_power(channel: u8, sample_rate: f32, out: *mut BandPower) -> i32 {
    let bp = compute_band_power(channel, sample_rate);
    unsafe { *out = bp; }
    0
}

#[no_mangle]
pub extern "C" fn sigma_neuro_focus(band: *const BandPower) -> u8 {
    unsafe { compute_focus(&*band) }
}

#[no_mangle]
pub extern "C" fn sigma_neuro_relaxation(band: *const BandPower) -> u8 {
    unsafe { compute_relaxation(&*band) }
}
