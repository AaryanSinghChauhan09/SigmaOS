#ifndef SIGMA_GAME_LAYER_H
#define SIGMA_GAME_LAYER_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

void sigma_game_layer_init(void);
void sigma_game_layer_set_proton(sigma_bool enabled);
sigma_bool sigma_game_layer_is_proton_enabled(void);
void sigma_game_layer_apply_gamemode(void);

/* Proton/Wine shim hooks (SteamOS-class) */
sigma_status sigma_game_launch_with_proton(const char* exe_path, const char* prefix_path);
void sigma_game_set_gpu_performance_mode(sigma_bool high_perf);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_GAME_LAYER_H */
