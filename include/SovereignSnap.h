#include "./sigma_types.h"
#ifndef SOVEREIGN_SNAP_H
#define SOVEREIGN_SNAP_H

#include "./core/sigma_kernel_types.h"
#include "./sigma_snap_types.h"
#include "./SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Snap Engine
 * Principles: Spatial Lattice, Golden Ratio Layouts, Intent-Based Snapping.
 */

namespace SigmaOS {
namespace Kernel {
namespace UI {

class SovereignSnapEngine : public SigmaObject {
public:
    static SovereignSnapEngine& getInstance();
    
    const char* type_name() const noexcept override { return "SovereignSnapEngine"; }
    
    void init();
    void applyLayout(sigma_u32 layout_id);
    void registerZone(sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h);

private:
    SovereignSnapEngine();
    sigma_u32 initialized;
    sigma_u32 active_zone_count;
    sigma_snap_zone_t m_zones[8];
};

} // namespace UI
} // namespace Kernel
} // namespace SigmaOS

#endif
