// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// drivers/audio/sigma_hda.rs — Intel HDA Audio Controller Driver
// Language: Rust #![no_std]
// Pattern: OOP via HdaController struct implementing AudioDevice trait

#![no_std]

// ── HDA Register Offsets ──────────────────────────────────────────────────────
const REG_GCAP:    usize = 0x00;
const REG_GCTL:    usize = 0x08;
const REG_WAKEEN:  usize = 0x0C;
const REG_STATESTS: usize = 0x0E;
const REG_INTCTL:  usize = 0x20;
const REG_INTSTS:  usize = 0x24;
const REG_CORBLBASE: usize = 0x40;
const REG_CORBUBASE: usize = 0x44;
const REG_CORBWP:  usize = 0x48;
const REG_CORBRP:  usize = 0x4A;
const REG_CORBCTL: usize = 0x4C;
const REG_RIRBLBASE: usize = 0x50;
const REG_RIRBUBASE: usize = 0x54;
const REG_RIRBWP:  usize = 0x58;
const REG_RINTCNT: usize = 0x5A;
const REG_RIRBCTL: usize = 0x5C;
const REG_DPLBASE: usize = 0x70;
const REG_DPUBASE: usize = 0x74;

const GCTL_CRST:  u32 = 1 << 0;
const GCTL_FCNTRL: u32 = 1 << 1;
const CORBCTL_MEIE: u8 = 1 << 0;
const CORBCTL_RUN:  u8 = 1 << 1;
const RIRBCTL_RINTCTL: u8 = 1 << 0;
const RIRBCTL_RIRBDMAEN: u8 = 1 << 1;

// ── HDA Verb Definitions ──────────────────────────────────────────────────────
const VERB_GET_PARAM: u32 = 0xF0000;
const PARAM_VENDOR:   u32 = 0x00;
const PARAM_NODE_CNT: u32 = 0x04;
const PARAM_AUDIO_WIDGET: u32 = 0x09;
const VERB_SET_POWER:  u32 = 0x70500;
const VERB_SET_VOLUME: u32 = 0x30000; // amplifier gain/mute
const VERB_SET_FORMAT: u32 = 0x20000;
const VERB_SET_STREAM: u32 = 0x70600;

// ── Ring Sizes ────────────────────────────────────────────────────────────────
const CORB_ENTRIES: usize = 256;
const RIRB_ENTRIES: usize = 256;

// ── RIRB Entry (64-bit) ───────────────────────────────────────────────────────
#[repr(C)]
struct RirbEntry { response: u32, ex: u32 }

// ── Audio Device Trait ────────────────────────────────────────────────────────
pub trait AudioDevice: Send + Sync {
    fn name(&self) -> &'static str;
    fn sample_rate(&self) -> u32;
    fn channels(&self) -> u8;
    fn bit_depth(&self) -> u8;
    fn play(&mut self, buf: &[u8]) -> bool;
    fn stop(&mut self);
    fn set_volume(&mut self, pct: u8);
}

// ── HDA Controller ────────────────────────────────────────────────────────────
pub struct HdaController {
    mmio:        usize,
    corb:        [u32; CORB_ENTRIES],
    rirb:        [RirbEntry; RIRB_ENTRIES],
    corbwp:      u16,
    rirbwp:      u16,
    codecs:      [u8; 15],   // codec addresses found during enumeration
    n_codecs:    usize,
    sample_rate: u32,
    channels:    u8,
    bit_depth:   u8,
    volume_pct:  u8,
    playing:     bool,
}

impl HdaController {
    pub fn new(mmio: usize) -> Self {
        Self {
            mmio,
            corb:    [0u32; CORB_ENTRIES],
            rirb:    unsafe { core::mem::zeroed() },
            corbwp:  0, rirbwp: 0,
            codecs:  [0u8; 15], n_codecs: 0,
            sample_rate: 48000, channels: 2, bit_depth: 16,
            volume_pct: 75, playing: false,
        }
    }

    pub fn probe(vendor: u16, device: u16) -> bool {
        // Intel HDA: 8086:2668, 8086:293E, 8086:3A3E, 8086:3B56, 8086:1C20 …
        vendor == 0x8086 && matches!(device,
            0x2668 | 0x293E | 0x3A3E | 0x3B56 | 0x1C20 | 0x8C20 | 0xA170)
    }

    pub fn init(&mut self) -> bool {
        // 1. Reset controller
        self.write32(REG_GCTL, 0);
        for _ in 0..10_000 {
            if self.read32(REG_GCTL) & GCTL_CRST == 0 { break; }
        }
        self.write32(REG_GCTL, GCTL_CRST);
        for _ in 0..10_000 {
            if self.read32(REG_GCTL) & GCTL_CRST != 0 { break; }
        }

        // 2. Set up CORB
        let corb_phys = self.corb.as_ptr() as u64;
        self.write32(REG_CORBLBASE, (corb_phys & 0xFFFF_FFFF) as u32);
        self.write32(REG_CORBUBASE, (corb_phys >> 32) as u32);
        // CORB size = 256 entries (0b10 in bits 1:0 of CORBSIZE at 0x4E)
        let corbsize_reg = self.mmio + 0x4E;
        unsafe { (corbsize_reg as *mut volatile u8).write_volatile(0x02); }
        // Start CORB DMA
        unsafe { (self.mmio as *mut u8).add(REG_CORBCTL).write_volatile(CORBCTL_RUN); }

        // 3. Set up RIRB
        let rirb_phys = self.rirb.as_ptr() as u64;
        self.write32(REG_RIRBLBASE, (rirb_phys & 0xFFFF_FFFF) as u32);
        self.write32(REG_RIRBUBASE, (rirb_phys >> 32) as u32);
        let rirbsize_reg = self.mmio + 0x5E;
        unsafe { (rirbsize_reg as *mut volatile u8).write_volatile(0x02); }
        unsafe { (self.mmio as *mut u8).add(REG_RIRBCTL)
            .write_volatile(RIRBCTL_RIRBDMAEN | RIRBCTL_RINTCTL); }

        // 4. Enable interrupts
        self.write32(REG_INTCTL, 0x8000_00FF);

        // 5. Wait for codec wakup
        for _ in 0..100_000 {
            if self.read16(REG_STATESTS) != 0 { break; }
        }
        let statests = self.read16(REG_STATESTS);
        for i in 0..15u8 {
            if statests & (1 << i) != 0 {
                self.codecs[self.n_codecs] = i;
                self.n_codecs += 1;
            }
        }
        self.n_codecs > 0
    }

    /// Send a verb to a codec and return the response
    fn send_verb(&mut self, codec: u8, nid: u8, verb: u32, payload: u32) -> u32 {
        let cmd = ((codec as u32) << 28) | ((nid as u32) << 20) | verb | payload;
        let wp = (self.corbwp as usize + 1) % CORB_ENTRIES;
        self.corb[wp] = cmd;
        self.corbwp = wp as u16;
        // Update CORB write pointer
        unsafe { (self.mmio as *mut u16).add(REG_CORBWP / 2).write_volatile(self.corbwp); }
        // Spin-wait for RIRB response
        for _ in 0..100_000 {
            let rp = self.read16(REG_RIRBWP) as usize;
            if rp != self.rirbwp as usize {
                self.rirbwp = rp as u16;
                return self.rirb[rp % RIRB_ENTRIES].response;
            }
        }
        0
    }

    fn read32(&self, off: usize) -> u32 {
        unsafe { ((self.mmio + off) as *const volatile u32).read_volatile() }
    }
    fn write32(&self, off: usize, v: u32) {
        unsafe { ((self.mmio + off) as *mut volatile u32).write_volatile(v); }
    }
    fn read16(&self, off: usize) -> u16 {
        unsafe { ((self.mmio + off) as *const volatile u16).read_volatile() }
    }
}

impl AudioDevice for HdaController {
    fn name(&self)        -> &'static str { "sigma-hda" }
    fn sample_rate(&self) -> u32          { self.sample_rate }
    fn channels(&self)    -> u8           { self.channels }
    fn bit_depth(&self)   -> u8           { self.bit_depth }

    fn play(&mut self, _buf: &[u8]) -> bool {
        // TODO: set up BDL (Buffer Descriptor List) and start stream
        self.playing = true;
        true
    }

    fn stop(&mut self) { self.playing = false; }

    fn set_volume(&mut self, pct: u8) {
        self.volume_pct = pct;
        if self.n_codecs == 0 { return; }
        let codec = self.codecs[0];
        // Set DAC output amplifier (nid=2 typical for first output widget)
        let gain = ((pct as u32) * 127 / 100) & 0x7F;
        let amp_verb = 0x70000 | (1 << 15) | (1 << 13) | (1 << 12) | gain;
        self.send_verb(codec, 2, 0x30000, amp_verb);
    }
}
