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
