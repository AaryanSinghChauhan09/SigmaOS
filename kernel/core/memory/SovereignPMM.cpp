#include "../../../include/sigma_kernel_types.h"
#include "../../../include/SovereignLibC.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Physical Memory Manager (PMM) Shard
 * Principles: Zero-Abstract Page Orchestration, Silicon-Direct.
 */

extern "C" void pmm_init(sigma_u64 mem_size);
extern "C" void* pmm_alloc_page();
extern "C" void pmm_unlock_page(sigma_u64 addr);

namespace SigmaOS {
namespace Kernel {
namespace Memory {

class SovereignPMM : public SigmaObject {
public:
    static SovereignPMM& getInstance() {
        static SovereignPMM instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignPMM"; }

    void init(sigma_u64 mem_size) {
        sigma_log("[PMM] Initializing Sovereign Physical Memory Shard...");
        pmm_init(mem_size);
        sigma_log("[PMM] Silicon Memory Lattice mapped and active.");
    }

    void* allocatePage() {
        void* page = pmm_alloc_page();
        if (page == SIGMA_NULL) {
            sigma_log("[PMM] ERR: Physical Out of Memory Shard!");
        }
        return page;
    }

    void freePage(void* addr) {
        pmm_unlock_page((sigma_u64)addr);
    }

private:
    SovereignPMM() {}
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
