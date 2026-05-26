/*
 * =========================================================================
 * Σ SIGMAOS: POSIX COMPATIBILITY SHIM IMPLEMENTATION
 * =========================================================================
 */

#include "compat_shim.h"
#include <sys/stat.h>

// Mock fd table
#define MAX_FDS 256

struct mock_fd_state {
    int used;
    off_t offset;
};

static struct mock_fd_state g_fd_table[MAX_FDS];

shim_fd_t posix_open(const char* path, int flags) {
    if (!path) return -1;
    
    // Stub: Route through Sovereign VFS, map LBAC capabilities, check tokens
    // Then allocate a shim fd.
    for (int i = 0; i < MAX_FDS; i++) {
        if (g_fd_table[i].used == 0) {
            g_fd_table[i].used = 1; // Mark used
            g_fd_table[i].offset = 0;
            return i;
        }
    }
    
    return -1; // ENFILE
}

ssize_t posix_read(shim_fd_t fd, void* buf, size_t count) {
    if (fd < 0 || fd >= MAX_FDS || g_fd_table[fd].used == 0) return -1; // EBADF
    
    // Stub: Route to Sovereign read
    // Advance mock offset
    g_fd_table[fd].offset += count;
    return count; // Mock read full count
}

ssize_t posix_write(shim_fd_t fd, const void* buf, size_t count) {
    if (fd < 0 || fd >= MAX_FDS || g_fd_table[fd].used == 0) return -1; // EBADF
    
    // Stub: Route to Sovereign write
    g_fd_table[fd].offset += count;
    return count;
}

int posix_close(shim_fd_t fd) {
    if (fd < 0 || fd >= MAX_FDS || g_fd_table[fd].used == 0) return -1; // EBADF
    
    g_fd_table[fd].used = 0; // Free
    g_fd_table[fd].offset = 0;
    return 0;
}

shim_pid_t posix_fork(void) {
    // Stub: In SigmaOS, fork() translates into sigma_spawn_shard()
    // It creates a new capability-gated sandboxed execution context.
    return 9999; // Dummy shard ID
}

void* posix_mmap(void* addr, size_t length, int prot, int flags, shim_fd_t fd, uint64_t offset) {
    // Stub: Call S-HAL mapPageTableMemory or Sovereign MMU manager
    return (void*) -1; // MAP_FAILED
}

off_t posix_lseek(shim_fd_t fd, off_t offset, int whence) {
    if (fd < 0 || fd >= MAX_FDS || g_fd_table[fd].used == 0) return (off_t)-1; // EBADF
    
    // Very basic mock implementation
    if (whence == 0) { // SEEK_SET
        g_fd_table[fd].offset = offset;
    } else if (whence == 1) { // SEEK_CUR
        g_fd_table[fd].offset += offset;
    }
    return g_fd_table[fd].offset;
}

int posix_stat(const char *pathname, struct stat *statbuf) {
    if (!pathname || !statbuf) return -1;
    // Mock successful stat
    statbuf->st_size = 1024;
    statbuf->st_mode = 0644; // Regular file
    return 0;
}

int posix_mkdir(const char *pathname, mode_t mode) {
    if (!pathname) return -1;
    // Mock successful directory creation
    return 0;
}
