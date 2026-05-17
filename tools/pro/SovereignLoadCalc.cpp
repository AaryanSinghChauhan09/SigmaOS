/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PRO TOOL - SovereignLoadCalc
 * =========================================================================
 * REGULATORY CONTEXT: BIS IS-875 (Part 1, 2, 3) / Building Structural Design
 * Principle: Bare-metal execution, zero standard library dependencies.
 * =========================================================================
 */
#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"

namespace SigmaOS {
namespace ProTools {

class SovereignLoadCalc {
public:
    void init() {
        sigma_log_info("[SovereignLoad] Structural Load Calculator (BIS IS-875 Compliant) initialized.");
    }

    // Computes Slab Dead Load and Occupancy Live Load as per IS-875 Part 1 & 2
    // Loads returned in Pascals (N/m^2)
    sigma_u32 calculate_gravity_loads(sigma_u32 slab_thickness_mm, sigma_u32 finishing_thickness_mm, 
                                      sigma_u32 occupancy_type_residential_vs_commercial,
                                      sigma_u32* out_dead_load_pa, sigma_u32* out_live_load_pa, 
                                      sigma_u32* out_total_gravity_load_pa) {
        
        // RCC concrete density = 25 kN/m^3 (represented as 25000 N/m^3)
        // Floor finish density = 24 kN/m^3 (represented as 24000 N/m^3)
        sigma_u32 rcc_dead = (slab_thickness_mm * 25000) / 1000;
        sigma_u32 finish_dead = (finishing_thickness_mm * 24000) / 1000;
        
        *out_dead_load_pa = rcc_dead + finish_dead;

        // Occupancy loads (IS-875 Part 2): 
        // Residential: 2 kN/m^2 (2000 N/m^2), Commercial: 4 kN/m^2 (4000 N/m^2)
        if (occupancy_type_residential_vs_commercial == 0) {
            *out_live_load_pa = 2000; // Residential
        } else {
            *out_live_load_pa = 4000; // Commercial
        }

        *out_total_gravity_load_pa = *out_dead_load_pa + *out_live_load_pa;

        sigma_log_info("[SovereignLoad] RCC Slab: %umm | Finish: %umm | Dead: %u N/m^2 | Live: %u N/m^2 | Total: %u N/m^2",
                       slab_thickness_mm, finishing_thickness_mm, *out_dead_load_pa, *out_live_load_pa, *out_total_gravity_load_pa);

        return SIGMA_OK;
    }

    // Computes Wind Pressure as per IS-875 Part 3: Pz = 0.6 * (Vz)^2
    // Vz is design wind speed at height z
    // Returns wind pressure in Pascals scaled by 100 to avoid floating point issues
    sigma_u32 calculate_wind_pressure_scaled(sigma_u32 design_wind_speed_m_per_s, sigma_u32* out_wind_pressure_scaled) {
        // Pz = 0.6 * Vz * Vz -> scaled by 100 is: 60 * Vz * Vz / 100
        sigma_u32 pz_scaled = 60 * design_wind_speed_m_per_s * design_wind_speed_m_per_s;
        *out_wind_pressure_scaled = pz_scaled;

        sigma_log_info("[SovereignLoad] Wind Speed: %u m/s | Wind Pressure: %u.%02u N/m^2",
                       design_wind_speed_m_per_s, pz_scaled / 100, pz_scaled % 100);

        return SIGMA_OK;
    }
};

} // namespace ProTools
} // namespace SigmaOS

extern "C" {
    void load_init() {
        SigmaOS::ProTools::SovereignLoadCalc calc;
        calc.init();
    }

    sigma_u32 load_calculate_gravity(sigma_u32 thickness, sigma_u32 finish, sigma_u32 type, 
                                     sigma_u32* dl, sigma_u32* ll, sigma_u32* total) {
        SigmaOS::ProTools::SovereignLoadCalc calc;
        return calc.calculate_gravity_loads(thickness, finish, type, dl, ll, total);
    }

    sigma_u32 load_calculate_wind(sigma_u32 speed, sigma_u32* pressure_scaled) {
        SigmaOS::ProTools::SovereignLoadCalc calc;
        return calc.calculate_wind_pressure_scaled(speed, pressure_scaled);
    }
}
