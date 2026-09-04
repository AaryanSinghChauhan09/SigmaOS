// Toolchain Adapter Module
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolchainProfile {
    GNU,
    LLVM,
    Musl,
}

pub struct ToolchainAdapter {
    pub profile: ToolchainProfile,
    pub flags: HashMap<String, String>,
}

impl ToolchainAdapter {
    pub fn new(profile: ToolchainProfile) -> Self {
        let mut flags = HashMap::new();
        match profile {
            ToolchainProfile::GNU => {
                flags.insert("CC".to_string(), "gcc".to_string());
                flags.insert("CFLAGS".to_string(), "-O2 -Wall".to_string());
            }
            ToolchainProfile::LLVM => {
                flags.insert("CC".to_string(), "clang".to_string());
                flags.insert("CFLAGS".to_string(), "-O3 -flto".to_string());
            }
            ToolchainProfile::Musl => {
                flags.insert("CC".to_string(), "musl-gcc".to_string());
                flags.insert("CFLAGS".to_string(), "-O2 -static".to_string());
            }
        }
        ToolchainAdapter { profile, flags }
    }
}
