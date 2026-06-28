// SPDX-License-Identifier: GPL-2.0-only
// sigma_landlock.h — SigmaOS Landlock + seccomp-bpf per-app profile generation
// Purpose: Auto-generate per-app Landlock filesystem restriction + seccomp-bpf
//          syscall filter from the app's declared capabilities (manifest.json).
//          Defense-in-depth beyond sigma-jail namespace isolation.
//          Inspired by Linux Landlock LSM (5.13+) and Android seccomp policy.

#pragma once
#include <stdint.h>
#include <stdbool.h>

// ---------------------------------------------------------------------------
// Landlock ruleset (filesystem access restriction)
// ---------------------------------------------------------------------------

typedef struct {
    // Allowed filesystem paths (deny-all by default, allow-list)
    struct {
        char   path[256];
        bool   read;
        bool   write;
        bool   execute;
        bool   recursive;
    } fs_rules[64];
    int    fs_rule_count;
    // Allowed network (future Landlock v5+)
    bool   can_bind_tcp;
    bool   can_connect_tcp;
    uint16_t allowed_tcp_ports[16];
    int    tcp_port_count;
} sigma_landlock_ruleset_t;

// ---------------------------------------------------------------------------
// seccomp-bpf profile
// ---------------------------------------------------------------------------

typedef struct {
    char   app_id[64];             // App identifier
    // Syscall whitelist (SECCOMP_RET_ALLOW for listed, SECCOMP_RET_KILL for others)
    int    allowed_syscalls[256];  // syscall numbers
    int    allowed_count;
    // Syscall argument filters (per-syscall fine-grained)
    struct {
        int    syscall_nr;
        uint64_t arg_mask;         // Only allow calls where (arg & mask) == value
        uint64_t arg_value;
        int    arg_index;          // 0-5
    } arg_filters[32];
    int    arg_filter_count;
    bool   allow_new_privs;        // NO_NEW_PRIVS prctl
    bool   ptrace_allowed;         // Deny ptrace by default
} sigma_seccomp_profile_t;

// ---------------------------------------------------------------------------
// Profile generation from app manifest capabilities
// ---------------------------------------------------------------------------

// Capability → minimal syscall set mapping:
// cap_network    → socket, bind, connect, accept, recv*, send*, poll
// cap_filesystem → open, read, write, stat, fstat, close, rename, unlink
// cap_exec       → execve, fork, clone, wait4
// cap_ipc        → sigma-bus socket only (abstract namespace)
// cap_display    → sigma-display protocol socket only
// cap_camera     → /dev/video* read-only
// cap_microphone → /dev/snd/* read-only

int sigma_landlock_profile_generate(const char *app_manifest_json,
                                     sigma_landlock_ruleset_t *ruleset_out);
int sigma_seccomp_profile_generate(const char *app_manifest_json,
                                    sigma_seccomp_profile_t *profile_out);
int sigma_landlock_apply(const sigma_landlock_ruleset_t *ruleset);
int sigma_seccomp_apply(const sigma_seccomp_profile_t *profile);

// Audit: log every landlock/seccomp denial via sigma-bus
// Format: { "app": "...", "denied": "open", "path": "/etc/passwd", "pid": 1234 }
int sigma_security_denial_log(const char *app_id, const char *syscall_name,
                                const char *resource, int pid);
