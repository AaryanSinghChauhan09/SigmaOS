// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_scheme.h — unified resource URL dispatcher (Redox OS-inspired)
 * sigma_open("net:tcp/8.8.8.8/443") dispatches to the "net" scheme handler.
 * sigma_open("file:/sigma/etc/passwd") dispatches to the "file" scheme handler.
 * sigma_open("sigma-key:tpm2/pcr7") dispatches to the "sigma-key" scheme handler.
 */
#include <sigma_kernel_types.h>
#include <fcntl.h>

typedef struct {
    const char* scheme;
    int     (*open) (const char* path, int flags, void* ctx);
    ssize_t (*read) (int fd, void* buf, size_t len, void* ctx);
    ssize_t (*write)(int fd, const void* buf, size_t len, void* ctx);
    int     (*close)(int fd, void* ctx);
    void*   ctx;
} sigma_scheme_t;

void sigma_scheme_register(const sigma_scheme_t* scheme);
int  sigma_open(const char* url, int flags);
ssize_t sigma_read(int fd, void* buf, size_t len);
ssize_t sigma_write(int fd, const void* buf, size_t len);
int  sigma_close(int fd);
void sigma_scheme_init(void);
