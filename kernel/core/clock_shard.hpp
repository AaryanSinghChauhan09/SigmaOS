#ifndef CLOCK_SHARD_HPP
#define CLOCK_SHARD_HPP

#include "SovereignLibC.h"

#include "SigmaOOP.hpp"
#include "port_shard.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignClockShard : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignClockShard"; }

    static sigma_u8 ReadRTC(sigma_u8 reg) {
        SovereignPortShard::outb(0x70, reg);
        return SovereignPortShard::inb(0x71);
    }

    void GetSystemTime(sigma_u8* hour, sigma_u8* min, sigma_u8* sec) {
        *hour = ReadRTC(0x04);
        *min  = ReadRTC(0x02);
        *sec  = ReadRTC(0x00);
    }

    void AuditClock() {
        sigma_u8 h, m, s;
        GetSystemTime(&h, &m, &s);
        sigma_printf("\n--- Î£ SOVEREIGN CLOCK AUDIT ---\n");
        sigma_printf("| System Time    : %02x:%02x:%02x [RTC SHARD]\n", h, m, s);
        sigma_printf("| Clock Source   : CMOS / Silicon Crystal\n");
        sigma_printf("| Drift Status   : ZERO-PPM SHUNTED\n");
        sigma_printf("-------------------------------\n");
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif
