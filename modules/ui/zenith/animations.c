#include "libc/sigma_libc.h"
#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Zenith Micro-Animations Engine
// USP: Kernel-level spring physics for 120Hz smooth UX
// ---------------------------------------------------------

typedef struct {
    float current_val;
    float target_val;
    float velocity;
    float tension;  // Spring stiffness
    float friction; // Spring damping
} spring_anim_t;

// Standard Apple-like fluid spring constants
#define SPRING_TENSION_DEFAULT  300.0f
#define SPRING_FRICTION_DEFAULT 30.0f

// Process one frame of a spring animation
// Called during the v-sync interrupt (e.g. 120 times a second)
uint8_t anim_step_spring(spring_anim_t* anim, float delta_time) {
    if (!anim) return 0;

    // Hooke's Law: F = -k * x - c * v
    float displacement = anim->current_val - anim->target_val;
    float spring_force = -anim->tension * displacement;
    float damping_force = -anim->friction * anim->velocity;
    
    float acceleration = spring_force + damping_force; // Assumes mass = 1.0
    
    anim->velocity += acceleration * delta_time;
    anim->current_val += anim->velocity * delta_time;

    // Check if animation has settled
    if (displacement < 0.5f && displacement > -0.5f && 
        anim->velocity < 0.5f && anim->velocity > -0.5f) {
        anim->current_val = anim->target_val;
        anim->velocity = 0.0f;
        return 1; // Animation complete
    }

    return 0; // Still animating
}

// Example hook: Animate a window opening (scale from 0 to 1)
void anim_trigger_window_open(uint32_t window_id) {
    // spring_anim_t* scale_anim = zenith_get_window_scale_anim(window_id);
    // scale_anim->current_val = 0.0f;
    // scale_anim->target_val = 1.0f;
    // scale_anim->tension = SPRING_TENSION_DEFAULT;
    // scale_anim->friction = SPRING_FRICTION_DEFAULT;
}
