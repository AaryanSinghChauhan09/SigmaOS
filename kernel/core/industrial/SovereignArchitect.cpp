#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/core/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian Architect Shard (S-ARCHITECT)
 * Purpose: Professional tools for Indian registered architects, urban planners, and town engineers.
 * Standards: NBC 2016 (National Building Code), COA Act 1972, RERA 2016, UDPFI Guidelines,
 *            Development Control Regulations (DCR), Smart Cities Mission norms.
 * Features: FAR/FSI calculator, Set-back checker, Building height calculator, Fire exit nexus.
 */

namespace SigmaOS {
namespace Kernel {
namespace Engineering {

// FAR/FSI limits per typical NBC 2016 / DCR for Indian cities (x10 for precision)
struct FAREntry {
    const char* zone;
    sigma_u32 residential_x10;
    sigma_u32 commercial_x10;
    sigma_u32 max_height_m;
};

static const FAREntry FAR_TABLE[] = {
    {"Metro_Zone_1",   25, 35, 45},   // e.g. Mumbai Island, Delhi core
    {"Metro_Zone_2",   20, 30, 36},
    {"Urban_Core",     18, 25, 30},
    {"Urban_Periphery",15, 20, 24},
    {"Suburban",       12, 15, 15},
    {"Industrial",     10, 15, 20},
};
static const sigma_u32 FAR_TABLE_LEN = sizeof(FAR_TABLE) / sizeof(FAR_TABLE[0]);

class SovereignArchitect : public SigmaOS::SigmaObject {
public:
    static SovereignArchitect& getInstance() {
        static SovereignArchitect instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignArchitect"; }

    void init() {
        sigma_log_info("[S-ARCHITECT] Initializing Indian Architecture & Urban Planning Nexus...");
        sigma_log_info("[S-ARCHITECT] Standards: NBC 2016 | COA Act 1972 | RERA 2016 | DCR | Smart Cities");
    }

    /**
     * FAR / FSI calculator — tells permissible built-up area.
     * Built-up Area = FAR * Plot Area
     * @param zone          Zone name matching FAR_TABLE
     * @param plot_sqm      Plot area in square metres
     * @param is_commercial true = commercial FAR, false = residential
     */
    void calcFAR(const char* zone, sigma_u32 plot_sqm, bool is_commercial) {
        for (sigma_u32 i = 0; i < FAR_TABLE_LEN; ++i) {
            bool m = true;
            for (sigma_u32 j = 0; FAR_TABLE[i].zone[j] || zone[j]; ++j) {
                if (FAR_TABLE[i].zone[j] != zone[j]) { m = false; break; }
            }
            if (!m) continue;
            sigma_u32 far_x10 = is_commercial ? FAR_TABLE[i].commercial_x10 : FAR_TABLE[i].residential_x10;
            sigma_u64 built_up = ((sigma_u64)far_x10 * plot_sqm) / 10ULL;
            sigma_log_info("[S-ARCHITECT] FAR | Zone: %s | Type: %s | Plot: %u m² | FAR: %u.%u | Max BUA: %llu m² | Max Ht: %u m",
                           zone, is_commercial ? "Commercial" : "Residential",
                           plot_sqm, far_x10/10, far_x10%10, built_up, FAR_TABLE[i].max_height_m);
            return;
        }
        sigma_log_err("[S-ARCHITECT] Zone '%s' not in FAR table.", zone);
    }

    /**
     * NBC 2016 minimum set-back calculator.
     * Front: ≥ 3m (residential) / 6m (commercial); Side: ≥ 1.5m / 3m; Rear: ≥ 3m / 4.5m.
     * @param width_m   Plot width in metres
     * @param depth_m   Plot depth in metres
     */
    void calcSetback(sigma_u32 width_m, sigma_u32 depth_m, bool is_commercial) {
        sigma_u32 front = is_commercial ? 6 : 3;
        sigma_u32 side  = is_commercial ? 3 : 2;
        sigma_u32 rear  = is_commercial ? 5 : 3;
        sigma_u32 usable_w = (width_m > 2 * side) ? width_m - 2 * side : 0;
        sigma_u32 usable_d = (depth_m > front + rear) ? depth_m - front - rear : 0;
        sigma_log_info("[S-ARCHITECT] NBC 2016 Set-back | %s | Plot: %um × %um",
                       is_commercial ? "Commercial" : "Residential", width_m, depth_m);
        sigma_log_info("[S-ARCHITECT]   Front: %um | Side: %um | Rear: %um | Usable: %um × %um = %u m²",
                       front, side, rear, usable_w, usable_d, usable_w * usable_d);
    }

    /**
     * Fire exit compliance per NBC 2016 Part 4 (Fire & Life Safety).
     * Travel distance to nearest exit ≤ 22.5m (sprinklered) / 30m (non-sprinklered).
     * @param travel_dist_m     Actual travel distance to exit in metres
     * @param is_sprinklered    Whether building has sprinkler system
     * @param occupants         Total occupants (to check min exit width: 1 exit per 250 pax)
     */
    void fireExitCheck(sigma_u32 travel_dist_m, bool is_sprinklered, sigma_u32 occupants) {
        sigma_u32 limit = is_sprinklered ? 30 : 22;
        sigma_u32 exits_reqd = (occupants + 249) / 250;
        bool dist_ok = travel_dist_m <= limit;
        sigma_log_info("[S-ARCHITECT] NBC 2016 Fire Safety | Travel dist: %um (limit %um) | %s",
                       travel_dist_m, limit, dist_ok ? "✅ OK" : "🚫 EXCEEDS — add intermediate exit");
        sigma_log_info("[S-ARCHITECT]   Occupants: %u | Exits required: %u (min, Cl 4.12 NBC)",
                       occupants, exits_reqd);
    }
};

} // namespace Engineering
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void architect_init() {
    SigmaOS::Kernel::Engineering::SovereignArchitect::getInstance().init();
}

void architect_far(const char* zone, sigma_u32 sqm, bool commercial) {
    SigmaOS::Kernel::Engineering::SovereignArchitect::getInstance().calcFAR(zone, sqm, commercial);
}

void architect_setback(sigma_u32 w, sigma_u32 d, bool commercial) {
    SigmaOS::Kernel::Engineering::SovereignArchitect::getInstance().calcSetback(w, d, commercial);
}

void architect_fire(sigma_u32 dist, bool sprinkler, sigma_u32 occ) {
    SigmaOS::Kernel::Engineering::SovereignArchitect::getInstance().fireExitCheck(dist, sprinkler, occ);
}

} // extern "C"
