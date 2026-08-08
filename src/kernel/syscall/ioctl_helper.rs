<<<<<<< HEAD
/// Linux and BSD-grade ioctl (Input/Output Control) Command Translation and Security Subsystem

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoctlDirection {
    None = 0,
    Read = 1,
    Write = 2,
    ReadWrite = 3,
}

/// Represents a decoded Unix/BSD-grade 32-bit ioctl command request.
/// Structure (Linux style):
/// - Bits 0-7:   Command number/sequence (8 bits)
/// - Bits 8-15:  Device type / magic group (8 bits)
/// - Bits 16-29: Parameter size (14 bits)
/// - Bits 30-31: Transfer direction (2 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoctlCommand {
    pub raw_request: u32,
    pub direction: IoctlDirection,
    pub size: usize,
    pub group_type: u8,
    pub sequence_number: u8,
}

impl IoctlCommand {
    /// Decodes a raw 32-bit ioctl integer into its semantic parameters
    pub fn decode(request: u32) -> Self {
        let sequence_number = (request & 0xFF) as u8;
        let group_type = ((request >> 8) & 0xFF) as u8;
        let size = ((request >> 16) & 0x3FFF) as usize;
        let dir_bits = (request >> 30) & 0x03;

        let direction = match dir_bits {
            1 => IoctlDirection::Read,
            2 => IoctlDirection::Write,
            3 => IoctlDirection::ReadWrite,
            _ => IoctlDirection::None,
        };

        IoctlCommand {
            raw_request: request,
            direction,
            size,
            group_type,
            sequence_number,
        }
    }

    /// Encodes semantic parameters into a raw 32-bit ioctl integer (equivalent to Unix _IOC macros)
    pub fn encode(direction: IoctlDirection, group_type: u8, sequence_number: u8, size: usize) -> u32 {
        let dir_bits = direction as u32;
        let size_bits = (size & 0x3FFF) as u32;
        let group_bits = group_type as u32;
        let seq_bits = sequence_number as u32;

        (dir_bits << 30) | (size_bits << 16) | (group_bits << 8) | seq_bits
    }
}

/// Security Boundary Validation Helper.
/// Prevents unprivileged/malicious user space tasks from triggering buffer overflows or out-of-bounds pointer reads/writes.
pub fn validate_ioctl_buffer(
    command: &IoctlCommand,
    arg_ptr: usize,
    user_memory_limit: usize,
) -> Result<(), &'static str> {
    if command.direction != IoctlDirection::None && command.size > 0 {
        // Prevent pointer overflow
        let end_address = arg_ptr.checked_add(command.size).ok_or("Pointer overflow")?;
        if end_address > user_memory_limit {
            return Err("Security Violation: ioctl parameter buffer exceeds user memory bounds");
        }
        if arg_ptr == 0 {
            return Err("Security Violation: null pointer passed to ioctl");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioctl_encoding_decoding() {
        // Replicate Linux standard command definition: _IOR('k', 5, u32)
        // Group: 'k' (107), Sequence: 5, Size: 4 (u32), Direction: Read (1)
        let raw = IoctlCommand::encode(IoctlDirection::Read, b'k', 5, 4);

        let decoded = IoctlCommand::decode(raw);
        assert_eq!(decoded.direction, IoctlDirection::Read);
        assert_eq!(decoded.group_type, b'k');
        assert_eq!(decoded.sequence_number, 5);
        assert_eq!(decoded.size, 4);
    }

    #[test]
    fn test_ioctl_bounds_checking() {
        let cmd = IoctlCommand::decode(IoctlCommand::encode(IoctlDirection::ReadWrite, b'x', 1, 64));

        // Safe buffer location
        assert!(validate_ioctl_buffer(&cmd, 0x10000, 0x100000).is_ok());

        // Malicious/exceeding buffer location
        let result = validate_ioctl_buffer(&cmd, 0x100000 - 10, 0x100000);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Security Violation: ioctl parameter buffer exceeds user memory bounds");

        // Null pointer check
        let null_result = validate_ioctl_buffer(&cmd, 0, 0x100000);
        assert!(null_result.is_err());
    }
}
||||||| 43be3a7e8
=======
// Unix/BSD-grade ioctl (Input/Output Control) syscall helper and decoder
// Decodes direction, size, group-type, and sequence parameters for hardware control calls.

#![no_std]

/// Direction bitmasks for standard UNIX/BSD ioctl commands
pub const IOC_VOID: u32  = 0x20000000; // No parameters, purely action command
pub const IOC_OUT: u32   = 0x40000000; // Copy out parameters to userspace
pub const IOC_IN: u32    = 0x80000000; // Copy in parameters from userspace
pub const IOC_INOUT: u32 = 0xC0000000; // Bidirectional data transfer

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoctlDirection {
    Void,
    In,
    Out,
    InOut,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DecodedIoctl {
    pub direction: IoctlDirection,
    pub parameter_size: usize,
    pub group_type: char,
    pub sequence_number: u8,
}

pub struct IoctlDecoder;

impl IoctlDecoder {
    /// Decodes a 32-bit ioctl command code following the standard UNIX/BSD bitfield encoding:
    /// - Bits 30-31: Direction (IOC_IN, IOC_OUT)
    /// - Bits 16-29: Parameter size in bytes (14 bits)
    /// - Bits 8-15: Group type (character byte, e.g. 'T' for tty)
    /// - Bits 0-7: Command sequence number
    pub fn decode_command(cmd: u32) -> DecodedIoctl {
        let dir_bits = cmd & 0xC0000000;
        let direction = match dir_bits {
            0x20000000 => IoctlDirection::Void,
            0x40000000 => IoctlDirection::Out,
            0x80000000 => IoctlDirection::In,
            0xC0000000 => IoctlDirection::InOut,
            _ => IoctlDirection::Unknown,
        };

        let parameter_size = ((cmd >> 16) & 0x3FFF) as usize;
        let group_char_byte = ((cmd >> 8) & 0xFF) as u8;
        let group_type = group_char_byte as char;
        let sequence_number = (cmd & 0xFF) as u8;

        DecodedIoctl {
            direction,
            parameter_size,
            group_type,
            sequence_number,
        }
    }

    /// Enforces memory boundaries: verifies the user-supplied pointer matches decoded size constraints.
    pub fn validate_parameter_bounds(cmd: u32, arg_ptr: usize, memory_limit: usize) -> Result<(), &'static str> {
        let decoded = Self::decode_command(cmd);
        if decoded.parameter_size > 0 {
            if arg_ptr == 0 {
                return Err("SYS_IOCTL_FAULT: Null parameter pointer passed");
            }
            // Simple overflow verification
            let end_addr = arg_ptr.checked_add(decoded.parameter_size).ok_or("SYS_IOCTL_FAULT: Pointer bounds overflow")?;
            if end_addr > memory_limit {
                return Err("SYS_IOCTL_FAULT: Parameter exceeds safe memory space limits");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioctl_direction_decoding() {
        // Encode a typical 'IOC_IN' command: 'T' group (0x54), sequence 42, parameter size 256
        // cmd = IOC_IN | (256 << 16) | (0x54 << 8) | 42
        let cmd = IOC_IN | (256 << 16) | (0x54 << 8) | 42;
        let decoded = IoctlDecoder::decode_command(cmd);

        assert_eq!(decoded.direction, IoctlDirection::In);
        assert_eq!(decoded.parameter_size, 256);
        assert_eq!(decoded.group_type, 'T');
        assert_eq!(decoded.sequence_number, 42);
    }

    #[test]
    fn test_ioctl_void_decoding() {
        // Encode a typical 'IOC_VOID' action command
        let cmd = IOC_VOID | (0 << 16) | (0x4E << 8) | 12; // 'N' group, seq 12, size 0
        let decoded = IoctlDecoder::decode_command(cmd);

        assert_eq!(decoded.direction, IoctlDirection::Void);
        assert_eq!(decoded.parameter_size, 0);
        assert_eq!(decoded.group_type, 'N');
        assert_eq!(decoded.sequence_number, 12);
    }

    #[test]
    fn test_ioctl_pointer_bounds_validations() {
        let cmd = IOC_OUT | (16 << 16) | (0x57 << 8) | 1; // size 16

        // 1. Valid pointer -> success
        assert!(IoctlDecoder::validate_parameter_bounds(cmd, 0x1000, 0x5000).is_ok());

        // 2. Null pointer -> fails
        assert!(IoctlDecoder::validate_parameter_bounds(cmd, 0, 0x5000).is_err());

        // 3. Pointer exceeds memory boundaries -> fails
        assert!(IoctlDecoder::validate_parameter_bounds(cmd, 0x4FFF, 0x5000).is_err());
    }
}
>>>>>>> origin/fix/mem-leak-custom-vec-drop-7188808108065826003
