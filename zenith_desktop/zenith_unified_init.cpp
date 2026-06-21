/**
 * Zenith unified subsystem bootstrap — wires profile → theme → WM → compositor.
 */
#include "../include/sigma_kernel_types.h"

extern "C" void zenith_compositor_init(void);
extern "C" void zenith_compositor_run_loop(sigma_u32 frames);
extern "C" void zenith_theme_init(void);
extern "C" void zenith_theme_set_metrics(sigma_u32 r, sigma_u32 ig, sigma_u32 og);
extern "C" void zenith_profile_init(void);
extern "C" sigma_status sigma_wm_init(sigma_u32 w, sigma_u32 h);
extern "C" sigma_status sigma_wm_gaps(sigma_u32 inner, sigma_u32 outer);
extern "C" sigma_status sigma_wm_layout(sigma_u32 mode);
extern "C" sigma_status sigma_wm_auto_tile(void);

namespace Zenith {
namespace Unified {

static sigma_u32 layout_name_to_mode(const char* name) {
    if (!name) return 6;
    if (sigma_strcmp(name, "bsp") == 0) return 0;
    if (sigma_strcmp(name, "columns") == 0) return 1;
    if (sigma_strcmp(name, "floating") == 0) return 3;
    if (sigma_strcmp(name, "monocle") == 0) return 4;
    if (sigma_strcmp(name, "grid") == 0) return 5;
    return 6; /* master-stack */
}

void subsystem_init(sigma_u32 screen_w, sigma_u32 screen_h) {
    zenith_theme_init();
    zenith_compositor_init();
    sigma_wm_init(screen_w, screen_h);
    zenith_profile_init();
}

void run_desktop_loop(sigma_u32 frames) {
    zenith_compositor_run_loop(frames);
}

} // namespace Unified
} // namespace Zenith

extern "C" void zenith_subsystem_init(sigma_u32 w, sigma_u32 h) {
    Zenith::Unified::subsystem_init(w, h);
}

extern "C" void zenith_subsystem_run(sigma_u32 frames) {
    Zenith::Unified::run_desktop_loop(frames);
}
