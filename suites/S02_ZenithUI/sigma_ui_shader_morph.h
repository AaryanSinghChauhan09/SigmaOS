// SigmaOS — sigma-ui-shader-morph: Vulkan Morphic Shaders
// Module: sigma-ui-shader-morph
// USP: High-performance C struct definitions mapping directly to Vulkan push constants and uniform buffers for glassmorphism and blur.

#ifndef SIGMA_UI_SHADER_MORPH_H
#define SIGMA_UI_SHADER_MORPH_H

namespace sigma {
namespace ui {

// Maps directly to a Vulkan Push Constant for real-time shader morphing
struct MorphicPushConstants {
    float time;           // RDTSC-derived time for animations
    float blur_radius;    // Glassmorphism intensity
    float opacity;        // Base opacity
    float corner_radius;  // Rounded corners
    float resolution[2];  // Screen res
};

class ShaderMorpher {
public:
    void apply_glassmorphism(MorphicPushConstants* push_constants, float intensity) {
        if (!push_constants) return;
        push_constants->blur_radius = intensity * 20.0f; 
        push_constants->opacity = 0.85f - (intensity * 0.2f);
    }

    void update_time(MorphicPushConstants* push_constants, unsigned long rdtsc_cycles) {
        if (!push_constants) return;
        push_constants->time = (float)(rdtsc_cycles % 1000000) / 1000000.0f;
    }
};

} // namespace ui
} // namespace sigma

#endif /* SIGMA_UI_SHADER_MORPH_H */
