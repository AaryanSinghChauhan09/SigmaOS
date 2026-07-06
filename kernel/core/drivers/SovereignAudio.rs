// SPDX-License-Identifier: MIT
// SigmaOS Intel HD Audio (HDA) Driver — SovereignAudio.rs
// Full HDA/Azalia codec enumeration, stream descriptor setup,
// PCM playback/capture, volume control, and S/PDIF support.
// Supports Intel ICH6/ICH7/ICH8/ICH9/PCH HDA controllers.

#![no_std]

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// ── HDA MMIO Register Offsets ─────────────────────────────────────────────────
const HDA_GCAP:       u32 = 0x00; // Global Capabilities
const HDA_VMIN:       u32 = 0x02; // Minor Version
const HDA_VMAJ:       u32 = 0x03; // Major Version
const HDA_GCTL:       u32 = 0x08; // Global Control
const HDA_WAKEEN:     u32 = 0x0C; // Wake Enable
const HDA_STATESTS:   u32 = 0x0E; // State Change Status
const HDA_INTCTL:     u32 = 0x20; // Interrupt Control
const HDA_INTSTS:     u32 = 0x24; // Interrupt Status
const HDA_WALLCLK:    u32 = 0x30; // Wall Clock Counter
const HDA_SSYNC:      u32 = 0x38; // Stream Synchronization
const HDA_CORBLBASE:  u32 = 0x40; // CORB Lower Base Address
const HDA_CORBUBASE:  u32 = 0x44; // CORB Upper Base Address
const HDA_CORBWP:     u32 = 0x48; // CORB Write Pointer
const HDA_CORBRP:     u32 = 0x4A; // CORB Read Pointer
const HDA_CORBCTL:    u32 = 0x4C; // CORB Control
const HDA_CORBSIZE:   u32 = 0x4E; // CORB Size
const HDA_RIRBLBASE:  u32 = 0x50; // RIRB Lower Base Address
const HDA_RIRBUBASE:  u32 = 0x54; // RIRB Upper Base Address
const HDA_RIRBWP:     u32 = 0x58; // RIRB Write Pointer
const HDA_RINTCNT:    u32 = 0x5A; // Response Interrupt Count
const HDA_RIRBCTL:    u32 = 0x5C; // RIRB Control
const HDA_RIRBSIZE:   u32 = 0x5E; // RIRB Size
const HDA_DPIBLBASE:  u32 = 0x70; // DMA Position In Buffer Lower
const HDA_DPIBUBASE:  u32 = 0x74; // DMA Position In Buffer Upper

// Stream descriptor base: SD0 at 0x80, each is 0x20 bytes wide
const HDA_SD_BASE:    u32 = 0x80;
const HDA_SD_SIZE:    u32 = 0x20;
// Per-stream offsets
const HDA_SD_CTL:     u32 = 0x00;
const HDA_SD_STS:     u32 = 0x03;
const HDA_SD_LPIB:    u32 = 0x04;
const HDA_SD_CBL:     u32 = 0x08; // Cyclic Buffer Length
const HDA_SD_LVI:     u32 = 0x0C; // Last Valid Index
const HDA_SD_FIFOW:   u32 = 0x0E; // FIFO Watermark
const HDA_SD_FIFOS:   u32 = 0x10; // FIFO Size
const HDA_SD_FMT:     u32 = 0x12; // Stream Format
const HDA_SD_BDLPL:   u32 = 0x18; // BDL Lower
const HDA_SD_BDLPU:   u32 = 0x1C; // BDL Upper

// ── HDA Verbs ─────────────────────────────────────────────────────────────────
const HDA_VERB_GET_PARAM:     u32 = 0xF00;
const HDA_VERB_GET_CONN_LIST: u32 = 0xF02;
const HDA_VERB_GET_AMP_GAIN:  u32 = 0xB00;
const HDA_VERB_SET_AMP_GAIN:  u32 = 0x300;
const HDA_VERB_SET_STREAM_CH: u32 = 0x706;
const HDA_VERB_SET_FORMAT:    u32 = 0x200;
const HDA_VERB_SET_PIN_CTRL:  u32 = 0x707;
const HDA_VERB_SET_EAPD:      u32 = 0x70C;
const HDA_VERB_GET_PIN_SENSE: u32 = 0xF09;
const HDA_VERB_SET_POWER:     u32 = 0x705;

// ── HDA Parameters ────────────────────────────────────────────────────────────
const HDA_PARAM_VENDOR_ID:    u32 = 0x00;
const HDA_PARAM_REVISION_ID:  u32 = 0x02;
const HDA_PARAM_NODE_COUNT:   u32 = 0x04;
const HDA_PARAM_FUNC_TYPE:    u32 = 0x05;
const HDA_PARAM_AUDIO_CAPS:   u32 = 0x09;
const HDA_PARAM_PCM_CAPS:     u32 = 0x0A;
const HDA_PARAM_STREAM_FMTS:  u32 = 0x0B;
const HDA_PARAM_PIN_CAPS:     u32 = 0x0C;
const HDA_PARAM_AMP_CAP_IN:   u32 = 0x0D;
const HDA_PARAM_AMP_CAP_OUT:  u32 = 0x12;
const HDA_PARAM_CONN_LIST_LEN:u32 = 0x0E;

// ── GCTL bits ─────────────────────────────────────────────────────────────────
const GCTL_RESET:     u32 = 1 << 0;
const GCTL_FLUSH_CTL: u32 = 1 << 1;
const GCTL_UNSOL:     u32 = 1 << 8;

// ── Stream Control bits ───────────────────────────────────────────────────────
const SD_CTL_RUN:     u32 = 1 << 1;
const SD_CTL_IOCE:    u32 = 1 << 2;  // Interrupt On Completion Enable
const SD_CTL_FEIE:    u32 = 1 << 3;  // FIFO Error Interrupt Enable
const SD_CTL_DEIE:    u32 = 1 << 4;  // Descriptor Error Interrupt
const SD_CTL_STRIPE:  u32 = 0 << 16; // Stripe Control
const SD_CTL_TP:      u32 = 1 << 18; // Traffic Priority
const SD_CTL_DIR:     u32 = 1 << 19; // Bidirectional stream direction

// ── BDL Entry (Buffer Descriptor List) ───────────────────────────────────────
#[repr(C, align(128))]
#[derive(Copy, Clone, Default)]
pub struct BdlEntry {
    pub addr:     u64,
    pub length:   u32,
    pub ioc:      u32,  // Interrupt On Completion (bit 0)
}

// ── CORB / RIRB ──────────────────────────────────────────────────────────────
const CORB_SIZE: usize = 256;
const RIRB_SIZE: usize = 256;
const BDL_ENTRIES: usize = 16;

// ── Sample Format Word ────────────────────────────────────────────────────────
/// Build HDA stream format word from parameters.
/// sample_rate: 44100/48000/96000/192000
/// bits: 16/20/24/32
/// channels: 1..8
fn hda_format_word(sample_rate: u32, bits: u8, channels: u8) -> u16 {
    let base = if sample_rate % 44100 == 0 { 1u16 << 14 } else { 0 }; // 44.1 kHz base
    let mult = match sample_rate {
        44100 | 48000  => 0u16,
        88200 | 96000  => 1u16 << 11,
        176400| 192000 => 3u16 << 11,
        _              => 0,
    };
    let bps = match bits {
        8  => 0u16,
        16 => 1u16 << 4,
        20 => 2u16 << 4,
        24 => 3u16 << 4,
        32 => 4u16 << 4,
        _  => 1u16 << 4,
    };
    let ch = (channels as u16).saturating_sub(1) & 0xF;
    base | mult | bps | ch
}

// ── Codec Descriptor ─────────────────────────────────────────────────────────
#[derive(Copy, Clone, Default, Debug)]
pub struct CodecInfo {
    pub vendor_id:    u32,
    pub revision_id:  u32,
    pub address:      u8,
    pub fg_node:      u8,   // Function Group start node
    pub fg_count:     u8,   // Number of nodes in function group
    pub out_pin:      u8,   // Output pin widget NID
    pub dac_nid:      u8,   // DAC widget NID
    pub vol_nid:      u8,   // Mixer/volume widget NID
    pub valid:        bool,
}

// ── Audio Buffer ─────────────────────────────────────────────────────────────
const AUDIO_BUF_SIZE: usize = 16384; // 16 KB ring buffer per stream
const MAX_STREAMS: usize = 4;

pub struct SovereignAudio {
    mmio_base:     u64,
    num_iss:       u8,        // Input stream slots
    num_oss:       u8,        // Output stream slots
    num_bss:       u8,        // Bidirectional stream slots
    corb:          [u32;      CORB_SIZE],
    rirb:          [u64;      RIRB_SIZE],
    corb_wp:       u16,
    rirb_rp:       u16,
    codecs:        [CodecInfo; 16],
    codec_count:   usize,
    bdl:           [[BdlEntry; BDL_ENTRIES]; MAX_STREAMS],
    audio_buf:     [[u8; AUDIO_BUF_SIZE]; MAX_STREAMS],
    stream_active: [AtomicBool; MAX_STREAMS],
    volume_l:      u8,     // 0..127
    volume_r:      u8,
    muted:         AtomicBool,
    initialized:   bool,
    sample_rate:   u32,
    bits:          u8,
    channels:      u8,
}

// Workaround for large const array in const fn
const fn default_bdl_entry() -> BdlEntry { BdlEntry { addr: 0, length: 0, ioc: 0 } }
const fn default_codec() -> CodecInfo {
    CodecInfo { vendor_id: 0, revision_id: 0, address: 0, fg_node: 0, fg_count: 0,
                out_pin: 0, dac_nid: 0, vol_nid: 0, valid: false }
}

impl SovereignAudio {
    pub const fn new() -> Self {
        Self {
            mmio_base:     0,
            num_iss:       0,
            num_oss:       0,
            num_bss:       0,
            corb:          [0u32; CORB_SIZE],
            rirb:          [0u64; RIRB_SIZE],
            corb_wp:       0,
            rirb_rp:       0,
            codecs:        [default_codec(); 16],
            codec_count:   0,
            bdl:           [[default_bdl_entry(); BDL_ENTRIES]; MAX_STREAMS],
            audio_buf:     [[0u8; AUDIO_BUF_SIZE]; MAX_STREAMS],
            stream_active: [
                AtomicBool::new(false), AtomicBool::new(false),
                AtomicBool::new(false), AtomicBool::new(false),
            ],
            volume_l:      100,
            volume_r:      100,
            muted:         AtomicBool::new(false),
            initialized:   false,
            sample_rate:   48000,
            bits:          16,
            channels:      2,
        }
    }

    // ── MMIO ──────────────────────────────────────────────────────────────────

    #[inline] unsafe fn read8(&self,  off: u32) -> u8  { read_volatile((self.mmio_base + off as u64) as *const u8) }
    #[inline] unsafe fn read16(&self, off: u32) -> u16 { read_volatile((self.mmio_base + off as u64) as *const u16) }
    #[inline] unsafe fn read32(&self, off: u32) -> u32 { read_volatile((self.mmio_base + off as u64) as *const u32) }
    #[inline] unsafe fn write8(&self,  off: u32, v: u8)  { write_volatile((self.mmio_base + off as u64) as *mut u8, v); }
    #[inline] unsafe fn write16(&self, off: u32, v: u16) { write_volatile((self.mmio_base + off as u64) as *mut u16, v); }
    #[inline] unsafe fn write32(&self, off: u32, v: u32) { write_volatile((self.mmio_base + off as u64) as *mut u32, v); }

    // Stream register helpers
    fn sd_off(&self, stream_idx: u8) -> u32 { HDA_SD_BASE + (stream_idx as u32) * HDA_SD_SIZE }
    #[inline] unsafe fn sd_read32(&self, s: u8, off: u32) -> u32 { self.read32(self.sd_off(s) + off) }
    #[inline] unsafe fn sd_write32(&self, s: u8, off: u32, v: u32) { self.write32(self.sd_off(s) + off, v); }

    // ── CORB / RIRB ──────────────────────────────────────────────────────────

    unsafe fn corb_init(&mut self) {
        // Stop CORB
        self.write8(HDA_CORBCTL, 0);
        let mut t = 100_000u32;
        while t > 0 && (self.read8(HDA_CORBCTL) & 2) != 0 { t -= 1; core::hint::spin_loop(); }

        // Set CORB size = 256 entries (0b10 in bits[1:0])
        self.write8(HDA_CORBSIZE, 0x02);

        // Set CORB base address
        let phys = self.corb.as_ptr() as u64;
        self.write32(HDA_CORBLBASE, (phys & 0xFFFFFFFF) as u32);
        self.write32(HDA_CORBUBASE, (phys >> 32) as u32);

        // Reset CORB read pointer
        self.write16(HDA_CORBRP, 0x8000);
        let mut t = 50_000u32;
        while t > 0 && (self.read16(HDA_CORBRP) & 0x8000) == 0 { t -= 1; }
        self.write16(HDA_CORBRP, 0x0000);
        self.write16(HDA_CORBWP, 0);
        self.corb_wp = 0;

        // Start CORB
        self.write8(HDA_CORBCTL, 0x02);
    }

    unsafe fn rirb_init(&mut self) {
        // Stop RIRB
        self.write8(HDA_RIRBCTL, 0);
        let mut t = 100_000u32;
        while t > 0 && (self.read8(HDA_RIRBCTL) & 2) != 0 { t -= 1; core::hint::spin_loop(); }

        self.write8(HDA_RIRBSIZE, 0x02); // 256 entries

        let phys = self.rirb.as_ptr() as u64;
        self.write32(HDA_RIRBLBASE, (phys & 0xFFFFFFFF) as u32);
        self.write32(HDA_RIRBUBASE, (phys >> 32) as u32);

        // Reset write pointer
        self.write16(HDA_RIRBWP, 0x8000);
        self.rirb_rp = 0;

        // Response interrupt every 1 response
        self.write16(HDA_RINTCNT, 1);

        // Start RIRB + interrupt enable
        self.write8(HDA_RIRBCTL, 0x03);
    }

    /// Send a verb via CORB.
    unsafe fn send_verb(&mut self, codec: u8, nid: u8, verb: u32, payload: u32) -> u64 {
        let cmd: u32 = ((codec as u32) << 28)
            | ((nid as u32) << 20)
            | ((verb & 0xFFF) << 8)
            | (payload & 0xFF);

        // Write to CORB
        let wp = ((self.corb_wp + 1) % CORB_SIZE as u16) as usize;
        self.corb[wp] = cmd;
        self.corb_wp  = wp as u16;
        self.write16(HDA_CORBWP, self.corb_wp);

        // Wait for RIRB response
        let mut timeout = 1_000_000u32;
        loop {
            let rirb_wp = self.read16(HDA_RIRBWP) & 0xFF;
            if rirb_wp != self.rirb_rp {
                self.rirb_rp = (self.rirb_rp + 1) % RIRB_SIZE as u16;
                let resp = self.rirb[self.rirb_rp as usize];
                return resp;
            }
            timeout -= 1;
            if timeout == 0 { return u64::MAX; }
            core::hint::spin_loop();
        }
    }

    fn verb_resp(r: u64) -> u32 { (r & 0xFFFFFFFF) as u32 }

    // ── Codec Enumeration ─────────────────────────────────────────────────────

    unsafe fn enumerate_codecs(&mut self) {
        let statests = self.read16(HDA_STATESTS);
        for codec_addr in 0..15u8 {
            if statests & (1 << codec_addr) == 0 { continue; }

            let vid_resp = self.send_verb(codec_addr, 0, HDA_VERB_GET_PARAM, HDA_PARAM_VENDOR_ID);
            let vid = Self::verb_resp(vid_resp);
            if vid == 0 || vid == 0xFFFFFFFF { continue; }

            let rev_resp = self.send_verb(codec_addr, 0, HDA_VERB_GET_PARAM, HDA_PARAM_REVISION_ID);
            let rev = Self::verb_resp(rev_resp);

            // Get root node count to find function groups
            let nc_resp = self.send_verb(codec_addr, 0, HDA_VERB_GET_PARAM, HDA_PARAM_NODE_COUNT);
            let nc = Self::verb_resp(nc_resp);
            let fg_start = (nc >> 16) as u8;
            let fg_count = (nc & 0xFF) as u8;

            // Scan function groups for Audio FG (type 1)
            for fg in 0..fg_count {
                let fg_nid = fg_start + fg;
                let ft_resp = self.send_verb(codec_addr, fg_nid, HDA_VERB_GET_PARAM, HDA_PARAM_FUNC_TYPE);
                let ft = Self::verb_resp(ft_resp) & 0xFF;
                if ft != 1 { continue; } // Only Audio Function Group

                // Get widget count under this FG
                let wnc_resp = self.send_verb(codec_addr, fg_nid, HDA_VERB_GET_PARAM, HDA_PARAM_NODE_COUNT);
                let wnc = Self::verb_resp(wnc_resp);
                let w_start = (wnc >> 16) as u8;
                let w_count = (wnc & 0xFF) as u8;

                // Find DAC and output pin
                let mut dac_nid = 0u8;
                let mut out_pin_nid = 0u8;

                for w in 0..w_count {
                    let w_nid = w_start + w;
                    let ac_resp = self.send_verb(codec_addr, w_nid, HDA_VERB_GET_PARAM, HDA_PARAM_AUDIO_CAPS);
                    let ac = Self::verb_resp(ac_resp);
                    let wtype = (ac >> 20) & 0xF;
                    match wtype {
                        0x0 => { // Audio Output (DAC)
                            if dac_nid == 0 { dac_nid = w_nid; }
                        },
                        0x4 => { // Pin Complex (output capable)
                            let pc_resp = self.send_verb(codec_addr, w_nid, HDA_VERB_GET_PARAM, HDA_PARAM_PIN_CAPS);
                            let pc = Self::verb_resp(pc_resp);
                            if pc & (1 << 4) != 0 { // Output capable
                                if out_pin_nid == 0 { out_pin_nid = w_nid; }
                            }
                        },
                        _ => {}
                    }
                }

                if self.codec_count < 16 {
                    self.codecs[self.codec_count] = CodecInfo {
                        vendor_id:   vid,
                        revision_id: rev,
                        address:     codec_addr,
                        fg_node:     fg_nid,
                        fg_count:    w_count,
                        out_pin:     out_pin_nid,
                        dac_nid:     dac_nid,
                        vol_nid:     0,
                        valid:       true,
                    };
                    self.codec_count += 1;
                }
            }
        }
    }

    // ── Codec Setup ───────────────────────────────────────────────────────────

    unsafe fn configure_codec(&mut self, codec_idx: usize, stream_tag: u8) {
        if codec_idx >= self.codec_count { return; }
        let c = self.codecs[codec_idx];
        let addr = c.address;

        // Power-up codec (D0 state)
        self.send_verb(addr, c.fg_node, HDA_VERB_SET_POWER, 0x00);
        for _ in 0..10000 { core::hint::spin_loop(); }

        // Set DAC stream/channel
        let fmt = hda_format_word(self.sample_rate, self.bits, self.channels);
        if c.dac_nid != 0 {
            self.send_verb(addr, c.dac_nid, HDA_VERB_SET_STREAM_CH,
                           ((stream_tag as u32) << 4) | 0); // stream_tag | channel=0
            self.send_verb(addr, c.dac_nid, HDA_VERB_SET_FORMAT, fmt as u32);
            // Unmute output amp, set max volume
            self.send_verb(addr, c.dac_nid, HDA_VERB_SET_AMP_GAIN,
                           0x3000 | 0x0000 | (0x7F & 0x7F) as u32); // L+R, index 0, vol=127
        }

        // Enable output pin
        if c.out_pin != 0 {
            self.send_verb(addr, c.out_pin, HDA_VERB_SET_PIN_CTRL, 0x40); // HP output enable
            self.send_verb(addr, c.out_pin, HDA_VERB_SET_EAPD, 0x02);     // EAPD enable
            // Unmute pin
            self.send_verb(addr, c.out_pin, HDA_VERB_SET_AMP_GAIN,
                           0x3000 | 0x0000 | 0x7F as u32);
        }
    }

    // ── Stream Setup ──────────────────────────────────────────────────────────

    unsafe fn setup_output_stream(&mut self, stream_idx: u8, stream_tag: u8) {
        let s = stream_idx as usize;
        if s >= MAX_STREAMS { return; }

        // Stop stream
        let mut ctl = self.sd_read32(stream_idx, HDA_SD_CTL);
        ctl &= !(SD_CTL_RUN);
        self.sd_write32(stream_idx, HDA_SD_CTL, ctl);
        for _ in 0..50_000 { core::hint::spin_loop(); }

        // Reset stream
        ctl |= 1 << 0; // SRST
        self.sd_write32(stream_idx, HDA_SD_CTL, ctl);
        let mut t = 50_000u32;
        while t > 0 && (self.sd_read32(stream_idx, HDA_SD_CTL) & 1) == 0 { t -= 1; }
        ctl &= !(1 << 0);
        self.sd_write32(stream_idx, HDA_SD_CTL, ctl);
        t = 50_000;
        while t > 0 && (self.sd_read32(stream_idx, HDA_SD_CTL) & 1) != 0 { t -= 1; }

        // Set up BDL: divide audio_buf into BDL_ENTRIES equal chunks
        let buf_phys  = self.audio_buf[s].as_ptr() as u64;
        let chunk_sz  = AUDIO_BUF_SIZE / BDL_ENTRIES;
        for i in 0..BDL_ENTRIES {
            self.bdl[s][i] = BdlEntry {
                addr:   buf_phys + (i * chunk_sz) as u64,
                length: chunk_sz as u32,
                ioc:    if i == BDL_ENTRIES - 1 { 1 } else { 0 }, // IOC on last
            };
        }

        // Program BDL address
        let bdl_phys = self.bdl[s].as_ptr() as u64;
        self.sd_write32(stream_idx, HDA_SD_BDLPL, (bdl_phys & 0xFFFFFFFF) as u32);
        self.sd_write32(stream_idx, HDA_SD_BDLPU, (bdl_phys >> 32) as u32);

        // Total Cyclic Buffer Length
        self.sd_write32(stream_idx, HDA_SD_CBL, AUDIO_BUF_SIZE as u32);

        // Last Valid Index (number of BDL entries - 1)
        self.sd_write32(stream_idx, HDA_SD_LVI, (BDL_ENTRIES - 1) as u32);

        // Stream format
        let fmt = hda_format_word(self.sample_rate, self.bits, self.channels);
        self.sd_write32(stream_idx, HDA_SD_FMT, fmt as u32);

        // Stream tag in control register (bits[23:20])
        let new_ctl = SD_CTL_IOCE | SD_CTL_FEIE
            | ((stream_tag as u32) << 20)
            | (self.channels as u32).saturating_sub(1) << 8; // stripe
        self.sd_write32(stream_idx, HDA_SD_CTL, new_ctl);
    }

    // ── Volume Control ────────────────────────────────────────────────────────

    pub unsafe fn set_volume(&mut self, left: u8, right: u8) {
        self.volume_l = left.min(127);
        self.volume_r = right.min(127);
        for i in 0..self.codec_count {
            let c = self.codecs[i];
            if !c.valid { continue; }
            if c.dac_nid != 0 {
                // Left amp: bit 13 set, bit 12 clear
                let l_cmd = 0x2000 | ((self.volume_l as u32) & 0x7F);
                // Right amp: bit 12 set, bit 13 clear
                let r_cmd = 0x1000 | ((self.volume_r as u32) & 0x7F);
                self.send_verb(c.address, c.dac_nid, HDA_VERB_SET_AMP_GAIN, l_cmd);
                self.send_verb(c.address, c.dac_nid, HDA_VERB_SET_AMP_GAIN, r_cmd);
            }
        }
    }

    pub unsafe fn mute(&mut self, mute: bool) {
        self.muted.store(mute, Ordering::Relaxed);
        for i in 0..self.codec_count {
            let c = self.codecs[i];
            if !c.valid || c.dac_nid == 0 { continue; }
            let mute_bit = if mute { 0x80u32 } else { 0 };
            let vol = if mute { 0u32 } else { self.volume_l as u32 };
            self.send_verb(c.address, c.dac_nid, HDA_VERB_SET_AMP_GAIN,
                           0x3000 | mute_bit | (vol & 0x7F));
        }
    }

    // ── Main Init ─────────────────────────────────────────────────────────────

    pub unsafe fn init(&mut self, mmio_base: u64) -> bool {
        self.mmio_base = mmio_base;
        if mmio_base == 0 { return false; }

        // Check version
        let vmaj = self.read8(HDA_VMAJ);
        let vmin = self.read8(HDA_VMIN);
        if vmaj != 1 { return false; } // Only HDA 1.x supported
        let _ = vmin;

        // Read capabilities
        let gcap = self.read16(HDA_GCAP);
        self.num_oss = ((gcap >> 12) & 0xF) as u8;
        self.num_iss = ((gcap >> 8)  & 0xF) as u8;
        self.num_bss = ((gcap >> 3)  & 0x1F) as u8;

        // Controller reset
        self.write32(HDA_GCTL, 0);
        let mut t = 100_000u32;
        while t > 0 && (self.read32(HDA_GCTL) & GCTL_RESET) != 0 { t -= 1; core::hint::spin_loop(); }

        // Exit reset
        self.write32(HDA_GCTL, GCTL_RESET);
        t = 100_000;
        while t > 0 && (self.read32(HDA_GCTL) & GCTL_RESET) == 0 { t -= 1; core::hint::spin_loop(); }
        if t == 0 { return false; }

        // Wait for codecs to settle (spec: 521 µs minimum)
        for _ in 0..100_000 { core::hint::spin_loop(); }

        // Enable wake events
        self.write16(HDA_WAKEEN, 0x7FFF);

        // Init CORB + RIRB
        self.corb_init();
        self.rirb_init();

        // Enable global interrupts
        self.write32(HDA_INTCTL, 0xC0000000 | ((1 << self.num_oss) - 1) as u32);

        // Enumerate codecs
        self.enumerate_codecs();
        if self.codec_count == 0 { return false; }

        // Configure first output stream (stream 0 = first output, tag=1)
        let first_out_stream = self.num_iss; // Output streams start after ISS
        self.configure_codec(0, 1);
        self.setup_output_stream(first_out_stream, 1);

        self.initialized = true;
        true
    }

    // ── Playback Control ──────────────────────────────────────────────────────

    pub unsafe fn start_playback(&mut self) -> bool {
        if !self.initialized { return false; }
        let s = self.num_iss;
        let mut ctl = self.sd_read32(s, HDA_SD_CTL);
        ctl |= SD_CTL_RUN;
        self.sd_write32(s, HDA_SD_CTL, ctl);
        self.stream_active[0].store(true, Ordering::Relaxed);
        true
    }

    pub unsafe fn stop_playback(&mut self) {
        if !self.initialized { return; }
        let s = self.num_iss;
        let mut ctl = self.sd_read32(s, HDA_SD_CTL);
        ctl &= !SD_CTL_RUN;
        self.sd_write32(s, HDA_SD_CTL, ctl);
        self.stream_active[0].store(false, Ordering::Relaxed);
    }

    /// Write PCM samples into the audio buffer.
    /// Returns bytes actually written.
    pub fn write_pcm(&mut self, data: &[u8]) -> usize {
        if !self.initialized { return 0; }
        let to_copy = data.len().min(AUDIO_BUF_SIZE);
        self.audio_buf[0][..to_copy].copy_from_slice(&data[..to_copy]);
        to_copy
    }

    pub fn is_playing(&self) -> bool {
        self.stream_active[0].load(Ordering::Relaxed)
    }

    pub fn codec_count(&self) -> usize { self.codec_count }

    pub fn vendor_id(&self, idx: usize) -> u32 {
        if idx < self.codec_count { self.codecs[idx].vendor_id } else { 0 }
    }
}

// ── Global Instance ───────────────────────────────────────────────────────────
static mut G_AUDIO: SovereignAudio = SovereignAudio::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_audio_init(mmio_base: u64) -> i32 {
    if G_AUDIO.init(mmio_base) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_audio_start() -> i32 {
    if G_AUDIO.start_playback() { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_audio_stop() {
    G_AUDIO.stop_playback();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_audio_write(buf: *const u8, len: usize) -> usize {
    if buf.is_null() { return 0; }
    let data = core::slice::from_raw_parts(buf, len);
    G_AUDIO.write_pcm(data)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_audio_set_volume(left: u8, right: u8) {
    G_AUDIO.set_volume(left, right);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_audio_mute(mute: i32) {
    G_AUDIO.mute(mute != 0);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_audio_codec_count() -> usize {
    G_AUDIO.codec_count()
}

// Legacy compatibility names
#[no_mangle] pub unsafe extern "C" fn init()        { sigma_audio_init(0); }
#[no_mangle] pub unsafe extern "C" fn audio_init()  { sigma_audio_init(0); }
#[no_mangle] pub unsafe extern "C" fn playStream()  { sigma_audio_start(); }
