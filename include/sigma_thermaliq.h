#ifndef SIGMA_THERMALIQ_H
#define SIGMA_THERMALIQ_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 core_id;
    sigma_u32 temperature;
    sigma_u32 fan_speed;
    sigma_u32 thermal_state;
} sigma_thermal_info_t;

typedef struct {
    sigma_u32 cpu_temp_avg;
    sigma_u32 gpu_temp_avg;
    sigma_u32 active_cooling_zones;
} sigma_thermal_state_t;

void thermaliq_init(void);
void thermaliq_update(void);
const sigma_thermal_state_t* thermaliq_get_state(void);

#ifdef __cplusplus
}
#endif

#endif
