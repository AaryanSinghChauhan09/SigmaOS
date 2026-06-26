// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/* sigma_delta.h — binary delta updates (Clear Linux swupd-inspired) */
#include <stdint.h>
#include <stddef.h>

typedef struct {
    uint32_t from_version;
    uint32_t to_version;
    char     bundle_name[64];
    char     sha256[65];       /* of the delta file itself */
    uint64_t delta_size;
    uint64_t full_size;
} sigma_delta_manifest_t;

typedef enum {
    SIGMA_UPDATE_USE_DELTA,
    SIGMA_UPDATE_USE_FULL,
} sigma_update_strategy_t;

/* Apply a bsdiff binary delta: from_path + delta_path → to_path */
int sigma_delta_apply(const char* from_path,
                       const char* delta_path,
                       const char* to_path);

/* Choose download strategy: delta if < 70% of full size */
sigma_update_strategy_t sigma_delta_choose_strategy(
    const sigma_delta_manifest_t* m);
