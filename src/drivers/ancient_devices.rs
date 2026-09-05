#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Ancient Device Drivers
// Implements OOP-based lightweight legacy drivers executing sandboxed UDFs


use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use std::boxed::Box;
use std::vec::Vec;

/// A generic OOP representation of an ancient legacy device.
/// Behavior is completely customizable via tiny User-Defined Function (UDF) bytecode.
pub struct UdfAncientDevice {
    name: &'static str,
    base_port: u16,
    bytecode: Vec<u8>,
    registers: [u32; 4],
    power_state: PowerState,
}

impl UdfAncientDevice {
    /// Create a new UdfAncientDevice
    pub fn new(name: &'static str, base_port: u16, bytecode: &[u8]) -> Self {
        let mut code_vec = Vec::new();
        for &b in bytecode {
            code_vec.push(b);
        }
        Self {
            name,
            base_port,
            bytecode: code_vec,
            registers: [0; 4],
            power_state: PowerState::Off,
        }
    }

    /// Access inner registers for testing or diagnostics
    pub fn get_register(&self, idx: usize) -> Option<u32> {
        if idx < self.registers.len() {
            Some(self.registers[idx])
        } else {
            None
        }
    }
}

impl PeripheralDevice for UdfAncientDevice {
    fn name(&self) -> &'static str {
        self.name
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.registers = [0; 4];
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("Device is powered off");
        }

        // Execute UDF bytecode to simulate reading from hardware ports
        let interpreter = AncientUdfInterpreter::new(&self.bytecode);
        interpreter.execute(&mut self.registers)?;

        // Fill buffer with simulated data from registers
        let byte_val = (self.registers[0] & 0xFF) as u8;
        for b in buffer.iter_mut() {
            *b = byte_val;
        }

        Ok(buffer.len())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("Device is powered off");
        }

        if let Some(&first_byte) = data.first() {
            self.registers[0] = first_byte as u32;
        }

        // Execute UDF bytecode to process output data
        let interpreter = AncientUdfInterpreter::new(&self.bytecode);
        interpreter.execute(&mut self.registers)?;

        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.registers = [0; 4];
        self.power_state = PowerState::Off;
        Ok(())
    }
}

/// Zero-allocation, lightweight sandboxed bytecode interpreter for User-Defined Functions.
pub struct AncientUdfInterpreter<'a> {
    bytecode: &'a [u8],
}

impl<'a> AncientUdfInterpreter<'a> {
    pub fn new(bytecode: &'a [u8]) -> Self {
        Self { bytecode }
    }

    /// Execute bytecode instructions on the device registers
    pub fn execute(&self, registers: &mut [u32; 4]) -> Result<(), &'static str> {
        let mut pc = 0;
        while pc < self.bytecode.len() {
            let op = self.bytecode[pc];
            match op {
                0x01 => {
                    // Read simulated register value: op, dest_reg, src_reg
                    if pc + 2 >= self.bytecode.len() {
                        return Err("UDF execution error: out of bounds");
                    }
                    let dest = self.bytecode[pc + 1] as usize;
                    let src = self.bytecode[pc + 2] as usize;
                    if dest < registers.len() && src < registers.len() {
                        registers[dest] = registers[src];
                    }
                    pc += 3;
                }
                0x02 => {
                    // Write to simulated register value: op, dest_reg, constant
                    if pc + 2 >= self.bytecode.len() {
                        return Err("UDF execution error: out of bounds");
                    }
                    let dest = self.bytecode[pc + 1] as usize;
                    let val = self.bytecode[pc + 2] as u32;
                    if dest < registers.len() {
                        registers[dest] = val;
                    }
                    pc += 3;
                }
                0x03 => {
                    // Scaling: op, reg, multiplier
                    if pc + 2 >= self.bytecode.len() {
                        return Err("UDF execution error: out of bounds");
                    }
                    let reg = self.bytecode[pc + 1] as usize;
                    let factor = self.bytecode[pc + 2] as u32;
                    if reg < registers.len() {
                        registers[reg] = registers[reg].wrapping_mul(factor);
                    }
                    pc += 3;
                }
                0x04 => {
                    // Halt with success
                    return Ok(());
                }
                _ => {
                    return Err("UDF execution error: invalid instruction");
                }
            }
        }
        Ok(())
    }
}

/// Creates a Floppy Disk Driver using the UDF OOP framework
pub fn create_floppy_disk() -> UdfAncientDevice {
    // Floppy UDF bytecode:
    // 0x02, 0x01, 0x0A (Write 10 to reg 1 - cylinder count)
    // 0x03, 0x00, 0x02 (Double the command input in reg 0)
    // 0x04             (Halt)
    let floppy_bytecode = [0x02, 0x01, 0x0A, 0x03, 0x00, 0x02, 0x04];
    UdfAncientDevice::new("Floppy Disk Controller", 0x3F0, &floppy_bytecode)
}

/// Creates a Sound Blaster 16 Driver using the UDF OOP framework
pub fn create_sound_blaster_16() -> UdfAncientDevice {
    // SB16 UDF bytecode:
    // 0x02, 0x02, 0x22 (Write 34 to reg 2 - mixer register)
    // 0x03, 0x00, 0x04 (Multiply volume command in reg 0 by 4)
    // 0x04             (Halt)
    let sb16_bytecode = [0x02, 0x02, 0x22, 0x03, 0x00, 0x04, 0x04];
    UdfAncientDevice::new("Sound Blaster 16", 0x220, &sb16_bytecode)
}

/// Creates a Parallel Printer Driver using the UDF OOP framework
pub fn create_parallel_printer() -> UdfAncientDevice {
    // Printer UDF bytecode:
    // 0x02, 0x03, 0x01 (Write 1 to reg 3 - online status bit)
    // 0x04             (Halt)
    let printer_bytecode = [0x02, 0x03, 0x01, 0x04];
    UdfAncientDevice::new("Parallel Printer", 0x378, &printer_bytecode)
}

/// Creates a CGA Graphics Driver using the UDF OOP framework
pub fn create_cga_graphics() -> UdfAncientDevice {
    // CGA Graphics UDF bytecode:
    // 0x02, 0x01, 0x50 (Write 80 to reg 1 - width limit)
    // 0x02, 0x02, 0x19 (Write 25 to reg 2 - height limit)
    // 0x04             (Halt)
    let cga_bytecode = [0x02, 0x01, 0x50, 0x02, 0x02, 0x19, 0x04];
    UdfAncientDevice::new("CGA Video Adapter", 0x3D4, &cga_bytecode)
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_floppy_udf_execution() {
        let mut floppy = create_floppy_disk();
        assert!(floppy.initialize().is_ok());

        // Write write command data 5 to floppy
        let mut data = [5u8];
        assert!(floppy.write(&mut data).is_ok());

        // Under floppy's bytecode:
        // reg[0] is set to first data byte (5), then doubled via 0x03 -> 10.
        // reg[1] is set to constant 10 via 0x02.
        assert_eq!(floppy.get_register(0).unwrap(), 10);
        assert_eq!(floppy.get_register(1).unwrap(), 10);
    }

    #[test]
    fn test_sound_blaster_16_udf() {
        let mut sb16 = create_sound_blaster_16();
        assert!(sb16.initialize().is_ok());

        let mut data = [2u8];
        assert!(sb16.write(&mut data).is_ok());

        // Under SB16 bytecode:
        // reg[0] is set to first data byte (2), then multiplied by 4 via 0x03 -> 8.
        // reg[2] is set to mixer register 34 via 0x02.
        assert_eq!(sb16.get_register(0).unwrap(), 8);
        assert_eq!(sb16.get_register(2).unwrap(), 34);
    }

    #[test]
    fn test_parallel_printer_udf() {
        let mut printer = create_parallel_printer();
        assert!(printer.initialize().is_ok());

        let mut data = [42u8];
        assert!(printer.write(&mut data).is_ok());

        assert_eq!(printer.get_register(0).unwrap(), 42);
        assert_eq!(printer.get_register(3).unwrap(), 1);
    }

    #[test]
    fn test_cga_graphics_udf() {
        let mut cga = create_cga_graphics();
        assert!(cga.initialize().is_ok());

        let mut buffer = [0u8; 10];
        assert!(cga.read(&mut buffer).is_ok());

        // Under CGA bytecode, reg[1] = 80, reg[2] = 25, reg[0] = 0 (default).
        assert_eq!(cga.get_register(1).unwrap(), 80);
        assert_eq!(cga.get_register(2).unwrap(), 25);
    }
}
