/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN IOT & ARM UTILITIES (S-IOT)
 * =========================================================================
 * Mission: Event-driven GPIO orchestration and sensor toolkit.
 * Inspired by RPi-Distro / Raspbian.
 * =========================================================================
 */

#ifndef SIGMA_IOT_H
#define SIGMA_IOT_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    GPIO_MODE_INPUT,
    GPIO_MODE_OUTPUT,
    GPIO_MODE_INTERRUPT,
    GPIO_MODE_PWM
} sigma_gpio_mode_t;

/* --- IoT Primitives --- */
void      iot_init(void);
void      iot_gpio_set_mode(sigma_u32 pin, sigma_gpio_mode_t mode);
void      iot_gpio_write(sigma_u32 pin, bool high);
bool      iot_gpio_read(sigma_u32 pin);
void      iot_sensor_poll_all(void);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace IoT {

class SovereignIoTManager {
public:
    static SovereignIoTManager& getInstance() {
        static SovereignIoTManager instance;
        return instance;
    }

    void init();
    void setMode(sigma_u32 pin, sigma_gpio_mode_t mode);
    void write(sigma_u32 pin, bool high);
    bool read(sigma_u32 pin);
    void pollSensors();

private:
    SovereignIoTManager() = default;
};

} // namespace IoT
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_IOT_H */
