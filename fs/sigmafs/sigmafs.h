/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * sigmafs.h — SigmaFS: native copy-on-write filesystem for SigmaOS
 *
 * Design goals:
 *   • Copy-on-write (COW) — every write creates a new block; old blocks
 *     are preserved for snapshots and crash consistency.
 *   • Shard-aware — metadata transactions are serialised via IPC journal.
 *   • Inline xattrs — SIGMA:TRUST, SIGMA:CLASS, SIGMA:SIGNER stored in inode.
 *   • Atomic rename — guaranteed even on power loss.
 *   • Transparent compression — per-extent zstd (optional).
 *   • Max volume size: 256 PB | Max file size: 16 TB | Max filename: 255 bytes
 *
 * Inspired by: btrfs (COW, B-trees), ZFS (checksums), ext4 (htree directories)
 */

#pragma once
#include "../../include/fs/vfs.h"
#include <stdint.h>
#include <stddef.h>

/* ── Magic and version ───────────────────────────────────────────────────── */

#define SIGMAFS_MAGIC   0x5349474D41465301ULL  /* "SIGMAFS\x01" */
#define SIGMAFS_VERSION 1

/* ── On-disk superblock (first 4KB of volume) ────────────────────────────── */

typedef struct __attribute__((packed)) sigmafs_superblock {
    uint64_t    magic;
    uint32_t    version;
    uint32_t    flags;
#define SFS_FLAG_COW_ENABLED    (1u << 0)
#define SFS_FLAG_COMPRESS       (1u << 1)
#define SFS_FLAG_ENCRYPT        (1u << 2)

    uint64_t    block_size;      /* must be 4096 */
    uint64_t    total_blocks;
    uint64_t    free_blocks;
    uint64_t    inode_count;
    uint64_t    free_inodes;
    uint64_t    root_inode;
    uint64_t    journal_block;   /* block number of journal superblock */
    uint64_t    journal_size;    /* journal size in blocks */
    uint64_t    snap_tree_root;  /* B-tree of snapshots */
    uint64_t    generation;      /* monotonically increasing */
    uint8_t     uuid[16];
    char        label[32];
    uint64_t    mount_time;
    uint64_t    write_time;
    uint32_t    mount_count;
    uint8_t     checksum[32];    /* SHA-256 of rest of superblock */
} sigmafs_superblock_t;

/* ── On-disk inode (256 bytes, 16 per 4KB block) ─────────────────────────── */

typedef struct __attribute__((packed)) sigmafs_inode {
    uint32_t    mode;
    uint32_t    uid;
    uint32_t    gid;
    uint32_t    nlink;
    uint64_t    size;
    uint64_t    blocks;
    uint64_t    atime_ns;
    uint64_t    mtime_ns;
    uint64_t    ctime_ns;
    uint64_t    crtime_ns;       /* creation time */
    /* Extent tree root (replaces legacy block pointers) */
    uint64_t    extent_tree_root;
    /* COW snapshot chain */
    uint64_t    cow_gen;         /* generation when this inode was written */
    uint64_t    cow_prev_inode;  /* previous version of this inode (0=none) */
    /* Inline xattrs (Haiku-inspired SemanticFS) */
    uint8_t     xattr_trust[8];  /* SIGMA:TRUST label */
    uint8_t     xattr_class[8];  /* SIGMA:CLASS label */
    uint8_t     xattr_signer[32];/* Dilithium3 public key hash */
    uint8_t     checksum[32];    /* SHA-256 of inode data */
    uint8_t     _pad[24];
} sigmafs_inode_t;

_Static_assert(sizeof(sigmafs_inode_t) == 256, "sigmafs_inode size mismatch");

/* ── Extent (describes a contiguous run of blocks) ───────────────────────── */

typedef struct __attribute__((packed)) sigmafs_extent {
    uint64_t    logical_block;   /* file offset in blocks */
    uint64_t    physical_block;  /* volume offset in blocks */
    uint32_t    block_count;     /* run length */
    uint16_t    flags;
#define SFS_EXT_COMPRESSED (1u << 0)
#define SFS_EXT_ENCRYPTED  (1u << 1)
#define SFS_EXT_SHARED     (1u << 2)  /* COW shared extent */
    uint32_t    compressed_size; /* if SFS_EXT_COMPRESSED, stored size */
    uint32_t    checksum;        /* CRC32C of block data */
} sigmafs_extent_t;

/* ── Directory entry ─────────────────────────────────────────────────────── */

typedef struct __attribute__((packed)) sigmafs_dirent {
    uint64_t    inode;
    uint16_t    reclen;          /* length of this record (for alignment) */
    uint8_t     name_len;
    uint8_t     file_type;       /* VNODE_TYPE_* */
    char        name[];          /* name_len bytes, not NUL-terminated */
} sigmafs_dirent_t;

/* ── Journal record ──────────────────────────────────────────────────────── */

typedef struct __attribute__((packed)) sigmafs_journal_rec {
    uint64_t    seq;             /* monotonic sequence number */
    uint32_t    type;
#define SFS_JRN_INODE_UPDATE 1
#define SFS_JRN_BLOCK_ALLOC  2
#define SFS_JRN_BLOCK_FREE   3
#define SFS_JRN_DIRENT_ADD   4
#define SFS_JRN_DIRENT_DEL   5
#define SFS_JRN_COMMIT       6
    uint32_t    data_len;
    uint8_t     checksum[32];
    uint8_t     data[];
} sigmafs_journal_rec_t;

/* ── Snapshot descriptor ─────────────────────────────────────────────────── */

typedef struct sigmafs_snapshot {
    uint64_t    id;
    uint64_t    creation_time;
    uint64_t    root_inode;
    char        name[64];
    uint64_t    parent_snap_id;
    uint64_t    generation;
} sigmafs_snapshot_t;

/* ── SigmaFS shard operations (C API) ───────────────────────────────────────── */

int  sigmafs_mkfs    (int fd, uint64_t block_count, const char *label);
int  sigmafs_mount   (int fd, const char *mountpoint, uint32_t flags);
int  sigmafs_umount  (const char *mountpoint);

/* Snapshot management */
int  sigmafs_snap_create(const char *mountpoint, const char *name,
                          sigmafs_snapshot_t *out);
int  sigmafs_snap_delete(const char *mountpoint, uint64_t snap_id);
int  sigmafs_snap_list  (const char *mountpoint, sigmafs_snapshot_t *out,
                          size_t max, size_t *count);
int  sigmafs_snap_restore(const char *mountpoint, uint64_t snap_id);

/* Scrub: verify checksums of all data blocks */
int  sigmafs_scrub   (const char *mountpoint, uint64_t *errors_found);
