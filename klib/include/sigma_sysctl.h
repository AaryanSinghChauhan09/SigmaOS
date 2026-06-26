// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_sysctl.h — runtime kernel tuning interface (FreeBSD/OpenBSD sysctl-inspired)
 *
 * Every kernel subsystem registers tunable parameters via sigma_sysctl_register().
 * Parameters are readable and optionally writable at runtime — no reboot needed.
 * They are exposed through the SemanticFS at /sigma/sys/<name>.
 *
 * CLI usage:
 *   sigma-sysctl kernel.sched.timeslice_ms          # read
 *   sigma-sysctl kernel.sched.timeslice_ms=5        # write
 *   sigma-sysctl -a                                 # list all registered nodes
 *
 * Registration at module init:
 *   static int g_timeslice = 10;
 *   sigma_sysctl_register("kernel.sched.timeslice_ms",
 *                         SYSCTL_TYPE_INT, &g_timeslice, false);
 */

#include <sigma_kernel_types.h>

/* ── Value types ──────────────────────────────────────────────────────────── */
typedef enum {
    SYSCTL_TYPE_INT,     /* int32                                        */
    SYSCTL_TYPE_UINT64,  /* uint64                                       */
    SYSCTL_TYPE_BOOL,    /* bool                                         */
    SYSCTL_TYPE_STRING,  /* NUL-terminated string (up to 128 bytes)      */
} sigma_sysctl_type_t;

/* ── Node (linked list — no heap required, nodes are statically allocated) ── */
typedef struct sigma_sysctl_node {
    const char*               name;      /* "kernel.sched.timeslice_ms"  */
    sigma_sysctl_type_t       type;
    void*                     value;     /* pointer into kernel variable  */
    bool                      readonly;
    struct sigma_sysctl_node* next;
} sigma_sysctl_node_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/*
 * sigma_sysctl_register — register a kernel variable as a sysctl node.
 * @node must be statically allocated (not on the stack).
 */
void sigma_sysctl_register(sigma_sysctl_node_t* node,
                           const char*          name,
                           sigma_sysctl_type_t  type,
                           void*                value_ptr,
                           bool                 readonly);

/*
 * Read a sysctl by name into out buffer.
 * Returns number of bytes written, or -ENOENT if not found.
 */
int sigma_sysctl_get(const char* name, void* out, sigma_size_t* out_len);

/*
 * Write a sysctl by name from in buffer.
 * Returns 0 on success, -ENOENT if not found, -EPERM if readonly.
 */
int sigma_sysctl_set(const char* name, const void* in, sigma_size_t in_len);

/*
 * Iterate all registered nodes (for sigma-sysctl -a).
 * Returns pointer to first node; walk via node->next.
 */
const sigma_sysctl_node_t* sigma_sysctl_head(void);

/* ── Convenience registration macro ──────────────────────────────────────── */
/*
 * Declares a static sigma_sysctl_node_t with a mangled name, then registers
 * it in a constructor-like fashion.  Example:
 *
 *   static int g_timeslice = 10;
 *   SIGMA_SYSCTL(kernel_sched_timeslice, "kernel.sched.timeslice_ms",
 *                SYSCTL_TYPE_INT, &g_timeslice, false);
 */
#define SIGMA_SYSCTL(_id, _name, _type, _ptr, _ro)              \
    static sigma_sysctl_node_t _sysctl_node_##_id;              \
    static void __attribute__((constructor))                     \
    _sysctl_register_##_id(void) {                              \
        sigma_sysctl_register(&_sysctl_node_##_id,              \
                              (_name), (_type), (_ptr), (_ro)); \
    }
