/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

// SigmaOS Native Hardware Interface (OOP Design)
// ============================================
// Zero dependency. Replaces <sys/io.h>, kernel IOCTLs, or Windows DeviceIoControl.
// Pure low-level generic OS interface using basic machine-level assembly instructions.
// Designed for customisation & automation over raw physical peripherals (PCI, USB).

#ifndef SIGMA_HARDWARE_HPP
#define SIGMA_HARDWARE_HPP

#include "types.h"

// Forward assembly hook points for hardware IN/OUT instructions
extern "C" u8 sigma_asm_inb(u16 port);
extern "C" void sigma_asm_outb(u16 port, u8 data);
extern "C" u16 sigma_asm_inw(u16 port);
extern "C" void sigma_asm_outw(u16 port, u16 data);
extern "C" u32 sigma_asm_ind(u16 port);
extern "C" void sigma_asm_outd(u16 port, u32 data);

namespace Sigma {
namespace Hardware {

// Object-Oriented Interface for a peripheral port
class IOPort {
private:
    u16 port_address;

public:
    IOPort(u16 address) : port_address(address) {}

    u8 ReadByte() const {
        return sigma_asm_inb(port_address);
    }

    void WriteByte(u8 data) const {
        sigma_asm_outb(port_address, data);
    }

    u16 ReadWord() const {
        return sigma_asm_inw(port_address);
    }

    void WriteWord(u16 data) const {
        sigma_asm_outw(port_address, data);
    }

    u32 ReadDWord() const {
        return sigma_asm_ind(port_address);
    }

    void WriteDWord(u32 data) const {
        sigma_asm_outd(port_address, data);
    }
};

// Advanced Native Bus Interface
class PCIBus {
public:
    // Pure Machine-Level PCI Configuration Space Scanning
    static u32 ReadConfig(u32 bus, u32 device, u32 function, u32 offset) {
        // Construct PCI address: Bit 31 (Enable), Bits 23-16 (Bus), 15-11 (Device), 10-8 (Func), 7-2 (Reg)
        u32 address = (u32)((bus << 16) | (device << 11) | (function << 8) | (offset & 0xFC) | ((u32)0x80000000));
        IOPort config_address(0xCF8);
        IOPort config_data(0xCFC);
        
        config_address.WriteDWord(address);
        return config_data.ReadDWord();
    }
};

} // namespace Hardware
} // namespace Sigma

#endif // SIGMA_HARDWARE_HPP

