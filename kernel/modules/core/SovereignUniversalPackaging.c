/**
 * Σ SIGMAOS ZENITH : Sovereign Universal Packaging Matrix (Modular v2.0)
 * 
 * Refactored into Container and Engine components.
 */

#include "../../../include/sigma_kernel.h"
#include "SovereignAppContainer.h"

/**
 * @brief Initialize universal packaging engine
 */
void sigma_universal_packaging_init() {
    sigma_printf("Σ [PKG-MATRIX] Initializing Universal Packaging Engine (Modular v2.0)...\n");
    app_container_init();
}

/**
 * @brief Deploy a sandboxed application (Flatpak style)
 */
void sigma_deploy_sandboxed_app(const char* identifier, int sandbox_level) {
    SovereignAppContainer_t* new_app = app_container_alloc();
    if (!new_app) {
        sigma_printf("Σ [PKG-MATRIX] Capacity reached. Cannot deploy [%s]\n", identifier);
        return;
    }
    
    sigma_strncpy(new_app->app_id, identifier, 64);
    new_app->is_sandboxed = 1;
    new_app->has_network_access = (sandbox_level < 2) ? 1 : 0;
    new_app->has_fs_access = (sandbox_level < 3) ? 1 : 0;
    
    sigma_printf("Σ [PKG-MATRIX] Application [%s] deployed. NET:%d FS:%d\n", 
                     new_app->app_id, new_app->has_network_access, new_app->has_fs_access);
}

void SovereignUniversalPackaging_Register(void) {
    static SovereignModule_t s_pkg_module = {
        .name = "SovereignUniversalPackaging",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))sigma_universal_packaging_init,
    };
    sigma_module_register(&s_pkg_module);
}
