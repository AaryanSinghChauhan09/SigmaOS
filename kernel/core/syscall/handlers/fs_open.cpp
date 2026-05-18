#include "../../../../sigma_libc.h"

extern "C" sigma_u64 handle_fs_open(const char* path, int flags, int mode) {
    sigma_printf("[Syscall Handler] fs_open called for path: %s\n", path ? path : "NULL");
    return 3;
}
