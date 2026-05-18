/*
 * =========================================================================
 * Σ SIGMAOS: ARM HAL IMPLEMENTATION
 * =========================================================================
 */
#include "hal.hpp"
#include "sigma_log.h"

namespace SigmaOS {
namespace HAL {

class HALARM : public AbstractHAL {
public:
    void initCPU() override { sigma_log_info("[HAL] Initializing ARM CPU cores."); }
    void initMemory() override { sigma_log_info("[HAL] Configuring ARM MMU Translation Tables."); }
    void initInterrupts() override { sigma_log_info("[HAL] Configuring ARM GIC."); }
    void initTimer() override { sigma_log_info("[HAL] Configuring ARM Generic Timer."); }
    void writePort(sigma_u16 port, sigma_u8 value) override {
        *((volatile sigma_u8*)port) = value;
    }
    sigma_u8 readPort(sigma_u16 port) override {
        return *((volatile sigma_u8*)port);
    }
};

} // namespace HAL
} // namespace SigmaOS
 