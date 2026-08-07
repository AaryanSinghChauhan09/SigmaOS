#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Device Driver Environment (DDE) Translation Layer
// Universal driver support for Linux, Windows NDIS, and Wasm drivers

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    NotFound,
    AccessDenied,
    InvalidOperation,
    Timeout,
    HardwareError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Native,
    LinuxDde,
    WindowsNdis,
    Wasm,
    Udf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusType {
    Pci,
    Usb,
    Acpi,
    Cmos,
    I2c,
    Spi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeviceId {
    pub vendor_id: u16,
    pub product_id: u16,
    pub class_code: u8,
    pub subclass: u8,
}

impl DeviceId {
    pub fn new(vendor_id: u16, product_id: u16, class_code: u8, subclass: u8) -> Self {
        Self {
            vendor_id,
            product_id,
            class_code,
            subclass,
        }
    }

    pub fn matches(&self, other: &DeviceId) -> bool {
        self.vendor_id == other.vendor_id && self.product_id == other.product_id
    }
}

/// Unified Peripheral Trait - core abstraction for all drivers
pub trait UnifiedPeripheral {
    fn query_channel(&self) -> u32;
    fn read_byte(&mut self, offset: u32) -> Result<u8, DeviceError>;
    fn write_byte(&mut self, offset: u32, value: u8) -> Result<(), DeviceError>;
    fn get_device_id(&self) -> DeviceId;
    fn get_driver_type(&self) -> DriverType;
}

/// Generic Driver Implementation
pub struct GenericDriver {
    pub device_id: DeviceId,
    pub driver_type: DriverType,
    pub base_address: u32,
    pub enabled: bool,
}

impl GenericDriver {
    pub fn new(device_id: DeviceId, driver_type: DriverType, base_address: u32) -> Self {
        Self {
            device_id,
            driver_type,
            base_address,
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl UnifiedPeripheral for GenericDriver {
    fn query_channel(&self) -> u32 {
        self.base_address
    }

    fn read_byte(&mut self, _offset: u32) -> Result<u8, DeviceError> {
        if !self.enabled {
            return Err(DeviceError::AccessDenied);
        }
        Ok(0)
    }

    fn write_byte(&mut self, _offset: u32, _value: u8) -> Result<(), DeviceError> {
        if !self.enabled {
            return Err(DeviceError::AccessDenied);
        }
        Ok(())
    }

    fn get_device_id(&self) -> DeviceId {
        self.device_id
    }

    fn get_driver_type(&self) -> DriverType {
        self.driver_type
    }
}

/// Linux DDE Shim Layer
pub struct LinuxDdeShim {
    pub driver: GenericDriver,
    pub pci_registered: bool,
    pub irq_requested: bool,
}

impl LinuxDdeShim {
    pub fn new(device_id: DeviceId, base_address: u32) -> Self {
        Self {
            driver: GenericDriver::new(device_id, DriverType::LinuxDde, base_address),
            pci_registered: false,
            irq_requested: false,
        }
    }

    pub fn pci_register_driver(&mut self) -> Result<(), DeviceError> {
        self.pci_registered = true;
        self.driver.enable();
        Ok(())
    }

    pub fn request_irq(&mut self) -> Result<(), DeviceError> {
        if !self.pci_registered {
            return Err(DeviceError::InvalidOperation);
        }
        self.irq_requested = true;
        Ok(())
    }

    pub fn kmalloc(&self, size: usize) -> Result<usize, DeviceError> {
        Ok(size)
    }
}

/// Windows NDIS Wrapper
pub struct WindowsNdisWrapper {
    pub driver: GenericDriver,
    pub miniport_registered: bool,
}

impl WindowsNdisWrapper {
    pub fn new(device_id: DeviceId, base_address: u32) -> Self {
        Self {
            driver: GenericDriver::new(device_id, DriverType::WindowsNdis, base_address),
            miniport_registered: false,
        }
    }

    pub fn ndis_m_register_miniport_driver(&mut self) -> Result<(), DeviceError> {
        self.miniport_registered = true;
        self.driver.enable();
        Ok(())
    }

    pub fn ndis_allocate_memory(&self, size: usize) -> Result<usize, DeviceError> {
        Ok(size)
    }
}

/// Wasm Driver VM
pub struct WasmDriverVm {
    pub driver: GenericDriver,
    pub bytecode: Vec<u8>,
    pub memory: Vec<u8>,
}

impl WasmDriverVm {
    pub fn new(device_id: DeviceId, base_address: u32, bytecode: Vec<u8>) -> Self {
        Self {
            driver: GenericDriver::new(device_id, DriverType::Wasm, base_address),
            bytecode,
            memory: Vec::new(),
        }
    }

    pub fn load_bytecode(&mut self) -> Result<(), DeviceError> {
        self.driver.enable();
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), DeviceError> {
        if !self.driver.enabled {
            return Err(DeviceError::AccessDenied);
        }
        Ok(())
    }
}

/// UDF Bytecode Interpreter with advanced Linux/BSD/hardware-inspired CPU ALU model
pub struct UdfInterpreter {
    pub driver: GenericDriver,
    pub registers: [u64; 4], // R0, R1, R2, R3

    // CPU ALU status flag registers (ZF, SF, OF, CF)
    pub zf: bool, // Zero Flag
    pub sf: bool, // Sign Flag
    pub of: bool, // Overflow Flag
    pub cf: bool, // Carry Flag

    pub trace_log: Vec<String>,
}

impl UdfInterpreter {
    pub fn new(device_id: DeviceId, base_address: u32) -> Self {
        Self {
            driver: GenericDriver::new(device_id, DriverType::Udf, base_address),
            registers: [0; 4],
            zf: false,
            sf: false,
            of: false,
            cf: false,
            trace_log: Vec::new(),
        }
    }

    pub fn load_udf(&mut self, _bytecode: &[u8]) -> Result<(), DeviceError> {
        self.driver.enable();
        Ok(())
    }

    /// Update CPU ALU status flags based on operation result
    fn update_flags(&mut self, result: u64, is_signed_overflow: bool, is_carry_borrow: bool) {
        self.zf = result == 0;
        self.sf = (result as i64) < 0;
        self.of = is_signed_overflow;
        self.cf = is_carry_borrow;
    }

    /// Execute instruction with advanced CPU ALU logical/arithmetic pipelines
    pub fn execute_instruction(&mut self, opcode: u8, operands: &[u64]) -> Result<(), DeviceError> {
        if operands.is_empty() {
            return Err(DeviceError::InvalidOperation);
        }

        // Support backward compatible single operand (registers[0]) & multi-register addressing
        let (dest_idx, src_val) = if operands.len() >= 2 {
            let idx = operands[0] as usize;
            if idx >= 4 {
                return Err(DeviceError::InvalidOperation);
            }
            (idx, operands[1])
        } else {
            (0, operands[0])
        };

        match opcode {
            0x01 => {
                // MOV: dest = src
                self.registers[dest_idx] = src_val;
                self.update_flags(src_val, false, false);
                self.trace_log.push(format!(
                    "[ALU] MOV R{} = {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, src_val, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x02 => {
                // ADD: dest = dest + src
                let val1 = self.registers[dest_idx];
                let (res, carry) = val1.overflowing_add(src_val);

                // Signed overflow detection: same signs added resulting in different sign
                let signed_val1 = val1 as i64;
                let signed_val2 = src_val as i64;
                let signed_res = res as i64;
                let overflow = ((signed_val1 ^ signed_res) & (signed_val2 ^ signed_res)) < 0;

                self.registers[dest_idx] = res;
                self.update_flags(res, overflow, carry);
                self.trace_log.push(format!(
                    "[ALU] ADD R{} = {} + {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x03 => {
                // SUB: dest = dest - src
                let val1 = self.registers[dest_idx];
                let (res, borrow) = val1.overflowing_sub(src_val);

                // Signed overflow detection
                let signed_val1 = val1 as i64;
                let signed_val2 = src_val as i64;
                let signed_res = res as i64;
                let overflow = ((signed_val1 ^ signed_res) & (signed_val1 ^ signed_val2)) < 0;

                self.registers[dest_idx] = res;
                self.update_flags(res, overflow, borrow);
                self.trace_log.push(format!(
                    "[ALU] SUB R{} = {} - {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x04 => {
                // MUL: dest = dest * src
                let val1 = self.registers[dest_idx];
                let (res, carry) = val1.overflowing_mul(src_val);
                self.registers[dest_idx] = res;
                self.update_flags(res, carry, carry);
                self.trace_log.push(format!(
                    "[ALU] MUL R{} = {} * {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x05 => {
                // DIV: dest = dest / src
                if src_val == 0 {
                    return Err(DeviceError::InvalidOperation);
                }
                let val1 = self.registers[dest_idx];
                let res = val1 / src_val;
                self.registers[dest_idx] = res;
                self.update_flags(res, false, false);
                self.trace_log.push(format!(
                    "[ALU] DIV R{} = {} / {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x06 => {
                // AND: dest = dest & src
                let val1 = self.registers[dest_idx];
                let res = val1 & src_val;
                self.registers[dest_idx] = res;
                self.update_flags(res, false, false);
                self.trace_log.push(format!(
                    "[ALU] AND R{} = {} & {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x07 => {
                // OR: dest = dest | src
                let val1 = self.registers[dest_idx];
                let res = val1 | src_val;
                self.registers[dest_idx] = res;
                self.update_flags(res, false, false);
                self.trace_log.push(format!(
                    "[ALU] OR R{} = {} | {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x08 => {
                // XOR: dest = dest ^ src
                let val1 = self.registers[dest_idx];
                let res = val1 ^ src_val;
                self.registers[dest_idx] = res;
                self.update_flags(res, false, false);
                self.trace_log.push(format!(
                    "[ALU] XOR R{} = {} ^ {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x09 => {
                // SHL: dest = dest << src
                let val1 = self.registers[dest_idx];
                let shift = (src_val % 64) as u32;
                let res = val1 << shift;
                let carry = if shift > 0 {
                    ((val1 >> (64 - shift)) & 1) != 0
                } else {
                    false
                };
                self.registers[dest_idx] = res;
                self.update_flags(res, false, carry);
                self.trace_log.push(format!(
                    "[ALU] SHL R{} = {} << {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x0A => {
                // SHR: dest = dest >> src
                let val1 = self.registers[dest_idx];
                let shift = (src_val % 64) as u32;
                let res = val1 >> shift;
                let carry = if shift > 0 {
                    ((val1 >> (shift - 1)) & 1) != 0
                } else {
                    false
                };
                self.registers[dest_idx] = res;
                self.update_flags(res, false, carry);
                self.trace_log.push(format!(
                    "[ALU] SHR R{} = {} >> {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x0B => {
                // CMP: Compare dest with src, updating flags without storing result
                let val1 = self.registers[dest_idx];
                let (res, borrow) = val1.overflowing_sub(src_val);

                let signed_val1 = val1 as i64;
                let signed_val2 = src_val as i64;
                let signed_res = res as i64;
                let overflow = ((signed_val1 ^ signed_res) & (signed_val1 ^ signed_val2)) < 0;

                self.update_flags(res, overflow, borrow);
                self.trace_log.push(format!(
                    "[ALU] CMP R{} ({}) with {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x0C => {
                // SADD: Signed Saturation Addition (clamping to i64 bounds)
                let val1 = self.registers[dest_idx] as i64;
                let operand = src_val as i64;
                let (res, overflow) = val1.overflowing_add(operand);

                let sat_res = if overflow {
                    if val1 >= 0 {
                        i64::MAX
                    } else {
                        i64::MIN
                    }
                } else {
                    res
                };

                self.registers[dest_idx] = sat_res as u64;
                self.update_flags(sat_res as u64, overflow, overflow);
                self.trace_log.push(format!(
                    "[ALU] SADD R{} = {} +_sat {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, operand, sat_res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x0D => {
                // SSUB: Signed Saturation Subtraction
                let val1 = self.registers[dest_idx] as i64;
                let operand = src_val as i64;
                let (res, overflow) = val1.overflowing_sub(operand);

                let sat_res = if overflow {
                    if val1 >= 0 {
                        i64::MAX
                    } else {
                        i64::MIN
                    }
                } else {
                    res
                };

                self.registers[dest_idx] = sat_res as u64;
                self.update_flags(sat_res as u64, overflow, overflow);
                self.trace_log.push(format!(
                    "[ALU] SSUB R{} = {} -_sat {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, operand, sat_res, self.zf, self.sf, self.of, self.cf
                ));
            }
            0x0E => {
                // SAR: Arithmetic Shift Right (preserving sign)
                let val1 = self.registers[dest_idx] as i64;
                let shift = (src_val % 64) as u32;
                let res = val1 >> shift;
                let carry = if shift > 0 {
                    ((val1 >> (shift - 1)) & 1) != 0
                } else {
                    false
                };
                self.registers[dest_idx] = res as u64;
                self.update_flags(res as u64, false, carry);
                self.trace_log.push(format!(
                    "[ALU] SAR R{} = {} >>_sar {} -> {} (flags: ZF={}, SF={}, OF={}, CF={})",
                    dest_idx, val1, src_val, res, self.zf, self.sf, self.of, self.cf
                ));
            }
            _ => return Err(DeviceError::InvalidOperation),
        }
        Ok(())
    }
}

/// Hardware Auto-Negotiation Broker
pub struct HardwareBroker {
    pub drivers: BTreeMap<String, Box<dyn UnifiedPeripheral>>,
    pub device_database: BTreeMap<DeviceId, DriverType>,
}

impl HardwareBroker {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            drivers: BTreeMap::new(),
            device_database: BTreeMap::new(),
        }
    }

    pub fn register_device(&mut self, device_id: DeviceId, driver_type: DriverType) {
        self.device_database.insert(device_id, driver_type);
    }

    pub fn scan_bus(&self, _bus_type: BusType) -> Vec<DeviceId> {
        Vec::new()
    }

    pub fn match_driver(&self, device_id: &DeviceId) -> Option<DriverType> {
        self.device_database.get(device_id).copied()
    }

    pub fn load_driver(&mut self, name: String, driver: Box<dyn UnifiedPeripheral>) {
        self.drivers.insert(name, driver);
    }

    pub fn get_driver(&self, name: &str) -> Option<&Box<dyn UnifiedPeripheral>> {
        self.drivers.get(name)
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }
}

impl Default for HardwareBroker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id() {
        let id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        assert_eq!(id.vendor_id, 0x1234);
        assert!(id.matches(&id));
    }

    #[test]
    fn test_generic_driver() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let mut driver = GenericDriver::new(device_id, DriverType::Native, 0x1000);

        driver.enable();
        assert!(driver.enabled);

        let channel = driver.query_channel();
        assert_eq!(channel, 0x1000);
    }

    #[test]
    fn test_linux_dde_shim() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let mut shim = LinuxDdeShim::new(device_id, 0x1000);

        shim.pci_register_driver().unwrap();
        assert!(shim.pci_registered);

        shim.request_irq().unwrap();
        assert!(shim.irq_requested);

        let size = shim.kmalloc(1024).unwrap();
        assert_eq!(size, 1024);
    }

    #[test]
    fn test_linux_dde_order() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let mut shim = LinuxDdeShim::new(device_id, 0x1000);

        assert!(shim.request_irq().is_err());
    }

    #[test]
    fn test_windows_ndis() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let mut wrapper = WindowsNdisWrapper::new(device_id, 0x1000);

        wrapper.ndis_m_register_miniport_driver().unwrap();
        assert!(wrapper.miniport_registered);

        let size = wrapper.ndis_allocate_memory(512).unwrap();
        assert_eq!(size, 512);
    }

    #[test]
    fn test_wasm_driver_vm() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let bytecode = vec![0x01, 0x02, 0x03];
        let mut vm = WasmDriverVm::new(device_id, 0x1000, bytecode);

        vm.load_bytecode().unwrap();
        vm.execute().unwrap();
    }

    #[test]
    fn test_udf_interpreter() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let mut interpreter = UdfInterpreter::new(device_id, 0x1000);

        interpreter.load_udf(&[0x01, 0x02]).unwrap();
        interpreter.execute_instruction(0x01, &[100]).unwrap();
        assert_eq!(interpreter.registers[0], 100);

        interpreter.execute_instruction(0x02, &[50]).unwrap();
        assert_eq!(interpreter.registers[0], 150);
    }

    #[test]
    fn test_alu_arithmetic_flags() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let mut interpreter = UdfInterpreter::new(device_id, 0x1000);

        // 1. Test Zero Flag (ZF) on MOV 0
        interpreter.execute_instruction(0x01, &[0, 0]).unwrap();
        assert_eq!(interpreter.registers[0], 0);
        assert!(interpreter.zf);

        // 2. Test Sign Flag (SF) on MOV -5
        let val_neg_5 = -5i64 as u64;
        interpreter
            .execute_instruction(0x01, &[0, val_neg_5])
            .unwrap();
        assert_eq!(interpreter.registers[0], val_neg_5);
        assert!(interpreter.sf);

        // 3. Test Carry Flag (CF) on addition overflow: u64::MAX + 1
        interpreter
            .execute_instruction(0x01, &[1, u64::MAX])
            .unwrap();
        interpreter.execute_instruction(0x02, &[1, 1]).unwrap(); // R1 = R1 + 1
        assert_eq!(interpreter.registers[1], 0);
        assert!(interpreter.cf);
        assert!(interpreter.zf);

        // 4. Test signed overflow flag (OF): i64::MAX + 1 -> negative
        interpreter
            .execute_instruction(0x01, &[2, i64::MAX as u64])
            .unwrap();
        interpreter.execute_instruction(0x02, &[2, 1]).unwrap(); // R2 = R2 + 1
        assert_eq!(interpreter.registers[2], i64::MIN as u64);
        assert!(interpreter.of);
        assert!(interpreter.sf);
    }

    #[test]
    fn test_alu_bitwise_and_shifts() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let mut interpreter = UdfInterpreter::new(device_id, 0x1000);

        // XOR R0 with itself
        interpreter.execute_instruction(0x01, &[0, 0xAA55]).unwrap();
        interpreter.execute_instruction(0x08, &[0, 0xAA55]).unwrap(); // R0 = R0 ^ 0xAA55
        assert_eq!(interpreter.registers[0], 0);
        assert!(interpreter.zf);

        // SHL / SHR shift bit checks
        interpreter.execute_instruction(0x01, &[1, 1]).unwrap();
        interpreter.execute_instruction(0x09, &[1, 4]).unwrap(); // R1 = R1 << 4
        assert_eq!(interpreter.registers[1], 16);

        // SAR (Arithmetic Shift Right) checks
        let signed_neg_16 = -16i64 as u64;
        interpreter
            .execute_instruction(0x01, &[2, signed_neg_16])
            .unwrap();
        interpreter.execute_instruction(0x0E, &[2, 2]).unwrap(); // R2 = R2 >>_sar 2
        assert_eq!(interpreter.registers[2] as i64, -4);
    }

    #[test]
    fn test_alu_saturation_clamping() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let mut interpreter = UdfInterpreter::new(device_id, 0x1000);

        // SADD i64::MAX + 10 -> clamp to i64::MAX
        interpreter
            .execute_instruction(0x01, &[0, i64::MAX as u64])
            .unwrap();
        interpreter.execute_instruction(0x0C, &[0, 10]).unwrap(); // R0 = SADD R0, 10
        assert_eq!(interpreter.registers[0] as i64, i64::MAX);
        assert!(interpreter.of);

        // SSUB i64::MIN - 10 -> clamp to i64::MIN
        interpreter
            .execute_instruction(0x01, &[1, i64::MIN as u64])
            .unwrap();
        interpreter.execute_instruction(0x0D, &[1, 10]).unwrap(); // R1 = SSUB R1, 10
        assert_eq!(interpreter.registers[1] as i64, i64::MIN);
        assert!(interpreter.of);
    }

    #[test]
    fn test_alu_compare_and_trace_log() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let mut interpreter = UdfInterpreter::new(device_id, 0x1000);

        interpreter.execute_instruction(0x01, &[0, 100]).unwrap();
        interpreter.execute_instruction(0x0B, &[0, 50]).unwrap(); // CMP R0, 50

        // CMP must not store subtraction result back
        assert_eq!(interpreter.registers[0], 100);
        assert!(!interpreter.zf);
        assert!(!interpreter.sf);

        // Check trace logs
        assert!(interpreter.trace_log.len() >= 2);
        assert!(interpreter.trace_log[1].contains("CMP"));
    }

    #[test]
    fn test_hardware_broker() {
        let mut broker = HardwareBroker::new();
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);

        broker.register_device(device_id, DriverType::Native);
        let matched = broker.match_driver(&device_id);
        assert_eq!(matched, Some(DriverType::Native));
    }

    #[test]
    fn test_unified_peripheral_trait() {
        let device_id = DeviceId::new(0x1234, 0x5678, 0x01, 0x02);
        let driver: Box<dyn UnifiedPeripheral> =
            Box::new(GenericDriver::new(device_id, DriverType::Native, 0x1000));

        let id = driver.get_device_id();
        assert_eq!(id.vendor_id, 0x1234);
    }
}
