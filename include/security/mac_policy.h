/* SPDX-License-Identifier: GPL-2.0-or-later */
/**
 * mac_policy.h — SigmaOS MAC policy language + capability system
 *
 * The policy language (.sigma-policy files) is parsed by sigma-macd and
 * compiled to a compact decision table loaded into kernel AVC cache.
 *
 * Example policy:
 *
 *   process zenith_browser {
 *     allow read:   /home/$USER/Downloads/**
 *     allow write:  /home/$USER/Downloads/**
 *     deny  read:   /home/$USER/.ssh/**
 *     deny  net:    10.0.0.0/8
 *   }
 *
 *   driver * {
 *     require pqc_signature: sigma-official-key
 *     deny write: /kernel/**
 *   }
 *
 * Inspired by: SELinux policy, AppArmor profiles, Capsicum (FreeBSD)
 */

#pragma once
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

/* ── Rights bitmask ─────────────────────────────────────────────────────── */

typedef uint32_t sigma_right_t;
#define RIGHT_READ      (1u << 0)
#define RIGHT_WRITE     (1u << 1)
#define RIGHT_EXEC      (1u << 2)
#define RIGHT_CREATE    (1u << 3)
#define RIGHT_DELETE    (1u << 4)
#define RIGHT_IPC_SEND  (1u << 5)
#define RIGHT_IPC_RECV  (1u << 6)
#define RIGHT_NET_BIND  (1u << 7)
#define RIGHT_NET_CONN  (1u << 8)
#define RIGHT_MMIO_MAP  (1u << 9)
#define RIGHT_IRQ_HDLR  (1u << 10)
#define RIGHT_PTRACE    (1u << 11)
#define RIGHT_SETUID    (1u << 12)
#define RIGHT_LOAD_MOD  (1u << 13)

/* ── MAC label (Bell-LaPadula + Biba combined) ───────────────────────────── */

typedef struct sigma_mac_label {
    uint8_t  sensitivity;    /* 0=public → 7=top-secret (Bell-LaPadula) */
    uint8_t  integrity;      /* 0=untrusted → 7=system (Biba) */
    uint32_t compartments;   /* bitmask: finance, medical, legal, ... */
    char     context[64];    /* human-readable: "system_u:kernel_t" */
} sigma_mac_label_t;

/* ── Capability token ────────────────────────────────────────────────────── */

typedef struct sigma_capability {
    uint64_t        resource_id;  /* file inode, device ID, IPC channel, ... */
    sigma_right_t   rights;
    uint64_t        expiry_ns;    /* 0 = never expires */
    uint8_t         signature[64];/* Dilithium3 signature of (resource_id+rights+expiry) */
    uint32_t        issuer_shard; /* capability authority that issued this */
} sigma_capability_t;

/* ── Policy rule ─────────────────────────────────────────────────────────── */

typedef enum sigma_policy_action {
    POLICY_ALLOW   = 0,
    POLICY_DENY    = 1,
    POLICY_AUDIT   = 2,   /* allow but log */
    POLICY_KILL    = 3,   /* deny + SIGKILL the process */
} sigma_policy_action_t;

typedef struct sigma_policy_rule {
    char                  subject[64];   /* process name or "*" */
    char                  object[256];   /* path glob, net CIDR, device name */
    sigma_right_t         rights;
    sigma_policy_action_t action;
    bool                  require_pqc_sig;
    char                  pqc_key_id[32];
} sigma_policy_rule_t;

/* ── Policy compilation (text → decision table) ──────────────────────────── */

typedef struct sigma_policy_db {
    sigma_policy_rule_t *rules;
    size_t               rule_count;
    size_t               rule_cap;
    /* AVC (Access Vector Cache) — O(1) lookup for hot paths */
    struct sigma_avc_entry {
        uint32_t hash;          /* FNV(subject + object + rights) */
        sigma_policy_action_t action;
    } *avc;
    size_t avc_size;            /* must be power of 2 */
} sigma_policy_db_t;

/* ── Policy API ─────────────────────────────────────────────────────────── */

/* Load and compile a .sigma-policy text file */
int  sigma_policy_load   (const char *path, sigma_policy_db_t **out);
void sigma_policy_free   (sigma_policy_db_t *db);

/* Check a proposed operation */
sigma_policy_action_t sigma_policy_check(
    const sigma_policy_db_t *db,
    const char *subject,      /* process name */
    const char *object,       /* path / address / device */
    sigma_right_t rights);

/* AVC fast path (called from every syscall handler) */
sigma_policy_action_t sigma_avc_check(
    uint32_t subject_shard,
    uint64_t object_id,
    sigma_right_t rights);

/* ── Capability space API ────────────────────────────────────────────────── */

int  sigma_cap_grant  (uint32_t to_shard, const sigma_capability_t *cap);
int  sigma_cap_revoke (uint32_t from_shard, uint64_t resource_id);
bool sigma_cap_check  (uint32_t shard, uint64_t resource_id, sigma_right_t right);
int  sigma_cap_list   (uint32_t shard, sigma_capability_t *out, size_t max,
                        size_t *count);
/* Transfer capability to another shard (with optional attenuation) */
int  sigma_cap_delegate(uint32_t to_shard, const sigma_capability_t *cap,
                         sigma_right_t mask);  /* mask limits rights */

/* ── MAC label API ───────────────────────────────────────────────────────── */

int  sigma_mac_get_label (uint32_t shard, sigma_mac_label_t *out);
int  sigma_mac_set_label (uint32_t shard, const sigma_mac_label_t *label);
bool sigma_mac_dominates (const sigma_mac_label_t *a, const sigma_mac_label_t *b);
bool sigma_mac_can_read  (const sigma_mac_label_t *subj, const sigma_mac_label_t *obj);
bool sigma_mac_can_write (const sigma_mac_label_t *subj, const sigma_mac_label_t *obj);
