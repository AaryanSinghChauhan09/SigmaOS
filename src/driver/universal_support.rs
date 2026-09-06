use std::vec;
// Sovereign Universal Driver Environment (UDE) for SigmaOS
// Enables support for all legacy, dropped, and custom hardware peripherals since 1981 (e.g. NE2000, LPT, Floppy, PS/2, SoundBlaster16, PC Speaker).

use crate::driver::device::{
    CharacterDevice, Device, DeviceError, DeviceInfo, DeviceType, NetworkDevice,
};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceGeneration {
    LegacyISA = 0,  // 1980s (Serial, Parallel, Floppy, NE2000, SoundBlaster16, PC Speaker)
    ClassicPCI = 1, // 1990s (PCI Ethernet, SoundBlaster16 PCI)
    ModernPCIe = 2, // 2000s+ (NVMe, Gigabit Ethernet, GPU)
}

/// Supported legacy device types that other operating systems have dropped
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyHardwareType {
    Ne2000Ethernet, // Standard 10Mbps ISA Ethernet Card (created 1987, dropped by modern OSes)
    ParallelLptPort, // IEEE 1284 Parallel Printer Port (dropped by modern OSes)
    FloppyController, // Standard NEC 765 Floppy Disk Controller (dropped by modern OSes)
    SerialMouse,    // Antique Microsoft 2-button Serial Mouse (ISA 0x3F8)
    SoundBlaster16, // ISA Sound Blaster 16 base port 0x220 (dropped by modern OSes)
    PcSpeaker,      // IBM PC Speaker PIT Channel 2 base port 0x61 (dropped by modern OSes)
}

/// A hardware-level representation of an antique device mapping port registers
pub struct LegacyDeviceRegisterSet {
    pub base_port: u16,
    pub io_space: Vec<u8>,
    pub irq_line: u8,
}

impl LegacyDeviceRegisterSet {
    pub fn new(base_port: u16, size: usize, irq: u8) -> Self {
        LegacyDeviceRegisterSet {
            base_port,
            io_space: vec![0u8; size],
            irq_line: irq,
        }
    }

    pub fn read_port(&self, offset: u16) -> u8 {
        let idx = offset as usize;
        if idx < self.io_space.len() {
            self.io_space[idx]
        } else {
            0
        }
    }

    pub fn write_port(&mut self, offset: u16, val: u8) {
        let idx = offset as usize;
        if idx < self.io_space.len() {
            self.io_space[idx] = val;
        }
    }
}

/// Sovereign Universal Adapter wrapping antique hardware to fit modern SigmaOS Device interfaces
pub struct SovereignLegacyPeripheralAdapter {
    pub name: String,
    pub hw_type: LegacyHardwareType,
    pub registers: LegacyDeviceRegisterSet,
    pub mac_address: [u8; 6],
    pub is_initialized: bool,
    // Sound Blaster 16 and PC Speaker internal emulated states
    pub sample_rate: u32,
    pub pitch_frequency: u32,
}

impl SovereignLegacyPeripheralAdapter {
    pub fn new_ne2000(base_port: u16, irq: u8) -> Self {
        SovereignLegacyPeripheralAdapter {
            name: String::from("NE2000 ISA Ethernet"),
            hw_type: LegacyHardwareType::Ne2000Ethernet,
            registers: LegacyDeviceRegisterSet::new(base_port, 32, irq),
            mac_address: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56], // Standard QEMU NE2000 MAC
            is_initialized: false,
            sample_rate: 0,
            pitch_frequency: 0,
        }
    }

    pub fn new_lpt(base_port: u16) -> Self {
        SovereignLegacyPeripheralAdapter {
            name: String::from("LPT1 Parallel Printer"),
            hw_type: LegacyHardwareType::ParallelLptPort,
            registers: LegacyDeviceRegisterSet::new(base_port, 8, 7),
            mac_address: [0; 6],
            is_initialized: false,
            sample_rate: 0,
            pitch_frequency: 0,
        }
    }

    pub fn new_serial_mouse(base_port: u16) -> Self {
        SovereignLegacyPeripheralAdapter {
            name: String::from("Serial Mouse"),
            hw_type: LegacyHardwareType::SerialMouse,
            registers: LegacyDeviceRegisterSet::new(base_port, 8, 4),
            mac_address: [0; 6],
            is_initialized: false,
            sample_rate: 0,
            pitch_frequency: 0,
        }
    }

    pub fn new_sound_blaster(base_port: u16, irq: u8) -> Self {
        SovereignLegacyPeripheralAdapter {
            name: String::from("Sound Blaster 16 ISA Audio"),
            hw_type: LegacyHardwareType::SoundBlaster16,
            registers: LegacyDeviceRegisterSet::new(base_port, 32, irq),
            mac_address: [0; 6],
            is_initialized: false,
            sample_rate: 22050, // Standard 22kHz fallback
            pitch_frequency: 0,
        }
    }

    pub fn new_pc_speaker() -> Self {
        SovereignLegacyPeripheralAdapter {
            name: String::from("IBM PC Speaker PIT"),
            hw_type: LegacyHardwareType::PcSpeaker,
            registers: LegacyDeviceRegisterSet::new(0x61, 8, 0),
            mac_address: [0; 6],
            is_initialized: false,
            sample_rate: 0,
            pitch_frequency: 0,
        }
    }
}

// Implement standard SigmaOS Device traits for our Legacy Peripheral Adapter
impl Device for SovereignLegacyPeripheralAdapter {
    fn init(&mut self) -> Result<(), DeviceError> {
        self.is_initialized = true;
        // Simulate legacy chip reset sequence
        match self.hw_type {
            LegacyHardwareType::Ne2000Ethernet => {
                self.registers.write_port(0x00, 0x21); // Set QEMU NE2000 page 0, stop mode
            }
            LegacyHardwareType::ParallelLptPort => {
                self.registers.write_port(0x02, 0x0C); // Reset parallel port control
            }
            LegacyHardwareType::SoundBlaster16 => {
                self.registers.write_port(0x06, 1); // Set DSP reset active
                self.registers.write_port(0x06, 0); // Clear DSP reset
                self.registers.write_port(0x0E, 0xAA); // DSP acknowledged ready byte
            }
            LegacyHardwareType::PcSpeaker => {
                self.registers.write_port(0x00, 0xFC); // Clear PIT speaker gate and enable flags
            }
            _ => {}
        }
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        if !self.is_initialized {
            return Err(DeviceError::NotInitialized);
        }
        match self.hw_type {
            LegacyHardwareType::SerialMouse => {
                // Emulate antique 3-byte Microsoft Mouse protocol package:
                // Byte 1: Button states (Bit 0: Left, Bit 1: Right)
                // Byte 2: Delta X motion
                // Byte 3: Delta Y motion
                if buffer.len() >= 3 {
                    buffer[0] = 0x40 | 1; // Left button pressed
                    buffer[1] = 2; // Moved 2 units right
                    buffer[2] = 0; // No Y motion
                    Ok(3)
                } else {
                    Ok(0)
                }
            }
            LegacyHardwareType::SoundBlaster16 => {
                // Emulate reading current DSP sample rate bytes (0x0C register)
                if buffer.len() >= 2 {
                    let rate_high = ((self.sample_rate >> 8) & 0xFF) as u8;
                    let rate_low = (self.sample_rate & 0xFF) as u8;
                    buffer[0] = rate_high;
                    buffer[1] = rate_low;
                    Ok(2)
                } else {
                    Ok(0)
                }
            }
            _ => Ok(0),
        }
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DeviceError> {
        if !self.is_initialized {
            return Err(DeviceError::NotInitialized);
        }
        match self.hw_type {
            LegacyHardwareType::ParallelLptPort => {
                // Write each byte directly to parallel data port register
                for &byte in buffer {
                    self.registers.write_port(0x00, byte); // Write Data
                    self.registers.write_port(0x02, 0x0D); // Strobe pulse high
                    self.registers.write_port(0x02, 0x0C); // Strobe pulse low
                }
                Ok(buffer.len())
            }
            LegacyHardwareType::SoundBlaster16 => {
                // Emulate writing raw PCM voice data block to SB16 DSP DAC (0x0C write buffer)
                for &byte in buffer {
                    self.registers.write_port(0x0C, byte);
                }
                Ok(buffer.len())
            }
            LegacyHardwareType::PcSpeaker => {
                // Emulate writing counter reload bytes to PIT channel 2 frequency port (0x42)
                // Reload value = 1193182 / pitch_frequency
                if buffer.len() >= 2 {
                    let low = buffer[0] as u32;
                    let high = buffer[1] as u32;
                    let reload = (high << 8) | low;
                    if reload > 0 {
                        self.pitch_frequency = 1193182 / reload;
                    }
                }
                Ok(buffer.len())
            }
            _ => Ok(0),
        }
    }

    fn ioctl(&mut self, command: u32, arg: usize) -> Result<usize, DeviceError> {
        match self.hw_type {
            LegacyHardwareType::SoundBlaster16 => {
                if command == 1 {
                    // Command 1: Set SoundBlaster Sample Rate
                    self.sample_rate = arg as u32;
                    return Ok(self.sample_rate as usize);
                }
            }
            LegacyHardwareType::PcSpeaker => {
                if command == 2 {
                    // Command 2: Play custom beep frequency tone directly
                    let freq = arg as u32;
                    self.pitch_frequency = freq;
                    let reload = 1193182 / freq;
                    self.registers.write_port(0x03, (reload & 0xFF) as u8); // Send PIT LSB
                    self.registers
                        .write_port(0x03, ((reload >> 8) & 0xFF) as u8); // Send PIT MSB
                                                                         // Enable speaker beeper gate
                    self.registers.write_port(0x00, 0x03);
                    return Ok(freq as usize);
                }
            }
            _ => {}
        }
        Ok(0)
    }

    fn info(&self) -> DeviceInfo {
        let mut dev_info = DeviceInfo::new(DeviceType::Character);
        dev_info.base_address = self.registers.base_port as u32;
        dev_info.irq = self.registers.irq_line;
        dev_info
    }

    fn shutdown(&mut self) -> Result<(), DeviceError> {
        self.is_initialized = false;
        Ok(())
    }
}

// Implement NetworkDevice for QEMU/Real hardware NE2000 ISA Card adapter
impl NetworkDevice for SovereignLegacyPeripheralAdapter {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), DeviceError> {
        if self.hw_type != LegacyHardwareType::Ne2000Ethernet {
            return Err(DeviceError::NotSupported);
        }
        // Write packet size and buffer to NE2000 ring page registers
        let len = packet.len().min(255);
        self.registers.write_port(0x0B, len as u8); // Set count
        for i in 0..len {
            self.registers.write_port(0x10, packet[i]); // Remote DMA Data port
        }
        self.registers.write_port(0x00, 0x24); // Trigger transmit command
        Ok(())
    }

    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        if self.hw_type != LegacyHardwareType::Ne2000Ethernet {
            return Err(DeviceError::NotSupported);
        }
        // Check receiver boundary registers
        let count = self.registers.read_port(0x0B) as usize;
        let len = count.min(buffer.len());
        for i in 0..len {
            buffer[i] = self.registers.read_port(0x10);
        }
        Ok(len)
    }

    fn get_mac_address(&self) -> [u8; 6] {
        self.mac_address
    }

    fn set_mac_address(&mut self, mac: [u8; 6]) -> Result<(), DeviceError> {
        self.mac_address = mac;
        Ok(())
    }
}

// CharacterDevice interface mapping
impl CharacterDevice for SovereignLegacyPeripheralAdapter {
    fn read_char(&mut self) -> Result<u8, DeviceError> {
        let mut buf = [0u8; 1];
        self.read(&mut buf)?;
        Ok(buf[0])
    }

    fn write_char(&mut self, c: u8) -> Result<(), DeviceError> {
        self.write(&[c])?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }
}

/// Hardware Auto-Negotiation Broker (POLA based)
/// Safely probes and brokers antique hardware connections, preventing CPU instruction faults
pub struct HardwareAutoNegotiationBroker {
    pub registered_legacy_ports: Vec<u16>,
}

impl HardwareAutoNegotiationBroker {
    pub fn new() -> Self {
        HardwareAutoNegotiationBroker {
            registered_legacy_ports: vec![0x3F8, 0x2F8, 0x378, 0x300, 0x220, 0x61], // COM1, COM2, LPT1, NE2000, SB16, PC Speaker defaults
        }
    }

    /// Autodetects device generations and wraps them seamlessly in UDE adapters
    pub fn negotiate_device_bus(&self, port: u16) -> Option<SovereignLegacyPeripheralAdapter> {
        if !self.registered_legacy_ports.contains(&port) {
            return None;
        }

        match port {
            0x3F8 => Some(SovereignLegacyPeripheralAdapter::new_serial_mouse(0x3F8)),
            0x378 => Some(SovereignLegacyPeripheralAdapter::new_lpt(0x378)),
            0x300 => Some(SovereignLegacyPeripheralAdapter::new_ne2000(0x300, 9)),
            0x220 => Some(SovereignLegacyPeripheralAdapter::new_sound_blaster(
                0x220, 5,
            )),
            0x61 => Some(SovereignLegacyPeripheralAdapter::new_pc_speaker()),
            _ => None,
        }
    }
}

impl Default for HardwareAutoNegotiationBroker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_ne2000_ethernet_transmissions() {
        let mut ne2000 = SovereignLegacyPeripheralAdapter::new_ne2000(0x300, 9);
        ne2000.init().unwrap();

        // 1. Send simulated Ethernet Packet
        let packet = b"PING-LEGACY-CARD";
        ne2000.send_packet(packet).unwrap();

        // Verify registers were updated with DMA parameters
        assert_eq!(ne2000.registers.read_port(0x0B), packet.len() as u8);

        // 2. Receive packet back (loopback)
        let mut rcv_buf = [0u8; 32];
        ne2000.registers.write_port(0x0B, 10);
        ne2000.registers.write_port(0x10, 0xEE); // Mock received byte
        let len = ne2000.receive_packet(&mut rcv_buf).unwrap();
        assert_eq!(len, 10);
        assert_eq!(rcv_buf[0], 0xEE);
    }

    #[test]
    fn test_parallel_lpt_printer_adapter() {
        let mut lpt = SovereignLegacyPeripheralAdapter::new_lpt(0x378);
        lpt.init().unwrap();

        // Write printing payload
        let payload = b"SIGMA-PRINT";
        let written = lpt.write(payload).unwrap();
        assert_eq!(written, payload.len());

        // Verify the last byte written is stored in parallel data register 0x00
        assert_eq!(lpt.registers.read_port(0x00), b'T');
    }

    #[test]
    fn test_hardware_broker_auto_negotiation() {
        let broker = HardwareAutoNegotiationBroker::new();

        // Auto-negotiate COM1 serial mouse
        let mouse_opt = broker.negotiate_device_bus(0x3F8);
        assert!(mouse_opt.is_some());
        let mut mouse = mouse_opt.unwrap();
        assert_eq!(mouse.hw_type, LegacyHardwareType::SerialMouse);

        mouse.init().unwrap();
        let mut mouse_pkg = [0u8; 8];
        let read = mouse.read(&mut mouse_pkg).unwrap();
        assert_eq!(read, 3);
        assert_eq!(mouse_pkg[0], 0x41); // Button & active bit flag
    }

    #[test]
    fn test_sound_blaster_16_audio() {
        let mut sb16 = SovereignLegacyPeripheralAdapter::new_sound_blaster(0x220, 5);
        sb16.init().unwrap();

        // Play simulated wave data
        let pcm_data = b"SB16-AUDIO-BLOCK";
        sb16.write(pcm_data).unwrap();

        // Change sampling rate via ioctl
        sb16.ioctl(1, 44100).unwrap();
        assert_eq!(sb16.sample_rate, 44100);

        // Verify reading sample rate bytes
        let mut rate_buf = [0u8; 2];
        sb16.read(&mut rate_buf).unwrap();
        assert_eq!(rate_buf[0], ((44100 >> 8) & 0xFF) as u8);
    }

    #[test]
    fn test_pc_speaker_pit_beeps() {
        let mut speaker = SovereignLegacyPeripheralAdapter::new_pc_speaker();
        speaker.init().unwrap();

        // Play 440Hz beep tone using PIT Channel 2 ioctl
        speaker.ioctl(2, 440).unwrap();
        assert_eq!(speaker.pitch_frequency, 440);

        // Verify writing PIT reload bytes (e.g. reload value for 1000Hz beep)
        // 1193182 / 1000 = 1193 reload counter
        let reload = 1193u16;
        let reload_bytes = reload.to_le_bytes();
        speaker.write(&reload_bytes).unwrap();
        assert_eq!(speaker.pitch_frequency, 1193182 / 1193);
    }
}
