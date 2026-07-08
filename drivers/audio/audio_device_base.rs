// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/audio/audio_device_base.rs — Base Device Trait for Audio Drivers
//
// Defines the OOP base class for all audio devices using Rust traits.
// This provides a common interface for audio operations with ALSA compatibility.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Audio Error Codes ─────────────────────────────────────────────────────

pub const AUDIO_OK: I32 = 0;
pub const AUDIO_ERR_NO_DEVICE: I32 = -1;
pub const AUDIO_ERR_INIT_FAILED: I32 = -2;
pub const AUDIO_ERR_OUT_OF_MEM: I32 = -3;
pub const AUDIO_ERR_NOT_SUPPORTED: I32 = -4;
pub const AUDIO_ERR_INVALID_PARAM: I32 = -5;
pub const AUDIO_ERR_BUSY: I32 = -6;
pub const AUDIO_ERR_IO: I32 = -7;

// ─── Audio Format Types ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioFormat {
    U8,
    S16LE,
    S16BE,
    S24LE,
    S24BE,
    S32LE,
    S32BE,
    Float32LE,
    Float32BE,
    Float64LE,
    Float64BE,
    IEC958SubframeLE,
    IEC958SubframeBE,
    MuLaw,
    ALaw,
}

// ─── Audio Sample Rate ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioSampleRate {
    Rate8000,
    Rate11025,
    Rate16000,
    Rate22050,
    Rate32000,
    Rate44100,
    Rate48000,
    Rate64000,
    Rate88200,
    Rate96000,
    Rate176400,
    Rate192000,
    Custom(U32),
}

impl AudioSampleRate {
    pub fn to_u32(&self) -> U32 {
        match self {
            AudioSampleRate::Rate8000 => 8000,
            AudioSampleRate::Rate11025 => 11025,
            AudioSampleRate::Rate16000 => 16000,
            AudioSampleRate::Rate22050 => 22050,
            AudioSampleRate::Rate32000 => 32000,
            AudioSampleRate::Rate44100 => 44100,
            AudioSampleRate::Rate48000 => 48000,
            AudioSampleRate::Rate64000 => 64000,
            AudioSampleRate::Rate88200 => 88200,
            AudioSampleRate::Rate96000 => 96000,
            AudioSampleRate::Rate176400 => 176400,
            AudioSampleRate::Rate192000 => 192000,
            AudioSampleRate::Custom(rate) => *rate,
        }
    }
}

// ─── Audio Channels ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioChannels {
    Mono,
    Stereo,
    Surround21,
    Surround40,
    Surround41,
    Surround50,
    Surround51,
    Surround61,
    Surround71,
    Custom(U8),
}

impl AudioChannels {
    pub fn to_u8(&self) -> U8 {
        match self {
            AudioChannels::Mono => 1,
            AudioChannels::Stereo => 2,
            AudioChannels::Surround21 => 3,
            AudioChannels::Surround40 => 4,
            AudioChannels::Surround41 => 5,
            AudioChannels::Surround50 => 5,
            AudioChannels::Surround51 => 6,
            AudioChannels::Surround61 => 7,
            AudioChannels::Surround71 => 8,
            AudioChannels::Custom(ch) => *ch,
        }
    }
}

// ─── Audio Stream Direction ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioStreamDirection {
    Playback,
    Capture,
    Loopback,
}

// ─── Audio Hardware Parameters ───────────────────────────────────────

#[repr(C)]
pub struct AudioHwParams {
    pub format: AudioFormat,
    pub sample_rate: AudioSampleRate,
    pub channels: AudioChannels,
    pub buffer_size: U32,
    pub period_size: U32,
    pub periods: U32,
}

impl AudioHwParams {
    pub const fn new() -> Self {
        AudioHwParams {
            format: AudioFormat::S16LE,
            sample_rate: AudioSampleRate::Rate48000,
            channels: AudioChannels::Stereo,
            buffer_size: 0,
            period_size: 0,
            periods: 0,
        }
    }
}

// ─── Audio Software Parameters ───────────────────────────────────────

#[repr(C)]
pub struct AudioSwParams {
    pub start_threshold: U32,
    pub stop_threshold: U32,
    pub silence_threshold: U32,
    pub silence_size: U32,
    pub avail_min: U32,
}

impl AudioSwParams {
    pub const fn new() -> Self {
        AudioSwParams {
            start_threshold: 0,
            stop_threshold: 0,
            silence_threshold: 0,
            silence_size: 0,
            avail_min: 0,
        }
    }
}

// ─── Audio Control Element ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioControlType {
    Boolean,
    Integer,
    Integer64,
    Enumerated,
    Bytes,
    IEC958,
}

#[repr(C)]
pub struct AudioControlInfo {
    pub id: U32,
    pub iface: U32,
    pub device: U32,
    pub subdevice: U32,
    pub name: [U8; 64],
    pub name_len: U8,
    pub index: U32,
    pub control_type: AudioControlType,
}

impl AudioControlInfo {
    pub const fn new() -> Self {
        AudioControlInfo {
            id: 0,
            iface: 0,
            device: 0,
            subdevice: 0,
            name: [0; 64],
            name_len: 0,
            index: 0,
            control_type: AudioControlType::Boolean,
        }
    }
}

// ─── Audio Device Trait ─────────────────────────────────────────────

/// Trait for audio device operations
pub trait AudioDevice {
    /// Initialize the audio device
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get device name
    fn get_device_name(&self) -> &'static str;
    
    /// Open audio stream
    fn open(&mut self, direction: AudioStreamDirection) -> I32;
    
    /// Close audio stream
    fn close(&mut self) -> I32;
    
    /// Set hardware parameters
    fn set_hw_params(&mut self, params: &AudioHwParams) -> I32;
    
    /// Get hardware parameters
    fn get_hw_params(&self, params: *mut AudioHwParams) -> I32;
    
    /// Set software parameters
    fn set_sw_params(&mut self, params: &AudioSwParams) -> I32;
    
    /// Get software parameters
    fn get_sw_params(&self, params: *mut AudioSwParams) -> I32;
    
    /// Prepare for playback/capture
    fn prepare(&mut self) -> I32;
    
    /// Start playback/capture
    fn start(&mut self) -> I32;
    
    /// Stop playback/capture
    fn stop(&mut self) -> I32;
    
    /// Pause playback/capture
    fn pause(&mut self, enable: bool) -> I32;
    
    /// Drop audio frames
    fn drop(&mut self) -> I32;
    
    /// Write audio data (playback)
    fn write(&mut self, buffer: *const U8, frames: U32) -> I32;
    
    /// Read audio data (capture)
    fn read(&mut self, buffer: *mut U8, frames: U32) -> I32;
    
    /// Get available frames
    fn avail(&self) -> U32;
    
    /// Get buffer size
    fn get_buffer_size(&self) -> U32;
    
    /// Get period size
    fn get_period_size(&self) -> U32;
    
    /// Get control info
    fn get_control_info(&self, control_id: U32, info: *mut AudioControlInfo) -> I32;
    
    /// Get control value
    fn get_control_value(&self, control_id: U32, value: *mut I32) -> I32;
    
    /// Set control value
    fn set_control_value(&mut self, control_id: U32, value: I32) -> I32;
    
    /// Reset the device
    fn reset(&mut self) -> I32;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> I32;
}

// ─── Audio Mixer Trait ─────────────────────────────────────────────

/// Trait for audio mixer operations
pub trait AudioMixer {
    /// Get number of mixer controls
    fn get_control_count(&self) -> U32;
    
    /// Get control info by index
    fn get_control_info_by_index(&self, index: U32, info: *mut AudioControlInfo) -> I32;
    
    /// Set mixer volume
    fn set_volume(&mut self, control_id: U32, left: I32, right: I32) -> I32;
    
    /// Get mixer volume
    fn get_volume(&self, control_id: U32, left: *mut I32, right: *mut I32) -> I32;
    
    /// Mute/unmute control
    fn set_mute(&mut self, control_id: U32, mute: bool) -> I32;
    
    /// Get mute status
    fn get_mute(&self, control_id: U32) -> bool;
}

// ─── Audio PCM Trait ───────────────────────────────────────────────

/// Trait for PCM (Pulse Code Modulation) operations
pub trait AudioPCM {
    /// Get PCM state
    fn get_state(&self) -> AudioPcmState;
    
    /// Get PCM delay
    fn get_delay(&self) -> U32;
    
    /// Get PCM position
    fn get_position(&self) -> U32;
    
    /// Reset PCM pointer
    fn reset_pointer(&mut self) -> I32;
}

// ─── Audio PCM State ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioPcmState {
    Open,
    Setup,
    Prepared,
    Running,
    XRun,
    Draining,
    Paused,
    Suspended,
    Disconnected,
}
