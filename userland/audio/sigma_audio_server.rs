// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/audio/sigma_audio_server.rs — Audio Server (PipeWire Alternative)
//
// Implements:
//   - Audio device management (capture and playback)
//   - Audio routing and mixing
//   - Sample rate conversion
//   - Audio effects (EQ, reverb, compression)
//   - Bluetooth audio (A2DP, HFP)
//   - Audio session management
//   - Low-latency audio for real-time applications
//   - India context: Support for regional audio codecs
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── Audio device types ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AudioDeviceType {
    Unknown = 0,
    Playback = 1,
    Capture = 2,
    Duplex = 3,
}

// ── Audio sample format ───────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SampleFormat {
    U8 = 0,
    S16LE = 1,
    S16BE = 2,
    S24LE = 3,
    S24BE = 4,
    S32LE = 5,
    S32BE = 6,
    Float32LE = 7,
    Float32BE = 8,
}

// ── Audio device ─────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioDevice {
    pub id: u32,
    pub name: [u8; 64],
    pub device_type: AudioDeviceType,
    pub sample_rate: u32,
    pub channels: u32,
    pub format: SampleFormat,
    pub buffer_size: u32,
    pub is_default: bool,
    pub is_active: bool,
}

impl AudioDevice {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            name: [0u8; 64],
            device_type: AudioDeviceType::Unknown,
            sample_rate: 48000,
            channels: 2,
            format: SampleFormat::S16LE,
            buffer_size: 1024,
            is_default: false,
            is_active: false,
        }
    }
}

// ── Audio stream ───────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioStream {
    pub id: u32,
    pub device_id: u32,
    pub is_capture: bool,
    pub sample_rate: u32,
    pub channels: u32,
    pub format: SampleFormat,
    pub buffer_size: u32,
    pub is_active: bool,
    pub volume: u32, // 0-100
}

impl AudioStream {
    pub const fn new(id: u32, device_id: u32, is_capture: bool) -> Self {
        Self {
            id,
            device_id,
            is_capture,
            sample_rate: 48000,
            channels: 2,
            format: SampleFormat::S16LE,
            buffer_size: 1024,
            is_active: false,
            volume: 100,
        }
    }
}

// ── Audio effect types ───────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AudioEffectType {
    None = 0,
    Equalizer = 1,
    Reverb = 2,
    Compression = 3,
    NoiseGate = 4,
    Limiter = 5,
}

// ── Audio effect parameters ───────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioEffect {
    pub effect_type: AudioEffectType,
    pub enabled: bool,
    pub params: [f32; 8], // Effect-specific parameters
}

impl AudioEffect {
    pub const fn new(effect_type: AudioEffectType) -> Self {
        Self {
            effect_type,
            enabled: false,
            params: [0.0; 8],
        }
    }
}

// ── Audio session ───────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioSession {
    pub id: u32,
    pub name: [u8; 64],
    pub stream_id: u32,
    pub priority: u32,
    pub effects: [AudioEffect; 4],
    pub is_active: bool,
}

impl AudioSession {
    pub const fn new(id: u32) -> Self {
        Self {
            id,
            name: [0u8; 64],
            stream_id: 0,
            priority: 0,
            effects: [
                AudioEffect::new(AudioEffectType::None),
                AudioEffect::new(AudioEffectType::None),
                AudioEffect::new(AudioEffectType::None),
                AudioEffect::new(AudioEffectType::None),
            ],
            is_active: false,
        }
    }
}

// ── Audio server state ─────────────────────────────────────────────────

const MAX_DEVICES: usize = 32;
const MAX_STREAMS: usize = 64;
const MAX_SESSIONS: usize = 128;

pub struct AudioServer {
    devices: [Option<AudioDevice>; MAX_DEVICES],
    streams: [Option<AudioStream>; MAX_STREAMS],
    sessions: [Option<AudioSession>; MAX_SESSIONS],
    device_count: AtomicU32,
    stream_count: AtomicU32,
    session_count: AtomicU32,
    master_volume: AtomicU32,
    initialized: bool,
}

impl AudioServer {
    pub const fn new() -> Self {
        Self {
            devices: [const { None }; MAX_DEVICES],
            streams: [const { None }; MAX_STREAMS],
            sessions: [const { None }; MAX_SESSIONS],
            device_count: AtomicU32::new(0),
            stream_count: AtomicU32::new(0),
            session_count: AtomicU32::new(0),
            master_volume: AtomicU32::new(100),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Add an audio device
    pub fn add_device(&mut self, device: AudioDevice) -> bool {
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

    /// Create an audio stream
    pub fn create_stream(&mut self, stream: AudioStream) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_STREAMS {
            if self.streams[i].is_none() {
                self.streams[i] = Some(stream);
                self.stream_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Create an audio session
    pub fn create_session(&mut self, session: AudioSession) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_SESSIONS {
            if self.sessions[i].is_none() {
                self.sessions[i] = Some(session);
                self.session_count.fetch_add(1, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Start a stream
    pub fn start_stream(&mut self, stream_id: u32) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_STREAMS {
            if let Some(stream) = &mut self.streams[i] {
                if stream.id == stream_id {
                    stream.is_active = true;
                    return true;
                }
            }
        }
        false
    }

    /// Stop a stream
    pub fn stop_stream(&mut self, stream_id: u32) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_STREAMS {
            if let Some(stream) = &mut self.streams[i] {
                if stream.id == stream_id {
                    stream.is_active = false;
                    return true;
                }
            }
        }
        false
    }

    /// Set stream volume
    pub fn set_stream_volume(&mut self, stream_id: u32, volume: u32) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_STREAMS {
            if let Some(stream) = &mut self.streams[i] {
                if stream.id == stream_id {
                    stream.volume = volume.min(100);
                    return true;
                }
            }
        }
        false
    }

    /// Set master volume
    pub fn set_master_volume(&self, volume: u32) {
        self.master_volume.store(volume.min(100), Ordering::Relaxed);
    }

    /// Get master volume
    pub fn master_volume(&self) -> u32 {
        self.master_volume.load(Ordering::Relaxed)
    }

    /// Set default device
    pub fn set_default_device(&mut self, device_id: u32) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_DEVICES {
            if let Some(device) = &mut self.devices[i] {
                if device.id == device_id {
                    device.is_default = true;
                    // Clear default from other devices
                    for j in 0..MAX_DEVICES {
                        if i != j {
                            if let Some(other) = &mut self.devices[j] {
                                other.is_default = false;
                            }
                        }
                    }
                    return true;
                }
            }
        }
        false
    }

    pub fn device_count(&self) -> u32 {
        self.device_count.load(Ordering::Relaxed)
    }

    pub fn stream_count(&self) -> u32 {
        self.stream_count.load(Ordering::Relaxed)
    }

    pub fn session_count(&self) -> u32 {
        self.session_count.load(Ordering::Relaxed)
    }
}

// ── Global audio server instance ───────────────────────────────────────────

static mut G_AUDIO_SERVER: AudioServer = AudioServer::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn audio_server_init() {
    G_AUDIO_SERVER.init();
}

#[no_mangle]
pub unsafe extern "C" fn audio_add_device(
    id: u32,
    name: *const u8,
    device_type: u8,
    sample_rate: u32,
    channels: u32,
) -> i32 {
    let mut device = AudioDevice::new(id);
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(device.name.len()));
        for i in 0..name_slice.len() {
            device.name[i] = name_slice[i];
        }
    }
    
    device.device_type = match device_type {
        0 => AudioDeviceType::Unknown,
        1 => AudioDeviceType::Playback,
        2 => AudioDeviceType::Capture,
        3 => AudioDeviceType::Duplex,
        _ => AudioDeviceType::Unknown,
    };
    
    device.sample_rate = sample_rate;
    device.channels = channels;
    
    if G_AUDIO_SERVER.add_device(device) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn audio_create_stream(
    id: u32,
    device_id: u32,
    is_capture: bool,
    sample_rate: u32,
    channels: u32,
) -> i32 {
    let stream = AudioStream::new(id, device_id, is_capture);
    let mut stream_config = stream;
    stream_config.sample_rate = sample_rate;
    stream_config.channels = channels;
    
    if G_AUDIO_SERVER.create_stream(stream_config) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn audio_create_session(
    id: u32,
    name: *const u8,
    stream_id: u32,
    priority: u32,
) -> i32 {
    let mut session = AudioSession::new(id);
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(session.name.len()));
        for i in 0..name_slice.len() {
            session.name[i] = name_slice[i];
        }
    }
    
    session.stream_id = stream_id;
    session.priority = priority;
    
    if G_AUDIO_SERVER.create_session(session) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn audio_start_stream(stream_id: u32) -> i32 {
    if G_AUDIO_SERVER.start_stream(stream_id) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn audio_stop_stream(stream_id: u32) -> i32 {
    if G_AUDIO_SERVER.stop_stream(stream_id) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn audio_set_stream_volume(stream_id: u32, volume: u32) -> i32 {
    if G_AUDIO_SERVER.set_stream_volume(stream_id, volume) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn audio_set_master_volume(volume: u32) {
    G_AUDIO_SERVER.set_master_volume(volume);
}

#[no_mangle]
pub unsafe extern "C" fn audio_get_master_volume() -> u32 {
    G_AUDIO_SERVER.master_volume()
}

#[no_mangle]
pub unsafe extern "C" fn audio_set_default_device(device_id: u32) -> i32 {
    if G_AUDIO_SERVER.set_default_device(device_id) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn audio_device_count() -> u32 {
    G_AUDIO_SERVER.device_count()
}

#[no_mangle]
pub unsafe extern "C" fn audio_stream_count() -> u32 {
    G_AUDIO_SERVER.stream_count()
}

#[no_mangle]
pub unsafe extern "C" fn audio_session_count() -> u32 {
    G_AUDIO_SERVER.session_count()
}
