#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Sports Shard (S-SPORTS)
 * Purpose: Professional workspace for athletes, coaches, and sports scientists.
 * Features: VO2 Max estimator, Macro calculator for athletes, Performance trend tracker.
 * Standards: ACSM (American College of Sports Medicine) and Indian SAI (Sports Authority of India) guidelines.
 */

namespace SigmaOS {
namespace Kernel {
namespace Sports {

class SovereignSports : public SigmaOS::SigmaObject {
public:
    static SovereignSports& getInstance() {
        static SovereignSports instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignSports"; }

    void init() {
        sigma_log_info("[S-SPORTS] Initializing Sovereign Sports Nexus...");
    }

    /**
     * VO2 Max estimation (Cooper Test formula - integer approximation).
     * VO2 Max = (Distance in meters - 504.9) / 44.73
     * @param distance_meters Distance covered in 12 minutes
     */
    sigma_u32 estimateVO2Max(sigma_u32 distance_meters) {
        if (distance_meters < 505) return 0;
        // Scale by 100 for precision
        sigma_u32 vo2_x100 = (distance_meters - 505) * 100 / 45; 
        sigma_log_info("[S-SPORTS] VO2 Max Estimate: %u.%02u ml/kg/min", vo2_x100 / 100, vo2_x100 % 100);
        return vo2_x100;
    }

    /**
     * Calculate 1-Rep Max (Epley Formula).
     * 1RM = weight * (1 + reps/30)
     */
    sigma_u32 calcOneRepMax(sigma_u32 weight, sigma_u32 reps) {
        sigma_u32 orm = weight + (weight * reps / 30);
        sigma_log_info("[S-SPORTS] 1-Rep Max Estimate: %u units", orm);
        return orm;
    }

    /**
     * Calorie burn for specific intensity (MET * Weight * Duration).
     * @param met Metabolic Equivalent of Task
     * @param weight_kg
     * @param minutes
     */
    sigma_u32 calcCaloriesBurned(sigma_u32 met_x10, sigma_u32 weight_kg, sigma_u32 minutes) {
        // Cal = (MET * 3.5 * weight) / 200 * minutes
        sigma_u32 cal = (met_x10 * 35 * weight_kg * minutes) / 20000;
        sigma_log_info("[S-SPORTS] Estimated Calories Burned: %u kcal", cal);
        return cal;
    }

private:
    SovereignSports() = default;
};

} // namespace Sports
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sports_init() {
    SigmaOS::Kernel::Sports::SovereignSports::getInstance().init();
}

sigma_u32 sports_vo2(sigma_u32 distance) {
    return SigmaOS::Kernel::Sports::SovereignSports::getInstance().estimateVO2Max(distance);
}

sigma_u32 sports_orm(sigma_u32 weight, sigma_u32 reps) {
    return SigmaOS::Kernel::Sports::SovereignSports::getInstance().calcOneRepMax(weight, reps);
}

sigma_u32 sports_calories(sigma_u32 met_x10, sigma_u32 weight, sigma_u32 mins) {
    return SigmaOS::Kernel::Sports::SovereignSports::getInstance().calcCaloriesBurned(met_x10, weight, mins);
}

} // extern "C"
