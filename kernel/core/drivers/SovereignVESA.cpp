#include "core/sigma_types.h"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign VESA Framebuffer Driver (S-VESA)
 * Implementation: Linear Frame Buffer (LFB) orchestration for high-res graphics.
 * Absorbed: VBE 2.0+ industrial display standards.
 */

namespace SigmaOS {
namespace Kernel {
namespace Drivers {

struct VesaInfo {
    sigma_u32 width;
    sigma_u32 height;
    sigma_u32 bpp;
    sigma_u64 phys_addr;
} SIGMA_PACKED;

class SovereignVESA : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignVESA> {
    friend class SigmaOS::SigmaSingleton<SovereignVESA>;
public:
    const char* type_name() const noexcept override { return "SovereignVESA"; }

    void init(sigma_u32 w, sigma_u32 h, sigma_u32 bpp, sigma_u64 lfb_phys) {
        m_info.width = w;
        m_info.height = h;
        m_info.bpp = bpp;
        m_info.phys_addr = lfb_phys;

        sigma_log_info("[VESA] Initializing Sovereign LFB: %ux%ux%u @ 0x%016llX", w, h, bpp, lfb_phys);
        sigma_log_info("[VESA] Zenith Compositor: READY.");
    }

    void drawPixel(sigma_u32 x, sigma_u32 y, sigma_u32 color) {
        if (x >= m_info.width || y >= m_info.height) return;
        
        sigma_u32* fb = (sigma_u32*)m_info.phys_addr;
        fb[y * m_info.width + x] = color;
    }

    void clear(sigma_u32 color) {
        sigma_u32* fb = (sigma_u32*)m_info.phys_addr;
        for (sigma_u32 i = 0; i < m_info.width * m_info.height; i++) {
            fb[i] = color;
        }
    }

private:
    SovereignVESA() : m_info{0,0,0,0} {}
    VesaInfo m_info;
};

} // namespace Drivers
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void vesa_init(sigma_u32 w, sigma_u32 h, sigma_u32 bpp, sigma_u64 lfb) { 
        SigmaOS::Kernel::Drivers::SovereignVESA::getInstance().init(w, h, bpp, lfb); 
    }
    void vesa_put_pixel(sigma_u32 x, sigma_u32 y, sigma_u32 color) {
        SigmaOS::Kernel::Drivers::SovereignVESA::getInstance().drawPixel(x, y, color);
    }
}
