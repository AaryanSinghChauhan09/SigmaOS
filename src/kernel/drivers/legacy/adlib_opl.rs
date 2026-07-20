use crate::kernel::subsystems::registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority,
};
/// SigmaOS Legacy Driver — AdLib/OPL2/OPL3 FM Synthesis + Sound Blaster ISA
/// Absorbs Linux ALSA snd-opl3 + snd-sb* driver families
/// AdLib OPL2 (YM3812), OPL3 (YMF262), SB 1.0/2.0/Pro/16/AWE32
use core::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;

/// OPL register space
pub const OPL_BASE_ADDR: u16 = 0x388;
pub const OPL2_NUM_OPERATORS: usize = 18;
pub const OPL3_NUM_OPERATORS: usize = 36;

/// OPL chip version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OplVersion {
    Opl2,
    Opl3,
}

/// OPL operator (slot) register state
#[derive(Debug, Clone, Default)]
pub struct OplOperator {
    pub am: bool,        // Amplitude modulation
    pub vib: bool,       // Vibrato
    pub eg_type: bool,   // Envelope generator type
    pub ksr: bool,       // Key scale rate
    pub mult: u8,        // Frequency multiplier (0-15)
    pub ksl: u8,         // Key scale level (0-3)
    pub total_level: u8, // Total level / attenuation (0-63)
    pub attack: u8,      // Attack rate (0-15)
    pub decay: u8,       // Decay rate (0-15)
    pub sustain: u8,     // Sustain level (0-15)
    pub release: u8,     // Release rate (0-15)
    pub wave_select: u8, // Waveform select (0-7 for OPL3)
}

/// OPL channel
#[derive(Debug, Clone)]
pub struct OplChannel {
    pub num: u8,
    pub frequency: u16, // F-Num (0-1023)
    pub octave: u8,     // Block (0-7)
    pub key_on: bool,
    pub feedback: u8,   // Feedback modulation (0-7)
    pub connection: u8, // 0=FM, 1=Additive
    pub operators: [OplOperator; 2],
}

impl OplChannel {
    pub fn new(num: u8) -> Self {
        OplChannel {
            num,
            frequency: 0,
            octave: 0,
            key_on: false,
            feedback: 0,
            connection: 0,
            operators: [OplOperator::default(), OplOperator::default()],
        }
    }

    /// Note-on: set F-Num and octave, enable key
    pub fn note_on(&mut self, fnum: u16, octave: u8) {
        self.frequency = fnum & 0x3FF;
        self.octave = octave & 0x07;
        self.key_on = true;
    }

    pub fn note_off(&mut self) {
        self.key_on = false;
    }
}

/// OPL FM Synthesizer
pub struct OplSynth {
    pub version: OplVersion,
    pub channels: Vec<OplChannel>,
    pub base_addr: u16,
    note_count: AtomicUsize,
    initialized: bool,
}

impl OplSynth {
    pub fn opl2() -> Self {
        Self::new(OplVersion::Opl2)
    }
    pub fn opl3() -> Self {
        Self::new(OplVersion::Opl3)
    }

    pub fn new(version: OplVersion) -> Self {
        let ch_count = match version {
            OplVersion::Opl2 => 9,
            OplVersion::Opl3 => 18,
        };
        OplSynth {
            version,
            channels: (0..ch_count).map(OplChannel::new).collect(),
            base_addr: OPL_BASE_ADDR,
            note_count: AtomicUsize::new(0),
            initialized: false,
        }
    }

    pub fn note_on(&mut self, channel: u8, fnum: u16, octave: u8) -> Result<(), &'static str> {
        let ch = self
            .channels
            .get_mut(channel as usize)
            .ok_or("OPL: invalid channel")?;
        ch.note_on(fnum, octave);
        self.note_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn note_off(&mut self, channel: u8) -> Result<(), &'static str> {
        let ch = self
            .channels
            .get_mut(channel as usize)
            .ok_or("OPL: invalid channel")?;
        ch.note_off();
        Ok(())
    }

    pub fn note_count(&self) -> usize {
        self.note_count.load(Ordering::Relaxed)
    }
    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }
}

// ── Sound Blaster ISA driver ──────────────────────────────────────────────

/// Sound Blaster variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbVariant {
    Sb10,
    Sb20,
    SbPro,
    Sb16,
    SbAwE32,
}

impl SbVariant {
    pub fn dsp_version(&self) -> (u8, u8) {
        match self {
            SbVariant::Sb10 => (1, 5),
            SbVariant::Sb20 => (2, 1),
            SbVariant::SbPro => (3, 1),
            SbVariant::Sb16 => (4, 5),
            SbVariant::SbAwE32 => (4, 11),
        }
    }

    pub fn max_sample_rate(&self) -> u32 {
        match self {
            SbVariant::Sb10 | SbVariant::Sb20 => 22050,
            SbVariant::SbPro => 44100,
            SbVariant::Sb16 | SbVariant::SbAwE32 => 48000,
        }
    }

    pub fn supports_16bit(&self) -> bool {
        matches!(self, SbVariant::Sb16 | SbVariant::SbAwE32)
    }
}

/// Sound Blaster DSP state
pub struct SoundBlasterDriver {
    pub variant: SbVariant,
    pub base_io: u16,
    pub irq: u8,
    pub dma8: u8,
    pub dma16: Option<u8>,
    pub opl: OplSynth,
    pub sample_rate: u32,
    pub bits: u8,
    pub channels: u8,
    samples_played: AtomicUsize,
    initialized: bool,
}

impl SoundBlasterDriver {
    /// Typical ISA factory defaults
    pub fn new(variant: SbVariant) -> Self {
        let dma16 = if variant.supports_16bit() {
            Some(5)
        } else {
            None
        };
        SoundBlasterDriver {
            variant,
            base_io: 0x220,
            irq: 5,
            dma8: 1,
            dma16,
            opl: OplSynth::opl2(),
            sample_rate: 44100,
            bits: if variant.supports_16bit() { 16 } else { 8 },
            channels: if variant == SbVariant::SbPro
                || variant == SbVariant::Sb16
                || variant == SbVariant::SbAwE32
            {
                2
            } else {
                1
            },
            samples_played: AtomicUsize::new(0),
            initialized: false,
        }
    }

    pub fn play_pcm(&self, buf: &[u8]) -> usize {
        self.samples_played.fetch_add(buf.len(), Ordering::Relaxed);
        buf.len()
    }

    pub fn samples_played(&self) -> usize {
        self.samples_played.load(Ordering::Relaxed)
    }
}

impl KernelSubsystem for SoundBlasterDriver {
    fn name(&self) -> &str {
        "soundblaster"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::Device
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::Optional
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["isa_bus"]
    }

    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }
}

/// PC Speaker driver (PIT channel 2, 8254 timer)
pub struct PcSpeaker {
    pub pit_base: u16,
    pub enabled: bool,
    pub frequency: u32,
    beep_count: AtomicUsize,
}

impl PcSpeaker {
    pub fn new() -> Self {
        PcSpeaker {
            pit_base: 0x40,
            enabled: false,
            frequency: 0,
            beep_count: AtomicUsize::new(0),
        }
    }

    pub fn beep(&mut self, freq_hz: u32, duration_ms: u32) {
        self.frequency = freq_hz;
        self.enabled = true;
        self.beep_count.fetch_add(1, Ordering::Relaxed);
        let _ = duration_ms; // In real HW would set PIT then wait
        self.enabled = false;
    }

    pub fn beep_count(&self) -> usize {
        self.beep_count.load(Ordering::Relaxed)
    }
}

impl Default for PcSpeaker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opl_channels() {
        let opl2 = OplSynth::opl2();
        assert_eq!(opl2.channel_count(), 9);
        let opl3 = OplSynth::opl3();
        assert_eq!(opl3.channel_count(), 18);
    }

    #[test]
    fn test_opl_note_on_off() {
        let mut opl = OplSynth::opl2();
        opl.note_on(0, 512, 4).unwrap();
        assert!(opl.channels[0].key_on);
        assert_eq!(opl.note_count(), 1);
        opl.note_off(0).unwrap();
        assert!(!opl.channels[0].key_on);
    }

    #[test]
    fn test_soundblaster_variants() {
        let sb16 = SoundBlasterDriver::new(SbVariant::Sb16);
        assert!(sb16.variant.supports_16bit());
        assert_eq!(sb16.variant.max_sample_rate(), 48000);
        let sb10 = SoundBlasterDriver::new(SbVariant::Sb10);
        assert!(!sb10.variant.supports_16bit());
    }

    #[test]
    fn test_pcm_playback() {
        let sb = SoundBlasterDriver::new(SbVariant::Sb16);
        let buf = vec![0u8; 4096];
        let played = sb.play_pcm(&buf);
        assert_eq!(played, 4096);
        assert_eq!(sb.samples_played(), 4096);
    }

    #[test]
    fn test_pc_speaker_beep() {
        let mut spk = PcSpeaker::new();
        spk.beep(1000, 100);
        assert_eq!(spk.beep_count(), 1);
    }
}
