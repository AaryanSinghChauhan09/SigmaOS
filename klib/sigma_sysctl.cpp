// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_sysctl.cpp — runtime kernel tuning (FreeBSD/OpenBSD sysctl-inspired)
 *
 * All registered nodes form a singly-linked list anchored at g_sysctl_head.
 * Nodes must be statically allocated; no dynamic allocation is used.
 */

#include "include/sigma_sysctl.h"
#include "sigma_log.h"

extern "C" int   sigma_strcmp(const char* a, const char* b);
extern "C" void  sigma_strncpy(char* dst, const char* src, sigma_size_t n);
extern "C" sigma_size_t sigma_strlen(const char* s);

static sigma_sysctl_node_t* g_sysctl_head = nullptr;

/* ── Registration ─────────────────────────────────────────────────────────── */

void sigma_sysctl_register(sigma_sysctl_node_t* node,
                           const char*          name,
                           sigma_sysctl_type_t  type,
                           void*                value_ptr,
                           bool                 readonly) {
    node->name     = name;
    node->type     = type;
    node->value    = value_ptr;
    node->readonly = readonly;
    node->next     = g_sysctl_head;
    g_sysctl_head  = node;
    sigma_log_info("[sysctl] registered '%s' type=%d ro=%d\n",
                   name, (int)type, (int)readonly);
}

/* ── Lookup ───────────────────────────────────────────────────────────────── */

static sigma_sysctl_node_t* find(const char* name) {
    for (sigma_sysctl_node_t* n = g_sysctl_head; n; n = n->next) {
        if (sigma_strcmp(n->name, name) == 0) return n;
    }
    return nullptr;
}

/* ── Get ──────────────────────────────────────────────────────────────────── */

int sigma_sysctl_get(const char* name, void* out, sigma_size_t* out_len) {
    sigma_sysctl_node_t* n = find(name);
    if (!n) return -1; /* -ENOENT */

    switch (n->type) {
    case SYSCTL_TYPE_INT: {
        if (*out_len < sizeof(int)) return -1;
        *((int*)out) = *((int*)n->value);
        *out_len = sizeof(int);
        break;
    }
    case SYSCTL_TYPE_UINT64: {
        if (*out_len < sizeof(sigma_u64)) return -1;
        *((sigma_u64*)out) = *((sigma_u64*)n->value);
        *out_len = sizeof(sigma_u64);
        break;
    }
    case SYSCTL_TYPE_BOOL: {
        if (*out_len < sizeof(bool)) return -1;
        *((bool*)out) = *((bool*)n->value);
        *out_len = sizeof(bool);
        break;
    }
    case SYSCTL_TYPE_STRING: {
        const char* s = (const char*)n->value;
        sigma_size_t slen = sigma_strlen(s) + 1;
        if (*out_len < slen) return -1;
        sigma_strncpy((char*)out, s, slen);
        *out_len = slen;
        break;
    }
    }
    return (int)*out_len;
}

/* ── Set ──────────────────────────────────────────────────────────────────── */

int sigma_sysctl_set(const char* name, const void* in, sigma_size_t in_len) {
    sigma_sysctl_node_t* n = find(name);
    if (!n) return -1; /* -ENOENT */
    if (n->readonly) {
        sigma_log_warn("[sysctl] '%s' is read-only\n", name);
        return -2; /* -EPERM */
    }

    switch (n->type) {
    case SYSCTL_TYPE_INT:
        if (in_len < sizeof(int)) return -1;
        *((int*)n->value) = *((const int*)in);
        break;
    case SYSCTL_TYPE_UINT64:
        if (in_len < sizeof(sigma_u64)) return -1;
        *((sigma_u64*)n->value) = *((const sigma_u64*)in);
        break;
    case SYSCTL_TYPE_BOOL:
        if (in_len < sizeof(bool)) return -1;
        *((bool*)n->value) = *((const bool*)in);
        break;
    case SYSCTL_TYPE_STRING:
        sigma_strncpy((char*)n->value, (const char*)in,
                      in_len < 128 ? in_len : 127);
        ((char*)n->value)[127] = '\0';
        break;
    }

    sigma_log_info("[sysctl] '%s' updated\n", name);
    return 0;
}

/* ── Iteration ────────────────────────────────────────────────────────────── */

const sigma_sysctl_node_t* sigma_sysctl_head(void) {
    return g_sysctl_head;
}
