// SigmaOS — Sigma-PKG: Sovereign Package Manager (Linux apt/pacman-inspired)
// Module: sigma-pkg
// USP: No dpkg/rpm/libalpm — pure C, hash-verified, capability-gated installs
// Each package is a signed capability token bundle — no root required

#ifndef SIGMA_PKG_H
#define SIGMA_PKG_H

#include "include/sigma_types.h"
#include "../../include/S43_SovereignCaps/sigma_caps.h"

#define SIGMA_PKG_MAX        128
#define SIGMA_PKG_NAME_LEN   32
#define SIGMA_PKG_VER_LEN    16

typedef enum SigmaPkgState {
    PKG_UNINSTALLED = 0,
    PKG_INSTALLED   = 1,
    PKG_BROKEN      = 2,
    PKG_HELD        = 3
} SigmaPkgState;

typedef struct SigmaPkg {
    char          name[SIGMA_PKG_NAME_LEN];
    char          version[SIGMA_PKG_VER_LEN];
    sigma_u64     content_hash;   // FNV-1a of package content
    SigmaPkgState state;
    unsigned char requires_cap;   // SIGMA_CAP_* flags
    int           depends_on;     // pkg_id of dependency, or -1
} SigmaPkg;

typedef struct SigmaPkgDB {
    SigmaPkg     packages[SIGMA_PKG_MAX];
    unsigned int count;
} SigmaPkgDB;

// FNV-1a hash (reused from journal)
static inline sigma_u64 pkg_fnv1a(const unsigned char* d, sigma_u64 n) {
    sigma_u64 h = 14695981039346656037ULL, p = 1099511628211ULL;
    for (sigma_u64 i = 0; i < n; i++) { h ^= d[i]; h *= p; }
    return h;
}

static inline void pkgdb_init(SigmaPkgDB* db) { db->count = 0; }

// Register a package into the DB
static inline int pkg_register(SigmaPkgDB* db, const char* name,
                                 const char* ver, sigma_u64 hash,
                                 unsigned char cap_req) {
    if (db->count >= SIGMA_PKG_MAX) return -1;
    SigmaPkg* p = &db->packages[db->count++];
    // manual strncpy
    for (int i = 0; i < SIGMA_PKG_NAME_LEN - 1 && name[i]; i++) p->name[i] = name[i];
    for (int i = 0; i < SIGMA_PKG_VER_LEN  - 1 && ver[i];  i++) p->version[i] = ver[i];
    p->content_hash = hash;
    p->state        = PKG_UNINSTALLED;
    p->requires_cap = cap_req;
    p->depends_on   = -1;
    return (int)(db->count - 1);
}

static inline int pkg_register_with_dep(SigmaPkgDB* db, const char* name,
                                 const char* ver, sigma_u64 hash,
                                 unsigned char cap_req, int dep_id) {
    int id = pkg_register(db, name, ver, hash, cap_req);
    if (id >= 0) db->packages[id].depends_on = dep_id;
    return id;
}

// Install a package — verified by capability token
static inline int pkg_install(SigmaPkgDB* db, unsigned int pkg_id,
                                SigmaCapToken* tok,
                                const unsigned char* data, sigma_u64 len) {
    if (pkg_id >= db->count) return -1;
    SigmaPkg* p = &db->packages[pkg_id];

    // Dependency Check
    if (p->depends_on >= 0) {
        if (db->packages[p->depends_on].state != PKG_INSTALLED) return -4; // dependency missing
    }

    if (!cap_check(tok, p->requires_cap)) return -2; // permission denied
    sigma_u64 actual = pkg_fnv1a(data, len);
    if (actual != p->content_hash) return -3; // integrity violation
    p->state = PKG_INSTALLED;
    return 0;
}

// Remove a package
static inline int pkg_remove(SigmaPkgDB* db, unsigned int pkg_id,
                               SigmaCapToken* tok) {
    if (pkg_id >= db->count) return -1;
    if (!cap_check(tok, SIGMA_CAP_ADMIN)) return -2;
    db->packages[pkg_id].state = PKG_UNINSTALLED;
    return 0;
}

// Find package by name — returns id or -1
static inline int pkg_find(SigmaPkgDB* db, const char* name) {
    for (unsigned int i = 0; i < db->count; i++) {
        const char* n = db->packages[i].name;
        const char* s = name;
        while (*n && *s && *n == *s) { n++; s++; }
        if (!*n && !*s) return (int)i;
    }
    return -1;
}

#endif /* SIGMA_PKG_H */
