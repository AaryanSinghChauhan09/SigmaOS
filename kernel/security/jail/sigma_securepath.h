// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
#include <stddef.h>

/*
 * Resolve unsafe_path relative to root, preventing any symlink escape.
 * Returns 0 on success, -1 if the path escapes root.
 */
int sigma_secure_join(const char* root, const char* unsafe_path,
                       char* out, size_t out_len);
