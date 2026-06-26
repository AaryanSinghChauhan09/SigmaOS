// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_scheme.cpp — resource URL dispatcher (Redox OS-inspired)
 */
#include "include/sigma_scheme.h"
#include "sigma_log.h"
#include <string.h>
#include <stdlib.h>

#define MAX_SCHEMES  32
#define MAX_OPEN_FDS 256

static const sigma_scheme_t* g_schemes[MAX_SCHEMES];
static int g_scheme_count = 0;

/* Open fd → (scheme, path) mapping */
static struct { const sigma_scheme_t* scheme; int scheme_fd; bool used; }
    g_fd_table[MAX_OPEN_FDS];

void sigma_scheme_register(const sigma_scheme_t* s) {
    if (g_scheme_count >= MAX_SCHEMES) {
        sigma_log_err("[sigma-scheme] scheme table full\n");
        return;
    }
    g_schemes[g_scheme_count++] = s;
    sigma_log_info("[sigma-scheme] registered scheme: %s\n", s->scheme);
}

/* Parse "scheme:rest" → find handler, call open */
int sigma_open(const char* url, int flags) {
    /* Find the colon */
    const char* colon = strchr(url, ':');
    if (!colon) {
        sigma_log_err("[sigma-scheme] no scheme in URL: %s\n", url);
        return -1;
    }

    size_t scheme_len = (size_t)(colon - url);
    char   scheme_name[64] = {};
    if (scheme_len >= sizeof(scheme_name)) return -1;
    memcpy(scheme_name, url, scheme_len);

    const char* path = colon + 1;

    /* Find the registered scheme */
    for (int i = 0; i < g_scheme_count; i++) {
        if (strcmp(g_schemes[i]->scheme, scheme_name) == 0) {
            int scheme_fd = g_schemes[i]->open(path, flags, g_schemes[i]->ctx);
            if (scheme_fd < 0) return scheme_fd;

            /* Allocate a global fd */
            for (int j = 3; j < MAX_OPEN_FDS; j++) {
                if (!g_fd_table[j].used) {
                    g_fd_table[j].scheme    = g_schemes[i];
                    g_fd_table[j].scheme_fd = scheme_fd;
                    g_fd_table[j].used      = true;
                    return j;
                }
            }
            sigma_log_err("[sigma-scheme] fd table full\n");
            return -1;
        }
    }

    sigma_log_err("[sigma-scheme] unknown scheme: %s\n", scheme_name);
    return -1;
}

ssize_t sigma_read(int fd, void* buf, size_t len) {
    if (fd < 0 || fd >= MAX_OPEN_FDS || !g_fd_table[fd].used) return -1;
    auto& e = g_fd_table[fd];
    return e.scheme->read(e.scheme_fd, buf, len, e.scheme->ctx);
}

ssize_t sigma_write(int fd, const void* buf, size_t len) {
    if (fd < 0 || fd >= MAX_OPEN_FDS || !g_fd_table[fd].used) return -1;
    auto& e = g_fd_table[fd];
    return e.scheme->write(e.scheme_fd, buf, len, e.scheme->ctx);
}

int sigma_close(int fd) {
    if (fd < 0 || fd >= MAX_OPEN_FDS || !g_fd_table[fd].used) return -1;
    auto& e = g_fd_table[fd];
    int rc = e.scheme->close(e.scheme_fd, e.scheme->ctx);
    e.used = false;
    return rc;
}

void sigma_scheme_init(void) {
    for (int i = 0; i < MAX_OPEN_FDS; i++) g_fd_table[i].used = false;
    sigma_log_info("[sigma-scheme] dispatcher initialised\n");
}
