#ifndef SIGMA_SNAP_H
#define SIGMA_SNAP_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SNAP_ZONE_LEFT_HALF,
    SNAP_ZONE_RIGHT_HALF,
    SNAP_ZONE_TOP_HALF,
    SNAP_ZONE_BOTTOM_HALF,
    SNAP_ZONE_QUARTER_TL,
    SNAP_ZONE_QUARTER_TR,
    SNAP_ZONE_QUARTER_BL,
    SNAP_ZONE_QUARTER_BR,
    SNAP_ZONE_CENTER_FLOAT,
    SNAP_ZONE_FULLSCREEN
} sigma_snap_zone_id_t;

typedef struct {
    sigma_u32 x, y, w, h;
    sigma_snap_zone_id_t zone_id;
} sigma_snap_zone_t;

/* --- Window Snapping Primitives --- */
void snap_init(void);
void snap_window_to_zone(uint32_t window_id, sigma_snap_zone_id_t zone);
void snap_auto_arrange(void);
void snap_register_zone(sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h);

#ifdef __cplusplus
}

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

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SNAP_H */
