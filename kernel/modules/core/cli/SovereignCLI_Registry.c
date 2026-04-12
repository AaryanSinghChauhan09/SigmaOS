#include "SovereignCLI_Core.h"
#include "../../../../include/SovereignCLI.h"

void SovereignCLI_Register(void) {
    static SovereignModule_t s_cli_module = {
        .name = "SovereignCLI",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))SovereignCLI_Init,
    };
    sigma_module_register(&s_cli_module);
}
