#ifndef SIGMA_VMM_H
#define SIGMA_VMM_H

#include "include/sigma_types.h"
#include "include/SigmaOOP.hpp"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Memory {

class SovereignVMM : public SigmaObject {
public:
    static SovereignVMM& getInstance();

    const char* type_name() const noexcept override { return "SovereignVMM"; }

    void init();
    void map(void* virtual_addr, void* physical_addr, sigma_u32 flags);
    void* translate(void* virtual_addr);
    void audit();

private:
    SovereignVMM() : m_active_tables(0) {}
    sigma_u32 m_active_tables;
};

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void  vmm_init_shard(void);
void  vmm_map_shard(void* v, void* p, sigma_u32 f);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VMM_H */
