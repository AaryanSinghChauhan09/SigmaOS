/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZFS — COPY-ON-WRITE FILESYSTEM (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: OpenZFS / FreeBSD ZFS
 *   https://github.com/freebsd/freebsd-src/tree/main/sys/contrib/openzfs
 *
 * Features implemented:
 *   §1  Storage Pool (zpool) — RAID-Z1/Z2/Mirror stripe management
 *   §2  Dataset (zfs) — Filesystem, Volume, Snapshot, Clone
 *   §3  Copy-on-Write block pointer tree (blkptr simulation)
 *   §4  LZ4 / ZSTD compression hooks (in-kernel decompression)
 *   §5  Snapshot: atomic point-in-time freeze, rollback, destroy
 *   §6  ZIL (ZFS Intent Log) — synchronous write journaling
 *   §7  ARC (Adaptive Replacement Cache) — L1ARC hot/cold lists
 *   §8  Scrub & resilver — background data integrity verification
 *   §9  Send/Receive — incremental stream replication
 *   §10 CLI: zpool / zfs command parity
 *
 * Inspiration: Jeff Bonwick's ZFS (Sun/Oracle), OpenZFS community
 * =========================================================================
 */

#ifndef SOVEREIGN_ZFS_H
#define SOVEREIGN_ZFS_H

#include "sigma_types.h"

/* -------------------------------------------------------------------------
 * Constants
 * ---------------------------------------------------------------------- */
#define SIGMA_ZFS_POOL_MAX      8
#define SIGMA_ZFS_VDEV_MAX     16          /* virtual devices per pool     */
#define SIGMA_ZFS_DATASET_MAX  256
#define SIGMA_ZFS_SNAP_MAX     64
#define SIGMA_ZFS_NAME_MAX     128
#define SIGMA_ZFS_BLK_SIZE     (128u*1024) /* 128 KiB default recordsize   */

/* -------------------------------------------------------------------------
 * RAID-Z topology
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_VDEV_DISK   = 0,
    SIGMA_VDEV_MIRROR = 1,
    SIGMA_VDEV_RAIDZ1 = 2,     /* 1 parity disk */
    SIGMA_VDEV_RAIDZ2 = 3,     /* 2 parity disks */
    SIGMA_VDEV_STRIPE = 4,
} SigmaVdevType_t;

/* -------------------------------------------------------------------------
 * Compression algorithms
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_COMPRESS_OFF  = 0,
    SIGMA_COMPRESS_LZ4  = 1,
    SIGMA_COMPRESS_ZSTD = 2,
    SIGMA_COMPRESS_GZIP = 3,
} SigmaZFSCompress_t;

/* -------------------------------------------------------------------------
 * Virtual device (vdev)
 * ---------------------------------------------------------------------- */
typedef struct {
    char             path[64];      /* e.g. /dev/nvme0n1 */
    sigma_u64        size_bytes;
    SigmaVdevType_t  type;
    sigma_bool       healthy;
    sigma_u64        read_errors;
    sigma_u64        write_errors;
    sigma_u64        cksum_errors;
} SigmaVdev_t;

/* -------------------------------------------------------------------------
 * Storage Pool (zpool)
 * ---------------------------------------------------------------------- */
typedef struct {
    char          name[SIGMA_ZFS_NAME_MAX];
    SigmaVdev_t   vdevs[SIGMA_ZFS_VDEV_MAX];
    sigma_u32     vdev_count;
    sigma_u64     total_bytes;
    sigma_u64     used_bytes;
    sigma_u64     free_bytes;
    sigma_bool    active;
    sigma_u32     health;           /* 0=ONLINE 1=DEGRADED 2=FAULTED */
} SigmaZPool_t;

/* -------------------------------------------------------------------------
 * Dataset types
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_DS_FILESYSTEM = 0,
    SIGMA_DS_VOLUME     = 1,
    SIGMA_DS_SNAPSHOT   = 2,
    SIGMA_DS_CLONE      = 3,
} SigmaDSType_t;

/* -------------------------------------------------------------------------
 * Dataset (zfs create / zfs snapshot)
 * ---------------------------------------------------------------------- */
typedef struct SigmaDataset {
    char               name[SIGMA_ZFS_NAME_MAX];   /* pool/dataset[@snap] */
    SigmaDSType_t      type;
    sigma_u64          used_bytes;
    sigma_u64          avail_bytes;
    sigma_u64          refer_bytes;
    SigmaZFSCompress_t compress;
    sigma_bool         readonly;
    sigma_bool         mounted;
    char               mountpoint[128];
    char               origin[SIGMA_ZFS_NAME_MAX]; /* for clone/snapshot   */
    struct SigmaDataset *parent;
    sigma_bool         active;
} SigmaDataset_t;

/* -------------------------------------------------------------------------
 * ARC (Adaptive Replacement Cache)
 * ---------------------------------------------------------------------- */
typedef struct {
    sigma_u64  max_bytes;
    sigma_u64  used_bytes;
    sigma_u64  hits;
    sigma_u64  misses;
    sigma_u64  l2arc_hits;
} SigmaARC_t;

/* -------------------------------------------------------------------------
 * Public API  (mirrors zpool / zfs CLI)
 * ---------------------------------------------------------------------- */
/* Pool */
sigma_err_t   sigma_zpool_create   (const char *name, SigmaVdevType_t type,
                                     const char *devs[], sigma_u32 ndev);
sigma_err_t   sigma_zpool_destroy  (const char *name);
sigma_err_t   sigma_zpool_status   (const char *name);
sigma_err_t   sigma_zpool_scrub    (const char *name);
void          sigma_zpool_list     (void);
sigma_err_t   sigma_zpool_import   (const char *name);
sigma_err_t   sigma_zpool_export   (const char *name);

/* Dataset */
sigma_err_t   sigma_zfs_create     (const char *dataset_path, SigmaDSType_t type);
sigma_err_t   sigma_zfs_destroy    (const char *dataset_path, sigma_bool recursive);
sigma_err_t   sigma_zfs_snapshot   (const char *dataset, const char *snap_name);
sigma_err_t   sigma_zfs_rollback   (const char *snapshot);
sigma_err_t   sigma_zfs_clone      (const char *snapshot, const char *dest);
sigma_err_t   sigma_zfs_mount      (const char *dataset, const char *mountpoint);
sigma_err_t   sigma_zfs_set        (const char *dataset, const char *prop,
                                     const char *val);
sigma_err_t   sigma_zfs_get        (const char *dataset, const char *prop);
void          sigma_zfs_list       (const char *pool);

/* Send/Receive */
sigma_err_t   sigma_zfs_send       (const char *snapshot, int out_fd);
sigma_err_t   sigma_zfs_recv       (const char *pool, int in_fd);

/* ARC stats */
void          sigma_arc_stats      (void);

void SovereignZFS_Init(void);

#endif /* SOVEREIGN_ZFS_H */
