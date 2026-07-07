// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/audio/sigma_pipewire.rs — PipeWire Audio Stack
//
// Implements PipeWire-inspired audio stack for SigmaOS.
// Provides low-latency audio, sandboxed streams, and device management.
// Inspired by: PipeWire, PulseAudio, ALSA
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum number of audio devices.
const MAX_DEVICES: SigmaUsize = 32;
/// Maximum number of audio streams.
const MAX_STREAMS: SigmaUsize = 64;
/// Device name length.
const DEVICE_NAME_LEN: SigmaUsize = 64;
/// Stream name length.
const STREAM_NAME_LEN: SigmaUsize = 64;
/// Default sample rate.
const DEFAULT_SAMPLE_RATE: SigmaU32 = 48000;
/// Default buffer size.
const DEFAULT_BUFFER_SIZE: SigmaU32 = 1024;

// ── Audio Format ───────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AudioFormat {
    /// Signed 16-bit little-endian.
    S16Le = 0,
    /// Signed 32-bit little-endian.
    S32Le = 1,
    /// Float 32-bit little-endian.
    F32Le = 2,
    /// Signed 24-bit little-endian (packed).
    S24Le = 3,
}

// ── Stream Direction ───────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StreamDirection {
    /// Playback (output).
    Playback = 0,
    /// Capture (input).
    Capture = 1,
    /// Duplex (both).
    Duplex = 2,
}

// ── Audio Device ─────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioDevice {
    pub device_id: SigmaU32,
    pub name: [SigmaU8; DEVICE_NAME_LEN],
    pub card: SigmaU32,
    pub direction: StreamDirection,
    pub sample_rate: SigmaU32,
    pub channels: SigmaU32,
    pub format: AudioFormat,
    pub active: SigmaBool,
    pub default: SigmaBool,
    pub _pad: [SigmaU8; 7],
}

impl AudioDevice {
    pub const fn new() -> Self {
        Self {
            device_id: 0,
            name: [0u8; DEVICE_NAME_LEN],
            card: 0,
            direction: StreamDirection::Playback,
            sample_rate: DEFAULT_SAMPLE_RATE,
            channels: 2,
            format: AudioFormat::S16Le,
            active: false,
            default: false,
            _pad: [0u8; 7],
        }
    }
}

// ── Audio Stream ─────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioStream {
    pub stream_id: SigmaU32,
    pub name: [SigmaU8; STREAM_NAME_LEN],
    pub device_id: SigmaU32,
    pub direction: StreamDirection,
    pub sample_rate: SigmaU32,
    pub channels: SigmaU32,
    pub format: AudioFormat,
    pub buffer_size: SigmaU32,
    pub active: SigmaBool,
    pub sandboxed: SigmaBool,
    pub _pad: [SigmaU8; 7],
}

impl AudioStream {
    pub const fn new() -> Self {
        Self {
            stream_id: 0,
            name: [0u8; STREAM_NAME_LEN],
            device_id: 0,
            direction: StreamDirection::Playback,
            sample_rate: DEFAULT_SAMPLE_RATE,
            channels: 2,
            format: AudioFormat::S16Le,
            buffer_size: DEFAULT_BUFFER_SIZE,
            active: false,
            sandboxed: true,
            _pad: [0u8; 7],
        }
    }
}

// ── Audio Manager ─────────────────────────────────────────────────────────
pub struct AudioManager {
    devices: [AudioDevice; MAX_DEVICES],
    streams: [AudioStream; MAX_STREAMS],
    device_count: SigmaUsize,
    stream_count: SigmaUsize,
    next_device_id: SigmaU32,
    next_stream_id: SigmaU32,
    default_playback: SigmaU32,
    default_capture: SigmaU32,
    low_latency_mode: SigmaBool,
}

impl AudioManager {
    pub const fn new() -> Self {
        Self {
            devices: [AudioDevice::new(); MAX_DEVICES],
            streams: [AudioStream::new(); MAX_STREAMS],
            device_count: 0,
            stream_count: 0,
            next_device_id: 1,
            next_stream_id: 1,
            default_playback: 0,
            default_capture: 0,
            low_latency_mode: true,
        }
    }

    pub fn init(&mut self) {
        self.low_latency_mode = true;
        // Detect and register audio devices
        self.detect_devices();
    }

    fn copy_str(dst: &mut [SigmaU8], src: &[SigmaU8]) {
        let len = src.len().min(dst.len() - 1);
        let mut i = 0;
        while i < len { dst[i] = src[i]; i += 1; }
        dst[len] = 0;
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Detect and register audio devices.
    fn detect_devices(&mut self) {
        // In production: scan PCI/USB for audio devices
        // For now, register a default device
        self.register_device(b"Default Audio Device", 0, StreamDirection::Duplex, DEFAULT_SAMPLE_RATE, 2, AudioFormat::S16Le);
    }

    /// Register an audio device.
    pub fn register_device(
        &mut self,
        name: &[SigmaU8],
        card: SigmaU32,
        direction: StreamDirection,
        sample_rate: SigmaU32,
        channels: SigmaU32,
        format: AudioFormat,
    ) -> SigmaU32 {
        if self.device_count >= MAX_DEVICES {
            return 0;
        }

        let idx = self.device_count;
        let id = self.next_device_id;
        self.next_device_id += 1;

        self.devices[idx].device_id = id;
        Self::copy_str(&mut self.devices[idx].name, name);
        self.devices[idx].card = card;
        self.devices[idx].direction = direction;
        self.devices[idx].sample_rate = sample_rate;
        self.devices[idx].channels = channels;
        self.devices[idx].format = format;
        self.devices[idx].active = true;

        // Set as default if first of its type
        if direction == StreamDirection::Playback && self.default_playback == 0 {
            self.default_playback = id;
            self.devices[idx].default = true;
        } else if direction == StreamDirection::Capture && self.default_capture == 0 {
            self.default_capture = id;
            self.devices[idx].default = true;
        }

        self.device_count += 1;
        id
    }

    /// Create an audio stream.
    pub fn create_stream(
        &mut self,
        name: &[SigmaU8],
        device_id: SigmaU32,
        direction: StreamDirection,
        sample_rate: SigmaU32,
        channels: SigmaU32,
        format: AudioFormat,
        buffer_size: SigmaU32,
        sandboxed: SigmaBool,
    ) -> SigmaU32 {
        if self.stream_count >= MAX_STREAMS {
            return 0;
        }

        let idx = self.stream_count;
        let id = self.next_stream_id;
        self.next_stream_id += 1;

        self.streams[idx].stream_id = id;
        Self::copy_str(&mut self.streams[idx].name, name);
        self.streams[idx].device_id = device_id;
        self.streams[idx].direction = direction;
        self.streams[idx].sample_rate = sample_rate;
        self.streams[idx].channels = channels;
        self.streams[idx].format = format;
        self.streams[idx].buffer_size = buffer_size;
        self.streams[idx].sandboxed = sandboxed;
        self.streams[idx].active = true;

        self.stream_count += 1;
        id
    }

    /// Destroy an audio stream.
    pub fn destroy_stream(&mut self, stream_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.stream_count {
            if self.streams[i].stream_id == stream_id {
                self.streams[i] = AudioStream::new();
                self.stream_count -= 1;
                return 0;
            }
        }
        -1
    }

    /// Set default playback device.
    pub fn set_default_playback(&mut self, device_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.device_count {
            if self.devices[i].device_id == device_id && self.devices[i].direction == StreamDirection::Playback {
                // Clear old default
                for j in 0..self.device_count {
                    if self.devices[j].default && self.devices[j].direction == StreamDirection::Playback {
                        self.devices[j].default = false;
                    }
                }
                // Set new default
                self.devices[i].default = true;
                self.default_playback = device_id;
                return 0;
            }
        }
        -1
    }

    /// Set default capture device.
    pub fn set_default_capture(&mut self, device_id: SigmaU32) -> SigmaI32 {
        for i in 0..self.device_count {
            if self.devices[i].device_id == device_id && self.devices[i].direction == StreamDirection::Capture {
                // Clear old default
                for j in 0..self.device_count {
                    if self.devices[j].default && self.devices[j].direction == StreamDirection::Capture {
                        self.devices[j].default = false;
                    }
                }
                // Set new default
                self.devices[i].default = true;
                self.default_capture = device_id;
                return 0;
            }
        }
        -1
    }

    /// Enable/disable low latency mode.
    pub fn set_low_latency_mode(&mut self, enabled: SigmaBool) {
        self.low_latency_mode = enabled;
    }

    /// Get low latency mode status.
    pub fn low_latency_mode(&self) -> SigmaBool {
        self.low_latency_mode
    }

    /// List all devices.
    pub fn list_devices(&self, out: *mut AudioDevice, max: SigmaUsize) -> SigmaUsize {
        let mut written = 0;
        for i in 0..self.device_count {
            if written >= max { break; }
            unsafe { core::ptr::write(out.add(written), self.devices[i]); }
            written += 1;
        }
        written
    }

    /// List all streams.
    pub fn list_streams(&self, out: *mut AudioStream, max: SigmaUsize) -> SigmaUsize {
        let mut written = 0;
        for i in 0..self.stream_count {
            if written >= max { break; }
            unsafe { core::ptr::write(out.add(written), self.streams[i]); }
            written += 1;
        }
        written
    }

    /// Get default playback device ID.
    pub fn default_playback_device(&self) -> SigmaU32 {
        self.default_playback
    }

    /// Get default capture device ID.
    pub fn default_capture_device(&self) -> SigmaU32 {
        self.default_capture
    }
}

static mut G_AUDIO_MGR: AudioManager = AudioManager::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_init() {
    G_AUDIO_MGR.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_register_device(
    name: *const SigmaU8,
    name_len: SigmaUsize,
    card: SigmaU32,
    direction: SigmaU32,
    sample_rate: SigmaU32,
    channels: SigmaU32,
    format: SigmaU32,
) -> SigmaU32 {
    let n = core::slice::from_raw_parts(name, name_len.min(DEVICE_NAME_LEN));
    let dir = match direction {
        0 => StreamDirection::Playback,
        1 => StreamDirection::Capture,
        2 => StreamDirection::Duplex,
        _ => StreamDirection::Playback,
    };
    let fmt = match format {
        0 => AudioFormat::S16Le,
        1 => AudioFormat::S32Le,
        2 => AudioFormat::F32Le,
        3 => AudioFormat::S24Le,
        _ => AudioFormat::S16Le,
    };
    G_AUDIO_MGR.register_device(n, card, dir, sample_rate, channels, fmt)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_create_stream(
    name: *const SigmaU8,
    name_len: SigmaUsize,
    device_id: SigmaU32,
    direction: SigmaU32,
    sample_rate: SigmaU32,
    channels: SigmaU32,
    format: SigmaU32,
    buffer_size: SigmaU32,
    sandboxed: SigmaU32,
) -> SigmaU32 {
    let n = core::slice::from_raw_parts(name, name_len.min(STREAM_NAME_LEN));
    let dir = match direction {
        0 => StreamDirection::Playback,
        1 => StreamDirection::Capture,
        2 => StreamDirection::Duplex,
        _ => StreamDirection::Playback,
    };
    let fmt = match format {
        0 => AudioFormat::S16Le,
        1 => AudioFormat::S32Le,
        2 => AudioFormat::F32Le,
        3 => AudioFormat::S24Le,
        _ => AudioFormat::S16Le,
    };
    G_AUDIO_MGR.create_stream(n, device_id, dir, sample_rate, channels, fmt, buffer_size, sandboxed != 0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_destroy_stream(stream_id: SigmaU32) -> SigmaI32 {
    G_AUDIO_MGR.destroy_stream(stream_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_set_default_playback(device_id: SigmaU32) -> SigmaI32 {
    G_AUDIO_MGR.set_default_playback(device_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_set_default_capture(device_id: SigmaU32) -> SigmaI32 {
    G_AUDIO_MGR.set_default_capture(device_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_set_low_latency_mode(enabled: SigmaU32) {
    G_AUDIO_MGR.set_low_latency_mode(enabled != 0);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_low_latency_mode() -> SigmaU32 {
    if G_AUDIO_MGR.low_latency_mode() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_list_devices(
    out: *mut AudioDevice,
    max: SigmaU32,
) -> SigmaU32 {
    G_AUDIO_MGR.list_devices(out, max as SigmaUsize) as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_list_streams(
    out: *mut AudioStream,
    max: SigmaU32,
) -> SigmaU32 {
    G_AUDIO_MGR.list_streams(out, max as SigmaUsize) as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_default_playback() -> SigmaU32 {
    G_AUDIO_MGR.default_playback_device()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pipewire_default_capture() -> SigmaU32 {
    G_AUDIO_MGR.default_capture_device()
}
