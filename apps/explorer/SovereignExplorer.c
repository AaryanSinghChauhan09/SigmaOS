#include "../include/sigma_kernel.h"

void SovereignExplorer_Run() {
    sigma_printf("\nΣ [APP]: Sovereign File Explorer v1.0\n");
    sigma_printf("Σ [VFS]: Mapping Root Directory [/]...\n");
    
    /* Mock VFS listing */
    const char* files[] = { "kernel.bin", "system.config", "identity.vault", "apps/" };
    for (int i = 0; i < 4; i++) {
        sigma_printf("  + [FILE] %s\n", files[i]);
    }
    sigma_printf("Σ [EXPLORER]: Directory listing complete.\n");
}
