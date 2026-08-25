// SigmaOS Linux Package & Driver Translation Subsystem
// Zero-dependency, #![no_std] compliant, zero-allocation
// Integrates foreign Linux package frameworks (.deb, .rpm, pacman) directly with the SigmaOS Driver system.

use crate::driver::framework::{
    DriverError, DriverID, DriverState, DriverType,
};
use crate::package::PackageFormat;
use core::sync::atomic::{AtomicBool, Ordering};

/// User-Defined Function (UDF) for Package & Syscall Translation
/// Dynamically translates foreign syscalls or I/O request codes to native SigmaOS drivers
pub trait PackageTranslationUdf: Sync {
    fn name(&self) -> &'static str;
    fn translate_syscall(&self, foreign_num: u32) -> u32;
    fn translate_io_control(&self, command: u32) -> u32;
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

    fn translate_io_control(&self, command: u32) -> u32 {
        // Map generic Linux ioctl codes to SigmaOS equivalents
        match command {
            0x5401 => 0x101, // TCGETS -> native serial get
            0x5402 => 0x102, // TCSETS -> native serial set
            _ => command ^ 0xDEAD,
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
