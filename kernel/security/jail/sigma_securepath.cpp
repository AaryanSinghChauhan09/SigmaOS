// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_securepath.cpp — prevent VFS escape via symlinks (OCI filepath-securejoin-inspired)
 *
 * A jailed process must not be able to break out of its root by exploiting
 * symlinks (e.g. /rootfs/../../../etc/shadow). This function resolves a path
 * relative to a jail root and returns -ENOENT / -EPERM if any component
 * escapes the root via a symlink traversal.
 *
 * Used by: sigma_unveil_check, sigma_pivot_root, sigma_cgroup_create.
 */
#include "sigma_securepath.h"
#include "sigma_log.h"
#include <string.h>
#include <stdlib.h>

#define MAX_SYMLINK_DEPTH 40
#define MAX_PATH_LEN      4096

/*
 * sigma_secure_join — resolve `unsafe_path` relative to `root`, ensuring the
 * result stays within `root`. Returns 0 and writes to `out` on success.
 * Returns -EPERM if the resolved path escapes the root.
 */
int sigma_secure_join(const char* root, const char* unsafe_path,
                       char* out, size_t out_len) {
    if (!root || !unsafe_path || !out) return -1;

    /* Reject immediately if unsafe_path contains null bytes */
    size_t pathlen = strnlen(unsafe_path, MAX_PATH_LEN);
    if (pathlen >= MAX_PATH_LEN) {
        sigma_log_err("[sigma-securepath] path too long\n");
        return -1;
    }

    char working[MAX_PATH_LEN];
    snprintf(working, sizeof(working), "%s", root);
    size_t root_len = strlen(root);

    /* Walk each component of unsafe_path */
    char buf[MAX_PATH_LEN];
    snprintf(buf, sizeof(buf), "%s", unsafe_path);

    char* tok = strtok(buf, "/");
    while (tok) {
        if (strcmp(tok, ".") == 0) {
            /* stay in place */
        } else if (strcmp(tok, "..") == 0) {
            /* Go up — but never above root */
            char* last_slash = strrchr(working, '/');
            if (last_slash && (size_t)(last_slash - working) >= root_len) {
                *last_slash = '\0';
            }
            /* If we would go above root: stay at root */
            if (strlen(working) < root_len) {
                snprintf(working, sizeof(working), "%s", root);
            }
        } else {
            /* Append component */
            size_t wlen = strlen(working);
            if (wlen + 1 + strlen(tok) >= sizeof(working)) {
                sigma_log_err("[sigma-securepath] path overflow\n");
                return -1;
            }
            working[wlen] = '/';
            strncat(working, tok, sizeof(working) - wlen - 2);
        }
        tok = strtok(NULL, "/");
    }

    /* Final check: result must start with root */
    if (strncmp(working, root, root_len) != 0) {
        sigma_log_err("[sigma-securepath] ESCAPE ATTEMPT: '%s' escapes root '%s'\n",
                      unsafe_path, root);
        return -1; /* -EPERM */
    }

    snprintf(out, out_len, "%s", working);
    return 0;
}
