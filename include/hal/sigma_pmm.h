#ifndef SIGMA_PMM_H
#define SIGMA_PMM_H

#include "../sigma_types.h"
#include "../SigmaOOP.hpp"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Memory {

#define BITMAP_SIZE (1024 * 1024 / 8) /* Supports 4GB of RAM */

class SovereignPMM : public SigmaObject {
public:
    static SovereignPMM& getInstance() {
        static SovereignPMM instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignPMM"; }

    void init(sigma_u64 mem_size);
    void* allocatePage();
    void lockPage(sigma_u64 addr);
    void unlockPage(sigma_u64 addr);
    void compactMemory();
    sigma_u64 getUsedMemory() const;

private:
    SovereignPMM() {}
    sigma_u32 m_bitmap[BITMAP_SIZE];
};

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void      pmm_init_shard(sigma_u64 mem_size);
void*     pmm_alloc_shard(void);
void      pmm_free_shard(void* addr);
void      pmm_compact_shard(void);
sigma_u64 pmm_get_used_shard(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PMM_H */
