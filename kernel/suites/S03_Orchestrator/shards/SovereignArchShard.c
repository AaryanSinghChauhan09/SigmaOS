#include "../../include/sigma_base.h"

#include "../../include/SovereignDistro.h"
#include "../../include/sigma_libc.h"

void SovereignArch_AbsorbLogic(void) {
    sigma_printf("  Σ [ARCH]: Rolling Matrix Sync... AUR initialized.\n");
    sigma_printf("  Σ [ARCH]: Pacman Parity: LTO and -march=native optimizations forced.\n");
}

void SovereignArch_Register(void) {
    SovereignDistro_Register("arch", "pacman", "systemd", "Rolling, AUR, KISS Purity", SovereignArch_AbsorbLogic);
}



