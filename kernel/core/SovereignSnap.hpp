#ifndef SOVEREIGN_SNAP_HPP
#define SOVEREIGN_SNAP_HPP

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_snap.h"

class SovereignSnapEngine {
public:
    static SovereignSnapEngine& getInstance() {
        static SovereignSnapEngine instance;
        return instance;
    }

    void init();
    void applyLayout(sigma_u32 layout_id);
    void registerZone(sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h);

private:
    SovereignSnapEngine() : active_zone_count(0), initialized(0) {}
    
    sigma_snap_zone_t zones[8];
    sigma_u32 active_zone_count;
    sigma_u32 initialized;
};

typedef sigma_u32 sigma_snap_zone_id_t;

#endif
