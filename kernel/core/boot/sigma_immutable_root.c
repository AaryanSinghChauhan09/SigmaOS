/*
 * Immutable root filesystem policy — Fedora CoreOS / Flatcar class behavior.
 */
#include "../../../include/sigma_kernel_types.h"

extern void sigma_puts(const char* s);

static sigma_bool g_immutable = SIGMA_TRUE;
static sigma_bool g_verified;

void sigma_immutable_root_init(void) {
    g_verified = SIGMA_TRUE;
    sigma_puts("[immutable] Root filesystem locked (A/B updates via rollback gate).\n");
}

sigma_bool sigma_immutable_root_is_locked(void) {
    return g_immutable;
}

sigma_bool sigma_immutable_root_allow_write(const char* path) {
  (void)path;
    if (!g_immutable) return SIGMA_TRUE;
    /* /var, /home overlays are writable in full implementation */
    return SIGMA_FALSE;
}

void sigma_immutable_root_set_locked(sigma_bool locked) {
    g_immutable = locked;
}
