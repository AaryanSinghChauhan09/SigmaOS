//! Intel High Definition Audio (HDA) Driver
//! Supports modern audio controllers and codecs
//! Reference: Intel HD Audio Specification and Linux sound/pci/hda/

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// HDA PCI registers (memory-mapped I/O)
#[repr(C)]
#[derive(Debug)]
pub struct HdaRegisters {
    pub gcap: u16,     // Global Capabilities
    pub vmin: u8,      // Minor Version
    pub vmaj: u8,      // Major Version
    pub outpay: u16,   // Output Payload Capability
    pub inpay: u16,    // Input Payload Capability
    pub gctl: u32,     // Global Control
    pub wakeen: u16,   // Wake Enable
    pub statests: u16, // State Change Status
    pub gsts: u16,     // Global Status
    pub _reserved1: u16,
    pub outstrmpay: u16, // Output Stream Payload Capability
    pub instrmpay: u16,  // Input Stream Payload Capability
    pub _reserved2: u32,
    pub intctl: u32, // Interrupt Control
    pub intsts: u32, // Interrupt Status
    pub _reserved3: [u32; 2],
    pub walclk: u32, // Wall Clock Counter
    pub _reserved4: u32,
    pub ssync: u32, // Stream Synchronization
    pub _reserved5: u32,
    pub corblbase: u32, // CORB Lower Base Address
    pub corbubase: u32, // CORB Upper Base Address
    pub corbwp: u16,    // CORB Write Pointer
    pub corbrp: u16,    // CORB Read Pointer
    pub corbctl: u8,    // CORB Control
    pub corbsts: u8,    // CORB Status
    pub corbsize: u8,   // CORB Size
    pub _reserved6: u8,
    pub rirblbase: u32, // RIRB Lower Base Address
    pub rirbubase: u32, // RIRB Upper Base Address
    pub rirbwp: u16,    // RIRB Write Pointer
    pub rintcnt: u16,   // Response Interrupt Count
    pub rirbctl: u8,    // RIRB Control
    pub rirbsts: u8,    // RIRB Status
    pub rirbsize: u8,   // RIRB Size
    pub _reserved7: u8,
}

/// HDA Verb (command sent to codec)
#[derive(Debug, Clone, Copy)]
pub struct HdaVerb {
    pub codec_addr: u8, // Codec address (0-14)
    pub node_id: u8,    // Node ID
    pub verb: u16,      // Verb ID
    pub payload: u8,    // Payload/parameter
}

impl HdaVerb {
    pub fn to_u32(&self) -> u32 {
        ((self.codec_addr as u32) << 28)
            | ((self.node_id as u32) << 20)
            | ((self.verb as u32) << 8)
            | (self.payload as u32)
    }

    pub fn from_u32(val: u32) -> Self {
        Self {
            codec_addr: ((val >> 28) & 0x0F) as u8,
            node_id: ((val >> 20) & 0x7F) as u8,
            verb: ((val >> 8) & 0xFFF) as u16,
            payload: (val & 0xFF) as u8,
        }
    }
}

/// HDA Response (from codec)
#[derive(Debug, Clone, Copy)]
pub struct HdaResponse {
    pub data: u32,     // Response data
    pub extended: u32, // Extended response (codec addr + unsolicited)
}

impl HdaResponse {
    pub fn codec_addr(&self) -> u8 {
        (self.extended & 0x0F) as u8
    }

    pub fn is_unsolicited(&self) -> bool {
        (self.extended & 0x10) != 0
    }
}

/// HDA Codec Node types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdaNodeType {
    AudioOut = 0x0,
    AudioIn = 0x1,
    AudioMixer = 0x2,
    AudioSelector = 0x3,
    PinComplex = 0x4,
    Power = 0x5,
    VolumeKnob = 0x6,
    BeepGen = 0x7,
    VendorDefined = 0xF,
}

/// HDA Widget capabilities
#[derive(Debug, Clone, Copy)]
pub struct WidgetCapabilities {
    pub stereo: bool,
    pub in_amp_present: bool,
    pub out_amp_present: bool,
    pub amp_param_override: bool,
    pub format_override: bool,
    pub stripe: bool,
    pub proc_widget: bool,
    pub unsol_capable: bool,
    pub conn_list: bool,
    pub digital: bool,
    pub power_cntrl: bool,
    pub lr_swap: bool,
    pub cp_caps: bool,
    pub widget_type: HdaNodeType,
    pub delay: u8,
    pub channel_count: u8,
}

/// Pin widget configuration (from BIOS)
#[derive(Debug, Clone, Copy)]
pub struct PinConfig {
    pub port_connectivity: u8,
    pub location: u8,
    pub default_device: u8,
    pub connection_type: u8,
    pub color: u8,
    pub misc: u8,
    pub default_association: u8,
    pub sequence: u8,
}

impl PinConfig {
    pub fn from_u32(val: u32) -> Self {
        Self {
            port_connectivity: ((val >> 30) & 0x3) as u8,
            location: ((val >> 24) & 0x3F) as u8,
            default_device: ((val >> 20) & 0xF) as u8,
            connection_type: ((val >> 16) & 0xF) as u8,
            color: ((val >> 12) & 0xF) as u8,
            misc: ((val >> 8) & 0xF) as u8,
            default_association: ((val >> 4) & 0xF) as u8,
            sequence: (val & 0xF) as u8,
        }
    }
}

/// Audio stream format (sample rate, bit depth, channels)
#[derive(Debug, Clone, Copy)]
pub struct StreamFormat {
    pub sample_rate: u32, // Hz (8000, 11025, 16000, 22050, 32000, 44100, 48000, 96000, 192000)
    pub bits_per_sample: u8, // 8, 16, 20, 24, 32
    pub channels: u8,     // 1-16
}

impl StreamFormat {
    pub fn encode(&self) -> u16 {
        let base_rate: u16 = match self.sample_rate {
            48000 => 0x0,
            44100 => 0x4000,
            _ => 0x0,
        };
        let mult: u16 = match self.sample_rate {
            48000 | 44100 => 0x0,
            96000 | 88200 => 0x1,
            192000 | 176400 => 0x3,
            _ => 0x0,
        };
        let bits: u16 = match self.bits_per_sample {
            8 => 0x0,
            16 => 0x1,
            20 => 0x2,
            24 => 0x3,
            32 => 0x4,
            _ => 0x1,
        };
        let chan = ((self.channels - 1) & 0xF) as u16;

        base_rate | (mult << 11) | (bits << 4) | chan
    }
}

/// HDA Codec structure
pub struct HdaCodec {
    pub codec_addr: u8,
    pub vendor_id: u32,
    pub revision_id: u32,
    pub function_group_nodes: Vec<u8>,
    pub audio_widgets: BTreeMap<u8, WidgetCapabilities>,
}

impl HdaCodec {
    pub fn new(codec_addr: u8) -> Self {
        Self {
            codec_addr,
            vendor_id: 0,
            revision_id: 0,
            function_group_nodes: Vec::new(),
            audio_widgets: BTreeMap::new(),
        }
    }

    /// Get codec vendor ID and revision
    pub fn probe(&mut self, driver: &mut HdaDriver) -> Result<(), HdaError> {
        // Read vendor ID (verb 0xF00, parameter 0x00)
        let verb = HdaVerb {
            codec_addr: self.codec_addr,
            node_id: 0,
            verb: 0xF00,
            payload: 0x00,
        };
        let response = driver.send_verb(verb)?;
        self.vendor_id = response.data;

        // Read revision ID (verb 0xF00, parameter 0x02)
        let verb = HdaVerb {
            codec_addr: self.codec_addr,
            node_id: 0,
            verb: 0xF00,
            payload: 0x02,
        };
        let response = driver.send_verb(verb)?;
        self.revision_id = response.data;

        Ok(())
    }

    /// Enumerate function groups
    pub fn enumerate_function_groups(&mut self, driver: &mut HdaDriver) -> Result<(), HdaError> {
        // Get subordinate node count (verb 0xF00, parameter 0x04)
        let verb = HdaVerb {
            codec_addr: self.codec_addr,
            node_id: 0,
            verb: 0xF00,
            payload: 0x04,
        };
        let response = driver.send_verb(verb)?;

        let start_node = ((response.data >> 16) & 0xFF) as u8;
        let num_nodes = (response.data & 0xFF) as u8;

        for i in 0..num_nodes {
            self.function_group_nodes.push(start_node + i);
        }

        Ok(())
    }
}

/// HDA Driver main structure
pub struct HdaDriver {
    pub regs: *mut HdaRegisters,
    pub codecs: Vec<HdaCodec>,
    pub corb: Vec<u32>,     // Command Outbound Ring Buffer
    pub rirb: Vec<u64>,     // Response Inbound Ring Buffer
    pub corb_wp: AtomicU32, // CORB write pointer
    pub rirb_rp: AtomicU32, // RIRB read pointer
}

impl HdaDriver {
    pub fn new(mmio_base: u64) -> Self {
        Self {
            regs: mmio_base as *mut HdaRegisters,
            codecs: Vec::new(),
            corb: vec![0; 256],
            rirb: vec![0; 256],
            corb_wp: AtomicU32::new(0),
            rirb_rp: AtomicU32::new(0),
        }
    }

    /// Initialize HDA controller
    #[allow(clippy::while_immutable_condition)] // MMIO polling: the register changes in hardware, not in this loop body
    pub fn init(&mut self) -> Result<(), HdaError> {
        unsafe {
            // Reset controller (GCTL.CRST = 0, then 1)
            (*self.regs).gctl &= !0x01;
            // Wait for reset
            while ((*self.regs).gctl & 0x01) != 0 {}

            (*self.regs).gctl |= 0x01;
            // Wait for ready
            while ((*self.regs).gctl & 0x01) == 0 {}

            // Initialize CORB and RIRB
            self.init_corb_rirb()?;

            // Detect codecs
            let statests = (*self.regs).statests;
            for i in 0..15 {
                if (statests & (1 << i)) != 0 {
                    let mut codec = HdaCodec::new(i);
                    codec.probe(self)?;
                    self.codecs.push(codec);
                }
            }
        }

        Ok(())
    }

    /// Initialize CORB (Command Outbound Ring Buffer) and RIRB (Response Inbound)
    #[allow(clippy::while_immutable_condition)] // MMIO polling: the register changes in hardware, not in this loop body
    fn init_corb_rirb(&mut self) -> Result<(), HdaError> {
        unsafe {
            // Set CORB size to 256 entries
            (*self.regs).corbsize = 0x02;

            // Set CORB base address (would be DMA-allocated physical address)
            let corb_phys = self.corb.as_ptr() as u64;
            (*self.regs).corblbase = (corb_phys & 0xFFFFFFFF) as u32;
            (*self.regs).corbubase = ((corb_phys >> 32) & 0xFFFFFFFF) as u32;

            // Reset CORB read pointer
            (*self.regs).corbrp = 0x8000; // Set reset bit
            while ((*self.regs).corbrp & 0x8000) != 0 {}
            (*self.regs).corbrp = 0;

            // Set RIRB size to 256 entries
            (*self.regs).rirbsize = 0x02;

            // Set RIRB base address
            let rirb_phys = self.rirb.as_ptr() as u64;
            (*self.regs).rirblbase = (rirb_phys & 0xFFFFFFFF) as u32;
            (*self.regs).rirbubase = ((rirb_phys >> 32) & 0xFFFFFFFF) as u32;

            // Reset RIRB write pointer
            (*self.regs).rirbwp = 0x8000;

            // Enable CORB and RIRB
            (*self.regs).corbctl = 0x02; // Enable CORB DMA
            (*self.regs).rirbctl = 0x02; // Enable RIRB DMA
        }

        Ok(())
    }

    /// Send verb to codec and wait for response
    pub fn send_verb(&mut self, verb: HdaVerb) -> Result<HdaResponse, HdaError> {
        let wp = self.corb_wp.load(Ordering::SeqCst);
        let next_wp = (wp + 1) % 256;

        // Write verb to CORB
        self.corb[next_wp as usize] = verb.to_u32();

        unsafe {
            // Update hardware write pointer
            (*self.regs).corbwp = next_wp as u16;
        }

        self.corb_wp.store(next_wp, Ordering::SeqCst);

        // Wait for response in RIRB
        let mut timeout = 1000;
        loop {
            let hw_wp = unsafe { (*self.regs).rirbwp } as u32;
            let rp = self.rirb_rp.load(Ordering::SeqCst);

            if hw_wp != rp {
                let next_rp = (rp + 1) % 256;
                let response_raw = self.rirb[next_rp as usize];

                self.rirb_rp.store(next_rp, Ordering::SeqCst);

                return Ok(HdaResponse {
                    data: (response_raw & 0xFFFFFFFF) as u32,
                    extended: ((response_raw >> 32) & 0xFFFFFFFF) as u32,
                });
            }

            timeout -= 1;
            if timeout == 0 {
                return Err(HdaError::Timeout);
            }
        }
    }

    /// Set stream format
    pub fn set_stream_format(
        &mut self,
        stream_id: u8,
        format: StreamFormat,
    ) -> Result<(), HdaError> {
        // In real implementation: write to stream descriptor format register
        Ok(())
    }

    /// Start audio playback
    pub fn start_playback(&mut self, stream_id: u8, buffer: &[u8]) -> Result<(), HdaError> {
        // In real implementation:
        // 1. Set up Buffer Descriptor List (BDL) with DMA addresses
        // 2. Configure stream descriptor
        // 3. Start stream (set RUN bit)
        Ok(())
    }
}

/// HDA error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdaError {
    Timeout,
    NoCodec,
    InvalidVerb,
    DmaError,
    StreamError,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hda_verb() {
        let verb = HdaVerb {
            codec_addr: 0,
            node_id: 1,
            verb: 0xF00,
            payload: 0x00,
        };
        let encoded = verb.to_u32();
        let decoded = HdaVerb::from_u32(encoded);
        assert_eq!(decoded.codec_addr, 0);
        assert_eq!(decoded.node_id, 1);
    }

    #[test]
    fn test_stream_format() {
        let format = StreamFormat {
            sample_rate: 48000,
            bits_per_sample: 16,
            channels: 2,
        };
        let encoded = format.encode();
        assert_eq!(encoded & 0xF, 1); // 2 channels - 1
    }
}
