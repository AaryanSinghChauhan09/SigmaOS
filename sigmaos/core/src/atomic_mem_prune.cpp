#include "sigma_core.h"
#include "libc/sigma_libc.h"

extern "C" {

void mem_prune(int days_old) {
    sigma_kprint("[SigmaMem] Pruning atomic vector shards older than ");
    sigma_kprint_int(days_old);
    sigma_kprint(" days.\n");
    // Native filesystem/memory-mapped deletion logic
}

}

} // extern "C"
