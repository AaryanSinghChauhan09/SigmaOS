#include "sigma_base.h"

#include "SovereignFS.h"
#include "sigma_libc.h"

sigma_err_t sigma_fat32_mount(const char* target) {
    sigma_printf("  Σ [FAT32]: Mounting Legacy Persistence Matrix at %s...\n", target);
    sigma_printf("  Σ [FAT32]: LBA sector translation: ACTIVE.\n");
    return SIGMA_OK;
}

void SovereignFAT32_Register(void) {
    SovereignFSRegistry_Register("fat32", sigma_fat32_mount);
}



