#ifndef SOVEREIGN_MODULE_H
#define SOVEREIGN_MODULE_H

#include "suites/S01_Genesis/shards/sigma_kernel.h"

typedef enum {
    MODULE_TYPE_CORE,
    MODULE_TYPE_SECURITY,
    MODULE_TYPE_DRIVER,
    MODULE_TYPE_SYSTEM
} SovereignModuleType_t;

typedef struct SovereignModule {
    const char* name;
    SovereignModuleType_t type;
    sigma_err_t (*Init)(void);
    void (*Shutdown)(void);
    void (*Audit)(void);
} SovereignModule_t;

sigma_err_t sigma_module_register(SovereignModule_t* module);
sigma_err_t sigma_modules_init_all(void);

#endif /* SOVEREIGN_MODULE_H */
