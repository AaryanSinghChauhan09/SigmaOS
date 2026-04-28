#include "silicon_audit.hpp"

namespace SigmaOS {
namespace Build {

void SovereignSiliconAudit::DetectFeatures() {
    sigma_printf("[BUILD/DETECTION]: Probing CPUID for instructions...\n");
    
#if defined(SIGMA_ARCH_X86_64)
    unsigned int eax, ebx, ecx, edx;
    
    // Leaf 1 for SSE4.2
    __asm__ volatile ("cpuid" : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx) : "a"(1));
    m_has_sse42 = (ecx & (1 << 20)) != 0;
    
    // Leaf 7 for AVX2/AVX512
    __asm__ volatile ("cpuid" : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx) : "a"(7), "c"(0));
    m_has_avx2 = (ebx & (1 << 5)) != 0;
    m_has_avx512 = (ebx & (1 << 16)) != 0;
#else
    m_has_sse42 = false; m_has_avx2 = false; m_has_avx512 = false;
#endif

    sigma_printf("[BUILD/CPU]: SSE4.2: %s\n", (m_has_sse42 ? "[YES]" : "[NO]"));
    sigma_printf("[BUILD/CPU]: AVX2: %s\n", (m_has_avx2 ? "[YES]" : "[NO]"));
    sigma_printf("[BUILD/CPU]: AVX-512: %s\n", (m_has_avx512 ? "[YES]" : "[NO]"));
}

SigmaString SovereignSiliconAudit::GetOptimizationFlags() {
    SigmaString flags = "-march=native -O3";
    if (m_has_avx512) flags.append(" -mavx512f");
    else if (m_has_avx2) flags.append(" -mavx2");
    return flags;
}

} // namespace Build
} // namespace SigmaOS
