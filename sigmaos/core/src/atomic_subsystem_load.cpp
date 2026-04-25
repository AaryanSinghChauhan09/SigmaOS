#include "sigma_core.h"
#include "sigma_libc.h"

extern "C" {

void subsystem_load(const char* name) {
    sigma_kprint("[SigmaCore] Loading shard: ");
    sigma_kprint(name);
    sigma_kprint("\n");
    // Native logic to map the shard into memory
}

}
