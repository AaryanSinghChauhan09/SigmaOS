//! SigmaOS Native System Info Module
//! Replaces sysinfo dependency with simple system information gathering

#![no_std]

/// Simple CPU information
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CpuInfo {
    pub vendor_id: [u8; 12],
    pub brand_string: [u8; 48],
    pub cores: u32,
    pub frequency_mhz: u32,
}

/// Simple memory information
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryInfo {
    pub total_kb: u64,
    pub available_kb: u64,
    pub used_kb: u64,
}

/// Simple system information
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SystemInfo {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
}

/// Get CPU vendor ID using CPUID
pub fn get_cpu_vendor() -> [u8; 12] {
    let mut vendor = [0u8; 12];
    
    unsafe {
        let mut ebx: u32;
        let mut ecx: u32;
        let mut edx: u32;
        
        core::arch::asm!(
            "cpuid",
            inlateout("eax") 0 => _,
            lateout("ebx") ebx,
            lateout("ecx") ecx,
            lateout("edx") edx,
            options(nomem, nostack)
        );
        
        // EBX, EDX, ECX contain vendor string
        let ebx_bytes = ebx.to_le_bytes();
        let edx_bytes = edx.to_le_bytes();
        let ecx_bytes = ecx.to_le_bytes();
        
        vendor[0..4].copy_from_slice(&ebx_bytes);
        vendor[4..8].copy_from_slice(&edx_bytes);
        vendor[8..12].copy_from_slice(&ecx_bytes);
    }
    
    vendor
}

/// Get CPU brand string using CPUID
pub fn get_cpu_brand() -> [u8; 48] {
    let mut brand = [0u8; 48];
    
    unsafe {
        let mut eax: u32;
        let mut ebx: u32;
        let mut ecx: u32;
        let mut edx: u32;
        
        // Get brand string parts (0x80000002, 0x80000003, 0x80000004)
        for i in 0..3 {
            eax = 0x80000002 + i;
            core::arch::asm!(
                "cpuid",
                inlateout("eax") eax => _,
                lateout("ebx") ebx,
                lateout("ecx") ecx,
                lateout("edx") edx,
                options(nomem, nostack)
            );
            
            let offset = (i * 16) as usize;
            let ebx_bytes = ebx.to_le_bytes();
            let edx_bytes = edx.to_le_bytes();
            let ecx_bytes = ecx.to_le_bytes();
            
            brand[offset..offset+4].copy_from_slice(&ebx_bytes);
            brand[offset+4..offset+8].copy_from_slice(&edx_bytes);
            brand[offset+8..offset+12].copy_from_slice(&ecx_bytes);
        }
    }
    
    brand
}

/// Get CPU core count (simplified - returns 1 for now)
pub fn get_cpu_cores() -> u32 {
    // In a full implementation, this would use CPUID to get logical processor count
    // For now, return a placeholder
    1
}

/// Get CPU frequency (placeholder - requires proper timer)
pub fn get_cpu_frequency() -> u32 {
    // In a full implementation, this would measure TSC over known time
    // For now, return a placeholder (3 GHz)
    3000
}

/// Get memory information (placeholder)
pub fn get_memory_info() -> MemoryInfo {
    // In a full implementation, this would query the memory map
    // For now, return placeholder values (4 GB)
    MemoryInfo {
        total_kb: 4 * 1024 * 1024,
        available_kb: 3 * 1024 * 1024,
        used_kb: 1 * 1024 * 1024,
    }
}

/// Get complete system information
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        cpu: CpuInfo {
            vendor_id: get_cpu_vendor(),
            brand_string: get_cpu_brand(),
            cores: get_cpu_cores(),
            frequency_mhz: get_cpu_frequency(),
        },
        memory: get_memory_info(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_vendor() {
        let vendor = get_cpu_vendor();
        // Should be "GenuineIntel", "AuthenticAMD", or similar
        assert!(vendor.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_cpu_brand() {
        let brand = get_cpu_brand();
        // Should contain CPU name
        assert!(brand.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_system_info() {
        let info = get_system_info();
        assert!(info.cpu.cores > 0);
        assert!(info.cpu.frequency_mhz > 0);
        assert!(info.memory.total_kb > 0);
    }
}
