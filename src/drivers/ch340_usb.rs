// CH340 USB-to-Serial UART Controller Driver
// Conforms to SigmaOS UnifiedPeripheral interface

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use crate::security::capability::CapabilityToken;

// CH340 USB Vendor & Product IDs
pub const CH340_VENDOR_ID: u16 = 0x1A86;
pub const CH340_PRODUCT_ID: u16 = 0x7523;

// CH340 Register Commands (USB Control requests)
const CMD_WRITE_REG: u8 = 0x9A;
const CMD_READ_REG: u8 = 0x95;
const CMD_INIT: u8 = 0xA1;

// Baud rate configurations (Internal registers)
const REG_FACTOR_1200: (u16, u16) = (0x0F, 0xF300);
const REG_FACTOR_9600: (u16, u16) = (0x12, 0xF300);
const REG_FACTOR_115200: (u16, u16) = (0x13, 0xCC00);

/// CH340 Driver State
pub struct Ch340Driver {
    pub device_address: u8,
    pub bulk_out_endpoint: u8,
    pub bulk_in_endpoint: u8,
    pub active_baud_rate: u32,
    pub power_state: PowerState,
    pub capabilities: CapabilityToken,
}

impl Ch340Driver {
    /// Creates a new uninitialized CH340 USB driver mapped to a active USB device slot
    pub fn new(device_address: u8, capabilities: CapabilityToken) -> Self {
        Self {
            device_address,
            bulk_out_endpoint: 2, // Standard CH340 TX Bulk endpoint
            bulk_in_endpoint: 1,  // Standard CH340 RX Bulk endpoint
            active_baud_rate: 9600,
            power_state: PowerState::Off,
            capabilities,
        }
    }

    // Simulate standard USB control transfer (Vendor OUT request)
    unsafe fn control_write_vendor(
        &mut self,
        command: u8,
        value: u16,
        index: u16,
    ) -> Result<(), &'static str> {
        // In a real OS, submit an asynchronous USB Request Block (URB) to USB Host Controller (xHCI)
        println!(
            "USB: Control OUT Vendor transfer addressed to dev {}: cmd 0x{:X}, val 0x{:X}, idx 0x{:X}",
            self.device_address, command, value, index
        );
        Ok(())
    }

    // Configure specific baud rate factor into hardware registers
    unsafe fn configure_baud_rate(&mut self, baud_rate: u32) -> Result<(), &'static str> {
        let factors = match baud_rate {
            1200 => REG_FACTOR_1200,
            9600 => REG_FACTOR_9600,
            115200 => REG_FACTOR_115200,
            _ => return Err("CH340: Unsupported baud rate configuration factor"),
        };

        // Write baud factors (Register offsets 0x12 and 0x13 control timing loops)
        self.control_write_vendor(CMD_WRITE_REG, 0x12 | factors.0, factors.1)?;
        self.control_write_vendor(CMD_WRITE_REG, 0x13, 0x0000)?;

        self.active_baud_rate = baud_rate;
        Ok(())
    }
}

impl PeripheralDevice for Ch340Driver {
    fn name(&self) -> &'static str {
        "CH340 USB-to-Serial Adapter"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Serial drivers require USB peripheral communication capabilities
        if self.capabilities.bits() & 0x01 == 0 {
            return Err("CH340: PermissionDenied - Missing Device capabilities");
        }

        unsafe {
            // 1. Initialize hardware controller (A1 command resets status)
            self.control_write_vendor(CMD_INIT, 0, 0)?;

            // 2. Read current modem status values
            self.control_write_vendor(CMD_READ_REG, 0, 0)?;

            // 3. Set default line parameters (8 bits, 1 stop bit, no parity: index offset 0xC3)
            self.control_write_vendor(CMD_WRITE_REG, 0x2518, 0x0050)?;

            // 4. Configure default baud rate (9600 bps)
            self.configure_baud_rate(9600)?;

            // 5. Enable Handshake flow control pins (DTR/RTS)
            self.control_write_vendor(CMD_WRITE_REG, 0x2727, 0x0000)?;
        }

        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("CH340: Device is powered off");
        }

        // In a real OS, submit USB Bulk IN transfer to self.bulk_in_endpoint
        // Simulate reading 1 dummy serial character
        if !buffer.is_empty() {
            buffer[0] = b'S'; // Return standard indicator
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("CH340: Device is powered off");
        }

        // In a real OS, submit USB Bulk OUT transfer to self.bulk_out_endpoint
        println!(
            "USB: Bulk OUT transfer addressed to dev {} ep {}: writing {} bytes of serial telemetry",
            self.device_address, self.bulk_out_endpoint, data.len()
        );

        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        unsafe {
            // Reset DTR/RTS handshake pins to disable flow control
            self.control_write_vendor(CMD_WRITE_REG, 0, 0)?;
        }
        self.power_state = PowerState::Off;
        Ok(())
    }
}
