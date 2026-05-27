#include "sigma_wasi.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS WASI Implementation
// Translates POSIX-like WASI calls into Sovereign kernel
// primitives. Currently contains basic stubs for stdout/stderr.
// ---------------------------------------------------------

namespace sigma {
namespace wasi {

extern "C" {

wasi_errno_t wasi_snapshot_preview1_fd_read(wasi_fd_t fd, const wasi_iovec_t* iovs, size_t iovs_len, wasi_size_t* nread) {
    if (fd < 3) {
        // We only support writing to stdout/stderr in this stub.
        return WASI_EBADF;
    }
    // TODO: Map to Sovereign VFS vfs_read()
    *nread = 0;
    return WASI_ENOSYS;
}

wasi_errno_t wasi_snapshot_preview1_fd_write(wasi_fd_t fd, const wasi_iovec_t* iovs, size_t iovs_len, wasi_size_t* nwritten) {
    if (fd != 1 && fd != 2) {
        // Not stdout (1) or stderr (2)
        return WASI_EBADF;
    }
    
    wasi_size_t total_written = 0;
    for (size_t i = 0; i < iovs_len; i++) {
        // TODO: In a real WASM runtime, 'iovs[i].buf' is a pointer offset into WASM linear memory.
        // We would read from the isolated WASM instance memory bounds.
        // For now, we mock the success.
        total_written += iovs[i].buf_len;
    }
    
    *nwritten = total_written;
    return WASI_ESUCCESS;
}

wasi_errno_t wasi_snapshot_preview1_args_sizes_get(wasi_size_t* argc, wasi_size_t* argv_buf_size) {
    *argc = 0;
    *argv_buf_size = 0;
    return WASI_ESUCCESS;
}

wasi_errno_t wasi_snapshot_preview1_args_get(uint32_t* argv, uint32_t* argv_buf) {
    return WASI_ESUCCESS;
}

wasi_errno_t wasi_snapshot_preview1_proc_exit(uint32_t rval) {
    // TODO: Invoke Sovereign scheduler to kill the current process ring and free WASM memory.
    while(true) {
        // Halt
    }
    return WASI_ESUCCESS;
}

} // extern "C"

} // namespace wasi
} // namespace sigma
