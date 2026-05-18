#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Civil Engineering Shard (S-CIVIL)
 * Purpose: Professional environment for structural, geotechnical, and transport engineers.
 * Features: Beam deflection calculator, soil bearing capacity nexus, PQC-sealed site reports.
 */

namespace SigmaOS {
namespace Kernel {
namespace Engineering {

struct BeamParams {
    sigma_u32 length_mm;   // span in millimetres
    sigma_u32 load_n;      // point load in Newtons
    sigma_u32 e_mpa;       // Young's modulus (MPa)
    sigma_u32 i_mm4;       // Second moment of area (mm^4)
};

class SovereignCivil : public SigmaOS::SigmaObject {
public:
    static SovereignCivil& getInstance() {
        static SovereignCivil instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignCivil";
    }

    void init() {
        sigma_log_info("[S-CIVIL] Initializing Structural Engineering Nexus...");
    }

    /**
     * Calculate mid-span deflection of a simply supported beam under central point load.
     * Formula: delta = P*L^3 / (48*E*I)   [all in consistent mm/N/MPa units]
     * Returns deflection in micrometres to preserve integer precision.
     */
    sigma_u64 calcBeamDeflection(const BeamParams& p) {
        // (P * L^3) / (48 * E * I)
        // To avoid 32-bit overflow use 64-bit arithmetic throughout
        sigma_u64 L3 = (sigma_u64)p.length_mm * p.length_mm * p.length_mm;
        sigma_u64 numerator   = (sigma_u64)p.load_n * L3;          // N * mm^3
        sigma_u64 denominator = 48ULL * (sigma_u64)p.e_mpa * (sigma_u64)p.i_mm4; // MPa * mm^4

        if (denominator == 0) {
            sigma_log_err("[S-CIVIL] Invalid beam parameters (denominator == 0).");
            return 0;
        }

        sigma_u64 deflection_um = (numerator * 1000ULL) / denominator; // result in micrometres
        sigma_log_info("[S-CIVIL] Beam deflection: %llu µm (L=%umm, P=%uN, E=%uMPa)",
                       deflection_um, p.length_mm, p.load_n, p.e_mpa);
        return deflection_um;
    }

    /**
     * Estimate ultimate bearing capacity via Terzaghi's formula for strip footings
     * on cohesive soil (simplified: qu = c*Nc + q*Nq + 0.5*gamma*B*Ngamma).
     * Uses integer arithmetic; result in kPa.
     */
    sigma_u32 calcSoilBearing(sigma_u32 cohesion_kpa, sigma_u32 depth_m,
                               sigma_u32 unit_weight, sigma_u32 footing_width_m) {
        // Approximate bearing capacity factors for phi=0 (cohesive): Nc=5.7, Nq=1, Ng=0
        sigma_u32 qu = (cohesion_kpa * 57) / 10            // c * Nc
                     + unit_weight * depth_m * 1            // q * Nq (overburden)
                     + (unit_weight * footing_width_m) / 2; // 0.5 * gamma * B * Ngamma
        sigma_log_info("[S-CIVIL] Soil bearing capacity: %u kPa", qu);
        return qu;
    }
};

} // namespace Engineering
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void civil_init() {
    SigmaOS::Kernel::Engineering::SovereignCivil::getInstance().init();
}

sigma_u64 civil_beam_deflection(sigma_u32 len_mm, sigma_u32 load_n,
                                 sigma_u32 e_mpa, sigma_u32 i_mm4) {
    SigmaOS::Kernel::Engineering::BeamParams p{len_mm, load_n, e_mpa, i_mm4};
    return SigmaOS::Kernel::Engineering::SovereignCivil::getInstance().calcBeamDeflection(p);
}

sigma_u32 civil_soil_bearing(sigma_u32 c_kpa, sigma_u32 depth, sigma_u32 uw, sigma_u32 bw) {
    return SigmaOS::Kernel::Engineering::SovereignCivil::getInstance()
               .calcSoilBearing(c_kpa, depth, uw, bw);
}

} // extern "C"
 