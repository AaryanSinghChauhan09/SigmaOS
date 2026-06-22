/*
 * =========================================================================
 * Σ SIGMAOS: LIBZENITH 2D RENDERER
 * =========================================================================
 * Native vector graphics drawing API.
 * =========================================================================
 */
#include "../../klib/include/sigma_stdio.h"

extern "C" void zenith_draw_rect(int x, int y, int width, int height) {
    sigma_printf("[libzenith] Drawing GPU rect at %d,%d (%dx%d)...\n", x, y, width, height);
}
