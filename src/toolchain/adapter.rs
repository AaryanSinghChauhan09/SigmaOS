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

// SigmaOS Ancient Compiler & Toolchain Support Adapter
// Wraps legacy compilation profiles (GCC 2.x, early LLVM, and assembly) natively without source patching

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolchainProfile {
    LegacyC,
    LegacyCpp,
    LegacyAssembly,
}

pub struct ToolchainAdapter {
    pub profile: ToolchainProfile,
    pub legacy_cc_path: String,
    pub include_libc5: bool,
}

impl ToolchainAdapter {
    pub fn new(profile: ToolchainProfile) -> Self {
        ToolchainAdapter {
            profile,
            legacy_cc_path: "/opt/sigma/toolchain/gcc-2.95/bin/gcc".to_string(),
            include_libc5: true,
        }
    }

    pub fn generate_compiler_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        flags.push("-fno-stack-protector".to_string());
        flags.push("-m32".to_string()); // Target 32-bit x86 for legacy compatibility
        match self.profile {
            ToolchainProfile::LegacyC => {
                flags.push("-std=gnu89".to_string()); // Enforce ANSI/ISO C90
                flags.push("-D__SIGMA_LEGACY_C__".to_string());
            }
            ToolchainProfile::LegacyCpp => {
                flags.push("-std=gnu++98".to_string()); // Enforce legacy C++98 standard
                flags.push("-fno-exceptions".to_string());
            }
            ToolchainProfile::LegacyAssembly => {
                flags.push("-felf32".to_string());
                flags.push("-D__NASM__".to_string());
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
}
