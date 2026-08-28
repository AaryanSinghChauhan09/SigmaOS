//! ARM Thumb BX (Branch and Exchange) Instruction Decoder & Bytecode Data Word Formatter Subsystem
//! Implements ARM Thumb Mode BX/BLX instruction decoding, standardized data word encodings (Byte, Word, Dword, Qword, Oword),
//! and bytecode encryption primitives inspired by Linux & BSD kernel disassemblers/executors.
extern crate alloc;

use alloc::vec::Vec;

// ==========================================
// 1. ARM BX (Branch & Exchange) Instruction Decoder
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmExecutionState {
    ArmMode = 0,    // 32-bit ARM instruction set
    ThumbMode = 1,  // 16-bit / 32-bit Thumb-2 instruction set
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchExchangeType {
    BxReg,   // BX Rm (Branch and Exchange)
    BlxReg,  // BLX Rm (Branch, Link, and Exchange)
    Invalid,
}

#[derive(Debug, Clone)]
pub struct DecodedBxInstruction {
    pub instruction_type: BranchExchangeType,
    pub target_register: u8,
    pub target_address: u32,
    pub next_execution_state: ArmExecutionState,
}

pub struct ArmBxBranchExchangeDecoder;

impl ArmBxBranchExchangeDecoder {
    /// Decodes an ARM 32-bit opcode for BX / BLX instruction encoding (`0x012FFF10` / `0x012FFF30`)
    pub fn decode_arm_bx(opcode: u32, current_reg_value: u32) -> DecodedBxInstruction {
        // BX Rm opcode mask: Condition(4) 0001 0010 1111 1111 1111 0001 Rm(4) -> 0x012FFF10
        // BLX Rm opcode mask: Condition(4) 0001 0010 1111 1111 1111 0011 Rm(4) -> 0x012FFF30
        let base_mask = opcode & 0x0FFFFF00;
        let sub_mask = opcode & 0x0FFFFFF0;
        let reg = (opcode & 0x0F) as u8;

        let instruction_type = match sub_mask {
            0x012FFF10 => BranchExchangeType::BxReg,
            0x012FFF30 => BranchExchangeType::BlxReg,
            _ => if base_mask == 0x012FFF10 || base_mask == 0x012FFF00 {
                BranchExchangeType::BxReg
            } else if base_mask == 0x012FFF30 {
                BranchExchangeType::BlxReg
            } else {
                BranchExchangeType::Invalid
            },
        };

        // If the least significant bit (LSB) of target address is 1, switch to Thumb mode; otherwise ARM mode
        let next_execution_state = if (current_reg_value & 1) != 0 {
            ArmExecutionState::ThumbMode
        } else {
            ArmExecutionState::ArmMode
        };

        let target_address = current_reg_value & !1u32; // Align address to 2-byte or 4-byte boundary

        DecodedBxInstruction {
            instruction_type,
            target_register: reg,
            target_address,
            next_execution_state,
        }
    }
}

// ==========================================
// 2. Data Word Format Standardizer (Byte, Word, Dword, Qword, Oword)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataEndianness {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordWidth {
    Byte = 1,   // 8-bit
    Word = 2,   // 16-bit
    Dword = 4,  // 32-bit
    Qword = 8,  // 64-bit
    Oword = 16, // 128-bit
}

pub struct SovereignDataWordFormatter;

impl SovereignDataWordFormatter {
    /// Formats a 16-bit Word into bytes with target endianness
    pub fn format_word(value: u16, endianness: DataEndianness) -> [u8; 2] {
        match endianness {
            DataEndianness::LittleEndian => value.to_le_bytes(),
            DataEndianness::BigEndian => value.to_be_bytes(),
        }
    }

    /// Formats a 32-bit Dword into bytes with target endianness
    pub fn format_dword(value: u32, endianness: DataEndianness) -> [u8; 4] {
        match endianness {
            DataEndianness::LittleEndian => value.to_le_bytes(),
            DataEndianness::BigEndian => value.to_be_bytes(),
        }
    }

    /// Formats a 64-bit Qword into bytes with target endianness
    pub fn format_qword(value: u64, endianness: DataEndianness) -> [u8; 8] {
        match endianness {
            DataEndianness::LittleEndian => value.to_le_bytes(),
            DataEndianness::BigEndian => value.to_be_bytes(),
        }
    }

    /// Parses bytes into a 32-bit Dword with target endianness
    pub fn parse_dword(bytes: &[u8; 4], endianness: DataEndianness) -> u32 {
        match endianness {
            DataEndianness::LittleEndian => u32::from_le_bytes(*bytes),
            DataEndianness::BigEndian => u32::from_be_bytes(*bytes),
        }
    }

    /// Parses bytes into a 64-bit Qword with target endianness
    pub fn parse_qword(bytes: &[u8; 8], endianness: DataEndianness) -> u64 {
        match endianness {
            DataEndianness::LittleEndian => u64::from_le_bytes(*bytes),
            DataEndianness::BigEndian => u64::from_be_bytes(*bytes),
        }
    }
}

// ==========================================
// 3. Bytecode Encryption & Stream Masking Engine
// ==========================================

pub struct SovereignBytecodeEncryptor;

impl SovereignBytecodeEncryptor {
    /// Encrypts/decrypts a bytecode payload using rolling XOR mask stream cipher
    pub fn process_bytecode(payload: &[u8], key: &[u8]) -> Vec<u8> {
        if key.is_empty() {
            return payload.to_vec();
        }

        let mut output = Vec::with_capacity(payload.len());
        let mut rolling_state = 0x517cc1b727220a95u64;

        for (idx, &byte) in payload.iter().enumerate() {
            let key_byte = key[idx % key.len()];
            rolling_state = rolling_state.wrapping_mul(6364136223846793005).wrapping_add(key_byte as u64);
            let mask = (rolling_state >> 24) as u8 ^ key_byte;
            output.push(byte ^ mask);
        }

        output
    }
}

// ==========================================
// 4. Integration Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arm_bx_decoding() {
        // BX R1 instruction opcode: 0xE12FFF11 (Condition AL, BX, Rm=1)
        // R1 holds 0x00010001 (LSB is 1 -> Thumb mode transition to 0x00010000)
        let decoded_bx = ArmBxBranchExchangeDecoder::decode_arm_bx(0xE12FFF11, 0x00010001);
        assert_eq!(decoded_bx.instruction_type, BranchExchangeType::BxReg);
        assert_eq!(decoded_bx.target_register, 1);
        assert_eq!(decoded_bx.target_address, 0x00010000);
        assert_eq!(decoded_bx.next_execution_state, ArmExecutionState::ThumbMode);

        // BLX R3 instruction opcode: 0xE12FFF33 (Condition AL, BLX, Rm=3)
        // R3 holds 0x00020000 (LSB is 0 -> ARM mode transition to 0x00020000)
        let decoded_blx = ArmBxBranchExchangeDecoder::decode_arm_bx(0xE12FFF33, 0x00020000);
        assert_eq!(decoded_blx.instruction_type, BranchExchangeType::BlxReg);
        assert_eq!(decoded_blx.target_register, 3);
        assert_eq!(decoded_blx.target_address, 0x00020000);
        assert_eq!(decoded_blx.next_execution_state, ArmExecutionState::ArmMode);
    }

    #[test]
    fn test_word_dword_qword_formatter() {
        let val_32: u32 = 0x12345678;
        let le_bytes = SovereignDataWordFormatter::format_dword(val_32, DataEndianness::LittleEndian);
        assert_eq!(le_bytes, [0x78, 0x56, 0x34, 0x12]);

        let parsed_32 = SovereignDataWordFormatter::parse_dword(&le_bytes, DataEndianness::LittleEndian);
        assert_eq!(parsed_32, val_32);

        let val_64: u64 = 0x1122334455667788;
        let be_bytes = SovereignDataWordFormatter::format_qword(val_64, DataEndianness::BigEndian);
        assert_eq!(be_bytes, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);

        let parsed_64 = SovereignDataWordFormatter::parse_qword(&be_bytes, DataEndianness::BigEndian);
        assert_eq!(parsed_64, val_64);
    }

    #[test]
    fn test_bytecode_encryption() {
        let bytecode = b"\x7F\x45\x4C\x46\x02\x01\x01\x00"; // ELF header magic bytes
        let key = b"sovereign_key";

        let encrypted = SovereignBytecodeEncryptor::process_bytecode(bytecode, key);
        assert_ne!(encrypted.as_slice(), bytecode);

        let decrypted = SovereignBytecodeEncryptor::process_bytecode(&encrypted, key);
        assert_eq!(decrypted.as_slice(), bytecode);
    }
}
