/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SHARD REGISTRY (v2.0)
 * =========================================================================
 * Mission: Dynamic management and auditing of all kernel shards.
 * Design: Zero-Dependency / C11 / High-Performance Registry.
 * =========================================================================
 */

#ifndef SOVEREIGN_REGISTRY_H
#define SOVEREIGN_REGISTRY_H

/* Directly include the base primitives to bypass orchestrator latency */
typedef unsigned long long sigma_u64;
typedef unsigned int       sigma_u32;
typedef signed int         sigma_err_t;

#define MAX_SHARDS 1024
#define SHARD_NAME_MAX 64

typedef enum {
    SHARD_CAT_CORE,     /* S01-S05: VFS, MM, SCHED */
    SHARD_CAT_DISTRO,   /* Nix, Arch, Gentoo Absorption */
    SHARD_CAT_SECURITY, /* PQC, LSM, Enclaves */
    SHARD_CAT_PLATFORM, /* Android, Windows, macOS Parity */
    SHARD_CAT_TOOL      /* Excel, Python, PowerBI Shards */
} shard_category_t;

typedef enum {
    SHARD_STATUS_REGISTERED,
    SHARD_STATUS_INITIALIZING,
    SHARD_STATUS_ACTIVE,
    SHARD_STATUS_FAILED,
    SHARD_STATUS_ZOMBIE
} shard_status_t;

typedef void (*shard_init_fn)(void);

typedef struct {
    char name[SHARD_NAME_MAX];
    shard_category_t category;
    shard_status_t status;
    shard_init_fn init;
    sigma_u32 version;
    sigma_u64 load_timestamp;
} sovereign_shard_t;

typedef struct {
    sovereign_shard_t shards[MAX_SHARDS];
    sigma_u32 shard_count;
    sigma_u32 active_count;
    sigma_u64 registry_lock;
} sovereign_registry_t;

/* Public API */
void SovereignRegistry_Init(void);
sigma_err_t SovereignRegistry_Register(const char* name, shard_category_t cat, shard_init_fn init);
void SovereignRegistry_Finalize(void);
void SovereignRegistry_Audit(void);

#endif /* SOVEREIGN_REGISTRY_H */
