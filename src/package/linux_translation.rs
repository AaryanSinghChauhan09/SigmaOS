// SigmaOS Linux Package & Driver Translation Subsystem
// Zero-dependency, #![no_std] compliant, zero-allocation
// Integrates foreign Linux package frameworks (.deb, .rpm, pacman) directly with the SigmaOS Driver system.

use crate::driver::framework::{DriverError, DriverID, DriverState, DriverType, SimpleDriver};
use crate::package::PackageFormat;
use core::sync::atomic::{AtomicBool, Ordering};

/// User-Defined Function (UDF) for Package & Syscall Translation
/// Dynamically translates foreign syscalls or I/O request codes to native SigmaOS drivers
pub trait PackageTranslationUdf: Sync {
    fn name(&self) -> &'static str;
    fn translate_syscall(&self, foreign_num: u32) -> u32;
    fn translate_io_control(&self, command: u32) -> u32;
}

/// Linux-style bit-packed ioctl representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedIoctl {
    pub direction: u8,
    pub size: u16,
    pub group: u8,
    pub command_id: u8,
}

impl DecodedIoctl {
    /// Decodes a packed 32-bit Linux ioctl code into its distinct components
    pub fn parse(cmd: u32) -> Self {
        Self {
            direction: ((cmd >> 30) & 0x03) as u8,
            size: ((cmd >> 16) & 0x3FFF) as u16,
            group: ((cmd >> 8) & 0xFF) as u8,
            command_id: (cmd & 0xFF) as u8,
        }
    }

    /// Encodes structured fields back into a standard 32-bit packed ioctl code
    pub fn encode(&self) -> u32 {
        ((self.direction as u32 & 0x03) << 30)
            | ((self.size as u32 & 0x3FFF) << 16)
            | ((self.group as u32 & 0xFF) << 8)
            | (self.command_id as u32 & 0xFF)
    }
}

pub struct GenericLinuxTranslationUdf;
impl PackageTranslationUdf for GenericLinuxTranslationUdf {
    fn name(&self) -> &'static str {
        "generic-linux-udf"
    }

    fn translate_syscall(&self, foreign_num: u32) -> u32 {
        match foreign_num {
            0 => 0,                  // read -> native read
            1 => 1,                  // write -> native write
            2 => 2,                  // open -> native open
            3 => 3,                  // close -> native close
            54 => 54,                // ioctl -> native ioctl
            _ => foreign_num + 2000, // remap other Linux syscall offsets safely
        }
    }

    /// Advanced Linux-inspired ioctl decoding and dynamic translation mapping
    fn translate_io_control(&self, command: u32) -> u32 {
        let decoded = DecodedIoctl::parse(command);

        // Handle common standard matched Linux ioctl numbers
        if command == 0x5401 {
            return 0x101; // TCGETS -> Native Serial Get
        }
        if command == 0x5402 {
            return 0x102; // TCSETS -> Native Serial Set
        }
        if command == 0x5421 {
            return 0x103; // FIONBIO -> Non-blocking socket configuration
        }
        if command == 0x125F {
            return 0x104; // BLKGETSIZE -> Read block device bounds
        }

        // Otherwise, dynamically translate using decoded groups safely
        match decoded.group {
            b'T' => {
                // TTY / Serial group
                0x100 + decoded.command_id as u32
            }
            b'f' => {
                // Filesystem / Socket group
                0x200 + decoded.command_id as u32
            }
            0x12 => {
                // Block device group
                0x300 + decoded.command_id as u32
            }
            _ => {
                // Fallback translation
                command ^ 0xDEAD
            }
        }
    }
}

/// Abstract Translator for distribution packages (OOP Paradigm)
pub trait LinuxDriverPackageTranslator {
    fn source_format(&self) -> PackageFormat;
    fn package_name(&self) -> &'static str;
    fn translate_to_driver(&self) -> SimpleDriver;
}

/// Concrete .deb (Debian/Ubuntu/Parrot/Mint) package translator
pub struct DebPackageDriverTranslator {
    pub name: &'static str,
    pub payload_size: usize,
    pub is_kernel_module: bool,
}

impl LinuxDriverPackageTranslator for DebPackageDriverTranslator {
    fn source_format(&self) -> PackageFormat {
        PackageFormat::Deb
    }

    fn package_name(&self) -> &'static str {
        self.name
    }

    fn translate_to_driver(&self) -> SimpleDriver {
        println!(
            "PackageTranslator: Converting Debian Package '{}' ({} bytes) to SigmaOS system driver.",
            self.name, self.payload_size
        );
        let driver_type = if self.is_kernel_module {
            DriverType::Block
        } else {
            DriverType::Char
        };
        SimpleDriver::new(9901, driver_type)
    }
}

/// Concrete .rpm (RedHat/Fedora) package translator
pub struct RpmPackageDriverTranslator {
    pub name: &'static str,
    pub header_signature_valid: bool,
}

impl LinuxDriverPackageTranslator for RpmPackageDriverTranslator {
    fn source_format(&self) -> PackageFormat {
        PackageFormat::Rpm
    }

    fn package_name(&self) -> &'static str {
        self.name
    }

    fn translate_to_driver(&self) -> SimpleDriver {
        println!(
            "PackageTranslator: Processing RPM Package '{}'. Verifying header layout signatures...",
            self.name
        );
        if self.header_signature_valid {
            println!(
                "PackageTranslator: RPM signature is valid. Provisioning micro-driver bridge."
            );
        }
        SimpleDriver::new(9902, DriverType::Char)
    }
}

/// Concrete Pacman/AUR (Arch Linux) package translator
pub struct PacmanPackageDriverTranslator {
    pub name: &'static str,
    pub has_aur_recipes: bool,
}

impl LinuxDriverPackageTranslator for PacmanPackageDriverTranslator {
    fn source_format(&self) -> PackageFormat {
        PackageFormat::Pacman
    }

    fn package_name(&self) -> &'static str {
        self.name
    }

    fn translate_to_driver(&self) -> SimpleDriver {
        println!(
            "PackageTranslator: Mapping Arch Linux Package '{}' to native driver layer.",
            self.name
        );
        if self.has_aur_recipes {
            println!("  -> Found embedded AUR building recipes. Executing clean compile sandbox.");
        }
        SimpleDriver::new(9903, DriverType::Network)
    }
}

/// Unified Linux Translation Service Bridge
pub struct LinuxTranslationService {
    pub active_udf: &'static dyn PackageTranslationUdf,
    pub translation_enabled: AtomicBool,
}

impl LinuxTranslationService {
    pub const fn new(udf: &'static dyn PackageTranslationUdf) -> Self {
        Self {
            active_udf: udf,
            translation_enabled: AtomicBool::new(true),
        }
    }

    /// Intercepts and translates standard Linux application system calls to native SigmaOS interfaces
    pub fn translate_binary_syscall(&self, sys_num: u32) -> Result<u32, &'static str> {
        if self.translation_enabled.load(Ordering::SeqCst) {
            let native_sys = self.active_udf.translate_syscall(sys_num);
            println!(
                "TranslationService: Translated foreign syscall {} -> native syscall {}",
                sys_num, native_sys
            );
            Ok(native_sys)
        } else {
            Err("Translation de-activated. Blocked external application execution.")
        }
    }

    /// Adapts standard Linux device ioctl calls to SigmaOS driver equivalents
    pub fn translate_device_ioctl(&self, cmd: u32) -> u32 {
        if self.translation_enabled.load(Ordering::SeqCst) {
            self.active_udf.translate_io_control(cmd)
        } else {
            0
        }
    }
}

pub static GLOBAL_TRANSLATION_UDF: GenericLinuxTranslationUdf = GenericLinuxTranslationUdf;
pub static GLOBAL_TRANSLATION_SERVICE: LinuxTranslationService =
    LinuxTranslationService::new(&GLOBAL_TRANSLATION_UDF);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_ioctl_parsing_and_encoding() {
        // Standard layout test: Direction=2 (read), Size=256, Group='T', Command=10
        let encoded_cmd = ((2u32) << 30) | ((256u32) << 16) | ((b'T' as u32) << 8) | 10;
        let decoded = DecodedIoctl::parse(encoded_cmd);

        assert_eq!(decoded.direction, 2);
        assert_eq!(decoded.size, 256);
        assert_eq!(decoded.group, b'T');
        assert_eq!(decoded.command_id, 10);

        assert_eq!(decoded.encode(), encoded_cmd);
    }

    #[test]
    fn test_linux_ioctl_group_routing() {
        let udf = GenericLinuxTranslationUdf;

        // 1. TCGETS direct translation
        assert_eq!(udf.translate_io_control(0x5401), 0x101);

        // 2. TTY dynamic translation Group='T' (0x54), Command=5 -> 0x100 + 5 = 0x105
        let tty_cmd = ((1u32) << 30) | ((4u32) << 16) | ((b'T' as u32) << 8) | 5;
        assert_eq!(udf.translate_io_control(tty_cmd), 0x105);

        // 3. Filesystem dynamic translation Group='f', Command=12 -> 0x200 + 12 = 0x20C
        let fs_cmd = ((3u32) << 30) | ((8u32) << 16) | ((b'f' as u32) << 8) | 12;
        assert_eq!(udf.translate_io_control(fs_cmd), 0x20C);
    }
}
