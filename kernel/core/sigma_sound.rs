// SigmaOS — Sound Subsystem (PipeWire-inspired sovereign audio)
// Implements: audio graph, device abstraction, ring buffers, ALSA compat stubs
#![no_std]
#![allow(dead_code)]
use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};

pub const AUDIO_SAMPLE_RATE: u32 = 48000;
pub const AUDIO_CHANNELS:    u32 = 2;
pub const AUDIO_PERIOD_FRAMES: usize = 1024;
pub const AUDIO_RING_PERIODS: usize = 4;
pub const AUDIO_RING_SIZE: usize = AUDIO_PERIOD_FRAMES * AUDIO_RING_PERIODS;
pub const MAX_AUDIO_STREAMS: usize = 32;
pub const MAX_AUDIO_DEVICES: usize = 8;

// ─── Sample Formats ──────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SampleFormat { S16Le, S32Le, F32Le, U8 }

impl SampleFormat {
    pub fn bytes(&self) -> usize {
        match self { Self::U8 => 1, Self::S16Le => 2, Self::S32Le | Self::F32Le => 4 }
    }
}

// ─── Audio Device ────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct AudioDevice {
    pub id:       u8,
    pub name:     [u8; 32],
    pub channels: u8,
    pub rate:     u32,
    pub fmt:      SampleFormat,
    pub is_input: bool,
    pub active:   bool,
    pub hw_ptr:   AtomicU64,  // hardware write pointer
    pub sw_ptr:   AtomicU64,  // software read pointer
}

impl AudioDevice {
    pub const fn new(id: u8, is_input: bool) -> Self {
        AudioDevice {
            id, name: [0u8; 32], channels: AUDIO_CHANNELS as u8,
            rate: AUDIO_SAMPLE_RATE, fmt: SampleFormat::F32Le,
            is_input, active: false,
            hw_ptr: AtomicU64::new(0), sw_ptr: AtomicU64::new(0),
        }
    }
    pub fn frames_available(&self) -> u64 {
        let hw = self.hw_ptr.load(Ordering::Acquire);
        let sw = self.sw_ptr.load(Ordering::Acquire);
        if hw >= sw { hw - sw } else { 0 }
    }
}

// ─── Audio Ring Buffer ────────────────────────────────────────────────────────
pub struct AudioRingBuffer {
    pub buf:   [f32; AUDIO_RING_SIZE * 2], // stereo interleaved
    pub write: usize,
    pub read:  usize,
    pub level: usize,
}

impl AudioRingBuffer {
    pub const fn new() -> Self {
        AudioRingBuffer { buf: [0.0f32; AUDIO_RING_SIZE * 2], write: 0, read: 0, level: 0 }
    }
    pub fn push_frame(&mut self, l: f32, r: f32) -> bool {
        if self.level >= AUDIO_RING_SIZE { return false; }
        let i = self.write * 2;
        self.buf[i]     = l;
        self.buf[i + 1] = r;
        self.write = (self.write + 1) % AUDIO_RING_SIZE;
        self.level += 1;
        true
    }
    pub fn pop_frame(&mut self) -> Option<(f32, f32)> {
        if self.level == 0 { return None; }
        let i = self.read * 2;
        let (l, r) = (self.buf[i], self.buf[i + 1]);
        self.read = (self.read + 1) % AUDIO_RING_SIZE;
        self.level -= 1;
        Some((l, r))
    }
    pub fn fill_silence(&mut self, frames: usize) {
        for _ in 0..frames { self.push_frame(0.0, 0.0); }
    }
}

// ─── Audio Stream ────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum StreamState { Idle, Running, Paused, Error }

pub struct AudioStream {
    pub id:      u16,
    pub device:  u8,
    pub state:   StreamState,
    pub volume:  f32,     // 0.0–1.0
    pub muted:   bool,
    pub ring:    AudioRingBuffer,
    pub latency_ms: u32,
    pub xruns:   u32,
    pub frames_processed: u64,
}

impl AudioStream {
    pub const fn new(id: u16, device: u8) -> Self {
        AudioStream {
            id, device, state: StreamState::Idle,
            volume: 1.0, muted: false,
            ring: AudioRingBuffer::new(),
            latency_ms: 21, xruns: 0, frames_processed: 0,
        }
    }
    pub fn write_samples(&mut self, samples: &[f32]) -> usize {
        if self.state != StreamState::Running { return 0; }
        let nframes = samples.len() / 2;
        let mut written = 0;
        for i in 0..nframes {
            let l = samples[i * 2]     * if self.muted { 0.0 } else { self.volume };
            let r = samples[i * 2 + 1] * if self.muted { 0.0 } else { self.volume };
            if self.ring.push_frame(l, r) { written += 1; }
            else { self.xruns += 1; break; }
        }
        self.frames_processed += written as u64;
        written
    }
    pub fn read_samples(&mut self, out: &mut [f32]) -> usize {
        let nframes = out.len() / 2;
        let mut read = 0;
        for i in 0..nframes {
            match self.ring.pop_frame() {
                Some((l, r)) => {
                    out[i * 2]     = l;
                    out[i * 2 + 1] = r;
                    read += 1;
                }
                None => { out[i*2] = 0.0; out[i*2+1] = 0.0; }
            }
        }
        read
    }
}

// ─── Audio Mixer (simple summing mixer) ──────────────────────────────────────
pub struct AudioMixer {
    pub streams:   [AudioStream; MAX_AUDIO_STREAMS],
    pub devices:   [AudioDevice; MAX_AUDIO_DEVICES],
    pub n_streams: usize,
    pub n_devices: usize,
    pub master_vol: f32,
    pub master_mute: bool,
}

impl AudioMixer {
    pub const fn new() -> Self {
        const S: AudioStream = AudioStream::new(0, 0);
        const D: AudioDevice = AudioDevice::new(0, false);
        AudioMixer {
            streams: [S; MAX_AUDIO_STREAMS],
            devices: [D; MAX_AUDIO_DEVICES],
            n_streams: 0, n_devices: 0,
            master_vol: 1.0, master_mute: false,
        }
    }
    pub fn add_device(&mut self, mut dev: AudioDevice) -> Option<u8> {
        if self.n_devices >= MAX_AUDIO_DEVICES { return None; }
        dev.id = self.n_devices as u8;
        self.devices[self.n_devices] = dev;
        self.n_devices += 1;
        Some(dev.id)
    }
    pub fn open_stream(&mut self, device: u8) -> Option<u16> {
        if self.n_streams >= MAX_AUDIO_STREAMS { return None; }
        let id = self.n_streams as u16;
        self.streams[self.n_streams] = AudioStream::new(id, device);
        self.streams[self.n_streams].state = StreamState::Running;
        self.n_streams += 1;
        Some(id)
    }
    pub fn close_stream(&mut self, id: u16) {
        if let Some(s) = self.streams[..self.n_streams].iter_mut().find(|s| s.id == id) {
            s.state = StreamState::Idle;
        }
    }
    pub fn set_volume(&mut self, stream_id: u16, vol: f32) {
        if let Some(s) = self.streams[..self.n_streams].iter_mut().find(|s| s.id == stream_id) {
            s.volume = vol.clamp(0.0, 1.0);
        }
    }
    /// Mix all running streams into output buffer (period_frames × 2 channels).
    pub fn mix_to_output(&mut self, out: &mut [f32], period_frames: usize) {
        for o in out[..period_frames * 2].iter_mut() { *o = 0.0; }
        if self.master_mute { return; }
        let mv = self.master_vol;
        for s in &mut self.streams[..self.n_streams] {
            if s.state != StreamState::Running { continue; }
            for i in 0..period_frames {
                if let Some((l, r)) = s.ring.pop_frame() {
                    out[i * 2]     += l * mv;
                    out[i * 2 + 1] += r * mv;
                }
            }
        }
        // Soft clip
        for o in out[..period_frames * 2].iter_mut() {
            *o = o.clamp(-1.0, 1.0);
        }
    }
    pub fn set_master_volume(&mut self, vol: f32) { self.master_vol = vol.clamp(0.0, 1.0); }
    pub fn mute_master(&mut self, mute: bool) { self.master_mute = mute; }
    pub fn stream_latency_ms(&self, id: u16) -> u32 {
        self.streams[..self.n_streams].iter()
            .find(|s| s.id == id).map(|s| s.latency_ms).unwrap_or(0)
    }
    pub fn total_xruns(&self) -> u32 {
        self.streams[..self.n_streams].iter().map(|s| s.xruns).sum()
    }
}

// ─── HDA (Intel HD Audio) driver stub ────────────────────────────────────────
pub const HDA_MMIO_BASE: u64 = 0xFEB00000;
pub const HDA_GCAP: u32 = 0x00;
pub const HDA_CORB_BASE: u32 = 0x40;
pub const HDA_RIRB_BASE: u32 = 0x50;
pub const HDA_GCTL: u32 = 0x08;

pub fn hda_init(mmio_base: u64) {
    unsafe {
        let base = mmio_base as *mut u32;
        // Reset controller
        base.add((HDA_GCTL / 4) as usize).write_volatile(0);
        // Wait reset (simplified — real needs 100µs delay)
        for _ in 0..10000 { core::hint::spin_loop(); }
        // Bring out of reset
        base.add((HDA_GCTL / 4) as usize).write_volatile(1);
    }
}

pub fn hda_send_verb(mmio_base: u64, codec: u8, node: u8, verb: u32, param: u16) -> u32 {
    // Build 32-bit HDA verb: [31:28]=codec, [27:20]=node, [19:8]=verb, [7:0]=param
    let cmd = ((codec as u32) << 28) | ((node as u32) << 20) | ((verb as u32) << 8) | (param as u32 & 0xFF);
    // Would write to CORB and read from RIRB — stub returns 0
    let _ = (mmio_base, cmd);
    0
}
