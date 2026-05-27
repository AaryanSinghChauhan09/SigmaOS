#ifndef SIGMA_WASI_H
#define SIGMA_WASI_H

#include "sigma_types.h"

// ---------------------------------------------------------
// SigmaOS WASI Compatibility Layer Definitions
// This bridges standard WebAssembly System Interface calls
// to Sovereign IPC/VFS abstractions.
// ---------------------------------------------------------

namespace sigma {
namespace wasi {

typedef uint32_t wasi_fd_t;
typedef uint32_t wasi_size_t;
typedef uint16_t wasi_errno_t;

const wasi_errno_t WASI_ESUCCESS = 0;
const wasi_errno_t WASI_EBADF = 8;
const wasi_errno_t WASI_EINVAL = 28;
const wasi_errno_t WASI_ENOSYS = 52;

struct wasi_iovec_t {
    uint32_t buf;
    wasi_size_t buf_len;
};

// Syscall stubs (to be mapped by wasi_layer.cpp)
extern "C" {
    wasi_errno_t wasi_snapshot_preview1_fd_read(wasi_fd_t fd, const wasi_iovec_t* iovs, size_t iovs_len, wasi_size_t* nread);
    wasi_errno_t wasi_snapshot_preview1_fd_write(wasi_fd_t fd, const wasi_iovec_t* iovs, size_t iovs_len, wasi_size_t* nwritten);
    wasi_errno_t wasi_snapshot_preview1_args_sizes_get(wasi_size_t* argc, wasi_size_t* argv_buf_size);
    wasi_errno_t wasi_snapshot_preview1_args_get(uint32_t* argv, uint32_t* argv_buf);
    wasi_errno_t wasi_snapshot_preview1_proc_exit(uint32_t rval);
}

} // namespace wasi
} // namespace sigma

#endif // SIGMA_WASI_H
