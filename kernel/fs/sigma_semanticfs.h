// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_semanticfs.h — SemanticFS with rich inode attributes (Haiku BFS-inspired)
 *
 * Every inode carries SIGMA: extended attributes directly in the inode structure.
 * Attribute index server (sigma-indexd) maintains B-trees for fast queries.
 * ZeroTrust can query "all SECRET files created by UNTRUSTED workloads" in O(log n).
 */
#include <sigma_kernel_types.h>
#include <sigma_trust_labels.h>
#include <stdbool.h>

/* Attribute names (stored inline in inode, like Haiku BEOS:TYPE) */
#define SIGMA_ATTR_MIME_TYPE      "SIGMA:MIME"      /* "application/x-sigma-pkg"        */
#define SIGMA_ATTR_TRUST_LABEL    "SIGMA:TRUST"     /* sigma_trust_label_t              */
#define SIGMA_ATTR_SIGNER_ID      "SIGMA:SIGNER"    /* ed25519 key fingerprint          */
#define SIGMA_ATTR_VERITY_HASH    "SIGMA:VERITY"    /* dm-verity root hash (32 bytes)   */
#define SIGMA_ATTR_CREATED_BY     "SIGMA:CREATOR"   /* workload SPIFFE URI              */
#define SIGMA_ATTR_CLASSIFICATION "SIGMA:CLASS"     /* 0=PUBLIC 1=CONFIDENTIAL 2=SECRET */

/* Classification levels */
#define SIGMA_CLASS_PUBLIC        0
#define SIGMA_CLASS_CONFIDENTIAL  1
#define SIGMA_CLASS_SECRET        2

/* SemanticFS inode with inline attribute storage */
typedef struct {
    sigma_u64  inode_id;
    sigma_u64  size;
    sigma_u64  created_ns;
    sigma_u64  modified_ns;
    sigma_u32  mode;          /* POSIX permission bits                        */

    /* Extended attributes — inline up to this size, overflow to attr block */
    struct {
        char    mime_type[64];
        sigma_u8 trust_label;    /* sigma_trust_label_t                      */
        char    signer_id[64];   /* key fingerprint                          */
        sigma_u8 verity_hash[32];
        char    created_by[128]; /* SPIFFE URI of creating workload          */
        sigma_u8 classification; /* SIGMA_CLASS_*                            */
    } attrs;

    sigma_u8 data[0];            /* file data follows (for small files)      */
} sigma_semanticfs_inode_t;

/* ── Attribute query ──────────────────────────────────────────────────────── */

typedef enum {
    SIGMA_ATTR_OP_EQ       = 0,
    SIGMA_ATTR_OP_NEQ      = 1,
    SIGMA_ATTR_OP_GT       = 2,
    SIGMA_ATTR_OP_LT       = 3,
    SIGMA_ATTR_OP_CONTAINS = 4,
} sigma_attr_op_t;

typedef struct {
    const char*      attr_name;
    sigma_attr_op_t  op;
    const char*      value;
} sigma_attr_query_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

int  sigma_semanticfs_init(const char* mount_point);

/* Read/write individual attributes */
int  sigma_semanticfs_set_attr(sigma_u64 inode_id, const char* attr, const char* value);
int  sigma_semanticfs_get_attr(sigma_u64 inode_id, const char* attr,
                                char* out, sigma_size_t out_len);

/*
 * Execute an attribute query — returns matching inode IDs.
 * Uses sigma-indexd for O(log n) performance.
 * Example: find all SECRET files created by UNTRUSTED workloads
 *   sigma_attr_query_t q[] = {
 *       { SIGMA_ATTR_CLASSIFICATION, SIGMA_ATTR_OP_EQ, "2" },
 *       { SIGMA_ATTR_TRUST_LABEL,    SIGMA_ATTR_OP_EQ, "5" },
 *   };
 */
int  sigma_semanticfs_query(const sigma_attr_query_t* queries, int nqueries,
                              sigma_u64* out_inodes, int max_results);

/* Notify attribute index server of an attribute change */
void sigma_semanticfs_notify_attr_change(sigma_u64 inode_id,
                                          const char* attr_name,
                                          const char* new_value);
