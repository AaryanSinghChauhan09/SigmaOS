// SigmaOS Ancient Build Replay Capsules (BuildCapsule)
// Encapsulates legacy build environments (GCC 2.x, libc5) to compile ancient source code natively without patching

extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsuleProfile {
    LegacyCReplay,
    LegacyCppReplay,
    LegacyAsmReplay,
}

pub struct BuildCapsule {
    pub profile: CapsuleProfile,
    pub capsule_sysroot: String,
    pub is_isolated_env: bool,
}

impl BuildCapsule {
    pub fn new(profile: CapsuleProfile) -> Self {
        BuildCapsule {
            profile,
            capsule_sysroot: "/opt/sigma/capsules/sysroot-libc5".to_string(),
            is_isolated_env: true,
        }
    }

    pub fn prepare_replay_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        args.push(format!("--sysroot={}", self.capsule_sysroot));
        args.push("-nostdinc".to_string());
        match self.profile {
            CapsuleProfile::LegacyCReplay => {
                args.push("-D__LIBC5__".to_string());
                args.push("-D__GCC2__".to_string());
            }
            CapsuleProfile::LegacyCppReplay => {
                args.push("-D__LIBC5__".to_string());
                args.push("-traditional-cpp".to_string());
            }
            CapsuleProfile::LegacyAsmReplay => {
                args.push("-felf".to_string());
            }
        }
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_capsule_flags() {
        let capsule = BuildCapsule::new(CapsuleProfile::LegacyCReplay);
        let args = capsule.prepare_replay_args();
        assert!(args.contains(&"--sysroot=/opt/sigma/capsules/sysroot-libc5".to_string()));
        assert!(args.contains(&"-D__LIBC5__".to_string()));
        assert!(args.contains(&"-nostdinc".to_string()));
    }
}
