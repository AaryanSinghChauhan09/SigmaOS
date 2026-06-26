// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
#include <stdbool.h>

typedef struct {
    bool has_storage;
    int  file_count;
    int  user_count;
    int  kernel_arg_count;
} sigma_ignite_config_t;

int sigma_ignite_main(void);
int sigma_ignite_load_config(sigma_ignite_config_t* out);
int sigma_ignite_setup_filesystems(const sigma_ignite_config_t* cfg);
int sigma_ignite_write_files(const sigma_ignite_config_t* cfg);
int sigma_ignite_setup_users(const sigma_ignite_config_t* cfg);
int sigma_ignite_apply_kernel_args(const sigma_ignite_config_t* cfg);
