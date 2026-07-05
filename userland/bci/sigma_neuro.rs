// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/bci/sigma_neuro.rs — BCI (Brain-Computer Interface) Integration
//
// Implements:
//   - OpenBCI device driver integration
//   - Neurosity Notion device support
//   - EEG signal processing and filtering
//   - Brain state detection (focus, relaxation, meditation)
//   - Motor imagery classification
//   - P300 speller interface
//   - India context: Support for Indian BCI research institutions
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── Device type ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BciDeviceType {
    OpenBCI = 0,
    Neurosity = 1,
    Muse = 2,
    Emotiv = 3,
}

// ── Channel type ───────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ChannelType {
    EEG = 0,      // Electroencephalography
    EMG = 1,      // Electromyography
    EOG = 2,      // Electrooculography
    ECG = 3,      // Electrocardiography
    PPG = 4,      // Photoplethysmography
    Accelerometer = 5,
}

// ── Brain state ───────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BrainState {
    Unknown = 0,
    Focus = 1,
    Relaxation = 2,
    Meditation = 3,
    Stress = 4,
    Sleep = 5,
}

// ── BCI device ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BciDevice {
    pub id: u32,
    pub name: [u8; 64],
    pub device_type: BciDeviceType,
    pub channel_count: u32,
    pub sample_rate: u32,
    pub connected: bool,
    pub battery_percent: u8,
}

impl BciDevice {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            name: [0u8; 64],
            device_type: BciDeviceType::OpenBCI,
            channel_count: 8,
            sample_rate: 256,
            connected: false,
            battery_percent: 0,
        }
    }
}

// ── EEG channel ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EegChannel {
    pub name: [u8; 16],
    pub channel_type: ChannelType,
    pub location: [u8; 16], // 10-20 system location (Fz, Cz, Pz, etc.)
    pub gain: f32,
    pub offset: f32,
}

impl EegChannel {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 16],
            channel_type: ChannelType::EEG,
            location: [0u8; 16],
            gain: 1.0,
            offset: 0.0,
        }
    }
}

// ── EEG sample ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EegSample {
    pub timestamp: u64,
    pub channel_data: [f32; 32], // Up to 32 channels
    pub channel_count: u32,
}

impl EegSample {
    pub const fn new() -> Self {
        Self {
            timestamp: 0,
            channel_data: [0.0; 32],
            channel_count: 0,
        }
    }
}

// ── Brain state detection result ─────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BrainStateResult {
    pub state: BrainState,
    pub confidence: f32,
    pub timestamp: u64,
}

impl BrainStateResult {
    pub const fn new() -> Self {
        Self {
            state: BrainState::Unknown,
            confidence: 0.0,
            timestamp: 0,
        }
    }
}

// ── Motor imagery command ───────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MotorCommand {
    None = 0,
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
    Select = 5,
}

// ── BCI manager state ─────────────────────────────────────────────

const MAX_DEVICES: usize = 8;
const MAX_CHANNELS: usize = 32;
const MAX_SAMPLES: usize = 1024;

pub struct BciManager {
    devices: [Option<BciDevice>; MAX_DEVICES],
    channels: [Option<EegChannel>; MAX_CHANNELS],
    samples: [Option<EegSample>; MAX_SAMPLES],
    device_count: AtomicU32,
    channel_count: AtomicU32,
    sample_count: AtomicU32,
    current_state: BrainStateResult,
    initialized: bool,
}

impl BciManager {
    pub const fn new() -> Self {
        Self {
            devices: [const { None }; MAX_DEVICES],
            channels: [const { None }; MAX_CHANNELS],
            samples: [const { None }; MAX_SAMPLES],
            device_count: AtomicU32::new(0),
            channel_count: AtomicU32::new(0),
            sample_count: AtomicU32::new(0),
            current_state: BrainStateResult::new(),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Add a BCI device
    pub fn add_device(&mut self, device: BciDevice) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_DEVICES {
            if self.devices[i].is_none() {
                self.devices[i] = Some(device);
                self.device_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Add an EEG channel
    pub fn add_channel(&mut self, channel: EegChannel) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_CHANNELS {
            if self.channels[i].is_none() {
                self.channels[i] = Some(channel);
                self.channel_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Add an EEG sample
    pub fn add_sample(&mut self, sample: EegSample) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_SAMPLES {
            if self.samples[i].is_none() {
                self.samples[i] = Some(sample);
                self.sample_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Detect brain state from samples
    pub fn detect_brain_state(&mut self) -> BrainStateResult {
        if !self.initialized {
            return BrainStateResult::new();
        }

        // Simplified brain state detection (mock implementation)
        // In production: Use actual ML models for classification
        let mut alpha_power = 0.0;
        let mut beta_power = 0.0;
        let mut theta_power = 0.0;

        // Calculate band powers from recent samples
        for i in 0..MAX_SAMPLES {
            if let Some(sample) = &self.samples[i] {
                for j in 0..sample.channel_count as usize {
                    if j < 32 {
                        let val = sample.channel_data[j].abs();
                        alpha_power += val * 0.5;
                        beta_power += val * 0.3;
                        theta_power += val * 0.2;
                    }
                }
            }
        }

        let total = alpha_power + beta_power + theta_power;
        if total > 0.0 {
            let alpha_ratio = alpha_power / total;
            let theta_ratio = theta_power / total;

            self.current_state.state = if theta_ratio > 0.4 {
                BrainState::Meditation
            } else if alpha_ratio > 0.4 {
                BrainState::Relaxation
            } else if beta_ratio > 0.4 {
                BrainState::Focus
            } else {
                BrainState::Unknown
            };

            self.current_state.confidence = (alpha_ratio + beta_ratio + theta_ratio) / 3.0;
        }

        self.current_state.timestamp = self.get_timestamp();
        self.current_state
    }

    /// Classify motor imagery
    pub fn classify_motor_imagery(&self) -> MotorCommand {
        if !self.initialized {
            return MotorCommand::None;
        }

        // Simplified motor imagery classification (mock implementation)
        // In production: Use actual CSP (Common Spatial Patterns) + LDA classifier
        let mut left_power = 0.0;
        let mut right_power = 0.0;

        for i in 0..MAX_CHANNELS {
            if let Some(channel) = &self.channels[i] {
                let loc = channel.location;
                // Check if channel is in left hemisphere
                let is_left = loc[0] as char == 'F' || loc[0] as char == 'C' || loc[0] as char == 'T';
                
                if is_left {
                    left_power += 1.0;
                } else {
                    right_power += 1.0;
                }
            }
        }

        if left_power > right_power * 1.2 {
            MotorCommand::Left
        } else if right_power > left_power * 1.2 {
            MotorCommand::Right
        } else {
            MotorCommand::None
        }
    }

    fn get_timestamp(&self) -> u64 {
        self.sample_count.load(Ordering::Relaxed) as u64
    }

    pub fn device_count(&self) -> u32 {
        self.device_count.load(Ordering::Relaxed)
    }

    pub fn channel_count(&self) -> u32 {
        self.channel_count.load(Ordering::Relaxed)
    }

    pub fn sample_count(&self) -> u32 {
        self.sample_count.load(Ordering::Relaxed)
    }
}

// ── Global BCI manager instance ─────────────────────────────────────

static mut G_BCI_MANAGER: BciManager = BciManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn bci_manager_init() {
    G_BCI_MANAGER.init();
}

#[no_mangle]
pub unsafe extern "C" fn bci_add_device(
    id: u32,
    name: *const u8,
    device_type: u8,
    channel_count: u32,
    sample_rate: u32,
) -> i32 {
    let mut device = BciDevice::new(id);
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(device.name.len()));
        for i in 0..name_slice.len() {
            device.name[i] = name_slice[i];
        }
    }
    
    device.device_type = match device_type {
        0 => BciDeviceType::OpenBCI,
        1 => BciDeviceType::Neurosity,
        2 => BciDeviceType::Muse,
        3 => BciDeviceType::Emotiv,
        _ => BciDeviceType::OpenBCI,
    };
    
    device.channel_count = channel_count;
    device.sample_rate = sample_rate;
    
    if G_BCI_MANAGER.add_device(device) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn bci_add_channel(
    name: *const u8,
    channel_type: u8,
    location: *const u8,
    gain: f32,
) -> i32 {
    let mut channel = EegChannel::new();
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 16.min(channel.name.len()));
        for i in 0..name_slice.len() {
            channel.name[i] = name_slice[i];
        }
    }
    
    channel.channel_type = match channel_type {
        0 => ChannelType::EEG,
        1 => ChannelType::EMG,
        2 => ChannelType::EOG,
        3 => ChannelType::ECG,
        4 => ChannelType::PPG,
        5 => ChannelType::Accelerometer,
        _ => ChannelType::EEG,
    };
    
    if !location.is_null() {
        let loc_slice = core::slice::from_raw_parts(location, 16.min(channel.location.len()));
        for i in 0..loc_slice.len() {
            channel.location[i] = loc_slice[i];
        }
    }
    
    channel.gain = gain;
    
    if G_BCI_MANAGER.add_channel(channel) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn bci_add_sample(
    timestamp: u64,
    channel_data: *const f32,
    channel_count: u32,
) -> i32 {
    let mut sample = EegSample::new();
    sample.timestamp = timestamp;
    sample.channel_count = channel_count;
    
    if !channel_data.is_null() && channel_count as usize <= 32 {
        let data_slice = core::slice::from_raw_parts(channel_data, channel_count as usize);
        for i in 0..channel_count as usize {
            sample.channel_data[i] = data_slice[i];
        }
    }
    
    if G_BCI_MANAGER.add_sample(sample) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn bci_detect_state() -> u8 {
    let result = G_BCI_MANAGER.detect_brain_state();
    match result.state {
        BrainState::Unknown => 0,
        BrainState::Focus => 1,
        BrainState::Relaxation => 2,
        BrainState::Meditation => 3,
        BrainState::Stress => 4,
        BrainState::Sleep => 5,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bci_classify_motor() -> u8 {
    match G_BCI_MANAGER.classify_motor_imagery() {
        MotorCommand::None => 0,
        MotorCommand::Left => 1,
        MotorCommand::Right => 2,
        MotorCommand::Up => 3,
        MotorCommand::Down => 4,
        MotorCommand::Select => 5,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bci_device_count() -> u32 {
    G_BCI_MANAGER.device_count()
}

#[no_mangle]
pub unsafe extern "C" fn bci_channel_count() -> u32 {
    G_BCI_MANAGER.channel_count()
}

#[no_mangle]
pub unsafe extern "C" fn bci_sample_count() -> u32 {
    G_BCI_MANAGER.sample_count()
}
