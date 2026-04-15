/*
 * =========================================================================
 * S SIGMAOS ZENITH: SovereignOmniCLI_DistroAbsorber.h (v2.0)
 * =========================================================================
 * Absorbs command syntax from 20+ legacy Linux distributions.
 * All foreign package managers & shell syntaxes map to native Sovereign
 * ring-0 C11 shard execution primitives via the Omni-CLI.
 *
 * FIXED v2.0:
 *   - Moved mutable global array definition to SovereignOmniCLI_DistroAbsorber.c
 *     (was a multiple-definition ODR violation in every TU that included this header).
 *   - Changed sigma_strcmp → sigma_streq (correct SigmaOS LibC spelling).
 *   - sigma_print_info / sigma_print_warn → sigma_printf (available in all TUs).
 * =========================================================================
 */
#ifndef SOVEREIGN_OMNICLI_DISTRO_ABSORBER_H
#define SOVEREIGN_OMNICLI_DISTRO_ABSORBER_H

#include "sigma_libc.h"

/* -------------------------------------------------------------------------
 * Unified Absorption Token Structure
 * ---------------------------------------------------------------------- */
typedef struct {
    char legacy_command     [64];
    char legacy_distro_origin[32];
    char target_sigma_shard [64];
} OmniCLIPromptMapping_t;

/*
 * Table lives in SovereignOmniCLI_DistroAbsorber.c — extern declaration only.
 * Including TUs must NOT define this; they link against the single definition.
 */
extern OmniCLIPromptMapping_t g_omnicli_absorption_table[];

/*
 * @brief Map a raw legacy command string to the appropriate Sovereign shard.
 *        Returns the shard name string, or NULL if not recognised.
 */
const char *sigma_omnicli_map_command(const char *legacy_input);

/*
 * @brief Full absorb-and-dispatch: prints the mapping to the console and
 *        could call the native shard's init function.
 */
void sigma_omnicli_absorb_command(const char *legacy_input);

#endif /* SOVEREIGN_OMNICLI_DISTRO_ABSORBER_H */
