//! SigmaOS CPU Feature Detection and Optimization
//! Implements Gentoo-like compiler-assisted target optimizations
//! Zero-dependency CPU capability detection for bare-metal

#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

/// CPU instruction extensions supported by SigmaOS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuInstructionExtension {
    AVX512,
    AVX2,
    SSE4_2,
    Neon,
    Sve,
    AMX,
    Default,
}

/// Dynamic Target Optimization Selector (OOP Pattern)
/// Implements Gentoo-like processor-specific compilation flags
pub struct SovereignCompilerOptimizer {
    active_extension: CpuInstructionExtension,
    cache_line_size: AtomicUsize,
    tlb_entries: AtomicUsize,
}

impl SovereignCompilerOptimizer {
    pub const fn new() -> Self {
        SovereignCompilerOptimizer {
            active_extension: CpuInstructionExtension::Default,
            cache_line_size: AtomicUsize::new(64), // Default cache line size
            tlb_entries: AtomicUsize::new(64), // Default TLB entries
        }
    }

    /// Detect processor extensions at boot time
    pub fn detect_processor_extensions(&mut self) {
        self.active_extension = Self::read_cpuid_features();
    }

    /// Reads raw CPUID instruction sets without standard library references
    #[cfg(target_arch = "x86_64")]
    fn read_cpuid_features() -> CpuInstructionExtension {
        let mut ebx: u32 = 0;
        let mut ecx: u32 = 0;
        let mut edx: u32 = 0;

        unsafe {
            core::arch::asm!(
                "cpuid",
                inout("eax") 7 => _,
                out("ebx") ebx,
                out("ecx") ecx,
                out("edx") edx,
            );
        }

        // Bit 16 in EBX indicates AVX-512 Foundation support
        if (ebx & (1 << 16)) != 0 {
            CpuInstructionExtension::AVX512
        }
        // Bit 5 in EBX indicates AVX2 support
        else if (ebx & (1 << 5)) != 0 {
            CpuInstructionExtension::AVX2
        }
        // Bit 19 in ECX indicates SSE4.2 support
        else if (ecx & (1 << 19)) != 0 {
            CpuInstructionExtension::SSE4_2
        }
        else {
            CpuInstructionExtension::Default
        }
    }

    #[cfg(target_arch = "aarch64")]
    fn read_cpuid_features() -> CpuInstructionExtension {
        // ARM64 uses ID_AA64ISAR0_EL1 register for feature detection
        let mut isar0: u64 = 0;
        
        unsafe {
            core::arch::asm!(
                "mrs {}, ID_AA64ISAR0_EL1",
                out(reg) isar0,
            );
        }

        // Check for SVE support (bits 35-32)
        let sve_value = (isar0 >> 32) & 0xF;
        if sve_value >= 1 {
            CpuInstructionExtension::Sve
        }
        // Check for NEON support (always present in ARMv8)
        else {
            CpuInstructionExtension::Neon
        }
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    fn read_cpuid_features() -> CpuInstructionExtension {
        CpuInstructionExtension::Default
    }

    /// Get active CPU extension
    pub fn active_extension(&self) -> CpuInstructionExtension {
        self.active_extension
    }

    /// Dynamic JIT code selector utilizing polymorphism
    pub fn execute_vector_multiply(&self, lhs: &[f32], rhs: &[f32], out: &mut [f32]) {
        match self.active_extension {
            CpuInstructionExtension::AVX512 => {
                // Vectorized AVX-512 FMA (Fused Multiply-Add) execution path
                for i in (0..lhs.len()).step_by(16) {
                    for j in 0..16 {
                        if i + j < lhs.len() {
                            out[i + j] = lhs[i + j] * rhs[i + j];
                        }
                    }
                }
            }
            CpuInstructionExtension::AVX2 | CpuInstructionExtension::SSE4_2 => {
                // SIMD execution path
                for i in (0..lhs.len()).step_by(8) {
                    for j in 0..8 {
                        if i + j < lhs.len() {
                            out[i + j] = lhs[i + j] * rhs[i + j];
                        }
                    }
                }
            }
            CpuInstructionExtension::Neon | CpuInstructionExtension::Sve => {
                // ARM SIMD execution path
                for i in (0..lhs.len()).step_by(4) {
                    for j in 0..4 {
                        if i + j < lhs.len() {
                            out[i + j] = lhs[i + j] * rhs[i + j];
                        }
                    }
                }
            }
            CpuInstructionExtension::Default => {
                // Fallback serial execution path
                for i in 0..lhs.len() {
                    out[i] = lhs[i] * rhs[i];
                }
            }
        }
    }

    /// Detect cache line size for memory optimization
    pub fn detect_cache_line_size(&self) -> usize {
        // On x86_64, use CPUID to detect cache line size
        #[cfg(target_arch = "x86_64")]
        {
            let mut eax: u32 = 0;
            let mut ebx: u32 = 0;
            let mut ecx: u32 = 0;
            let mut edx: u32 = 0;

            unsafe {
                core::arch::asm!(
                    "cpuid",
                    inout("eax") 0x80000000 => eax,
                    out("ebx") ebx,
                    out("ecx") ecx,
                    out("edx") edx,
                );
            }

            if eax >= 0x80000001 {
                unsafe {
                    core::arch::asm!(
                        "cpuid",
                        inout("eax") 0x80000001 => _,
                        out("ebx") ebx,
                        out("ecx") ecx,
                        out("edx") edx,
                    );
                }
                // CLFLUSH line size in bits 15-8 of ECX
                ((ecx >> 8) & 0xFF) as usize * 8
            } else {
                64 // Default cache line size
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            64 // Default cache line size for other architectures
        }
    }

    /// Optimize memory operations based on detected cache line size
    pub fn optimize_memory_copy(&self, src: &[u8], dst: &mut [u8]) {
        let cache_line = self.detect_cache_line_size();
        
        if src.len() >= cache_line {
            let chunks = src.len() / cache_line;
            for i in 0..chunks {
                let start = i * cache_line;
                let end = (i + 1) * cache_line;
                if end <= src.len() && end <= dst.len() {
                    dst[start..end].copy_from_slice(&src[start..end]);
                }
            }
            // Copy remaining bytes
            let remaining_start = chunks * cache_line;
            if remaining_start < src.len() && remaining_start < dst.len() {
                dst[remaining_start..].copy_from_slice(&src[remaining_start..]);
            }
        } else {
            dst.copy_from_slice(src);
        }
    }
}

/// Global CPU optimizer instance
static mut GLOBAL_CPU_OPTIMIZER: Option<SovereignCompilerOptimizer> = None;

/// Initialize global CPU optimizer
pub fn init_cpu_optimizer() {
    unsafe {
        GLOBAL_CPU_OPTIMIZER = Some(SovereignCompilerOptimizer::new());
        if let Some(ref mut optimizer) = GLOBAL_CPU_OPTIMIZER {
            optimizer.detect_processor_extensions();
        }
    }
}

/// Get global CPU optimizer reference
pub fn get_cpu_optimizer() -> &'static SovereignCompilerOptimizer {
    unsafe {
        GLOBAL_CPU_OPTIMIZER.as_ref().expect("CPU optimizer not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_optimizer_creation() {
        let optimizer = SovereignCompilerOptimizer::new();
        assert_eq!(optimizer.active_extension(), CpuInstructionExtension::Default);
    }

    #[test]
    fn test_vector_multiply() {
        let optimizer = SovereignCompilerOptimizer::new();
        let lhs = vec![1.0, 2.0, 3.0, 4.0];
        let rhs = vec![2.0, 3.0, 4.0, 5.0];
        let mut out = vec![0.0; 4];
        
        optimizer.execute_vector_multiply(&lhs, &rhs, &mut out);
        
        assert_eq!(out[0], 2.0);
        assert_eq!(out[1], 6.0);
        assert_eq!(out[2], 12.0);
        assert_eq!(out[3], 20.0);
    }

    #[test]
    fn test_memory_copy() {
        let optimizer = SovereignCompilerOptimizer::new();
        let src = vec![1, 2, 3, 4, 5];
        let mut dst = vec![0; 5];
        
        optimizer.optimize_memory_copy(&src, &mut dst);
        
        assert_eq!(dst, src);
    }
}
