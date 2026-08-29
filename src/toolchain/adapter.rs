use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS Ancient Compiler & Toolchain Support Adapter
// Wraps legacy compilation profiles (GCC 2.x, early LLVM, and assembly) natively without source patching
// Enhanced with Gentoo / Clear Linux optimization matrices and Fedora / NixOS-style compiler hardening injections.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolchainProfile {
    LegacyC,
    LegacyCpp,
    LegacyAssembly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolchainOptProfile {
    None,
    Size,        // Minimal memory footprints (Debian/Ubuntu-style -Os)
    Performance, // standard distro optimizing (-O2 -mtune=generic)
    ClearLinux, // Clear Linux performance optimization (-O3 -march=native -ftree-vectorize -ffast-math)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolchainHardeningLevel {
    None,
    Standard, // standard Ubuntu/Fedora hardening (-fstack-protector-strong, -D_FORTIFY_SOURCE=2)
    NixOSHardened, // Fedora/NixOS strict defensive posture (-D_FORTIFY_SOURCE=3, -fPIE -pie, -Wl,-z,relro,-z,now)
}

pub struct ToolchainAdapter {
    pub profile: ToolchainProfile,
    pub legacy_cc_path: String,
    pub include_libc5: bool,
    pub opt_profile: ToolchainOptProfile,
    pub hardening_level: ToolchainHardeningLevel,
}

impl ToolchainAdapter {
    pub fn new(profile: ToolchainProfile) -> Self {
        ToolchainAdapter {
            profile,
            legacy_cc_path: "/opt/sigma/toolchain/gcc-2.95/bin/gcc".to_string(),
            include_libc5: true,
            opt_profile: ToolchainOptProfile::Performance,
            hardening_level: ToolchainHardeningLevel::Standard,
        }
    }

    pub fn with_optimization(mut self, opt: ToolchainOptProfile) -> Self {
        self.opt_profile = opt;
        self
    }

    pub fn with_hardening(mut self, hardening: ToolchainHardeningLevel) -> Self {
        self.hardening_level = hardening;
        self
    }

    pub fn generate_compiler_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        flags.push("-fno-stack-protector".to_string());
        flags.push("-m32".to_string());

        match self.profile {
            ToolchainProfile::LegacyC => {
                flags.push("-std=gnu89".to_string());
                flags.push("-D__SIGMA_LEGACY_C__".to_string());
            }
            ToolchainProfile::LegacyCpp => {
                flags.push("-std=gnu++98".to_string());
                flags.push("-fno-exceptions".to_string());
            }
            ToolchainProfile::LegacyAssembly => {
                flags.push("-felf32".to_string());
                flags.push("-D__NASM__".to_string());
            }
        }

        match self.opt_profile {
            ToolchainOptProfile::None => {
                flags.push("-O0".to_string());
            }
            ToolchainOptProfile::Size => {
                flags.push("-Os".to_string());
                flags.push("-ffunction-sections".to_string());
                flags.push("-fdata-sections".to_string());
                flags.push("-Wl,--gc-sections".to_string());
            }
            ToolchainOptProfile::Performance => {
                flags.push("-O2".to_string());
                flags.push("-mtune=generic".to_string());
            }
            ToolchainOptProfile::ClearLinux => {
                flags.push("-O3".to_string());
                flags.push("-march=native".to_string());
                flags.push("-mtune=native".to_string());
                flags.push("-ftree-vectorize".to_string());
                flags.push("-ffast-math".to_string());
                flags.push("-fno-semantic-interposition".to_string());
            }
        }

        match self.hardening_level {
            ToolchainHardeningLevel::None => {}
            ToolchainHardeningLevel::Standard => {
                flags.push("-fstack-protector-strong".to_string());
                flags.push("-D_FORTIFY_SOURCE=2".to_string());
                flags.push("-Wformat".to_string());
                flags.push("-Wformat-security".to_string());
            }
            ToolchainHardeningLevel::NixOSHardened => {
                flags.push("-fstack-protector-strong".to_string());
                flags.push("-D_FORTIFY_SOURCE=3".to_string());
                flags.push("-fstack-clash-protection".to_string());
                flags.push("-fPIE".to_string());
                flags.push("-pie".to_string());
                flags.push("-Wl,-z,relro".to_string());
                flags.push("-Wl,-z,now".to_string());
            }
        }

        flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolchain_flags_c90() {
        let adapter = ToolchainAdapter::new(ToolchainProfile::LegacyC);
        let flags = adapter.generate_compiler_flags();
        assert!(flags.contains(&"-std=gnu89".to_string()));
        assert!(flags.contains(&"-m32".to_string()));
    }

    #[test]
    fn test_toolchain_flags_cpp98() {
        let adapter = ToolchainAdapter::new(ToolchainProfile::LegacyCpp);
        let flags = adapter.generate_compiler_flags();
        assert!(flags.contains(&"-std=gnu++98".to_string()));
        assert!(flags.contains(&"-fno-exceptions".to_string()));
    }

    #[test]
    fn test_gentoo_optimization_matrix() {
        let adapter = ToolchainAdapter::new(ToolchainProfile::LegacyC)
            .with_optimization(ToolchainOptProfile::ClearLinux);
        let flags = adapter.generate_compiler_flags();
        assert!(flags.contains(&"-O3".to_string()));
        assert!(flags.contains(&"-march=native".to_string()));
        assert!(flags.contains(&"-ftree-vectorize".to_string()));
        assert!(flags.contains(&"-ffast-math".to_string()));
    }

    #[test]
    fn test_fedora_nixos_hardening_injections() {
        let adapter = ToolchainAdapter::new(ToolchainProfile::LegacyC)
            .with_hardening(ToolchainHardeningLevel::NixOSHardened);
        let flags = adapter.generate_compiler_flags();
        assert!(flags.contains(&"-D_FORTIFY_SOURCE=3".to_string()));
        assert!(flags.contains(&"-fPIE".to_string()));
        assert!(flags.contains(&"-pie".to_string()));
        assert!(flags.contains(&"-Wl,-z,now".to_string()));
    }
}
