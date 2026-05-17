/*
 * =========================================================================
 * Σ SIGMAOS: x86_64 HAL IMPLEMENTATION
 * =========================================================================
 */
#include "hal.hpp"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace HAL {

class HALx86 : public AbstractHAL {
public:
    void initCPU() override { sigma_log_info("[HAL] Initializing x86 CPU via Long Mode constraints."); }
    void initMemory() override { sigma_log_info("[HAL] Configuring x86 Paging/GDT."); }
    void initInterrupts() override { sigma_log_info("[HAL] Loading x86 IDT/APIC."); }
    void initTimer() override { sigma_log_info("[HAL] Configuring x86 PIT/HPET."); }
    void writePort(sigma_u16 port, sigma_u8 value) override {
        asm volatile ("outb %0, %1" : : "a"(value), "Nd"(port));
    }
    sigma_u8 readPort(sigma_u16 port) override {
        sigma_u8 ret;
        asm volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
        return ret;
    }
};

} // namespace HAL
} // namespace SigmaOS
 