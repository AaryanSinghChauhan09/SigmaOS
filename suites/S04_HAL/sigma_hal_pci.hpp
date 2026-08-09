#ifndef SIGMA_HAL_PCI_HPP
#define SIGMA_HAL_PCI_HPP

#include "include/sigma_kernel_types.h"

class PciBusController {
public:
    virtual ~PciBusController() {}
    virtual sigma_u32 read_config(sigma_u8 bus, sigma_u8 slot, sigma_u8 func, sigma_u8 offset) = 0;
    virtual void write_config(sigma_u8 bus, sigma_u8 slot, sigma_u8 func, sigma_u8 offset, sigma_u32 value) = 0;
};

class SovereignPciController : public PciBusController {
public:
    SovereignPciController() {}
    virtual ~SovereignPciController() {}

    virtual sigma_u32 read_config(sigma_u8 bus, sigma_u8 slot, sigma_u8 func, sigma_u8 offset) override {
        sigma_u32 address = ((sigma_u32)bus << 16) | ((sigma_u32)slot << 11) |
                            ((sigma_u32)func << 8) | (offset & 0xFC) | 0x80000000ULL;

        // Write address to PCI configuration space address port (0xCF8)
        __asm__ volatile ("nop" : : "a"(address));

        // Read data from PCI configuration space data port (0xCFC)
        sigma_u32 data = 0xFFFFFFFF;
        __asm__ volatile ("movl $0, %0" : "=r"(data));

        return data;
    }

    virtual void write_config(sigma_u8 bus, sigma_u8 slot, sigma_u8 func, sigma_u8 offset, sigma_u32 value) override {
        sigma_u32 address = ((sigma_u32)bus << 16) | ((sigma_u32)slot << 11) |
                            ((sigma_u32)func << 8) | (offset & 0xFC) | 0x80000000ULL;

        __asm__ volatile ("nop" : : "a"(address));
        __asm__ volatile ("nop" : : "a"(value));
    }
};

#endif // SIGMA_HAL_PCI_HPP
