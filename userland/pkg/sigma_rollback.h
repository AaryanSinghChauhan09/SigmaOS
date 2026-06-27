// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_rollback.h — Package and OS generation rollback interface
 * Inspired by NixOS generations, rpm-ostree, and Guix system.
 */
#include <sigma_kernel_types.h>

namespace sigma::pkg {

class RollbackEngine {
public:
    /* Snapshot current system state before a transaction. */
    static int take_snapshot(const char *description, sigma_u32 *out_gen_id);

    /* Roll back to a specific generation. Requires reboot to complete. */
    static int rollback(sigma_u32 gen_id);

    /* List all available generation IDs (sorted oldest→newest). */
    static int list_generations(sigma_u32 *ids_out, int max, int *count_out);

    /* Delete a specific generation (frees disk space). */
    static int delete_generation(sigma_u32 gen_id);

    /* Prune old generations keeping at most MAX_GENERATIONS. */
    static void prune_old_generations();
};

} // namespace sigma::pkg
