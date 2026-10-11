// SigmaOS ACPI Machine Language (AML) Bytecode Interpreter
// Evaluates DSDT/SSDT control methods, power state transitions (S3/S4/S5), and hardware object trees.

#![allow(dead_code)]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Standard AML Opcodes
pub const AML_ZERO_OP: u8 = 0x00;
pub const AML_ONE_OP: u8 = 0x01;
pub const AML_NAME_OP: u8 = 0x08;
pub const AML_BYTE_PREFIX: u8 = 0x0A;
pub const AML_WORD_PREFIX: u8 = 0x0B;
pub const AML_DWORD_PREFIX: u8 = 0x0C;
pub const AML_STRING_PREFIX: u8 = 0x0D;
pub const AML_QWORD_PREFIX: u8 = 0x0E;
pub const AML_SCOPE_OP: u8 = 0x10;
pub const AML_BUFFER_OP: u8 = 0x11;
pub const AML_PACKAGE_OP: u8 = 0x12;
pub const AML_METHOD_OP: u8 = 0x14;
pub const AML_STORE_OP: u8 = 0x70;
pub const AML_ADD_OP: u8 = 0x72;
pub const AML_SUB_OP: u8 = 0x74;
pub const AML_RETURN_OP: u8 = 0xA4;

/// Sleep states supported by ACPI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcpiSleepState {
    #[default]
    S0Working,
    S1Sleeping,
    S3SuspendToRam,
    S4Hibernate,
    S5SoftOff,
}

/// AML Evaluated Value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmlValue {
    Uninitialized,
    Integer(u64),
    String(String),
    Buffer(Vec<u8>),
    Package(Vec<AmlValue>),
}

/// Dynamic AML Execution Context & Namespace Tree
#[derive(Debug, Clone, Default)]
pub struct AmlInterpreter {
    pub namespace: BTreeMap<String, AmlValue>,
    pub current_sleep_state: AcpiSleepState,
    pub pm1a_control_reg: u16,
    pub pm1b_control_reg: u16,
}

impl AmlInterpreter {
    pub fn new(pm1a_control_reg: u16, pm1b_control_reg: u16) -> Self {
        let mut interp = Self {
            namespace: BTreeMap::new(),
            current_sleep_state: AcpiSleepState::S0Working,
            pm1a_control_reg,
            pm1b_control_reg,
        };
        interp.namespace.insert(String::from("\\_OS"), AmlValue::String(String::from("SigmaOS")));
        interp.namespace.insert(String::from("\\_REV"), AmlValue::Integer(2));
        interp
    }

    /// Store a named object in the AML namespace
    pub fn define_name(&mut self, path: &str, val: AmlValue) {
        self.namespace.insert(String::from(path), val);
    }

    /// Resolve an object from the AML namespace
    pub fn resolve_name(&self, path: &str) -> Option<&AmlValue> {
        self.namespace.get(path)
    }

    /// Parse integer from AML stream
    fn parse_integer(bytecode: &[u8], offset: &mut usize) -> Option<u64> {
        if *offset >= bytecode.len() {
            return None;
        }
        let op = bytecode[*offset];
        *offset += 1;

        match op {
            AML_ZERO_OP => Some(0),
            AML_ONE_OP => Some(1),
            AML_BYTE_PREFIX => {
                if *offset < bytecode.len() {
                    let val = bytecode[*offset] as u64;
                    *offset += 1;
                    Some(val)
                } else {
                    None
                }
            }
            AML_WORD_PREFIX => {
                if *offset + 2 <= bytecode.len() {
                    let val = u16::from_le_bytes([bytecode[*offset], bytecode[*offset + 1]]) as u64;
                    *offset += 2;
                    Some(val)
                } else {
                    None
                }
            }
            AML_DWORD_PREFIX => {
                if *offset + 4 <= bytecode.len() {
                    let val = u32::from_le_bytes(bytecode[*offset..*offset + 4].try_into().ok()?) as u64;
                    *offset += 4;
                    Some(val)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Execute a bytecode slice containing a simple AML method
    pub fn execute_method(&mut self, bytecode: &[u8]) -> Result<AmlValue, &'static str> {
        let mut pc = 0;
        let mut last_result = AmlValue::Uninitialized;

        while pc < bytecode.len() {
            let op = bytecode[pc];
            pc += 1;

            match op {
                AML_ZERO_OP => last_result = AmlValue::Integer(0),
                AML_ONE_OP => last_result = AmlValue::Integer(1),
                AML_BYTE_PREFIX | AML_WORD_PREFIX | AML_DWORD_PREFIX => {
                    pc -= 1;
                    if let Some(int_val) = Self::parse_integer(bytecode, &mut pc) {
                        last_result = AmlValue::Integer(int_val);
                    }
                }
                AML_ADD_OP => {
                    let a = Self::parse_integer(bytecode, &mut pc).ok_or("Invalid add op 1")?;
                    let b = Self::parse_integer(bytecode, &mut pc).ok_or("Invalid add op 2")?;
                    last_result = AmlValue::Integer(a.wrapping_add(b));
                }
                AML_SUB_OP => {
                    let a = Self::parse_integer(bytecode, &mut pc).ok_or("Invalid sub op 1")?;
                    let b = Self::parse_integer(bytecode, &mut pc).ok_or("Invalid sub op 2")?;
                    last_result = AmlValue::Integer(a.saturating_sub(b));
                }
                AML_RETURN_OP => {
                    if let Some(int_val) = Self::parse_integer(bytecode, &mut pc) {
                        return Ok(AmlValue::Integer(int_val));
                    }
                    return Ok(last_result);
                }
                _ => {}
            }
        }

        Ok(last_result)
    }

    /// Transition to a sleep state (e.g. S3 suspend or S5 poweroff)
    pub fn transition_sleep_state(&mut self, target: AcpiSleepState) -> Result<u16, &'static str> {
        self.current_sleep_state = target;

        // Compute SLP_TYPa | SLP_EN bitmask
        let slp_typ = match target {
            AcpiSleepState::S0Working => 0x00,
            AcpiSleepState::S1Sleeping => 0x01,
            AcpiSleepState::S3SuspendToRam => 0x05,
            AcpiSleepState::S4Hibernate => 0x06,
            AcpiSleepState::S5SoftOff => 0x07,
        };

        let slp_en = 1u16 << 13;
        let pm1_val = (slp_typ << 10) | slp_en;
        Ok(pm1_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aml_interpreter_execution() {
        let mut interp = AmlInterpreter::new(0x404, 0x408);
        assert_eq!(
            interp.resolve_name("\\_OS"),
            Some(&AmlValue::String(String::from("SigmaOS")))
        );

        // Bytecode for ADD(10, 20) -> RETURN result
        let bytecode = [
            AML_ADD_OP,
            AML_BYTE_PREFIX, 10,
            AML_BYTE_PREFIX, 20,
            AML_RETURN_OP,
        ];

        let result = interp.execute_method(&bytecode).expect("Execution success");
        assert_eq!(result, AmlValue::Integer(30));

        let pm1 = interp
            .transition_sleep_state(AcpiSleepState::S3SuspendToRam)
            .expect("S3 transition");
        assert_ne!(pm1 & (1 << 13), 0); // SLP_EN bit set
    }
}
