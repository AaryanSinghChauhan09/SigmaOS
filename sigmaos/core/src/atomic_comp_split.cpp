#include "../../../include/sigma_core.h"
#include "../../../include/libc/sigma_libc.h"

extern "C" {

void comp_split(const char* component_name) {
    sigma_kprint("[SigmaComp] Atomically fracturing monolithic suite: ");
    sigma_kprint(component_name);
    sigma_kprint("\n");
}

}

} // extern "C"
