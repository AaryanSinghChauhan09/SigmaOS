/**
 * Σ SIGMAOS ZENITH : Sovereign Service Lattice (Init System) (Modular v2.0)
 */

#include "../../../include/sigma_kernel.h"
#include "SovereignServiceUnit.h"

/**
 * @brief Initialize the Service Lattice
 */
void sigma_service_lattice_init() {
    sigma_printf("Σ [INIT] Bootstrapping Sovereign Service Lattice (Modular v2.0)...\n");
    service_unit_init();
}

/**
 * @brief Register a new service unit
 */
void sigma_register_service(const char* name, int restart_flag) {
    SovereignServiceUnit_t* unit = service_unit_alloc();
    if (unit) {
        sigma_strcpy(unit->service_name, name);
        unit->is_active = 0;
        unit->restart_on_failure = restart_flag;
        sigma_printf("Σ [INIT] Registered new service unit: %s.target\n", name);
    }
}

/**
 * @brief Ignite (start) a service and its dependencies
 */
void sigma_ignite_service_shard(const char* target_name) {
    SovereignServiceUnit_t* unit = service_unit_find(target_name);
    if (unit) {
        if (unit->is_active) {
            sigma_printf("Σ [INIT] Service %s is already running.\n", target_name);
            return;
        }
        unit->is_active = 1;
        sigma_printf("Σ [INIT] Ignited service shard: [%s] successfully.\n", target_name);
        return;
    }
    sigma_printf("Σ [INIT] CRITICAL: Service %s not found in lattice.\n", target_name);
}

void SovereignServiceLattice_Register(void) {
    static SovereignModule_t s_lattice_module = {
        .name = "SovereignServiceLattice",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))sigma_service_lattice_init,
    };
    sigma_module_register(&s_lattice_module);
}
