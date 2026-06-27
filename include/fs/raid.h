/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * raid.h — SigmaOS software RAID subsystem
 *
 * Supported levels: RAID 0, 1, 5, 6, 10
 * Runs as a BlockDevice shim: sits between VFS and the underlying block devices.
 * Inspired by: Linux md driver, FreeBSD geom_raid
 */

#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

typedef enum sigma_raid_level {
    RAID_0  = 0,   /* striping — max performance, no redundancy */
    RAID_1  = 1,   /* mirroring — full redundancy, n/2 capacity */
    RAID_5  = 5,   /* striping + parity — n-1 capacity, 1 disk fault */
    RAID_6  = 6,   /* striping + double parity — n-2 capacity, 2 disk faults */
    RAID_10 = 10,  /* striped mirrors — n/2 capacity, balanced */
} sigma_raid_level_t;

typedef enum sigma_disk_state {
    DISK_ONLINE    = 0,
    DISK_REBUILDING = 1,
    DISK_FAILED    = 2,
    DISK_SPARE     = 3,
    DISK_REMOVED   = 4,
} sigma_disk_state_t;

#define SIGMA_RAID_MAX_DISKS 8

typedef struct sigma_raid_disk {
    uint32_t         shard_id;        /* block device shard */
    char             devname[32];
    sigma_disk_state_t state;
    uint64_t         offset;          /* start sector on this device */
    uint64_t         size_blocks;     /* usable blocks */
    uint64_t         rebuild_progress; /* blocks rebuilt so far */
} sigma_raid_disk_t;

typedef struct sigma_raid_array {
    uint32_t             id;
    char                 name[32];
    sigma_raid_level_t   level;
    sigma_raid_disk_t    disks[SIGMA_RAID_MAX_DISKS];
    uint32_t             disk_count;
    uint32_t             chunk_size_kb;   /* stripe chunk in KB (64, 128, 256, 512) */
    uint64_t             array_size;      /* usable blocks for the array */
    uint32_t             degraded;        /* number of failed disks */
    bool                 rebuilding;
    uint8_t              uuid[16];
} sigma_raid_array_t;

/* ── RAID API ─────────────────────────────────────────────────────────────── */

int  sigma_raid_create  (sigma_raid_level_t level, uint32_t *shards,
                          size_t count, uint32_t chunk_kb,
                          sigma_raid_array_t *out);
int  sigma_raid_assemble(const char *name, sigma_raid_array_t *out);
int  sigma_raid_stop    (uint32_t array_id);
int  sigma_raid_read    (uint32_t array_id, uint64_t lba,
                          uint32_t blocks, void *buf);
int  sigma_raid_write   (uint32_t array_id, uint64_t lba,
                          uint32_t blocks, const void *buf);
int  sigma_raid_status  (uint32_t array_id, sigma_raid_array_t *out);
int  sigma_raid_add_disk(uint32_t array_id, uint32_t disk_shard,
                          bool as_spare);
int  sigma_raid_remove_disk(uint32_t array_id, uint32_t disk_index);
int  sigma_raid_rebuild (uint32_t array_id);   /* start background rebuild */
int  sigma_raid_scrub   (uint32_t array_id,    /* verify parity/checksums */
                          uint64_t *errors);
int  sigma_raid_list    (sigma_raid_array_t *out, size_t max, size_t *count);
