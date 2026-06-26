// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

typedef struct {
    char     service_name[64];
    uint32_t pid;
    uint64_t last_pong_ns;
    uint64_t deadline_ns;
    uint32_t stuck_count;
    bool     vital;
} sigma_heartbeat_entry_t;

void sigma_heartbeat_register(const char* name, uint32_t pid,
                               uint64_t deadline_ns, bool vital);
void sigma_heartbeat_pong(const char* name);
void sigma_heartbeat_monitor_loop(void);
