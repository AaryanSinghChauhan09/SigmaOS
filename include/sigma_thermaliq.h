#ifndef SIGMA_THERMALIQ_H
#define SIGMA_THERMALIQ_H

#include "sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Hardware {

class SovereignThermalIQ {
public:
    static SovereignThermalIQ& getInstance();

    void init();
    sigma_u32 getPackageTemp();
    void applyThermalPolicy();
    void emergencyThrottle(sigma_u32 threshold_celsius);

private:
    SovereignThermalIQ() : history_ptr(0), initialized(0) {
        for(int i=0; i<4; i++) temp_history[i] = 60u;
    }
    
    sigma_u32 temp_history[4];
    sigma_u32 history_ptr;
    sigma_u32 initialized;
};

} // namespace Hardware
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void      thermaliq_init(void);
sigma_u32 thermaliq_get_package_temp(void);
void      thermaliq_apply_thermal_policy(void);
void      thermaliq_emergency_throttle(sigma_u32 threshold_celsius);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_THERMALIQ_H */
