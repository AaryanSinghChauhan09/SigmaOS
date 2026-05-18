#ifndef SOVEREIGN_LATTICE_REGISTRY_H
#define SOVEREIGN_LATTICE_REGISTRY_H
#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"
#ifdef __cplusplus
namespace SigmaOS { namespace Registry {
class SovereignLatticeRegistry {
public:
    static SovereignLatticeRegistry& getInstance();
    void init();
    void registerShard(const char* name, sigma_u32 id);
private:
    SovereignLatticeRegistry() = default;
};
}}
#endif
#endif
