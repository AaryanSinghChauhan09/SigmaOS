#ifndef SOVEREIGN_MATH_HPP
#define SOVEREIGN_MATH_HPP

#include "../include/core/sigma_types.h"

namespace SigmaOS {
namespace Core {

/*
 * =========================================================================
 * SOVEREIGN MATH (Silicon-Native / Zero-Dependency)
 * =========================================================================
 * Industrial-grade math shards for high-performance graphics, neural 
 * calculations, and relativistic drift correction.
 */
class SovereignMath {
public:
    // Fast Inverse Square Root (Quake-style, industrial variant)
    static sigma_f32 FastInvSqrt(sigma_f32 number) {
        sigma_i32 i;
        sigma_f32 x2, y;
        const sigma_f32 threehalfs = 1.5F;

        x2 = number * 0.5F;
        y  = number;
        i  = *(sigma_i32*)&y;                       // evil floating point bit level hacking
        i  = 0x5f3759df - (i >> 1);               // what the...
        y  = *(sigma_f32*)&i;
        y  = y * (threehalfs - (x2 * y * y));   // 1st iteration
        // y  = y * (threehalfs - (x2 * y * y));   // 2nd iteration, this can be removed

        return y;
    }

    static sigma_f64 Absolute(sigma_f64 val) { return (val < 0) ? -val : val; }
    
    // Fixed-point sharding for deterministic silicon math
    static sigma_i64 FixedMultiply(sigma_i64 a, sigma_i64 b, sigma_i32 shift) {
        return (a * b) >> shift;
    }
};

} // namespace Core
} // namespace SigmaOS

#endif
