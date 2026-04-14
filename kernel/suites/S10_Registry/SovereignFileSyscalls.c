#include "../../include/sigma_base.h"

#include "../../include/SovereignSyscall.h"
#include "../../include/sigma_libc.h"

sigma_i64 sys_read_shard(sigma_u64 fd, sigma_u64 buf, sigma_u64 count, sigma_u64 a4, sigma_u64 a5, sigma_u64 a6) {
    sigma_printf("  Σ [SYS-FS]: Routing read(fd=%llu) via SovereignVFS.\n", (unsigned long long)fd);
    return (sigma_i64)count;
}

void SovereignFileSyscalls_Register(void) {
    SovereignSyscall_Register(0, sys_read_shard); /* SYS_read = 0 */
}
