#![allow(dead_code)]
// SigmaOS Arduino Microcontroller Integration Engine
// Implements Linux udev / BSD devd inspired USB serial probing,
// STK500v1 / Optiboot bootloader firmware flashing protocols,
// board pinmap capabilities, and high-performance serial monitoring.

use std::collections::HashMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Supported Arduino board architectures and target MCUs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArduinoBoardArchitecture {
    AvrAtmega328p, // Arduino Uno / Nano
    AvrAtmega2560, // Arduino Mega 2560
    AvrAtmega32u4, // Arduino Leonardo / Micro
    Esp32,          // ESP32 WROOM / S3
    Rp2040,         // Raspberry Pi Pico / Arduino Nano RP2040 Connect
}

/// Detailed profile for an Arduino board
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArduinoBoardProfile {
    pub name: &'static str,
    pub architecture: ArduinoBoardArchitecture,
    pub flash_size_bytes: usize,
    pub eeprom_size_bytes: usize,
    pub ram_size_bytes: usize,
    pub default_baud_rate: u32,
    pub page_size_bytes: usize,
}

impl ArduinoBoardProfile {
    pub fn get_profile(arch: ArduinoBoardArchitecture) -> Self {
        match arch {
            ArduinoBoardArchitecture::AvrAtmega328p => Self {
                name: "Arduino Uno R3 / Nano (ATmega328P)",
                architecture: arch,
                flash_size_bytes: 32 * 1024,
                eeprom_size_bytes: 1024,
                ram_size_bytes: 2048,
                default_baud_rate: 115_200,
                page_size_bytes: 128,
            },
            ArduinoBoardArchitecture::AvrAtmega2560 => Self {
                name: "Arduino Mega 2560 R3 (ATmega2560)",
                architecture: arch,
                flash_size_bytes: 256 * 1024,
                eeprom_size_bytes: 4096,
                ram_size_bytes: 8192,
                default_baud_rate: 115_200,
                page_size_bytes: 256,
            },
            ArduinoBoardArchitecture::AvrAtmega32u4 => Self {
                name: "Arduino Leonardo / Micro (ATmega32u4)",
                architecture: arch,
                flash_size_bytes: 32 * 1024,
                eeprom_size_bytes: 1024,
                ram_size_bytes: 2560,
                default_baud_rate: 57_600,
                page_size_bytes: 128,
            },
            ArduinoBoardArchitecture::Esp32 => Self {
                name: "ESP32 Dev Module / NodeMCU-32S",
                architecture: arch,
                flash_size_bytes: 4 * 1024 * 1024,
                eeprom_size_bytes: 4096,
                ram_size_bytes: 520 * 1024,
                default_baud_rate: 921_600,
                page_size_bytes: 1024,
            },
            ArduinoBoardArchitecture::Rp2040 => Self {
                name: "Arduino Nano RP2040 Connect / Pico",
                architecture: arch,
                flash_size_bytes: 2 * 1024 * 1024,
                eeprom_size_bytes: 0,
                ram_size_bytes: 264 * 1024,
                default_baud_rate: 115_200,
                page_size_bytes: 256,
            },
        }
    }
}

/// USB Serial Bridge auto-probe record (USB VID/PID matching)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsbSerialBridgeRecord {
    pub vendor_id: u16,
    pub product_id: u16,
    pub bridge_chip: &'static str,
    pub target_architecture: ArduinoBoardArchitecture,
}

/// Linux udev & FreeBSD devd inspired USB Serial Probe Engine
pub struct ArduinoUsbVidPidProbe {
    known_bridges: Vec<UsbSerialBridgeRecord>,
}

impl ArduinoUsbVidPidProbe {
    pub fn new() -> Self {
        let known_bridges = vec![
            // Official Arduino Uno (ATmega16U2)
            UsbSerialBridgeRecord {
                vendor_id: 0x2341,
                product_id: 0x0043,
                bridge_chip: "ATmega16U2 (Official Arduino)",
                target_architecture: ArduinoBoardArchitecture::AvrAtmega328p,
            },
            // Official Arduino Mega 2560
            UsbSerialBridgeRecord {
                vendor_id: 0x2341,
                product_id: 0x0010,
                bridge_chip: "ATmega16U2 (Arduino Mega 2560)",
                target_architecture: ArduinoBoardArchitecture::AvrAtmega2560,
            },
            // WCH CH340 / CH341 (Popular clone bridge)
            UsbSerialBridgeRecord {
                vendor_id: 0x1A86,
                product_id: 0x7523,
                bridge_chip: "WCH CH340G Serial Bridge",
                target_architecture: ArduinoBoardArchitecture::AvrAtmega328p,
            },
            // FTDI FT232R
            UsbSerialBridgeRecord {
                vendor_id: 0x0403,
                product_id: 0x6001,
                bridge_chip: "FTDI FT232R USB UART",
                target_architecture: ArduinoBoardArchitecture::AvrAtmega328p,
            },
            // Silicon Labs CP210x (ESP32 / NodeMCU)
            UsbSerialBridgeRecord {
                vendor_id: 0x10C4,
                product_id: 0xEA60,
                bridge_chip: "Silicon Labs CP2102/CP2104 USB to UART",
                target_architecture: ArduinoBoardArchitecture::Esp32,
            },
            // Raspberry Pi RP2040 Bootloader / CDC
            UsbSerialBridgeRecord {
                vendor_id: 0x2E8A,
                product_id: 0x000A,
                bridge_chip: "RP2040 USB CDC Serial",
                target_architecture: ArduinoBoardArchitecture::Rp2040,
            },
        ];

        Self { known_bridges }
    }

    /// Probe device node matching vendor and product IDs
    pub fn probe_device(&self, vendor_id: u16, product_id: u16, dev_index: usize) -> Option<ArduinoDiscoveredDevice> {
        for record in &self.known_bridges {
            if record.vendor_id == vendor_id && record.product_id == product_id {
                let linux_device = format!("/dev/ttyUSB{}", dev_index);
                let bsd_device = format!("/dev/cuaU{}", dev_index);
                let sigma_alias = format!("/dev/arduino{}", dev_index);

                return Some(ArduinoDiscoveredDevice {
                    vendor_id,
                    product_id,
                    bridge_chip: record.bridge_chip,
                    architecture: record.target_architecture,
                    linux_device_node: linux_device,
                    bsd_device_node: bsd_device,
                    sigma_dev_alias: sigma_alias,
                });
            }
        }
        None
    }
}

/// Discovered Arduino USB Serial Device
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArduinoDiscoveredDevice {
    pub vendor_id: u16,
    pub product_id: u16,
    pub bridge_chip: &'static str,
    pub architecture: ArduinoBoardArchitecture,
    pub linux_device_node: String,
    pub bsd_device_node: String,
    pub sigma_dev_alias: String,
}

/// STK500v1 / Optiboot Serial Protocol Constants
pub mod stk500_constants {
    pub const STK_OK: u8 = 0x10;
    pub const STK_FAILED: u8 = 0x11;
    pub const STK_UNKNOWN: u8 = 0x12;
    pub const STK_INSYNC: u8 = 0x14;
    pub const STK_NOSYNC: u8 = 0x15;
    pub const CRC_EOP: u8 = 0x20;

    pub const STK_GET_SYNC: u8 = 0x30;
    pub const STK_GET_SIGN_ON: u8 = 0x31;
    pub const STK_SET_PARAMETER: u8 = 0x40;
    pub const STK_GET_PARAMETER: u8 = 0x41;
    pub const STK_LOAD_ADDRESS: u8 = 0x55;
    pub const STK_PROG_PAGE: u8 = 0x64;
    pub const STK_READ_PAGE: u8 = 0x74;
    pub const STK_READ_SIGN: u8 = 0x75;
}

/// STK500v1 Flashing Engine for AVR-based Arduino bootloaders
pub struct ArduinoStk500FlashingEngine {
    pub is_synced: bool,
    pub target_arch: ArduinoBoardArchitecture,
    pub flash_memory: Vec<u8>,
}

impl ArduinoStk500FlashingEngine {
    pub fn new(arch: ArduinoBoardArchitecture) -> Self {
        let profile = ArduinoBoardProfile::get_profile(arch);
        Self {
            is_synced: false,
            target_arch: arch,
            flash_memory: vec![0xFF; profile.flash_size_bytes],
        }
    }

    /// Process STK500 command packet frame
    pub fn process_command(&mut self, payload: &[u8]) -> Result<Vec<u8>, &'static str> {
        if payload.is_empty() {
            return Err("Empty STK500 command payload");
        }

        let cmd = payload[0];
        let mut response = Vec::new();

        match cmd {
            stk500_constants::STK_GET_SYNC => {
                if payload.len() >= 2 && payload[1] == stk500_constants::CRC_EOP {
                    self.is_synced = true;
                    response.push(stk500_constants::STK_INSYNC);
                    response.push(stk500_constants::STK_OK);
                    Ok(response)
                } else {
                    Err("Invalid CRC_EOP marker for GET_SYNC")
                }
            }
            stk500_constants::STK_READ_SIGN => {
                if !self.is_synced {
                    return Err("STK500 not in sync");
                }
                response.push(stk500_constants::STK_INSYNC);
                // AVR Signature bytes for ATmega328P: 0x1E, 0x95, 0x0F
                match self.target_arch {
                    ArduinoBoardArchitecture::AvrAtmega328p => {
                        response.extend_from_slice(&[0x1E, 0x95, 0x0F]);
                    }
                    ArduinoBoardArchitecture::AvrAtmega2560 => {
                        response.extend_from_slice(&[0x1E, 0x98, 0x01]);
                    }
                    ArduinoBoardArchitecture::AvrAtmega32u4 => {
                        response.extend_from_slice(&[0x1E, 0x95, 0x87]);
                    }
                    _ => {
                        response.extend_from_slice(&[0xFF, 0xFF, 0xFF]);
                    }
                }
                response.push(stk500_constants::STK_OK);
                Ok(response)
            }
            stk500_constants::STK_PROG_PAGE => {
                if !self.is_synced {
                    return Err("STK500 not in sync");
                }
                // Expects: [STK_PROG_PAGE, addr_high, addr_low, page_size_high, page_size_low, mem_type ('F'), data..., CRC_EOP]
                if payload.len() < 7 {
                    return Err("Malformed PROG_PAGE payload length");
                }
                let addr = ((payload[1] as usize) << 8) | (payload[2] as usize);
                let page_size = ((payload[3] as usize) << 8) | (payload[4] as usize);
                let data_start = 6;
                let data_end = data_start + page_size;

                if payload.len() <= data_end || payload[data_end] != stk500_constants::CRC_EOP {
                    return Err("PROG_PAGE framing error or missing CRC_EOP");
                }

                if addr + page_size > self.flash_memory.len() {
                    return Err("Flash write address out of bounds");
                }

                self.flash_memory[addr..addr + page_size].copy_from_slice(&payload[data_start..data_end]);

                response.push(stk500_constants::STK_INSYNC);
                response.push(stk500_constants::STK_OK);
                Ok(response)
            }
            _ => Err("Unsupported STK500 command"),
        }
    }
}

/// Pin Capability Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PinCapability {
    DigitalInput,
    DigitalOutput,
    AnalogInput,
    PwmOutput,
    I2cSda,
    I2cScl,
    SpiMosi,
    SpiMiso,
    SpiSck,
    Interrupt,
}

/// Hardware Pin Capability Registry for Arduino Boards
pub struct ArduinoPinmapRegistry {
    pub pin_capabilities: HashMap<u8, Vec<PinCapability>>,
}

impl ArduinoPinmapRegistry {
    pub fn for_uno() -> Self {
        let mut pin_capabilities = HashMap::new();

        // Digital Pins 0-13
        for pin in 0..=13 {
            let mut caps = vec![PinCapability::DigitalInput, PinCapability::DigitalOutput];
            if matches!(pin, 3 | 5 | 6 | 9 | 10 | 11) {
                caps.push(PinCapability::PwmOutput);
            }
            if matches!(pin, 2 | 3) {
                caps.push(PinCapability::Interrupt);
            }
            if pin == 11 {
                caps.push(PinCapability::SpiMosi);
            }
            if pin == 12 {
                caps.push(PinCapability::SpiMiso);
            }
            if pin == 13 {
                caps.push(PinCapability::SpiSck);
            }
            pin_capabilities.insert(pin, caps);
        }

        // Analog Pins A0-A5 (Pins 14-19)
        for pin in 14..=19 {
            let mut caps = vec![
                PinCapability::DigitalInput,
                PinCapability::DigitalOutput,
                PinCapability::AnalogInput,
            ];
            if pin == 18 {
                caps.push(PinCapability::I2cSda);
            }
            if pin == 19 {
                caps.push(PinCapability::I2cScl);
            }
            pin_capabilities.insert(pin, caps);
        }

        Self { pin_capabilities }
    }

    pub fn supports_capability(&self, pin: u8, capability: PinCapability) -> bool {
        if let Some(caps) = self.pin_capabilities.get(&pin) {
            caps.contains(&capability)
        } else {
            false
        }
    }
}

/// High-Performance Arduino Serial Telemetry Monitor
pub struct ArduinoSerialMonitorBridge {
    pub baud_rate: u32,
    pub rx_buffer: Vec<String>,
    pub tx_buffer: Vec<u8>,
}

impl ArduinoSerialMonitorBridge {
    pub fn new(baud_rate: u32) -> Self {
        Self {
            baud_rate,
            rx_buffer: Vec::new(),
            tx_buffer: Vec::new(),
        }
    }

    pub fn receive_raw_bytes(&mut self, incoming: &[u8]) {
        if let Ok(text) = std::str::from_utf8(incoming) {
            for line in text.lines() {
                if !line.is_empty() {
                    self.rx_buffer.push(line.to_string());
                }
            }
        }
    }

    pub fn send_command(&mut self, command: &str) {
        self.tx_buffer.extend_from_slice(command.as_bytes());
        self.tx_buffer.push(b'\n');
    }

    pub fn clear_rx(&mut self) {
        self.rx_buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arduino_usb_probe() {
        let probe = ArduinoUsbVidPidProbe::new();
        let official_uno = probe.probe_device(0x2341, 0x0043, 0).unwrap();
        assert_eq!(official_uno.bridge_chip, "ATmega16U2 (Official Arduino)");
        assert_eq!(official_uno.architecture, ArduinoBoardArchitecture::AvrAtmega328p);
        assert_eq!(official_uno.sigma_dev_alias, "/dev/arduino0");

        let ch340_clone = probe.probe_device(0x1A86, 0x7523, 1).unwrap();
        assert_eq!(ch340_clone.bridge_chip, "WCH CH340G Serial Bridge");
        assert_eq!(ch340_clone.linux_device_node, "/dev/ttyUSB1");
    }

    #[test]
    fn test_stk500_flashing_engine() {
        let mut engine = ArduinoStk500FlashingEngine::new(ArduinoBoardArchitecture::AvrAtmega328p);

        // Send GET_SYNC
        let sync_cmd = vec![stk500_constants::STK_GET_SYNC, stk500_constants::CRC_EOP];
        let sync_resp = engine.process_command(&sync_cmd).unwrap();
        assert_eq!(sync_resp, vec![stk500_constants::STK_INSYNC, stk500_constants::STK_OK]);
        assert!(engine.is_synced);

        // Read Signature
        let sign_cmd = vec![stk500_constants::STK_READ_SIGN, stk500_constants::CRC_EOP];
        let sign_resp = engine.process_command(&sign_cmd).unwrap();
        assert_eq!(
            sign_resp,
            vec![
                stk500_constants::STK_INSYNC,
                0x1E,
                0x95,
                0x0F,
                stk500_constants::STK_OK
            ]
        );

        // Flash page at addr 0x0000, length 4
        let page_data = vec![0x11, 0x22, 0x33, 0x44];
        let mut prog_cmd = vec![
            stk500_constants::STK_PROG_PAGE,
            0x00,
            0x00, // Addr 0
            0x00,
            0x04, // Size 4
            b'F',
        ];
        prog_cmd.extend_from_slice(&page_data);
        prog_cmd.push(stk500_constants::CRC_EOP);

        let prog_resp = engine.process_command(&prog_cmd).unwrap();
        assert_eq!(prog_resp, vec![stk500_constants::STK_INSYNC, stk500_constants::STK_OK]);
        assert_eq!(&engine.flash_memory[0..4], &[0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn test_pinmap_registry() {
        let registry = ArduinoPinmapRegistry::for_uno();
        assert!(registry.supports_capability(3, PinCapability::PwmOutput));
        assert!(registry.supports_capability(3, PinCapability::Interrupt));
        assert!(!registry.supports_capability(4, PinCapability::PwmOutput));

        // A4 (Pin 18) is SDA
        assert!(registry.supports_capability(18, PinCapability::I2cSda));
        assert!(registry.supports_capability(18, PinCapability::AnalogInput));
    }

    #[test]
    fn test_serial_monitor_bridge() {
        let mut monitor = ArduinoSerialMonitorBridge::new(115_200);
        monitor.receive_raw_bytes(b"Sensor A: 23.5C\nSensor B: 1013hPa\n");
        assert_eq!(monitor.rx_buffer.len(), 2);
        assert_eq!(monitor.rx_buffer[0], "Sensor A: 23.5C");

        monitor.send_command("SET_BAUD 9600");
        assert_eq!(monitor.tx_buffer, b"SET_BAUD 9600\n");
    }
}
