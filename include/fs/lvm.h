/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * lvm.h — SigmaOS Logical Volume Manager
 *
 * Provides flexible volume management on top of physical block devices.
 * Supports: create, resize, snapshot, delete without rebooting.
 *
 * Hierarchy:
 *   Physical Volume (PV) — one block device or partition
 *       ↓
 *   Volume Group (VG) — pool of PVs
 *       ↓
 *   Logical Volume (LV) — virtual block device carved from a VG
 *
 * Each LV appears as /dev/sigma/<vg>/<lv> and can be formatted with any FS.
 *
 * Inspired by: Linux LVM2 (device-mapper), macOS Core Storage, ZFS zvols
 */

#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#define LVM_MAX_PVS  32
#define LVM_MAX_LVS  256
#define LVM_MAX_VGS  16
#define LVM_PE_SIZE  (4ULL * 1024 * 1024)  /* 4MB physical extent size */
#define LVM_NAME_MAX 64

/* ── Physical Volume ─────────────────────────────────────────────────────── */

typedef struct sigma_pv {
    char     device[128];       /* e.g. "/dev/sda2" */
    uint8_t  uuid[16];
    uint64_t size_bytes;
    uint64_t pe_count;          /* number of physical extents */
    uint64_t pe_free;
    uint32_t vg_id;             /* which VG this PV belongs to (0=none) */
    char     vg_name[LVM_NAME_MAX];
} sigma_pv_t;

/* ── Volume Group ────────────────────────────────────────────────────────── */

typedef struct sigma_vg {
    uint32_t id;
    char     name[LVM_NAME_MAX];
    uint8_t  uuid[16];
    uint64_t pe_size;
    uint64_t pe_count;          /* total extents in VG */
    uint64_t pe_free;
    uint32_t pv_count;
    uint32_t lv_count;
    uint32_t pv_ids[LVM_MAX_PVS];
} sigma_vg_t;

/* ── Logical Volume ──────────────────────────────────────────────────────── */

typedef enum sigma_lv_type {
    LV_LINEAR   = 0,  /* simple linear volume */
    LV_STRIPED  = 1,  /* striped across PVs (like RAID 0) */
    LV_MIRROR   = 2,  /* mirrored (like RAID 1) */
    LV_SNAPSHOT = 3,  /* COW snapshot of another LV */
    LV_THIN     = 4,  /* thin-provisioned (sparse) */
} sigma_lv_type_t;

typedef struct sigma_lv {
    uint32_t          id;
    char              name[LVM_NAME_MAX];
    uint8_t           uuid[16];
    uint32_t          vg_id;
    sigma_lv_type_t   type;
    uint64_t          size_bytes;   /* current size */
    uint64_t          pe_count;
    bool              active;       /* mapped to block device? */
    bool              open;         /* currently opened by a filesystem? */
    /* For snapshots */
    uint32_t          origin_lv_id; /* LV we're snapshotting (0=none) */
    uint64_t          cow_used_bytes;
    /* Device path when active */
    char              dev_path[128]; /* /dev/sigma/<vg>/<lv> */
} sigma_lv_t;

/* ── LVM API ─────────────────────────────────────────────────────────────── */

/* Physical Volume operations */
int  sigma_pv_create (const char *device, sigma_pv_t *out);
int  sigma_pv_remove (const char *device);
int  sigma_pv_scan   (sigma_pv_t *out, size_t max, size_t *count);
int  sigma_pv_display(const char *device, sigma_pv_t *out);
int  sigma_pv_resize (const char *device);  /* after disk was resized */

/* Volume Group operations */
int  sigma_vg_create  (const char *name, const char **pvs, size_t pv_count,
                        uint64_t pe_size, sigma_vg_t *out);
int  sigma_vg_remove  (const char *name);
int  sigma_vg_extend  (const char *name, const char *new_pv);
int  sigma_vg_reduce  (const char *name, const char *pv);
int  sigma_vg_rename  (const char *old_name, const char *new_name);
int  sigma_vg_display (const char *name, sigma_vg_t *out);
int  sigma_vg_list    (sigma_vg_t *out, size_t max, size_t *count);
int  sigma_vg_activate(const char *name);
int  sigma_vg_deactivate(const char *name);

/* Logical Volume operations */
int  sigma_lv_create   (const char *vg_name, const char *lv_name,
                         uint64_t size_bytes, sigma_lv_type_t type,
                         sigma_lv_t *out);
int  sigma_lv_remove   (const char *vg_name, const char *lv_name);
int  sigma_lv_resize   (const char *vg_name, const char *lv_name,
                         int64_t delta_bytes, bool resize_fs);
int  sigma_lv_rename   (const char *vg_name, const char *old_name,
                         const char *new_name);
int  sigma_lv_display  (const char *vg_name, const char *lv_name,
                         sigma_lv_t *out);
int  sigma_lv_list     (const char *vg_name, sigma_lv_t *out, size_t max,
                         size_t *count);
int  sigma_lv_activate  (const char *vg_name, const char *lv_name);
int  sigma_lv_deactivate(const char *vg_name, const char *lv_name);

/* Snapshot operations */
int  sigma_lv_snapshot  (const char *vg_name, const char *origin_lv,
                          const char *snap_name, uint64_t cow_size,
                          sigma_lv_t *out);
int  sigma_lv_snap_merge(const char *vg_name, const char *snap_name);
int  sigma_lv_snap_info (const char *vg_name, const char *snap_name,
                          sigma_lv_t *out);

/* Thin provisioning */
int  sigma_lv_thin_pool (const char *vg_name, const char *pool_name,
                          uint64_t size_bytes, uint32_t chunk_kb);
int  sigma_lv_thin_create(const char *vg_name, const char *pool_name,
                           const char *lv_name, uint64_t virtual_size,
                           sigma_lv_t *out);
