// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/* sigma_pkg_transaction.h — transactional package ops (Flatpak-inspired) */
#include <stdbool.h>

typedef enum {
    SIGMA_PKG_OP_INSTALL, SIGMA_PKG_OP_UPDATE,
    SIGMA_PKG_OP_UNINSTALL, SIGMA_PKG_OP_REINSTALL,
} sigma_pkg_op_type_t;

typedef struct {
    sigma_pkg_op_type_t type;
    char package_name[64];
    char version[32];    /* empty = latest */
    bool resolved;
} sigma_pkg_op_t;

typedef struct {
    sigma_pkg_op_t ops[64];
    int  op_count;
    bool no_download;
    bool no_deploy;
    bool disable_deps;
    bool allow_downgrade;
    void (*on_op_start)(const sigma_pkg_op_t*, void*);
    void (*on_op_done) (const sigma_pkg_op_t*, int, void*);
    void* userdata;
} sigma_pkg_transaction_t;

int sigma_pkg_transaction_add(sigma_pkg_transaction_t* txn,
                               sigma_pkg_op_type_t type,
                               const char* name, const char* version);
int sigma_pkg_transaction_resolve(sigma_pkg_transaction_t* txn);
int sigma_pkg_transaction_run(sigma_pkg_transaction_t* txn);
