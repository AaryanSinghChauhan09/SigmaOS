#ifndef SOVEREIGN_LATTICE_REGISTRY_H
#define SOVEREIGN_LATTICE_REGISTRY_H
#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"
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
