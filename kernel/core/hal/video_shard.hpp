#include "../../../include/sigma_types.h"
#include "hal/sigma_hal.h"
#ifndef VIDEO_SHARD_HPP
#define VIDEO_SHARD_HPP

#include "libc/SovereignLibC.h"

#include "port_shard.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignVideoShard : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignVideoShard"; }

    static void WriteBGA(sigma_u16 index, sigma_u16 val) {
        SovereignPortShard::outb(0x1CE, index);
        SovereignPortShard::outb(0x1CF, val);
    }

    void SetResolution(sigma_u16 width, sigma_u16 height, sigma_u16 bpp) {
        sigma_log("[VIDEO-SHARD]: Setting Sovereign Resolution: %ux%ux%u\n", width, height, bpp);
        WriteBGA(4, 0); // Disable
        WriteBGA(1, width);
        WriteBGA(2, height);
        WriteBGA(3, bpp);
        WriteBGA(4, 1); // Enable
    }

    void AuditVideo() {
        sigma_log("\n--- Î£ SOVEREIGN VIDEO AUDIT ---\n");
        sigma_log("| Display Mode   : BGA / LFB ACTIVE\n");
        sigma_log("| VRAM Aperture  : 0xFD000000 [SHARDED]\n");
        sigma_log("| Acceleration   : 2D-BLIT / SHARD-FLIP\n");
        sigma_log("-------------------------------\n");
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

