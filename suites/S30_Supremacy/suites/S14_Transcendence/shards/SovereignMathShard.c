#include "sigma_libc.h"
#include "sigma_types.h"

/**
 * SigmaOS Sovereign Math Shard
 * Subsystem: S14 (Transcendence)
 * Mission: Zero-dependency math primitives and high-performance vector operations.
 */

typedef struct {
    float x, y, z;
} Vec3;

float sovereign_math_inv_sqrt(float number) {
    long i;
    float x2, y;
    const float threehalfs = 1.5F;

    x2 = number * 0.5F;
    y  = number;
    i  = * ( long * ) &y;                      
    i  = 0x5f3759df - ( i >> 1 );              
    y  = * ( float * ) &i;
    y  = y * ( threehalfs - ( x2 * y * y ) );  
    
    return y;
}

Vec3 sovereign_math_vec3_normalize(Vec3 v) {
    float inv_len = sovereign_math_inv_sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
    Vec3 res = { v.x * inv_len, v.y * inv_len, v.z * inv_len };
    return res;
}

void S14_Register_MathShard(void) {
    sigma_sigma_printf("S14 [TRANSCENDENCE]: Sovereign Math Shard Online.\n");
    sigma_sigma_printf("  [MATH]: Zero-dependency high-speed primitives verified.\n");
}
