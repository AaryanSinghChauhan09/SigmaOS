// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
#include <sys/types.h>
#include <stdbool.h>
#include <stdint.h>

typedef struct {
    char     name[64];
    char     exec_path[256];
    bool     essential;
    uint32_t max_restarts;
    uint32_t restart_count;
    pid_t    pid;
    bool     alive;
} sigma_rs_entry_t;

void sigma_rs_monitor_loop(void);
