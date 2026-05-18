#include "../../../../sigma_libc.h"

extern "C" sigma_u64 handle_process_spawn(const char* binary, const char** argv) {
    sigma_printf("[Syscall Handler] process_spawn called for binary: %s\n", binary ? binary : "NULL");
    return 1001; // New child PID
}
