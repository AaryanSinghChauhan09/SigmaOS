#include "../../include/sigma_base.h"

#include "../../include/SovereignDistro.h"
#include "../../include/sigma_libc.h"

void SovereignGaruda_AbsorbLogic(void) {
    sigma_printf("  Σ [GARUDA]: Zen-Kernel optimizations applied. Scheduler tuned for low latency.\n");
    sigma_printf("  Σ [POPOS]: Auto-tiling window manager sharded into SigmaUI.\n");
}

void SovereignGaruda_Register(void) {
    SovereignDistro_Register("garuda", "pacman", "systemd", "Zen-Kernel, Dragonized Aesthetics", SovereignGaruda_AbsorbLogic);
    SovereignDistro_Register("popos", "apt", "systemd", "Auto-Tiling, NVIDIA Optimized", SovereignGaruda_AbsorbLogic);
}



