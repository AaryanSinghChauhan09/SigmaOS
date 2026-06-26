/*
 * sigma_tiling.h — Zenith auto-tiling WM (i3/bspwm-class) public API.
 * Implementation: sigma_tiling_wm.cpp · C shim: sigma_tiling.c
 */
#ifndef SIGMA_TILING_H
#define SIGMA_TILING_H

#include "../../include/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

sigma_status sigma_tiling_init(sigma_u32 screen_w, sigma_u32 screen_h);
sigma_status sigma_tiling_auto_tile(void);
sigma_status sigma_tiling_set_layout(sigma_u32 mode);
sigma_status sigma_tiling_set_gaps(sigma_u32 inner, sigma_u32 outer);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TILING_H */
