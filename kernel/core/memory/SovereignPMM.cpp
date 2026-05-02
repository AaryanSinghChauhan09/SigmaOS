#include "../../../include/sigma_kernel_types.h"
#include "../../../include/SovereignLibC.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Memory {

#define BITMAP_SIZE (1024 * 1024 / 8) /* Supports 4GB of RAM */

/**
 * SigmaOS Sovereign Physical Memory Manager (PMM) Shard
 * Principles: Zero-Abstract Page Orchestration, Silicon-Direct.
 */
class SovereignPMM : public SigmaObject {
public:
    static SovereignPMM& getInstance() {
        static SovereignPMM instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignPMM"; }

    void init(sigma_u64 mem_size) {
        (void)mem_size;
        sigma_log("Σ [PMM]: Initializing Sovereign Physical Memory Shard...");
        
        // Zero out bitmap
        for (sigma_u32 i = 0; i < BITMAP_SIZE; i++) m_bitmap[i] = 0;
        
        // Lock first 1MB and kernel area (simulated)
        for (sigma_u64 addr = 0; addr < 0x200000; addr += PAGE_SIZE) {
            lockPage(addr);
        }
        
        sigma_log("Σ [PMM]: Silicon Memory Lattice mapped and active.");
    }

    void* allocatePage() {
        for (sigma_u32 i = 0; i < BITMAP_SIZE; i++) {
            if (m_bitmap[i] != 0xFFFFFFFF) {
                for (int j = 0; j < 32; j++) {
                    if (!(m_bitmap[i] & (1 << j))) {
                        sigma_u64 addr = (sigma_u64)(i * 32 + j) * PAGE_SIZE;
                        lockPage(addr);
                        return (void*)addr;
                    }
                }
            }
        }
        sigma_log("Σ [PMM]: ERR: Physical Out of Memory Shard!");
        return SIGMA_NULL;
    }

    void lockPage(sigma_u64 addr) {
        sigma_u32 index = addr / PAGE_SIZE;
        m_bitmap[index / 32] |= (1 << (index % 32));
    }

    void unlockPage(sigma_u64 addr) {
        sigma_u32 index = addr / PAGE_SIZE;
        m_bitmap[index / 32] &= ~(1 << (index % 32));
    }

private:
    SovereignPMM() {}
    sigma_u32 m_bitmap[BITMAP_SIZE];
};

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void pmm_init_shard(sigma_u64 mem_size) {
    SigmaOS::Kernel::Memory::SovereignPMM::getInstance().init(mem_size);
}

extern "C" void* pmm_alloc_shard() {
    return SigmaOS::Kernel::Memory::SovereignPMM::getInstance().allocatePage();
}

extern "C" void pmm_free_shard(void* addr) {
    SigmaOS::Kernel::Memory::SovereignPMM::getInstance().unlockPage((sigma_u64)addr);
}
