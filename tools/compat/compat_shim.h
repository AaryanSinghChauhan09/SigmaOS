#ifndef COMPAT_SHIM_H
#define COMPAT_SHIM_H

#include <stdint.h>
#include <stddef.h>
#include <sys/types.h>

#if defined(_MSC_VER) || defined(_WIN32)
#ifndef _SSIZE_T_DEFINED
#define _SSIZE_T_DEFINED
typedef intptr_t ssize_t;
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

/*
 * =========================================================================
 * Σ SIGMAOS: POSIX COMPATIBILITY SHIM
 * =========================================================================
 * Intercepts POSIX syscalls from legacy Linux ELF binaries and translates
 * them to SigmaOS native Sovereign syscalls.
 * =========================================================================
 */

// Basic POSIX types
typedef int shim_fd_t;
typedef int shim_pid_t;

/**
 * Intercepts POSIX open().
 * Translates path and invokes SovereignFS `sfs_open()`.
 */
shim_fd_t posix_open(const char* path, int flags);

/**
 * Intercepts POSIX read().
 * Translates to Sovereign native capability-gated read.
 */
ssize_t posix_read(shim_fd_t fd, void* buf, size_t count);

/**
 * Intercepts POSIX write().
 * Translates to Sovereign native capability-gated write.
 */
ssize_t posix_write(shim_fd_t fd, const void* buf, size_t count);

/**
 * Intercepts POSIX close().
 */
int posix_close(shim_fd_t fd);

/**
 * Intercepts POSIX fork().
 * Translates to Sovereign Shard spawner. Note: Will map to a capability-restricted
 * child shard rather than a full copy-on-write legacy fork if strict mode is active.
 */
shim_pid_t posix_fork(void);

/**
 * Intercepts POSIX mmap().
 */
void* posix_mmap(void* addr, size_t length, int prot, int flags, shim_fd_t fd, uint64_t offset);

/**
 * Intercepts POSIX lseek().
 */
off_t posix_lseek(shim_fd_t fd, off_t offset, int whence);

struct stat;

/**
 * Intercepts POSIX stat().
 */
int posix_stat(const char *pathname, struct stat *statbuf);

/**
 * Intercepts POSIX mkdir().
 */
int posix_mkdir(const char *pathname, int mode);

#ifdef __cplusplus
}
#endif

#endif // COMPAT_SHIM_H
