#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// CPU Instruction Set Architecture (ISA) Level Assessor & Vectorized Router
// Inspired by CachyOS hardware-specific optimizations, directing performance-optimal memory copying.


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IsaLevel {
    V1 = 1, // Generic x86-64 baseline
    V2 = 2, // Adds SSE4.2, SSSE3, Popcnt
    V3 = 3, // Adds AVX2, FMA3, BMI1/2
    V4 = 4, // Adds AVX-512, foundation vectors
}

/// Dynamic Assessor matching CachyOS hardware compilation levels
pub struct CpuIsaAssessor {
    pub level: IsaLevel,
    pub has_sse4_2: bool,
    pub has_avx2: bool,
    pub has_avx512: bool,
    pub has_fma3: bool,
}

impl CpuIsaAssessor {
    /// Initialize assessor and auto-detect ISA Level based on CPU flags
    pub fn new(has_sse4_2: bool, has_avx2: bool, has_avx512: bool, has_fma3: bool) -> Self {
        let level = if has_avx512 {
            IsaLevel::V4
        } else if has_avx2 && has_fma3 {
            IsaLevel::V3
        } else if has_sse4_2 {
            IsaLevel::V2
        } else {
            IsaLevel::V1
        };

        Self {
            level,
            has_sse4_2,
            has_avx2,
            has_avx512,
            has_fma3,
        }
    }

    /// High-performance memory copy. Routes to the fastest vectorization lanes based on the detected ISA level.
    pub fn direct_memcpy(&self, dest: &mut [u8], src: &[u8]) {
        let len = dest.len().min(src.len());
        if len == 0 {
            return;
        }

        match self.level {
            IsaLevel::V4 => {
                // Simulates AVX-512 (512-bit / 64-byte vector chunks copy)
                let chunk_size = 64;
                let mut i = 0;
                while i + chunk_size <= len {
                    dest[i..i + chunk_size].copy_from_slice(&src[i..i + chunk_size]);
                    i += chunk_size;
                }
                // Copy trailing bytes
                if i < len {
                    dest[i..len].copy_from_slice(&src[i..len]);
                }
            }
            IsaLevel::V3 => {
                // Simulates AVX2 (256-bit / 32-byte vector chunks copy)
                let chunk_size = 32;
                let mut i = 0;
                while i + chunk_size <= len {
                    dest[i..i + chunk_size].copy_from_slice(&src[i..i + chunk_size]);
                    i += chunk_size;
                }
                if i < len {
                    dest[i..len].copy_from_slice(&src[i..len]);
                }
            }
            IsaLevel::V2 => {
                // Simulates SSE4.2 (128-bit / 16-byte vector chunks copy)
                let chunk_size = 16;
                let mut i = 0;
                while i + chunk_size <= len {
                    dest[i..i + chunk_size].copy_from_slice(&src[i..i + chunk_size]);
                    i += chunk_size;
                }
                if i < len {
                    dest[i..len].copy_from_slice(&src[i..len]);
                }
            }
            IsaLevel::V1 => {
                // Simple byte-by-byte baseline fallback copy
                dest[..len].copy_from_slice(&src[..len]);
            }
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_isa_level_auto_detection() {
        // v1 baseline check
        let v1_cpu = CpuIsaAssessor::new(false, false, false, false);
        assert_eq!(v1_cpu.level, IsaLevel::V1);

        // v2 SSE4.2 check
        let v2_cpu = CpuIsaAssessor::new(true, false, false, false);
        assert_eq!(v2_cpu.level, IsaLevel::V2);

        // v3 AVX2 check
        let v3_cpu = CpuIsaAssessor::new(true, true, false, true);
        assert_eq!(v3_cpu.level, IsaLevel::V3);

        // v4 AVX-512 check
        let v4_cpu = CpuIsaAssessor::new(true, true, true, true);
        assert_eq!(v4_cpu.level, IsaLevel::V4);
    }

    #[test]
    fn test_vectorized_memcpy_routing() {
        let cpu = CpuIsaAssessor::new(true, true, false, true); // Level v3

        let mut dest = [0u8; 100];
        let mut src = [0u8; 100];
        for i in 0..100 {
            src[i] = i as u8;
        }

        cpu.direct_memcpy(&mut dest, &src);
        assert_eq!(dest[0], 0);
        assert_eq!(dest[45], 45);
        assert_eq!(dest[99], 99);
    }
}
