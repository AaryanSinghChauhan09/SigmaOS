// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
typedef struct {
    const char*   device;
    const char*   mountpoint;
    const char*   fstype;
    unsigned long flags;
    const char*   options;
} sigma_mount_entry_t;
int sigma_mount_all(void);
