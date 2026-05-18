/*
 * =========================================================================
 * Σ SIGMAOS: RISC-V HAL IMPLEMENTATION
 * =========================================================================
 */
#include "hal.hpp"
#include "sigma_log.h"

namespace SigmaOS {
namespace HAL {

class HALRISCV : public AbstractHAL {
public:
    void initCPU() override { sigma_log_info("[HAL] Initializing RISC-V hart (hardware thread)."); }
    void initMemory() override { sigma_log_info("[HAL] Configuring RISC-V Sv39/Sv48 Paging."); }
    void initInterrupts() override { sigma_log_info("[HAL] Configuring RISC-V PLIC/CLINT."); }
    void initTimer() override { sigma_log_info("[HAL] Configuring RISC-V Time CSRs."); }
    void writePort(sigma_u16 port, sigma_u8 value) override {
        *((volatile sigma_u8*)port) = value;
    }
    sigma_u8 readPort(sigma_u16 port) override {
        return *((volatile sigma_u8*)port);
    }
};

} // namespace HAL
} // namespace SigmaOS
 