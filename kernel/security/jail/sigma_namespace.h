// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_namespace.h — Linux namespace isolation config (Bubblewrap-inspired)
 */

#include <sys/types.h>
#include <stdbool.h>

typedef struct {
    bool  isolate_pid;       /* CLONE_NEWPID — private PID namespace          */
    bool  isolate_net;       /* CLONE_NEWNET — private network stack           */
    bool  isolate_mnt;       /* CLONE_NEWNS  — private mount namespace         */
    bool  isolate_ipc;       /* CLONE_NEWIPC — private IPC objects             */
    bool  isolate_uts;       /* CLONE_NEWUTS — private hostname                */
    bool  isolate_cgroup;    /* CLONE_NEWCGROUP — private cgroup view          */
    uid_t uid_map_inside;    /* inside-namespace UID (usually 0 = "root")      */
    uid_t uid_map_outside;   /* host UID that maps to uid_map_inside           */
    char  new_root[256];     /* path to pivot_root into (empty = no pivot)     */
} sigma_ns_config_t;

/*
 * Enter namespaces BEFORE exec'ing the jailed process.
 * Call this from the child side of fork(), then execve().
 */
int sigma_jail_enter(const sigma_ns_config_t* cfg);

/* C-compatible wrapper used by sigma_jail.cpp */
int sigma_jail_create(const char* jail_name);
