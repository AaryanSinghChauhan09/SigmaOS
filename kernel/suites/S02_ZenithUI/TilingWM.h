#ifndef SIGMA_TILING_WM_H
#define SIGMA_TILING_WM_H

// SigmaOS Zenith Tiling Window Manager Shard
// Absorbing structural WM paradigms (i3/bspwm) with GPU acceleration
#include <stdint.h>

void ui_init_tiling_wm();
void ui_set_gpu_acceleration(uint8_t enabled);
void ui_arrange_active_windows();

#endif // SIGMA_TILING_WM_H
